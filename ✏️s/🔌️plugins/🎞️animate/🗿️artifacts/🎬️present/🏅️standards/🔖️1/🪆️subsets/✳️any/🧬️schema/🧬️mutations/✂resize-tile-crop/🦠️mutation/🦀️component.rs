//! ✂️ `resize-tile-crop` mutation payload — recrops a figure tile's normalized `x,y,width,height`
//! frame within the shared source (the play app's `patch-tile-crops` gesture).
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// ✂️ Replaces the `tiles` entry addressed by `id`'s `crop` with `new_crop` — the crop rect is
/// always authored as one atomic `x,y,width,height` block, so this is `resize` on the whole extent.
/// Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "resize-tile-crop")]
pub struct ResizeTileCrop {
    pub id: String,
    #[dsl(block)]
    pub new_crop: FigureTileFrame,
}

impl MutationKind<PresentSnapshot, PresentMutation> for ResizeTileCrop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "tile-crop", kind: "resize-tile-crop", record: "ResizedTileCrop" };

    async fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Resize tile \"{}\" crop", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec!["tiles".into(), self.id.clone(), "crop".into()]
    }
}
//#endregion 🔹Payload
