//! 🗺️ GIS map viewer — the Map window: a read-only tiled-map render of the document's
//! positions/routes/regions, built from the same `crate::schema::gis_map_descriptor_json`
//! pure snapshot→descriptor helper the editor's own Map window uses — this file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! layer toggles and no selection: a viewer has no utilities that edit and emits no document mutation.
//!
//! 🧭️ It DOES pan, though, and the react `TiledMapHost` dispatches `setCamera` on its own after every
//! pan/zoom whether the window is read-only or not. A window kind that does not declare that verb
//! drops every gesture (`dropped action "setCamera" … no window kind declares it`, S15 session 11),
//! so this one declares it and retains the camera in its own window config — the config lane, never
//! the artifact lane, which is what keeps the surface structurally read-only.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use crate::schema::gis_map_descriptor_json;
use crate::GisMapSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::{scene_surface, ActionArgDef, ActionDefinition, ActionKind, BuiltNode, InteractiveJobClassification, LocalizedLabel, SurfaceKind, TiledMapScene, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "gis2d-view-map";
pub const BODY_KEY: &str = "gis2d.view.map";
const SURFACE_ID: &str = "gis2d.view.composite";
/// 👁️ The host's default camera, published while a window has never been panned: the `TiledMapHost`
/// fits the world exactly when the scene carries this string.
const GIS_MAP_VIEW_DEFAULT_CAMERA_JSON: &str = r#"{"x":0,"y":0,"zoom":1}"#;
/// 🧭️ The camera verb the react `TiledMapHost` dispatches after every pan/zoom. It is NOT
/// framework-reserved: undeclared, it lands as `dropped action "setCamera" … no window kind declares it`.
pub const SET_CAMERA_ACTION_ID: &str = "setCamera";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧭️ The one verb a read-only map window owns. `ActionKind::View` and a window-config publication: a
/// camera is never document data, so it publishes no document mutation — which is exactly why a
/// VIEWER may own it. It declares its `camera` argument so the host's `{surfaceId, camera:{x,y,zoom}}`
/// survives `effective_action_args`, which keeps ONLY declared arg ids once an action declares any.
pub fn set_camera_action() -> ActionDefinition {
    let mut action = ActionDefinition::bounded_catalog(SET_CAMERA_ACTION_ID, LocalizedLabel::native("Set camera", "Kamera setzen"), ActionKind::View)
        .with_args(vec![ActionArgDef::text("camera", LocalizedLabel::native("Map camera", "Kartenkamera")).required()]);
    action.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    action
}

/// 🧱️ Stitched into the viewer manifest by `crate::viewer::gismap::create_gismap_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Map", "Karte"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::TiledMap,
        icon_id: "globe".into(),
        options: WindowOptions::default(),
        actions: vec![set_camera_action()],
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
/// 👁️ Pure `(GisMapSnapshot, retained camera) -> UiNode` read: the window's retained camera, or the
/// default the host fits to the world while it has never been panned; default render mode
/// (`TiledMapScene::base`'s own defaults already match the editor map's "combined"/"colored"/
/// "automatic"), every layer visible, nothing selected/hovered.
pub fn render(document: &GisMapSnapshot, window: Option<&config::GisMapViewerWindowConfig>) -> UiAssemblyResult<BuiltNode> {
    let camera_json = window.map_or_else(|| GIS_MAP_VIEW_DEFAULT_CAMERA_JSON.to_string(), |window| window.camera.scene_camera_json());
    let scene = TiledMapScene::base(gis_map_descriptor_json(document), camera_json);
    scene_surface(SURFACE_ID, ContractSurfaceKind::TiledMap, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
