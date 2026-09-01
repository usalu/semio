//! 🔧 `change-theta-rm-c` payload — changes the Din16798 document's `theta_rm_c` (running mean outdoor temperature).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_theta_rm_c::ChangeThetaRmC;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeThetaRmC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeThetaRmC {
    pub new_theta_rm_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeThetaRmC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-rm-c", kind: "change-theta-rm-c", record: "ChangedThetaRmC" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change running mean outdoor temperature to {}", self.new_theta_rm_c)
    }
}
//#endregion 🔖️ChangeThetaRmC
