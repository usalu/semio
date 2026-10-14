//! 🧬️ Shared Generation2d generation-command state transition.

use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;

use semio_s_artifact_procedural_2d_mutations::generation_mutation_to_generation2d;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_artifact_playbook_playbook::{apply_generation_mutation, generation_operations, select_generation, selected_generation, PlaybookValues};
use semio_framework_os_flow::forms_bridge::flow_host_snapshot_to_form_spec;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit};

pub struct Generation2dGenerationCommandResult {
    pub emit: Emit<Generation2dMutation, Generation2dConfigMutation>,
    pub preview_values: Option<PlaybookValues>,
    pub publishes_preview: bool,
}

pub fn handle_generation(action: &str, args: Option<&semio_framework_value::DslValue>, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Generation2dGenerationCommandResult {
    let projection = doc.snapshot;
    let spec = flow_host_snapshot_to_form_spec(&projection.host_snapshot, &PlaybookFlowFormProducer);
    let mut state = projection.generation.as_state().clone();
    state.selected_generation_id = cfg.snapshot.selected_generation_id.clone();
    if action == "selectGeneration" {
        if let Some(id) = args.and_then(|value| value.get("id")).and_then(semio_framework_value::DslValue::as_str) {
            select_generation(&mut state, id);
        }
        return Generation2dGenerationCommandResult {
            emit: Emit::config(vec![Generation2dConfigMutation::SetSelectedGeneration(semio_s_artifact_procedural_2d_editor_config::SetSelectedGenerationSetting { selected_generation_id: state.selected_generation_id.clone() })]),
            preview_values: selected_generation(&state).map(|selected| selected.values.clone()),
            publishes_preview: true,
        };
    }
    let Some(operations) = generation_operations(action, args, &state, &spec) else {
        return Generation2dGenerationCommandResult { emit: Emit::default(), preview_values: None, publishes_preview: false };
    };
    for operation in &operations {
        apply_generation_mutation(&mut state, operation);
    }
    let config_mutations = (state.selected_generation_id != cfg.snapshot.selected_generation_id).then(|| Generation2dConfigMutation::SetSelectedGeneration(semio_s_artifact_procedural_2d_editor_config::SetSelectedGenerationSetting { selected_generation_id: state.selected_generation_id.clone() })).into_iter().collect();
    Generation2dGenerationCommandResult {
        emit: Emit { artifact_mutations: operations.into_iter().map(generation_mutation_to_generation2d).collect(), config_mutations, ..Default::default() },
        preview_values: selected_generation(&state).map(|selected| selected.values.clone()),
        publishes_preview: true,
    }
}

