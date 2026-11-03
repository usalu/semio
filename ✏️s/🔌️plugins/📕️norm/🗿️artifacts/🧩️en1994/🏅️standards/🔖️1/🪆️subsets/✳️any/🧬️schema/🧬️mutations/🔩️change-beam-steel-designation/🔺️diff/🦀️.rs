//! 🔺️ Diff for `change-beam-steel-designation`.
use crate::change_beam_steel_designation::ChangeBeamSteelDesignation;
use semio_s_artifact_norm_en1994_model::{En1994Snapshot, SteelSection};
use semio_s_artifact_norm_en1994_diff::{En1994Diff, En1994BeamsRows, En1994BeamsPatch};

pub fn diff(payload: &ChangeBeamSteelDesignation, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("beam {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(beam) = base.beams.get(index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "beam missing", [index.to_string()]);
    };
    let next = SteelSection::from_catalogue(&payload.new_designation).unwrap_or_else(|| SteelSection { designation: payload.new_designation.clone(), ..beam.steel.clone() });
    if beam.steel == next {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(En1994Diff {
        beams: Some(En1994BeamsRows::modification(&payload.id, En1994BeamsPatch { steel: Some(next), ..Default::default() })),
        ..Default::default()
    })
}
