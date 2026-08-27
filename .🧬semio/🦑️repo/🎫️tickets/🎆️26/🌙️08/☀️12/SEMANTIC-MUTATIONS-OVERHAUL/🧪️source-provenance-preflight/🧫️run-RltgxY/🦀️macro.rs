extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
#[proc_macro_derive(OwnerProbe)]
pub fn owner_probe(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let name = loop {
        match tokens.next() {
            Some(TokenTree::Ident(value)) if value.to_string() == "struct" => break tokens.next().expect("name"),
            Some(_) => (),
            None => panic!("struct missing"),
        }
    };
    let span = name.span();
    let local = span.local_file().expect("local source").canonicalize().expect("source exists");
    format!("impl {} {{ pub const LOCAL: &'static str = {:?}; pub const REPORTED: &'static str = {:?}; }}", name, local.to_str().expect("utf8"), span.file()).parse().expect("generated declaration")
}
