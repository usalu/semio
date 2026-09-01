//! 🗑️ Assembly mutation — `RemoveWeight`: drops a module's weight override entirely, restoring the
//! `wfc_engine` `WeightTable` neutral default (`1.0`) for that module id. Kept as its own kind
//! (verb `remove`, distinct from `change-weight`'s `change`) so `ChangeWeight`'s own inverse over an
//! absent prior entry has a genuine removal to delegate to — a true undo, not a lossy approximation.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️RemoveWeight
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveWeight {
    pub module_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_weight(module_id: String) -> AssemblyMutation {
    AssemblyMutation::RemoveWeight(RemoveWeight { module_id })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for RemoveWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "weight", kind: "remove-weight", record: "RemovedWeight" };

    fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove weight override for module \"{}\"", self.module_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.module_id.clone()]
    }
}
//#endregion 🔖️RemoveWeight
