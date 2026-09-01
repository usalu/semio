//! 🔧 `change-theta-amb-c` payload — changes the Din16798 document's `theta_amb_c` (ambient temperature).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_theta_amb_c::ChangeThetaAmbC;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeThetaAmbC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeThetaAmbC {
    pub new_theta_amb_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeThetaAmbC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-amb-c", kind: "change-theta-amb-c", record: "ChangedThetaAmbC" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ambient temperature to {}", self.new_theta_amb_c)
    }
}
//#endregion 🔖️ChangeThetaAmbC
