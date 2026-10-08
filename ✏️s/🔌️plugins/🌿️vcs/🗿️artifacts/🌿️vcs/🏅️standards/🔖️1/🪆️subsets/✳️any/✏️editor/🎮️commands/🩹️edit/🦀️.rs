//! 🩹️ 🩹️ VCS play app commands command — `edit`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️EditRules
/// 📐️ One field's edit rule: reads the committed and the edited snapshot and names the concrete kinds that field's edit is.
type VcsEditRule = fn(&VcsSnapshot, &VcsSnapshot) -> Vec<VcsDemoMutation>;

/// 📐️ The per-field edit-rules table (norm's `EDIT_RULES` shape): one rule per snapshot field, in emission order; tag
/// removals run before position-exact tag insertions so every insertion index is an index of the edited list.
const VCS_EDIT_RULES: [VcsEditRule; 5] = [
    |current, next| (next.title != current.title).then(|| crate::mutations::rename_vcs(next.title.clone())).into_iter().collect(),
    |current, next| (next.counter != current.counter).then(|| crate::mutations::change_counter(next.counter)).into_iter().collect(),
    |current, next| (next.status != current.status).then(|| crate::mutations::change_status(next.status.clone())).into_iter().collect(),
    |current, next| (next.notes != current.notes).then(|| crate::mutations::change_notes(next.notes.clone())).into_iter().collect(),
    |current, next| {
        let removals = current.tags.iter().filter(|tag| !next.tags.contains(tag)).map(|tag| crate::mutations::remove_tag(tag.clone()));
        let additions = next.tags.iter().enumerate().filter(|(_, tag)| !current.tags.contains(tag)).map(|(index, tag)| crate::mutations::add_tag_at(tag.clone(), index as u32));
        removals.chain(additions).collect()
    },
];
//#endregion 🔖️EditRules

//#region 🔖️TextEdit
//#endregion 🔖️TextEdit

//#region 🔖️Edit
//#endregion 🔖️Edit

/// 🧩️ The former `TextEdit`/`Edit` match arm body, shared by both payload modules: parses the given text as a whole
/// `VcsSnapshot` and emits the concrete kinds of every edited field. A live typing delivery (`typing` argument) folds into its
/// window's typing run — a single buffer, so the run's net is the diff of its last text — which commits as ONE edit (design
/// §13.2); a one-shot dispatch is one edit.
pub(crate) fn text_edit_operations(text: &str, current: &VcsSnapshot) -> Emit<VcsDemoMutation, VcsDemoConfigMutation> {
    match semio_framework_pack_json::from_json_str::<VcsSnapshot>(text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(next_projection) => Emit::mutations(VCS_EDIT_RULES.iter().flat_map(|rule| rule(current, &next_projection)).collect()),
        Err(_) => Emit::default(),
    }
}

/// 🩹️ Alias for [`text_edit::TextEdit`] — same payload shape, same handler body.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit")]
pub struct Edit {
    pub text: String,
}

pub fn handle(payload: &Edit, doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(text_edit_operations(&payload.text, doc.snapshot))
}
