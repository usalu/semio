//! ➕️ `create-frame` — inserts a new {@link Frame} into a page's `frames` list (paint-order
//! significant), optionally registering it on one of the page's layers.


use crate::artifacts::layout::{Frame, LayoutDiff, LayoutSnapshot, PageFrameAdded, PagePatch};
use crate::artifacts::layout::mutations::{LayoutMutation, delete_frame};
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ➕️CreateFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateFrame {
    pub page_id: String,
    pub frame: Frame,
    pub index: Option<usize>,
    pub layer_id: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "frame", kind: "create-frame", record: "CreatedFrame" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_frame(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_create_frame(self, base)
    }
    async fn label(&self) -> String {
        format!("Create frame \"{}\"", self.frame.id())
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame.id().to_string()]
    }
}
//#endregion ➕️CreateFrame


//#region ➕️CreateFrame
pub async fn diff_create_frame(payload: &CreateFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if page.frames.iter().any(|frame| frame.id() == payload.frame.id()) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A frame with id \"{}\" already exists on page \"{}\".", payload.frame.id(), payload.page_id), [payload.frame.id().to_string()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: payload.page_id.clone(), patch: PagePatch { frame_added: Some(PageFrameAdded { frame: payload.frame.clone(), index: payload.index, layer_id: payload.layer_id.clone() }), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion ➕️CreateFrame


//#region ➕️CreateFrame
pub async fn inverse_create_frame(payload: &CreateFrame, _base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    vec![LayoutMutation::DeleteFrame(delete_frame::DeleteFrame { page_id: payload.page_id.clone(), frame_id: payload.frame.id().to_string() })]
}
//#endregion ➕️CreateFrame
