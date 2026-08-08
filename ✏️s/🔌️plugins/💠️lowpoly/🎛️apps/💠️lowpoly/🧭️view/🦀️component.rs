//! 🧭️ Lowpoly play app — the borrowed read view (projection + config) and the pure config/selection
//! helpers threaded through commands, panels and window renders. Every helper here takes `LowpolyConfig`
//! (an app-only view-state type) as a parameter, so per the DocumentHelpers placement rule these stay at
//! app level no matter how many taxonomy nodes consume them — artifacts must never depend on apps.

use crate::apps::lowpoly::config::LowpolyConfig;
use crate::artifacts::lowpoly::engine::LowpolyDocument;
use crate::artifacts::lowpoly::{LowpolyObject, LowpolyProjection, LowpolySelection, LowpolySelectionTargets};
use serde_json::Value;

//#region 🔖️View
/// @emoji 🧭️ A borrowed read view — the document projection plus the config — threaded into the
/// render/panel/utility/scene builders.
#[derive(Clone, Copy)]
pub struct LowpolyView<'a> {
    pub projection: &'a LowpolyProjection,
    pub config: &'a LowpolyConfig,
}
//#endregion 🔖️View

//#region 🔖️ActiveObject
pub fn resolve_active_object_id(projection: &LowpolyProjection, config: &LowpolyConfig) -> String {
    if projection.objects.iter().any(|object| object.id == config.active_object_id) {
        config.active_object_id.clone()
    } else {
        projection.objects.first().map(|object| object.id.clone()).unwrap_or_default()
    }
}

pub fn active_object<'a>(view: LowpolyView<'a>) -> Option<&'a LowpolyObject> {
    let id = resolve_active_object_id(view.projection, view.config);
    view.projection.objects.iter().find(|object| object.id == id)
}
//#endregion 🔖️ActiveObject

//#region 🔖️Selection
/// 🧮️ Rebuilds a `LowpolySelection` from `LowpolyConfig`'s flattened selection fields — the boundary
/// where the config's scalar fields become the compute session's structured selection value.
pub fn selection_from_config(config: &LowpolyConfig) -> LowpolySelection {
    LowpolySelection { targets: selection_targets_from_config(config), keys: config.selection_keys.clone(), mode: config.selection_mode.clone(), ids: config.selection_ids.clone() }
}

pub fn selection_targets_from_config(config: &LowpolyConfig) -> LowpolySelectionTargets {
    LowpolySelectionTargets { mesh: config.selection_targets_mesh, vertex: config.selection_targets_vertex, edge: config.selection_targets_edge, face: config.selection_targets_face }
}

pub fn build_doc(projection: &LowpolyProjection, config: &LowpolyConfig) -> Option<LowpolyDocument> {
    let active = resolve_active_object_id(projection, config);
    LowpolyDocument::with_context(projection.clone(), active, selection_from_config(config)).ok()
}

pub fn merge_selection_ids(existing: &[u32], incoming: &[u32], merge: &str) -> Vec<u32> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(*id);
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(*id);
                }
            }
            merged
        }
        "remove" | "subtractive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                merged.retain(|entry| entry != id);
            }
            merged
        }
        _ => incoming.to_vec(),
    }
}

pub fn document_target_row_id(object_id: &str, _object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly-document.{object_id}.{mode}.{id}")
}

pub fn selection_key(object_id: &str, object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly:{object_id}:{object_index}:{mode}:{id}")
}

pub fn object_index_for(projection: &LowpolyProjection, object_id: &str) -> usize {
    projection.objects.iter().position(|object| object.id == object_id).unwrap_or(0)
}

pub fn enable_selection_target_kind(targets: &mut LowpolySelectionTargets, mode: &str) {
    match mode {
        "vertex" => targets.vertex = true,
        "edge" => targets.edge = true,
        "face" => targets.face = true,
        _ => targets.mesh = true,
    }
}

/// 🎯️ The pure, typed-command counterpart of the pre-B1 `sync_selection_keys` — computes the
/// document-target selection keys for `mode`/`ids` without mutating anything.
pub fn selection_keys_for(projection: &LowpolyProjection, config: &LowpolyConfig, mode: &str, ids: &[u32]) -> Vec<String> {
    let active = resolve_active_object_id(projection, config);
    let object_index = object_index_for(projection, &active);
    ids.iter().map(|id| selection_key(&active, object_index, mode, *id)).collect()
}

