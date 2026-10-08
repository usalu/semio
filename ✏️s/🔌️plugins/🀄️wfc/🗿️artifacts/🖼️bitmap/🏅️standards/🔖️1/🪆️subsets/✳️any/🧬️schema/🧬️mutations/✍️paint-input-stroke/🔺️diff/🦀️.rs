//! 🔺️ Sparse diff builder for `PaintInputStroke` — one region entry on the `inputRegions` lane over the
//! bounding box of the painted cells, read off the BASE buffer, so the stroke replays on any base: cells a
//! smaller sample no longer holds are skipped (`mutation.partial`), and a stroke with none left is an error.

use super::{stroke_cells, stroke_extent, BITMAP_STROKE_MAXIMUM_POINTS};
use crate::diff::{BitmapDiff, BitmapInputOp, BitmapPixelRegion};
use crate::schema::snapshot::{read_region, BitmapSnapshot};

pub fn diff(payload: &super::PaintInputStroke, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.points.is_empty() || payload.points.len() > BITMAP_STROKE_MAXIMUM_POINTS {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A stroke carries 1 to {BITMAP_STROKE_MAXIMUM_POINTS} points, not {}.", payload.points.len()), ["points".to_string()]);
    }
    if payload.color as usize >= base.input.palette.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Palette index {} is not in this document's palette.", payload.color), ["color".to_string()]);
    }
    let Some(buffer) = base.input.indices() else {
        return protocol::MutationOutcome::fatal("mutation.apply.invalid-base", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    };
    let Some(((x, y, width, height), inside)) = stroke_extent(&payload.points, base.input.width, base.input.height) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No stroke cell lies inside the {}×{} input sample.", base.input.width, base.input.height), ["points".to_string()]);
    };
    let Some(prior) = read_region(&buffer, base.input.width, base.input.height, x, y, width, height) else {
        return protocol::MutationOutcome::fatal("mutation.apply.invalid-base", "The stroke region does not read from the base buffer.".to_string(), ["input".to_string()]);
    };
    let mut region = prior.clone();
    for cell in &inside {
        region[((cell.y - y) * width + (cell.x - x)) as usize] = payload.color as u8;
    }
    if region == prior {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Every stroke cell already holds this colour.".to_string());
    }
    let outcome = protocol::MutationOutcome::new(BitmapDiff { input_ops: vec![BitmapInputOp::Region { region: BitmapPixelRegion { x, y, width, height, pixels: region.to_vec() } }], ..Default::default() });
    let total = stroke_cells(&payload.points).len();
    if inside.len() < total {
        return outcome.warning("mutation.partial", format!("{} of {total} stroke cells lie outside the {}×{} input sample and are skipped.", total - inside.len(), base.input.width, base.input.height));
    }
    outcome
}
