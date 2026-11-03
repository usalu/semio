//! ↩️ `change-ring-shell-id` inverse: restores the previous `shell_id`.

use crate::change_ring_shell_id::ChangeRingShellId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeRingShellId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ring_shell_id::ChangeRingShellId> {
    Ok(match base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) {
        Some(row) => vec![Op::from(ChangeRingShellId { ring_stiffener_id: payload.ring_stiffener_id.clone(), new_shell_id: row.shell_id.clone() })],
        None => Vec::new(),
    })
}
