//! 🔁 `s.wfc.grid3d` mutation — `ChangePeriodicity`: sets all three wrap flags at once. A periodic
//! axis becomes `Boundary::Wrap` in the solve topology; a non-periodic one stays `Boundary::Open`.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangePeriodicity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePeriodicity {
    pub periodic_x: bool,
    pub periodic_y: bool,
    pub periodic_z: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_periodicity(periodic_x: bool, periodic_y: bool, periodic_z: bool) -> Grid3dMutation {
    Grid3dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y, periodic_z })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for ChangePeriodicity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "periodicity", kind: "change-periodicity", record: "ChangedPeriodicity" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change periodicity to {}/{}/{}", self.periodic_x, self.periodic_y, self.periodic_z)
    }
}
//#endregion 🔖️ChangePeriodicity
