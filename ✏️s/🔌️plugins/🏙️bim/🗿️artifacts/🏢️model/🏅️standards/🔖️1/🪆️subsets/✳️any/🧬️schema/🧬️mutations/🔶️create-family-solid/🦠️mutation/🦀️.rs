//! 🔶️ `create-family-solid` payload. Adds a solid to a family: an extrusion, revolution, sweep or cuboid whose dimensions, material and visibility are formulas of the family parameters.

use crate::{FamilySolid, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateFamilySolid {
    pub id: String,
    pub solid: FamilySolid,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateFamilySolid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "family-solid", kind: "create-family-solid", record: "CreateFamilySolid" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create family solid \"{}\"", self.solid.name), &format!("Familienkörper \"{}\" anlegen", self.solid.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
