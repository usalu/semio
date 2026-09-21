//! 🔁 Sets whether each axis wraps. A periodic axis becomes `Boundary::Wrap` in the solve topology; a non-periodic one becomes `Boundary::Open`.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangePeriodicity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePeriodicity {
    pub periodic_x: bool,
    pub periodic_y: bool,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_periodicity(periodic_x: bool, periodic_y: bool) -> Grid2dMutation {
    Grid2dMutation::ChangePeriodicity(ChangePeriodicity { periodic_x, periodic_y })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for ChangePeriodicity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "periodicity", kind: "change-periodicity", record: "ChangedPeriodicity" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change periodicity to x={} y={}", self.periodic_x, self.periodic_y), &format!("Periodizität auf x={} y={} ändern", self.periodic_x, self.periodic_y))
    }
}
//#endregion 🔖️ChangePeriodicity
