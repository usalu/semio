//! 🕹️ `move-frame` — absolute spatial reposition of a frame's `bounds.x`/`bounds.y`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🕹️MoveFrame
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveFrame {
    pub page_id: String,
    pub frame_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for MoveFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "frame", kind: "move-frame", record: "MovedFrame" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_move_frame(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_move_frame(self, base)
    }
    async fn label(&self) -> String {
        format!("Move frame \"{}\"", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🕹️MoveFrame
