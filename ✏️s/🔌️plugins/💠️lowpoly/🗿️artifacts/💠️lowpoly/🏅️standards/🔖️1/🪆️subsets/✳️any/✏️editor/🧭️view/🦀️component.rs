//! 🧭️ Lowpoly play app — the borrowed read view (projection + config) and the pure config/selection
//! helpers threaded through commands, panels and window renders. Every helper here takes `LowpolyConfig`
//! (an app-only view-state type) as a parameter, so per the DocumentHelpers placement rule these stay at
//! app level no matter how many taxonomy nodes consume them — artifacts must never depend on apps.
//!
//! 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the mesh domain's selection/hover now
//! lives in the framework's `InteractionState`, never in `LowpolyConfig` — see `🔖️MeshDomain` below for
//! the target-id scheme (`"lowpoly-document.<objectId>[.<granularity>.<id>]"`, the SAME ids the Document
//! panel tree (`📌️panels/📄️artifact`) already renders, so a click there and the framework's `UiTree`
//! presence auto-stamp share one id space) and `selection_from_interaction`, the boundary that turns a
//! resolved `InteractionView` into the engine's `LowpolySelection`.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolySelection, LowpolySelectionTargets, LowpolySnapshot};
use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::session::LowpolyScratch;
use semio_framework_plugin::app::InteractionView;
use serde_json::Value;

//#region 🔖️View
/// @emoji 🧭️ A borrowed read view — the document projection plus the config — threaded into the
/// render/panel/utility/scene builders.
#[derive(Clone, Copy)]
pub struct LowpolyView<'a> {
    pub snapshot: &'a LowpolySnapshot,
    pub config: &'a LowpolyConfig,
}
//#endregion 🔖️View

//#region 🔖️ActiveObject
pub async fn resolve_active_object_id(snapshot: &LowpolySnapshot, config: &LowpolyConfig) -> String {
    if snapshot.objects.iter().any(|object| object.id == config.active_object_id) {
        config.active_object_id.clone()
    } else {
        snapshot.objects.first().map(|object| object.id.clone()).unwrap_or_default()
    }
}

pub async fn active_object<'a>(view: LowpolyView<'a>) -> Option<&'a LowpolyObject> {
    let id = resolve_active_object_id(view.snapshot, view.config);
    view.snapshot.objects.iter().find(|object| object.id == id)
}
//#endregion 🔖️ActiveObject

//#region 🔖️Selection
/// 🕸️ Takes `ctx: &LowpolyScratch` (round 2 of ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's
/// round-trip law fix) — the compute session's live mesh content now lives in the session-local
/// `mesh_workspace` cache, never on `LowpolySnapshot`/`LowpolyObject`. `ctx.current_selection()` is the
/// mesh-domain selection `LowpolyPlayApp::handle` resolved from `InteractionView` for THIS dispatch
/// (see `🔖️MeshDomain` below) — render call sites never populate it, which is harmless: geometry/
/// texture rendering never reads `LowpolyDocument::selection()`.
pub async fn build_doc(snapshot: &LowpolySnapshot, config: &LowpolyConfig, ctx: &LowpolyScratch) -> Option<LowpolyDocument> {
    let active = resolve_active_object_id(snapshot, config);
    LowpolyDocument::with_context(snapshot.clone(), active, ctx.current_selection().clone(), ctx.mesh_workspace_map()).ok()
}

pub async fn document_target_row_id(object_id: &str, _object_index: usize, mode: &str, id: u32) -> String {
    format!("lowpoly-document.{object_id}.{mode}.{id}")
}

pub async fn document_object_row_id(object_id: &str) -> String {
    format!("lowpoly-document.{object_id}")
}

pub async fn object_index_for(snapshot: &LowpolySnapshot, object_id: &str) -> usize {
    snapshot.objects.iter().position(|object| object.id == object_id).unwrap_or(0)
}
//#endregion 🔖️Selection

//#region 🔖️MeshDomain
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" interaction domain's own
/// id/granularity vocabulary. `InteractionTarget.id` reuses the Document panel tree's own row ids
/// (`document_object_row_id`/`document_target_row_id`) verbatim — a click there and the framework's
/// `UiTree` presence auto-stamp (`PanelTreeBuilder::interaction_domain`) then share one id namespace,
/// no separate translation table. u32 component ids stringify into that row-id shape at this boundary;
/// `parse_mesh_target_id`/`selection_from_interaction` own the round-trip back.
pub const MESH_INTERACTION_DOMAIN: &str = "mesh";
pub const MESH_GRANULARITY_OBJECT: &str = "object";

/// 🔎️ Parses a mesh-domain target id back into `(objectId, Option<(granularity, numericId)>)` — the
/// second slot is `None` for an object-granularity row (`"lowpoly-document.<objectId>"`).
pub async fn parse_mesh_target_id(id: &str) -> Option<(String, Option<(String, u32)>)> {
    let rest = id.strip_prefix("lowpoly-document.")?;
    let mut parts = rest.splitn(3, '.');
    let object_id = parts.next()?.to_string();
    match (parts.next(), parts.next()) {
        (Some(mode), Some(raw_id)) => raw_id.parse::<u32>().ok().map(|numeric| (object_id, Some((mode.to_string(), numeric)))),
        _ => Some((object_id, None)),
    }
}

