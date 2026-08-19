//! ➖️ `delete-frame` — removes a {@link Frame} from a page by id (and every layer's `object_ids`
//! referencing it); inverse recreates it via `create-frame`.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ➖️DeleteFrame
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteFrame {
    pub page_id: String,
    pub frame_id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "frame", kind: "delete-frame", record: "DeletedFrame" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        super::diff::diff_delete_frame(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_delete_frame(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete frame \"{}\"", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion ➖️DeleteFrame
