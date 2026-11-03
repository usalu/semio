//! ↩️ Inverse for `change-beam-stud-edge-distance-m`.
use crate::change_beam_stud_edge_distance_m::ChangeBeamStudEdgeDistanceM;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamStudEdgeDistanceM, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_stud_edge_distance_m::ChangeBeamStudEdgeDistanceM> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamStudEdgeDistanceM { id: payload.id.clone(), new_stud_edge_distance_m: beam.stud_edge_distance_m })])
}
