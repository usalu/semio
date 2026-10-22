//! 🕳 `s.wfc.grid3d` mutation — `DeleteTile`: removes one tile AND cascades every adjacency rule and
//! every cell pin that named it, so the document never keeps a dangling tile reference. The inverse
//! is base-derived and restores each cascaded row at its own canonical sorted position.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteTile {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_tile<Op>(id: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    Op::from(DeleteTile { id })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for DeleteTile
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete tile \"{}\"", self.id), &format!("Kachel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Grid3dSnapshot, Op> for DeleteTile
where
    Op: protocol::ColdMutation<Grid3dSnapshot, ReferenceDiff = Grid3dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️DeleteTile
