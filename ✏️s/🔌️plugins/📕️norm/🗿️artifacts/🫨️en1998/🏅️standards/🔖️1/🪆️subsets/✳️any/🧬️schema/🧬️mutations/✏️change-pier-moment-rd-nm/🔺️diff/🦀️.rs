//! ✏️ `change-pier-moment-rd-nm` diff: patches `moment_rd_nm` of the addressed pier row.

use crate::change_pier_moment_rd_nm::ChangePierMomentRdNm;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierDelta, En1998PierPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangePierMomentRdNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_moment_rd_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.moment_rd_nm == payload.new_moment_rd_nm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pier already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { piers: En1998PierDelta::modification(&payload.id, En1998PierPatch { moment_rd_nm: Some(payload.new_moment_rd_nm), ..Default::default() }), ..Default::default() })
}
