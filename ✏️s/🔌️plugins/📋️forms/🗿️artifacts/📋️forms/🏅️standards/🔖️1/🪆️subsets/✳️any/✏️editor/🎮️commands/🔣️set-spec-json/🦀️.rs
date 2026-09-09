//! 📥️ 📥️ Forms play app commands command — `set-spec-json`.

use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::reset_try_config_mutations;
use crate::{forms_steps, op::FormMutation, FormsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

// 🧷️ Aliased: the payload structs below derive the EXTERN `dsl` crate's `dsl::DslRecord` — importing the
// artifact's own `dsl` submodule under the bare name would shadow it.

//#region 🔖️Shell
/// ✏️ Emits the operations that replace the current form spec's title + steps with those of `next` — a
/// legitimate whole-document swap for import/example-switch, expressed granularly through the existing
/// `FormMutation` vocabulary (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: `CreateStep`/
/// `DeleteStep`/`ChangeFormTitle`, reading through `forms_steps` now that `FormsSnapshot` no longer
/// carries a bare `steps` field) so it still records a true inverse.
fn replace_spec_operations(current: &FormsSnapshot, next: &FormsSnapshot) -> Vec<FormMutation> {
    use crate::mutations::{change_form_title, create_step, delete_step};
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "spec-json")]
pub struct SetSpecJson {
    pub json: String,
}

pub fn handle(payload: &SetSpecJson, doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> Result<Emit<FormMutation, FormsConfigMutation>, Fault> {
    // 🩹️ `FormsSnapshot` composes `structure`/`results` handles (ticket
    // 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM) so it no longer deserializes raw step/block
    // JSON directly; `semio_framework_artifact_playbook_playbook::PlaybookSpec` is the SAME `{schema,id,version,title,steps}`
    // camelCase shape `FormsSnapshot` used before composition, so it stays the deserialize target.
    let Ok(spec) = dsl::os_pack::json::from_json_str::<semio_framework_artifact_playbook_playbook::PlaybookSpec>(&payload.json) else {
        return Ok(Emit::default());
    };
    let next = crate::forms_snapshot_with_state(spec.schema, spec.id, spec.version, spec.title, &spec.steps);
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: no longer clears a config-owned
    // selection here — swapping in a whole new document prunes every stale "fields" selection id
    // automatically via `revalidate_interaction_state_after_document_change`.
    Ok(Emit { artifact_mutations: replace_spec_operations(doc.snapshot, &next), config_mutations: reset_try_config_mutations(), ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
