//! 🔧 `change-theta-st-c` payload — changes the Din16798 document's `theta_st_c` (storage temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeThetaStC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeThetaStC {
    pub new_theta_st_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeThetaStC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "theta-st-c", kind: "change-theta-st-c", record: "ChangedThetaStC" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_theta_st_c::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_theta_st_c::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change storage temperature to {}", self.new_theta_st_c)
    }
}
//#endregion 🔖️ChangeThetaStC
