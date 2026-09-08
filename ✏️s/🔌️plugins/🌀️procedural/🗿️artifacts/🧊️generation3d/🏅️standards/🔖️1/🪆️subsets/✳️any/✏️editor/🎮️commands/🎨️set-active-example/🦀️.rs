//! 🎨️ 🎨️ Generation3d play app commands command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::text::{generation_mutation_to_generation3d, generation3d_fixture_operations, Generation3dMutation};
use crate::standards::v1::subsets::any::schema::{default_snapshot, example_snapshot, is_generation3d_example_id};
use crate::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_os_flow::{FlowEvalSession};
use semio_framework_artifact_flow_flow::{CameraJson};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧾️ Resets the ephemeral generation-preview to match a freshly-loaded example, keeping every other
/// display option (preview camera, LOD, show mode, sun, active utility)
/// unchanged. `graph`'s selection resets on its own — the framework prunes it against the new
/// fixture's `interaction_topology` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn config_after_example_load(previous: &Generation3dConfig, flow_camera: &CameraJson) -> Generation3dConfig {
    Generation3dConfig {
        camera: flow_camera.clone(),
        selected_generation_id: None,
        generation_preview_text: None,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        sun_json: previous.sun_json.clone(),
        active_utility_id: previous.active_utility_id.clone(),
        preview_eval_text: None,
    }
}

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    session.set_eval_json(String::new());
    let fixture = &doc.snapshot.fixture;
    let target = if payload.example_id.is_empty() {
        default_snapshot()
    } else if is_generation3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };
    let mut operations: Vec<Generation3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(generation3d_fixture_operations(fixture, &target.fixture));
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation3dConfigMutation::Snapshot { config: config_after_example_load(cfg.snapshot, &target.fixture.camera) }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
