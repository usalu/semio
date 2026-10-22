//! 🀄️ `wfc3d` mutation — `CreateTile`: brings a new id-keyed tile into the placeable catalogue at a
//! FINAL-state insertion index.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::{Tile, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateTile {
    pub index: usize,
    pub tile: Tile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile<Op>(index: usize, tile: Tile) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile>,
{
    Op::from(CreateTile { index, tile })
}

impl<Op> MutationKind<Wfc3dSnapshot, Op> for CreateTile
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create tile \"{}\"", self.tile.id), &format!("Kachel \"{}\" erstellen", self.tile.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Wfc3dSnapshot, Op> for CreateTile
where
    Op: protocol::ColdMutation<Wfc3dSnapshot, ReferenceDiff = Wfc3dDiff> + From<CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
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
//#endregion 🔖️CreateTile
