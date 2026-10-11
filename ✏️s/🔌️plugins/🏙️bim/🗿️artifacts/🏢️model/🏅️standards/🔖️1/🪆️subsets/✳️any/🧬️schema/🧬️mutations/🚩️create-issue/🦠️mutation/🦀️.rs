//! 🚩️ `create-issue` payload. Raises an issue (a BCF topic): title, description, status, priority, assignee, author, the moment it was raised, labels, the elements it concerns, the clash it was raised from and the viewpoint (orbit camera, section box, isolated elements) that shows it.

use crate::{Issue, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateIssue {
    pub id: String,
    pub issue: Issue,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateIssue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "issue", kind: "create-issue", record: "CreateIssue" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Raise issue \"{}\"", self.issue.title), &format!("Hinweis \"{}\" erfassen", self.issue.title))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
