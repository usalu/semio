//! 🧬️ Shared Generation3d generation-command projection.

use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

use semio_s_artifact_procedural_3d_mutations::generation_mutation_to_generation3d;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::{apply_generation_mutation, generation_operations, select_generation, GenerationMutation, GenerationPlayState};
use semio_os_flow::forms_bridge::flow_host_snapshot_to_form_spec;
use semio_framework_value::FromValue;
use semio_plugin::Emit;

pub struct Generation3dGenerationCommandResult {
    pub emit: Emit<Generation3dMutation, Generation3dConfigMutation>,
}

pub fn generation_command_result(action: &str, args: Option<&semio_framework_value::DslValue>, projection: &Generation3dSnapshot, config: &Generation3dConfig) -> Option<Generation3dGenerationCommandResult> {
    let spec = flow_host_snapshot_to_form_spec(&projection.host_snapshot, &PlaybookFlowFormProducer);
    let mut state = projection.generation.as_state().clone();
    state.selected_generation_id.clone_from(&config.selected_generation_id);
    let result = (|| {
        let operations = if action == "selectGeneration" {
            let id = args.and_then(|value| value.get("id")).and_then(semio_framework_value::DslValue::as_str)?;
            select_generation(&mut state, id);
            Vec::new()
        } else {
            let operations = generation_operations(action, args, &state, &spec)?;
            for operation in &operations {
                if matches!(operation, GenerationMutation::Add { .. } | GenerationMutation::Remove { .. }) {
                    apply_generation_mutation(&mut state, operation);
                }
            }
            operations
        };
        let config_mutations = (state.selected_generation_id != config.selected_generation_id)
            .then(|| Generation3dConfigMutation::SetSelectedGeneration(semio_s_artifact_procedural_3d_editor_config::SetSelectedGeneration { selected_generation_id: state.selected_generation_id.clone() }))
            .into_iter()
            .collect();
        Some(Generation3dGenerationCommandResult {
            emit: Emit { artifact_mutations: operations.into_iter().map(generation_mutation_to_generation3d).collect(), config_mutations, ..Default::default() },
        })
    })();
    GenerationPlayState::retire_decoded(state);
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::update_generation_values::UpdateGenerationValues;
    use semio_framework_artifact_flow_flow::Widget;
    use semio_framework_artifact_playbook_playbook::{FormGeneration, PlaybookValues};
    use semio_framework_value::DslValue;

    #[test]
    fn generation3d_axis_action_emits_one_complete_vector_mutation() {
        let mut snapshot = Generation3dSnapshot::default();
        snapshot.host_snapshot.replace_widgets(vec![
            Widget::Variable { id: "axis".into(), name: "Extrusion".into(), schema: "vector".into() }
        ]);
        let values: PlaybookValues = [("axis".to_string(), DslValue::Array([1.0, 2.0, 3.0].into_iter().map(DslValue::float).collect()))].into_iter().collect();
        snapshot.generation = GenerationPlayState {
            generations: vec![FormGeneration { id: "g".into(), name: "Variant".into(), values }],
            ..Default::default()
        }.into();
        let config = Generation3dConfig { selected_generation_id: Some("g".into()), ..Default::default() };
        let payload = UpdateGenerationValues { generation_id: None, question_id: "axis".into(), field_index: Some(1), value: DslValue::float(-5.0) };
        let emitted = generation_command_result("updateGenerationValues", Some(&payload.arguments()), &snapshot, &config).unwrap().emit;
        let [Generation3dMutation::ChangeGenerationValue(change)] = emitted.artifact_mutations.as_slice() else {
            panic!("expected one typed generation mutation");
        };
        assert_eq!(change.id, "g");
        assert_eq!(change.question_id, "axis");
        assert_eq!(change.new_value, DslValue::Array([1.0, -5.0, 3.0].into_iter().map(DslValue::float).collect()));
        assert!(emitted.config_mutations.is_empty());
        snapshot.retire_cold();
    }
}
