//! 🔃️ `rotate-frames` — a relative, parametric turn of frames of one page about one recorded pivot: every frame centre
//! orbits the pivot by the angle and every frame's rotation grows by it, so editing the turn in history re-derives every
//! pose from whatever base it replays on.

use crate::mutations::{layout_frame_centre, layout_frame_selection_diff, layout_frame_selection_inverse, layout_label_frames, layout_label_number, LayoutMutation};
use crate::{FramePatch, LayoutBounds, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔃️RotateFrames
/// 🔃️ `rotate-frames` payload — the page, the frames it turns (literal ids), the pivot and the counter-clockwise angle
/// in radians.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
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

impl MutationKind<LayoutSnapshot, LayoutMutation> for RotateFrames {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rotate", entity: "frames", kind: "rotate-frames", record: "RotatedFrames" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rotate_frames(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        layout_frame_selection_inverse(base, &self.page_id, diff_rotate_frames(self, base))
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
    let (sin, cos) = (payload.angle.sin(), payload.angle.cos());
    let transform = |bounds: &LayoutBounds| {
        let (centre_x, centre_y) = layout_frame_centre(bounds);
        let (offset_x, offset_y) = (centre_x - payload.pivot_x, centre_y - payload.pivot_y);
        let (turned_x, turned_y) = (payload.pivot_x + offset_x * cos - offset_y * sin, payload.pivot_y + offset_x * sin + offset_y * cos);
        LayoutBounds { x: turned_x - bounds.width * 0.5, y: turned_y - bounds.height * 0.5, rotation: bounds.rotation + payload.angle, ..bounds.clone() }
    };
    layout_frame_selection_diff(base, &payload.page_id, &payload.targets, transform, |next| FramePatch { x: Some(next.x), y: Some(next.y), rotation: Some(next.rotation), ..Default::default() })
}
//#endregion 🔺️Diff
