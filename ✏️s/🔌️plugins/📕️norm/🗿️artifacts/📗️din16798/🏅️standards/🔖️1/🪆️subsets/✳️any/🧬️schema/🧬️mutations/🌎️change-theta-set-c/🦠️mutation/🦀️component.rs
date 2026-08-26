//! 🔧 `change-theta-set-c` payload — changes the Din16798 document's `theta_set_c` (cooling set-point temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeThetaSetC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeThetaSetC {
    pub new_theta_set_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeThetaSetC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-set-c", kind: "change-theta-set-c", record: "ChangedThetaSetC" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_theta_set_c::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_theta_set_c::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cooling set-point temperature to {}", self.new_theta_set_c)
    }
}
//#endregion 🔖️ChangeThetaSetC
