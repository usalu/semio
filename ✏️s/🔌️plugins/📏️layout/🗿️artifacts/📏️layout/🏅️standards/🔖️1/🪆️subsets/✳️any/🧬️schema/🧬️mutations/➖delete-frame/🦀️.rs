//! ➖️ `delete-frame` — removes a {@link Frame} from a page by id (and every layer's `object_ids`
//! referencing it); inverse recreates it via `create-frame`.


use crate::artifacts::layout::{LayoutDiff, LayoutSnapshot, PagePatch};
use crate::artifacts::layout::mutations::{LayoutMutation, create_frame};
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ➖️DeleteFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteFrame {
    pub page_id: String,
    pub frame_id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "frame", kind: "delete-frame", record: "DeletedFrame" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_frame(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_delete_frame(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete frame \"{}\"", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion ➖️DeleteFrame


//#region ➖️DeleteFrame
pub async fn diff_delete_frame(payload: &DeleteFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if !page.frames.iter().any(|frame| frame.id() == payload.frame_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.page_id.clone(), patch: PagePatch { frame_removed: Some(payload.frame_id.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ➖️DeleteFrame


//#region ➖️DeleteFrame
pub async fn inverse_delete_frame(payload: &DeleteFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(index) = page.frames.iter().position(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let frame = page.frames[index].clone();
    let layer_id = page.layers.iter().find(|layer| layer.object_ids.iter().any(|id| id == &payload.frame_id)).map(|layer| layer.id.clone());
    vec![LayoutMutation::CreateFrame(create_frame::CreateFrame { page_id: payload.page_id.clone(), frame, index: Some(index), layer_id })]
}
//#endregion ➖️DeleteFrame
