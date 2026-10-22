//! 📌 Pre-assigns one cell to a tile — a hard fix the solver seeds its domains with, not a hint.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinCell {
    pub x: u32,
    pub y: u32,
    pub tile_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_cell<Op>(x: u32, y: u32, tile_id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    Op::from(PinCell { x, y, tile_id })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for PinCell
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "cell", kind: "pin-cell", record: "FixedCell" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Pin cell ({}, {}) to \"{}\"", self.x, self.y, self.tile_id), &format!("Zelle ({}, {}) auf \"{}\" fixieren", self.x, self.y, self.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![format!("{},{}", self.x, self.y)]
    }
}

impl<Op> protocol::ColdMutationKind<Grid2dSnapshot, Op> for PinCell
where
    Op: protocol::ColdMutation<Grid2dSnapshot, ReferenceDiff = Grid2dDiff> + From<PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell>,
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
//#endregion 🔖️PinCell
