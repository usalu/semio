//! 🔺️ `change-rh-int` diff — sets the indoor relative humidity φ_i as a fraction, refused as a `mutation.invariant` outside the bound its leaf payload schema states.

use super::ChangeRhInt;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeRhInt, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !((0.0..=1.0).contains(&payload.new_rh_int)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "rh_int must be a relative humidity within [0, 1].", Vec::<String>::new());
    }
    if base.rh_int == payload.new_rh_int {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "rh_int already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { rh_int: Some(payload.new_rh_int), ..Default::default() })
}
