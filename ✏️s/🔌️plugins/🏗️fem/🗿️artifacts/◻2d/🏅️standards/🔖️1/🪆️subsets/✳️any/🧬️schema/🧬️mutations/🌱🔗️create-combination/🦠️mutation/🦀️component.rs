//! 🌱️ Fem2d mutation — `CreateCombination` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::{Fem2dSnapshot, FemCombination};
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

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateCombination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "combination", kind: "create-combination", record: "CreatedCombination" };

    fn diff(&self, base: &Fem2dSnapshot) -> crate::artifacts::fem2d::diff::Fem2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create combination \"{}\"", self.combination.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.combination.id.clone()]
    }
}
//#endregion 🔖️Mutation
