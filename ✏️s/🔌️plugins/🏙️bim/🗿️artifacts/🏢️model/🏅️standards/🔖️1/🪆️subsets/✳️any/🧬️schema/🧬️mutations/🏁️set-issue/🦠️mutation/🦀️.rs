//! 🏁️ `set-issue` payload. Sparsely changes an issue: title, description, status, priority, assignee, author, creation moment, labels, elements, the clash it was raised from (an assigned null clears it) and the viewpoint (an assigned null clears it).

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use crate::{Assigned, ClashRef, IssuePatch, IssuePriority, IssueStatus, IssueViewpoint};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetIssue {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IssueStatus>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<IssuePriority>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub clash: Option<Assigned<Option<ClashRef>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub viewpoint: Option<Assigned<Option<IssueViewpoint>>>,
}

impl SetIssue {
    /// 🩹 The sparse entity patch this payload names: every provided field, restated values included.
    pub fn patch(&self) -> IssuePatch {
        IssuePatch { title: self.title.clone(), description: self.description.clone(), status: self.status, priority: self.priority, assignee: self.assignee.clone(), author: self.author.clone(), created: self.created.clone(), labels: self.labels.clone(), elements: self.elements.clone(), clash: self.clash.clone(), viewpoint: self.viewpoint.clone() }
    }

    /// 🧩 The payload that provides exactly the fields `patch` names.
    pub fn from_patch(id: String, patch: IssuePatch) -> Self {
        Self { id, title: patch.title, description: patch.description, status: patch.status, priority: patch.priority, assignee: patch.assignee, author: patch.author, created: patch.created, labels: patch.labels, elements: patch.elements, clash: patch.clash, viewpoint: patch.viewpoint }
    }
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetIssue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "issue", kind: "set-issue", record: "SetIssue" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change issue \"{}\"", self.id), &format!("Hinweis \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
