//! 🎲 WFC 2D mutation — `ChangeSeed`: sets the deterministic solve seed. PERSISTED snapshot field,
//! authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays sound.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeSeed
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed<Op>(seed: u64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed>,
{
    Op::from(ChangeSeed { seed })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for ChangeSeed
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<ChangeSeed>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Seed", "Startwert ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for ChangeSeed
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<ChangeSeed>,
{
    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeSeed
