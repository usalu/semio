//! 🎚️ Typed operator inputs and source text are editable through the document's event history: a committed field is ONE
//! absolute `change-widget-input` leaf (design §19), so a history edit changes exactly that typed value.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{change_widget_input, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_value::ToValue as _;

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-widget-input")]
#[value(rename_all = "camelCase")]
pub struct SetWidgetInput {
    pub widget_id: String,
    pub channel: String,
    pub value: String,
    pub component: Option<String>,
    pub operation: Option<String>,
    pub index: Option<u32>,
    pub destination: Option<u32>,
    pub facet: Option<String>,
    pub path: Option<Vec<String>>,
}

/// 📃️ Computes one absolute ordered list value, validating indices, element type and cardinality before publication.
pub(crate) fn edit_collection_value(types: &[String], current: Option<&semio_framework_value::DslValue>, payload: &SetWidgetInput, cardinality: &semio_framework_os_flow::neural::Cardinality) -> Result<WidgetInputValue, String> {
    if !cardinality.is_collection() { return Err("Select a collection input".into()); }
    let schema = current.and_then(|value| value.get("0")).and_then(|value| value.get("$schema")).and_then(semio_framework_value::DslValue::as_str).or_else(|| types.first().map(String::as_str)).ok_or("Select a supported collection type")?;
    if !types.iter().any(|kind| kind == schema) { return Err("Collection item type does not match its port".into()); }
    let mut items = match current {
        Some(value) => WidgetInputValue::of_literal_as(value, schema).and_then(|value| value.items()).ok_or("Input must be a contiguous homogeneous list")?,
        None => WidgetInputValue::collection(schema, Vec::new()).and_then(|value| value.items()).ok_or("Unsupported collection type")?,
    };
    let index = payload.index.ok_or("Choose a list item")? as usize;
    if index >= 1024 { return Err("List index exceeds its bound".into()); }
    let operation = payload.operation.as_deref().unwrap_or("set");
    if operation != "move" && payload.destination.is_some() { return Err("Only moving an item has a destination".into()); }
    if operation != "set" && payload.component.is_some() { return Err("Only editing an item has a coordinate".into()); }
    match operation {
        "set" if index < items.len() => {
            let edited = edit_input_value(types, Some(&items[index].literal()), &payload.value, payload.component.as_deref())?;
            items[index] = WidgetInputValue::of_literal(&edited).ok_or("Unsupported item type")?;
        }
        "add" if index <= items.len() => {
            let value = match schema {
                "number" => WidgetInputValue::Number(0.0), "text" => WidgetInputValue::Text(String::new()), "boolean" => WidgetInputValue::Boolean(false), "point" => WidgetInputValue::Point([0.0; 3]), "vector" => WidgetInputValue::Vector([0.0; 3]), _ => return Err("Unsupported collection type".into()),
            };
            let value = if payload.value.is_empty() { value } else { WidgetInputValue::of_literal(&edit_input_value(types, Some(&value.literal()), &payload.value, None)?).ok_or("Unsupported item type")? };
            items.insert(index, value);
        }
        "remove" if index < items.len() && payload.value.is_empty() => { items.remove(index); }
        "move" if index < items.len() && payload.value.is_empty() => {
            let destination = payload.destination.ok_or("Choose the item destination")? as usize;
            if destination >= items.len() { return Err("Item destination is outside the list".into()); }
            let value = items.remove(index);
            items.insert(destination, value);
        }
        _ => return Err("Invalid list operation or item index".into()),
    }
    if !cardinality.accepts(items.len()) { return Err("List length does not match the port cardinality".into()); }
    WidgetInputValue::collection(schema, items).ok_or_else(|| "Collection exceeds its type or size bounds".into())
}

