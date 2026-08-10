//! 🧬️ Procedural3d play app commands — generation authoring and selection.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::engine::evaluate_generation_preview;
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::FlowEvalSession;
use flow::playbook::{apply_generation_mutation, generation_operations, select_generation, selected_generation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `flow::playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
fn handle_generation(action: &str, args: Option<&Value>, projection: &Procedural3dSnapshot, cfg: &Procedural3dConfig) -> Emit<Procedural3dMutation, Procedural3dConfigMutation> {
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
        artifact_mutations: operations.into_iter().map(Procedural3dMutation::Generation).collect(),
        config_mutations: vec![Procedural3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
        coalesce_key,
        ..Default::default()
    }
}
//#endregion 🔖️Shared

//#region 🔖️AddGeneration
pub mod add_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-generation")]
    pub struct AddGeneration {}

    pub fn handle(_payload: &AddGeneration, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(handle_generation("addGeneration", None, doc.snapshot, cfg.snapshot))
    }
}
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
pub mod remove_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-generation")]
    pub struct RemoveGeneration {
        pub id: String}

    pub fn handle(payload: &RemoveGeneration, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(handle_generation("removeGeneration", Some(&json!({ "id": payload.id })), doc.snapshot, cfg.snapshot))
    }
}
//#endregion 🔖️RemoveGeneration

//#region 🔖️RenameGeneration
pub mod rename_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "rename-generation")]
    pub struct RenameGeneration {
        pub id: String,
        pub name: String}

    pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        Ok(handle_generation("renameGeneration", Some(&json!({ "id": payload.id, "name": payload.name })), doc.snapshot, cfg.snapshot))
    }
}
//#endregion 🔖️RenameGeneration

//#region 🔖️UpdateGenerationValues
pub mod update_generation_values {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "update-generation-values")]
    pub struct UpdateGenerationValues {
        pub generation_id: Option<String>,
        pub question_id: String,
        pub value: dsl::DslValue}

    pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let value_json = dsl::from_dsl_value(payload.value.clone()).unwrap_or(Value::Null);
        Ok(handle_generation("updateGenerationValues", Some(&json!({ "generationId": payload.generation_id, "questionId": payload.question_id, "value": value_json })), doc.snapshot, cfg.snapshot))
    }
}
//#endregion 🔖️UpdateGenerationValues

//#region 🔖️SelectGeneration
pub mod select_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-generation")]
    pub struct SelectGeneration {
        pub id: String}

    pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
        let fixture = &doc.snapshot.fixture;
        let mut state = doc.snapshot.generation.clone();
        state.selected_generation_id = cfg.snapshot.selected_generation_id.clone();
        select_generation(&mut state, &payload.id);
        let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(fixture, &selected.values));
        Ok(Emit::config(vec![Procedural3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }]))
    }
}
//#endregion 🔖️SelectGeneration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddGeneration(add_generation::AddGeneration {}), |app| app.snapshot().expect("snapshot").generation.generations.len(), 0, 1);
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        assert!(crate::apps::procedural3d::testkit::render(&mut app, crate::apps::procedural3d::modes::generate::windows::generations::PROCEDURAL_3D_PLAY_BODY_GENERATIONS).contains("addGeneration"));
    }

    #[test]
    fn select_generation_does_not_mutate_the_document() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::AddGeneration(add_generation::AddGeneration {}));
        let before = app.snapshot().expect("snapshot");
        let generation_id = before.generation.generations.first().expect("generation").id.clone();
        dispatch(&mut app, Procedural3dCommand::SelectGeneration(select_generation::SelectGeneration { id: generation_id }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
