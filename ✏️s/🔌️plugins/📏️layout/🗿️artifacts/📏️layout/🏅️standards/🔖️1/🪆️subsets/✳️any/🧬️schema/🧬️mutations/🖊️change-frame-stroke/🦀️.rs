//! 🖊️ `change-frame-stroke` — sets a `Frame::Rect`'s `stroke` color (`None` clears it). A no-op on
//! non-rect frames, matching the pre-migration `PatchFrame`'s `stroke` handling.


use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, PageFramesDelta, PagePatch};
use semio_s_artifact_layout_layout_diff::LayoutPagesModification;
use semio_s_artifact_layout_layout_model::Frame;
use semio_s_artifact_layout_layout_diff::{FramePatch, LayoutDiff};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🖊️ChangeFrameStroke
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeFrameStroke {
    pub page_id: String,
    pub frame_id: String,
    pub new_stroke: Option<[f32; 4]>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ChangeFrameStroke where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_frame_stroke::ChangeFrameStroke> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "frame-stroke", kind: "change-frame-stroke", record: "ChangedFrameStroke" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change frame \"{}\" stroke", self.frame_id), &format!("Kontur von Rahmen \"{}\" ändern", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for ChangeFrameStroke where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_frame_stroke::ChangeFrameStroke> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_frame_stroke(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_frame_stroke(self, base)?
    
    })
}
}
//#endregion 🖊️ChangeFrameStroke

//#region 🖊️ChangeFrameStroke
pub fn diff_change_frame_stroke(payload: &ChangeFrameStroke, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    if let Frame::Rect { stroke, .. } = frame {
        if *stroke == payload.new_stroke {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Frame \"{}\" already has that stroke.", payload.frame_id));
        }
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta {
            modified: vec![LayoutPagesModification {
                id: payload.page_id.clone(),
                patch: PagePatch { frames: PageFramesDelta::modification(payload.frame_id.clone(), FramePatch { stroke: Some(payload.new_stroke), ..Default::default() }), ..Default::default() },
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
//#endregion 🖊️ChangeFrameStroke

//#region 🖊️ChangeFrameStroke
pub fn inverse_change_frame_stroke<Op: From<crate::change_frame_stroke::ChangeFrameStroke>>(payload: &ChangeFrameStroke, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(frame) = page.frames.iter().find(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let Frame::Rect { stroke, .. } = frame else {
        return Vec::new();
    };
    vec![Op::from(ChangeFrameStroke { page_id: payload.page_id.clone(), frame_id: payload.frame_id.clone(), new_stroke: *stroke })]

    })())
}
//#endregion 🖊️ChangeFrameStroke

