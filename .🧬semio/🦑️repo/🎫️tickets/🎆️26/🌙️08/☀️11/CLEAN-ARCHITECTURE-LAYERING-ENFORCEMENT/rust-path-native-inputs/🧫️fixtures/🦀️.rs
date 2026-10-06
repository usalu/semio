//! 📁️ Inspects whole first literal path arguments with caller-owned neutral roots.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustPathLiteralContext {
    Method,
    Qualified,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RustPathLiteral {
    pub value: String,
    pub path: String,
    pub root: String,
    pub call: String,
    pub context: RustPathLiteralContext,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RustPathRootError;

impl RustPathRootError {
    pub fn code(&self) -> &'static str {
        "invalid-root"
    }
}

impl std::fmt::Display for RustPathRootError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Rust path root must be unique and canonical relative")
    }
}

impl std::error::Error for RustPathRootError {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Identifier,
    String,
    Other,
}

struct Token<'a> {
    kind: Kind,
    text: &'a str,
    line: usize,
    start: usize,
    end: usize,
}

fn whitespace(value: char) -> bool {
    matches!(value, '\u{0009}'..='\u{000d}' | ' ' | '\u{00a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

fn identifier(value: char) -> bool {
    value == '_' || value.is_alphanumeric()
}

fn tokens(source: &str) -> Vec<Token<'_>> {
    let mut output = Vec::new();
    let mut byte = 0;
    let mut utf16 = 0;
    let mut line = 1;
    while byte < source.len() {
        let start = byte;
        let remaining = &source[byte..];
        let character = remaining.chars().next().unwrap();
        let mut kind = Kind::Other;
        let mut emit = true;
        if whitespace(character) {
            byte += character.len_utf8();
            emit = false;
        } else if remaining.starts_with("//") {
            byte += remaining.find('\n').unwrap_or(remaining.len());
            emit = false;
        } else if remaining.starts_with("/*") {
            let mut depth = 1;
            byte += 2;
            while byte < source.len() && depth > 0 {
                if source[byte..].starts_with("/*") { depth += 1; byte += 2; }
                else if source[byte..].starts_with("*/") { depth -= 1; byte += 2; }
                else { byte += source[byte..].chars().next().unwrap().len_utf8(); }
            }
            emit = false;
        } else {
            let raw_prefix = if remaining.starts_with("br") { 2 } else if remaining.starts_with('r') { 1 } else { 0 };
            let hashes = remaining.as_bytes().iter().skip(raw_prefix).take_while(|value| **value == b'#').count();
            if raw_prefix > 0 && remaining.as_bytes().get(raw_prefix + hashes) == Some(&b'"') {
                let content = raw_prefix + hashes + 1;
                let suffix = format!("\"{}", "#".repeat(hashes));
                byte += remaining[content..].find(&suffix).map_or(remaining.len(), |close| content + close + suffix.len());
                kind = Kind::String;
            } else if character == '"' || remaining.starts_with("b\"") {
                byte += if character == 'b' { 2 } else { 1 };
                while byte < source.len() {
                    let next = source[byte..].chars().next().unwrap();
                    byte += next.len_utf8();
                    if next == '\\' && byte < source.len() { byte += source[byte..].chars().next().unwrap().len_utf8(); }
                    else if next == '"' { break; }
                }
                kind = Kind::String;
            } else if character == '\'' {
                let mut cursor = byte + 1;
                if source[cursor..].starts_with('\\') {
                    cursor += 1;
                    if let Some(selector) = source[cursor..].chars().next() {
                        cursor += selector.len_utf8();
                        if selector == 'x' { cursor = (cursor + 2).min(source.len()); }
                        else if selector == 'u' && source[cursor..].starts_with('{') { cursor = source[cursor..].find('}').map_or(source.len(), |close| cursor + close + 1); }
                    }
                } else if let Some(next) = source[cursor..].chars().next() { cursor += next.len_utf8(); }
                if source.as_bytes().get(cursor) == Some(&b'\'') { byte = cursor + 1; kind = Kind::String; }
                else { byte += 1; }
            } else if remaining.starts_with("r#") && remaining[2..].chars().next().is_some_and(|value| identifier(value) && !value.is_ascii_digit()) {
                byte += 2;
                while byte < source.len() && source[byte..].chars().next().is_some_and(identifier) { byte += source[byte..].chars().next().unwrap().len_utf8(); }
                kind = Kind::Identifier;
            } else if identifier(character) {
                byte += character.len_utf8();
                while byte < source.len() && source[byte..].chars().next().is_some_and(identifier) { byte += source[byte..].chars().next().unwrap().len_utf8(); }
                if !character.is_ascii_digit() { kind = Kind::Identifier; }
            } else if remaining.starts_with("::") { byte += 2; }
            else { byte += character.len_utf8(); }
        }
        let end = utf16 + source[start..byte].encode_utf16().count();
        if emit { output.push(Token { kind, text: &source[start..byte], line, start: utf16, end }); }
        line += source[start..byte].bytes().filter(|value| *value == b'\n').count();
        utf16 = end;
    }
    output
}

fn string_value(token: &Token<'_>) -> Option<String> {
    if token.kind != Kind::String { return None; }
    let text = token.text;
    if text.starts_with('r') || text.starts_with("br") {
        let quote = text.find('"')?;
        let prefix = if text.starts_with("br") { 2 } else { 1 };
        let hashes = quote - prefix;
        let suffix = format!("\"{}", "#".repeat(hashes));
        return text.strip_suffix(&suffix).and_then(|value| value.get(quote + 1..)).map(str::to_owned);
    }
    let text = text.strip_prefix('b').unwrap_or(text);
    let body = text.strip_prefix('"')?.strip_suffix('"')?;
    let mut characters = body.chars().peekable();
    let mut value = String::new();
    while let Some(character) = characters.next() {
        if character != '\\' { value.push(character); continue; }
        match characters.next()? {
            'n' => value.push('\n'), 'r' => value.push('\r'), 't' => value.push('\t'), '0' => value.push('\0'),
            '\\' => value.push('\\'), '"' => value.push('"'), '\'' => value.push('\''),
            'x' => { let first = characters.next()?.to_digit(16)?; let second = characters.next()?.to_digit(16)?; value.push(char::from_u32(first * 16 + second)?); }
            'u' => {
                if characters.next()? != '{' { return None; }
                let mut digits = String::new();
                loop { let next = characters.next()?; if next == '}' { break; } if next != '_' { digits.push(next); } }
                if digits.is_empty() || digits.len() > 6 || !digits.chars().all(|value| value.is_ascii_hexdigit()) { return None; }
                value.push(char::from_u32(u32::from_str_radix(&digits, 16).ok()?)?);
            }
            '\n' => { while characters.peek().is_some_and(|value| whitespace(*value)) { characters.next(); } }
            '\r' => { if characters.next()? != '\n' { return None; } while characters.peek().is_some_and(|value| whitespace(*value)) { characters.next(); } }
            _ => return None,
        }
    }
    Some(value)
}

fn qualified_call(call: &str) -> bool {
    matches!(call, "std::path::Path::new" | "std::path::PathBuf::from" | "std::fs::File::open" | "std::fs::File::create")
        || call.strip_prefix("std::fs::").is_some_and(|name| matches!(name, "read" | "read_to_string" | "read_dir" | "write" | "metadata" | "symlink_metadata" | "canonicalize" | "create_dir" | "create_dir_all" | "remove_file" | "remove_dir" | "remove_dir_all"))
}

/// 🧭️ Returns syntactic whole first-literal references; receiver and alias types remain unresolved.
pub fn inspect_rust_path_literals(source: &str, roots: &[&str]) -> Result<Vec<RustPathLiteral>, RustPathRootError> {
    let mut seen = std::collections::HashSet::new();
    for root in roots {
        if !seen.insert(*root) || root.chars().any(|value| matches!(value, '\\' | ':' | '\0' | '\r' | '\n')) || root.split('/').any(|part| matches!(part, "" | "." | "..")) { return Err(RustPathRootError); }
    }
    let tokens = tokens(source);
    let mut pairs = vec![None; tokens.len()];
    let mut stack = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if matches!(token.text, "(" | "[" | "{") { stack.push(index); }
        else if matches!(token.text, ")" | "]" | "}") {
            if let Some(open) = stack.last().copied() {
                if matches!((tokens[open].text, token.text), ("(", ")") | ("[", "]") | ("{", "}")) { stack.pop(); pairs[open] = Some(index); pairs[index] = Some(open); }
            }
        }
    }
    let mut output = Vec::new();
    for open in 1..tokens.len() {
        if tokens[open].text != "(" || tokens[open - 1].kind != Kind::Identifier { continue; }
        let Some(close) = pairs[open] else { continue; };
        let name = tokens[open - 1].text.strip_prefix("r#").unwrap_or(tokens[open - 1].text);
        let method = open >= 2 && tokens[open - 2].text == ".";
        let mut first = open - 1;
        if !method { while first >= 2 && tokens[first - 1].text == "::" && tokens[first - 2].kind == Kind::Identifier { first -= 2; } }
        let call = if method { name.to_owned() } else { tokens[first..open].iter().map(|token| token.text).collect::<String>() };
        if !(if method { matches!(name, "join" | "push") } else { qualified_call(&call) }) { continue; }
        let Some(argument) = tokens.get(open + 1) else { continue; };
        if open + 2 != close && tokens.get(open + 2).is_none_or(|token| token.text != ",") { continue; }
        let Some(value) = string_value(argument) else { continue; };
        let normalized = value.replace('\\', "/");
        let mut path = normalized.as_str();
        while let Some(next) = path.strip_prefix("./").or_else(|| path.strip_prefix("../")) { path = next; }
        let Some(root) = roots.iter().filter(|root| path == **root || path.strip_prefix(**root).is_some_and(|suffix| suffix.starts_with('/'))).max_by_key(|root| root.encode_utf16().count()) else { continue; };
        if path.chars().any(|value| matches!(value, '\0' | '\r' | '\n')) { continue; }
        output.push(RustPathLiteral { value, path: path.to_owned(), root: (*root).to_owned(), call, context: if method { RustPathLiteralContext::Method } else { RustPathLiteralContext::Qualified }, start: argument.start, end: argument.end, line: argument.line });
    }
    output.sort_by_key(|value| value.start);
    Ok(output)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
