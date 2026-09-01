//! 🔧 `change-cellar-area-m2` payload — changes the Din16798 document's `cellar_area_m2` (cellar area).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_cellar_area_m2::ChangeCellarAreaM2;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCellarAreaM2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCellarAreaM2 {
    pub new_cellar_area_m2: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCellarAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cellar-area-m2", kind: "change-cellar-area-m2", record: "ChangedCellarAreaM2" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cellar area to {}", self.new_cellar_area_m2)
    }
}
//#endregion 🔖️ChangeCellarAreaM2
