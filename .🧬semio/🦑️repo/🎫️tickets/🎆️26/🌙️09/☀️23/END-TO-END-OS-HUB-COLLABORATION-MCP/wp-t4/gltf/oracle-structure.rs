
//#region 🔖️StructureKinds
/// 🧭️ The 76 kinds of the `🎬️scene`, `💿️buffer` and `🕸️mesh` catalogs, each performed on the
/// independently parsed JSON tree. Every top-level family change re-derives, from the format's own
/// reference rules, which sites point into that family and how they must track the change:
/// scene roots, node children, skin skeleton/joints and animation channel targets point at nodes;
/// node meshes point at meshes; primitive attributes/indices/morph targets, skin inverse-bind
/// matrices and animation sampler input/output point at accessors; accessor and image buffer views
/// point at buffer views; buffer views point at buffers. A dropped optional reference is removed, a
/// dropped required one (sampler input/output, sparse views, a view's buffer) refuses the change.
#[cfg(feature = "oracles")]
fn remap_optional_field(item: &mut json::JsonValue, key: &str, change: &IndexChange) {
    if let Some(value) = obj_get(item, key).and_then(json::JsonValue::as_usize) {
        match remap_index(change, value) {
            Some(mapped) => obj_set(item, key, json::JsonValue::from(mapped)),
            None => remove_key(item, key),
        }
    }
}

/// 🔀️ A required index field: a change that drops its target refuses instead of detaching it.
#[cfg(feature = "oracles")]
fn remap_required_field(item: &mut json::JsonValue, key: &str, change: &IndexChange, path: &str) -> Result<(), String> {
    if let Some(value) = obj_get(item, key).and_then(json::JsonValue::as_usize) {
        let mapped = remap_index(change, value).ok_or_else(|| format!("{path}: the change would break a required reference"))?;
        obj_set(item, key, json::JsonValue::from(mapped));
    }
    Ok(())
}

/// 🔀️ An index list field (`children`, scene `nodes`, skin `joints`): dropped entries leave the list.
#[cfg(feature = "oracles")]
fn remap_list_field(item: &mut json::JsonValue, key: &str, change: &IndexChange) {
    if let Some(items) = arr_mut(item, key) {
        *items = items.iter().filter_map(json::JsonValue::as_usize).filter_map(|value| remap_index(change, value)).map(json::JsonValue::from).collect();
    }
}

/// 🔀️ A semantic → accessor map (`attributes`, one morph target): dropped entries leave the map,
/// every other entry keeps its position.
#[cfg(feature = "oracles")]
fn remap_semantic_map(map: &mut json::JsonValue, change: &IndexChange) {
    if let json::JsonValue::Object(object) = map {
        let mut next = json::object::Object::new();
        for (semantic, value) in object.iter() {
            if let Some(mapped) = value.as_usize().and_then(|index| remap_index(change, index)) {
                next.insert(semantic, json::JsonValue::from(mapped));
            }
        }
        *object = next;
    }
}

/// ✂️ Removes one object member; a no-op when absent.
#[cfg(feature = "oracles")]
fn remove_key(item: &mut json::JsonValue, key: &str) {
    if let json::JsonValue::Object(object) = item {
        object.remove(key);
    }
}

/// 🔀️ Every reference site of one top-level family, moved with `change`.
#[cfg(feature = "oracles")]
fn remap_family_references(doc: &mut json::JsonValue, family: &str, change: &IndexChange) -> Result<(), String> {
    match family {
        "scenes" => remap_optional_field(doc, "scene", change),
        "nodes" => {
            for scene in arr_mut(doc, "scenes").into_iter().flatten() {
                remap_list_field(scene, "nodes", change);
            }
            for node in arr_mut(doc, "nodes").into_iter().flatten() {
                remap_list_field(node, "children", change);
            }
            for skin in arr_mut(doc, "skins").into_iter().flatten() {
                remap_optional_field(skin, "skeleton", change);
                remap_list_field(skin, "joints", change);
            }
            for animation in arr_mut(doc, "animations").into_iter().flatten() {
                for channel in arr_mut(animation, "channels").into_iter().flatten() {
                    if let json::JsonValue::Object(object) = channel {
                        if let Some(target) = object.get_mut("target") {
                            remap_optional_field(target, "node", change);
                        }
                    }
                }
            }
        }
        "meshes" => {
            for node in arr_mut(doc, "nodes").into_iter().flatten() {
                remap_optional_field(node, "mesh", change);
            }
        }
        "accessors" => {
            for mesh in arr_mut(doc, "meshes").into_iter().flatten() {
                for primitive in arr_mut(mesh, "primitives").into_iter().flatten() {
                    if let json::JsonValue::Object(object) = primitive {
                        if let Some(attributes) = object.get_mut("attributes") {
                            remap_semantic_map(attributes, change);
                        }
                    }
                    remap_optional_field(primitive, "indices", change);
                    for target in arr_mut(primitive, "targets").into_iter().flatten() {
                        remap_semantic_map(target, change);
                    }
                }
            }
            for skin in arr_mut(doc, "skins").into_iter().flatten() {
                remap_optional_field(skin, "inverseBindMatrices", change);
            }
            for animation in arr_mut(doc, "animations").into_iter().flatten() {
                for sampler in arr_mut(animation, "samplers").into_iter().flatten() {
                    remap_required_field(sampler, "input", change, "document/animations")?;
                    remap_required_field(sampler, "output", change, "document/animations")?;
                }
            }
        }
        "bufferViews" => {
            for accessor in arr_mut(doc, "accessors").into_iter().flatten() {
                remap_optional_field(accessor, "bufferView", change);
                if let json::JsonValue::Object(object) = accessor {
                    if let Some(sparse) = object.get_mut("sparse") {
                        for part in ["indices", "values"] {
                            if let json::JsonValue::Object(sparse_object) = sparse {
                                if let Some(section) = sparse_object.get_mut(part) {
                                    remap_required_field(section, "bufferView", change, "document/accessors")?;
                                }
                            }
                        }
                    }
                }
            }
            for image in arr_mut(doc, "images").into_iter().flatten() {
                remap_optional_field(image, "bufferView", change);
            }
        }
        "buffers" => {
            for view in arr_mut(doc, "bufferViews").into_iter().flatten() {
                remap_required_field(view, "buffer", change, "document/bufferViews")?;
            }
        }
        other => return Err(format!("no reference rule for top-level family {other}")),
    }
    Ok(())
}

