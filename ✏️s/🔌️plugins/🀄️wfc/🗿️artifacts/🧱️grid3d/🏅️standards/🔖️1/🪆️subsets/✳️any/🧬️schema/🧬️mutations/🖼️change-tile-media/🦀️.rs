//! 🖼 `s.wfc.grid3d` mutation — `ChangeTileMedia`: swaps one tile's geometry, inline or composed-child,
//! without disturbing its id, weight or any rule that names it.

use crate::diff::Grid3dDiff;
use crate::mutations::Grid3dMutation;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileMedia {
    pub tile_id: String,
    pub media: Grid3dTileMedia,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media(tile_id: String, media: Grid3dTileMedia) -> Grid3dMutation {
    Grid3dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media })
}

impl MutationKind<Grid3dSnapshot, Grid3dMutation> for ChangeTileMedia {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Vec<Grid3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change media of tile \"{}\"", self.tile_id), &format!("Medien von Kachel \"{}\" ändern", self.tile_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.tile_id.clone()]
    }
}
//#endregion 🔖️ChangeTileMedia
