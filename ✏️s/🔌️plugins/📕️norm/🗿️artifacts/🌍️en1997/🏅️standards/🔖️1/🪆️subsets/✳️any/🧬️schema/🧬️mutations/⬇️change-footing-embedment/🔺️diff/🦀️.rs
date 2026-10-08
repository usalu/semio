use super::ChangeFootingEmbedment;
use crate::En1997Snapshot;
use crate::diff::{En1997Diff, En1997FootingsRows, En1997FootingsPatch};

pub fn diff(payload: &ChangeFootingEmbedment, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_embedment.is_finite() || payload.new_embedment < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "embedment must be non-negative", vec![payload.id.clone()]);
    }
    let Some(idx) = base.footings.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1997Diff {
        footings: Some(En1997FootingsRows { modified: vec![En1997FootingsPatch { id: payload.id.clone(), embedment: Some(payload.new_embedment), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
