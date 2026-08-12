//! 🔧 `change-theta-ec` payload — changes the Din16798 document's `theta_e_c` (outdoor design temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeThetaEC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeThetaEC {
    pub new_theta_e_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeThetaEC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-ec", kind: "change-theta-ec", record: "ChangedThetaEC" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_theta_e_c::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_theta_e_c::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change outdoor design temperature to {}", self.new_theta_e_c)
    }
}
//#endregion 🔖️ChangeThetaEC
