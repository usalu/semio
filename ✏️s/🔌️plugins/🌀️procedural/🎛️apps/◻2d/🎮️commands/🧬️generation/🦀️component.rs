//! 🧬️ Procedural2d play app commands — generation authoring/selection and entering generate mode.
//!
//! `AddGeneration`/`RemoveGeneration`/`RenameGeneration`/`UpdateGenerationValues`/`SelectGeneration` all
//! route through the shared `handle_generation` helper (the pre-migration `Procedural2dPlayApp::handle_generation`
//! method, now a free function taking the action id + a synthetic JSON args value built from the typed
//! payload fields — mirrors `procedural_3d`'s identical shape).

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::engine::refresh_generation_preview;
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dDocument;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::FlowEvalSession;
use flow::playbook::{apply_generation_mutation, generation_operations, select_generation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode commands, updating the config's ephemeral
/// selection and preview from the post-operation state via a whole-config `Snapshot`.
/// `selectGeneration` is config-only (no document operations).
fn handle_generation(action: &str, args: Option<&Value>, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Emit<Procedural2dMutation, Procedural2dConfigMutation> {
    let projection = doc.projection;
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.clone();
    state.selected_generation_id = cfg.projection.selected_generation_id.clone();
    let mut next_config = cfg.projection.clone();
    if action == "selectGeneration" {
        if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
            select_generation(&mut state, id);
        }
        next_config.selected_generation_id = state.selected_generation_id.clone();
        refresh_generation_preview(&mut next_config, &projection.fixture, &state);
        if let Some(preview) = next_config.generation_preview_text.clone() {
            session.set_eval_json(preview);
        }
        return Emit::config(vec![Procedural2dConfigMutation::Snapshot { config: next_config }]);
    }
    let Some(operations) = generation_operations(action, args, &state, &spec) else {
        return Emit::default();
    };
    for operation in &operations {
        apply_generation_mutation(&mut state, operation);
    }
    next_config.selected_generation_id = state.selected_generation_id.clone();
    refresh_generation_preview(&mut next_config, &projection.fixture, &state);
    if let Some(preview) = next_config.generation_preview_text.clone() {
        session.set_eval_json(preview);
    }
    let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
    Emit { document_mutations: operations.into_iter().map(Procedural2dMutation::Generation).collect(), config_mutations: vec![Procedural2dConfigMutation::Snapshot { config: next_config }], coalesce_key, ..Default::default() }
}
//#endregion 🔖️Shared

//#region 🔖️AddGeneration
pub mod add_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-generation")]
    pub struct AddGeneration {}

    pub fn handle(_payload: &AddGeneration, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(handle_generation("addGeneration", None, doc, cfg, session))
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

    pub fn handle(payload: &RemoveGeneration, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(handle_generation("removeGeneration", Some(&json!({ "id": payload.id })), doc, cfg, session))
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

    pub fn handle(payload: &RenameGeneration, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(handle_generation("renameGeneration", Some(&json!({ "id": payload.id, "name": payload.name })), doc, cfg, session))
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

    pub fn handle(payload: &UpdateGenerationValues, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        let value_json = dsl::from_dsl_value(payload.value.clone()).unwrap_or(Value::Null);
        Ok(handle_generation("updateGenerationValues", Some(&json!({ "generationId": payload.generation_id, "questionId": payload.question_id, "value": value_json })), doc, cfg, session))
    }
}
//#endregion 🔖️UpdateGenerationValues

//#region 🔖️SelectGeneration
pub mod select_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-generation")]
    pub struct SelectGeneration {
        pub id: Option<String>,
    }

    pub fn handle(payload: &SelectGeneration, doc: &DocumentView<'_, Procedural2dDocument>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(handle_generation("selectGeneration", Some(&json!({ "id": payload.id })), doc, cfg, session))
    }
}
//#endregion 🔖️SelectGeneration

//#region 🔖️Generate
/// 🏷️ Named `enter_generate` (not `generate`) so it never collides with the app's `modes::generate`
/// mode module in the same `use` scope — the wire key/manifest action id both stay `"generate"`.
pub mod enter_generate {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "generate")]
    pub struct Generate {}

    pub fn handle(_payload: &Generate, _doc: &DocumentView<'_, Procedural2dDocument>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Procedural2dConfigMutation::SetShowMode { value: "generate".into() }]))
    }
}
//#endregion 🔖️Generate

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let mut app = app();
        let before = app.projection().expect("projection").generation.generations.len();
        dispatch(&mut app, Procedural2dCommand::AddGeneration(add_generation::AddGeneration {}));
        assert_eq!(app.projection().expect("projection").generation.generations.len(), before + 1);
    }

    #[test]
    fn generate_is_a_view_action_with_no_document_mutations() {
        let mut app = app();
        let before = app.projection().expect("projection");
        dispatch(&mut app, Procedural2dCommand::Generate(enter_generate::Generate {}));
        assert_eq!(app.projection().expect("projection"), before, "generate must not mutate the document");
    }
}
//#endregion 🧪️Tests
