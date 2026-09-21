//! 🎲 Bitmap mutation — `ChangeSeed`: sets the deterministic WFC solve seed. A PERSISTED snapshot
//! field, authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays
//! sound (wave function collapse is seeded-random internally).

use crate::diff::BitmapDiff;
use crate::mutations::BitmapMutation;
use crate::schema::snapshot::BitmapSnapshot;
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
pub fn change_seed(seed: u64) -> BitmapMutation {
    BitmapMutation::ChangeSeed(ChangeSeed { seed })
}

impl MutationKind<BitmapSnapshot, BitmapMutation> for ChangeSeed {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change seed to {}", self.seed), &format!("Startwert auf {} ändern", self.seed))
    }
}
//#endregion 🔖️ChangeSeed
