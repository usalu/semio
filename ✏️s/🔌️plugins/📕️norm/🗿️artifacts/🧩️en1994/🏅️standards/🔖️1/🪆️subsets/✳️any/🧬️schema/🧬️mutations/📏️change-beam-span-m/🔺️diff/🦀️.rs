//! 🔺️ Diff for `change-beam-span-m`.
use crate::change_beam_span_m::ChangeBeamSpanM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamSpanM, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    if !payload.new_span_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "must be finite", [payload.index.to_string()]);
    }
    let Some(beam) = base.beams.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [payload.index.to_string()]);
    };
    if beam.span_m == payload.new_span_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows { modified: vec![En1994BeamsPatch { index: payload.index, span_m: Some(payload.new_span_m), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
