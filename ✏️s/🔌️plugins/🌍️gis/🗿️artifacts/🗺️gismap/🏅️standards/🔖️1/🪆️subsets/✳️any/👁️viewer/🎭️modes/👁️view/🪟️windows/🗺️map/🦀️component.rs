//! 🗺️ GIS map viewer — the Map window: a read-only tiled-map render of the document's
//! positions/routes/regions, built from the same `crate::artifacts::gismap::schema::gis_map_descriptor_json`
//! pure snapshot→descriptor helper the editor's own Map window uses — this file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! layer toggles, no camera persistence, no selection: a viewer has no utilities that edit and emits
//! no mutations by construction (`ViewEmit`).

use crate::artifacts::gismap::schema::gis_map_descriptor_json;
use crate::artifacts::gismap::GisMapSnapshot;
use semio_framework_plugin::{build_tiled_map_scene, LocalizedLabel, SurfaceKind, TiledMapScene, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "gis2d-view-map";
pub const BODY_KEY: &str = "gis2d.view.map";
const SURFACE_ID: &str = "gis2d.view.composite";
/// 👁️ Read-only counterpart of the editor's `GIS2D_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's tiled-map controller can never be mistaken for an editor session's.
const GIS_MAP_VIEW_CONTROLLER_ID: &str = "gis2d-view";
/// 👁️ Matches the editor's `Gis2dConfig::default()` camera — a viewer has no persisted per-session
/// camera (`Config = NoConfig`), so this is a hardcoded default, not a bug.
const GIS_MAP_VIEW_DEFAULT_CAMERA_JSON: &str = r#"{"x":0,"y":0,"zoom":1}"#;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::gismap::create_gismap_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Map", "Karte"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::TiledMap,
        icon_id: "globe".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `GisMapSnapshot -> UiNode` read: default camera/render mode (`TiledMapScene::base`'s own
/// defaults already match `Gis2dConfig::default()`'s render/vector/LOD mode — "combined"/"colored"/
/// "automatic"), every layer visible, nothing selected/hovered.
pub fn render(document: &GisMapSnapshot) -> UiNode {
    let scene = TiledMapScene::base(gis_map_descriptor_json(document), GIS_MAP_VIEW_DEFAULT_CAMERA_JSON.into());
    build_tiled_map_scene(SURFACE_ID, GIS_MAP_VIEW_CONTROLLER_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_tiled_map_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::TiledMap);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::gismap::schema::default_document();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
