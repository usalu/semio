//! 🔳️ `delete-grid-line` payload. Removes a grid line together with its properties and classifications.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteGridLine {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteGridLine {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "grid-line", kind: "delete-grid-line", record: "DeleteGridLine" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete grid line \"{}\"", self.id), &format!("Rasterlinie \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
