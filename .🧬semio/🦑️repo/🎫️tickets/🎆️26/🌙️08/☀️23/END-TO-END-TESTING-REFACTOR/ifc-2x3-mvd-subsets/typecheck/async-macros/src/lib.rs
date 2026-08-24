//! 🧪️ Scratch shim for the repository's own `#[semio_framework_async_macros::async_test]`, so the
//! REAL `part21` module (whose own test block uses it) compiles inside this standalone harness.
//! Rewrites the annotated `async fn` into a plain `#[test]` that drives the future to completion on
//! a no-op waker — every body here is non-suspending, which is the whole premise of the repo's own
//! async-convention debt note.
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let source = item.to_string();
    let source = source.replacen("async fn", "fn", 1);
    let open = source.find('{').expect("a function body");
    let close = source.rfind('}').expect("a function body");
    let signature = &source[..open];
    let body = &source[open + 1..close];
    format!("#[test]\n{signature}{{ ::protocol::block_on(async {{ {body} }}) }}").parse().expect("valid rewritten test")
}
