//! 🧬️ 🧬️ Generate-mode commands command — `update-generation-values`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::seed_host_catalogue;
use crate::editor::flow::FLOW_PLAY_APP_ID;
use crate::playbook::{handle_generation_action, selected_generation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use flow::{
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    FlowEvalSession, FlowHost,
    };

//#region 🔖️SharedDispatch
async fn evaluate_generation_preview(fixture: &FlowSnapshot, config: &FlowConfig, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.to_fixture());
    let mut host = FlowHost::from_fixture(patched_fixture);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

/// 🧬️ Shared body for all five Generate-mode commands — one `playbook` CRUD call, then (for the three
/// verbs that change which values are active) a fresh preview evaluation seeded into the eval session.
async fn handle_generation(action_id: &str, args: Option<&Value>, fixture: &FlowSnapshot, config: &FlowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, FlowConfigMutation> {
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
//#endregion 🔖️AddGeneration

//#region 🔖️RemoveGeneration
//#endregion 🔖️RemoveGeneration

//#region 🔖️SelectGeneration
//#endregion 🔖️SelectGeneration

//#region 🔖️RenameGeneration
//#endregion 🔖️RenameGeneration

//#region 🔖️UpdateGenerationValues
//#endregion 🔖️UpdateGenerationValues

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct UpdateGenerationValues {
    pub generation_id: Option<String>,
    pub question_id: String,
    pub value: dsl::DslValue,
}

pub async fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let value_json: Value = dsl::from_dsl_value(payload.value.clone()).unwrap_or(Value::Null);
    Ok(handle_generation("updateGenerationValues", Some(&json!({ "generationId": payload.generation_id, "questionId": payload.question_id, "value": value_json })), doc.snapshot, cfg.snapshot, session))
}
