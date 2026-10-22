//! 🔁 Sets whether each axis wraps. A periodic axis becomes `Boundary::Wrap` in the solve topology; a non-periodic one becomes `Boundary::Open`.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangePeriodicity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePeriodicity {
    pub periodic_x: bool,
    pub periodic_y: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_periodicity<Op>(periodic_x: bool, periodic_y: bool) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_periodicity::ChangePeriodicity>,
{
    Op::from(ChangePeriodicity { periodic_x, periodic_y })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for ChangePeriodicity
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<ChangePeriodicity>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "periodicity", kind: "change-periodicity", record: "ChangedPeriodicity" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change periodicity to x={} y={}", self.periodic_x, self.periodic_y), &format!("Periodizität auf x={} y={} ändern", self.periodic_x, self.periodic_y))
    }
}

impl<Op> protocol::ColdMutationKind<Grid2dSnapshot, Op> for ChangePeriodicity
where
    Op: protocol::ColdMutation<Grid2dSnapshot, ReferenceDiff = Grid2dDiff> + From<ChangePeriodicity>,
{
    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangePeriodicity
