//! 🔺️ `change-ring-shell-id` diff: patches `shell_id` of the addressed ring stiffener row.

use crate::change_ring_shell_id::ChangeRingShellId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999RingStiffenersPatch, En1999RingStiffenersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeRingShellId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "ring-stiffener", Vec::<String>::new());
    };
    if r0.shell_id == payload.new_shell_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The ring stiffener already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { ring_stiffeners: Some(En1999RingStiffenersRows::modification(&payload.ring_stiffener_id, En1999RingStiffenersPatch { shell_id: Some(payload.new_shell_id.clone()), ..Default::default() })), ..Default::default() })
}
