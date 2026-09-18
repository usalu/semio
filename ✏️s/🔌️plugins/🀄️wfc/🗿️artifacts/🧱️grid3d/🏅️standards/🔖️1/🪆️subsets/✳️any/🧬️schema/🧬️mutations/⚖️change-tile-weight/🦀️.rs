//! ⚖ `s.wfc.grid3d` mutation — `ChangeTileWeight`: retunes one tile's selection bias, the weight the
//! solver's weighted-roulette sampler and the entropy inference both read.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
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
pub fn change_tile_weight(tile_id: String, weight: f64) -> Grid3dMutation {
    Grid3dMutation::ChangeTileWeight(ChangeTileWeight { tile_id, weight })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for ChangeTileWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change weight of tile \"{}\"", self.tile_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile_id.clone()]
    }
}
//#endregion 🔖️ChangeTileWeight
