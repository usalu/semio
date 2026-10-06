//! 📤️ Inspects proven generated token inputs without treating them as current-crate reads.
use super::tokens::{Kind, Token, tokens, token_pairs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustGeneratedInputKind { Include, IncludeStr, IncludeBytes }

impl RustGeneratedInputKind {
    pub fn as_str(self) -> &'static str { match self { Self::Include => "include", Self::IncludeStr => "include_str", Self::IncludeBytes => "include_bytes" } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustGeneratorOriginKind { DirectImport, Qualified }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustGeneratorOrigin {
    pub kind: RustGeneratorOriginKind,
    pub import_start: Option<usize>,
    pub import_end: Option<usize>,
}

impl RustGeneratorOrigin {
    pub fn provider(&self) -> &'static str { "quote" }
    pub fn symbol(&self) -> &'static str { "quote" }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustGeneratedTokenInput {
    pub kind: RustGeneratedInputKind,
    pub expression: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustGeneratedTokenOutput {
    pub macro_name: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub origin: RustGeneratorOrigin,
    pub inputs: Vec<RustGeneratedTokenInput>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustGeneratorErrorKind { UnresolvedOrigin, InvalidDelimiter }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustGeneratorError { pub kind: RustGeneratorErrorKind, pub macro_name: String }

impl RustGeneratorError {
    pub fn code(&self) -> &'static str { match self.kind { RustGeneratorErrorKind::UnresolvedOrigin => "unresolved-generator-origin", RustGeneratorErrorKind::InvalidDelimiter => "invalid-generator-delimiter" } }
}

impl std::fmt::Display for RustGeneratorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(formatter, "{}: {}", self.code(), self.macro_name) }
}

impl std::error::Error for RustGeneratorError {}

fn symbol<'a>(token: &Token<'a>) -> Option<&'a str> { (token.kind == Kind::Identifier).then(|| token.text.strip_prefix("r#").unwrap_or(token.text)) }

