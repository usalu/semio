//! 🧬️ 🧬️ Generation3d play app commands command — `remove-generation`.

use crate::standards::v1::subsets::any::schema::mutations::text::{generation_mutation_to_generation3d, Generation3dMutation};
use crate::standards::v1::subsets::any::schema::evaluate_generation_preview;
use crate::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use semio_framework_os_flow::forms_bridge::flow_fixture_to_form_spec;
use semio_framework_artifact_playbook_playbook::{apply_generation_mutation, generation_operations, selected_generation};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `semio_framework_artifact_playbook_playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
fn handle_generation(action: &str, args: Option<&dsl::DslValue>, projection: &Generation3dSnapshot, cfg: &Generation3dConfig) -> Emit<Generation3dMutation, Generation3dConfigMutation> {
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
#[dsl(keyword = "remove-generation")]
pub struct RemoveGeneration {
    pub id: String,
}

pub fn handle(payload: &RemoveGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(handle_generation("removeGeneration", Some(&dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))])), doc.snapshot, cfg.snapshot))
}
