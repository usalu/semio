//! 🏔️ GIS 3D play app — the terrain window (view mode): the World3d viewport over the DEM patch.
//!
//! ⛰️ Reuses the existing `World3d` viewport/renderer rather than a bespoke one; deliberately
//! read-mostly for this first pass — exaggeration and the `map:in` overlay layer are the only
//! editable/undoable document state (see `crate::artifacts::gisterrain`).

use crate::apps::gis3d::config::Gis3dConfig;
use crate::apps::gis3d::GIS3D_PLAY_APP_ID;
use crate::artifacts::gisterrain::engine::parse_descriptor;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use crate::modules::terrain::{build_terrain_scene_json, TerrainDescriptorJson};
use framework_surface::terrain::projection;
use semio_framework_plugin::{build_world_3d_scene, world3d_scene_extended, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";
pub const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GIS3D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Terrain", "Gelände"),
        body_key: GIS3D_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "terrain-3d".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
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

pub fn render(document: &GisTerrainSnapshot, cfg: &Gis3dConfig) -> UiNode {
    let descriptor = parse_descriptor(document);
    let mut scene = world3d_scene_extended(
        cfg.camera_json.clone(),
        "[]".into(),
        instances_json(&descriptor),
        world3d_selection_json("rectangle", &cfg.selected_ids, None),
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
    );
    scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
    build_world_3d_scene(GIS3D_PLAY_SURFACE, GIS3D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis3d::testkit::{app, render as render_body};

    #[test]
    fn renders_the_world_3d_terrain_scene() {
        let mut app = app();
        let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn the_fixture_pins_reach_the_scene_as_world_instances() {
        let mut app = app();
        let json = render_body(&mut app, GIS3D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("p_institut_de_botanique_ulg_liege"));
        assert!(json.contains("pin"));
    }

    #[test]
    fn the_definition_binds_the_world3d_surface_to_the_composite_body() {
        let definition = definition();
        assert_eq!(definition.id, GIS3D_PLAY_WINDOW_MAIN);
        assert_eq!(definition.body_key, GIS3D_PLAY_BODY_COMPOSITE);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    }
}
//#endregion 🧪️Tests
