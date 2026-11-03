//! 🔺️ Diff for `change-slab-support`.
use crate::change_slab_support::ChangeSlabSupport;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994SlabsRows, En1994SlabsPatch};

pub fn diff(payload: &ChangeSlabSupport, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.slabs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("slab {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(slab) = base.slabs.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "slab missing", [index.to_string()]);
    };
    if slab.support == payload.new_support {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        slabs: Some(En1994SlabsRows::modification(&payload.id, En1994SlabsPatch { support: Some(payload.new_support), ..Default::default() })),
        ..Default::default()
    })
}