/// 🧬️ Preserves the declared schema while changing one literal or coordinate.
pub(crate) fn edit_input_value(types: &[String], current: Option<&semio_framework_value::DslValue>, text: &str, component: Option<&str>) -> Result<semio_framework_value::DslValue, String> {
    let schema = current.and_then(|value| value.get("$schema")).and_then(semio_framework_value::DslValue::as_str).or_else(|| types.first().map(String::as_str)).ok_or("Connect a compatible output to this input")?;
    if !types.iter().any(|kind| kind == schema) || !matches!(schema, "number" | "text" | "boolean" | "point" | "vector") { return Err("Connect a compatible output to this input".into()); }
    let mut fields = match current { Some(semio_framework_value::DslValue::Object(fields)) => fields.clone(), _ => vec![("$schema".into(), semio_framework_value::DslValue::String(schema.into()))] };
    let number = || text.trim().parse::<f64>().ok().filter(|number| number.is_finite()).map(semio_framework_value::DslValue::float).ok_or_else(|| "Input must be a finite number".to_string());
    let (field, value) = match schema {
        "point" | "vector" => {
            let axis = component.filter(|axis| matches!(*axis, "x" | "y" | "z")).ok_or("Choose a coordinate to edit")?;
            for coordinate in ["x", "y", "z"] { if !fields.iter().any(|(name, _)| name == coordinate) { fields.push((coordinate.into(), semio_framework_value::DslValue::float(0.0))); } }
            (axis, number()?)
        }
        _ if component.is_some() => return Err("Scalar input has no coordinates".into()),
        "number" => ("value", number()?),
        "text" => ("value", semio_framework_value::DslValue::String(text.into())),
        "boolean" => ("value", semio_framework_value::DslValue::Bool(match text { "true" => true, "false" => false, _ => return Err("Boolean input must be true or false".into()) })),
        _ => return Err("Unsupported input schema".into()),
    };
    if let Some((_, current)) = fields.iter_mut().find(|(name, _)| name == field) { *current = value; } else { fields.push((field.into(), value)); }
    Ok(semio_framework_value::DslValue::Object(fields))
}

/// ✍️ The ONE `change-widget-input` leaf a committed field edit is, validated against the operator's declared port before
/// any document mutation (an unconnected scalar input of a declared literal type); a text source's `text` is a text
/// input. `None` when the input already holds the edited value — an unchanged field is no edit.
pub(crate) fn input_leaf(host_snapshot: &FlowHostSnapshot, payload: &SetWidgetInput) -> Result<Option<Generation3dMutation>, String> {
    if payload.facet.as_deref() == Some("meshSource") { return mesh_source_leaf(host_snapshot, payload); }
    if payload.path.is_some() { return Err("Only structured mesh fields have a path".into()); }
    if payload.facet.as_deref().is_some_and(|facet| facet != "port") { return widget_facet_leaf(host_snapshot, payload); }
    if payload.value.len() > 1_048_576 { return Err("Input text exceeds 1 MiB".into()); }
    let widget = host_snapshot.widgets.iter().find(|widget| crate::widget_id(widget) == payload.widget_id).ok_or("The selected widget no longer exists")?;
    if let Widget::InputNote { text, .. } = widget {
        if payload.channel != "text" || payload.component.is_some() || payload.index.is_some() || payload.destination.is_some() || payload.operation.as_deref().is_some_and(|operation| operation != "set") { return Err("Select an operator input or a text source".into()); }
        return Ok((*text != payload.value).then(|| change_widget_input(&payload.widget_id, "text", WidgetInputValue::Text(payload.value.clone()))));
    }
    let Widget::Neuron { neuron_kind, params, .. } = widget else { return Err("Select an operator input or a text source".into()) };
    if host_snapshot.synapses.iter().any(|synapse| synapse.to == payload.widget_id && synapse.to_port == payload.channel) { return Err("This input is connected; edit its source or disconnect it first".into()); }
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    let port = infos.get(neuron_kind).and_then(|info| info.inputs.iter().find(|input| input.name == payload.channel)).ok_or("The selected operator input is unavailable")?;
    let stored = params.get(&payload.channel).map(|value| value.to_value());
    let current = stored.clone().or_else(|| port.default.as_ref().map(|value| value.to_value()));
    if port.cardinality.is_collection() {
        let input = edit_collection_value(&port.item_types, current.as_ref(), payload, &port.cardinality)?;
        return Ok((stored.as_ref() != Some(&input.literal())).then(|| change_widget_input(&payload.widget_id, &payload.channel, input)));
    }
    if payload.index.is_some() || payload.destination.is_some() || payload.operation.as_deref().is_some_and(|operation| operation != "set") { return Err("Scalar input has no list operation".into()); }
    let edited = edit_input_value(&port.value_types, current.as_ref(), &payload.value, payload.component.as_deref())?;
    let input = WidgetInputValue::of_literal(&edited).ok_or("Unsupported input schema")?;
    Ok((stored.as_ref().and_then(WidgetInputValue::of_literal).as_ref() != Some(&input)).then(|| change_widget_input(&payload.widget_id, &payload.channel, input)))
}

