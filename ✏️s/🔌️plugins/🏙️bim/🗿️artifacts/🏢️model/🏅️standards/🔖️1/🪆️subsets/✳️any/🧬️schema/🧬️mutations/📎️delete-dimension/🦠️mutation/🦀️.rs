//! 📎️ `delete-dimension` payload. Removes a dimension; the elements it measured stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteDimension {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteDimension {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "dimension", kind: "delete-dimension", record: "DeleteDimension" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete dimension \"{}\"", self.id), &format!("Bemaßung \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
