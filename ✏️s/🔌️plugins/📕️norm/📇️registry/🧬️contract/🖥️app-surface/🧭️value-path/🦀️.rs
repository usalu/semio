//! 🧭️ Value paths of the norm app surface: the path parser and the get, set, insert and remove operations on a document value. See [`super`].


//#region 🔖️ValuePath
/// 🧭 One segment of a camelCase snapshot path (`field`, `[index]`, or `[id=…]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathSegment {
    Field(String),
    Index(usize),
    Id(String),
}

/// 🧭 Validates a list-element id for `[id=…]` selectors (must not contain `]`, `.`, or `=`).
pub fn validate_path_element_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("list element id must not be empty".into());
    }
    if id.contains([']', '.', '=']) {
        return Err(format!("list element id '{id}' must not contain ']', '.', or '='"));
    }
    Ok(())
}

/// 🧭 Parses `elements[2].layers[id=L1].thicknessM` into segments (`[index]` or `[id=…]`).
pub fn parse_path(path: &str) -> Result<Vec<PathSegment>, String> {
    let mut segments = Vec::new();
    let mut rest = path.trim();
    if rest.is_empty() {
        return Err("path must not be empty".into());
    }
    while !rest.is_empty() {
        if rest.starts_with('.') {
            rest = &rest[1..];
        }
        if rest.starts_with('[') {
            let close = rest.find(']').ok_or_else(|| format!("unclosed selector in path '{path}'"))?;
            let inner = &rest[1..close];
            if let Some(id) = inner.strip_prefix("id=") {
                validate_path_element_id(id).map_err(|error| format!("{error} in path '{path}'"))?;
                segments.push(PathSegment::Id(id.to_string()));
            } else if inner.chars().all(|c| c.is_ascii_digit()) && !inner.is_empty() {
                let index: usize = inner.parse().map_err(|_| format!("invalid index in path '{path}'"))?;
                segments.push(PathSegment::Index(index));
            } else {
                return Err(format!("malformed list selector '[{inner}]' in path '{path}' (expected [index] or [id=…])"));
            }
            rest = &rest[close + 1..];
            continue;
        }
        let end = rest.find(['.', '[']).unwrap_or(rest.len());
        let field = &rest[..end];
        if field.is_empty() {
            return Err(format!("empty field in path '{path}'"));
        }
        segments.push(PathSegment::Field(field.to_string()));
        rest = &rest[end..];
    }
    Ok(segments)
}

pub(super) fn object_id_field(value: &semio_framework_value::DslValue) -> Option<&str> {
    let semio_framework_value::DslValue::Object(entries) = value else {
        return None;
    };
    entries.iter().find(|(key, _)| key == "id").and_then(|(_, value)| match value {
        semio_framework_value::DslValue::String(raw) => Some(raw.as_str()),
        _ => None,
    })
}

/// 🧭 Formats a list-element path segment: `[id=…]` when the element has a string `id`, else `[index]`.
pub fn list_element_selector(item: &semio_framework_value::DslValue, index: usize) -> String {
    match object_id_field(item).and_then(|id| validate_path_element_id(id).ok().map(|_| id)) {
        Some(id) => format!("[id={id}]"),
        None => format!("[{index}]"),
    }
}

/// 🧭 Joins an array path with a list-element selector.
pub fn list_element_path(array_path: &str, item: &semio_framework_value::DslValue, index: usize) -> String {
    format!("{array_path}{}", list_element_selector(item, index))
}

pub(super) fn resolve_list_index(items: &[semio_framework_value::DslValue], segment: &PathSegment, path_hint: &str) -> Result<usize, String> {
    match segment {
        PathSegment::Index(index) => {
            if *index >= items.len() {
                return Err(format!("index {index} out of range in path '{path_hint}'"));
            }
            Ok(*index)
        }
        PathSegment::Id(id) => {
            let mut found = None;
            for (index, item) in items.iter().enumerate() {
                if object_id_field(item) == Some(id.as_str()) {
                    if found.is_some() {
                        return Err(format!("duplicate list element id '{id}' in path '{path_hint}'"));
                    }
                    found = Some(index);
                }
            }
            found.ok_or_else(|| format!("unknown list element id '{id}' in path '{path_hint}'"))
        }
        PathSegment::Field(name) => Err(format!("expected list selector before continuing past field '{name}' in path '{path_hint}'")),
    }
}

fn value_at_mut<'a>(root: &'a mut semio_framework_value::DslValue, segments: &[PathSegment], path_hint: &str) -> Result<&'a mut semio_framework_value::DslValue, String> {
    let mut cursor = root;
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                let semio_framework_value::DslValue::Object(entries) = cursor else {
                    return Err(format!("expected object before field '{name}' in path '{path_hint}'"));
                };
                let Some(index) = entries.iter().position(|(key, _)| key == name) else {
                    return Err(format!("missing field '{name}' in path '{path_hint}'"));
                };
                cursor = &mut entries[index].1;
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                cursor = resolve_collection_mut(cursor, segment, path_hint)?;
            }
        }
    }
    Ok(cursor)
}

