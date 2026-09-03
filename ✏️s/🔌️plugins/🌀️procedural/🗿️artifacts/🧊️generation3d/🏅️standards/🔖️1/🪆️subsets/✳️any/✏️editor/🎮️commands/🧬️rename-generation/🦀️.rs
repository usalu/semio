//! 🧬️ 🧬️ Generation3d play app commands command — `rename-generation`.

use crate::artifacts::generation3d::op::{generation_mutation_to_generation3d, Generation3dMutation};
use crate::artifacts::generation3d::schema::evaluate_generation_preview;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{apply_generation_mutation, generation_operations, selected_generation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde_json::{json, Value};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `flow::playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
fn handle_generation(action: &str, args: Option<&Value>, projection: &Generation3dSnapshot, cfg: &Generation3dConfig) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.as_state().clone();
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
        artifact_mutations: operations.into_iter().map(generation_mutation_to_generation3d).collect(),
        config_mutations: vec![Generation3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "rename-generation")]
pub struct RenameGeneration {
    pub id: String,
    pub name: String,
}

pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(handle_generation("renameGeneration", Some(&json!({ "id": payload.id, "name": payload.name })), doc.snapshot, cfg.snapshot))
}
