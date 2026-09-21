//! 🀄️ `wfc3d` mutation — `CreateTile`: brings a new id-keyed tile into the placeable catalogue at a
//! FINAL-state insertion index.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::{Tile, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTile {
    pub index: usize,
    pub tile: Tile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile(index: usize, tile: Tile) -> Wfc3dMutation {
    Wfc3dMutation::CreateTile(CreateTile { index, tile })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for CreateTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
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
