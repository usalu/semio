//! 🎲 Sets the deterministic WFC solve seed. PERSISTED snapshot field, authored ONLY here — never ambient — so the solve inference's `DepHash` caching stays sound.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
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

impl<Op> MutationKind<Grid2dSnapshot, Op> for ChangeSeed
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<ChangeSeed>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change seed to {}", self.seed), &format!("Startwert auf {} ändern", self.seed))
    }
}
//#endregion 🔖️ChangeSeed
