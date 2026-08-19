//! 🌱️ Fem3d mutation — `CreateCombination` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemCombination};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemCombination`] into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-combination")]
pub struct CreateCombination {
    pub combination: FemCombination,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateCombination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "combination", kind: "create-combination", record: "CreatedCombination" };

    async fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create combination \"{}\"", self.combination.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.combination.id.clone()]
    }
}
//#endregion 🔖️Mutation
