//! 🔺️ `change-shell-material-id` diff: patches `material_id` of the addressed shell row.

use crate::change_shell_material_id::ChangeShellMaterialId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ShellsPatch, En1999ShellsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeShellMaterialId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.shells.iter().find(|row| row.id == payload.shell_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "shell", Vec::<String>::new());
    };
    if r0.material_id == payload.new_material_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The shell already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { shells: Some(En1999ShellsRows::modification(&payload.shell_id, En1999ShellsPatch { material_id: Some(payload.new_material_id.clone()), ..Default::default() })), ..Default::default() })
}
