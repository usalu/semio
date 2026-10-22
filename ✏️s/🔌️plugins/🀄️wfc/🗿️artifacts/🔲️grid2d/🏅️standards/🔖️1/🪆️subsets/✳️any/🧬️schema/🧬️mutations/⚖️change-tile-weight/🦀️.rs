//! ⚖ Re-biases one tile's sampling weight — the `WeightTable` column the solver's entropy heuristic reads.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileWeight
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileWeight {
    pub id: String,
    pub weight: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_weight<Op>(id: String, weight: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight>,
{
    Op::from(ChangeTileWeight { id, weight })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<ChangeTileWeight>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change weight of tile \"{}\" to {}", self.id, self.weight), &format!("Gewicht von Kachel \"{}\" auf {} ändern", self.id, self.weight))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Grid2dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::ColdMutation<Grid2dSnapshot, ReferenceDiff = Grid2dDiff> + From<ChangeTileWeight>,
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
//#endregion 🔖️ChangeTileWeight
