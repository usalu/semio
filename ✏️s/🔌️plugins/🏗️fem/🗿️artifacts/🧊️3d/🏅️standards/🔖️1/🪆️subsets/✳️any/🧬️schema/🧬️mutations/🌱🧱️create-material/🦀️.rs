//! 🌱️ Fem3d mutation — `CreateMaterial` payload + `MutationKind` impl.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemMaterial};
use crate::artifacts::fem3d::diff::{Fem3dDiff, Fem3dMaterialsDelta};
use crate::artifacts::fem3d::mutations::{Fem3dMutation, delete_material};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemMaterial`] material into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-material")]
pub struct CreateMaterial {
    pub material: FemMaterial,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create material \"{}\"", self.material.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.material.id.clone()]
    }
}
//#endregion 🔖️Mutation
