//! 🖊️ `change-frame-stroke` — sets a `Frame::Rect`'s `stroke` color (`None` clears it). A no-op on
//! non-rect frames, matching the pre-migration `PatchFrame`'s `stroke` handling.


use crate::artifacts::layout::{Frame, FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖊️ChangeFrameStroke
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFrameStroke {
    pub page_id: String,
    pub frame_id: String,
    pub new_stroke: Option<[f32; 4]>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeFrameStroke {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-stroke", kind: "change-frame-stroke", record: "ChangedFrameStroke" };
    async fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_frame_stroke(self, base)
    }
    async fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_change_frame_stroke(self, base)
    }
    async fn label(&self) -> String {
        format!("Change frame \"{}\" stroke", self.frame_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🖊️ChangeFrameStroke


//#region 🖊️ChangeFrameStroke
pub async fn diff_change_frame_stroke(payload: &ChangeFrameStroke, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Rect { stroke, .. } = frame {
        if *stroke == payload.new_stroke {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Frame \"{}\" already has that stroke.", payload.frame_id));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { stroke: Some(payload.new_stroke), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🖊️ChangeFrameStroke


//#region 🖊️ChangeFrameStroke
pub async fn inverse_change_frame_stroke(payload: &ChangeFrameStroke, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Rect { stroke, .. } = frame else {
        return Vec::new();
    };
    vec![LayoutMutation::ChangeFrameStroke(ChangeFrameStroke { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_stroke: *stroke })]
}
//#endregion 🖊️ChangeFrameStroke
