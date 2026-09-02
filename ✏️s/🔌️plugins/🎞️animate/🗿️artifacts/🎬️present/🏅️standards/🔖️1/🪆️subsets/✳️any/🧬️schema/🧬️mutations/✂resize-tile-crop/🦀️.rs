//! ✂️ `resize-tile-crop` mutation payload — recrops a figure tile's normalized `x,y,width,height`
//! frame within the shared source (the play app's `patch-tile-crops` gesture).

use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// ✂️ Replaces the `tiles` entry addressed by `id`'s `crop` with `new_crop` — the crop rect is
/// always authored as one atomic `x,y,width,height` block, so this is `resize` on the whole extent.
/// Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "resize-tile-crop")]
pub struct ResizeTileCrop {
    pub id: String,
    #[dsl(block)]
    pub new_crop: FigureTileFrame,
}

impl MutationKind<PresentSnapshot, PresentMutation> for ResizeTileCrop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "tile-crop", kind: "resize-tile-crop", record: "ResizedTileCrop" };

    fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Resize tile \"{}\" crop", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone(), "crop".into()]
    }
}
//#endregion 🔹Payload
