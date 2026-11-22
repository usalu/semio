fn main() {
    if std::env::var("TARGET").as_deref() == Ok("wasm32-wasip2") {
        println!("cargo::rustc-link-arg-cdylib=-zstack-size=33554432");
    }
}