fn value_at<'a>(root: &'a semio_framework_value::DslValue, segments: &[PathSegment], path_hint: &str) -> Result<&'a semio_framework_value::DslValue, String> {
    let mut cursor = root;
    for segment in segments {
        match segment {
            PathSegment::Field(name) => {
                let semio_framework_value::DslValue::Object(entries) = cursor else {
                    return Err(format!("expected object before field '{name}' in path '{path_hint}'"));
                };
                let Some((_, value)) = entries.iter().find(|(key, _)| key == name) else {
                    return Err(format!("missing field '{name}' in path '{path_hint}'"));
                };
                cursor = value;
            }
            PathSegment::Index(_) | PathSegment::Id(_) => {
                cursor = resolve_collection(cursor, segment, path_hint)?;
            }
        }
    }
    Ok(cursor)
}

/// 🧭 Resolve `[index]` / `[id=…]` on arrays, or `[id=…]` on map objects keyed by entity id.
fn resolve_collection_mut<'a>(cursor: &'a mut semio_framework_value::DslValue, segment: &PathSegment, path_hint: &str) -> Result<&'a mut semio_framework_value::DslValue, String> {
    match cursor {
        semio_framework_value::DslValue::Array(items) => {
            let index = resolve_list_index(items, segment, path_hint)?;
            Ok(&mut items[index])
        }
        semio_framework_value::DslValue::Object(entries) => {
            let PathSegment::Id(id) = segment else {
                return Err(format!("expected array before numeric index in path '{path_hint}'"));
            };
            if let Some(index) = entries.iter().position(|(key, _)| key == id) {
                return Ok(&mut entries[index].1);
            }
            if let Some(index) = entries.iter().position(|(_, value)| object_id_field(value) == Some(id.as_str())) {
                return Ok(&mut entries[index].1);
            }
            Err(format!("unknown map/list element id '{id}' in path '{path_hint}'"))
        }
        _ => Err(format!("expected array or map before list selector in path '{path_hint}'")),
    }
}

fn resolve_collection<'a>(cursor: &'a semio_framework_value::DslValue, segment: &PathSegment, path_hint: &str) -> Result<&'a semio_framework_value::DslValue, String> {
    match cursor {
        semio_framework_value::DslValue::Array(items) => {
            let index = resolve_list_index(items, segment, path_hint)?;
            Ok(&items[index])
        }
        semio_framework_value::DslValue::Object(entries) => {
            let PathSegment::Id(id) = segment else {
                return Err(format!("expected array before numeric index in path '{path_hint}'"));
            };
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == id) {
                return Ok(value);
            }
            if let Some((_, value)) = entries.iter().find(|(_, value)| object_id_field(value) == Some(id.as_str())) {
                return Ok(value);
            }
            Err(format!("unknown map/list element id '{id}' in path '{path_hint}'"))
        }
        _ => Err(format!("expected array or map before list selector in path '{path_hint}'")),
    }
}

/// 🔎 Reads the value at `path` (supports `[index]` and `[id=…]`).
pub fn get_value_at_path<'a>(root: &'a semio_framework_value::DslValue, path: &str) -> Result<&'a semio_framework_value::DslValue, String> {
    let segments = parse_path(path)?;
    value_at(root, &segments, path)
}

/// ✏️ Sets the value at `path` inside a camelCase document value tree.
pub fn set_value_at_path(root: &mut semio_framework_value::DslValue, path: &str, value: semio_framework_value::DslValue) -> Result<(), String> {
    let segments = parse_path(path)?;
    *value_at_mut(root, &segments, path)? = value;
    Ok(())
}

/// ➕ Inserts `value` (or null) into the array at `path` at `index` (append when index ≥ len).
///
/// `path` must address the array itself (not a list element selector).
pub fn insert_value_at_path(root: &mut semio_framework_value::DslValue, path: &str, index: usize, value: Option<semio_framework_value::DslValue>) -> Result<(), String> {
    let segments = parse_path(path)?;
    if matches!(segments.last(), Some(PathSegment::Index(_) | PathSegment::Id(_))) {
        return Err(format!("insertItem path '{path}' must address an array, not a list element"));
    }
    let slot = value_at_mut(root, &segments, path)?;
    let semio_framework_value::DslValue::Array(items) = slot else {
        return Err(format!("insertItem path '{path}' must address an array"));
    };
    let insert_at = index.min(items.len());
    items.insert(insert_at, value.unwrap_or(semio_framework_value::DslValue::Null));
    Ok(())
}

/// ➖ Removes a list element.
///
/// - `path` ends with `[index]` / `[id=…]` → remove that element (the `index` arg is ignored).
/// - `path` addresses an array → remove at `index`.
pub fn remove_value_at_path(root: &mut semio_framework_value::DslValue, path: &str, index: usize) -> Result<(), String> {
    let segments = parse_path(path)?;
    if let Some(PathSegment::Index(_) | PathSegment::Id(_)) = segments.last() {
        let (parent, last) = segments.split_at(segments.len() - 1);
        let slot = if parent.is_empty() {
            root
        } else {
            value_at_mut(root, parent, path)?
        };
        let semio_framework_value::DslValue::Array(items) = slot else {
            return Err(format!("removeItem path '{path}' must address a list element"));
        };
        let remove_at = resolve_list_index(items, &last[0], path)?;
        items.remove(remove_at);
        return Ok(());
    }
    let slot = value_at_mut(root, &segments, path)?;
    let semio_framework_value::DslValue::Array(items) = slot else {
        return Err(format!("removeItem path '{path}' must address an array"));
    };
    if index >= items.len() {
        return Err(format!("removeItem index {index} out of range for '{path}'"));
    }
    items.remove(index);
    Ok(())
}
//#endregion 🔖️ValuePath
