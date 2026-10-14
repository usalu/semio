//! 🔺️ `change-strict-mode` — sparse diff construction.

use crate::change_strict_mode::ChangeStrictMode;
use semio_s_artifact_norm_vdi3805_diff::Vdi3805Diff;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeStrictMode, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.strict_mode == payload.new_strict_mode {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Strict mode already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { strict_mode: Some(payload.new_strict_mode), ..Default::default() })
}
//#endregion 🔖️Diff