/// 🔢️ `order` is a genuine permutation of `0..total` that moves at least one entry.
#[cfg(feature = "oracles")]
fn checked_permutation(kind: &str, order: &[usize], total: usize) -> Result<(), String> {
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err(format!("{kind}: order must contain every index exactly once"));
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(format!("{kind}: order already matches"));
    }
    Ok(())
}

/// 🌱️ Inserts `value` at `position` of top-level `family`, moving every reference first.
#[cfg(feature = "oracles")]
fn family_insert(doc: &mut json::JsonValue, kind: &str, family: &str, position: usize, value: json::JsonValue) -> Result<(), String> {
    if position > top_level_len(doc, family) {
        return Err(format!("{kind}: position {position} out of range"));
    }
    remap_family_references(doc, family, &IndexChange::Insert(position))?;
    ensure_array(doc, family).insert(position, value);
    Ok(())
}

/// 🗑️ Removes entry `index` of top-level `family`, dropping or refusing every reference to it.
#[cfg(feature = "oracles")]
fn family_delete(doc: &mut json::JsonValue, kind: &str, family: &str, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, family) {
        return Err(format!("{kind}: index {index} out of range"));
    }
    remap_family_references(doc, family, &IndexChange::Delete(index))?;
    ensure_array(doc, family).remove(index);
    Ok(())
}

/// 🚚️ Moves entry `index` of top-level `family` to `position`, moving every reference with it.
#[cfg(feature = "oracles")]
fn family_move(doc: &mut json::JsonValue, kind: &str, family: &str, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, family);
    if index >= total || position >= total {
        return Err(format!("{kind}: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    remap_family_references(doc, family, &IndexChange::Move(index, position))?;
    let items = ensure_array(doc, family);
    let value = items.remove(index);
    items.insert(position, value);
    Ok(())
}

/// 🔀️ Permutes top-level `family` by `order`, moving every reference with it.
#[cfg(feature = "oracles")]
fn family_reorder(doc: &mut json::JsonValue, kind: &str, family: &str, order: &[usize]) -> Result<(), String> {
    checked_permutation(kind, order, top_level_len(doc, family))?;
    remap_family_references(doc, family, &IndexChange::Reorder(order))?;
    let items = ensure_array(doc, family);
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| items[*index].clone()).collect();
    *items = reordered;
    Ok(())
}

/// 🔎️ `document/<family>/<index>`, mutable, or a range error.
#[cfg(feature = "oracles")]
fn entry_mut<'a>(doc: &'a mut json::JsonValue, kind: &str, family: &str, index: usize) -> Result<&'a mut json::JsonValue, String> {
    arr_mut(doc, family).and_then(|items| items.get_mut(index)).ok_or_else(|| format!("{kind}: {family} {index} out of range"))
}

/// 🔎️ `document/meshes/<mesh>/primitives/<primitive>`, mutable, or a range error.
#[cfg(feature = "oracles")]
fn primitive_mut<'a>(doc: &'a mut json::JsonValue, kind: &str, mesh: usize, primitive: usize) -> Result<&'a mut json::JsonValue, String> {
    let mesh_value = entry_mut(doc, kind, "meshes", mesh)?;
    arr_mut(mesh_value, "primitives").and_then(|items| items.get_mut(primitive)).ok_or_else(|| format!("{kind}: primitive {primitive} of mesh {mesh} out of range"))
}

/// 🔎️ One morph target of one primitive, mutable, or a range error.
#[cfg(feature = "oracles")]
fn target_mut<'a>(doc: &'a mut json::JsonValue, kind: &str, mesh: usize, primitive: usize, target: usize) -> Result<&'a mut json::JsonValue, String> {
    let primitive_value = primitive_mut(doc, kind, mesh, primitive)?;
    arr_mut(primitive_value, "targets").and_then(|items| items.get_mut(target)).ok_or_else(|| format!("{kind}: target {target} out of range"))
}

/// 🔧️ Sets `key` to `value`, or removes it when `value` is `None`.
#[cfg(feature = "oracles")]
fn set_or_remove(item: &mut json::JsonValue, key: &str, value: Option<json::JsonValue>) {
    match value {
        Some(value) => obj_set(item, key, value),
        None => remove_key(item, key),
    }
}

