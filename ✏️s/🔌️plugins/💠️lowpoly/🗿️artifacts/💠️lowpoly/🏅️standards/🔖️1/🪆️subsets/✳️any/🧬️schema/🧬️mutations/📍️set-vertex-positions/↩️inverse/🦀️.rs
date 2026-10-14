//! ↩️ `set-vertex-positions` — undo writes the base position of every named vertex that moves back as ONE `set-vertex-positions`,
//! read off `base` and the payload; a refused payload or one that moves nothing inverts to nothing.
use semio_s_artifact_lowpoly_lowpoly_model_diff::{LowpolySelectionMotion, lowpoly_selection_moves, lowpoly_selection_normal_channels};

use super::SetVertexPositions;
use semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyVertexPosition;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::set_vertex_positions::SetVertexPositions>>(payload: &SetVertexPositions, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Some(state) = payload.invariant_violation().is_none().then(|| base.objects.iter().find(|object| object.id == payload.object_id)).flatten().and_then(|object| object.mesh.as_ref().and(object.mesh_state.as_ref())) else {
        return Ok(Vec::new());
    };
    let mut positions: Vec<LowpolyVertexPosition> = payload
        .positions
        .iter()
        .filter_map(|row| state.vertices.get(row.vertex as usize).filter(|vertex| vertex.position.map(f32::to_bits) != row.position.map(f32::to_bits)).map(|vertex| LowpolyVertexPosition { vertex: row.vertex, position: vertex.position }))
        .collect();
    positions.sort_by_key(|row| row.vertex);
    let mut channels: Vec<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute> = payload.channels.iter().filter_map(|channel| state.attributes.iter().find(|held| held.name == channel.name && *held != channel)).cloned().collect();
    channels.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(if positions.is_empty() && channels.is_empty() { Vec::new() } else { vec![Op::from(SetVertexPositions { object_id: payload.object_id.clone(), positions, channels })] })
}
//#endregion 🔖️Inverse

//#region 🧲️SelectionMotion
/// ↩️ The exact undo of an invariant-clean selection motion, read off `base` alone: ONE `set-vertex-positions` writing back the
/// base position of every vertex the motion moves (point-invertible however many vertices moved, never a negated motion that
/// would accumulate float error); nothing when the motion is refused or moves no vertex.
pub fn lowpoly_selection_motion_inverse<Op: From<crate::set_vertex_positions::SetVertexPositions>>(base: &LowpolySnapshot, object_id: &str, vertex_ids: &[u32], motion: &LowpolySelectionMotion) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Some(state) = base.objects.iter().find(|object| object.id == object_id).and_then(|object| object.mesh.as_ref().and(object.mesh_state.as_ref())) else {
        return Ok(Vec::new());
    };
    let count = u32::try_from(state.vertices.len()).unwrap_or(u32::MAX);
    let present: Vec<u32> = if vertex_ids.is_empty() { (0..count).collect() } else { vertex_ids.iter().copied().filter(|id| *id < count).collect() };
    let positions: Vec<LowpolyVertexPosition> = lowpoly_selection_moves(state, &present, motion).unwrap_or_default().into_iter().map(|(vertex, _, position)| LowpolyVertexPosition { vertex, position }).collect();
    let channels: Vec<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshAttribute> = lowpoly_selection_normal_channels(state, &present, motion).unwrap_or_default().iter().filter_map(|rewritten| state.attributes.iter().find(|held| held.name == rewritten.name).cloned()).collect();
    Ok(if positions.is_empty() && channels.is_empty() { Vec::new() } else { vec![Op::from(crate::set_vertex_positions::SetVertexPositions { object_id: object_id.to_string(), positions, channels })] })
}
//#endregion 🧲️SelectionMotion
