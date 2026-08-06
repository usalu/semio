//! ⚙️ GIS map artifact — headless compute over the map projection (constitutional: engine).
//!
//! 🧭️ Placement rule for helpers: anything here takes ONLY document-side types
//! (`GisMapDocument`/`MapFeature`/descriptor JSON). Helpers that also need the ◻2d app's view state
//! (`crate::apps::gis2d::config::Gis2dConfig`) stay at app level — an artifact must never depend on an
//! app.

use crate::artifacts::gismap::dsl::REUSE_MAP_EXAMPLE_TEXT;
use crate::artifacts::gismap::op::GisMapOperation;
use crate::artifacts::gismap::{GisMapDocument, MapFeature, MapFeaturePatch, GIS_MAP_SCHEMA};
use protocol::CollectionOperation;
use semio_framework_plugin::{DwgDrawing, DwgGeometry};
use serde_json::{json, Value};
use std::collections::HashSet;

fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
}

fn dsl_to_value(value: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
}

//#region 🔖️DocumentHelpers
pub fn empty_gis_map_projection() -> GisMapDocument {
    GisMapDocument::default()
}

/// 📥️ Parses a `{ positions, routes, regions }` map-descriptor JSON into a `GisMapDocument` — each
/// array entry becomes a `MapFeature` keyed by its `id`, keeping the full object as the payload.
pub fn gis_map_document_from_descriptor_json(json: &str) -> GisMapDocument {
    let value: Value = serde_json::from_str(json).unwrap_or_else(|_| serde_json::json!({}));
    let features = |key: &str| -> Vec<MapFeature> {
        value
            .get(key)
            .and_then(|entry| entry.as_array())
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|item| {
                        let id = item.get("id").and_then(|value| value.as_str())?.to_string();
                        Some(MapFeature { id, data: value_to_dsl(item) })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    GisMapDocument { positions: features("positions"), routes: features("routes"), regions: features("regions") }
}

/// 📤️ Rebuilds the `{ positions, routes, regions }` map-descriptor JSON the `MapHost`/renderer consume,
/// emitting each feature's opaque payload.
pub fn gis_map_descriptor_json(document: &GisMapDocument) -> String {
    let payloads = |features: &[MapFeature]| -> Vec<Value> { features.iter().map(|feature| dsl_to_value(&feature.data)).collect() };
    serde_json::json!({
        "positions": payloads(&document.positions),
        "routes": payloads(&document.routes),
        "regions": payloads(&document.regions),
    })
    .to_string()
}

/// 🗺️ The default map document, seeded from the bundled reuse example (see
/// `crate::artifacts::gismap::GisMapDocument`'s derive-generated `.gismap` DSL).
pub fn default_document() -> GisMapDocument {
    <GisMapDocument as store::DocumentDsl>::parse_dsl(REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_projection())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️CollectionDiffing
/// 🌉️ Diffs one feature collection before/after an in-place edit into granular id-keyed
/// add/remove/patch operations — used by `patchPositions` and the `features:in` import (whole-array
/// replacements still converge per-feature). `wrap` picks which `GisMapOperation` variant
/// (`Positions`/`Routes`/`Regions`) the diff belongs to.
fn feature_collection_operations(before: &[MapFeature], after: &[MapFeature], wrap: impl Fn(CollectionOperation<String, MapFeature, MapFeaturePatch>) -> GisMapOperation) -> Vec<GisMapOperation> {
    let mut operations = Vec::new();
    let after_ids: HashSet<&str> = after.iter().map(|feature| feature.id.as_str()).collect();
    for feature in before {
        if !after_ids.contains(feature.id.as_str()) {
            operations.push(wrap(CollectionOperation::Remove { id: feature.id.clone() }));
        }
    }
    for (index, feature) in after.iter().enumerate() {
        match before.iter().find(|entry| entry.id == feature.id) {
            None => operations.push(wrap(CollectionOperation::Add { id: feature.id.clone(), item: feature.clone(), at: index })),
            Some(prev) if prev.data != feature.data => operations.push(wrap(CollectionOperation::Patch { id: feature.id.clone(), patch: MapFeaturePatch { data: Some(feature.data.clone()) } })),
            Some(_) => {}
        }
    }
    operations
}

pub fn positions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOperation> {
    feature_collection_operations(before, after, GisMapOperation::Positions)
}

pub fn routes_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOperation> {
    feature_collection_operations(before, after, GisMapOperation::Routes)
}

pub fn regions_operations(before: &[MapFeature], after: &[MapFeature]) -> Vec<GisMapOperation> {
    feature_collection_operations(before, after, GisMapOperation::Regions)
}
//#endregion 🔖️CollectionDiffing

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec`
/// `crate::artifacts::gismap::artifact_kind()` declares (schema/media type/export+import
/// formats/presentation fields copied verbatim), plus the two app-specific workflow ports
/// (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe): `features:in`
/// (any TwoD×Vector producer feeds new/patched positions/routes/regions) and `map:out` (this document's
/// own feature layers, the `2d.map` interchange kind gis3d's `map:in` consumes).
pub fn gis2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: GIS_MAP_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        ports: vec![gis2d_features_in_port(), gis2d_map_out_port()],
        export_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        import_formats: vec![semio_framework_plugin::OsMediaFormat::Svg, semio_framework_plugin::OsMediaFormat::Png],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.map".into(), name: "2D Map".into(), dimension: "2d".into(), component_kind: "gismap".into() },
    }
}

/// 🔌️ `features:in` — accepts any TwoD×Vector producer (draw's `vector:out`, another gis2d's
/// `map:out`, …); no `kind_id` pin since it's a generic vector-features sink, not one specific kind.
/// `Many`/optional: several producers may fan into one map, and a map with no upstream edge is valid.
pub fn gis2d_features_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "features:in".into(),
        label: "Features".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        kind_id: None,
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🔌️ `map:out` — this document's positions/routes/regions as the `2d.map` interchange kind (gis3d's
/// `map:in` consumes it). `Many`/optional: several downstream consumers may fan out from one map, and a
/// map with no downstream edge is valid.
pub fn gis2d_map_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:out".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        kind_id: Some("2d.map".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🎞️ `map:out`'s `Media` value — this document's positions/routes/regions as a `2d.map` structured
/// payload; reuses the exact descriptor JSON shape the ◻2d window's renderer/`MapHost` already consume,
/// so there is exactly one "gis map as JSON" shape in the whole app.
pub fn gis2d_map_media(document: &GisMapDocument) -> semio_framework_plugin::Media {
    semio_framework_plugin::Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "2d.map".into(), json: gis_map_descriptor_json(document) },
    }
}
//#endregion 🔖️Io

//#region 🔖️MediaExport
pub fn gis2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::map_points_svg(value, "GIS 2D")
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 🗺️ Imports a DWG drawing into a bare gis map document: entity vertices become position features.
/// Falls back to the default reuse-map document when the DWG carries no point-like geometry.
pub fn gis2d_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let points: Vec<[f64; 2]> = drawing
        .entities
        .iter()
        .flat_map(|entity| match &entity.geometry {
            DwgGeometry::Point { at } => vec![[at[0], at[1]]],
            DwgGeometry::Line { start, end } => vec![[start[0], start[1]], [end[0], end[1]]],
            DwgGeometry::LwPolyline { vertices, .. } => vertices.clone(),
            DwgGeometry::Polyline3d { vertices, .. } => vertices.iter().map(|v| [v[0], v[1]]).collect(),
            _ => Vec::new(),
        })
        .collect();
    if points.is_empty() {
        return serde_json::to_value(default_document()).map_err(|error| error.to_string());
    }
    let positions: Vec<MapFeature> = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let id = format!("dwg-{index}");
            MapFeature { id: id.clone(), data: value_to_dsl(&json!({ "id": id, "lon": point[0], "lat": point[1] })) }
        })
        .collect();
    serde_json::to_value(GisMapDocument { positions, routes: Vec::new(), regions: Vec::new() }).map_err(|error| error.to_string())
}
//#endregion 🔖️MediaImport

//#region 🔖️Registration
/// 🗂️ Native setup hook for the `gis.map` artifact — the SVG export + DWG import handlers plus the
/// pack↔dsl document codec `framework/sync`'s `FolderEndpoint` reaches for. Called from the plugin
/// root's `📦️glue.rs` setup fn.
pub fn register() {
    semio_framework_os::register_2d_export_handlers("2d.map", "gis2d", gis2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.map", gis2d_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::gis2d::Gis2dPlayApp>(GIS_MAP_SCHEMA);
}
//#endregion 🔖️Registration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dwg_import_collects_point_and_line_vertices() {
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [1.0, 2.0, 0.0] } });
        drawing.entities.push(semio_framework_os::DwgEntity { layer, color: semio_framework_os::DwgColor::ByLayer, geometry: DwgGeometry::Line { start: [0.0, 0.0, 0.0], end: [3.0, 4.0, 0.0] } });
        let value = gis2d_document_json_from_dwg(&drawing).expect("import dwg");
        let positions = value.get("positions").and_then(|v| v.as_array()).expect("positions array");
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn dwg_import_falls_back_to_default_document_when_empty() {
        let drawing = DwgDrawing::default();
        let value = gis2d_document_json_from_dwg(&drawing).expect("import empty dwg");
        let document: GisMapDocument = serde_json::from_value(value).expect("document");
        assert!(!document.positions.is_empty(), "fallback seeds the reuse-map document");
    }

    #[test]
    fn gis2d_io_declares_the_features_in_and_map_out_ports() {
        let io = gis2d_io();
        assert_eq!(io.document_schema, GIS_MAP_SCHEMA);
        assert_eq!(io.artifact.id, "2d.map");
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "features:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
        let map_out = ports.iter().find(|port| port.id == "map:out").expect("map:out declared");
        assert_eq!(map_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(map_out.kind_id.as_deref(), Some("2d.map"));
    }

    #[test]
    fn gis2d_map_media_exports_the_document_descriptor() {
        let document = default_document();
        let media = gis2d_map_media(&document);
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else {
            panic!("expected a structured map:out payload");
        };
        assert_eq!(schema, "2d.map");
        assert!(json.contains("positions"));
    }

    #[test]
    fn feature_collection_diffing_emits_add_patch_and_remove() {
        let feature = |id: &str, label: &str| MapFeature { id: id.into(), data: value_to_dsl(&json!({ "id": id, "label": label })) };
        let before = vec![feature("keep", "a"), feature("gone", "b")];
        let after = vec![feature("keep", "changed"), feature("new", "c")];
        let operations = positions_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, GisMapOperation::Positions(CollectionOperation::Remove { id }) if id == "gone")));
        assert!(operations.iter().any(|operation| matches!(operation, GisMapOperation::Positions(CollectionOperation::Patch { id, .. }) if id == "keep")));
        assert!(operations.iter().any(|operation| matches!(operation, GisMapOperation::Positions(CollectionOperation::Add { id, .. }) if id == "new")));
        assert!(routes_operations(&before, &before).is_empty(), "an unchanged collection produces no operations");
        assert!(regions_operations(&before, &before).is_empty());
    }
}
//#endregion 🧪️Tests
