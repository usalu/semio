//! 🖼 `s.wfc.grid3d` mutation — `ChangeTileMedia`: swaps one tile's geometry, inline or composed-child,
//! without disturbing its id, weight or any rule that names it.

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileMedia {
    pub tile_id: String,
    pub media: Grid3dTileMedia,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media<Op>(tile_id: String, media: Grid3dTileMedia) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia>,
{
    Op::from(ChangeTileMedia { tile_id, media })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for ChangeTileMedia
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<ChangeTileMedia>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change media of tile \"{}\"", self.tile_id), &format!("Medien von Kachel \"{}\" ändern", self.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile_id.clone()]
    }
}
//#endregion 🔖️ChangeTileMedia
