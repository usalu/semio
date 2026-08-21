//! 🧬️ 🧬️ Procedural3d play app commands command — `update-generation-values`.

use crate::artifacts::procedural3d::op::{generation_mutation_to_procedural3d, Procedural3dMutation};
use crate::artifacts::procedural3d::schema::evaluate_generation_preview;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{apply_generation_mutation, generation_operations, selected_generation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `flow::playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
async fn handle_generation(action: &str, args: Option<&Value>, projection: &Procedural3dSnapshot, cfg: &Procedural3dConfig) -> Emit<Procedural3dMutation, Procedural3dConfigMutation> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.clone();
    state.selected_generation_id = cfg.selected_generation_id.clone();
    let Some(operations) = generation_operations(action, args, &state, &spec) else {
        return Emit::default();
    };
    for operation in &operations {
        apply_generation_mutation(&mut state, operation);
    }
    let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(&projection.fixture, &selected.values));
    let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
    Emit {
        artifact_mutations: operations.into_iter().map(generation_mutation_to_procedural3d).collect(),
        config_mutations: vec![Procedural3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
        coalesce_key,
        ..Default::default()
    }
}
//#endregion 🔖️Shared

//#region 🔖️AddGeneration
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
//#endregion 🔖️RemoveGeneration

//#region 🔖️RenameGeneration
//#endregion 🔖️RenameGeneration

//#region 🔖️UpdateGenerationValues
//#endregion 🔖️UpdateGenerationValues

//#region 🔖️SelectGeneration
//#endregion 🔖️SelectGeneration

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "update-generation-values")]
pub struct UpdateGenerationValues {
    pub generation_id: Option<String>,
    pub question_id: String,
    pub value: dsl::DslValue,
}

pub async fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let value_json = dsl::from_dsl_value(payload.value.clone()).unwrap_or(Value::Null);
    Ok(handle_generation("updateGenerationValues", Some(&json!({ "generationId": payload.generation_id, "questionId": payload.question_id, "value": value_json })), doc.snapshot, cfg.snapshot))
}
