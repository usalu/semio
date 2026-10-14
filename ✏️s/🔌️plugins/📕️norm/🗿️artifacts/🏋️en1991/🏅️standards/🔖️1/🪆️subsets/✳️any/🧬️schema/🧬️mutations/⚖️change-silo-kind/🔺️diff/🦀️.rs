//! 🔺️ Diff for `change-silo-kind`.
use crate::change_silo_kind::ChangeSiloKind;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSiloKind, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.silo_kind == payload.new_silo_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_kind: Some(payload.new_silo_kind.clone()), ..Default::default() })
}
