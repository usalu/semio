//! 🎨 WFC 2D mutation — `ChangeTileMedia`: replaces what a tile LOOKS like without touching its id,
//! weight or any rule that names it — including swapping a vector tile for a raster one, which is
//! what this kind's committed vector does.

use crate::diff::Wfc2dDiff;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dTileMedia;

//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileMedia {
    pub tile_id: String,
    pub media: Wfc2dTileMedia,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media<Op>(tile_id: String, media: Wfc2dTileMedia) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia>,
{
    Op::from(ChangeTileMedia { tile_id, media })
}

impl<Op> MutationKind<Wfc2dSnapshot, Op> for ChangeTileMedia
where
    Op: protocol::Mutation<Wfc2dSnapshot, Diff = Wfc2dDiff> + From<ChangeTileMedia>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Tile Media", "Medien der Kachel ändern")
    }
}

impl<Op> protocol::ColdMutationKind<Wfc2dSnapshot, Op> for ChangeTileMedia
where
    Op: protocol::ColdMutation<Wfc2dSnapshot, ReferenceDiff = Wfc2dDiff> + From<ChangeTileMedia>,
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
//#endregion 🔖️ChangeTileMedia
