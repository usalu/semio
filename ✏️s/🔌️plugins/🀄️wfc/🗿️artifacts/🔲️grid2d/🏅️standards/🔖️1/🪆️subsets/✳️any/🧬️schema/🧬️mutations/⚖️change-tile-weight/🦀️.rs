//! ⚖ Re-biases one tile's sampling weight — the `WeightTable` column the solver's entropy heuristic reads.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileWeight
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileWeight {
    pub id: String,
    pub weight: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_weight(id: String, weight: f64) -> Grid2dMutation {
    Grid2dMutation::ChangeTileWeight(ChangeTileWeight { id, weight })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for ChangeTileWeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change weight of tile \"{}\" to {}", self.id, self.weight), &format!("Gewicht von Kachel \"{}\" auf {} ändern", self.id, self.weight))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeTileWeight
