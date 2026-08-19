//! 🌱️ Fem2d mutation — `CreateMaterial` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemMaterial};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemMaterial`] into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-material")]
pub struct CreateMaterial {
    pub material: FemMaterial,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateMaterial {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };

    async fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create material \"{}\"", self.material.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.material.id.clone()]
    }
}
//#endregion 🔖️Mutation
