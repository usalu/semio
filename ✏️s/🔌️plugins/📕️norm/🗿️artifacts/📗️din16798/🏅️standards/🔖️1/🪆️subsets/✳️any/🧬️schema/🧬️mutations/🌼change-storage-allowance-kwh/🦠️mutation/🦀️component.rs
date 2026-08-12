//! 🔧 `change-storage-allowance-kwh` payload — changes the Din16798 document's `storage_allowance_kwh` (storage loss allowance).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStorageAllowanceKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStorageAllowanceKwh {
    pub new_storage_allowance_kwh: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeStorageAllowanceKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "storage-allowance-kwh", kind: "change-storage-allowance-kwh", record: "ChangedStorageAllowanceKwh" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_storage_allowance_kwh::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_storage_allowance_kwh::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change storage loss allowance to {}", self.new_storage_allowance_kwh)
    }
}
//#endregion 🔖️ChangeStorageAllowanceKwh
