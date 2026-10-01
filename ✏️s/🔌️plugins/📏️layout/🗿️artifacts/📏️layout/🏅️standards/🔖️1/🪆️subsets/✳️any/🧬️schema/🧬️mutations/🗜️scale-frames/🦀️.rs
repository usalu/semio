//! 🗜️ `scale-frames` — a relative, parametric scaling of frames of one page about one recorded pivot: every frame centre
//! moves away from the pivot by the per-axis factors and every frame's extent grows by them, so editing the scaling in
//! history re-derives every frame from whatever base it replays on. Extents scale along each frame's own axes.

use crate::mutations::{layout_frame_centre, layout_frame_selection_diff, layout_frame_selection_inverse, layout_label_frames, layout_label_number, LayoutMutation};
use crate::{FramePatch, LayoutBounds, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🗜️ScaleFrames
/// 🗜️ `scale-frames` payload — the page, the frames it scales (literal ids), the pivot and the positive factor per axis.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ScaleFrames {
    pub page_id: String,
    pub targets: Vec<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub sx: f64,
    pub sy: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ScaleFrames {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "scale", entity: "frames", kind: "scale-frames", record: "ScaledFrames" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_scale_frames(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        layout_frame_selection_inverse(base, &self.page_id, diff_scale_frames(self, base))
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let ((sx_en, sx_de), (sy_en, sy_de)) = (layout_label_number(self.sx), layout_label_number(self.sy));
        let (en, de) = layout_label_frames(self.targets.len());
        protocol::LocalizedLabel::native(&format!("Scale {en} by ({sx_en}, {sy_en})"), &format!("{de} um ({sx_de}; {sy_de}) skalieren"))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone()]
    }
}
//#endregion 🗜️ScaleFrames

//#region 🔺️Diff
/// 🔺️ Every unlocked target frame's BASE centre moves away from the pivot by the factors and its extent grows by them.
pub fn diff_scale_frames(payload: &ScaleFrames, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.sx.is_finite() && payload.sy.is_finite() && payload.sx > 0.0 && payload.sy > 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A scaling's pivot must be finite and its factors finite and positive.", payload.targets.clone());
    }
    let transform = |bounds: &LayoutBounds| {
        let (centre_x, centre_y) = layout_frame_centre(bounds);
        let (width, height) = (bounds.width * payload.sx, bounds.height * payload.sy);
        let (scaled_x, scaled_y) = (payload.pivot_x + (centre_x - payload.pivot_x) * payload.sx, payload.pivot_y + (centre_y - payload.pivot_y) * payload.sy);
        LayoutBounds { x: scaled_x - width * 0.5, y: scaled_y - height * 0.5, width, height, ..bounds.clone() }
    };
    layout_frame_selection_diff(base, &payload.page_id, &payload.targets, transform, |next| FramePatch { x: Some(next.x), y: Some(next.y), width: Some(next.width), height: Some(next.height), ..Default::default() })
}
//#endregion 🔺️Diff
