//! 🧬️ Shared Generation3d generation-command projection and preview input owner.

use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_model::generation_host_snapshot_for;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

use semio_s_artifact_procedural_3d_mutations::generation_mutation_to_generation3d;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::{apply_generation_mutation, generation_operations, select_generation, selected_generation};
use semio_framework_os_flow::forms_bridge::flow_host_snapshot_to_form_spec;
use semio_framework_plugin::Emit;

pub struct Generation3dGenerationCommandResult {
    pub emit: Emit<Generation3dMutation, Generation3dConfigMutation>,
    pub preview_fixture: Option<FlowHostSnapshot>,
}

pub fn generation_command_result(action: &str, args: Option<&semio_framework_value::DslValue>, projection: &Generation3dSnapshot, config: &Generation3dConfig) -> Option<Generation3dGenerationCommandResult> {
    let spec = flow_host_snapshot_to_form_spec(&projection.host_snapshot, &PlaybookFlowFormProducer);
    let mut state = projection.generation.as_state().clone();
    state.selected_generation_id.clone_from(&config.selected_generation_id);
    let operations = if action == "selectGeneration" {
        let id = args.and_then(|value| value.get("id")).and_then(semio_framework_value::DslValue::as_str)?;
        select_generation(&mut state, id);
        Vec::new()
    } else {
        let operations = generation_operations(action, args, &state, &spec)?;
        for operation in &operations {
            apply_generation_mutation(&mut state, operation);
        }
        operations
    };
    let preview_fixture = selected_generation(&state).map(|_| generation_host_snapshot_for(&projection.host_snapshot, &state, state.selected_generation_id.as_deref()));
    if semio_framework_job::runtime_diagnostics_enabled() {
        eprintln!("[TRACE] gen3d command action={action} before={} after={} ops={} selected={:?}", projection.generation.as_state().generations.len(), state.generations.len(), operations.len(), state.selected_generation_id);
    }
    let config_mutations = (state.selected_generation_id != config.selected_generation_id)
        .then(|| Generation3dConfigMutation::SetSelectedGeneration(semio_s_artifact_procedural_3d_editor_config::SetSelectedGeneration { selected_generation_id: state.selected_generation_id.clone() }))
        .into_iter()
        .collect();
    Some(Generation3dGenerationCommandResult {
        emit: Emit { artifact_mutations: operations.into_iter().map(generation_mutation_to_generation3d).collect(), config_mutations, ..Default::default() },
        preview_fixture,
    })
}

