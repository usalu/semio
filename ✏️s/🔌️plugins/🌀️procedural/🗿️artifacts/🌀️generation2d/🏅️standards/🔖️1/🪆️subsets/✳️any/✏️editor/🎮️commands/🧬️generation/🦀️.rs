//! 🧬️ Shared Generation2d generation-command state transition.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::editor::generation2d::Generation2dCommand;
use crate::standards::v1::subsets::any::schema::mutations::{Generation2dMutation};

use crate::standards::v1::subsets::any::schema::mutations::{generation_mutation_to_generation2d};

use crate::Generation2dSnapshot;
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
    let spec = flow_host_snapshot_to_form_spec(&projection.host_snapshot);
    let mut state = projection.generation.as_state().clone();
    state.selected_generation_id = cfg.snapshot.selected_generation_id.clone();
    if action == "selectGeneration" {
        if let Some(id) = args.and_then(|value| value.get("id")).and_then(semio_framework_value::DslValue::as_str) {
            select_generation(&mut state, id);
        }
        return Generation2dGenerationCommandResult {
            emit: Emit::config(vec![Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: state.selected_generation_id.clone() }]),
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
    let config_mutations = (state.selected_generation_id != cfg.snapshot.selected_generation_id).then(|| Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: state.selected_generation_id.clone() }).into_iter().collect();
    Generation2dGenerationCommandResult {
        emit: Emit { artifact_mutations: operations.into_iter().map(generation_mutation_to_generation2d).collect(), config_mutations, ..Default::default() },
        preview_values: selected_generation(&state).map(|selected| selected.values.clone()),
        publishes_preview: true,
    }
}

pub fn prepare_command(command: &Generation2dCommand, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Option<Generation2dGenerationCommandResult> {
    match command {
        Generation2dCommand::AddGeneration(_) => Some(handle_generation("addGeneration", None, doc, cfg)),
        Generation2dCommand::RemoveGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(payload.id.clone()))]);
            Some(handle_generation("removeGeneration", Some(&args), doc, cfg))
        }
        Generation2dCommand::RenameGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".into(), semio_framework_value::DslValue::String(payload.name.clone()))]);
            Some(handle_generation("renameGeneration", Some(&args), doc, cfg))
        }
        Generation2dCommand::UpdateGenerationValues(payload) => {
            let args = semio_framework_value::DslValue::object([
                ("generationId".into(), payload.generation_id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String)),
                ("questionId".into(), semio_framework_value::DslValue::String(payload.question_id.clone())),
                ("value".into(), payload.value.clone()),
            ]);
            Some(handle_generation("updateGenerationValues", Some(&args), doc, cfg))
        }
        Generation2dCommand::SelectGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".into(), payload.id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String))]);
            Some(handle_generation("selectGeneration", Some(&args), doc, cfg))
        }
        _ => None,
    }
}
