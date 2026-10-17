//! 📌 `s.wfc.grid3d` mutation — `PinCell`: fixes one cell to one tile before the solve runs. A pin on
//! a masked cell is refused (the cell is not in the topology at all), and a pin that replaces an
//! existing one inverts back to the tile that was there.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinCell {
    pub pinned: Grid3dPinnedCell,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_cell<Op>(pinned: Grid3dPinnedCell) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    Op::from(PinCell { pinned })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for PinCell
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<PinCell> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_cell::UnpinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "cell", kind: "pin-cell", record: "Fixed" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Pin cell {} to tile \"{}\"", cell_key(self.pinned.x, self.pinned.y, self.pinned.z), self.pinned.tile_id), &format!("Zelle {} auf Kachel \"{}\" fixieren", cell_key(self.pinned.x, self.pinned.y, self.pinned.z), self.pinned.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.pinned.x, self.pinned.y, self.pinned.z)]
    }
}
//#endregion 🔖️PinCell
