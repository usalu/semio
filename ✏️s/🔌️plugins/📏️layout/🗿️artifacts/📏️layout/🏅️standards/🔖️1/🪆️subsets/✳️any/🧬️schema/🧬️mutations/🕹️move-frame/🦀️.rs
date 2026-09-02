//! 🕹️ `move-frame` — absolute spatial reposition of a frame's `bounds.x`/`bounds.y`.


use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🕹️MoveFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveFrame {
    pub page_id: String,
    pub frame_id: String,
    pub new_x: f64,
    pub new_y: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for MoveFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "frame", kind: "move-frame", record: "MovedFrame" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_move_frame(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_move_frame(self, base)
    }
    async fn label(&self) -> String {
        format!("Move frame \"{}\"", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🕹️MoveFrame


//#region 🕹️MoveFrame
pub async fn diff_move_frame(payload: &MoveFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if !page.frames.iter().any(|frame| frame.id() == payload.frame_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    }
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame \"{}\" position must be finite, got ({}, {}).", payload.frame_id, payload.new_x, payload.new_y), [payload.frame_id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { x: Some(payload.new_x), y: Some(payload.new_y), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🕹️MoveFrame


//#region 🕹️MoveFrame
pub async fn inverse_move_frame(payload: &MoveFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let bounds = frame.bounds();
    vec![LayoutMutation::MoveFrame(MoveFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_x: bounds.x, new_y: bounds.y })]
}
//#endregion 🕹️MoveFrame
