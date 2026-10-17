//! 🧩️ Composition of the GIS map artifact: the composed drawing, image and value child slots, their stable handles and the derived-children snapshot constructors. See [`super`].

use crate::schema::feature::MapFeature;
use crate::schema::snapshot::GisMapSnapshot;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueEntry;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA;

//#region 🔖️Composition
/// 🧩️ Composed `s.stdio.semio.drawing`/`s.stdio.semio.image`/`s.stdio.semio.value` child slots
/// (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, design map §4: "map C:drawing+image+value").
/// `positions`/`routes`/`regions` stay gis's own domain-specific id-keyed feature lists (analogous to
/// `📐️cad`'s `nodes`/`references_by_model_definition_id`, kept inline rather than gutted — see that
/// plugin's `🧬️schema/📸️snapshot/🦀️.rs` module doc for the precedent) since they are NOT a
/// duplicated stdio type, just gis's own vocabulary; `drawing`/`value` are DERIVED composed children
/// with stable admitted member identities. Their typed stores move content while the parent handles
/// remain fixed; `gis_map_snapshot_with_derived_children` enforces those identities whenever the
/// parent is constructed or changed. New maps have no `image`: gis
/// carries no raster basemap capability today (see `render_mode`'s app-level raster/vector TOGGLE,
/// which selects a rendering STYLE of the same vector data, not a second raster document) — the slot
/// exists as a typed member. A supplied image handle is preserved; image-free proposal planning
/// rejects such a map without changing or dropping that member.
pub type GisMapDrawingChild = store::os_store::artifact_child::ArtifactChild<SemioDrawingSnapshot>;

pub type GisMapImageChild = store::os_store::artifact_child::ArtifactChild<SemioImageSnapshot>;

pub type GisMapValueChild = store::os_store::artifact_child::ArtifactChild<SemioValueSnapshot>;

/// 🕸️ Stable admitted CHILD handle for the map's composed drawing member.
pub fn gis_map_drawing_child_handle() -> GisMapDrawingChild {
    let child_id = "gismap-drawing".to_string();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "gismap-drawing".into(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🕸️ Stable admitted CHILD handle for the map's composed value member.
pub fn gis_map_value_child_handle() -> GisMapValueChild {
    let child_id = "gismap-value".to_string();
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "value".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "gismap-value".into(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🌱️ Maps the complete intrinsic tree to the composed value domain.
pub fn semio_value_from_intrinsic(value:&semio_framework_value::DslValue)->SemioValue{
 use semio_framework_value::{DslValue,Number};
 match value{
 DslValue::Null=>SemioValue::Null,DslValue::Bool(value)=>SemioValue::Bool{value:*value},DslValue::String(value)=>SemioValue::Str{value:value.clone()},DslValue::Bytes(bytes)=>SemioValue::Bytes{value:bytes.clone()},
 DslValue::Number(Number::UInt(value))=>SemioValue::Int{lexeme:value.to_string()},DslValue::Number(Number::Int(value))=>SemioValue::Int{lexeme:value.to_string()},DslValue::Number(Number::Float(value))=>SemioValue::Float{lexeme:format!("{value:?}")},
 DslValue::Array(items)=>SemioValue::List{items:items.iter().map(semio_value_from_intrinsic).collect()},DslValue::Object(members)=>SemioValue::Map{entries:members.iter().map(|(key,value)|SemioValueEntry{key:key.clone(),value:semio_value_from_intrinsic(value)}).collect()}
 }
}

/// 🗺️ Materializes the descriptor's owned value child without JSON admission.
pub fn gis_map_value_from_descriptor(value:&semio_framework_value::DslValue)->SemioValueSnapshot{SemioValueSnapshot{schema:STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(),root:semio_value_from_intrinsic(value),nodes:Vec::new()}}

/// 🌉️ WRITE direction, real (not a stub): `serde_json::Value` → `SemioValue`, a direct structural
/// mapping (json has no binary/graph-reference primitive, so `Bytes`/`Ref` are never produced —
/// mirrors stdio's own `semio_value_from_json` for its `json` artifact, written locally here since
/// that one converts stdio's OWN `JsonValue` AST, not `serde_json::Value`, and gis already speaks
/// `serde_json::Value` everywhere else in this file).
pub fn semio_value_from_serde_json(value: &serde_json::Value) -> SemioValue {
    match value {
        serde_json::Value::Null => SemioValue::Null,
        serde_json::Value::Bool(value) => SemioValue::Bool { value: *value },
        serde_json::Value::Number(number) => {
            let lexeme = number.to_string();
            if lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E') {
                SemioValue::Float { lexeme }
            } else {
                SemioValue::Int { lexeme }
            }
        }
        serde_json::Value::String(value) => SemioValue::Str { value: value.clone() },
        serde_json::Value::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_serde_json).collect() },
        serde_json::Value::Object(members) => SemioValue::Map { entries: members.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_from_serde_json(value) }).collect() },
    }
}

