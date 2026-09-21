//! 🎨 WFC 2D mutation — `ChangeTileMedia`: replaces what a tile LOOKS like without touching its id,
//! weight or any rule that names it — including swapping a vector tile for a raster one, which is
//! what this kind's committed vector does.

use crate::diff::Wfc2dDiff;
use crate::mutations::Wfc2dMutation;
use crate::schema::snapshot::Wfc2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::schema::snapshot::Wfc2dTileMedia;

//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileMedia {
    pub tile_id: String,
    pub media: Wfc2dTileMedia,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media(tile_id: String, media: Wfc2dTileMedia) -> Wfc2dMutation {
    Wfc2dMutation::ChangeTileMedia(ChangeTileMedia { tile_id, media })
}

impl MutationKind<Wfc2dSnapshot, Wfc2dMutation> for ChangeTileMedia {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Wfc2dSnapshot) -> protocol::MutationOutcome<Wfc2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc2dSnapshot) -> Vec<Wfc2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change Tile Media", "Kachelmedien ändern")
    }
}
//#endregion 🔖️ChangeTileMedia
