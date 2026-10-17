//! 🧭️ Path helpers of the EN 1999 compliance tests: the family directory, the oracle script and the snapshot path grammar and editors. See [`super`].

use super::*;

pub(super) fn family_any_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

pub(super) fn oracle_script() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("⚖️compliance").join("🐍️.py")
}

pub(super) fn snapshot_schema_path() -> PathBuf {
    family_any_dir().join("🧬️schema").join("📸️snapshot").join("🔣️.json")
}

pub(super) fn path_grammar_ok(path: &str) -> bool {
    let mut rest = path;
    if rest.is_empty() {
        return false;
    }
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let Some(close) = rest.find(']') else {
                return false;
            };
            let inner = &rest[1..close];
            if let Some(id) = inner.strip_prefix("id=") {
                if id.is_empty() || id.contains([']', '.', '=']) {
                    return false;
                }
            } else if inner.parse::<usize>().is_err() {
                return false;
            }
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        if end == 0 {
            return false;
        }
        rest = &rest[end..];
    }
    true
}

pub(super) fn resolve_path<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let mut cursor = root;
    let mut rest = path;
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']')?;
            let inner = &rest[1..close];
            let arr = cursor.as_array()?;
            cursor = if let Some(id) = inner.strip_prefix("id=") {
                arr.iter().find(|el| el.get("id").and_then(|v| v.as_str()) == Some(id))?
            } else {
                arr.get(inner.parse::<usize>().ok()?)?
            };
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        let field = &rest[..end];
        cursor = cursor.get(field)?;
        rest = &rest[end..];
    }
    Some(cursor)
}

enum PathSeg {
    Field(String),
    Index(usize),
    Id(String),
}

fn walk_mut<'a>(root: &'a mut serde_json::Value, segments: &[PathSeg]) -> Result<&'a mut serde_json::Value, String> {
    let mut cursor = root;
    for seg in segments {
        cursor = match seg {
            PathSeg::Field(name) => cursor.get_mut(name).ok_or_else(|| format!("missing {name}"))?,
            PathSeg::Index(i) => cursor.get_mut(*i).ok_or_else(|| format!("missing [{i}]"))?,
            PathSeg::Id(id) => {
                let arr = cursor.as_array_mut().ok_or_else(|| "expected array".to_string())?;
                let idx = arr
                    .iter()
                    .position(|el| el.get("id").and_then(|v| v.as_str()) == Some(id.as_str()))
                    .ok_or_else(|| format!("missing id={id}"))?;
                &mut arr[idx]
            }
        };
    }
    Ok(cursor)
}

fn parse_segments(path: &str) -> Result<Vec<PathSeg>, String> {
    let mut segments = Vec::new();
    let mut rest = path;
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']').ok_or_else(|| "unclosed".to_string())?;
            let inner = &rest[1..close];
            if let Some(id) = inner.strip_prefix("id=") {
                segments.push(PathSeg::Id(id.to_string()));
            } else {
                segments.push(PathSeg::Index(inner.parse().map_err(|_| "bad index")?));
            }
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        segments.push(PathSeg::Field(rest[..end].to_string()));
        rest = &rest[end..];
    }
    Ok(segments)
}

pub(super) fn set_bool_at_path(root: &mut serde_json::Value, path: &str, value: bool) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::Value::Bool(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

pub(super) fn set_u64_at_path(root: &mut serde_json::Value, path: &str, value: u64) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::json!(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

pub(super) fn set_number_at_path(root: &mut serde_json::Value, path: &str, value: f64) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::json!(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}

pub(super) fn set_string_at_path(root: &mut serde_json::Value, path: &str, value: &str) -> Result<(), String> {
    let segments = parse_segments(path)?;
    let (last, parents) = segments.split_last().ok_or_else(|| "empty path".to_string())?;
    let cursor = walk_mut(root, parents)?;
    match last {
        PathSeg::Field(name) => {
            let slot = cursor.get_mut(name).ok_or_else(|| format!("missing leaf {name}"))?;
            *slot = serde_json::json!(value);
            Ok(())
        }
        _ => Err("leaf must be a field".into()),
    }
}
