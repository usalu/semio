//! 🎲 Assembly mutation — `ChangeSeed`: sets the deterministic WFC solve seed. PERSISTED snapshot
//! field, authored ONLY here — never ambient/`Math.random`-style — so `InferredField::compute`'s
//! `DepHash` caching stays sound (WFC is seeded-random internally).

use crate::artifacts::assembly::diff::AssemblyDiff;
use crate::artifacts::assembly::mutations::AssemblyMutation;
use crate::artifacts::assembly::schema::snapshot::AssemblySnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSeed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed(seed: u64) -> AssemblyMutation {
    AssemblyMutation::ChangeSeed(ChangeSeed { seed })
}

impl MutationKind<AssemblySnapshot, AssemblyMutation> for ChangeSeed {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &AssemblySnapshot) -> protocol::MutationOutcome<AssemblyDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &AssemblySnapshot) -> Vec<AssemblyMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change seed to {}", self.seed)
    }
}
//#endregion 🔖️ChangeSeed
