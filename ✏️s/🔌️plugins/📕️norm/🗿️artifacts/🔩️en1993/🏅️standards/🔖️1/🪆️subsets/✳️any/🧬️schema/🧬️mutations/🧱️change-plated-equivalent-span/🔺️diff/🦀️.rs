//! 🔺️ `change-plated-equivalent-span` diff: patches `equivalent_span` of the addressed plated panel row.

use crate::change_plated_equivalent_span::ChangePlatedEquivalentSpan;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PlatedPanelDelta, En1993PlatedPanelPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangePlatedEquivalentSpan, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_equivalent_span.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.plated_panels.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "plated", Vec::<String>::new());
    };
    if row.equivalent_span == payload.new_equivalent_span {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The plated panel already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { plated_panels: En1993PlatedPanelDelta::modification(payload.id.clone(), En1993PlatedPanelPatch { equivalent_span: Some(payload.new_equivalent_span), ..Default::default() }), ..Default::default() })
}
