extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }

#[proc_macro_attribute]
pub fn rewrite(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }

#[proc_macro_attribute]
pub fn doc(_: TokenStream, _: TokenStream) -> TokenStream { TokenStream::new() }
