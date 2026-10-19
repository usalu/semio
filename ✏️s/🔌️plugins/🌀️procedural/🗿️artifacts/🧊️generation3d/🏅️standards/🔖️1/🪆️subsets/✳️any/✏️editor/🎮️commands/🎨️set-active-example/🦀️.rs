//! 🎨️ 🎨️ Generation3d play app commands command — `set-active-example`.

use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_editor_config::SetCamera;
use semio_s_artifact_procedural_3d_editor_config::SetSelectedGeneration;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

use semio_s_artifact_procedural_3d_model::empty_generation3d_snapshot;
use semio_s_artifact_procedural_3d_model::is_generation3d_example_id;
use semio_s_artifact_procedural_3d_io_text::snapshot::example_snapshot;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
use semio_framework_plugin::Effect;

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
    let effect = crate::set_active_example::reset_generation3d_document_effect(&target);
    let config_mutations = config_load_mutations(cfg.snapshot, &target.host_snapshot.camera, target.generation.selected_generation_id.clone());
    target.retire_cold();
    Ok(Emit { effects: vec![effect], config_mutations, ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

/// 🧬️ The whole-document replacement that picking an example or importing a file emits: the artifact's load effect, outside undo history and
/// never a mutation row. `store::os_store::text_format::empty_document_spr` (never a minted `create_document_envelope`) keeps the guest off the
/// `terminal shell reached Drop before its app-owned bounded retirement authority detached` trap on this path.
pub fn reset_generation3d_document_effect(document: &Generation3dSnapshot) -> Effect {
    let pack = <Generation3dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("generation3d", GENERATION_3D_SCHEMA));
    Effect::LoadDocument { pack, spr }
}
