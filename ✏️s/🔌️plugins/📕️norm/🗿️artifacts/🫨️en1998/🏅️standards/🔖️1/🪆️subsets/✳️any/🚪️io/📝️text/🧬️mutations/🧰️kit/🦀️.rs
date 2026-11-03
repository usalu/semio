//! 🧰️ EN 1998 mutation text kit: the quoting, JSON-atom and `key=value` argument helpers every family codec shares.

/// 🧰️ The decoded `key=value` arguments of one mutation line.
pub type Args = std::collections::BTreeMap<String, String>;

/// 🔑 One required argument of `keyword`, or the missing-argument refusal.
pub fn required(args: &Args, keyword: &str, key: &str) -> Result<String, String> {
    args.get(key).cloned().ok_or_else(|| format!("en1998 mutation: missing arg '{key}' for '{keyword}'"))
}

pub fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
pub fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
pub fn enc_json<T: semio_framework_value::ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}
pub fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

/// 📍 An insert's optional `index=` argument: absent appends.
pub fn opt_index(args: &Args) -> Result<Option<usize>, String> {
    args.get("index").map_or(Ok(None), |raw| dec_json::<Option<usize>>(raw))
}
pub fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
pub fn parse_args(rest: &str) -> Result<Args, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