/// 🕹️ Builds the engine-facing `LowpolySelection` for `active_object_id` from the framework's CURRENT
/// mesh-domain selection — the boundary where `interaction.selection("mesh")` (`String` ids) crosses
/// into `LowpolyDocument`'s per-object `u32` component ids. Only ids belonging to `active_object_id`
/// survive: like the pre-migration model, a mesh-editing kernel op always targets the ACTIVE object's
/// own selected components.
///
/// 🎯️ Reads the granularity off `DomainSelection.granularity` itself — `next_selection` (the
/// framework's own pure machine) stamps this from the LAST picked target's granularity on every
/// `interactionSelect`, whereas `InteractionView::active_granularity` only changes on an explicit
/// `setInteractionGranularity` dispatch (a separate "what the NEXT pick defaults to" concern) and would
/// silently stay "object" for a plain face pick that never touched it.
pub async fn selection_from_interaction(active_object_id: &str, interaction: &InteractionView<'_>) -> LowpolySelection {
    let selected = interaction.selection(MESH_INTERACTION_DOMAIN);
    let granularity = if selected.granularity.is_empty() { MESH_GRANULARITY_OBJECT } else { selected.granularity.as_str() };
    let mode = LowpolyDocument::normalize_selection_mode(granularity);
    let ids: Vec<u32> = selected.ids.iter().filter_map(|raw| parse_mesh_target_id(raw)).filter(|(object_id, _)| object_id == active_object_id).filter_map(|(_, component)| component.map(|(_, numeric)| numeric)).collect();
    LowpolySelection { targets: LowpolySelectionTargets::default(), keys: Vec::new(), mode, ids }
}

/// 🕹️ Builds an `interactionSelect` dispatch for one mesh-domain target — the Document panel tree's row
/// click replaces the deleted `toggleSelectionTarget`. Hover has no per-row action to build any more:
/// `UiTreeItemNode.hoverAction`/`.unhoverAction` are DELETED (per `📋️master.md`'s UI section) — a
/// domain-bound tree's hover is translated generically by the renderer now, like its selection click
/// modifiers, never by an app-built per-row action.
pub async fn mesh_select_action(granularity: &str, target_id: &str, merge: &str) -> semio_framework_plugin::ActionDescriptor {
    let targets = serde_json::to_string(&Value::Array(vec![serde_json::json!({ "granularity": granularity, "id": target_id })])).unwrap_or_default();
    crate::editor::lowpoly::lowpoly_action("interactionSelect", Some(serde_json::json!({ "domainId": MESH_INTERACTION_DOMAIN, "targets": targets, "merge": merge })))
}
//#endregion 🔖️MeshDomain

//#region 🔖️Utility
pub async fn is_paint_utility(utility_id: &str) -> bool {
    matches!(utility_id, "brush" | "eraser" | "fill" | "eyedropper")
}

pub async fn primitive_kind(kind: &str) -> &str {
    match kind {
        "sphere" | "ico" => "ico_sphere",
        other => other,
    }
}

pub async fn mirror_axis_from_param(params: &Value) -> semio_framework_3d::mesh::MirrorAxis {
    match utility_param_u32(params, "mirrorAxis", 0) {
        1 => semio_framework_3d::mesh::MirrorAxis::Y,
        2 => semio_framework_3d::mesh::MirrorAxis::Z,
        _ => semio_framework_3d::mesh::MirrorAxis::X,
    }
}

pub async fn utility_param_f32(params: &Value, key: &str, default: f32) -> f32 {
    params.get(key).and_then(|value| value.as_f64()).map_or(default, |v| v as f32)
}

pub async fn utility_param_u32(params: &Value, key: &str, default: u32) -> u32 {
    params.get(key).and_then(|value| value.as_u64()).map_or(default, |v| v as u32)
}

pub async fn utility_param_f64(params: &Value, key: &str, default: f64) -> f64 {
    utility_param_f32(params, key, default as f32) as f64
}

/// 🧮️ Parses `config.utility_params_json` back into a `Value` — the flattened `LowpolyConfig` field
/// carries it as canonical JSON text since a raw `Value` field has no direct DSL binding.
pub async fn utility_params_value(config: &LowpolyConfig) -> Value {
    serde_json::from_str(&config.utility_params_json).unwrap_or_default()
}

pub async fn euler_degrees_to_quaternion(rotation: [f32; 3]) -> [f64; 4] {
    let to_rad = std::f32::consts::PI / 180.0;
    let (sx, cx) = (rotation[0] * to_rad * 0.5).sin_cos();
    let (sy, cy) = (rotation[1] * to_rad * 0.5).sin_cos();
    let (sz, cz) = (rotation[2] * to_rad * 0.5).sin_cos();
    [(sx * cy * cz + cx * sy * sz) as f64, (cx * sy * cz - sx * cy * sz) as f64, (cx * cy * sz + sx * sy * cz) as f64, (cx * cy * cz - sx * sy * sz) as f64]
}
//#endregion 🔖️Utility
