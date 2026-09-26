use super::ChangeFootingWidth;
use crate::diff::{En1997Diff, En1997FootingList};
use crate::En1997Snapshot;
pub fn diff(payload: &ChangeFootingWidth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_width.is_finite() || payload.new_width <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "footing width must be positive", vec![payload.id.clone()]);
    }
    let Some(idx) = base.footings.iter().position(|f| f.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("footing {} missing", payload.id), vec![payload.id.clone()]);
    };
    if base.footings[idx].width == payload.new_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "width unchanged");
    }
    let mut footings = base.footings.clone();
    footings[idx].width = payload.new_width;
    protocol::MutationOutcome::new(En1997Diff { footings: Some(En1997FootingList { values: footings }), ..Default::default() })
}
