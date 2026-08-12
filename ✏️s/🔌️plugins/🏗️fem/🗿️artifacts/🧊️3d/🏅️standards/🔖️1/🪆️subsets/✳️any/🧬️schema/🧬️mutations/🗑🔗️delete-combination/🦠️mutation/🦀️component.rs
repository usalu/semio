//! 🗑️ Fem3d mutation — `DeleteCombination` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ Removes an existing combination by id, capturing nothing itself (the removed payload is
/// recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-combination")]
pub struct DeleteCombination {
    pub id: String,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for DeleteCombination {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "combination", kind: "delete-combination", record: "DeletedCombination" };

    fn diff(&self, base: &Fem3dSnapshot) -> crate::artifacts::fem3d::diff::Fem3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete combination \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