/// 🌉️ READ direction, real (not a stub): the exact inverse of `semio_value_from_serde_json`. `Ref`
/// (this format's graph-reference variant) never appears in content this bridge itself produces —
/// resolved defensively to `Null` rather than panicking, matching the honesty convention this
/// ticket's other converters use for out-of-scope input shapes.
pub fn serde_json_from_semio_value(value: &SemioValue) -> serde_json::Value {
    match value {
        SemioValue::Null => serde_json::Value::Null,
        SemioValue::Bool { value } => serde_json::Value::Bool(*value),
        SemioValue::Int { lexeme } | SemioValue::Float { lexeme } => serde_json::from_str(lexeme).unwrap_or(serde_json::Value::Null),
        SemioValue::Str { value } => serde_json::Value::String(value.clone()),
        SemioValue::Bytes { .. } => serde_json::Value::Null,
        SemioValue::List { items } => serde_json::Value::Array(items.iter().map(serde_json_from_semio_value).collect()),
        SemioValue::Map { entries } => serde_json::Value::Object(entries.iter().map(|entry| (entry.key.clone(), serde_json_from_semio_value(&entry.value))).collect()),
        SemioValue::Ref { .. } => serde_json::Value::Null,
    }
}

/// 🌉️ Builds the map's composed `value` child content — the lossless `{positions,routes,regions}`
/// descriptor JSON (`gis_map_descriptor_json`) lifted into a real `SemioValueSnapshot` graph.
pub fn gis_map_value_from_descriptor_json(descriptor_json: &str) -> SemioValueSnapshot {
    let value: serde_json::Value = serde_json::from_str(descriptor_json).unwrap_or(serde_json::Value::Null);
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_serde_json(&value), nodes: Vec::new() }
}

/// 🌉️ The exact inverse of `gis_map_value_from_descriptor_json` — recovers the descriptor JSON a
/// `value` child's content actually carries.
pub fn gis_map_descriptor_json_from_value(value: &SemioValueSnapshot) -> String {
    serde_json_from_semio_value(&value.root).to_string()
}

/// 🔄️ Assigns the stable drawing/value coordinates used by [`GisMapSnapshot::default`].
/// A supplied image handle is preserved; content changes belong to typed child mutations.
pub fn gis_map_snapshot_with_derived_children(mut document: GisMapSnapshot) -> GisMapSnapshot {
    document.drawing = gis_map_drawing_child_handle();
    document.value = gis_map_value_child_handle();
    document
}

/// 🌱️ `ArtifactApp::genesis_child_pack` for the map's derived `drawing` and `value` members — pure
/// functions of the parent snapshot, so a fresh boot or whole-document load materializes the same
/// composed children inference would derive later.
pub fn genesis_gis_map_child_pack(document: &GisMapSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    use pack::snapshot::ArtifactPack;
    if slot == "drawing" && child_id == document.drawing.child_id {
        let drawing = crate::schema::drawing_bridge::gis_map_snapshot_to_drawing(document);
        return Some(<SemioDrawingSnapshot as ArtifactPack>::encode_pack(&drawing));
    }
    if slot == "value" && child_id == document.value.child_id {
        let descriptor = gis_map_descriptor_value(document);
        let value = gis_map_value_from_descriptor(&descriptor);
        return Some(<SemioValueSnapshot as ArtifactPack>::encode_pack(&value));
    }
    None
}
//#endregion 🔖️Composition

/// 🗺️ Owned descriptor tree for composed value child inference.
pub fn gis_map_descriptor_value(document:&GisMapSnapshot)->DslValue{let payloads=|features:&[MapFeature]|DslValue::Array(features.iter().map(|feature|feature.data.clone()).collect());DslValue::object([("positions".into(),payloads(&document.positions)),("routes".into(),payloads(&document.routes)),("regions".into(),payloads(&document.regions))])}
