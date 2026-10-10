//! 📢️ `delete-issue` payload. Removes an issue together with its comments, its properties and its classifications. The elements and the clash it named are not touched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteIssue {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteIssue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "issue", kind: "delete-issue", record: "DeleteIssue" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete issue \"{}\"", self.id), &format!("Hinweis \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
