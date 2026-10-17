//! ⚖ `s.wfc.grid3d` mutation — `ChangeTileWeight`: retunes one tile's selection bias, the weight the
//! solver's weighted-roulette sampler and the entropy inference both read.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
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

impl<Op> MutationKind<Grid3dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<ChangeTileWeight>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change weight of tile \"{}\"", self.tile_id), &format!("Gewicht von Kachel \"{}\" ändern", self.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile_id.clone()]
    }
}
//#endregion 🔖️ChangeTileWeight
