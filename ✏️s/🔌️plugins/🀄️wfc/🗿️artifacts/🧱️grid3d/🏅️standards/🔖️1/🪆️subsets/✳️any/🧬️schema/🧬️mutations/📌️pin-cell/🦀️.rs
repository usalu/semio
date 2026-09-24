//! 📌 `s.wfc.grid3d` mutation — `PinCell`: fixes one cell to one tile before the solve runs. A pin on
//! a masked cell is refused (the cell is not in the topology at all), and a pin that replaces an
//! existing one inverts back to the tile that was there.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️PinCell
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct PinCell {
    pub pinned: Grid3dPinnedCell,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn pin_cell(pinned: Grid3dPinnedCell) -> Grid3dMutation {
    Grid3dMutation::PinCell(PinCell { pinned })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for PinCell {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "fix", entity: "cell", kind: "pin-cell", record: "Fixed" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Pin cell {} to tile \"{}\"", cell_key(self.pinned.x, self.pinned.y, self.pinned.z), self.pinned.tile_id), &format!("Zelle {} auf Kachel \"{}\" fixieren", cell_key(self.pinned.x, self.pinned.y, self.pinned.z), self.pinned.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![cell_key(self.pinned.x, self.pinned.y, self.pinned.z)]
    }
}
//#endregion 🔖️PinCell
