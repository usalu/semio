//! 🧗️ `create-stair` payload. Brings a stair run onto a storey; risers and treads are never stored, they are inferred from the storey levels and the top constraint.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Stair};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateStair {
    pub id: String,
    pub stair: Stair,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateStair {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "stair", kind: "create-stair", record: "CreateStair" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create stair \"{}\"", self.stair.name), &format!("Treppe \"{}\" anlegen", self.stair.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
