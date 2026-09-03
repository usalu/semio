//! 🔤 `change-frame-wrap-mode` — sets a `Frame::Text`'s `wrap_mode`. A no-op on non-text frames.


use crate::artifacts::layout::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔤ChangeFrameWrapMode
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFrameWrapMode {
    pub page_id: String,
    pub frame_id: String,
    pub new_wrap_mode: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameWrapMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-wrap-mode", kind: "change-frame-wrap-mode", record: "ChangedFrameWrapMode" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_frame_wrap_mode(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_frame_wrap_mode(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" wrap mode", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🔤ChangeFrameWrapMode


//#region 🔤ChangeFrameWrapMode
pub async fn diff_change_frame_wrap_mode(payload: &ChangeFrameWrapMode, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Text { wrap_mode, .. } = frame {
        if *wrap_mode == payload.new_wrap_mode {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame \"{}\" already has wrap mode \"{}\".", payload.frame_id, payload.new_wrap_mode));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { wrap_mode: Some(payload.new_wrap_mode.clone()), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔤ChangeFrameWrapMode


//#region 🔤ChangeFrameWrapMode
pub async fn inverse_change_frame_wrap_mode(payload: &ChangeFrameWrapMode, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Text { wrap_mode, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameWrapMode(ChangeFrameWrapMode { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_wrap_mode: wrap_mode.clone() })]
}
//#endregion 🔤ChangeFrameWrapMode
