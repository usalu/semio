//! 🔺️ `change-geg-qp-factor` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::change_geg_qp_factor::ChangeGegQpFactor;
use crate::Din18599Snapshot;

pub fn diff(payload: &ChangeGegQpFactor, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_geg_qp_factor.is_finite() || payload.new_geg_qp_factor <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "geg-qp-factor must be a positive finite number.", Vec::<String>::new());
    }
    if base.geg_qp_factor == payload.new_geg_qp_factor {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "geg-qp-factor already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { geg_qp_factor: Some(payload.new_geg_qp_factor), ..Default::default() })
}