/// 🎯️ The pure, typed-command counterpart of the pre-B1 `apply_component_selection` — computes the new
/// selection mode/ids/keys/targets after selecting `incoming` at `mode` granularity, for the caller to
/// translate into `LowpolyConfigMutation`s (never mutates `config` directly).
pub fn apply_component_selection(config: &LowpolyConfig, projection: &LowpolyProjection, mode: &str, incoming: &[u32], merge: &str) -> (String, Vec<u32>, Vec<String>, LowpolySelectionTargets) {
    let normalized = LowpolyDocument::normalize_selection_mode(mode);
    let mut targets = selection_targets_from_config(config);
    enable_selection_target_kind(&mut targets, &normalized);
    let ids = merge_selection_ids(&config.selection_ids, incoming, merge);
    let keys = selection_keys_for(projection, config, &normalized, &ids);
    (normalized, ids, keys, targets)
}

pub fn selected_document_ids(view: LowpolyView<'_>) -> Vec<String> {
    let config = view.config;
    let active = resolve_active_object_id(view.projection, config);
    let object_index = object_index_for(view.projection, &active);
    config.selection_ids.iter().map(|id| document_target_row_id(&active, object_index, &config.selection_mode, *id)).collect()
}

pub fn highlighted_document_ids(view: LowpolyView<'_>) -> Vec<String> {
    let config = view.config;
    match (&config.hovered_target_object_id, &config.hovered_target_mode, config.hovered_target_id) {
        (Some(object_id), Some(mode), Some(id)) => {
            vec![document_target_row_id(object_id, object_index_for(view.projection, object_id), mode, id)]
        }
        _ => Vec::new(),
    }
}

pub fn format_selection_targets_label(targets: &LowpolySelectionTargets) -> String {
    let mut parts = Vec::new();
    if targets.mesh {
        parts.push("mesh");
    }
    if targets.vertex {
        parts.push("vertex");
    }
    if targets.edge {
        parts.push("edge");
    }
    if targets.face {
        parts.push("face");
    }
    if parts.is_empty() {
        "none".into()
    } else {
        parts.join("+")
    }
}
//#endregion 🔖️Selection

//#region 🔖️Utility
pub fn is_paint_utility(utility_id: &str) -> bool {
    matches!(utility_id, "brush" | "eraser" | "fill" | "eyedropper")
}

pub fn primitive_kind(kind: &str) -> &str {
    match kind {
        "sphere" | "ico" => "ico_sphere",
        other => other,
    }
}

pub fn mirror_axis_from_param(params: &Value) -> semio_s_3d::mesh::MirrorAxis {
    match utility_param_u32(params, "mirrorAxis", 0) {
        1 => semio_s_3d::mesh::MirrorAxis::Y,
        2 => semio_s_3d::mesh::MirrorAxis::Z,
        _ => semio_s_3d::mesh::MirrorAxis::X,
    }
}

pub fn utility_param_f32(params: &Value, key: &str, default: f32) -> f32 {
    params.get(key).and_then(|value| value.as_f64()).map_or(default, |v| v as f32)
}

pub fn utility_param_u32(params: &Value, key: &str, default: u32) -> u32 {
    params.get(key).and_then(|value| value.as_u64()).map_or(default, |v| v as u32)
}

pub fn utility_param_f64(params: &Value, key: &str, default: f64) -> f64 {
    utility_param_f32(params, key, default as f32) as f64
}

/// 🧮️ Parses `config.utility_params_json` back into a `Value` — the flattened `LowpolyConfig` field
/// carries it as canonical JSON text since a raw `Value` field has no direct DSL binding.
pub fn utility_params_value(config: &LowpolyConfig) -> Value {
    serde_json::from_str(&config.utility_params_json).unwrap_or_default()
}

pub fn euler_degrees_to_quaternion(rotation: [f32; 3]) -> [f64; 4] {
    let to_rad = std::f32::consts::PI / 180.0;
    let (sx, cx) = (rotation[0] * to_rad * 0.5).sin_cos();
    let (sy, cy) = (rotation[1] * to_rad * 0.5).sin_cos();
    let (sz, cz) = (rotation[2] * to_rad * 0.5).sin_cos();
    [(sx * cy * cz + cx * sy * sz) as f64, (cx * sy * cz - sx * cy * sz) as f64, (cx * cy * sz + sx * sy * cz) as f64, (cx * cy * cz - sx * sy * sz) as f64]
}
//#endregion 🔖️Utility
