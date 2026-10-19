//! 💬️ `create-issue-comment` payload. Adds a comment to an issue: who wrote it, when and what.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{IssueComment, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateIssueComment {
    pub id: String,
    pub issue_comment: IssueComment,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateIssueComment
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateIssueComment> + From<semio_s_artifact_bim_model_mut_any_121::standards::v1::subsets::any::schema::mutations::delete_issue_comment::DeleteIssueComment>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "issue-comment", kind: "create-issue-comment", record: "CreateIssueComment" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_issue_comment::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_issue_comment::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Comment on issue \"{}\"", self.issue_comment.issue), &format!("Hinweis \"{}\" kommentieren", self.issue_comment.issue))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
