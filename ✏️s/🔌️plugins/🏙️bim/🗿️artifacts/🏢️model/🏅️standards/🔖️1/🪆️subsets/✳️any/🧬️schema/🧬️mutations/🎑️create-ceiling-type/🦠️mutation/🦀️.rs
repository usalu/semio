//! 🎑️ `create-ceiling-type` payload. Brings a new layered ceiling type into the library; every layer names an existing material and has a positive thickness.

use crate::{CeilingType, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCeilingType {
    pub id: String,
    pub ceiling_type: CeilingType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateCeilingType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "ceiling-type", kind: "create-ceiling-type", record: "CreateCeilingType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create ceiling type \"{}\"", self.ceiling_type.name), &format!("Unterdeckentyp \"{}\" anlegen", self.ceiling_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
