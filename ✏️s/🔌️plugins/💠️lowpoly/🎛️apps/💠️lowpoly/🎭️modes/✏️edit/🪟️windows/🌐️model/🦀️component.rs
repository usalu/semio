//! 🌐️ Lowpoly play app — the Model window: the live 3D world-3d mesh scene (every mesh-editing/
//! transform/UV-unwrap operation runs here; paint operations are scoped on BOTH this window and the UV
//! window since the paint utilities apply to both).

use crate::apps::lowpoly::config::LowpolyConfig;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::view::{euler_degrees_to_quaternion, resolve_active_object_id, LowpolyView};
use crate::apps::lowpoly::{lowpoly_window_engagement, lowpoly_window_measures};
use crate::artifacts::lowpoly::engine::{mesh_data_from_transfer, LowpolyDocument};
use semio_framework_plugin::{
    build_world_3d_scene, world3d_camera_json, world3d_scene, world3d_selection_json, ActionRef, SurfaceKind, UiNode, UtilityRef, WindowEngagementSlot, WindowKindDefinition, WindowMeasure, WindowOptions,
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
/// 🧱️ Stitched into the app manifest by `crate::apps::lowpoly::create_lowpoly_app`.
pub fn definition() -> WindowKindDefinition {
    let projection = crate::artifacts::lowpoly::engine::default_snapshot();
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
        // `DocumentApp::window_measures`, never frozen into the manifest.
        options: WindowOptions { measures: Vec::new(), engagement: WindowEngagementSlot::Some(engagement) },
        actions: LOWPOLY_MAIN_ACTIONS.iter().map(|id| ActionRef::from(*id)).collect(),
        utilities: ["move", "rotate", "scale", "brush", "eraser", "fill", "eyedropper"].iter().map(|id| UtilityRef::from(*id)).collect(),
        params_schema: None,
        document_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from the app-level `🎚️options/*` shared by
/// both windows (see the master ticket's TEMPLATE.md §12.2 pattern).
pub fn window_measures(config: &LowpolyConfig, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    lowpoly_window_measures(config, labels)
}
//#endregion 🔖️Definition

//#region 🔖️Scene
fn gumball_target_world(doc: &LowpolyDocument, view: LowpolyView<'_>) -> Option<[f64; 3]> {
    let pivot = doc.selection_transform_pivot().ok()?;
    let active = resolve_active_object_id(view.snapshot, view.config);
    let object = view.snapshot.objects.iter().find(|entry| entry.id == active)?;
    let position = &object.transform.position;
    Some([position[0] as f64 + pivot.x() as f64, position[1] as f64 + pivot.y() as f64, position[2] as f64 + pivot.z() as f64])
}

fn gumball_active(view: LowpolyView<'_>) -> bool {
    let config = view.config;
    let active = resolve_active_object_id(view.snapshot, config);
    !config.selection_ids.is_empty() || (config.selection_targets_mesh && config.selected_object_ids.iter().any(|id| id == &active))
}

fn world_selection_json_for(view: LowpolyView<'_>, active_utility: &str, doc: Option<&LowpolyDocument>) -> String {
    use crate::apps::lowpoly::view::selection_targets_from_config;
    let config = view.config;
    let active = resolve_active_object_id(view.snapshot, config);
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&config.selection_method, &config.selected_object_ids, config.hovered_object_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!(config.selection_mode));
        object.insert("targets".into(), json!(selection_targets_from_config(config)));
        object.insert("transformMode".into(), json!(active_utility));
        object.insert("interactionMode".into(), json!(if crate::apps::lowpoly::view::is_paint_utility(active_utility) { "paint" } else { "model" }));
        object.insert("componentIds".into(), json!(config.selection_ids));
        object.insert("selectionMode".into(), json!(config.selection_mode));
        object.insert("selectionMergeMode".into(), json!(config.selection_mode_default));
        object.insert("activeObjectId".into(), json!(active));
        object.insert("gumballActive".into(), json!(gumball_active(view)));
        object.insert("showEdges".into(), json!(config.show_edges));
        if let Some(object_id) = config.hovered_target_object_id.clone() {
            object.insert("hoveredComponent".into(), json!({ "objectId": object_id, "mode": config.hovered_target_mode, "id": config.hovered_target_id }));
        }
        if let Some(loaded) = doc {
            if let Some(target) = gumball_target_world(loaded, view) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
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

fn world_instances_json(view: LowpolyView<'_>) -> String {
    let config = view.config;
    let instances: Vec<Value> = view
        .snapshot
        .objects
        .iter()
        .enumerate()
        .map(|(object_index, object)| {
            let selected = config.selected_object_ids.iter().any(|id| id == &object.id) || (config.selection_mode == "mesh" && config.selection_ids.iter().any(|id| *id as usize == object_index));
            let hovered = if config.hovered_target_object_id.is_some() {
                config.hovered_target_mode.as_deref() == Some("mesh") && config.hovered_target_object_id.as_deref() == Some(object.id.as_str())
            } else {
                config.hovered_object_id.as_deref() == Some(object.id.as_str())
            };
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
                "selected": selected,
                "hovered": hovered,
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
            crate::apps::lowpoly::LOWPOLY_PLAY_APP_ID,
            world3d_scene(world3d_camera_json(config.world_camera_position, config.world_camera_target, config.world_camera_fov), world_meshes_json(loaded, texture_cache), world_instances_json(view), world_selection_json_for(view, active_utility, Some(loaded)), &crate::apps::lowpoly::config::lowpoly_sun_config(config)),
        ),
        None => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data("Failed to load lowpoly document")),
    }
}
//#endregion 🔖️Scene

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, render};

    #[test]
    fn renders_world_scene() {
        let mut a = app();
        assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_MAIN).contains("world-3d"));
    }

    #[test]
    fn window_kind_actions_scope_mesh_ops_to_main_only() {
        let definition = crate::apps::lowpoly::create_lowpoly_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
        };
        let main = resolve(super::LOWPOLY_PLAY_WINDOW_MAIN);
        let uv = resolve(crate::apps::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_WINDOW_UV);
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
