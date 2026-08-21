//! 🧬️ 🧬️ Procedural2d play app commands command — `add-generation`.

use crate::artifacts::procedural2d::op::{generation_mutation_to_procedural2d, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{apply_generation_mutation, generation_operations, select_generation, GenerationPlayState};
use flow::FlowEvalSession;
use flow::FlowFixture;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️PreviewHelper
/// 👁️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — recomputes the ephemeral generation preview for the currently selected generation and stores it
/// on the config (never on the persisted document). References [`Procedural2dConfig`], an app type, so
/// it stayed out of the artifact's `🧬️schema` unlike its sibling document helpers.
async fn refresh_generation_preview(config: &mut Procedural2dConfig, fixture: &FlowFixture, generation: &GenerationPlayState) {
    let Some(selected) = flow::playbook::selected_generation(generation) else {
        config.generation_preview_text = None;
        return;
    };
    let preview = crate::artifacts::procedural2d::schema::evaluate_generation_preview(fixture, &selected.values);
    config.generation_preview_text = Some(preview);
}
//#endregion 🔖️PreviewHelper

//#region 🔖️Shared
/// 🧬️ Emits generation operations for the generate-mode commands, updating the config's ephemeral
/// selection and preview from the post-operation state via a whole-config `Snapshot`.
/// `selectGeneration` is config-only (no document operations).
async fn handle_generation(action: &str, args: Option<&Value>, doc: &ArtifactView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Emit<Procedural2dMutation, Procedural2dConfigMutation> {
    let projection = doc.snapshot;
    let spec = flow_fixture_to_form_spec(&projection.fixture);
    let mut state = projection.generation.clone();
    state.selected_generation_id = cfg.snapshot.selected_generation_id.clone();
    let mut next_config = cfg.snapshot.clone();
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
    Emit { artifact_mutations: operations.into_iter().map(generation_mutation_to_procedural2d).collect(), config_mutations: vec![Procedural2dConfigMutation::Snapshot { config: next_config }], coalesce_key, ..Default::default() }
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

//#region 🔖️Generate
/// 🏷️ Named `enter_generate` (not `generate`) so it never collides with the app's `modes::generate`
/// mode module in the same `use` scope — the wire key/manifest action id both stay `"generate"`.
//#endregion 🔖️Generate

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-generation")]
pub struct AddGeneration {}

pub async fn handle(_payload: &AddGeneration, doc: &ArtifactView<'_, Procedural2dSnapshot>, cfg: &ConfigView<'_, Procedural2dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    Ok(handle_generation("addGeneration", None, doc, cfg, session))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::commands::enter_generate;
    use crate::editor::procedural2d::testkit::{app, dispatch};
    use crate::editor::procedural2d::Procedural2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn add_generation_records_an_undoable_generation_operation() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").generation.generations.len();
        dispatch(&mut app, Procedural2dCommand::AddGeneration(AddGeneration {}));
        assert_eq!(app.snapshot().expect("snapshot").generation.generations.len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn generate_is_a_view_action_with_no_artifact_mutations() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural2dCommand::Generate(enter_generate::Generate {}));
        assert_eq!(app.snapshot().expect("snapshot"), before, "generate must not mutate the document");
    }
}
//#endregion 🧪️Tests
