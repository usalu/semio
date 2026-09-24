//! 🖼️ `wfc3d` mutation — `ChangeTileMedia`: replaces what one tile LOOKS like, leaving its id,
//! weight and every rule naming it untouched.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::{TileMedia3d, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeTileMedia {
    pub id: String,
    pub media: TileMedia3d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media(id: String, media: TileMedia3d) -> Wfc3dMutation {
    Wfc3dMutation::ChangeTileMedia(ChangeTileMedia { id, media })
}

impl MutationKind<Wfc3dSnapshot, Wfc3dMutation> for ChangeTileMedia {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Vec<Wfc3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Change tile \"{}\" media", self.id), &format!("Medien von Kachel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeTileMedia
