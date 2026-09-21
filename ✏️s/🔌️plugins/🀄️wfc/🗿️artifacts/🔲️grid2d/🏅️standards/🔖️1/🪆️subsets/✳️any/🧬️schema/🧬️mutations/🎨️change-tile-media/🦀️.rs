//! 🎨 Replaces what one tile LOOKS like — never what it means: the pattern universe, the rules and every pin keep addressing the same tile id.

use crate::diff::Grid2dDiff;
use crate::mutations::Grid2dMutation;
use crate::schema::snapshot::{Grid2dSnapshot, WfcTileMedia2d};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileMedia {
    pub id: String,
    pub media: WfcTileMedia2d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media(id: String, media: WfcTileMedia2d) -> Grid2dMutation {
    Grid2dMutation::ChangeTileMedia(ChangeTileMedia { id, media })
}

impl MutationKind<Grid2dSnapshot, Grid2dMutation> for ChangeTileMedia {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Vec<Grid2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change media of tile \"{}\"", self.id), &format!("Medien von Kachel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeTileMedia
