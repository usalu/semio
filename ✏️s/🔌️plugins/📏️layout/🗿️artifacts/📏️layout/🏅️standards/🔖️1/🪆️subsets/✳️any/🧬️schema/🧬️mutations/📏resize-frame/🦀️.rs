//! 📏 `resize-frame` — changes a frame's `bounds.width`/`bounds.height` extent.


use crate::artifacts::layout::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📏ResizeFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ResizeFrame {
    pub page_id: String,
    pub frame_id: String,
    pub new_width: f64,
    pub new_height: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ResizeFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "resize", entity: "frame", kind: "resize-frame", record: "ResizedFrame" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_resize_frame(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_resize_frame(self, base)
    }
    async fn label(&self) -> String {
        format!("Resize frame \"{}\"", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 📏ResizeFrame


//#region 📏ResizeFrame
pub async fn diff_resize_frame(payload: &ResizeFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if !page.frames.iter().any(|frame| frame.id() == payload.frame_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    }
    if !payload.new_width.is_finite() || !payload.new_height.is_finite() || payload.new_width <= 0.0 || payload.new_height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame \"{}\" size must be finite and positive, got ({}, {}).", payload.frame_id, payload.new_width, payload.new_height), [payload.frame_id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { width: Some(payload.new_width), height: Some(payload.new_height), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 📏ResizeFrame


//#region 📏ResizeFrame
pub async fn inverse_resize_frame(payload: &ResizeFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let bounds = frame.bounds();
    vec![LayoutMutation::ResizeFrame(ResizeFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_width: bounds.width, new_height: bounds.height })]
}
//#endregion 📏ResizeFrame
