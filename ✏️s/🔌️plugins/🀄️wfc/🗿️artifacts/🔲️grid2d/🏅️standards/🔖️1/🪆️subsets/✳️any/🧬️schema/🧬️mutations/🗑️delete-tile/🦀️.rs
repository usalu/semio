//! 🗑 Removes a tile and cascades to every adjacency rule naming it and every cell pinned to it — a rule or pin may never dangle.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::Grid2dSnapshot;
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

impl<Op> MutationKind<Grid2dSnapshot, Op> for DeleteTile
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete tile \"{}\"", self.id), &format!("Kachel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Grid2dSnapshot, Op> for DeleteTile
where
    Op: protocol::ColdMutation<Grid2dSnapshot, ReferenceDiff = Grid2dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_cell::PinCell>,
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
//#endregion 🔖️DeleteTile
