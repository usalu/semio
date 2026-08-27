extern crate proc_macro; use proc_macro::TokenStream; #[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))] pub fn mutation_leaf(_: TokenStream) -> TokenStream { TokenStream::new() }
