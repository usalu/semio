//! 🀄 WFC 2D mutation — `CreateTile`: adds one pattern to the alphabet the solve draws from, with
//! its own inline media and selection weight.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dTile;

//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateTile {
    pub tile: Wfc2dTile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile(tile: Wfc2dTile) -> Wfc2dMutation {
    Wfc2dMutation::CreateTile(CreateTile { tile })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for CreateTile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Create Tile", "Kachel erstellen")
    }
}
//#endregion 🔖️CreateTile
