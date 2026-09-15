//! 🎨️ 🎨️ Generation3d play app commands command — `set-active-example`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::{generation3d_host_snapshot_operations, generation_mutation_to_generation3d, Generation3dMutation};
use crate::standards::v1::subsets::any::schema::{empty_generation3d_snapshot, example_snapshot, is_generation3d_example_id};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧾️ Resets the ephemeral generation-preview to match a freshly-loaded document — a bundled example
/// here, an imported file in `📥️import-document` — keeping every other display option (preview
/// camera, LOD, show mode, and sun) unchanged. `graph`'s selection resets on its own — the framework
/// prunes it against the new fixture's `interaction_topology`
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
///
/// 📥️ Shared with `📥️import-document` rather than copied: loading an example and importing a file
/// are the SAME whole-document replacement, and the repeated code must live in one place.
pub fn config_after_document_load(previous: &Generation3dConfig, flow_camera: &CameraJson) -> Generation3dConfig {
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
    let host_snapshot = &doc.snapshot.host_snapshot;
    let target = if payload.example_id.is_empty() {
        empty_generation3d_snapshot()
    } else if is_generation3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };
    let mut operations: Vec<Generation3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(generation3d_host_snapshot_operations(host_snapshot, &target.host_snapshot));
    let config = config_after_document_load(cfg.snapshot, &target.host_snapshot.camera);
    // 🧹️ The loaded example projection is dead once its operations and camera are read — close it
    // through its explicit ladder, never leave the fixture's ordered layout root to drop glue.
    target.retire_cold();
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation3dConfigMutation::SetSnapshot(crate::editor::generation3d::config::SetSnapshot { config })], ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
