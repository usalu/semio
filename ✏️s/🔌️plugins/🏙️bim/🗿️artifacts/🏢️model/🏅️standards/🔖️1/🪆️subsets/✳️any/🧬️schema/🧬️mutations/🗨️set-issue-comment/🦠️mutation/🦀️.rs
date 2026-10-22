//! 🗨️ `set-issue-comment` payload. Sparsely changes a comment: its author, its moment and its text. The issue of a comment never changes.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::patches::coordination::IssueCommentPatch;

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

impl<Op> MutationKind<ModelSnapshot, Op> for SetIssueComment
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetIssueComment>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "issue-comment", kind: "set-issue-comment", record: "SetIssueComment" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change comment \"{}\"", self.id), &format!("Kommentar \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetIssueComment
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetIssueComment>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_issue_comment::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_issue_comment::inverse::inverse(self, base))
    }
}
