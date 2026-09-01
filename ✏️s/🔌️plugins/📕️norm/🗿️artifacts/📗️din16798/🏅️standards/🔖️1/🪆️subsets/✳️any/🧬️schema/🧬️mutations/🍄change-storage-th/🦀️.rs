//! 🔧 `change-storage-th` payload — changes the Din16798 document's `storage_t_h` (storage duration).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_storage_t_h::ChangeStorageTH;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeStorageTH
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeStorageTH {
    pub new_storage_t_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeStorageTH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "storage-th", kind: "change-storage-th", record: "ChangedStorageTH" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change storage duration to {}", self.new_storage_t_h)
    }
}
//#endregion 🔖️ChangeStorageTH
