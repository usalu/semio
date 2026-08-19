//! ➖️ Fem3d mutation — `RemoveLoad` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➖️ Detaches a load from an existing load case's `loads` member collection by id, capturing
/// nothing itself (the removed payload is recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-load")]
pub struct RemoveLoad {
    pub case_id: String,
    pub load_id: String,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for RemoveLoad {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "load", kind: "remove-load", record: "RemovedLoad" };

    async fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem3d::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Remove load \"{}\" from case \"{}\"", self.load_id, self.case_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
