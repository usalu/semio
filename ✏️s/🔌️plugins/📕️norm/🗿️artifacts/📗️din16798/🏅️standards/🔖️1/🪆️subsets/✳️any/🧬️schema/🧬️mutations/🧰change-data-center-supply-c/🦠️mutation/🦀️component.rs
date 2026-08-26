//! 🔧 `change-data-center-supply-c` payload — changes the Din16798 document's `data_center_supply_c` (data center supply temperature).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDataCenterSupplyC
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDataCenterSupplyC {
    pub new_data_center_supply_c: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDataCenterSupplyC {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "data-center-supply-c", kind: "change-data-center-supply-c", record: "ChangedDataCenterSupplyC" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_data_center_supply_c::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_data_center_supply_c::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change data center supply temperature to {}", self.new_data_center_supply_c)
    }
}
//#endregion 🔖️ChangeDataCenterSupplyC
