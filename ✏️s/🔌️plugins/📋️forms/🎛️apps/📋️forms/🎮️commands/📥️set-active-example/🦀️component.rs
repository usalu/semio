//! 📥️ 📥️ Forms play app commands command — `set-active-example`.

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
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    let next = match payload.example_id.as_str() {
        "" => Some(empty_forms_snapshot()),
        "building-component" => forms_dsl::parse_playbook_example_dsl(forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).ok(),
        "default" => Some(default_example_spec()),
        "onboarding" => Some(onboarding_example_spec()),
        _ => None,
    };
    let Some(next) = next else {
        return Ok(Emit::default());
    };
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer clears a config-owned
    // selection here — swapping in a whole new document prunes every stale "fields" selection id
    // automatically via `revalidate_interaction_state_after_document_change`.
    Ok(Emit { artifact_mutations: replace_spec_operations(doc.snapshot, &next), config_mutations: reset_try_config_mutations(), ..Default::default() })
}
