//! 🌱 Assembly mutation — `CreateSlot`: brings a new id-keyed WFC slot into existence at a
//! FINAL-state insertion index.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::{AssemblySlot, AssemblySnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSlot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSlot {
    pub index: usize,
    pub slot: AssemblySlot,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_slot(index: usize, slot: AssemblySlot) -> AssemblyMutation {
    AssemblyMutation::CreateSlot(CreateSlot { index, slot })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for CreateSlot {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "slot", kind: "create-slot", record: "CreatedSlot" };

    fn diff(&self, base: &AssemblySnapshot) -> AssemblyDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create slot \"{}\"", self.slot.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.slot.id.clone()]
    }
}
//#endregion 🔖️CreateSlot
