//! 🧱 `s.wfc.grid3d` mutation — `CreateTile`: brings one placeable tile into the pattern universe,
//! inserted at its CANONICAL SORTED position so `delete-tile`'s inverse restores it in place.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateTile {
    pub tile: Grid3dTile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile<Op>(tile: Grid3dTile) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile>,
{
    Op::from(CreateTile { tile })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for CreateTile
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create tile \"{}\"", self.tile.id), &format!("Kachel \"{}\" erstellen", self.tile.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile.id.clone()]
    }
}
//#endregion 🔖️CreateTile
