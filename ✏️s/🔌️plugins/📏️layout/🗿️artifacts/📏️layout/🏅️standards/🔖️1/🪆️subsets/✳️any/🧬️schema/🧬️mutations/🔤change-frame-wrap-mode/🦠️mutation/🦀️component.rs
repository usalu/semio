//! 🔤 `change-frame-wrap-mode` — sets a `Frame::Text`'s `wrap_mode`. A no-op on non-text frames.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔤ChangeFrameWrapMode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFrameWrapMode {
    pub page_id: String,
    pub frame_id: String,
    pub new_wrap_mode: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameWrapMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-wrap-mode", kind: "change-frame-wrap-mode", record: "ChangedFrameWrapMode" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_frame_wrap_mode(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_frame_wrap_mode(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" wrap mode", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🔤ChangeFrameWrapMode
