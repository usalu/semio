//! 📏 `resize-frame` — changes a frame's `bounds.width`/`bounds.height` extent.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 📏ResizeFrame
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResizeFrame {
    pub page_id: String,
    pub frame_id: String,
    pub new_width: f64,
    pub new_height: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ResizeFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "frame", kind: "resize-frame", record: "ResizedFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_resize_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_resize_frame(self, base)
    }
    fn label(&self) -> String {
        format!("Resize frame \"{}\"", self.frame_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 📏ResizeFrame
