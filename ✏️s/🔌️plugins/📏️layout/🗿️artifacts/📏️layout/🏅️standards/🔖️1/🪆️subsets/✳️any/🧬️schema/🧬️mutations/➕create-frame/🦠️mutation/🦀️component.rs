//! ➕️ `create-frame` — inserts a new {@link Frame} into a page's `frames` list (paint-order
//! significant), optionally registering it on one of the page's layers.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region ➕️CreateFrame
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFrame {
    pub page_id: String,
    pub frame: Frame,
    pub index: Option<usize>,
    pub layer_id: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "frame", kind: "create-frame", record: "CreatedFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> LayoutDiff {
        super::diff::diff_create_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        super::inverse::inverse_create_frame(self, base)
    }
    fn label(&self) -> String {
        format!("Create frame \"{}\"", self.frame.id())
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame.id().to_string()]
    }
}
//#endregion ➕️CreateFrame
