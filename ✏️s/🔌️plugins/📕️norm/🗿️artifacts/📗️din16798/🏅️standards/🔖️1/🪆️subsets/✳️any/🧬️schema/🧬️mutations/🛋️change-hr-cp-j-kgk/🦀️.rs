//! 🔧 `change-hr-cp-j-kgk` payload — changes the Din16798 document's `hr_cp_j_kgk` (heat recovery specific heat capacity).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_hr_cp_j_kgk::ChangeHrCpJKgk;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHrCpJKgk
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHrCpJKgk {
    pub new_hr_cp_j_kgk: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHrCpJKgk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hr-cp-j-kgk", kind: "change-hr-cp-j-kgk", record: "ChangedHrCpJKgk" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change heat recovery specific heat capacity to {}", self.new_hr_cp_j_kgk)
    }
}
//#endregion 🔖️ChangeHrCpJKgk