/// 🧩️ A `{state: present, value}` / `{state: absent}` payload member, as the tree value to store.
#[cfg(feature = "oracles")]
fn presence_param(params: &Json) -> Result<Option<json::JsonValue>, String> {
    let data = object_param(params, "data")?;
    match data.get("state") {
        Some(Json::String(state)) if state == "absent" => Ok(None),
        Some(Json::String(state)) if state == "present" => data.get("value").map(|value| Some(from_host_json(value))).ok_or_else(|| "present data carries no `value`".to_string()),
        _ => Err("`data.state` must be `present` or `absent`".to_string()),
    }
}

/// 📝️ A nullable string payload member.
#[cfg(feature = "oracles")]
fn nullable_str_param(params: &Json, key: &str) -> Result<Option<String>, String> {
    match params.get(key) {
        Some(Json::String(value)) => Ok(Some(value.clone())),
        Some(Json::Null) | None => Ok(None),
        _ => Err(format!("`{key}` must be a string or null")),
    }
}

/// 🔢️ A number array payload member.
#[cfg(feature = "oracles")]
fn f64_array_param(params: &Json, key: &str) -> Result<Vec<f64>, String> {
    match params.get(key) {
        Some(Json::Array(items)) => items
            .iter()
            .map(|item| match item {
                Json::Number(number) => Ok(*number),
                _ => Err(format!("`{key}` must hold only numbers")),
            })
            .collect(),
        _ => Err(format!("missing or non-array `{key}`")),
    }
}

/// 🔢️ `weights` as the tree stores it: an empty list is no member at all.
#[cfg(feature = "oracles")]
fn weights_value(weights: &[f64]) -> Option<json::JsonValue> {
    (!weights.is_empty()).then(|| json::JsonValue::Array(weights.iter().map(|value| json::JsonValue::from(*value)).collect()))
}

/// 🔢️ Every primitive of `mesh` carries exactly `count` morph targets.
#[cfg(feature = "oracles")]
fn mesh_target_arity_is(doc: &json::JsonValue, mesh: usize, count: usize) -> bool {
    arr(doc, "meshes").get(mesh).is_some_and(|mesh_value| arr(mesh_value, "primitives").iter().all(|primitive| arr(primitive, "targets").len() == count))
}

