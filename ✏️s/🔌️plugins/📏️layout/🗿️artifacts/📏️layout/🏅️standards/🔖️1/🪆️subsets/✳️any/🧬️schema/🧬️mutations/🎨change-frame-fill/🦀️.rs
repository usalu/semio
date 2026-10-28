//! 🎨 `change-frame-fill` — sets a `Frame::Rect`'s `fill` color (`None` clears it). A no-op on
//! non-rect frames, matching the pre-migration `PatchFrame`'s `fill` handling.

use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, PageFramesDelta, PagePatch};
use semio_s_artifact_layout_layout_diff::LayoutPagesModification;
use semio_s_artifact_layout_layout_model::Frame;
use semio_s_artifact_layout_layout_diff::{FramePatch, LayoutDiff};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🎨ChangeFrameFill
#[derive(Clone, Debug, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeFrameFill {
    pub page_id: String,
    pub frame_id: String,
    pub new_fill: Option<[f32; 4]>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ChangeFrameFill where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_frame_fill::ChangeFrameFill> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-fill", kind: "change-frame-fill", record: "ChangedFrameFill" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change frame \"{}\" fill", self.frame_id), &format!("Füllung von Rahmen \"{}\" ändern", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for ChangeFrameFill where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_frame_fill::ChangeFrameFill> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_frame_fill(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_frame_fill(self, base)?
    
    })
}
}
//#endregion 🎨ChangeFrameFill

//#region 🎨ChangeFrameFill
pub fn diff_change_frame_fill(payload: &ChangeFrameFill, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if !matches!(frame, Frame::Rect { .. }) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "The target is not a rectangle frame.", [payload.frame_id.clone()]);
    }
    if payload.new_fill.as_ref().is_some_and(|color| color.iter().any(|value| !value.is_finite())) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Frame colors must be finite.", [payload.frame_id.clone()]);
    }
    if let Frame::Rect { fill, .. } = frame {
        if semio_s_artifact_layout_layout_model::equality::equal_color(fill, &payload.new_fill) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Frame \"{}\" already has that fill.", payload.frame_id));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            modified: vec![LayoutPagesModification {
                id: payload.page_id.clone(),
                patch: PagePatch { frames: PageFramesDelta::modification(payload.frame_id.clone(), FramePatch { fill: Some(payload.new_fill), ..Default::default() }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🎨ChangeFrameFill

//#region 🎨ChangeFrameFill
pub fn inverse_change_frame_fill<Op: From<crate::change_frame_fill::ChangeFrameFill>>(payload: &ChangeFrameFill, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_change_frame_fill(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()); }
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Rect { fill, .. } = frame else {
        return Vec::new();
    };
    vec![Op::from(ChangeFrameFill { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_fill: *fill })]

    })())
}
//#endregion 🎨ChangeFrameFill


impl PartialEq for ChangeFrameFill {
    fn eq(&self, other: &Self) -> bool {
        self.page_id == other.page_id
            && self.frame_id == other.frame_id
            && semio_s_artifact_layout_layout_model::equality::equal_color(&self.new_fill, &other.new_fill)
    }
}
