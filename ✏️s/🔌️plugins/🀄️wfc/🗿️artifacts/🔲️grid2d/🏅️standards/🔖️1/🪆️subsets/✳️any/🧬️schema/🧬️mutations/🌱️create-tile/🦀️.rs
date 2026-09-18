//! 🌱 Brings a new tile into the pattern universe, inserted at its canonical sorted position so a later `delete-tile` inverse restores it exactly where it was.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::{Grid2dSnapshot, WfcTile2d};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateTile {
    pub tile: WfcTile2d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile(tile: WfcTile2d) -> Grid2dMutation {
    Grid2dMutation::CreateTile(CreateTile { tile })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for CreateTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create tile \"{}\"", self.tile.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile.id.clone()]
    }
}
//#endregion 🔖️CreateTile
