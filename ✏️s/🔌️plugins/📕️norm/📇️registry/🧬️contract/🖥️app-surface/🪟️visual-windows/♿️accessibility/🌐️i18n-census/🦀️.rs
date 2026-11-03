//! 🌐️ i18n census: scans the Rust sources of the whole norm plugin for user-visible bilingual strings and reports every one whose English or German text is missing.
//!
//! 🔗️ Constructors scanned: `LocalizedLabel::native`, `LocalizedCopy::new`, `World3dText::new`, `LocalizedText::new`, `FieldLabel::new` and the `en`/`de` and `label_en`/`label_de` field pairs; see `visual-editor-architecture.md` §8.1.

use std::path::{Path, PathBuf};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

const CALLS: [&str; 5] = ["LocalizedLabel::native(", "LocalizedCopy::new(", "World3dText::new(", "LocalizedText::new(", "FieldLabel::new("];
const FIELD_PAIRS: [(&str, &str); 2] = [("label_en:", "label_de:"), ("en:", "de:")];
const ENGLISH_WORDS: [&str; 18] = ["the", "of", "and", "for", "with", "check", "design", "load", "resistance", "strength", "moment", "shear", "force", "factor", "value", "increase", "limit", "section"];
const SKIPPED_DIRS: [&str; 5] = ["🧪️tests", "🔮️oracles", "target", "node_modules", "🗑️generated"];

/// 🚩️ One census finding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub file: String,
    pub line: usize,
    pub rule: &'static str,
    pub en: String,
    pub de: String,
}

/// 📊️ Result of one scan.
#[derive(Debug, Default)]
pub struct Census {
    pub pairs: usize,
    pub literal_pairs: usize,
    pub files: usize,
    pub findings: Vec<Finding>,
}

#[derive(Debug, PartialEq, Eq)]
enum Arg {
    Literal(String),
    Dynamic,
}

fn literal_end(src: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    if src[i] == b'r' {
        let mut hashes = 0;
        i += 1;
        while src.get(i) == Some(&b'#') {
            hashes += 1;
            i += 1;
        }
        if src.get(i) != Some(&b'"') {
            return None;
        }
        i += 1;
        while i < src.len() {
            if src[i] == b'"' && (0..hashes).all(|k| src.get(i + 1 + k) == Some(&b'#')) {
                return Some(i + 1 + hashes);
            }
            i += 1;
        }
        return None;
    }
    i += 1;
    while i < src.len() {
        match src[i] {
            b'\\' => i += 2,
            b'"' => return Some(i + 1),
            _ => i += 1,
        }
    }
    None
}

fn unquote(raw: &str) -> String {
    let body = raw.trim_start_matches('r').trim_matches('#');
    let body = body.strip_prefix('"').and_then(|b| b.strip_suffix('"')).unwrap_or(body);
    if raw.starts_with('r') { body.to_owned() } else { body.replace("\\\"", "\"").replace("\\n", "\n").replace("\\\\", "\\").replace("\\'", "'") }
}

fn classify(arg: &str) -> Arg {
    let a = arg.trim().trim_start_matches('&');
    let a = a.strip_suffix(".into()").or_else(|| a.strip_suffix(".to_owned()")).or_else(|| a.strip_suffix(".to_string()")).unwrap_or(a).trim();
    let bytes = a.as_bytes();
    if !bytes.is_empty() && (bytes[0] == b'"' || bytes[0] == b'r') && literal_end(bytes, 0) == Some(bytes.len()) {
        return Arg::Literal(unquote(a));
    }
    if let Some(rest) = a.strip_prefix("format!(") {
        let rest = rest.trim_start();
        if rest.starts_with('"') {
            if let Some(end) = literal_end(rest.as_bytes(), 0) {
                let template = unquote(&rest[..end]);
                let text = strip_placeholders(&template);
                return if template.contains('{') && text.trim().is_empty() { Arg::Dynamic } else { Arg::Literal(text) };
            }
        }
    }
    Arg::Dynamic
}

fn strip_placeholders(template: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for c in template.chars() {
        match c {
            '{' => inside = true,
            '}' => inside = false,
            _ if !inside => out.push(c),
            _ => {}
        }
    }
    out
}

