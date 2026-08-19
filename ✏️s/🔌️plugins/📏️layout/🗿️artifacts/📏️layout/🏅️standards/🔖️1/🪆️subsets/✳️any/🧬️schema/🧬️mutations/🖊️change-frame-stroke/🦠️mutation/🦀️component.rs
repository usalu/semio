//! 🖊️ `change-frame-stroke` — sets a `Frame::Rect`'s `stroke` color (`None` clears it). A no-op on
//! non-rect frames, matching the pre-migration `PatchFrame`'s `stroke` handling.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🖊️ChangeFrameStroke
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFrameStroke {
    pub page_id: String,
    pub frame_id: String,
    pub new_stroke: Option<[f32; 4]>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameStroke {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-stroke", kind: "change-frame-stroke", record: "ChangedFrameStroke" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_frame_stroke(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_frame_stroke(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" stroke", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🖊️ChangeFrameStroke
