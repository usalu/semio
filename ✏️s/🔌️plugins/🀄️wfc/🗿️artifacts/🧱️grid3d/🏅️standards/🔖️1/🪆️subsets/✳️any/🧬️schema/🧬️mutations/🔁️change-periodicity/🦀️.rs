//! 🔁 `s.wfc.grid3d` mutation — `ChangePeriodicity`: sets all three wrap flags at once. A periodic
//! axis becomes `Boundary::Wrap` in the solve topology; a non-periodic one stays `Boundary::Open`.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
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
    pub periodic_z: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_periodicity<Op>(periodic_x: bool, periodic_y: bool, periodic_z: bool) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_periodicity::ChangePeriodicity>,
{
    Op::from(ChangePeriodicity { periodic_x, periodic_y, periodic_z })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for ChangePeriodicity
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<ChangePeriodicity>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "periodicity", kind: "change-periodicity", record: "ChangedPeriodicity" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change periodicity to {}/{}/{}", self.periodic_x, self.periodic_y, self.periodic_z), &format!("Periodizität auf {}/{}/{} ändern", self.periodic_x, self.periodic_y, self.periodic_z))
    }
}
//#endregion 🔖️ChangePeriodicity
