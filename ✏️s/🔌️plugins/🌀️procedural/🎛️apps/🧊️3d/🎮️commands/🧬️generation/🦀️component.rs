//! 🧬️ Procedural3d play app commands — generation authoring and selection.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::engine::evaluate_generation_preview;
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::FlowEvalSession;
use playbook::{apply_generation_operation, generation_operations, select_generation, selected_generation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode document-mutating commands — reuses
/// `playbook::generation_operations`'s id-generation/values-seeding logic via a synthetic JSON args
/// value built from the typed command fields.
fn handle_generation(action: &str, args: Option<&Value>, projection: &Procedural3dDocument, cfg: &Procedural3dConfig) -> Emit<Procedural3dOperation, Procedural3dConfigOperation> {
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.clone();
    state.selected_generation_id = cfg.selected_generation_id.clone();
    let Some(operations) = generation_operations(action, args, &state, &spec) else {
        return Emit::default();
    };
    for operation in &operations {
        apply_generation_operation(&mut state, operation);
    }
    let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(&projection.fixture, &selected.values));
    let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
    Emit {
        document_operations: operations.into_iter().map(Procedural3dOperation::Generation).collect(),
        config_operations: vec![Procedural3dConfigOperation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }],
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

    pub fn handle(_payload: &AddGeneration, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(handle_generation("addGeneration", None, doc.projection, cfg.projection))
    }
}
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
pub mod remove_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-generation")]
    pub struct RemoveGeneration {
        pub id: String,
    }

    pub fn handle(payload: &RemoveGeneration, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(handle_generation("removeGeneration", Some(&json!({ "id": payload.id })), doc.projection, cfg.projection))
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
        pub name: String,
    }

    pub fn handle(payload: &RenameGeneration, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        Ok(handle_generation("renameGeneration", Some(&json!({ "id": payload.id, "name": payload.name })), doc.projection, cfg.projection))
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
        pub value: dsl::DslValue,
    }

    pub fn handle(payload: &UpdateGenerationValues, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let value_json = dsl::from_dsl_value(payload.value.clone()).unwrap_or(Value::Null);
        Ok(handle_generation("updateGenerationValues", Some(&json!({ "generationId": payload.generation_id, "questionId": payload.question_id, "value": value_json })), doc.projection, cfg.projection))
    }
}
//#endregion 🔖️UpdateGenerationValues

//#region 🔖️SelectGeneration
pub mod select_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-generation")]
    pub struct SelectGeneration {
        pub id: String,
    }

    pub fn handle(payload: &SelectGeneration, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let mut state = doc.projection.generation.clone();
        state.selected_generation_id = cfg.projection.selected_generation_id.clone();
        select_generation(&mut state, &payload.id);
        let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(fixture, &selected.values));
        Ok(Emit::config(vec![Procedural3dConfigOperation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }]))
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
        assert_undo_redo_round_trip(&mut app, Procedural3dCommand::AddGeneration(add_generation::AddGeneration {}), |app| app.projection().expect("projection").generation.generations.len(), 0, 1);
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
        let before = app.projection().expect("projection");
        let generation_id = before.generation.generations.first().expect("generation").id.clone();
        dispatch(&mut app, Procedural3dCommand::SelectGeneration(select_generation::SelectGeneration { id: generation_id }));
        assert_eq!(app.projection().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
