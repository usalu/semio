//! ✋️ `drag-frames` — a relative, parametric drag of frames of one page by one common offset. The gesture's own inputs
//! (which page, which frames, which offset) are the payload, so editing the drag in history re-derives every position
//! from whatever base it replays on.

use crate::mutations::{layout_frame_selection_diff, layout_frame_selection_targets, layout_frame_targets_invariant, layout_label_frames, layout_label_number, move_frame::MoveFrame, LayoutMutation};
use crate::{FramePatch, LayoutBounds, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ✋️DragFrames
/// ✋️ `drag-frames` payload — the page, the frames it moves (literal ids) and the offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

impl DragFrames {
    /// ✋️ The bounds a frame holds after the drag: the origin moves by the offset, the extent and rotation stay.
    pub fn moved(&self, bounds: &LayoutBounds) -> LayoutBounds {
        LayoutBounds { x: bounds.x + self.dx, y: bounds.y + self.dy, ..bounds.clone() }
    }
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DragFrames {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "drag", entity: "frames", kind: "drag-frames", record: "DraggedFrames" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_drag_frames(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
        inverse_drag_frames(self, base)
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
    layout_frame_selection_diff(base, &payload.page_id, &payload.targets, |bounds| payload.moved(bounds), |next| FramePatch { x: Some(next.x), y: Some(next.y), ..Default::default() })
}
//#endregion 🔺️Diff

//#region ↩️Inverse
/// ↩️ One absolute origin setter per frame the drag moves, restoring the BASE origin.
pub fn inverse_drag_frames(payload: &DragFrames, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) || layout_frame_targets_invariant(&payload.targets).is_err() {
        return Ok(Vec::new());
    }
    Ok(layout_frame_selection_targets(base, &payload.page_id, &payload.targets)
        .into_iter()
        .filter(|frame| payload.moved(frame.bounds()) != *frame.bounds())
        .map(|frame| LayoutMutation::MoveFrame(MoveFrame { page_id: payload.page_id.clone(), frame_id: frame.id().to_string(), new_x: frame.bounds().x, new_y: frame.bounds().y }))
        .collect())
}
//#endregion ↩️Inverse
