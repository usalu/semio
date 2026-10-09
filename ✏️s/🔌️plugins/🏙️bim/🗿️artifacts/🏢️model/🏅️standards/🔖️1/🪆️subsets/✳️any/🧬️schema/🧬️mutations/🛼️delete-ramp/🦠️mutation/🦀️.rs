//! 📉️ `delete-ramp` payload. Removes a ramp together with the railings hosted by it and its properties and classifications.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteRamp {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteRamp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "ramp", kind: "delete-ramp", record: "DeleteRamp" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete ramp \"{}\"", self.id), &format!("Rampe \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
