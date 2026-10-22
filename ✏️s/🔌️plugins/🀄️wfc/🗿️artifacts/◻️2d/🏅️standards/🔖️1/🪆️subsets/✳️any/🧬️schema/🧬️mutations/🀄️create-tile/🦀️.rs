//! 🀄 WFC 2D mutation — `CreateTile`: adds one pattern to the alphabet the solve draws from, with
//! its own inline media and selection weight.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dTile;

//#region 🔖️CreateTile
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateTile {
    pub tile: Wfc2dTile,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_tile<Op>(tile: Wfc2dTile) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_tile::CreateTile>,
{
    Op::from(CreateTile { tile })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for CreateTile
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "tile", kind: "create-tile", record: "CreatedTile" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Tile", "Kachel erstellen")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for CreateTile
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<CreateTile> + From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
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
//#endregion 🔖️CreateTile