/// 🚚️ Moves one key of a semantic map to `position`, keeping every other key's order.
#[cfg(feature = "oracles")]
fn move_semantic(map: &mut json::JsonValue, kind: &str, semantic: &str, position: usize) -> Result<(), String> {
    let json::JsonValue::Object(object) = map else { return Err(format!("{kind}: not a semantic map")) };
    let mut entries: Vec<(String, json::JsonValue)> = object.iter().map(|(key, value)| (key.to_string(), value.clone())).collect();
    let index = entries.iter().position(|(key, _)| key == semantic).ok_or_else(|| format!("{kind}: semantic {semantic} is not bound"))?;
    if position >= entries.len() {
        return Err(format!("{kind}: position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    let entry = entries.remove(index);
    entries.insert(position, entry);
    let mut next = json::object::Object::new();
    for (key, value) in entries {
        next.insert(&key, value);
    }
    *object = next;
    Ok(())
}

/// 🔀️ Rebuilds a semantic map in `order`, which must name every bound semantic once.
#[cfg(feature = "oracles")]
fn reorder_semantics(map: &mut json::JsonValue, kind: &str, order: &[String]) -> Result<(), String> {
    let json::JsonValue::Object(object) = map else { return Err(format!("{kind}: not a semantic map")) };
    let unique: std::collections::BTreeSet<&String> = order.iter().collect();
    if order.len() != object.len() || unique.len() != order.len() || order.iter().any(|semantic| object.get(semantic).is_none()) {
        return Err(format!("{kind}: order must contain every semantic once"));
    }
    let mut next = json::object::Object::new();
    for semantic in order {
        next.insert(semantic, object.get(semantic).cloned().expect("validated semantic"));
    }
    *object = next;
    Ok(())
}

/// 🔗️ Appends `semantic → accessor` to a semantic map; the semantic must be new and non-blank.
#[cfg(feature = "oracles")]
fn bind_semantic(map: &mut json::JsonValue, kind: &str, semantic: &str, accessor: usize) -> Result<(), String> {
    if semantic.trim().is_empty() || obj_get(map, semantic).is_some() {
        return Err(format!("{kind}: semantic must be non-empty and unique"));
    }
    if !matches!(map, json::JsonValue::Object(_)) {
        *map = json::JsonValue::Object(json::object::Object::new());
    }
    obj_set(map, semantic, json::JsonValue::from(accessor));
    Ok(())
}

/// ✂️ Removes a bound semantic from a semantic map.
#[cfg(feature = "oracles")]
fn unbind_semantic(map: &mut json::JsonValue, kind: &str, semantic: &str) -> Result<(), String> {
    if obj_get(map, semantic).is_none() {
        return Err(format!("{kind}: semantic {semantic} is not bound"));
    }
    remove_key(map, semantic);
    Ok(())
}

/// 🧮️ The node transform a `{kind: trs | matrix, …}` payload writes: a matrix clears TRS and
/// TRS clears the matrix, each absent TRS component removed.
#[cfg(feature = "oracles")]
fn change_node_transform(doc: &mut json::JsonValue, node: usize, transform: &Json) -> Result<(), String> {
    let kind = "change-node-transform";
    let numbers = |key: &str, length: usize| -> Result<Option<json::JsonValue>, String> {
        match transform.get(key) {
            None | Some(Json::Null) => Ok(None),
            Some(Json::Array(items)) if items.len() == length => items
                .iter()
                .map(|item| match item {
                    Json::Number(number) if number.is_finite() => Ok(json::JsonValue::from(*number)),
                    _ => Err(format!("{kind}: {key} must hold finite numbers")),
                })
                .collect::<Result<Vec<_>, String>>()
                .map(|values| Some(json::JsonValue::Array(values))),
            _ => Err(format!("{kind}: {key} must hold {length} numbers")),
        }
    };
    let (matrix, translation, rotation, scale) = match transform.get("kind") {
        Some(Json::String(mode)) if mode == "matrix" => (numbers("matrix", 16)?.ok_or(format!("{kind}: matrix missing"))?.into(), None, None, None),
        Some(Json::String(mode)) if mode == "trs" => (None, numbers("translation", 3)?, numbers("rotation", 4)?, numbers("scale", 3)?),
        _ => return Err(format!("{kind}: transform kind must be `trs` or `matrix`")),
    };
    let node_value = entry_mut(doc, kind, "nodes", node)?;
    set_or_remove(node_value, "matrix", matrix);
    set_or_remove(node_value, "translation", translation);
    set_or_remove(node_value, "rotation", rotation);
    set_or_remove(node_value, "scale", scale);
    Ok(())
}

/// 🌿️ `move-node-parent` — detaches the child from every parent and every scene root, then
/// inserts it under the new parent at `position`, refusing a cycle.
#[cfg(feature = "oracles")]
fn move_node_parent(doc: &mut json::JsonValue, parent: usize, child: usize, position: usize) -> Result<(), String> {
    let kind = "move-node-parent";
    let total = top_level_len(doc, "nodes");
    if parent >= total || child >= total {
        return Err(format!("{kind}: node out of range"));
    }
    if parent == child {
        return Err(format!("{kind}: a node cannot parent itself"));
    }
    let mut pending = vec![child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(current) = pending.pop() {
        if current == parent {
            return Err(format!("{kind}: relationship closes a cycle"));
        }
        if seen.insert(current) {
            pending.extend(node_children(doc, current));
        }
    }
    if position > node_children(doc, parent).iter().filter(|value| **value != child).count() {
        return Err(format!("{kind}: position out of range"));
    }
    for node in arr_mut(doc, "nodes").into_iter().flatten() {
        if let Some(children) = arr_mut(node, "children") {
            children.retain(|value| value.as_usize() != Some(child));
        }
    }
    for scene in arr_mut(doc, "scenes").into_iter().flatten() {
        if let Some(roots) = arr_mut(scene, "nodes") {
            roots.retain(|value| value.as_usize() != Some(child));
        }
    }
    ensure_array(entry_mut(doc, kind, "nodes", parent)?, "children").insert(position, json::JsonValue::from(child));
    Ok(())
}

/// 🚚️ Moves one element of an index list (`children`, scene `nodes`) to `position`.
#[cfg(feature = "oracles")]
fn move_in_index_list(list: &mut Vec<json::JsonValue>, kind: &str, value: usize, position: usize) -> Result<(), String> {
    let index = list.iter().position(|item| item.as_usize() == Some(value)).ok_or_else(|| format!("{kind}: {value} is not linked"))?;
    if position >= list.len() {
        return Err(format!("{kind}: position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    let item = list.remove(index);
    list.insert(position, item);
    Ok(())
}

/// 🔀️ Replaces an index list with `order`, which must name every current member once and differ.
#[cfg(feature = "oracles")]
fn reorder_index_list(list: &mut Vec<json::JsonValue>, kind: &str, order: &[usize]) -> Result<(), String> {
    let current: Vec<usize> = list.iter().filter_map(json::JsonValue::as_usize).collect();
    let mut sorted_order = order.to_vec();
    sorted_order.sort_unstable();
    sorted_order.dedup();
    let mut sorted_current = current.clone();
    sorted_current.sort_unstable();
    if order.len() != current.len() || sorted_order != sorted_current {
        return Err(format!("{kind}: order must contain every member once"));
    }
    if order == current.as_slice() {
        return Err(format!("{kind}: reorder must change order"));
    }
    *list = order.iter().map(|value| json::JsonValue::from(*value)).collect();
    Ok(())
}

/// 🔀️ Permutes a nested array (`primitives`, `targets`) by `order`.
#[cfg(feature = "oracles")]
fn reorder_nested(list: &mut Vec<json::JsonValue>, kind: &str, order: &[usize], require_change: bool) -> Result<(), String> {
    let total = list.len();
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err(format!("{kind}: order must contain every index exactly once"));
    }
    if require_change && order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(format!("{kind}: reorder must change order"));
    }
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| list[*index].clone()).collect();
    *list = reordered;
    Ok(())
}

/// 🚚️ Moves one element of a nested array to `position`.
#[cfg(feature = "oracles")]
fn move_nested(list: &mut Vec<json::JsonValue>, kind: &str, index: usize, position: usize) -> Result<(), String> {
    if index >= list.len() || position >= list.len() {
        return Err(format!("{kind}: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    let item = list.remove(index);
    list.insert(position, item);
    Ok(())
}

/// 🔤️ RFC 4648 base64 with padding, the encoding a `data:` buffer URI carries.
#[cfg(feature = "oracles")]
fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let triple = (u32::from(chunk[0]) << 16) | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8) | u32::from(*chunk.get(2).unwrap_or(&0));
        for (position, shift) in [18u32, 12, 6, 0].into_iter().enumerate() {
            if position <= chunk.len() {
                out.push(ALPHABET[((triple >> shift) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// 🔤️ RFC 4648 base64 decode, padding optional; `None` on any foreign character.
#[cfg(feature = "oracles")]
fn base64_decode(text: &str) -> Option<Vec<u8>> {
    let value = |byte: u8| -> Option<u32> {
        match byte {
            b'A'..=b'Z' => Some(u32::from(byte - b'A')),
            b'a'..=b'z' => Some(u32::from(byte - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(byte - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    };
    let digits: Vec<u8> = text.bytes().filter(|byte| *byte != b'=').collect();
    let mut out = Vec::with_capacity(digits.len() * 3 / 4);
    for chunk in digits.chunks(4) {
        let mut accumulator = 0u32;
        for (index, byte) in chunk.iter().enumerate() {
            accumulator |= value(*byte)? << (18 - 6 * index as u32);
        }
        out.extend_from_slice(&accumulator.to_be_bytes()[1..chunk.len()]);
    }
    Some(out)
}

/// 🦠️ Dispatches one of the 76 structure kinds; `None` when `kind` is not one of them.
#[cfg(feature = "oracles")]
fn apply_structure(doc: &mut json::JsonValue, kind: &str, params: &Json) -> Option<Result<(), String>> {
    let run = || -> Result<(), String> {
        match kind {
            "bind-default-scene" => {
                let scene = usize_param(params, "scene")?;
                if scene >= top_level_len(doc, "scenes") {
                    return Err(format!("{kind}: scene {scene} out of range"));
                }
                if default_scene_index(doc) == Some(scene) {
                    return Err(format!("{kind}: scene is already default"));
                }
                obj_set(doc, "scene", json::JsonValue::from(scene));
            }
            "unbind-default-scene" => {
                if default_scene_index(doc).is_none() {
                    return Err(format!("{kind}: no default scene is bound"));
                }
                remove_key(doc, "scene");
            }
            "bind-node-camera" | "bind-node-mesh" | "bind-node-skin" => {
                let (field, family) = match kind {
                    "bind-node-camera" => ("camera", "cameras"),
                    "bind-node-mesh" => ("mesh", "meshes"),
                    _ => ("skin", "skins"),
                };
                let target = usize_param(params, field)?;
                if target >= top_level_len(doc, family) {
                    return Err(format!("{kind}: {field} {target} out of range"));
                }
                obj_set(entry_mut(doc, kind, "nodes", usize_param(params, "node")?)?, field, json::JsonValue::from(target));
            }
            "unbind-node-camera" | "unbind-node-mesh" | "unbind-node-skin" => {
                let field = kind.trim_start_matches("unbind-node-");
                let node = entry_mut(doc, kind, "nodes", usize_param(params, "node")?)?;
                if obj_get(node, field).is_none() {
                    return Err(format!("{kind}: node has no binding"));
                }
                remove_key(node, field);
            }
            "bind-node-child" => bind_node_child(doc, usize_param(params, "parent")?, usize_param(params, "child")?, usize_param(params, "position")?)?,
            "unbind-node-child" => unbind_node_child(doc, usize_param(params, "parent")?, usize_param(params, "child")?)?,
            "bind-scene-root-node" => bind_scene_root_node(doc, usize_param(params, "scene")?, usize_param(params, "node")?, usize_param(params, "position")?)?,
            "unbind-scene-root-node" => unbind_scene_root_node(doc, usize_param(params, "scene")?, usize_param(params, "node")?)?,
            "move-node-child" => {
                let child = usize_param(params, "child")?;
                let position = usize_param(params, "position")?;
                let parent = entry_mut(doc, kind, "nodes", usize_param(params, "parent")?)?;
                move_in_index_list(ensure_array(parent, "children"), kind, child, position)?;
            }
            "reorder-node-children" => {
                let order = usize_array_param(params, "order")?;
                let parent = entry_mut(doc, kind, "nodes", usize_param(params, "parent")?)?;
                reorder_index_list(ensure_array(parent, "children"), kind, &order)?;
            }
            "move-scene-root-node" => {
                let node = usize_param(params, "node")?;
                let position = usize_param(params, "position")?;
                let scene = entry_mut(doc, kind, "scenes", usize_param(params, "scene")?)?;
                move_in_index_list(ensure_array(scene, "nodes"), kind, node, position)?;
            }
            "reorder-scene-root-nodes" => {
                let order = usize_array_param(params, "order")?;
                let scene = entry_mut(doc, kind, "scenes", usize_param(params, "scene")?)?;
                reorder_index_list(ensure_array(scene, "nodes"), kind, &order)?;
            }
            "move-node-parent" => move_node_parent(doc, usize_param(params, "parent")?, usize_param(params, "child")?, usize_param(params, "position")?)?,
            "change-node-name" | "change-scene-name" | "change-mesh-name" => {
                let (family, field) = match kind {
                    "change-node-name" => ("nodes", "node"),
                    "change-scene-name" => ("scenes", "scene"),
                    _ => ("meshes", "mesh"),
                };
                let value = nullable_str_param(params, "value")?;
                let item = entry_mut(doc, kind, family, usize_param(params, field)?)?;
                if kind == "change-node-name" && obj_get(item, "name").and_then(json::JsonValue::as_str).map(str::to_string) == value {
                    return Err(format!("{kind}: name already has the requested presence and value"));
                }
                set_or_remove(item, "name", value.map(json::JsonValue::from));
            }
            "change-node-extra-data" | "change-node-extension-data" | "change-scene-extra-data" | "change-scene-extension-data" | "change-mesh-extra-data" | "change-mesh-extension-data" => {
                let (family, field) = match kind.split('-').nth(1) {
                    Some("node") => ("nodes", "node"),
                    Some("scene") => ("scenes", "scene"),
                    _ => ("meshes", "mesh"),
                };
                let key = if kind.ends_with("extra-data") { "extras" } else { "extensions" };
                let value = presence_param(params)?;
                let item = entry_mut(doc, kind, family, usize_param(params, field)?)?;
                if kind == "change-node-extra-data" && obj_get(item, key).cloned() == value {
                    return Err(format!("{kind}: extras already has the requested presence and value"));
                }
                set_or_remove(item, key, value);
            }
            "change-primitive-extra-data" | "change-primitive-extension-data" => {
                let key = if kind.ends_with("extra-data") { "extras" } else { "extensions" };
                let value = presence_param(params)?;
                set_or_remove(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, key, value);
            }
            "change-node-transform" => change_node_transform(doc, usize_param(params, "node")?, object_param(params, "transform")?)?,
            "change-node-morph-weights" => {
                let node = usize_param(params, "node")?;
                let weights = f64_array_param(params, "weights")?;
                if weights.iter().any(|value| !value.is_finite()) {
                    return Err(format!("{kind}: weights must be finite"));
                }
                let mesh = arr(doc, "nodes").get(node).ok_or(format!("{kind}: node {node} out of range"))?.clone();
                match obj_get(&mesh, "mesh").and_then(json::JsonValue::as_usize) {
                    None if !weights.is_empty() => return Err(format!("{kind}: morph weights require a mesh")),
                    Some(mesh_index) if !mesh_target_arity_is(doc, mesh_index, weights.len()) => return Err(format!("{kind}: weights must match primitive target count")),
                    _ => {}
                }
                set_or_remove(entry_mut(doc, kind, "nodes", node)?, "weights", weights_value(&weights));
            }
            "change-mesh-morph-weights" => {
                let mesh = usize_param(params, "mesh")?;
                let weights = f64_array_param(params, "weights")?;
                if weights.iter().any(|value| !value.is_finite()) || !mesh_target_arity_is(doc, mesh, weights.len()) {
                    return Err(format!("{kind}: weights must be finite and match every primitive target list"));
                }
                set_or_remove(entry_mut(doc, kind, "meshes", mesh)?, "weights", weights_value(&weights));
            }
            "create-node" => family_insert(doc, kind, "nodes", usize_param(params, "position")?, json::JsonValue::Object(json::object::Object::new()))?,
            "delete-node" => family_delete(doc, kind, "nodes", usize_param(params, "index")?)?,
            "move-node" => family_move(doc, kind, "nodes", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-nodes" => family_reorder(doc, kind, "nodes", &usize_array_param(params, "order")?)?,
            "create-scene" => create_scene(doc, usize_param(params, "position")?)?,
            "delete-scene" => family_delete(doc, kind, "scenes", usize_param(params, "index")?)?,
            "move-scene" => family_move(doc, kind, "scenes", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-scenes" => family_reorder(doc, kind, "scenes", &usize_array_param(params, "order")?)?,
            "create-mesh" => {
                let mut mesh = json::object::Object::new();
                mesh.insert("primitives", json::JsonValue::Array(Vec::new()));
                family_insert(doc, kind, "meshes", usize_param(params, "position")?, json::JsonValue::Object(mesh))?;
            }
            "delete-mesh" => family_delete(doc, kind, "meshes", usize_param(params, "index")?)?,
            "move-mesh" => family_move(doc, kind, "meshes", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-meshs" => family_reorder(doc, kind, "meshes", &usize_array_param(params, "order")?)?,
            "create-accessor" => {
                let mut accessor = json::object::Object::new();
                accessor.insert("componentType", json::JsonValue::from(usize_param(params, "componentType")?));
                accessor.insert("count", json::JsonValue::from(usize_param(params, "count")?));
                accessor.insert("type", json::JsonValue::from(str_param(params, "kind")?));
                family_insert(doc, kind, "accessors", usize_param(params, "position")?, json::JsonValue::Object(accessor))?;
            }
            "delete-accessor" => family_delete(doc, kind, "accessors", usize_param(params, "index")?)?,
            "move-accessor" => family_move(doc, kind, "accessors", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-accessors" => family_reorder(doc, kind, "accessors", &usize_array_param(params, "order")?)?,
            "create-buffer-view" => {
                let buffer = usize_param(params, "buffer")?;
                if buffer >= top_level_len(doc, "buffers") {
                    return Err(format!("{kind}: backing buffer {buffer} missing"));
                }
                let mut view = json::object::Object::new();
                view.insert("buffer", json::JsonValue::from(buffer));
                view.insert("byteOffset", json::JsonValue::from(usize_param(params, "byteOffset")?));
                view.insert("byteLength", json::JsonValue::from(usize_param(params, "byteLength")?));
                family_insert(doc, kind, "bufferViews", usize_param(params, "position")?, json::JsonValue::Object(view))?;
            }
            "delete-buffer-view" => family_delete(doc, kind, "bufferViews", usize_param(params, "index")?)?,
            "move-buffer-view" => family_move(doc, kind, "bufferViews", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-buffer-views" => family_reorder(doc, kind, "bufferViews", &usize_array_param(params, "order")?)?,
            "create-buffer" => {
                let bytes: Vec<u8> = usize_array_param(params, "bytes")?.into_iter().map(|byte| u8::try_from(byte).map_err(|_| format!("{kind}: byte {byte} out of range"))).collect::<Result<_, _>>()?;
                let mut buffer = json::object::Object::new();
                buffer.insert("byteLength", json::JsonValue::from(bytes.len()));
                buffer.insert("uri", json::JsonValue::from(format!("data:application/octet-stream;base64,{}", base64_encode(&bytes))));
                family_insert(doc, kind, "buffers", usize_param(params, "position")?, json::JsonValue::Object(buffer))?;
            }
            "delete-buffer" => family_delete(doc, kind, "buffers", usize_param(params, "index")?)?,
            "move-buffer" => family_move(doc, kind, "buffers", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-buffers" => family_reorder(doc, kind, "buffers", &usize_array_param(params, "order")?)?,
            "create-primitive" => {
                let position = usize_param(params, "position")?;
                let mesh = entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?;
                let primitives = ensure_array(mesh, "primitives");
                if position > primitives.len() {
                    return Err(format!("{kind}: position {position} out of range"));
                }
                let mut primitive = json::object::Object::new();
                primitive.insert("attributes", json::JsonValue::Object(json::object::Object::new()));
                primitives.insert(position, json::JsonValue::Object(primitive));
            }
            "delete-primitive" => {
                let primitive = usize_param(params, "primitive")?;
                let mesh = entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?;
                let primitives = ensure_array(mesh, "primitives");
                if primitive >= primitives.len() {
                    return Err(format!("{kind}: primitive {primitive} out of range"));
                }
                primitives.remove(primitive);
            }
            "move-primitive" => {
                let (primitive, position) = (usize_param(params, "primitive")?, usize_param(params, "position")?);
                move_nested(ensure_array(entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?, "primitives"), kind, primitive, position)?;
            }
            "reorder-primitives" => {
                let order = usize_array_param(params, "order")?;
                reorder_nested(ensure_array(entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?, "primitives"), kind, &order, true)?;
            }
            "change-primitive-topology-mode" => {
                let mode = usize_param(params, "mode")?;
                if mode > 6 {
                    return Err(format!("{kind}: mode must be in the glTF topology domain"));
                }
                obj_set(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "mode", json::JsonValue::from(mode));
            }
            "bind-primitive-material" => {
                let material = usize_param(params, "material")?;
                if material >= top_level_len(doc, "materials") {
                    return Err(format!("{kind}: material {material} out of range"));
                }
                obj_set(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "material", json::JsonValue::from(material));
            }
            "unbind-primitive-material" | "unbind-primitive-indices" => {
                let field = if kind == "unbind-primitive-material" { "material" } else { "indices" };
                let primitive = primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?;
                if obj_get(primitive, field).is_none() {
                    return Err(format!("{kind}: primitive has no {field}"));
                }
                remove_key(primitive, field);
            }
            "bind-primitive-indices" => {
                let accessor = usize_param(params, "accessor")?;
                let accessor_value = arr(doc, "accessors").get(accessor).cloned().ok_or(format!("{kind}: accessor {accessor} out of range"))?;
                if obj_get(&accessor_value, "type").and_then(json::JsonValue::as_str) != Some("SCALAR") || obj_get(&accessor_value, "componentType").and_then(json::JsonValue::as_usize) == Some(5126) {
                    return Err(format!("{kind}: indices require a scalar integer accessor"));
                }
                obj_set(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "indices", json::JsonValue::from(accessor));
            }
            "bind-primitive-attribute" => {
                let accessor = usize_param(params, "accessor")?;
                if accessor >= top_level_len(doc, "accessors") {
                    return Err(format!("{kind}: accessor {accessor} out of range"));
                }
                let semantic = str_param(params, "semantic")?;
                let primitive = primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?;
                if obj_get(primitive, "attributes").is_none() {
                    obj_set(primitive, "attributes", json::JsonValue::Object(json::object::Object::new()));
                }
                let json::JsonValue::Object(object) = primitive else { return Err(format!("{kind}: primitive is not an object")) };
                bind_semantic(object.get_mut("attributes").expect("attributes ensured"), kind, &semantic, accessor)?;
            }
            "unbind-primitive-attribute" | "move-primitive-attribute" | "reorder-primitive-attributes" => {
                let primitive = primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?;
                let json::JsonValue::Object(object) = primitive else { return Err(format!("{kind}: primitive is not an object")) };
                let attributes = object.get_mut("attributes").ok_or(format!("{kind}: primitive has no attributes"))?;
                match kind {
                    "unbind-primitive-attribute" => unbind_semantic(attributes, kind, &str_param(params, "semantic")?)?,
                    "move-primitive-attribute" => move_semantic(attributes, kind, &str_param(params, "semantic")?, usize_param(params, "position")?)?,
                    _ => reorder_semantics(attributes, kind, &string_array_param(params, "order")?)?,
                }
            }
            "create-morph-target" => {
                let position = usize_param(params, "position")?;
                let mesh = usize_param(params, "mesh")?;
                let primitive_index = usize_param(params, "primitive")?;
                if arr(doc, "meshes").get(mesh).map(|value| arr(value, "primitives").len()) != Some(1) {
                    return Err(format!("{kind}: all primitive target counts must remain coherent"));
                }
                let targets = ensure_array(primitive_mut(doc, kind, mesh, primitive_index)?, "targets");
                if position > targets.len() {
                    return Err(format!("{kind}: position {position} out of range"));
                }
                targets.insert(position, json::JsonValue::Object(json::object::Object::new()));
            }
            "delete-morph-target" => {
                let (mesh, primitive_index, target) = (usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?);
                let mesh_value = arr(doc, "meshes").get(mesh).cloned().ok_or(format!("{kind}: mesh {mesh} out of range"))?;
                if arr(&mesh_value, "primitives").len() != 1 || !arr(&mesh_value, "weights").is_empty() {
                    return Err(format!("{kind}: target deletion would violate mesh target-count coherence"));
                }
                let targets = ensure_array(primitive_mut(doc, kind, mesh, primitive_index)?, "targets");
                if target >= targets.len() {
                    return Err(format!("{kind}: target {target} out of range"));
                }
                targets.remove(target);
            }
            "move-morph-target" => {
                let (target, position) = (usize_param(params, "target")?, usize_param(params, "position")?);
                move_nested(ensure_array(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "targets"), kind, target, position)?;
            }
            "reorder-morph-targets" => {
                let order = usize_array_param(params, "order")?;
                reorder_nested(ensure_array(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "targets"), kind, &order, false)?;
            }
            "bind-morph-target-attribute" => {
                let accessor = usize_param(params, "accessor")?;
                if accessor >= top_level_len(doc, "accessors") {
                    return Err(format!("{kind}: accessor {accessor} out of range"));
                }
                let semantic = str_param(params, "semantic")?;
                bind_semantic(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &semantic, accessor)?;
            }
            "unbind-morph-target-attribute" => {
                let semantic = str_param(params, "semantic")?;
                unbind_semantic(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &semantic)?;
            }
            "move-morph-target-attribute" => {
                let (semantic, position) = (str_param(params, "semantic")?, usize_param(params, "position")?);
                move_semantic(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &semantic, position)?;
            }
            "reorder-morph-target-attributes" => {
                let order = string_array_param(params, "order")?;
                reorder_semantics(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &order)?;
            }
            _ => unreachable!("dispatched only for structure kinds"),
        }
        Ok(())
    };
    STRUCTURE_KINDS.contains(&kind).then(run)
}

/// 🧾️ The 76 kinds [`apply_structure`] performs.
#[cfg(feature = "oracles")]
const STRUCTURE_KINDS: &[&str] = &[
    "bind-default-scene", "bind-node-camera", "bind-node-child", "bind-node-mesh", "bind-node-skin", "bind-scene-root-node", "change-node-extension-data", "change-node-extra-data", "change-node-morph-weights", "change-node-name", "change-node-transform", "change-scene-extension-data", "change-scene-extra-data", "change-scene-name", "create-node", "create-scene", "delete-node", "delete-scene", "move-node", "move-node-child", "move-node-parent", "move-scene", "move-scene-root-node", "reorder-node-children", "reorder-nodes", "reorder-scene-root-nodes", "reorder-scenes", "unbind-default-scene", "unbind-node-camera", "unbind-node-child", "unbind-node-mesh", "unbind-node-skin", "unbind-scene-root-node",
    "create-buffer", "create-buffer-view", "delete-buffer", "delete-buffer-view", "move-buffer", "move-buffer-view", "reorder-buffer-views", "reorder-buffers",
    "bind-morph-target-attribute", "bind-primitive-attribute", "bind-primitive-indices", "bind-primitive-material", "change-mesh-extension-data", "change-mesh-extra-data", "change-mesh-morph-weights", "change-mesh-name", "change-primitive-extension-data", "change-primitive-extra-data", "change-primitive-topology-mode", "create-accessor", "create-mesh", "create-morph-target", "create-primitive", "delete-accessor", "delete-mesh", "delete-morph-target", "delete-primitive", "move-accessor", "move-mesh", "move-morph-target", "move-morph-target-attribute", "move-primitive", "move-primitive-attribute", "reorder-accessors", "reorder-meshs", "reorder-morph-target-attributes", "reorder-morph-targets", "reorder-primitive-attributes", "reorder-primitives", "unbind-morph-target-attribute", "unbind-primitive-attribute", "unbind-primitive-indices", "unbind-primitive-material",
];

/// ↩️ The inverse of a kind whose own payload cannot carry what it removed (every delete, a node
/// weight override, a new morph target): restores the named top-level members straight off the
/// ORIGINAL document, removing any member the original lacked — `buffers` also restores the
/// binary chunk, which a buffer change can shift.
#[cfg(feature = "oracles")]
pub fn restore_members(mutated: &[u8], original: &[u8], members: &[String]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, source_bin) = read_glb(original)?;
    for member in members {
        set_or_remove(&mut doc, member, obj_get(&source, member).cloned());
    }
    let bin = if members.iter().any(|member| member == "buffers") { source_bin } else { bin };
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn restore_members(_mutated: &[u8], _original: &[u8], _members: &[String]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️StructureKinds