/// 🥽️ Resolves the canonical Construct input or its connected text source without changing graph ownership.
pub(crate) fn mesh_source_text(host: &FlowHostSnapshot, widget_id: &str, channel: &str) -> Result<(String, String, String), String> {
    let widget = host.widgets.iter().find(|widget| crate::widget_id(widget) == widget_id).ok_or("Choose an existing mesh source")?;
    match widget {
        Widget::InputNote { id, text } if channel == "text" && host.synapses.iter().any(|wire| wire.from == *id && wire.to_port == "data" && host.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == &wire.to && neuron_kind == "brep.mesh.construct"))) => Ok((id.clone(), "text".into(), text.clone())),
        Widget::Neuron { id, neuron_kind, params, .. } if channel == "data" && neuron_kind == "brep.mesh.construct" => {
            if let Some(wire) = host.synapses.iter().find(|wire| wire.to == *id && wire.to_port == "data") { return mesh_source_text(host, &wire.from, "text"); }
            let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
            let value = params.get("data").or_else(|| infos.get(neuron_kind).and_then(|info| info.inputs.iter().find(|port| port.name == "data")).and_then(|port| port.default.as_ref())).ok_or("Choose a mesh source")?.to_value();
            let text = value.get("value").and_then(semio_framework_value::DslValue::as_str).or_else(|| value.as_str()).ok_or("Mesh source must be text")?;
            Ok((id.clone(), "data".into(), text.into()))
        }
        _ => Err("Choose a Construct Mesh input or its connected source".into()),
    }
}

/// 🎛️ A structured field edits the existing polygon text through one absolute input leaf.
pub(crate) fn mesh_source_leaf(host: &FlowHostSnapshot, payload: &SetWidgetInput) -> Result<Option<Generation3dMutation>, String> {
    if payload.component.is_some() || payload.index.is_some() { return Err("Structured mesh fields use their declared path".into()); }
    let (id, channel, text) = mesh_source_text(host, &payload.widget_id, &payload.channel)?;
    let edited = edit_mesh_source(&text, payload)?;
    Ok(edited.map(|text| change_widget_input(&id, &channel, WidgetInputValue::Text(text))))
}

