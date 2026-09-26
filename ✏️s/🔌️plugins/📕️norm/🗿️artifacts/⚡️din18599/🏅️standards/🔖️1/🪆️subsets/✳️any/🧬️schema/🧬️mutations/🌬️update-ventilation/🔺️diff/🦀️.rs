//! 🔺️ `update-ventilation` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::update_ventilation::UpdateVentilation;
use crate::Din18599Snapshot;

pub fn diff(payload: &UpdateVentilation, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.ventilation == payload.new_ventilation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "ventilation already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { ventilation: Some(payload.new_ventilation.clone()), ..Default::default() })
}
