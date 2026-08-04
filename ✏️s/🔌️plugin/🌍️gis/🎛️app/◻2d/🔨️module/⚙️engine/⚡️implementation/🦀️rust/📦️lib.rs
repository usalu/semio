//! ⚙️ GIS 2D app — headless compute (constitutional: engine).

use gis2d::{GisMapDocument, MapFeature};
use gis2d_dsl::REUSE_MAP_EXAMPLE_TEXT;
use semio_framework_plugin::{DwgDrawing, DwgGeometry};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

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

/// 🗺️ The default map document, seeded from the bundled reuse example (see `gis2d::GisMapDocument`'s
/// derive-generated `.gismap` DSL).
pub fn default_document() -> GisMapDocument {
    <GisMapDocument as store::DocumentDsl>::parse_dsl(REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_projection())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Config
/// 🧮️ B1: gis2d's real `DocumentApp::Config` — absorbs every field that used to live in
/// `Gis2dPlayRuntime` (`gis2d_ui`, pre-B1): selection, per-layer visibility/stroke-weight, camera,
/// render/vector/LOD mode, feature selection/hover, selection method/mode — plus `locale` (was read off
/// the deleted `ViewState`). Session-only view state now round-trips through the config `DocumentStore`
/// exactly like document content, with a real `backwards` per `gis2d_op::Gis2dConfigOperation`.
/// Per-layer maps are `BTreeMap` (not `HashMap`) because the DSL derive only binds string-keyed maps
/// through `dsl_schema::Shape::Map`'s `BTreeMap<String, V>` case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gis2dcfg")]
#[dsl(layout = "lines")]
pub struct Gis2dConfig {
    /// 👁️ Selected document-tree layer ids — was `Gis2dPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ Per-layer visibility; a missing entry defaults to visible — was `Gis2dPlayRuntime::layer_visibility`.
    #[dsl(block)]
    pub layer_visibility: BTreeMap<String, bool>,
    /// 🎥️ The free/live map camera (`{x,y,zoom}` JSON) — was `Gis2dPlayRuntime::camera_json`.
    pub camera_json: String,
    /// 🖼️ `"image" | "vector" | "combined"` — was `Gis2dPlayRuntime::render_mode`.
    pub render_mode: String,
    /// 🎨️ `"colored" | "figureGround" | "invertedFigure"` — was `Gis2dPlayRuntime::vector_style`.
    pub vector_style: String,
    /// 🔽️ Active LOD tier id — was `Gis2dPlayRuntime::lod_mode`.
    pub lod_mode: String,
    /// 👁️ `{positions:[id],routes:[id]}` feature selection JSON — was `Gis2dPlayRuntime::feature_selection_json`.
    pub feature_selection_json: String,
    /// 👁️ Hovered feature JSON (or `"null"`) — was `Gis2dPlayRuntime::hover_json`.
    pub hover_json: String,
    /// 🖱️ `"rectangle" | "lasso"` marquee method — was `Gis2dPlayRuntime::selection_method`.
    pub selection_method: String,
    /// 🖱️ `"default" | "additive" | "subtractive" | "invertive"` — was `Gis2dPlayRuntime::selection_mode`.
    pub selection_mode: String,
    /// 👁️ Per-layer stroke-weight multiplier; a missing entry defaults to `1.0` — was `Gis2dPlayRuntime::layer_stroke_scale`.
    #[dsl(block)]
    pub layer_stroke_scale: BTreeMap<String, f64>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

fn default_gis2d_hover_json() -> String {
    "null".into()
}

fn default_gis2d_selection_method() -> String {
    "rectangle".into()
}

fn default_gis2d_selection_mode() -> String {
    "default".into()
}

fn default_gis2d_camera_json() -> String {
    r#"{"x":0,"y":0,"zoom":1}"#.into()
}

fn default_gis2d_render_mode() -> String {
    "combined".into()
}

fn default_gis2d_vector_style() -> String {
    "colored".into()
}

fn default_gis2d_feature_selection_json() -> String {
    r#"{"positions":[],"routes":[]}"#.into()
}

impl Default for Gis2dConfig {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            layer_visibility: BTreeMap::new(),
            camera_json: default_gis2d_camera_json(),
            render_mode: default_gis2d_render_mode(),
            vector_style: default_gis2d_vector_style(),
            // 🔽️ Mirrors `framework_surface_tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC` — not a dependency of
            // this crate (the engine has no other reason to depend on the tiled-map surface crate).
            lod_mode: "automatic".into(),
            feature_selection_json: default_gis2d_feature_selection_json(),
            hover_json: default_gis2d_hover_json(),
            selection_method: default_gis2d_selection_method(),
            selection_mode: default_gis2d_selection_mode(),
            layer_stroke_scale: BTreeMap::new(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(Gis2dConfig);

//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_gis2d_app` already declares via `.artifact_kind(...)` (schema/media type/export+import
/// formats/presentation fields copied verbatim), plus the two app-specific workflow ports
/// (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe): `features:in`
/// (any TwoD×Vector producer feeds new/patched positions/routes/regions) and `map:out` (this document's
/// own feature layers, the `2d.map` interchange kind gis3d's `map:in` consumes).
pub fn gis2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: gis2d::GIS_MAP_SCHEMA.into(),
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
/// payload; reuses the exact descriptor JSON shape `gis2d_ui`'s renderer/`MapHost` already consume, so
/// there is exactly one "gis map as JSON" shape in the whole app.
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
    fn gis2d_config_default_matches_the_existing_action_arg_sticky_defaults() {
        let config = Gis2dConfig::default();
        assert_eq!(config.render_mode, "combined");
        assert_eq!(config.vector_style, "colored");
        assert_eq!(config.lod_mode, "automatic");
        assert_eq!(config.selection_method, "rectangle");
        assert_eq!(config.selection_mode, "default");
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn gis2d_config_dsl_round_trips_default_and_populated() {
        store::test_support::assert_dsl_round_trip(&Gis2dConfig::default());
        let mut populated = Gis2dConfig::default();
        populated.selected_ids = vec!["roads".into()];
        populated.layer_visibility.insert("water".into(), false);
        populated.layer_stroke_scale.insert("roads".into(), 1.5);
        store::test_support::assert_dsl_round_trip(&populated);
        store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[test]
    fn gis2d_io_declares_the_features_in_and_map_out_ports() {
        let io = gis2d_io();
        assert_eq!(io.document_schema, gis2d::GIS_MAP_SCHEMA);
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
}
//#endregion 🧪️Tests