/// 🧬️ Edits schema-derived primitive leaves, then validates with the owning pure polygon contract.
pub(crate) fn edit_mesh_source(text: &str, payload: &SetWidgetInput) -> Result<Option<String>, String> {
    use semio_framework_pack_json as json;
    if text.len() > 16_777_216 { return Err("Mesh source exceeds 16 MiB".into()); }
    let path = payload.path.as_ref().filter(|path| !path.is_empty() && path.len() <= 16 && path.iter().all(|part| !part.is_empty() && part.chars().count() <= 128)).ok_or("Choose a structured mesh field")?;
    if !matches!(path[0].as_str(), "vertices" | "faces" | "attributes" | "materials" | "textures") { return Err("Choose a mesh coordinate, face or attribute".into()); }
    let mut source = json::parse(text, json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    if matches!(path[0].as_str(), "materials" | "textures") {
        let original = source.to_string(); edit_mesh_asset(&mut source, payload)?; let text = source.to_string();
        semio_framework::mesh_io::text::parse_polygon_mesh_source(&text)?; return Ok((text != original).then_some(text));
    }
    if path[0] == "attributes" && ((path.len() == 2 && matches!(payload.operation.as_deref(), Some("add" | "remove"))) || path.len() == 3 && path[2] == "name") {
        if path.len() == 3 && payload.operation.as_deref().unwrap_or("set") == "set" && payload.value.trim() == path[1] && source.get("attributes").and_then(|attributes| attributes.get(&path[1])).is_some() { return Ok(None); }
        edit_mesh_attribute(&mut source, payload)?; let text = source.to_string(); semio_framework::mesh_io::text::parse_polygon_mesh_source(&text)?; return Ok(Some(text));
    }
    if payload.operation.as_deref().unwrap_or("set") != "set" { edit_mesh_array(&mut source, payload)?; let text = source.to_string(); semio_framework::mesh_io::text::parse_polygon_mesh_source(&text)?; return Ok(Some(text)); }
    if payload.destination.is_some() { return Err("Choose a primitive mesh field to edit".into()); }
    let node = mesh_field_mut(&mut source, path)?;
    let next = if node.as_bool().is_some() { json::Value::from(match payload.value.as_str() { "true" => true, "false" => false, _ => return Err("Choose a boolean value".into()) }) }
    else if node.as_str().is_some() { json::Value::from(payload.value.as_str()) }
    else if node.as_f64().is_some() { let number = payload.value.trim().parse::<f64>().ok().filter(|number| number.is_finite()).ok_or("Input must be a finite number")?; if path[0] == "faces" || path.iter().any(|part| part == "indices") { if number < 0.0 || number.fract() != 0.0 || number > u32::MAX as f64 { return Err("Choose a non-negative integer index".into()); } json::Value::from(number as u32) } else if number.fract() == 0.0 && number >= i64::MIN as f64 && number < i64::MAX as f64 { json::Value::from(number as i64) } else { json::Value::from(number) } }
    else { return Err("Choose a primitive mesh field".into()); };
    if node == &next || node.as_f64().is_some() && node.as_f64() == next.as_f64() { return Ok(None); }
    *node = next;
    let text = source.to_string();
    semio_framework::mesh_io::text::parse_polygon_mesh_source(&text)?;
    Ok(Some(text))
}

/// 🖼️ A picked raster replaces one canonical texture in the existing source value.
pub(crate) fn import_mesh_texture(text: &str, name: &str, payload: &str) -> Result<String, String> {
    use semio_framework_pack_json::{Value, Object, JsonMemberPolicy};
    if name.is_empty() || name.chars().count() > 128 || text.len() > 16_777_216 || payload.len() > 22_369_664 { return Err("Texture import exceeds its bounds".into()); }
    let (mime, encoded) = if let Some(encoded) = payload.strip_prefix("data:image/png;base64,") { ("image/png", encoded) } else if let Some(encoded) = payload.strip_prefix("data:image/jpeg;base64,") { ("image/jpeg", encoded) } else { return Err("Choose a PNG or JPEG texture".into()); };
    let bytes = crate::standards::v1::subsets::any::io::mesh_bridge::base64_decode(encoded).map_err(|error| error.to_string())?;
    let signature = if mime == "image/png" { bytes.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]) } else { bytes.starts_with(&[255, 216, 255]) && bytes.ends_with(&[255, 217]) };
    if bytes.len() > 16_777_216 || !signature { return Err("Choose a PNG or JPEG texture".into()); }
    semio_framework::mesh_io::text::parse_polygon_mesh_source(text)?;
    let mut source = semio_framework_pack_json::parse(text, JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let root = source.as_object_mut().ok_or("Choose a mesh source")?;
    if !root.contains_key("textures") { root.insert("textures", Value::Object(Object::new())); }
    let textures = root.get_mut("textures").and_then(Value::as_object_mut).ok_or("Choose a texture table")?;
    let mut texture = Object::new(); texture.insert("mime", Value::from(mime)); texture.insert("bytes", Value::Array(bytes.into_iter().map(|byte| Value::from(byte as u32)).collect()));
    textures.insert(name, Value::Object(texture));
    let encoded = source.to_string(); semio_framework::mesh_io::text::parse_polygon_mesh_source(&encoded)?; Ok(encoded)
}

