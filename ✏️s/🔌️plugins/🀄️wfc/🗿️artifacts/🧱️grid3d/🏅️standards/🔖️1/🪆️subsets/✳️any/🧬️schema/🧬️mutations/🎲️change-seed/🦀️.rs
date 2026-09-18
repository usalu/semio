//! 🎲 `s.wfc.grid3d` mutation — `ChangeSeed`: sets the deterministic WFC solve seed. PERSISTED
//! snapshot field, authored ONLY here — never ambient — so the solve inference's `DepHash` caching
//! stays sound (WFC is seeded-random internally).

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeSeed
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed(seed: u64) -> Grid3dMutation {
    Grid3dMutation::ChangeSeed(ChangeSeed { seed })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for ChangeSeed {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change seed to {}", self.seed)
    }
}
//#endregion 🔖️ChangeSeed
