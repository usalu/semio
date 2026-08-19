//! 🏔️ GIS 3D play app — the terrain window (view mode): the World3d viewport over the DEM patch.
//!
//! ⛰️ Reuses the existing `World3d` viewport/renderer rather than a bespoke one; deliberately
//! read-mostly for this first pass — exaggeration and the `map:in` overlay layer are the only
//! editable/undoable document state (see `crate::artifacts::gisterrain`).

use crate::editor::gis3d::config::Gis3dConfig;
use crate::editor::gis3d::GIS3D_PLAY_APP_ID;
use crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::parse_descriptor;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
/// ⚠️ Fixed opportunistically (was a pre-existing, ticket-predating unresolved `crate::modules`
/// import — see `💡️inferences/🦀️component.rs`'s identical fix for the full story). Real home:
/// `crate::artifacts::gisterrain::schema`'s `🔖️TerrainDescriptor` region.
use crate::artifacts::gisterrain::schema::{build_terrain_scene_json, TerrainDescriptorJson};
use framework_surface::terrain::projection;
use semio_framework_plugin::{build_world_3d_scene, world3d_scene_extended, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";
pub const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GIS3D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Terrain", "Gelände"),
        body_key: GIS3D_PLAY_BODY_COMPOSITE.into(),
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
/// `WorldInstancesLayer`'s existing missing-mesh fallback renders a small colored box, so
/// selection/hover/context-menu all work for free without any new scene-schema surface.
async fn instances_json(descriptor: &TerrainDescriptorJson) -> String {
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

pub async fn render(document: &GisTerrainSnapshot, cfg: &Gis3dConfig) -> UiNode {
    let descriptor = parse_descriptor(document);
    let mut scene = world3d_scene_extended(
        cfg.camera_json.clone(),
        "[]".into(),
        instances_json(&descriptor),
        // 🕹️ Pin selection now lives in the framework-owned "features" interaction domain (ticket
        // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `ArtifactEditor::render` carries no
        // `InteractionView` (a known SDK gap — see `w3c-summary.md`'s flagged `EngineCanvas`/
        // `MapHost::sync_interaction` follow-up), so this scene payload can no longer embed a live
        // selection; every not-yet-migrated `world3d_selection_json` call site in this repo already
        // passes an empty selection for the same reason.
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
        // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): this window binds the "features"
        // domain (see `create_gis3d_app`'s `.window_kind_interactions`) — a plain pick/hover on this
        // surface targets its single `"pin"` granularity, not the OS's own bare `world` board domain.
        Some("features".into()),
        Some("pin".into()),
    );
    scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
    build_world_3d_scene(GIS3D_PLAY_SURFACE, GIS3D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis3d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_the_world_3d_terrain_scene() {
        let mut app = app();
        let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("world-3d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_fixture_pins_reach_the_scene_as_world_instances() {
        let mut app = app();
        let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("p_institut_de_botanique_ulg_liege"));
        assert!(json.contains("pin"));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_definition_binds_the_world3d_surface_to_the_composite_body() {
        let definition = definition();
        assert_eq!(definition.id, GIS3D_PLAY_WINDOW_MAIN);
        assert_eq!(definition.body_key, GIS3D_PLAY_BODY_COMPOSITE);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    }
}
//#endregion 🧪️Tests