/// 🎨️ Asset names and references publish atomically in the canonical source value.
fn edit_mesh_asset(source: &mut semio_framework_pack_json::Value, payload: &SetWidgetInput) -> Result<(), String> {
    use semio_framework_pack_json::{Value, Object};
    let path = payload.path.as_ref().ok_or("Choose an asset")?;
    if path.len() < 2 || payload.destination.is_some() { return Err("Choose an asset field".into()); }
    let kind = path[0].as_str(); let name = &path[1]; let operation = payload.operation.as_deref().unwrap_or("set");
    let defaults = || semio_framework_pack_json::parse(r#"{"baseColor":[1,1,1,1],"metallic":0,"roughness":1,"emissive":[0,0,0],"normalScale":[1,1],"occlusionStrength":1,"alphaMode":"OPAQUE","alphaCutoff":0.5,"doubleSided":false}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let used = if kind == "materials" { source.get("attributes").and_then(Value::as_object).is_some_and(|attributes| attributes.iter().any(|(_, attribute)| attribute.get("semantic").and_then(Value::as_str) == Some("material") && attribute.get("values").and_then(Value::as_array).is_some_and(|values| values.iter().any(|value| value.as_str() == Some(name))))) }
    else { source.get("materials").and_then(Value::as_object).is_some_and(|materials| materials.iter().any(|(_, material)| material.as_object().is_some_and(|fields| fields.iter().any(|(role, value)| role.ends_with("Texture") && value.as_str() == Some(name))))) };
    let Value::Object(root) = source else { return Err("Mesh source is invalid".into()); };
    if !root.contains_key(kind) { root.insert(kind, Value::Object(Object::new())); }
    let Some(Value::Object(table)) = root.get_mut(kind) else { return Err("Asset table is invalid".into()); };
    if path.len() == 2 && operation == "add" && kind == "materials" && payload.value.is_empty() && !table.contains_key(name) { table.insert(name.clone(), defaults()); return Ok(()); }
    if path.len() == 2 && operation == "remove" && payload.value.is_empty() { if used { return Err("Asset is in use".into()); } table.remove(name).ok_or("Choose an existing asset")?; return Ok(()); }
    if path.len() == 3 && path[2] == "name" && operation == "set" {
        let next = payload.value.trim(); if next.is_empty() || next.chars().count() > 128 || next != name && table.contains_key(next) { return Err("Choose a unique asset name".into()); }
        if next == name { return Ok(()); } let asset = table.remove(name).ok_or("Choose an existing asset")?; table.insert(next, asset);
        if kind == "materials" { if let Some(Value::Object(attributes)) = root.get_mut("attributes") { for (_, attribute) in attributes.iter_mut() { if attribute.get("semantic").and_then(Value::as_str) == Some("material") { if let Value::Array(values) = mesh_field_mut(attribute, &["values".into()])? { for value in values { if value.as_str() == Some(name) { *value = Value::from(next); } } } } } } }
        else if let Some(Value::Object(materials)) = root.get_mut("materials") { for (_, material) in materials.iter_mut() { if let Value::Object(fields) = material { for (role, value) in fields.iter_mut() { if role.ends_with("Texture") && value.as_str() == Some(name) { *value = Value::from(next); } } } } }
        return Ok(());
    }
    if kind == "textures" { return Err("Texture bytes are replaced by file import".into()); }
    if operation != "set" || path.len() < 3 { return Err("Choose a material field".into()); }
    let Some(Value::Object(material)) = table.get_mut(name) else { return Err("Choose an existing material".into()); };
    let field = path[2].as_str();
    if path.len() == 3 && field.ends_with("Texture") { if payload.value.is_empty() { material.remove(field); } else { material.insert(field, Value::from(payload.value.as_str())); } return Ok(()); }
    if !material.contains_key(field) {
        let value = if matches!(field, "textureCoordinates" | "textureSamplers") { Value::Object(Object::new()) } else { defaults().get(field).cloned().ok_or("Choose a canonical material field")? };
        material.insert(field, value);
    }
    if field == "textureCoordinates" && path.len() == 4 { let Some(Value::Object(coordinates)) = material.get_mut(field) else { return Err("Texture coordinates are invalid".into()); }; if !coordinates.contains_key(&path[3]) { coordinates.insert(path[3].clone(), Value::from(0)); } }
    if field == "textureSamplers" && path.len() == 5 {
        let Some(Value::Object(samplers)) = material.get_mut(field) else { return Err("Texture samplers are invalid".into()); }; if !samplers.contains_key(&path[3]) { samplers.insert(path[3].clone(), Value::Object(Object::new())); }
        let Some(Value::Object(sampler)) = samplers.get_mut(&path[3]) else { return Err("Texture sampler is invalid".into()); }; if !sampler.contains_key(&path[4]) { sampler.insert(path[4].clone(), Value::from(if path[4].starts_with("wrap") { 10497 } else { 9729 })); }
    }
    let node = mesh_field_mut(source, path)?;
    *node = if node.as_bool().is_some() { Value::from(match payload.value.as_str() { "true" => true, "false" => false, _ => return Err("Choose a boolean value".into()) }) }
    else if node.as_str().is_some() { Value::from(payload.value.as_str()) }
    else if node.as_f64().is_some() { let value = payload.value.trim().parse::<f64>().ok().filter(|value| value.is_finite()).ok_or("Choose a finite material value")?; if value.fract() == 0.0 && value >= i64::MIN as f64 && value < i64::MAX as f64 { Value::from(value as i64) } else { Value::from(value) } }
    else { return Err("Choose a primitive material field".into()); };
    Ok(())
}

/// 🌳️ Resolves only existing canonical object members and contiguous array indices.
fn mesh_field_mut<'a>(mut node: &'a mut semio_framework_pack_json::Value, path: &[String]) -> Result<&'a mut semio_framework_pack_json::Value, String> {
    use semio_framework_pack_json::Value;
    for part in path { node = match node {
        Value::Array(array) => { let index = part.parse::<usize>().ok().filter(|index| index.to_string() == *part).ok_or("Choose a canonical list index")?; array.get_mut(index).ok_or("Mesh field no longer exists")? }
        Value::Object(fields) => fields.get_mut(part).ok_or("Mesh field no longer exists")?,
        _ => return Err("Mesh field no longer exists".into()),
    }; }
    Ok(node)
}

