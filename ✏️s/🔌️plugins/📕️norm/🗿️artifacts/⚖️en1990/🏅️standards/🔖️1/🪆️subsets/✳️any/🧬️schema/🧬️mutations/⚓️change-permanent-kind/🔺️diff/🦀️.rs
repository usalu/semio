//! ⚓️ `change-permanent-kind` diff: patches `kind` of the addressed permanent action row.

use crate::change_permanent_kind::ChangePermanentKind;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990PermanentDelta, En1990PermanentPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangePermanentKind, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.permanents.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Permanent action {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The permanent action already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { permanents: En1990PermanentDelta::modification(&payload.id, En1990PermanentPatch { kind: Some(payload.new_kind), ..Default::default() }), ..Default::default() })
}
