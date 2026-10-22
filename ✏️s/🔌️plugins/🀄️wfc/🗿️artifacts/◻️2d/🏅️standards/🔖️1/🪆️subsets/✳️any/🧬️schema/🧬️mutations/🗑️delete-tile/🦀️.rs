//! 🗑 WFC 2D mutation — `DeleteTile`: removes one pattern from the alphabet AND cascades every rule
//! that named it plus every slot pin that held it, so no reference is ever left dangling.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
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

impl<Op> MutationKind<Wfc2dSnapshot, Op> for DeleteTile
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete Tile", "Kachel löschen")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for DeleteTile
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
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
//#endregion 🔖️DeleteTile
