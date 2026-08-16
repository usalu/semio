//! 🌐️ Lowpoly play app — the Model window: the live 3D world-3d mesh scene (every mesh-editing/
//! transform/UV-unwrap operation runs here; paint operations are scoped on BOTH this window and the UV
//! window since the paint utilities apply to both).

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{euler_degrees_to_quaternion, resolve_active_object_id, LowpolyView};
use crate::editor::lowpoly::{lowpoly_window_engagement, lowpoly_window_measures};
use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::artifacts::lowpoly::schema::mesh_data_from_transfer;
use semio_framework_plugin::{
    build_world_3d_scene, world3d_camera_json, world3d_scene, InteractionRef, SurfaceKind, UiNode, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions,
};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_WINDOW_MAIN: &str = "lowpoly-main";
pub const LOWPOLY_PLAY_BODY_MAIN: &str = "lowpoly.play.main";
const LOWPOLY_PLAY_SURFACE_MAIN: &str = "lowpoly.play.main";
/// 🧰️ The transform gumball utility a Model window falls back to when the host hasn't set an active utility.
pub const LOWPOLY_TRANSFORM_UTILITY_DEFAULT: &str = "move";

/// 📇️ Every action this window scopes — mesh-editing/transform/UV-unwrap operations plus the paint
/// operations it shares with the UV window.
pub const LOWPOLY_MAIN_ACTIONS: &[&str] = &[
    "addPrimitive", "patchObject", "extrude", "inset", "bevel", "loopCut", "subdivide", "triangulate", "mirror", "decimate", "flipFaces", "merge", "dissolve", "snap", "toggleSmooth", "unwrapActive", "markUvSeam", "clearSeam",
    "translateSelection", "rotateSelection", "scaleSelection", "transformEnd", "addPaintLayer", "paintStrokeEnd", "paintFill", "fillBucket",
];
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::lowpoly::create_lowpoly_app`.
pub fn definition() -> WindowKindDefinition {
    let projection = crate::artifacts::lowpoly::schema::default_snapshot();
    let config = LowpolyConfig::default();
    let labels = semio_framework_plugin::resolve_labels_for_locale::<LowpolyLabels>("en-US");
    let engagement = lowpoly_window_engagement(LowpolyView { snapshot: &projection, config: &config }, LOWPOLY_TRANSFORM_UTILITY_DEFAULT, labels);
    WindowKindDefinition {
        id: LOWPOLY_PLAY_WINDOW_MAIN.into(),
        label: semio_framework_plugin::LocalizedLabel::native("Model", "Modell"),
        body_key: LOWPOLY_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "lowpoly-model".into(),
        // 🎚️ `measures` stays empty here: measures are config-derived per frame by
        // `ArtifactApp::window_measures`, never frozen into the manifest.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: Vec::new(),
        utilities: ["move", "rotate", "scale", "brush", "eraser", "fill", "eyedropper"].iter().map(|id| UtilityRef::from(*id)).collect(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" interaction domain —
        // only the Model window selects/hovers mesh components; the UV window paints textures.
        interactions: vec![InteractionRef::new(crate::editor::lowpoly::view::MESH_INTERACTION_DOMAIN)],
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from the app-level `🛠️options/*` shared by
/// both windows (see the master ticket's TEMPLATE.md §12.2 pattern).
pub fn window_measures(config: &LowpolyConfig, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    lowpoly_window_measures(config, labels)
}
//#endregion 🔖️Definition

//#region 🔖️Scene
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the mesh domain's selection/hover is
/// framework-owned `InteractionState` now, never `LowpolyConfig` — and `ArtifactApp::render` (unlike
/// `handle`/`copy_fragment`/`cut_operations`) is not threaded an `InteractionView` this wave, so this
/// scene JSON can no longer embed a live selection/hover/gumball summary itself (deleted:
/// `granularity`/`targets`/`componentIds`/`selectionMode`/`selectionMergeMode`/`hoveredComponent`/
/// `gumballActive`/`gumballTarget`, plus the per-instance `selected`/`hovered` flags below). The shell
/// renders every peer's (and the local) selection/hover generically off the SAME "mesh" domain this
/// window declares via `.window_kind_interactions` — see `📋️master.md`'s UI section ("scene payloads
/// fed from InteractionView") — so this app never needs to re-embed it.
fn world_selection_json_for(view: LowpolyView<'_>, active_utility: &str) -> String {
    let config = view.config;
    let active = resolve_active_object_id(view.snapshot, config);
    json!({
        "transformMode": active_utility,
        "interactionMode": if crate::editor::lowpoly::view::is_paint_utility(active_utility) { "paint" } else { "model" },
        "activeObjectId": active,
        "showEdges": config.show_edges,
    })
    .to_string()
}

fn world_meshes_json(doc: &LowpolyDocument, texture_cache: &HashMap<String, String>) -> String {
    let items: Vec<Value> = serde_json::from_str(&doc.tessellate_all_json().unwrap_or_else(|_| "[]".into())).unwrap_or_default();
    let meshes: Vec<Value> = items
        .iter()
        .filter_map(|item| {
            let id = item.get("id")?.as_str()?;
            let tessellation = item.get("tessellation")?;
            let texture = texture_cache.get(id).cloned();
            Some(json!({
                "id": id,
                "data": mesh_data_from_transfer(tessellation, texture),
            }))
        })
        .collect();
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

/// 🕹️ `selected`/`hovered` per-instance flags are DELETED — see `world_selection_json_for`'s doc: the
/// shell overlays the mesh domain's live selection/hover generically now.
fn world_instances_json(view: LowpolyView<'_>) -> String {
    let instances: Vec<Value> = view
        .snapshot
        .objects
        .iter()
        .map(|object| {
            let rotation = euler_degrees_to_quaternion(object.transform.rotation);
            json!({
                "id": object.id,
                "meshId": object.id,
                "position": [
                    object.transform.position[0] as f64,
                    object.transform.position[1] as f64,
                    object.transform.position[2] as f64,
                ],
                "rotation": rotation,
                "scale": [
                    object.transform.scale[0] as f64,
                    object.transform.scale[1] as f64,
                    object.transform.scale[2] as f64,
                ],
                "label": object.name,
                "smoothShading": object.smooth_shading,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

pub fn render(view: LowpolyView<'_>, loaded: Option<&LowpolyDocument>, active_utility: &str, texture_cache: &HashMap<String, String>) -> UiNode {
    let config = view.config;
    match loaded {
        Some(loaded) => build_world_3d_scene(
            LOWPOLY_PLAY_SURFACE_MAIN,
            crate::editor::lowpoly::LOWPOLY_PLAY_APP_ID,
            world3d_scene(world3d_camera_json(config.world_camera_position, config.world_camera_target, config.world_camera_fov), world_meshes_json(loaded, texture_cache), world_instances_json(view), world_selection_json_for(view, active_utility), &crate::editor::lowpoly::config::lowpoly_sun_config(config)),
        ),
        None => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data("Failed to load lowpoly document")),
    }
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, render};

    #[test]
    fn renders_world_scene() {
        let mut a = app();
        assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_MAIN).contains("world-3d"));
    }

    #[test]
    fn window_kind_actions_scope_mesh_ops_to_main_only() {
        let definition = crate::editor::lowpoly::create_lowpoly_app();
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
        };
        let main = resolve(super::LOWPOLY_PLAY_WINDOW_MAIN);
        let uv = resolve(crate::editor::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_WINDOW_UV);
        for mesh_operation in ["extrude", "addPrimitive", "bevel", "loopCut", "mirror", "unwrapActive", "markUvSeam"] {
            assert!(main.contains(&mesh_operation.to_string()), "MAIN must expose mesh operation {mesh_operation}");
            assert!(!uv.contains(&mesh_operation.to_string()), "UV must NOT expose mesh operation {mesh_operation}");
        }
        for paint_operation in ["paintFill", "fillBucket", "addPaintLayer"] {
            assert!(main.contains(&paint_operation.to_string()), "MAIN must expose paint operation {paint_operation}");
            assert!(uv.contains(&paint_operation.to_string()), "UV must expose paint operation {paint_operation}");
        }
    }
}
//#endregion 🧪️Tests
