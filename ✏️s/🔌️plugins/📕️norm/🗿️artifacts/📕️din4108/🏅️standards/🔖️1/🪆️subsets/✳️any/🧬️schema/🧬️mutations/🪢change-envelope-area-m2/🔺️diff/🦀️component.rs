//! 🔺️ `change-envelope-area-m2` — sparse diff construction.

use super::mutation::ChangeEnvelopeAreaM2;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnvelopeAreaM2, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_envelope_area_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Envelope area m2 must be a finite number.", Vec::<String>::new());
    }
    if base.envelope_area_m2 == payload.new_envelope_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Envelope area m2 already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { envelope_area_m2: Some(payload.new_envelope_area_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
