//! 🔄️ `rotate-frame` — sets a frame's `bounds.rotation` in radians.

use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, PageFramesDelta, PagePatch};
use semio_s_artifact_layout_layout_diff::LayoutPagesModification;
use semio_s_artifact_layout_layout_diff::{FramePatch, LayoutDiff};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔄️RotateFrame
#[derive(Clone, Debug, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RotateFrame {
    pub page_id: String,
    pub frame_id: String,
    pub new_rotation: f64,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for RotateFrame where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::rotate_frame::RotateFrame> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "frame", kind: "rotate-frame", record: "RotatedFrame" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate frame \"{}\"", self.frame_id), &format!("Rahmen \"{}\" drehen", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for RotateFrame where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::rotate_frame::RotateFrame> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rotate_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_rotate_frame(self, base)?
    
    })
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
            modified: vec![LayoutPagesModification {
                id: payload.page_id.clone(),
                patch: PagePatch { frames: PageFramesDelta::modification(payload.frame_id.clone(), FramePatch { rotation: Some(payload.new_rotation), ..Default::default() }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🔄️RotateFrame

//#region 🔄️RotateFrame
pub fn inverse_rotate_frame<Op: From<crate::rotate_frame::RotateFrame>>(payload: &RotateFrame, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    vec![Op::from(RotateFrame { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_rotation: frame.bounds().rotation })]

    })())
}
//#endregion 🔄️RotateFrame


impl PartialEq for RotateFrame {
    fn eq(&self, other: &Self) -> bool {
        self.page_id == other.page_id
            && self.frame_id == other.frame_id
            && semio_framework_value::equal_f64(self.new_rotation, other.new_rotation)
    }
}
