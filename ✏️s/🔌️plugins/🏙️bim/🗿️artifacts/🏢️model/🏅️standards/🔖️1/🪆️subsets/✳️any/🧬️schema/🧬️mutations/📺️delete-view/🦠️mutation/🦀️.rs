//! 📺️ `delete-view` payload. Removes a view together with its properties and classifications. Views are leaves of the model: nothing else depends on them yet.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteView {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteView {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "view", kind: "delete-view", record: "DeleteView" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete view \"{}\"", self.id), &format!("Ansicht \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
