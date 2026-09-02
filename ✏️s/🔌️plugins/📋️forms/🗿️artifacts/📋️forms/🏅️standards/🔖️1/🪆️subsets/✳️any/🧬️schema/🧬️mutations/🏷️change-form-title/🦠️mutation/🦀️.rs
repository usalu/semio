//! 🖋️ Forms mutation payload — `change-form-title`, the document-level `title` scalar's `change`
//! verb (a `None` clears it — `title` is optional, so `rename`'s always-a-new-name shape doesn't
//! fit). Physical dir name (`📖update-playbook`, wired by `🦀️.rs`) predates the semantic
//! rename; the Rust module is still `update_playbook`, the type/variant/kind are `change-form-title`.

use serde::{Deserialize, Serialize};
use crate::artifacts::forms::{FormMutation, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🖋️ChangeFormTitle
/// 🖋️ Sets the document's `title` scalar.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, Serialize, Deserialize)]
pub struct ChangeFormTitle {
    pub new_title: Option<String>,
}

impl MutationKind<FormsSnapshot, FormMutation> for ChangeFormTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "form-title", kind: "change-form-title", record: "ChangedFormTitle" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_change_form_title(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_change_form_title(self, base)
    }
    async fn label(&self) -> String {
        match &self.new_title {
            Some(title) => format!("Change form title to \"{title}\""),
            None => "Clear form title".to_string(),
        }
    }
}
//#endregion 🖋️ChangeFormTitle
