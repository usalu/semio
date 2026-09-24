/// 🧾️ One tree object as a fixed member list: every listed member present, a missing one at its
/// declared default, so a writer that spells a default out and one that omits it project the same.
#[cfg(feature = "oracles")]
fn normalized(item: &json::JsonValue, members: &[(&str, Json)]) -> Json {
    Json::Object(members.iter().map(|(key, default)| ((*key).to_string(), obj_get(item, key).map(to_host_json).unwrap_or_else(|| default.clone()))).collect())
}

/// 🔤️ A semantic → accessor map as ordered `[semantic, accessor]` pairs, so key order is observed.
#[cfg(feature = "oracles")]
fn semantic_pairs(map: Option<&json::JsonValue>) -> Json {
    match map {
        Some(json::JsonValue::Object(object)) => Json::Array(object.iter().map(|(key, value)| Json::Array(vec![Json::String(key.to_string()), to_host_json(value)])).collect()),
        _ => Json::Array(Vec::new()),
    }
}

/// 💾️ One buffer's bytes as lowercase hex: a `data:` URI decoded, the binary chunk for a
/// URI-less first buffer, `null` for an external URI this reader does not follow.
#[cfg(feature = "oracles")]
fn buffer_bytes(buffer: &json::JsonValue, index: usize, bin: Option<&[u8]>) -> Json {
    let bytes = match obj_get(buffer, "uri").and_then(json::JsonValue::as_str) {
        Some(uri) => uri.strip_prefix("data:").and_then(|rest| rest.split_once(";base64,")).and_then(|(_, payload)| base64_decode(payload)),
        None if index == 0 => bin.map(|data| data[..obj_get(buffer, "byteLength").and_then(json::JsonValue::as_usize).unwrap_or(data.len()).min(data.len())].to_vec()),
        None => None,
    };
    bytes.map(|data| Json::String(data.iter().map(|byte| format!("{byte:02x}")).collect())).unwrap_or(Json::Null)
}
