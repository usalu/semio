use proc_macro::TokenStream;

#[proc_macro]
pub fn authority(_: TokenStream) -> TokenStream {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data.json");
    let text = std::fs::read_to_string(path).expect("data.json");
    let rows = text.matches("authority").count();
    format!("pub const AUTHORITY_ROWS: usize = {rows}; const _: &str = include_str!({path:?});").parse().unwrap()
}
