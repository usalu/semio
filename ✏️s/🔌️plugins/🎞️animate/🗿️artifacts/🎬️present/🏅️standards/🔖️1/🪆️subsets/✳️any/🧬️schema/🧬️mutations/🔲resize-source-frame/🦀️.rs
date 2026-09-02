//! 🔲 `resize-source-frame` mutation payload — recrops the shared figure source's normalized
//! `x,y,width,height` frame (the play app's `set-frame` gesture).

use crate::artifacts::present::{FigureTileFrame, PresentSnapshot};
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::mutations::PresentMutation;
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔹Payload
/// 🔲 Replaces `source.frame` with `new_frame` — the crop rect is always authored as one atomic
/// `x,y,width,height` block (never a field at a time), so this is `resize` on the whole extent, per
/// the taxonomy's spatial-verb rule. Diff/inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "resize-source-frame")]
pub struct ResizeSourceFrame {
    #[dsl(block)]
    pub new_frame: FigureTileFrame,
}

impl MutationKind<PresentSnapshot, PresentMutation> for ResizeSourceFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "source-frame", kind: "resize-source-frame", record: "ResizedSourceFrame" };

    fn diff(&self, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PresentSnapshot) -> Vec<PresentMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Resize source frame to {:.2}x{:.2}", self.new_frame.width, self.new_frame.height)
    }

    fn target(&self) -> Vec<String> {
        vec!["source".into(), "frame".into()]
    }
}
//#endregion 🔹Payload
