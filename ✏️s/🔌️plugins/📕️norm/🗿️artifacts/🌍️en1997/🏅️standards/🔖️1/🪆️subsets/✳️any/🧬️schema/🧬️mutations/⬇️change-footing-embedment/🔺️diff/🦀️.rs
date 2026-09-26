use super::ChangeFootingEmbedment;
use crate::diff::{En1997Diff, En1997FootingList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeFootingEmbedment, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_embedment.is_finite() || payload.new_embedment < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "embedment must be non-negative", vec![payload.id.clone()]);
    }
    let Some(idx) = base.footings.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.id), vec![payload.id.clone()]);
    };
    let mut footings = base.footings.clone();
    footings[idx].embedment = payload.new_embedment;
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingList { values: footings }), ..Default::default() })
}
