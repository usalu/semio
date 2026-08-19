//! 🎨 `change-frame-fill` — sets a `Frame::Rect`'s `fill` color (`None` clears it). A no-op on
//! non-rect frames, matching the pre-migration `PatchFrame`'s `fill` handling.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🎨ChangeFrameFill
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFrameFill {
    pub page_id: String,
    pub frame_id: String,
    pub new_fill: Option<[f32; 4]>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameFill {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-fill", kind: "change-frame-fill", record: "ChangedFrameFill" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_frame_fill(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_frame_fill(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" fill", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🎨ChangeFrameFill
