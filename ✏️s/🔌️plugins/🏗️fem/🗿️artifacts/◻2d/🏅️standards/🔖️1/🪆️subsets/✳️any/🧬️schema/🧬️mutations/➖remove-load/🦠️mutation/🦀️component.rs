//! ➖️ Fem2d mutation — `RemoveLoad` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;
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

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for RemoveLoad {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "load", kind: "remove-load", record: "RemovedLoad" };

    fn diff(&self, base: &Fem2dSnapshot) -> crate::artifacts::fem2d::diff::Fem2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove load \"{}\" from case \"{}\"", self.load_id, self.case_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }
}
//#endregion 🔖️Mutation
