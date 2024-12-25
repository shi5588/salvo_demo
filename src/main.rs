use salvo::prelude::*;
//use salvo::conn::rustls::{Keycert, RustlsConfig};

#[handler]
async fn hello() -> &'static str {
    // 任何类型都可以作为函数的返回类型, 只要它实现了 Writer trait. 比如 &str 实现了 Writer, 当它被作为返回值时, 就打印纯文本:
    "Hello World"
}

#[handler]
async fn h2(res: &mut Response) -> Result<&'static str, ()> {
    // 更普遍的情况是, 我们需要在将 Result<T, E> 作为返回类型, 以便处理函数执行过程中的错误. 如果 T 和 E 都实现了 Writer, 那么 Result<T, E> 就可以作为返回值:
    Ok("Hello world")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
