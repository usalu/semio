//! 🖼️ `wfc3d` mutation — `ChangeTileMedia`: replaces what one tile LOOKS like, leaving its id,
//! weight and every rule naming it untouched.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::{TileMedia3d, Wfc3dSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeTileMedia {
    pub id: String,
    pub media: TileMedia3d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media<Op>(id: String, media: TileMedia3d) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia>,
{
    Op::from(ChangeTileMedia { id, media })
}

impl<Op> MutationKind<Wfc3dSnapshot, Op> for ChangeTileMedia
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<ChangeTileMedia>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change tile \"{}\" media", self.id), &format!("Medien von Kachel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeTileMedia
