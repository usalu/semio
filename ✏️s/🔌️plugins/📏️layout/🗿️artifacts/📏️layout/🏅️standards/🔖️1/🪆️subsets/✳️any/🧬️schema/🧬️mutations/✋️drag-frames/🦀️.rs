//! ✋️ `drag-frames` — a relative, parametric drag of frames of one page by one common offset. The gesture's own inputs
//! (which page, which frames, which offset) are the payload, so editing the drag in history re-derives every position
//! from whatever base it replays on.

use crate::mutations::{layout_frame_selection_diff, layout_frame_selection_inverse, layout_label_frames, layout_label_number, LayoutMutation};
use crate::{FramePatch, LayoutBounds, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ✋️DragFrames
/// ✋️ `drag-frames` payload — the page, the frames it moves (literal ids) and the offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DragFrames {
    pub page_id: String,
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DragFrames {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "frames", kind: "drag-frames", record: "DraggedFrames" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_drag_frames(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
        layout_frame_selection_inverse(base, &self.page_id, diff_drag_frames(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (layout_label_number(self.dx), layout_label_number(self.dy));
        let (en, de) = layout_label_frames(self.targets.len());
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {en} by ({dx_en}, {dy_en})"), &format!("{de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone()]
    }
}
//#endregion ✋️DragFrames

//#region 🔺️Diff
/// 🔺️ Every unlocked target frame's origin moves by the offset, read off the BASE bounds.
pub fn diff_drag_frames(payload: &DragFrames, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A drag offset must be finite.", payload.targets.clone());
    }
    let transform = |bounds: &LayoutBounds| LayoutBounds { x: bounds.x + payload.dx, y: bounds.y + payload.dy, ..bounds.clone() };
    layout_frame_selection_diff(base, &payload.page_id, &payload.targets, transform, |next| FramePatch { x: Some(next.x), y: Some(next.y), ..Default::default() })
}
//#endregion 🔺️Diff
