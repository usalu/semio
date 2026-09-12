//! 🎨️ 🎨️ Generation3d play app commands command — `set-active-example`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::{generation3d_fixture_operations, generation_mutation_to_generation3d, Generation3dMutation};
use crate::standards::v1::subsets::any::schema::{empty_generation3d_snapshot, example_snapshot, is_generation3d_example_id};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧾️ Resets the ephemeral generation-preview to match a freshly-loaded example, keeping every other
/// display option (preview camera, LOD, show mode, and sun)
/// unchanged. `graph`'s selection resets on its own — the framework prunes it against the new
/// fixture's `interaction_topology` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn config_after_example_load(previous: &Generation3dConfig, flow_camera: &CameraJson) -> Generation3dConfig {
    Generation3dConfig {
        camera: flow_camera.clone(),
        selected_generation_id: None,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        sun_json: previous.sun_json.clone(),
    }
}

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Loads the picked example — or, for the EMPTY id, clears the graph.
///
/// 🕳️ The empty id is the picker's own `No example` row: `NavbarExampleSelect` normalizes its
/// `__none__` sentinel to `""` before dispatching. It used to resolve to `default_snapshot()`, which
/// is the hexagonal mushroom column — itself one of the eight bundled examples — so the row that
/// promises NO example silently loaded one and the switch reported nothing at all (measured live on
/// the served editor 2026-09-12: one config upsert, zero artifact mutations, the column still on
/// screen). An id the dialect never published is still a no-op rather than a blank, because the
/// picker can only ever offer declared ids and a typo must not destroy a graph
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn emit(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let target = if payload.example_id.is_empty() {
        empty_generation3d_snapshot()
    } else if is_generation3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };
    let mut operations: Vec<Generation3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(generation3d_fixture_operations(fixture, &target.fixture));
    let config = config_after_example_load(cfg.snapshot, &target.fixture.camera);
    // 🧹️ The loaded example projection is dead once its operations and camera are read — close it
    // through its explicit ladder, never leave the fixture's ordered layout root to drop glue.
    target.retire_cold();
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation3dConfigMutation::SetSnapshot(crate::editor::generation3d::config::SetSnapshot { config })], ..Default::default() })
}

/// 🔁️ Restarts every attached preview window's evaluation chain against the FRESHLY loaded fixture,
/// THROUGH the retained session's arming latch — a window that already owes a tick gets no second
/// one (`FlowEvalSession::arm_window_tick`).
///
/// ⚠️ The switch owes this to itself and must never rely on the host asking for it: the chain is armed
/// nowhere else but `Generation3dPlayApp::pending_effects`, which the shell reaches only through a
/// `refresh-ui` round trip, so a switch whose refresh is narrowed, coalesced or lost leaves the preview
/// showing the PREVIOUS example's evaluation forever and reports nothing at all
/// (`📓️runtime-verification-2026-09-09.md` boot #11 — "picker label changes; preview unchanged, no
/// fault"). Arming from the gesture's own emit makes the restart a consequence of the switch, not of a
/// host refresh (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn rearm_attached_previews(session: &mut FlowEvalSession, windows: &[(&str, &str)]) -> Vec<semio_framework_plugin::Effect> {
    crate::preview_eval::rearm_attached_previews(session, windows)
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
