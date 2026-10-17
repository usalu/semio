//! 🎨 Replaces what one tile LOOKS like — never what it means: the pattern universe, the rules and every pin keep addressing the same tile id.

use crate::diff::Grid2dDiff;
use crate::schema::snapshot::{Grid2dSnapshot, WfcTileMedia2d};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeTileMedia
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeTileMedia {
    pub id: String,
    pub media: WfcTileMedia2d,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_tile_media<Op>(id: String, media: WfcTileMedia2d) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_tile_media::ChangeTileMedia>,
{
    Op::from(ChangeTileMedia { id, media })
}

impl<Op> MutationKind<Grid2dSnapshot, Op> for ChangeTileMedia
where
    Op: protocol::Mutation<Grid2dSnapshot, Diff = Grid2dDiff> + From<ChangeTileMedia>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "tile-media", kind: "change-tile-media", record: "ChangedTileMedia" };

    fn diff(&self, base: &Grid2dSnapshot) -> protocol::MutationOutcome<Grid2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change media of tile \"{}\"", self.id), &format!("Medien von Kachel \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeTileMedia
