//! 🔃️ `rotate-frames` — a relative, parametric turn of frames of one page about one recorded pivot: every frame centre
//! orbits the pivot by the angle and every frame's rotation grows by it, so editing the turn in history re-derives every
//! pose from whatever base it replays on.

use crate::mutations::{layout_frame_centre, layout_frame_selection_diff, layout_frame_selection_targets, layout_frame_targets_invariant, layout_label_frames, layout_label_number, move_frame::MoveFrame, rotate_frame::RotateFrame, LayoutMutation};
use crate::{FramePatch, LayoutBounds, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔃️RotateFrames
/// 🔃️ `rotate-frames` payload — the page, the frames it turns (literal ids), the pivot and the counter-clockwise angle
/// in radians.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RotateFrames {
    pub page_id: String,
    pub targets: Vec<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub angle: f64,
}

impl RotateFrames {
    /// 🔃️ The bounds a frame holds after the turn: its centre orbits the pivot, its rotation grows by the angle, its extent stays.
    pub fn turned(&self, bounds: &LayoutBounds) -> LayoutBounds {
        let (sin, cos) = (self.angle.sin(), self.angle.cos());
        let (centre_x, centre_y) = layout_frame_centre(bounds);
        let (offset_x, offset_y) = (centre_x - self.pivot_x, centre_y - self.pivot_y);
        let (turned_x, turned_y) = (self.pivot_x + offset_x * cos - offset_y * sin, self.pivot_y + offset_x * sin + offset_y * cos);
        LayoutBounds { x: turned_x - bounds.width * 0.5, y: turned_y - bounds.height * 0.5, rotation: bounds.rotation + self.angle, ..bounds.clone() }
    }
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RotateFrames {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "frames", kind: "rotate-frames", record: "RotatedFrames" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rotate_frames(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
        inverse_rotate_frames(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (degrees_en, degrees_de) = layout_label_number(self.angle.to_degrees());
        let (en, de) = layout_label_frames(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {en} by {degrees_en}°"), &format!("{de} um {degrees_de}° drehen"))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone()]
    }
}
//#endregion 🔃️RotateFrames

//#region 🔺️Diff
/// 🔺️ Every unlocked target frame's BASE centre orbits the pivot and its rotation grows by the angle; the extent stays.
pub fn diff_rotate_frames(payload: &RotateFrames, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A turn's pivot and angle must be finite.", payload.targets.clone());
    }
    layout_frame_selection_diff(base, &payload.page_id, &payload.targets, |bounds| payload.turned(bounds), |next| FramePatch { x: Some(next.x), y: Some(next.y), rotation: Some(next.rotation), ..Default::default() })
}
//#endregion 🔺️Diff

//#region ↩️Inverse
/// ↩️ Per frame the turn moves, the absolute origin and rotation setters restoring the BASE pose.
pub fn inverse_rotate_frames(payload: &RotateFrames, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) || layout_frame_targets_invariant(&payload.targets).is_err() {
        return Ok(Vec::new());
    }
    Ok(layout_frame_selection_targets(base, &payload.page_id, &payload.targets)
        .into_iter()
        .filter(|frame| payload.turned(frame.bounds()) != *frame.bounds())
        .flat_map(|frame| {
            let bounds = frame.bounds();
            [
                LayoutMutation::MoveFrame(MoveFrame { page_id: payload.page_id.clone(), frame_id: frame.id().to_string(), new_x: bounds.x, new_y: bounds.y }),
                LayoutMutation::RotateFrame(RotateFrame { page_id: payload.page_id.clone(), frame_id: frame.id().to_string(), new_rotation: bounds.rotation }),
            ]
        })
        .collect())
}
//#endregion ↩️Inverse
