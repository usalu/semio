//! 📥️ 📥️ Forms play app commands command — `set-spec-json`.

use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::reset_try_config_mutations;
use crate::artifacts::forms::schema::{default_example_spec, empty_forms_snapshot, onboarding_example_spec};
use crate::artifacts::forms::dsl as forms_dsl;
use crate::artifacts::forms::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

// 🧷️ Aliased: the payload structs below derive the EXTERN `dsl` crate's `dsl::DslRecord` — importing the
// artifact's own `dsl` submodule under the bare name would shadow it.

//#region 🔖️Shell
/// ✏️ Emits the operations that replace the current form spec's title + steps with those of `next` — a
/// legitimate whole-document swap for import/example-switch, expressed granularly through the existing
/// `FormMutation` vocabulary (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `CreateStep`/
/// `DeleteStep`/`ChangeFormTitle`, reading through `forms_steps` now that `FormsSnapshot` no longer
/// carries a bare `steps` field) so it still records a true inverse.
fn replace_spec_operations(current: &FormsSnapshot, next: &FormsSnapshot) -> Vec<FormMutation> {
    use crate::artifacts::forms::mutations::{change_form_title, create_step, delete_step};
    let mut operations: Vec<FormMutation> = forms_steps(current).iter().map(|step| FormMutation::DeleteStep(delete_step::mutation::DeleteStep { id: step.id.clone() })).collect();
    if next.title != current.title {
        operations.push(FormMutation::ChangeFormTitle(change_form_title::mutation::ChangeFormTitle { new_title: next.title.clone() }));
    }
    for step in forms_steps(next) {
        operations.push(FormMutation::CreateStep(create_step::mutation::CreateStep { step, index: None }));
    }
    operations
}
//#endregion 🔖️Shell



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "spec-json")]
pub struct SetSpecJson {
    pub json: String,
}

pub fn handle(payload: &SetSpecJson, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    // 🩹️ `FormsSnapshot` composes `structure`/`results` handles (ticket
    // 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM) so it no longer deserializes raw step/block
    // JSON directly; `flow::playbook::PlaybookSpec` is the SAME `{schema,id,version,title,steps}`
    // camelCase shape `FormsSnapshot` used before composition, so it stays the deserialize target.
    let Ok(spec) = serde_json::from_str::<flow::playbook::PlaybookSpec>(&payload.json) else {
        return Ok(Emit::default());
    };
    let next = crate::artifacts::forms::forms_snapshot_with_state(spec.schema, spec.id, spec.version, spec.title, spec.steps);
    let mut config_mutations = reset_try_config_mutations();
    config_mutations.push(FormsConfigMutation::SetSelection { ids: Vec::new() });
    Ok(Emit { artifact_mutations: replace_spec_operations(doc.snapshot, &next), config_mutations, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{dispatch, forms_app};
    use crate::apps::forms::FormsCommand;
    use set_active_example::SetActiveExample;
    use SetSpecJson;

    #[test]
    fn set_active_example_switches_to_the_onboarding_fixture() {
        // 🩹️ `replace_spec_operations` deliberately never touches `id` (only title/steps — `id` is the
        // document's own stable identity, not part of the "example" content it swaps) — assert on the
        // steps/title it does replace, not on `id`.
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::SetActiveExample(SetActiveExample { example_id: "onboarding".into() }));
        let spec = app.snapshot().expect("projection");
        assert_eq!(forms_steps(&spec).len(), 3);
        assert_eq!(spec.title, onboarding_example_spec().title);
    }

    #[test]
    fn set_active_example_with_blank_id_clears_the_document() {
        let mut app = forms_app();
        dispatch(&mut app, FormsCommand::SetActiveExample(SetActiveExample { example_id: "".into() }));
        let spec = app.snapshot().expect("projection");
        assert!(crate::artifacts::forms::schema::flatten_questions(&spec).is_empty());
    }

    #[test]
    fn set_spec_json_replaces_the_document() {
        // 🩹️ `SetSpecJson`'s payload is raw `flow::playbook::PlaybookSpec`-shaped JSON (see
        // `handle`'s own doc comment, ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM)
        // — `onboarding_example_spec()` itself now serializes as `FormsSnapshot`'s OWN composed
        // `structure`/`results`-handle shape, so the test input is built from the playbook spec
        // directly, not from `serde_json::to_string(&onboarding_example_spec())`.
        let mut app = forms_app();
        let onboarding_snapshot = onboarding_example_spec();
        let onboarding_playbook = flow::playbook::PlaybookSpec {
            schema: onboarding_snapshot.schema.clone(),
            id: onboarding_snapshot.id.clone(),
            version: onboarding_snapshot.version.clone(),
            title: onboarding_snapshot.title.clone(),
            steps: forms_steps(&onboarding_snapshot),
        };
        let onboarding = serde_json::to_string(&onboarding_playbook).unwrap();
        dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: onboarding }));
        let spec = app.snapshot().expect("projection");
        assert_eq!(forms_steps(&spec).len(), 3);
        assert_eq!(spec.title, onboarding_example_spec().title);
    }

    #[test]
    fn set_spec_json_with_invalid_json_is_a_no_operation() {
        let mut app = forms_app();
        let before = app.snapshot().expect("projection");
        dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: "not json".into() }));
        assert_eq!(app.snapshot().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
