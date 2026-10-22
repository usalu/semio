//! ⚖ WFC 2D mutation — `ChangeTileWeight`: retunes one tile's selection bias, the `WeightTable`
//! input the engine's Shannon-entropy heuristic reads.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileWeight
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileWeight {
    pub tile_id: String,
    pub weight: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_weight<Op>(tile_id: String, weight: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_weight::ChangeTileWeight>,
{
    Op::from(ChangeTileWeight { tile_id, weight })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<ChangeTileWeight>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Tile Weight", "Gewicht der Kachel ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<ChangeTileWeight>,
{
    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeTileWeight
