//! 🔧 `change-theta-rm-c` payload — changes the Din16798 document's `theta_rm_c` (running mean outdoor temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeThetaRmC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeThetaRmC {
    pub new_theta_rm_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeThetaRmC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-rm-c", kind: "change-theta-rm-c", record: "ChangedThetaRmC" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_theta_rm_c::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_theta_rm_c::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change running mean outdoor temperature to {}", self.new_theta_rm_c)
    }
}
//#endregion 🔖️ChangeThetaRmC