fn statement_end(tokens: &[Token<'_>], pairs: &[Option<usize>], mut index: usize) -> Option<usize> {
    while index < tokens.len() {
        if tokens[index].text == ";" { return Some(index); }
        if matches!(tokens[index].text, "(" | "[" | "{") { index = pairs[index]? + 1; } else { index += 1; }
    }
    None
}

/// 🪪️ Returns explicit generated inputs only for closed direct-import or qualified quote origins.
pub fn inspect_rust_generated_token_outputs(source: &str) -> Result<Vec<RustGeneratedTokenOutput>, RustGeneratorError> {
    let tokens = tokens(source);
    let pairs = token_pairs(&tokens);
    let mut imports = std::collections::HashMap::<&str, (usize, usize)>::new();
    let mut ambiguous = std::collections::HashSet::<&str>::new();
    let mut qualified_closed = true;
    let mut index = 0;
    while index < tokens.len() {
        let is_quote_module = tokens[index].text == "mod" && tokens.get(index + 1).and_then(symbol) == Some("quote");
        let is_quote_extern = tokens[index].text == "extern" && tokens.get(index + 1).is_some_and(|token| token.text == "crate") && tokens.get(index + 2).and_then(symbol) == Some("quote");
        if is_quote_module || is_quote_extern { qualified_closed = false; }
        if tokens[index].text != "use" { index += 1; continue; }
        let Some(end) = statement_end(&tokens, &pairs, index + 1) else { index += 1; continue; };
        let body = &tokens[index + 1..end];
        let direct = body.len() == 3 || body.len() == 5 && body[3].text == "as";
        let from_quote = body.len() >= 3 && body[0].text == "quote" && body[1].text == "::" && body[2].text == "quote";
        let alias = if direct && from_quote { body.last().and_then(symbol) } else { None };
        let enclosed = pairs.iter().enumerate().any(|(open, close)| open < index && close.is_some_and(|close| close > index) && tokens[open].text == "{");
        let attributed = pairs.iter().enumerate().any(|(open, close)| *close == index.checked_sub(1) && tokens[open].text == "[" && open > 0 && tokens[open - 1].text == "#");
        if let Some(alias) = alias.filter(|_| !enclosed && !attributed && index.checked_sub(1).is_none_or(|before| tokens[before].text != "pub")) {
            if imports.insert(alias, (tokens[index].start, tokens[end].end)).is_some() { ambiguous.insert(alias); }
        } else if body.iter().any(|token| token.text == "*") { qualified_closed = false; ambiguous.insert("*"); }
        else if body.iter().any(|token| symbol(token) == Some("quote")) { qualified_closed = false; ambiguous.insert("quote"); }
        index = end + 1;
    }
    index = 0;
    while index < tokens.len() {
        if tokens[index].text != "use" { index += 1; continue; }
        let Some(end) = statement_end(&tokens, &pairs, index + 1) else { index += 1; continue; };
        for (alias, origin) in &imports { if tokens[index].start != origin.0 && tokens[index + 1..end].iter().any(|token| symbol(token) == Some(*alias)) { ambiguous.insert(alias); } }
        index = end + 1;
    }
    for (index, token) in tokens.iter().enumerate() {
        if token.text == "macro_rules" && tokens.get(index + 1).is_some_and(|token| token.text == "!") { if let Some(name) = tokens.get(index + 2).and_then(symbol) { ambiguous.insert(name); } }
        if matches!(token.text, "macro" | "mod") { if let Some(name) = tokens.get(index + 1).and_then(symbol) { ambiguous.insert(name); } }
    }
    let mut outputs = Vec::new();
    index = 0;
    while index < tokens.len() {
        if tokens.get(index + 1).is_none_or(|token| token.text != "!") { index += 1; continue; }
        let mut first = index;
        while first >= 2 && tokens[first - 1].text == "::" && tokens[first - 2].kind == Kind::Identifier { first -= 2; }
        let name = tokens[first..=index].iter().map(|token| token.text).collect::<String>();
        let Some(close) = pairs.get(index + 2).copied().flatten() else { index += 1; continue; };
        if name != "quote::quote" && !imports.contains_key(name.as_str()) && name != "quote" { index += 1; continue; }
        let mut inputs = Vec::new();
        let mut at = index + 3;
        while at < close {
            let kind = match symbol(&tokens[at]) { Some("include") => Some(RustGeneratedInputKind::Include), Some("include_str") => Some(RustGeneratedInputKind::IncludeStr), Some("include_bytes") => Some(RustGeneratedInputKind::IncludeBytes), _ => None };
            if let Some(kind) = kind.filter(|_| tokens.get(at + 1).is_some_and(|token| token.text == "!")) {
                let end = pairs.get(at + 2).copied().flatten().filter(|end| *end <= close).ok_or_else(|| RustGeneratorError { kind: RustGeneratorErrorKind::InvalidDelimiter, macro_name: name.clone() })?;
                inputs.push(RustGeneratedTokenInput { kind, expression: source[tokens[at + 2].byte_end..tokens[end].byte_start].into(), start: tokens[at].start, end: tokens[end].end, line: tokens[at].line });
                at = end + 1;
            } else { at += 1; }
        }
        if inputs.is_empty() { index += 1; continue; }
        let qualified = name == "quote::quote";
        let imported = imports.get(name.as_str());
        if !qualified_closed || ambiguous.contains("*") || ambiguous.contains(name.as_str()) || ambiguous.contains("quote") || !qualified && imported.is_none() { return Err(RustGeneratorError { kind: RustGeneratorErrorKind::UnresolvedOrigin, macro_name: name }); }
        outputs.push(RustGeneratedTokenOutput { macro_name: name, start: tokens[first].start, end: tokens[close].end, line: tokens[first].line, origin: RustGeneratorOrigin { kind: if qualified { RustGeneratorOriginKind::Qualified } else { RustGeneratorOriginKind::DirectImport }, import_start: imported.filter(|_| !qualified).map(|value| value.0), import_end: imported.filter(|_| !qualified).map(|value| value.1) }, inputs });
        index = close + 1;
    }
    Ok(outputs)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
