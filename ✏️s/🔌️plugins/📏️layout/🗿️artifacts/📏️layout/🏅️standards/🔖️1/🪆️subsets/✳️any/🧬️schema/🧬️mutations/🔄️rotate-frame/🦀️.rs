//! 🔄️ `rotate-frame` — sets a frame's `bounds.rotation` in radians.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{FramePatch, LayoutDiff, LayoutSnapshot, PageFramePatched, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔄️RotateFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct RotateFrame {
    pub page_id: String,
    pub frame_id: String,
    pub new_rotation: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RotateFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "frame", kind: "rotate-frame", record: "RotatedFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rotate_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        inverse_rotate_frame(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Rotate frame \"{}\"", self.frame_id), &format!("Rahmen \"{}\" drehen", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion 🔄️RotateFrame

//#region 🔄️RotateFrame
pub fn diff_rotate_frame(payload: &RotateFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if !page.frames.iter().any(|frame| frame.id() == payload.frame_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    }
    if !payload.new_rotation.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Frame \"{}\" rotation must be finite, got {}.", payload.frame_id, payload.new_rotation), [payload.frame_id.clone()]);
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry {
                id: payload.page_id.clone(),
                patch: PagePatch { frame_patched: Some(PageFramePatched { frame_id: payload.frame_id.clone(), patch: FramePatch { rotation: Some(payload.new_rotation), ..Default::default() } }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔄️RotateFrame

//#region 🔄️RotateFrame
pub fn inverse_rotate_frame(payload: &RotateFrame, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    vec![LayoutMutation::RotateFrame(RotateFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_rotation: frame.bounds().rotation })]
}
//#endregion 🔄️RotateFrame
