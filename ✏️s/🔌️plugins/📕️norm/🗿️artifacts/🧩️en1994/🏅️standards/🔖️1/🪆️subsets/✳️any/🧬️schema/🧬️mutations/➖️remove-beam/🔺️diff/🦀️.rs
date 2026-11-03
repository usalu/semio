//! 🔺️ Diff for `remove-beam`: the beam row and, in the same batch, the stud rows it owns through `beamId`.
use crate::remove_beam::RemoveBeam;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994StudsRows};

pub fn diff(payload: &RemoveBeam, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let owned = owned_studs(payload, base);
    let studs = (!owned.is_empty()).then(|| En1994StudsRows::removals(&base.studs, &owned));
    protocol::MutationOutcome::new(En1994Diff { beams: Some(En1994BeamsRows::removal(&base.beams, index)), studs, ..Default::default() })
}

/// 🪢 The positions of the stud rows of the beam, ascending.
pub fn owned_studs(payload: &RemoveBeam, base: &En1994Snapshot) -> Vec<usize> {
    base.studs.iter().enumerate().filter(|(_, stud)| stud.beam_id == payload.id).map(|(at, _)| at).collect()
}
