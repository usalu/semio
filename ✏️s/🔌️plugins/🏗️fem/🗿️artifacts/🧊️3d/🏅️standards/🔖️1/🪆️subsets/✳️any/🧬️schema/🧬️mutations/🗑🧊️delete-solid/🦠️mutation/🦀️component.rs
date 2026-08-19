//! 🗑️ Fem3d mutation — `DeleteSolid` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ Removes an existing meshed solid by id, capturing nothing itself (the removed payload is
/// recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-solid")]
pub struct DeleteSolid {
    pub id: String,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for DeleteSolid {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "solid", kind: "delete-solid", record: "DeletedSolid" };

    async fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete solid \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
