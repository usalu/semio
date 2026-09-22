//! 👁️ `s.wfc.grid3d` editor — the PREVIEW window: the INFERRED assignment rendered as one mesh entry
//! per tile and one instance per solved cell, each scaled into its own non-uniform cell box.
//!
//! 🚫️ Nothing here is persisted. `ArtifactEditor::render` is handed the document, the config and the
//! view state and nothing else, so the solve runs on this path and the previous publication is kept
//! in a PROCESS-LOCAL residency keyed by window instance — app-local scratch, never a document field,
//! never a checkpoint. That residency is what makes `instances_delta_json` a real incremental lane
//! instead of a second copy of the full set: `instances_json` stays AUTHORITATIVE on every
//! publication and the delta rides alongside it only when it actually names less than half the set
//! (puzzle 3d's own `delta_is_worth_publishing` rule).

use crate::editor::grid3d::modes::edit::tools::fill::{self, Grid3dFillPayload};
use crate::editor::grid3d::window::Grid3dWindowConfig;
use crate::schema::inferences::{solve, Grid3dAssignment};
use crate::schema::scene_internals;
use crate::Grid3dSnapshot;
use semio_framework_plugin::{world3d_camera_json, world3d_scene, world3d_selection_json, BuiltNode, LocalizedLabel, ToolRunView, UiAssemblyResult, WindowKindDefinition, WindowOptions, WorldSunConfig};
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind;
use std::collections::BTreeMap;
use std::sync::Mutex;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid3d-preview";
pub const BODY_KEY: &str = "wfc.grid3d.preview";
pub const SURFACE_ID: &str = "wfc.grid3d.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 👁️ Stitched into the editor manifest by `crate::editor::grid3d::create_grid3d_editor`. Read-only:
/// it declares no action and no utility, because the solve is an inference and this window only
/// shows it.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: semio_framework_plugin::SurfaceKind::World3d,
        icon_id: "preview".into(),
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

//#region 🚚️Residency
/// 🚚️ One window instance's last published assignment and its revision — the `base` the next delta
/// applies TO. Bounded by construction: one entry per live window instance, each holding at most the
/// cells of one document.
#[derive(Clone, Debug, Default)]
pub struct Grid3dPreviewResidency {
    pub previous: Vec<Grid3dAssignment>,
    pub current: Vec<Grid3dAssignment>,
    pub revision: u64,
}

impl Grid3dPreviewResidency {
    /// 🔄️ Folds one freshly solved assignment in. A publication that changed nothing keeps the last
    /// delta standing, so a second pane rendering the same document can still hand its own consumer
    /// the delta the first one produced.
    pub fn refresh(&mut self, next: Vec<Grid3dAssignment>) -> bool {
        if self.revision > 0 && self.current == next {
            return false;
        }
        self.previous = std::mem::replace(&mut self.current, next);
        self.revision = self.revision.saturating_add(1);
        true
    }

    /// ⚖️ Whether a delta is worth publishing at all. A cold publication and one naming at least half
    /// the set buy a consumer nothing — it reads `instances_json` either way — while costing a second
    /// copy of the same bytes on the wire.
    pub fn delta_is_worth_publishing(&self) -> bool {
        let changed = self.current.iter().filter(|row| !self.previous.contains(row)).count();
        self.revision > 1 && changed * 2 < self.current.len().max(1)
    }
}

static RESIDENCY: Mutex<BTreeMap<String, Grid3dPreviewResidency>> = Mutex::new(BTreeMap::new());

static LAST_FILL: Mutex<Option<Vec<Grid3dAssignment>>> = Mutex::new(None);

/// 🏁 Stores the finished fill assignment in residency so the preview keeps it after the run dismisses.
pub fn commit_fill_result(window_id: &str, document: &Grid3dSnapshot, assignments: Vec<Grid3dAssignment>) {
    let _ = publish(window_id, document, assignments.clone());
    match LAST_FILL.lock() {
        Ok(mut slot) => *slot = Some(assignments),
        Err(poisoned) => *poisoned.into_inner() = Some(assignments),
    }
}

/// 🧪 The last fill commit stored in this process, or `None` when none has been stored (or after abort).
pub fn last_fill_commit() -> Option<Vec<Grid3dAssignment>> {
    match LAST_FILL.lock() {
        Ok(slot) => slot.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    }
}