/// 🏷️ Attribute creation, removal and names share the owned polygon text value.
fn edit_mesh_attribute(source: &mut semio_framework_pack_json::Value, payload: &SetWidgetInput) -> Result<(), String> {
    use semio_framework_pack_json::{Value, Object};
    if payload.destination.is_some() { return Err("Attribute fields have no move destination".into()); }
    let path = payload.path.as_ref().ok_or("Choose an attribute")?;
    let (preset, chosen) = payload.value.split_once('/').map_or((payload.value.as_str(), None), |(preset, domain)| (preset, Some(domain)));
    let domain = if preset == "material" { "face" } else { chosen.unwrap_or(if preset == "uv" { "corner" } else { "vertex" }) };
    if !matches!(domain, "vertex" | "face" | "edge" | "corner") { return Err("Choose an attribute domain".into()); }
    let sample = match preset { "normal" => Some(Value::Array(vec![Value::from(0), Value::from(0), Value::from(1)])), "uv" => Some(Value::Array(vec![Value::from(0); 2])), "color" => Some(Value::Array(vec![Value::from(1); 4])), "number" => Some(Value::from(0)), "boolean" => Some(Value::from(false)), "vector" => Some(Value::Array(vec![Value::from(0); 3])), "text" => Some(Value::from("")), "material" => chosen.map(Value::from), _ => None };
    let count = if matches!(domain, "corner" | "edge") { source.get("faces").and_then(Value::as_array).ok_or("Mesh faces are unavailable")?.iter().map(|face| face.as_array().map_or(0, Vec::len)).sum() } else if domain == "face" { source.get("faces").and_then(Value::as_array).ok_or("Mesh faces are unavailable")?.len() } else { source.get("vertices").and_then(Value::as_array).ok_or("Mesh vertices are unavailable")?.len() };
    let Value::Object(fields) = source else { return Err("Mesh source is invalid".into()); };
    if !fields.contains_key("attributes") { fields.insert("attributes", Value::Object(Object::new())); }
    let Some(Value::Object(attributes)) = fields.get_mut("attributes") else { return Err("Mesh attributes are invalid".into()); };
    let name = &path[1];
    match payload.operation.as_deref().unwrap_or("set") {
        "add" if path.len() == 2 && !attributes.contains_key(name) => {
            let semantic = if matches!(preset, "uv" | "normal" | "color" | "material") { preset } else { "custom" };
            let mut attribute = Object::new(); attribute.insert("domain", Value::from(domain)); attribute.insert("semantic", Value::from(semantic)); attribute.insert("interpolation", Value::from(if matches!(preset, "text" | "boolean" | "material") { "nearest" } else { "linear" })); attribute.insert("values", Value::Array(vec![sample.ok_or("Choose an attribute type")?; count])); attributes.insert(name.clone(), Value::Object(attribute));
        }
        "remove" if path.len() == 2 && payload.value.is_empty() => { attributes.remove(name).ok_or("Choose an existing attribute")?; }
        "set" if path.len() == 3 && path[2] == "name" => {
            let next = payload.value.trim(); if next.is_empty() || next.chars().count() > 128 || next != name && attributes.contains_key(next) { return Err("Choose a unique attribute name".into()); }
            let attribute = attributes.remove(name).ok_or("Choose an existing attribute")?; attributes.insert(next, attribute);
        }
        _ => return Err("Choose an attribute action".into()),
    }
    Ok(())
}

