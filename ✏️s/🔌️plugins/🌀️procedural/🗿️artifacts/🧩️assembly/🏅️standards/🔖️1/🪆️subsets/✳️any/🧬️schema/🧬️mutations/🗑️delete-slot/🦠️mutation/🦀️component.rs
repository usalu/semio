//! 🗑️ Assembly mutation — `DeleteSlot`: removes an id-addressed WFC slot and cascades to any
//! incident edges (a slot cannot be referenced by a dangling edge).

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteSlot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSlot {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_slot(id: String) -> AssemblyMutation {
    AssemblyMutation::DeleteSlot(DeleteSlot { id })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for DeleteSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "slot", kind: "delete-slot", record: "DeletedSlot" };

    fn diff(&self, base: &AssemblySnapshot) -> AssemblyDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete slot \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteSlot
