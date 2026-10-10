//! 💬️ `create-issue-comment` payload. Adds a comment to an issue: who wrote it, when and what.

use crate::{IssueComment, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateIssueComment {
    pub id: String,
    pub issue_comment: IssueComment,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateIssueComment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "issue-comment", kind: "create-issue-comment", record: "CreateIssueComment" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Comment on issue \"{}\"", self.issue_comment.issue), &format!("Hinweis \"{}\" kommentieren", self.issue_comment.issue))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
