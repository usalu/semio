//! 🔺️ `change-envelope-n50` diff.
use super::ChangeEnvelopeN50;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeEnvelopeN50, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.envelope_n50_h_inv == payload.new_envelope_n50_h_inv {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { envelope_n50_h_inv: Some(payload.new_envelope_n50_h_inv), ..Default::default() })
}
