//! ↩️ `set-vertex-positions` — undo writes the base position of every named vertex that moves back as ONE `set-vertex-positions`,
//! read off `base` and the payload; a refused payload or one that moves nothing inverts to nothing.

use super::SetVertexPositions;
use crate::diff::LowpolyVertexPosition;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &SetVertexPositions, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    let Some(state) = payload.invariant_violation().is_none().then(|| base.objects.iter().find(|object| object.id == payload.object_id)).flatten().and_then(|object| object.mesh.as_ref().and(object.mesh_state.as_ref())) else {
        return Ok(Vec::new());
    };
    let mut positions: Vec<LowpolyVertexPosition> = payload
        .positions
        .iter()
        .filter_map(|row| state.vertices.get(row.vertex as usize).filter(|vertex| vertex.position.map(f32::to_bits) != row.position.map(f32::to_bits)).map(|vertex| LowpolyVertexPosition { vertex: row.vertex, position: vertex.position }))
        .collect();
    positions.sort_by_key(|row| row.vertex);
    let mut channels: Vec<crate::LowpolyMeshAttribute> = payload.channels.iter().filter_map(|channel| state.attributes.iter().find(|held| held.name == channel.name && *held != channel)).cloned().collect();
    channels.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(if positions.is_empty() && channels.is_empty() { Vec::new() } else { vec![LowpolyMutation::SetVertexPositions(SetVertexPositions { object_id: payload.object_id.clone(), positions, channels })] })
}
//#endregion 🔖️Inverse