/// 📐️ Source list edits preserve vertex references and ordered authored domain samples.
fn edit_mesh_array(source: &mut semio_framework_pack_json::Value, payload: &SetWidgetInput) -> Result<(), String> {
    use semio_framework_pack_json::Value;
    if !payload.value.is_empty() { return Err("Mesh list actions have no primitive value".into()); }
    let path = payload.path.as_ref().ok_or("Choose a mesh list")?;
    let add = payload.operation.as_deref() == Some("add");
    if payload.operation.as_deref() != Some("move") && payload.destination.is_some() { return Err("Mesh destinations belong to move actions".into()); }
    let parent = if add { path.as_slice() } else { &path[..path.len() - 1] };
    if !(parent.len() == 1 || parent.first().is_some_and(|name| name == "faces") && parent.len() == 2 || parent.first().is_some_and(|name| name == "attributes") && parent.last().is_some_and(|name| matches!(name.as_str(), "values" | "indices"))) { return Err("Choose an editable mesh list".into()); }
    let old_faces = source.get("faces").and_then(Value::as_array).ok_or("Mesh faces are unavailable")?.clone();
    let vertex_count = source.get("vertices").and_then(Value::as_array).ok_or("Mesh vertices are unavailable")?.len();
    let Value::Array(array) = mesh_field_mut(source, parent)? else { return Err("Choose an editable mesh list".into()); };
    let mut order: Vec<Option<usize>> = (0..array.len()).map(Some).collect();
    let index = if add { array.len() } else { let part = path.last().unwrap(); part.parse::<usize>().ok().filter(|index| index.to_string() == *part && *index < array.len()).ok_or("Choose a mesh list item")? };
    if add {
        let item = if parent == ["vertices"] { Value::Array(vec![Value::from(0); 3]) }
            else if parent == ["faces"] { Value::Array((vertex_count as u32..vertex_count as u32 + 3).map(Value::from).collect()) }
            else if parent[0] == "faces" { Value::from((0..vertex_count).find(|index| !array.iter().any(|value| value.as_u64() == Some(*index as u64))).ok_or("Every vertex already belongs to this face")? as u32) }
            else if parent.last().is_some_and(|name| name == "indices") { Value::from(0) }
            else { mesh_sample_default(array.first().ok_or("Choose an existing primitive attribute sample")?, "custom")? };
        let insertion = if parent[0] == "faces" && parent.len() == 2 { array.len() - 1 } else { array.len() };
        array.insert(insertion, item); order.insert(insertion, None);
    } else if payload.operation.as_deref() == Some("remove") { array.remove(index); order.remove(index); }
    else if payload.operation.as_deref() == Some("move") {
        let destination = payload.destination.map(|destination| destination as usize).filter(|destination| *destination < array.len()).ok_or("Choose a mesh destination")?;
        let item = array.remove(index); array.insert(destination, item); let old = order.remove(index); order.insert(destination, old);
    } else { return Err("Choose a mesh list operation".into()); }
    let mut orders = std::collections::BTreeMap::new();
    if add && parent == ["faces"] {
        let Value::Array(vertices) = mesh_field_mut(source, &["vertices".into()])? else { return Err("Mesh vertices are unavailable".into()); };
        vertices.extend([[0,0,0], [1,0,0], [0,1,0]].map(|point| Value::Array(point.into_iter().map(Value::from).collect())));
        let mut order: Vec<_> = (0..vertex_count).map(Some).collect(); order.extend([None; 3]); orders.insert("vertex", order);
    }
    if parent == ["vertices"] {
        let reverse: std::collections::BTreeMap<usize, usize> = order.iter().enumerate().filter_map(|(next, old)| old.map(|old| (old, next))).collect();
        let Value::Array(faces) = mesh_field_mut(source, &["faces".into()])? else { return Err("Mesh faces are unavailable".into()); };
        for face in faces { let Value::Array(vertices) = face else { return Err("Mesh face is invalid".into()); }; for vertex in vertices { *vertex = Value::from(*reverse.get(&(vertex.as_u64().ok_or("Mesh index is invalid")? as usize)).ok_or("A referenced vertex cannot be removed")? as u32); } }
        orders.insert("vertex", order);
    } else if parent[0] == "faces" {
        let mut count = 0; let offsets: Vec<usize> = old_faces.iter().map(|face| { let offset = count; count += face.as_array().map_or(0, Vec::len); offset }).collect();
        let mut corners = Vec::new();
        if parent == ["faces"] {
            for old in &order { if let Some(old) = old { corners.extend((0..old_faces[*old].as_array().ok_or("Mesh face is invalid")?.len()).map(|index| Some(offsets[*old] + index))); } else { corners.extend([None; 3]); } }
            orders.insert("face", order);
        } else {
            let target = parent[1].parse::<usize>().map_err(|_| "Mesh face index is invalid")?;
            for (face, items) in old_faces.iter().enumerate() { if face == target { corners.extend(order.iter().map(|old| old.map(|old| offsets[face] + old))); } else { corners.extend((0..items.as_array().ok_or("Mesh face is invalid")?.len()).map(|index| Some(offsets[face] + index))); } }
        }
        orders.insert("edge", corners.clone()); orders.insert("corner", corners);
    }
    if let Value::Object(fields) = source { if let Some(Value::Object(attributes)) = fields.get_mut("attributes") { for (_, attribute) in attributes.iter_mut() {
        let Value::Object(fields) = attribute else { return Err("Mesh attribute is invalid".into()); };
        let domain = fields.get("domain").and_then(Value::as_str).ok_or("Mesh attribute domain is invalid")?;
        let Some(order) = orders.get(domain) else { continue; };
        let indexed = fields.contains_key("indices"); let key = if indexed { "indices" } else { "values" };
        let semantic = fields.get("semantic").and_then(Value::as_str).unwrap_or("custom").to_string();
        let samples = fields.get(key).and_then(Value::as_array).ok_or("Mesh attribute samples are invalid")?;
        let samples = order.iter().map(|old| if let Some(old) = old { samples.get(*old).cloned().ok_or_else(|| "Mesh attribute cardinality is invalid".into()) } else if indexed { Ok(Value::from(0)) } else { mesh_sample_default(samples.first().ok_or("Mesh attribute sample is unavailable")?, &semantic) }).collect::<Result<Vec<_>, String>>()?;
        fields.insert(key, Value::Array(samples));
    } } }
    Ok(())
}

