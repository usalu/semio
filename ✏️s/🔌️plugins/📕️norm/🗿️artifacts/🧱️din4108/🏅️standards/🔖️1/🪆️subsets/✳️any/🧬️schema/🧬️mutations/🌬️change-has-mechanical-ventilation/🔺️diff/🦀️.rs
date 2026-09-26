//! 🔺️ `change-has-mechanical-ventilation` diff.

use super::ChangeHasMechanicalVentilation;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeHasMechanicalVentilation, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.has_mechanical_ventilation == payload.new_has_mechanical_ventilation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "has_mechanical_ventilation already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { has_mechanical_ventilation: Some(payload.new_has_mechanical_ventilation), ..Default::default() })
}
