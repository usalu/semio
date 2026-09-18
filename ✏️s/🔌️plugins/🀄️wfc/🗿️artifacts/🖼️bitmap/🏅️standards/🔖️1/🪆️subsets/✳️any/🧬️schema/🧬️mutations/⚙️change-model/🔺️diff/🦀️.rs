//! 🔺️ Sparse diff builder for `ChangeModel` — one model lane write, guarded against the parameter
//! ranges the extraction step can actually honour.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::{BitmapOverlappingModel, BitmapSnapshot, BITMAP_PATTERN_SIZE_RANGE, BITMAP_SYMMETRY_RANGE};

pub fn diff(payload: &super::ChangeModel, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.pattern_size < BITMAP_PATTERN_SIZE_RANGE.0 || payload.pattern_size > BITMAP_PATTERN_SIZE_RANGE.1 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("The pattern size must be between {} and {}.", BITMAP_PATTERN_SIZE_RANGE.0, BITMAP_PATTERN_SIZE_RANGE.1), ["model".to_string()]);
    }
    if payload.symmetry < BITMAP_SYMMETRY_RANGE.0 || payload.symmetry > BITMAP_SYMMETRY_RANGE.1 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("The symmetry count must be between {} and {}.", BITMAP_SYMMETRY_RANGE.0, BITMAP_SYMMETRY_RANGE.1), ["model".to_string()]);
    }
    if let Some(ground) = payload.ground {
        if ground as usize >= base.input.palette.len() {
            return protocol::MutationOutcome::fatal("mutation.unknown-palette-color", format!("Ground colour {ground} is not in this document's palette."), ["model".to_string()]);
        }
    }
    let model = BitmapOverlappingModel { pattern_size: payload.pattern_size, symmetry: payload.symmetry, periodic_input: payload.periodic_input, ground: payload.ground };
    if base.model == model {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The model already holds those parameters.".to_string());
    }
    protocol::MutationOutcome::new(BitmapDiff { model: Some(model), ..Default::default() })
}
