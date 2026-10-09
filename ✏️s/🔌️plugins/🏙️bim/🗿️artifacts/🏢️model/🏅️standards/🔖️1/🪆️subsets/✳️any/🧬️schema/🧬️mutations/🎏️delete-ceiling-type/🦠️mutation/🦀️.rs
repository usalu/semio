//! 🎏️ `delete-ceiling-type` payload. Removes a ceiling type that no ceiling uses.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteCeilingType {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteCeilingType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "ceiling-type", kind: "delete-ceiling-type", record: "DeleteCeilingType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete ceiling type \"{}\"", self.id), &format!("Unterdeckentyp \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
