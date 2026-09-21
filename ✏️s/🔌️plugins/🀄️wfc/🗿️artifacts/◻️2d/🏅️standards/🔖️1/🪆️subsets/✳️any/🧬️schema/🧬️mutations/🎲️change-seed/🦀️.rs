//! 🎲 WFC 2D mutation — `ChangeSeed`: sets the deterministic solve seed. PERSISTED snapshot field,
//! authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays sound.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
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
pub fn change_seed(seed: u64) -> Wfc2dMutation {
    Wfc2dMutation::ChangeSeed(ChangeSeed { seed })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for ChangeSeed {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change Seed", "Startwert ändern")
    }
}
//#endregion 🔖️ChangeSeed
