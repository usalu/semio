//! 🧱 `s.wfc.grid3d` mutation — `CreateTile`: brings one placeable tile into the pattern universe,
//! inserted at its CANONICAL SORTED position so `delete-tile`'s inverse restores it in place.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateTile {
    pub tile: Grid3dTile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile(tile: Grid3dTile) -> Grid3dMutation {
    Grid3dMutation::CreateTile(CreateTile { tile })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for CreateTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Create tile \"{}\"", self.tile.id), &format!("Kachel \"{}\" erstellen", self.tile.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile.id.clone()]
    }
}
//#endregion 🔖️CreateTile
