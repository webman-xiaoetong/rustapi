use salvo::prelude::*;

#[handler]
async fn hello(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}

#[tokio::main]
async fn main() {
    let mut router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:8787").bind().await;
    Server::new(acceptor).serve(router).await;
}