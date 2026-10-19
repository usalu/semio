//! 🚩️ `create-issue` payload. Raises an issue (a BCF topic): title, description, status, priority, assignee, author, the moment it was raised, labels, the elements it concerns, the clash it was raised from and the viewpoint (orbit camera, section box, isolated elements) that shows it.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Issue, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateIssue {
    pub id: String,
    pub issue: Issue,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateIssue
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateIssue> + From<semio_s_artifact_bim_model_mut_any_120::standards::v1::subsets::any::schema::mutations::delete_issue::DeleteIssue>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "issue", kind: "create-issue", record: "CreateIssue" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_issue::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_issue::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Raise issue \"{}\"", self.issue.title), &format!("Hinweis \"{}\" erfassen", self.issue.title))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
