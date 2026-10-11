//! 🎨️ 🎨️ Generation3d play app commands command — `set-active-example`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation, SetCamera, SetSelectedGeneration};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;

use crate::standards::v1::subsets::any::schema::{empty_generation3d_snapshot, is_generation3d_example_id};
use crate::standards::v1::subsets::any::io::text::snapshot::{example_snapshot};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧾️ Resets the ephemeral generation-preview to match a freshly-loaded document — a bundled example
/// here, an imported file in `📥️import-document` — keeping every other display option (preview
/// camera, LOD, show mode, and sun) unchanged, and showing the generation the loaded document selects. `graph`'s selection resets on its own — the framework
/// prunes it against the new fixture's `interaction_topology`
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
///
/// 📥️ Shared with `📥️import-document` rather than copied: loading an example and importing a file
/// are the SAME whole-document replacement, and the repeated code must live in one place.
pub fn config_after_document_load(previous: &Generation3dConfig, flow_camera: &CameraJson, selected_generation_id: Option<String>) -> Generation3dConfig {
    Generation3dConfig {
        camera: flow_camera.clone(),
        selected_generation_id,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        sun_json: previous.sun_json.clone(),
    }
}

/// 🧭️ The concrete config leaves a freshly-loaded document asks of the config lane: the flow camera and the selected generation, each only when it moved.
pub fn config_load_mutations(previous: &Generation3dConfig, flow_camera: &CameraJson, selected_generation_id: Option<String>) -> Vec<Generation3dConfigMutation> {
    let next = config_after_document_load(previous, flow_camera, selected_generation_id);
    let mut steps = Vec::new();
    if previous.camera != next.camera {
        steps.push(Generation3dConfigMutation::SetCamera(SetCamera { camera: next.camera }));
    }
    if previous.selected_generation_id != next.selected_generation_id {
        steps.push(Generation3dConfigMutation::SetSelectedGeneration(SetSelectedGeneration { selected_generation_id: next.selected_generation_id }));
    }
    steps
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🎨️ Loads the picked example — or, for the EMPTY id, clears the graph — as
/// the artifact's load effect, outside undo history.
///
/// 🕳️ The empty id is the picker's own `No example` row: `NavbarExampleSelect` normalizes its
/// `__none__` sentinel to `""` before dispatching. It used to resolve to `default_snapshot()`, which
/// is the hexagonal mushroom column — itself one of the eight bundled examples — so the row that
/// promises NO example silently loaded one and the switch reported nothing at all (measured live on
/// the served editor 2026-09-12: one config upsert, zero artifact mutations, the column still on
/// screen). An id the dialect never published is still a no-op rather than a blank, because the
/// picker can only ever offer declared ids and a typo must not destroy a graph
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn emit(payload: &SetActiveExample, _doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let target = if payload.example_id.is_empty() {
        empty_generation3d_snapshot()
    } else if is_generation3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };
    let effect = crate::editor::generation3d::reset_generation3d_document_effect(&target);
    let config_mutations = config_load_mutations(cfg.snapshot, &target.host_snapshot.camera, target.generation.selected_generation_id.clone());
    // 🧹️ The loaded example projection is dead once its camera and selection are read — close it
    // through its explicit ladder, never leave the fixture's ordered layout root to drop glue.
    target.retire_cold();
    Ok(Emit { effects: vec![effect], config_mutations, ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}
