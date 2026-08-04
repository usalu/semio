//! ⚙️ GIS 3D app — headless compute (constitutional: engine).

use gis3d::Gis3dTerrainDocument;
use gis3d_dsl::REUSE_TERRAIN_EXAMPLE_TEXT;
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentHelpers
pub fn empty_gis3d_terrain_projection() -> Gis3dTerrainDocument {
    Gis3dTerrainDocument { exaggeration: 1.0, ..Default::default() }
}

/// 🗺️ The default terrain document, seeded from the bundled reuse example's `gisterrain
/// exaggeration=...` header (see `gis3d::Gis3dTerrainDocument`'s derive-generated `.gisterrain` DSL).
pub fn default_terrain_document() -> Gis3dTerrainDocument {
    <Gis3dTerrainDocument as store::DocumentDsl>::parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis3d_terrain_projection())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Config
/// 🧮️ B1: gis3d's real `DocumentApp::Config` — the free/live viewport camera and world selection
/// (never document fields — panning/selecting never enters undo history) plus `locale` (was read off
/// the deleted `ViewState`). Mirrors `gis2d_engine::Gis2dConfig`/`shooting_engine::ShootingConfig`'s
/// identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gis3dcfg")]
#[dsl(layout = "lines")]
pub struct Gis3dConfig {
    /// 🎥️ The free/live world camera (`{position,target,up,fov}` JSON) — was `Gis3dPlayRuntime::camera_json`.
    pub camera_json: String,
    /// 👁️ Selected pin ids — was `Gis3dPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes an
/// object-scale scene and would sit inside the ground here. Mirrors `gis3d_ui`'s pre-B1
/// `initial_camera_json`.
fn default_gis3d_camera_json() -> String {
    serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}

impl Default for Gis3dConfig {
    fn default() -> Self {
        Self { camera_json: default_gis3d_camera_json(), selected_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Gis3dConfig);

//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`), plus the two app-specific workflow
/// ports (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe):
/// `map:in` (a `2d.map` producer — gis2d's `map:out` — feeds an overlay pin layer, see
/// `Gis3dTerrainDocument::imported_features_json`) and `scene:out` (this terrain as `3d.mesh`).
/// `document_media_type` is Data×Value (the document is a scalar "exaggeration + imported overlay"
/// record, not itself mesh geometry — `scene:out` is the actual renderable mesh/terrain surface).
pub fn gis3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: gis3d::GIS_3D_TERRAIN_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        ports: vec![gis3d_map_in_port(), gis3d_scene_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "gis.terrain".into(), name: "GIS Terrain".into(), dimension: "3d".into(), component_kind: "gisterrain".into() },
    }
}

/// 🔌️ `map:in` — a `2d.map` producer (gis2d's `map:out`) feeding an overlay pin layer into this
/// terrain (see `Gis3dTerrainDocument::imported_features_json`). `One`/optional: exactly one map may
/// be draped onto a terrain at a time, and a terrain with no upstream edge is valid.
pub fn gis3d_map_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:in".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        kind_id: Some("2d.map".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::One,
    }
}

/// 🔌️ `scene:out` — this terrain as `3d.mesh` (kind already registered by lowpoly; reused verbatim,
/// not redeclared — WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `Many`/optional: several
/// downstream consumers may fan out from one terrain, and a terrain with no downstream edge is valid.
pub fn gis3d_scene_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "scene:out".into(),
        label: "Scene".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        kind_id: Some("3d.mesh".into()),
        required: false,
        multiplicity: semio_framework_core::PortMultiplicity::Many,
    }
}

/// 🎞️ `scene:out`'s `Media` value. First pass (mirrors this app's own "deliberately minimal" module
/// doc): gis3d has no CPU-side heightmap tessellator yet (rendering is scene-descriptor driven, see
/// `gis3d_ui::render_canvas`/`build_terrain_scene_json`), so this exports the same terrain descriptor
/// fields (exaggeration + imported overlay) as a structured `3d.mesh` payload rather than a real
/// triangulated mesh — an honest placeholder for the day a tessellator lands, not a silent fake.
pub fn gis3d_scene_media(document: &Gis3dTerrainDocument) -> semio_framework_plugin::Media {
    semio_framework_plugin::Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        payload: semio_framework_plugin::MediaPayload::Structured {
            schema: "3d.mesh".into(),
            json: serde_json::json!({
                "exaggeration": document.exaggeration,
                "importedFeatures": serde_json::from_str::<serde_json::Value>(&document.imported_features_json).unwrap_or(serde_json::json!(null)),
            })
            .to_string(),
        },
    }
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis3d_config_default_matches_the_pre_b1_view_defaults() {
        let config = Gis3dConfig::default();
        assert!(config.camera_json.contains("800"));
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn gis3d_config_dsl_round_trips_default_and_populated() {
        store::test_support::assert_dsl_round_trip(&Gis3dConfig::default());
        let mut populated = Gis3dConfig::default();
        populated.selected_ids = vec!["p_institut_de_botanique_ulg_liege".into()];
        store::test_support::assert_dsl_round_trip(&populated);
        store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[test]
    fn gis3d_io_declares_the_map_in_and_scene_out_ports() {
        let io = gis3d_io();
        assert_eq!(io.document_schema, gis3d::GIS_3D_TERRAIN_SCHEMA);
        let ports = io.all_ports();
        let map_in = ports.iter().find(|port| port.id == "map:in").expect("map:in declared");
        assert_eq!(map_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(map_in.kind_id.as_deref(), Some("2d.map"));
        let scene_out = ports.iter().find(|port| port.id == "scene:out").expect("scene:out declared");
        assert_eq!(scene_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(scene_out.kind_id.as_deref(), Some("3d.mesh"));
    }

    #[test]
    fn gis3d_scene_media_exports_the_terrain_descriptor() {
        let document = default_terrain_document();
        let media = gis3d_scene_media(&document);
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else {
            panic!("expected a structured scene:out payload");
        };
        assert_eq!(schema, "3d.mesh");
        assert!(json.contains("exaggeration"));
    }
}
//#endregion 🧪️Tests
