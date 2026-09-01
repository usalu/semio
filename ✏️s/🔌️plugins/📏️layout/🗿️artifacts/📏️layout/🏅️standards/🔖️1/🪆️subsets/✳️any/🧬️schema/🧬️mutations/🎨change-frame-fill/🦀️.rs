//! 🎨 `change-frame-fill` — sets a `Frame::Rect`'s `fill` color (`None` clears it). A no-op on
//! non-rect frames, matching the pre-migration `PatchFrame`'s `fill` handling.


use crate::artifacts::layout::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🎨ChangeFrameFill
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFrameFill {
    pub page_id: String,
    pub frame_id: String,
    pub new_fill: Option<[f32; 4]>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameFill {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-fill", kind: "change-frame-fill", record: "ChangedFrameFill" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_frame_fill(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_frame_fill(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" fill", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🎨ChangeFrameFill


//#region 🎨ChangeFrameFill
pub async fn diff_change_frame_fill(payload: &ChangeFrameFill, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Rect { fill, .. } = frame {
        if *fill == payload.new_fill {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame \"{}\" already has that fill.", payload.frame_id));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { fill: Some(payload.new_fill), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🎨ChangeFrameFill


//#region 🎨ChangeFrameFill
pub async fn inverse_change_frame_fill(payload: &ChangeFrameFill, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Rect { fill, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameFill(ChangeFrameFill { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_fill: *fill })]
}
//#endregion 🎨ChangeFrameFill
