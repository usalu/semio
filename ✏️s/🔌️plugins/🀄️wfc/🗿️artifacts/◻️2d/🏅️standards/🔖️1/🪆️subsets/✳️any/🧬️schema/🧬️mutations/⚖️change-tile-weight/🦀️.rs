//! ⚖ WFC 2D mutation — `ChangeTileWeight`: retunes one tile's selection bias, the `WeightTable`
//! input the engine's Shannon-entropy heuristic reads.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileWeight
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileWeight {
    pub tile_id: String,
    pub weight: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_weight(tile_id: String, weight: f64) -> Wfc2dMutation {
    Wfc2dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for ChangeTileWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change Tile Weight", "Gewicht der Kachel ändern")
    }
}
//#endregion 🔖️ChangeTileWeight
