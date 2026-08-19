//! 🔢 `change-frame-columns` — sets a `Frame::Text`'s `columns` count. A no-op on non-text frames.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔢ChangeFrameColumns
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFrameColumns {
    pub page_id: String,
    pub frame_id: String,
    pub new_columns: u32,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameColumns {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-columns", kind: "change-frame-columns", record: "ChangedFrameColumns" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_change_frame_columns(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_change_frame_columns(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" columns", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🔢ChangeFrameColumns
