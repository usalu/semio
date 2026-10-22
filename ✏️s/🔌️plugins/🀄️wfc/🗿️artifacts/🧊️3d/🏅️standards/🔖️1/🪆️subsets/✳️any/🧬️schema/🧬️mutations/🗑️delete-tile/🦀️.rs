//! 🗑️ `wfc3d` mutation — `DeleteTile`: removes an id-addressed tile and cascades to every rule that
//! names it and every slot pinned to it (no dangling reference survives a delete).

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
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

impl<Op> MutationKind<Wfc3dSnapshot, Op> for DeleteTile
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "tile", kind: "delete-tile", record: "DeletedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete tile \"{}\"", self.id), &format!("Kachel \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Wfc3dSnapshot, Op> for DeleteTile
where
    Op: protocol::ColdMutation<Wfc3dSnapshot, ReferenceDiff = Wfc3dDiff> + From<DeleteTile> + From<crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule> + From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::pin_slot::PinSlot>,
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
//#endregion 🔖️DeleteTile
