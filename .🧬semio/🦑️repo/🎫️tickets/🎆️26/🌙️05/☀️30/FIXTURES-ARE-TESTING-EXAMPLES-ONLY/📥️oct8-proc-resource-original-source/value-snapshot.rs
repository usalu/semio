//! 📚️ Compile source-authored JSON into intrinsic value expressions.
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashSet;
use syn::LitStr;

pub(crate) fn expand(path: &LitStr) -> syn::Result<TokenStream> {
    let relative = path.value();
    if std::path::Path::new(&relative).is_absolute() { return Err(syn::Error::new(path.span(), "source asset path must be relative to its package")); }
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").ok_or_else(|| syn::Error::new(path.span(), "source asset package is unavailable"))?;
    let absolute = std::path::PathBuf::from(manifest).join(&relative);
    let metadata = std::fs::metadata(&absolute).map_err(|error| syn::Error::new(path.span(), error))?;
    if metadata.len() > 8 * 1024 * 1024 { return Err(syn::Error::new(path.span(), "source asset exceeds 8 MiB")); }
    let source = std::fs::read_to_string(absolute).map_err(|error| syn::Error::new(path.span(), error))?;
    let mut parser = Parser { source: &source, offset: 0, span: path.span() };
    let value = parser.value(0)?;
    parser.whitespace();
    if parser.offset != source.len() { return parser.refuse("unexpected trailing content"); }
    Ok(quote!({ const _: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #path)); #value }))
}

struct Parser<'s> { source: &'s str, offset: usize, span: Span }
impl Parser<'_> {
    fn refuse<T>(&self, message: &str) -> syn::Result<T> { Err(syn::Error::new(self.span, format!("{message} at source byte {}", self.offset))) }
    fn peek(&self) -> Option<u8> { self.source.as_bytes().get(self.offset).copied() }
    fn whitespace(&mut self) { while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) { self.offset += 1; } }
    fn take(&mut self, byte: u8) -> bool { if self.peek() == Some(byte) { self.offset += 1; true } else { false } }
    fn literal(&mut self, text: &str, value: TokenStream) -> syn::Result<TokenStream> {
        if !self.source[self.offset..].starts_with(text) { return self.refuse("invalid JSON literal"); }
        self.offset += text.len(); Ok(value)
    }
    fn value(&mut self, depth: usize) -> syn::Result<TokenStream> {
        if depth > 128 { return self.refuse("source asset nesting exceeds 128 levels"); }
        self.whitespace();
        match self.peek() {
            Some(b'n') => self.literal("null", quote!(::semio_framework_value::DslValue::Null)),
            Some(b't') => self.literal("true", quote!(::semio_framework_value::DslValue::Bool(true))),
            Some(b'f') => self.literal("false", quote!(::semio_framework_value::DslValue::Bool(false))),
            Some(b'"') => { let value = self.string()?; Ok(quote!(::semio_framework_value::DslValue::String(::std::string::String::from(#value)))) }
            Some(b'[') => {
                self.offset += 1; self.whitespace(); let mut values = Vec::new();
                if !self.take(b']') { loop { values.push(self.value(depth + 1)?); self.whitespace(); if self.take(b']') { break; } if !self.take(b',') { return self.refuse("expected array separator"); } } }
                Ok(quote!(::semio_framework_value::DslValue::Array(::std::vec![#(#values),*])))
            }
            Some(b'{') => {
                self.offset += 1; self.whitespace(); let mut values = Vec::new(); let mut keys = HashSet::new();
                if !self.take(b'}') { loop {
                    self.whitespace(); let key = self.string()?;
                    if !keys.insert(key.clone()) { return self.refuse("duplicate object key"); }
                    self.whitespace(); if !self.take(b':') { return self.refuse("expected object member separator"); }
                    let value = self.value(depth + 1)?; values.push(quote!((::std::string::String::from(#key), #value)));
                    self.whitespace(); if self.take(b'}') { break; } if !self.take(b',') { return self.refuse("expected object separator"); }
                } }
                Ok(quote!(::semio_framework_value::DslValue::Object(::std::vec![#(#values),*])))
            }
            Some(b'-' | b'0'..=b'9') => self.number(),
            _ => self.refuse("expected JSON value"),
        }
    }
    fn string(&mut self) -> syn::Result<String> {
        if !self.take(b'"') { return self.refuse("expected JSON string"); }
        let mut result = String::new();
        loop { match self.peek() {
            None => return self.refuse("unterminated JSON string"),
            Some(b'"') => { self.offset += 1; return Ok(result); }
            Some(0..=31) => return self.refuse("unescaped string control character"),
            Some(b'\\') => {
                self.offset += 1; let escaped = self.peek().ok_or_else(|| syn::Error::new(self.span, "unterminated JSON escape"))?; self.offset += 1;
                match escaped {
                    b'"' => result.push('"'), b'\\' => result.push('\\'), b'/' => result.push('/'), b'b' => result.push('\u{8}'), b'f' => result.push('\u{c}'), b'n' => result.push('\n'), b'r' => result.push('\r'), b't' => result.push('\t'),
                    b'u' => {
                        let first = self.hex_word()?;
                        let scalar = if (0xd800..=0xdbff).contains(&first) {
                            if !self.take(b'\\') || !self.take(b'u') { return self.refuse("missing low Unicode surrogate"); }
                            let second = self.hex_word()?; if !(0xdc00..=0xdfff).contains(&second) { return self.refuse("invalid low Unicode surrogate"); }
                            0x10000 + ((u32::from(first) - 0xd800) << 10) + u32::from(second) - 0xdc00
                        } else { u32::from(first) };
                        match char::from_u32(scalar) { Some(value) => result.push(value), None => return self.refuse("invalid Unicode scalar") }
                    }
                    _ => return self.refuse("invalid JSON escape"),
                }
            }
            Some(_) => { let scalar = self.source[self.offset..].chars().next().unwrap(); self.offset += scalar.len_utf8(); result.push(scalar); }
        } }
    }
    fn hex_word(&mut self) -> syn::Result<u16> {
        let mut value = 0_u16;
        for _ in 0..4 { let digit = match self.peek() { Some(b'0'..=b'9') => self.peek().unwrap() - b'0', Some(b'a'..=b'f') => self.peek().unwrap() - b'a' + 10, Some(b'A'..=b'F') => self.peek().unwrap() - b'A' + 10, _ => return self.refuse("invalid Unicode escape") }; self.offset += 1; value = (value << 4) | u16::from(digit); }
        Ok(value)
    }
    fn digits(&mut self) -> syn::Result<()> {
        let start = self.offset; while matches!(self.peek(), Some(b'0'..=b'9')) { self.offset += 1; }
        if start == self.offset { self.refuse("expected number digit") } else { Ok(()) }
    }
    fn number(&mut self) -> syn::Result<TokenStream> {
        let start = self.offset; self.take(b'-');
        if self.take(b'0') { if matches!(self.peek(), Some(b'0'..=b'9')) { return self.refuse("number has a leading zero"); } } else { self.digits()?; }
        let mut fractional = false;
        if self.take(b'.') { fractional = true; self.digits()?; }
        if matches!(self.peek(), Some(b'e' | b'E')) { fractional = true; self.offset += 1; if matches!(self.peek(), Some(b'+' | b'-')) { self.offset += 1; } self.digits()?; }
        let token = &self.source[start..self.offset];
        if !fractional && token != "-0" {
            if token.starts_with('-') { if let Ok(value) = token.parse::<i64>() { return Ok(quote!(::semio_framework_value::DslValue::int(#value))); } }
            else if let Ok(value) = token.parse::<u64>() { return Ok(quote!(::semio_framework_value::DslValue::uint(#value))); }
        }
        let value = token.parse::<f64>().map_err(|error| syn::Error::new(self.span, error))?;
        if !value.is_finite() { return self.refuse("number is outside finite binary64 range"); }
        let bits = value.to_bits(); Ok(quote!(::semio_framework_value::DslValue::float(::std::primitive::f64::from_bits(#bits))))
    }
}
