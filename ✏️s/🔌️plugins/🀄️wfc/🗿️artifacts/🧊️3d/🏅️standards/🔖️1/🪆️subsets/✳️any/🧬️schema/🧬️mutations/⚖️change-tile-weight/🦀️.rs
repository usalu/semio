//! ⚖️ `wfc3d` mutation — `ChangeTileWeight`: sets one tile's selection bias, the value the engine's
//! `WeightTable` turns into a Shannon-entropy term.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeTileWeight
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
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

impl<Op> MutationKind<Wfc3dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<ChangeTileWeight>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-weight", kind: "change-tile-weight", record: "ChangedTileWeight" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change tile \"{}\" weight", self.id), &format!("Gewicht von Kachel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Wfc3dSnapshot, Op> for ChangeTileWeight
where
    Op: protocol::ColdMutation<Wfc3dSnapshot, ReferenceDiff = Wfc3dDiff> + From<ChangeTileWeight>,
{
    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeTileWeight
