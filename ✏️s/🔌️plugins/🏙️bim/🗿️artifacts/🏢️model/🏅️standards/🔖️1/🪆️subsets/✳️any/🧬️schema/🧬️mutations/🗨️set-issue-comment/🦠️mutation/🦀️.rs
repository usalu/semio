//! 🗨️ `set-issue-comment` payload. Sparsely changes a comment: its author, its moment and its text. The issue of a comment never changes.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::IssueCommentPatch;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetIssueComment {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SetIssueComment {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> IssueCommentPatch {
        IssueCommentPatch { author: self.author.clone(), date: self.date.clone(), text: self.text.clone(), ..Default::default() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: IssueCommentPatch) -> Self {
        Self { id, author: patch.author, date: patch.date, text: patch.text }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetIssueComment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "issue-comment", kind: "set-issue-comment", record: "SetIssueComment" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change comment \"{}\"", self.id), &format!("Kommentar \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
