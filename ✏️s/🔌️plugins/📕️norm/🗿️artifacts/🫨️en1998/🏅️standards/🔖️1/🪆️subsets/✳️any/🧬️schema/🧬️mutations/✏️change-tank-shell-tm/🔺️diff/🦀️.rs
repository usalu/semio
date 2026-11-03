//! ✏️ `change-tank-shell-tm` diff: patches `shell_thickness_m` of the addressed tank row.

use crate::change_tank_shell_t_m::ChangeTankShellTM;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TankDelta, En1998TankPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeTankShellTM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_shell_thickness_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.tanks.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tank {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.shell_thickness_m == payload.new_shell_thickness_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tank already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { tanks: En1998TankDelta::modification(&payload.id, En1998TankPatch { shell_thickness_m: Some(payload.new_shell_thickness_m), ..Default::default() }), ..Default::default() })
}