fn split_args(src: &str, open: usize) -> Vec<String> {
    let bytes = src.as_bytes();
    let (mut depth, mut i, mut start) = (0usize, open, open);
    let mut args = Vec::new();
    while i < bytes.len() {
        match bytes[i] {
            b'"' => { i = literal_end(bytes, i).unwrap_or(bytes.len()); continue; }
            b'r' if matches!(bytes.get(i + 1), Some(b'"' | b'#')) && !bytes.get(i.wrapping_sub(1)).is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_') => { i = literal_end(bytes, i).unwrap_or(i + 1); continue; }
            b'\'' if bytes.get(i + 2) == Some(&b'\'') => { i += 3; continue; }
            b'(' | b'[' | b'{' => { depth += 1; if depth == 1 { start = i + 1; } }
            b')' | b']' | b'}' => { depth -= 1; if depth == 0 { args.push(src[start..i].to_owned()); return args; } }
            b',' if depth == 1 => { args.push(src[start..i].to_owned()); start = i + 1; }
            _ => {}
        }
        i += 1;
    }
    args
}

fn line_of(src: &str, at: usize) -> usize {
    src[..at].bytes().filter(|b| *b == b'\n').count() + 1
}

fn english_words(text: &str) -> bool {
    text.split(|c: char| !c.is_ascii_alphabetic()).any(|w| ENGLISH_WORDS.contains(&w.to_ascii_lowercase().as_str()))
}

fn judge(file: &str, line: usize, en: &Arg, de: &Arg, census: &mut Census) {
    census.pairs += 1;
    let (Arg::Literal(e), Arg::Literal(d)) = (en, de) else {
        let empty = |arg: &Arg| matches!(arg, Arg::Literal(t) if t.trim().is_empty());
        if empty(en) {
            census.findings.push(Finding { file: file.into(), line, rule: "empty English", en: String::new(), de: String::new() });
        }
        if empty(de) {
            census.findings.push(Finding { file: file.into(), line, rule: "empty German", en: String::new(), de: String::new() });
        }
        return;
    };
    census.literal_pairs += 1;
    let rule = if e.trim().is_empty() { Some("empty English") } else if d.trim().is_empty() { Some("empty German") } else if e == d && e.len() >= 8 && english_words(e) { Some("German equals English") } else { None };
    if let Some(rule) = rule {
        census.findings.push(Finding { file: file.into(), line, rule, en: e.clone(), de: d.clone() });
    }
}

/// 🔍️ Census of one source text; `file` only labels findings.
pub fn scan_source(file: &str, src: &str, census: &mut Census) {
    for call in CALLS {
        for (at, _) in src.match_indices(call) {
            let args = split_args(src, at + call.len() - 1);
            if args.len() >= 2 {
                judge(file, line_of(src, at), &classify(&args[0]), &classify(&args[1]), census);
            }
        }
    }
    for (en_key, de_key) in FIELD_PAIRS {
        for (at, _) in src.match_indices(en_key) {
            let boundary = src[..at].chars().next_back().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
            if !boundary {
                continue;
            }
            let rest = &src[at + en_key.len()..];
            let Some(en_end) = rest.trim_start().strip_prefix('"').and_then(|_| literal_end(rest.trim_start().as_bytes(), 0)) else { continue };
            let en_text = &rest.trim_start()[..en_end];
            let after = rest.trim_start()[en_end..].trim_start();
            let Some(after) = after.strip_prefix(',').map(str::trim_start) else { continue };
            let Some(de_rest) = after.strip_prefix(de_key).map(str::trim_start) else { continue };
            if !de_rest.starts_with('"') {
                continue;
            }
            let Some(de_end) = literal_end(de_rest.as_bytes(), 0) else { continue };
            judge(file, line_of(src, at), &classify(en_text), &classify(&de_rest[..de_end]), census);
        }
    }
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_owned();
        if p.is_dir() {
            if !SKIPPED_DIRS.contains(&name.as_str()) {
                walk(&p, out);
            }
        } else if name.ends_with(".rs") {
            out.push(p);
        }
    }
}

/// 🌍️ Census of every non-test Rust source below `root`.
pub fn scan_tree(root: &Path) -> Census {
    let mut files = Vec::new();
    walk(root, &mut files);
    let mut census = Census { files: files.len(), ..Census::default() };
    for f in &files {
        if let Ok(src) = std::fs::read_to_string(f) {
            scan_source(&f.strip_prefix(root).unwrap_or(f).display().to_string(), &src, &mut census);
        }
    }
    census
}
