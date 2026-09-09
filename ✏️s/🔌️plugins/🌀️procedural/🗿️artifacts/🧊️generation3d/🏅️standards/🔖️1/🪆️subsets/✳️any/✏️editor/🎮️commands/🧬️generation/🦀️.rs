//! 🧬️ Shared Generation3d generation-command projection and preview input owner.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::editor::generation3d::Generation3dCommand;
use crate::standards::v1::subsets::any::schema::generation_fixture_for;
use crate::standards::v1::subsets::any::schema::mutations::text::{generation_mutation_to_generation3d, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_artifact_playbook_playbook::{apply_generation_mutation, generation_operations, select_generation, selected_generation};
use semio_framework_os_flow::forms_bridge::flow_fixture_to_form_spec;
use semio_framework_plugin::Emit;

pub struct Generation3dGenerationCommandResult {
    pub emit: Emit<Generation3dMutation, Generation3dConfigMutation>,
    pub preview_fixture: Option<FlowFixture>,
}

pub fn generation_command_result(action: &str, args: Option<&dsl::DslValue>, projection: &Generation3dSnapshot, config: &Generation3dConfig) -> Option<Generation3dGenerationCommandResult> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.as_state().clone();
    state.selected_generation_id.clone_from(&config.selected_generation_id);
    let operations = if action == "selectGeneration" {
        let id = args.and_then(|value| value.get("id")).and_then(dsl::DslValue::as_str)?;
        select_generation(&mut state, id);
        Vec::new()
    } else {
        let operations = generation_operations(action, args, &state, &spec)?;
        for operation in &operations {
            apply_generation_mutation(&mut state, operation);
        }
        operations
    };
    let preview_fixture = selected_generation(&state).map(|_| generation_fixture_for(&projection.fixture, &state));
    Some(Generation3dGenerationCommandResult {
        emit: Emit {
            artifact_mutations: operations.into_iter().map(generation_mutation_to_generation3d).collect(),
            config_mutations: vec![Generation3dConfigMutation::SetSelectedGeneration { selected_generation_id: state.selected_generation_id.clone() }],
            coalesce_key: (action == "updateGenerationValues").then(|| "generation-values".to_string()),
            ..Default::default()
        },
        preview_fixture,
    })
}

pub fn generation_command_result_for(command: &Generation3dCommand, projection: &Generation3dSnapshot, config: &Generation3dConfig) -> Option<Generation3dGenerationCommandResult> {
    match command {
        Generation3dCommand::AddGeneration(_) => generation_command_result("addGeneration", None, projection, config),
        Generation3dCommand::RemoveGeneration(payload) => {
            let args = dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))]);
            generation_command_result("removeGeneration", Some(&args), projection, config)
        }
        Generation3dCommand::RenameGeneration(payload) => {
            let args = dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone())), ("name".to_string(), dsl::DslValue::String(payload.name.clone()))]);
            generation_command_result("renameGeneration", Some(&args), projection, config)
        }
        Generation3dCommand::UpdateGenerationValues(payload) => {
            let generation_id = payload.generation_id.clone().map_or(dsl::DslValue::Null, dsl::DslValue::String);
            let args = dsl::DslValue::object([("generationId".to_string(), generation_id), ("questionId".to_string(), dsl::DslValue::String(payload.question_id.clone())), ("value".to_string(), payload.value.clone())]);
            generation_command_result("updateGenerationValues", Some(&args), projection, config)
        }
        Generation3dCommand::SelectGeneration(payload) => {
            let args = dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))]);
            generation_command_result("selectGeneration", Some(&args), projection, config)
        }
        _ => None,
    }
}
