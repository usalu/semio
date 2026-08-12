//! 🧬️ Generate-mode commands — the generation CRUD surface (add / remove / select / rename / update).
//!
//! All five bridge into `playbook::handle_generation_action`'s still-untyped `args: Option<&Value>` CRUD
//! surface (out of scope to convert — it lives in the `playbook` kernel crate), so each payload's
//! `handle` re-serializes its typed fields into the JSON that helper expects. Flow keeps its generations
//! CONFIG-tracked rather than document-operation-backed (unlike the sibling `procedural_3d`/`procedural_2d`
//! apps) since flow's document model is a shared kernel crate — see `FlowConfig::generation_json`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::seed_host_catalogue;
use crate::apps::flow::FLOW_PLAY_APP_ID;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::{
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    FlowEvalSession, FlowHost,
};
use crate::playbook::{handle_generation_action, selected_generation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

//#region 🔖️SharedDispatch
fn evaluate_generation_preview(fixture: &FlowSnapshot, config: &FlowConfig, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.to_fixture());
    let mut host = FlowHost::from_fixture(patched_fixture);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

/// 🧬️ Shared body for all five Generate-mode commands — one `playbook` CRUD call, then (for the three
/// verbs that change which values are active) a fresh preview evaluation seeded into the eval session.
fn handle_generation(action_id: &str, args: Option<&Value>, fixture: &FlowSnapshot, config: &FlowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, FlowConfigMutation> {
    let spec = flow_fixture_to_form_spec(&fixture.to_fixture());
    let mut generation = config.generation();
    if !handle_generation_action(action_id, args, &mut generation, &spec, FLOW_PLAY_APP_ID) {
        return Emit::default();
    }
    let mut config_mutations = Vec::new();
    if matches!(action_id, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
        match selected_generation(&generation) {
            Some(active) => {
                let preview = evaluate_generation_preview(fixture, config, &active.values.clone());
                generation.preview_text = Some(preview.clone());
                session.set_eval_json(preview);
            }
            None => generation.preview_text = None,
        }
    }
    config_mutations.insert(0, FlowConfigMutation::SetGeneration { json: serde_json::to_string(&generation).unwrap_or_default() });
    let coalesce_key = (action_id == "updateGenerationValues").then(|| "generation-values".to_string());
    Emit { config_mutations, coalesce_key, ..Default::default() }
}
//#endregion 🔖️SharedDispatch

//#region 🔖️AddGeneration
pub mod add_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct AddGeneration {}

    pub fn handle(_payload: &AddGeneration, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(handle_generation("addGeneration", None, doc.snapshot, cfg.snapshot, session))
    }
}
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
pub mod remove_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct RemoveGeneration {
        pub id: String,
    }

    pub fn handle(payload: &RemoveGeneration, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(handle_generation("removeGeneration", Some(&json!({ "id": payload.id })), doc.snapshot, cfg.snapshot, session))
    }
}
//#endregion 🔖️RemoveGeneration

//#region 🔖️SelectGeneration
pub mod select_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SelectGeneration {
        pub id: String,
    }

    pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(handle_generation("selectGeneration", Some(&json!({ "id": payload.id })), doc.snapshot, cfg.snapshot, session))
    }
}
//#endregion 🔖️SelectGeneration

//#region 🔖️RenameGeneration
pub mod rename_generation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct RenameGeneration {
        pub id: String,
        pub name: String,
    }

    pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        Ok(handle_generation("renameGeneration", Some(&json!({ "id": payload.id, "name": payload.name })), doc.snapshot, cfg.snapshot, session))
    }
}
//#endregion 🔖️RenameGeneration

//#region 🔖️UpdateGenerationValues
pub mod update_generation_values {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct UpdateGenerationValues {
        pub generation_id: Option<String>,
        pub question_id: String,
        pub value: dsl::DslValue,
    }

    pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        let value_json: Value = dsl::from_dsl_value(payload.value.clone()).unwrap_or(Value::Null);
        Ok(handle_generation("updateGenerationValues", Some(&json!({ "generationId": payload.generation_id, "questionId": payload.question_id, "value": value_json })), doc.snapshot, cfg.snapshot, session))
    }
}
//#endregion 🔖️UpdateGenerationValues

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::modes::generate::windows::{form, generations};
    use crate::apps::flow::testkit::{dispatch, flow_app, render};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn adding_a_generation_populates_the_form_and_emits_no_artifact_mutations() {
        let mut app = flow_app();
        assert!(render(&mut app, form::FLOW_PLAY_BODY_GENERATE_FORM).contains("Add a generation"), "the form starts empty");
        let result = dispatch(&mut app, FlowCommand::AddGeneration(add_generation::AddGeneration {}));
        assert!(result.mutations.is_empty(), "generations are config state, never document operations");
        assert!(render(&mut app, generations::FLOW_PLAY_BODY_GENERATIONS).contains("selectGeneration"), "the new generation lands in the list");
    }

    #[test]
    fn removing_an_unknown_generation_is_a_no_operation() {
        let mut app = flow_app();
        let result = dispatch(&mut app, FlowCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "nope".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