/// 🧪 Clears the process-local fill commit so tests start from a clean slate.
pub fn clear_fill_commit_for_test() {
    match LAST_FILL.lock() {
        Ok(mut slot) => *slot = None,
        Err(poisoned) => *poisoned.into_inner() = None,
    }
}

/// 🪣 Live fill payload while the run is non-terminal.
pub fn live_fill_payload(tool_run: Option<&ToolRunView>) -> Option<Grid3dFillPayload> {
    let run = fill::live_fill_run(tool_run)?;
    let bytes = run.payload.as_ref()?;
    Grid3dFillPayload::decode(bytes)
}

/// 👁️ Assignments the pane paints: live fill payload first, else a fresh solve.
pub fn paint_assignments(document: &Grid3dSnapshot, tool_run: Option<&ToolRunView>) -> (Vec<Grid3dAssignment>, bool) {
    if let Some(payload) = live_fill_payload(tool_run) {
        return (payload.decided_assignments(), !payload.contradiction);
    }
    let solved = solve(document).unwrap_or_default();
    (solved.assignments, solved.satisfiable)
}


/// 🚚️ Folds one solve into the residency of one window instance and answers the payload pair the
/// scene publishes. A poisoned lock answers a cold residency rather than failing the whole surface.
pub fn publish(window_id: &str, document: &Grid3dSnapshot, assignments: Vec<Grid3dAssignment>) -> (String, Option<String>) {
    let mut residency = match RESIDENCY.lock() {
        Ok(residency) => residency,
        Err(poisoned) => poisoned.into_inner(),
    };
    let entry = residency.entry(window_id.to_string()).or_default();
    entry.refresh(assignments);
    let instances = scene_internals::preview_instances_json(document, &entry.current);
    let delta = entry.delta_is_worth_publishing().then(|| scene_internals::preview_instances_delta_json(document, &entry.previous, &entry.current, entry.revision));
    (instances, delta)
}
//#endregion 🚚️Residency

//#region 🔖️Render
/// 👁️ The solved scene. A contradiction paints the empty instance set and says so in the status line,
/// rather than failing the surface.
pub fn render(document: &Grid3dSnapshot, config: &Grid3dWindowConfig, window_id: &str, tool_run: Option<&ToolRunView>) -> UiAssemblyResult<BuiltNode> {
    let (position, target) = super::grid::framed_camera(document, config);
    let (assignments, satisfiable) = paint_assignments(document, tool_run);
    let live = live_fill_payload(tool_run).is_some();
    let (instances, delta) = if live {
        (scene_internals::preview_instances_json(document, &assignments), None)
    } else {
        publish(window_id, document, assignments)
    };
    let status = status_json(document, satisfiable, &instances);
    let scene = semio_framework_ui::wgpu::World3dScene {
        instances_delta_json: delta,
        status_json: Some(status),
        ..world3d_scene(
            world3d_camera_json(position, target, 45.0),
            scene_internals::preview_meshes_json(document),
            instances,
            world3d_selection_json("interactionSelect", &[], None),
            &WorldSunConfig::default(),
        )
    };
    semio_framework_plugin::scene_surface(SURFACE_ID, SurfaceKind::World3d, &scene)
}

/// 🩺️ The one-line verdict the window chrome shows: solved cell count against the cells the grid
/// actually has to fill, plus the last solve's verdict. Kept short on purpose — an oversized status
/// string fails the WHOLE surface's admission, not just this field.
pub fn status_json(document: &Grid3dSnapshot, satisfiable: bool, instances: &str) -> String {
    let total = (document.width as usize) * (document.height as usize) * (document.depth as usize);
    let cells = total.saturating_sub(document.masked.len().min(total));
    let solved = instances.matches("\"meshId\"").count();
    let state = if !satisfiable {
        "contradiction"
    } else if solved == 0 {
        "unsolved"
    } else {
        "solved"
    };
    dsl::json::to_string(&dsl::json::object([
        ("state".to_string(), dsl::json::Value::String(state.to_string())),
        ("solved".to_string(), dsl::json::Value::from(solved as u64)),
        ("cells".to_string(), dsl::json::Value::from(cells as u64)),
        ("seed".to_string(), dsl::json::Value::from(document.seed)),
    ]))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
