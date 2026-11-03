//! ↩️ Inverse for `change-beam-bridge-lanes`.
use crate::change_beam_bridge_lanes::ChangeBeamBridgeLanes;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeBeamBridgeLanes, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beam_bridge_lanes::ChangeBeamBridgeLanes> {
    let Some(index) = base.beams.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(beam) = base.beams.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeBeamBridgeLanes { id: payload.id.clone(), new_bridge_lanes: beam.bridge_lanes })])
}
