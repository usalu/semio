//! 🌱️ Fem3d mutation — `CreateSolid` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemSolid};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSolid`] meshed solid into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-solid")]
pub struct CreateSolid {
    pub solid: FemSolid,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateSolid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "solid", kind: "create-solid", record: "CreatedSolid" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create solid \"{}\"", self.solid.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.solid.id.clone()]
    }
}
//#endregion 🔖️Mutation