/// 🧫️ New samples retain their authored primitive and tuple shapes.
fn mesh_sample_default(value: &semio_framework_pack_json::Value, semantic: &str) -> Result<semio_framework_pack_json::Value, String> {
    use semio_framework_pack_json::Value;
    if semantic == "normal" { return Ok(Value::Array(vec![Value::from(0), Value::from(0), Value::from(1)])); }
    Ok(match value {
        Value::Array(array) => Value::Array(array.iter().map(|value| mesh_sample_default(value, "custom")).collect::<Result<_, _>>()?),
        Value::Number(_) => Value::from(0), Value::Bool(_) => Value::from(false),
        Value::String(text) => Value::from(if semantic == "material" { text.as_str() } else { "" }),
        _ => return Err("Choose an existing primitive attribute sample".into()),
    })
}

/// 🪪️ Variable metadata and export configuration update the existing cohesive widget facet.
pub(crate) fn widget_facet_leaf(host: &FlowHostSnapshot, payload: &SetWidgetInput) -> Result<Option<Generation3dMutation>, String> {
    if payload.component.is_some() || payload.index.is_some() || payload.destination.is_some() || payload.operation.is_some() { return Err("Widget metadata has no list operation or coordinate".into()); }
    let widget = host.widgets.iter().find(|widget| crate::widget_id(widget) == payload.widget_id).ok_or("Choose an existing widget")?;
    let value = payload.value.trim();
    if value.is_empty() || value.chars().count() > 256 { return Err("Widget metadata requires 1 to 256 characters".into()); }
    let next = match (payload.facet.as_deref(), widget, payload.channel.as_str()) {
        (Some("variableName"), Widget::Variable { id, schema, .. }, "name") => Widget::Variable { id: id.clone(), name: value.into(), schema: schema.clone() },
        (Some("variableSchema"), Widget::Variable { id, name, .. }, "schema") => {
            if !semio_framework_os_flow::flow_operator_registry().schema_ids().iter().any(|schema| schema == value) { return Err("Choose a registered variable type".into()); }
            let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
            for wire in &host.synapses {
                let endpoint = if wire.to == *id && wire.from != *id { Some((&wire.from, &wire.from_port, false)) } else if wire.from == *id && wire.to != *id { Some((&wire.to, &wire.to_port, true)) } else { None };
                let Some((target, port, incoming)) = endpoint else { continue; };
                let widget = host.widgets.iter().find(|widget| crate::widget_id(widget) == target).ok_or("Variable connection target is unavailable")?;
                let (inputs, outputs, _, _) = semio_framework_artifact_flow_flow::widget_io_ports(widget, &host.synapses, &infos);
                let channel = if incoming { &inputs } else { &outputs }.iter().find(|channel| channel.id == *port).ok_or("Variable connection port is unavailable")?;
                if channel.value_type.as_deref().is_some_and(|schemas| !schemas.split(',').any(|schema| schema == value)) { return Err("Variable type does not match its connected ports".into()); }
            }
            Widget::Variable { id: id.clone(), name: name.clone(), schema: value.into() }
        }
        (Some("exportFormat"), Widget::OutputExport { id, .. }, "format") => {
            if !crate::standards::v1::subsets::any::io::document_io::EXPORT_FORMATS.iter().any(|format| format.id == value) { return Err("Choose an artifact-owned export format".into()); }
            Widget::OutputExport { id: id.clone(), format: value.into() }
        }
        _ => return Err("Widget facet does not match its target or field".into()),
    };
    Ok((&next != widget).then(|| Generation3dMutation::UpdateWidget(crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget { widget: next })))
}

pub fn handle(payload: &SetWidgetInput, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::mutations(input_leaf(&doc.snapshot.host_snapshot, payload).map_err(Fault::from)?.into_iter().collect()))
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
