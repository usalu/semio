//! 🔢 Assembly mutation — `ChangeWeight`: sets a module's selection-bias weight (`wfc_engine`'s
//! `WeightTable` input), upserting the id-keyed `weights` entry.

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeWeight
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeWeight {
    pub module_id: String,
    pub weight: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_weight(module_id: String, weight: f64) -> AssemblyMutation {
    AssemblyMutation::ChangeWeight(ChangeWeight { module_id, weight })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for ChangeWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "weight", kind: "change-weight", record: "ChangedWeight" };

    async fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change weight of module \"{}\" to {}", self.module_id, self.weight)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.module_id.clone()]
    }
}
//#endregion 🔖️ChangeWeight
