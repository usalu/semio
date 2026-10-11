//! 🎲 `wfc3d` mutation — `ChangeSeed`: sets the deterministic solve seed. PERSISTED snapshot field,
//! authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays sound.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeSeed
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed(seed: u64) -> Wfc3dMutation {
    Wfc3dMutation::ChangeSeed(ChangeSeed { seed })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for ChangeSeed {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Wfc3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change seed to {}", self.seed), &format!("Startwert auf {} ändern", self.seed))
    }
}
//#endregion 🔖️ChangeSeed
