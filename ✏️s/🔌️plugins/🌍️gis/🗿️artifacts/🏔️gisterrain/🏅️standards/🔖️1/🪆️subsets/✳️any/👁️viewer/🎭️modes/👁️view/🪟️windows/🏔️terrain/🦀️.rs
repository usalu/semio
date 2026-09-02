//! 🏔️ GIS terrain viewer — the Terrain window: a read-only World3d render of the DEM patch, built
//! from the same `crate::artifacts::gisterrain::schema::{parse_descriptor, build_terrain_scene_json}`
//! pure snapshot→scene helpers the editor's own Terrain window uses — this file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! exaggeration control, no selection, no camera persistence: a viewer has no utilities that edit and
//! emits no mutations by construction (`ViewEmit`).

use crate::artifacts::gisterrain::schema::{build_terrain_scene_json, TerrainDescriptorJson};
use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::parse_descriptor;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use framework_surface::terrain::projection;
use semio_framework_plugin::{scene_surface, world3d_scene_extended, world3d_selection_json, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "gis3d-view-terrain";
pub const BODY_KEY: &str = "gis3d.view.terrain";
const SURFACE_ID: &str = "gis3d.view.composite";
/// 👁️ Read-only counterpart of the editor's `GIS3D_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's world-3d controller can never be mistaken for an editor session's.
const GIS_TERRAIN_VIEW_CONTROLLER_ID: &str = "gis3d-view";
/// 👁️ Matches the editor's `Gis3dConfig::default()` camera — a viewer has no persisted per-session
/// camera (`Config = NoConfig`), so this is a hardcoded default, not a bug.
const GIS_TERRAIN_VIEW_DEFAULT_CAMERA_JSON: &str = r#"{"position":[800.0,-800.0,600.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}"#;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::gisterrain::create_gisterrain_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Terrain", "Gelände"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "terrain-3d".into(),
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
/// 📍️ GIS pins are emitted as plain `World3d` instances with no matching `meshesJson` entry —
/// `WorldInstancesLayer`'s existing missing-mesh fallback renders a small colored box, so a viewer
/// still shows every imported overlay feature, just never lets one be selected.
fn instances_json(descriptor: &TerrainDescriptorJson) -> String {
    let instances: Vec<Value> = descriptor
        .positions
        .iter()
        .map(|position| {
            let (x, y) = projection::lonlat_to_local_meters(position.lon, position.lat, descriptor.project_origin.lon, descriptor.project_origin.lat);
            json!({
                "id": position.id,
                "meshId": "pin",
                "position": [x, y, 50.0],
                "color": "#ff3355",
                "label": position.label,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

/// 👁️ Pure `GisTerrainSnapshot -> UiNode` read: default camera (a viewer has no persisted per-session
/// camera), the same real overlay pins/terrain descriptor the editor renders, no selection overlay.
/// `world3d_scene_extended` takes 4 required args (camera/meshes/instances/selection) plus 17
/// trailing `Option<String>` extension fields — the last two (`domain_id`/`domain_granularity_id`)
/// bind this window to the "features"/"pin" interaction domain (read-only for a viewer, but still the
/// correct domain so a future hover affordance is a pure addition here), every other extension `None`.
pub fn render(document: &GisTerrainSnapshot) -> UiAssemblyResult<BuiltNode> {
    let descriptor = parse_descriptor(document);
    let mut scene = world3d_scene_extended(
        GIS_TERRAIN_VIEW_DEFAULT_CAMERA_JSON.into(),
        "[]".into(),
        instances_json(&descriptor),
        world3d_selection_json("rectangle", &[], None),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("features".into()),
        Some("pin".into()),
    );
    scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
    scene_surface(SURFACE_ID, ContractSurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_world3d_terrain_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::World3d);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::gisterrain::schema::default_terrain_document();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
