//! Inverse for `change-beam-span-m`.
use super::ChangeBeamSpanM;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(payload: &ChangeBeamSpanM, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    let Some(beam) = base.beams.get(payload.index) else { return Vec::new(); };
    vec![En1994Mutation::ChangeBeamSpanM(ChangeBeamSpanM { index: payload.index, new_span_m: beam.span_m })]
}
