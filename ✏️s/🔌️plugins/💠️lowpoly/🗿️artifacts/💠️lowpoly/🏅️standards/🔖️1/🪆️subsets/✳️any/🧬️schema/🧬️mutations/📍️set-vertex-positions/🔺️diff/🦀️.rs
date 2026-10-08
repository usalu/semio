//! 🔺️ `set-vertex-positions` — sparse diff construction: Fatal `invariant` for the payload's own breach (a vertex named twice or a
//! non-finite position), Error `target-missing` for an absent object, mesh or every named vertex, Warning `partial` for the named
//! vertices the mesh does not hold and Warning `no-op` when every named vertex already sits there. One position row per vertex that
//! actually moves, read against the base positions, and one absolute channel row per named Normal channel whose content changes.

use super::SetVertexPositions;
use crate::diff::LowpolyVertexPosition;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &SetVertexPositions, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let object_id = payload.object_id.as_str();
    if let Some(reason) = payload.invariant_violation() {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, [object_id.to_string()]);
    }
    let Some(object) = base.objects.iter().find(|object| object.id == object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{object_id}\" does not exist."), [object_id.to_string()]);
    };
    let Some(state) = object.mesh.as_ref().and(object.mesh_state.as_ref()) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{object_id}\" carries no mesh."), [object_id.to_string()]);
    };
    let (present, skipped): (Vec<&LowpolyVertexPosition>, Vec<&LowpolyVertexPosition>) = payload.positions.iter().partition(|row| (row.vertex as usize) < state.vertices.len());
    if present.is_empty() && payload.channels.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} named vertices exists on object \"{object_id}\".", payload.positions.len()), [object_id.to_string()]);
    }
    let partial = (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} named vertices skipped (not on object \"{object_id}\").", skipped.len(), payload.positions.len())).at([object_id.to_string()]));
    let mut rows: Vec<LowpolyVertexPosition> = present.into_iter().filter(|row| row.position.map(f32::to_bits) != state.vertices[row.vertex as usize].position.map(f32::to_bits)).cloned().collect();
    rows.sort_by_key(|row| row.vertex);
    let mut channels: Vec<crate::LowpolyMeshAttribute> = payload.channels.iter().filter(|channel| state.attributes.iter().any(|held| held.name == channel.name && held != *channel)).cloned().collect();
    channels.sort_by(|left, right| left.name.cmp(&right.name));
    if rows.is_empty() && channels.is_empty() {
        let no_op = protocol::MutationMessage::warning("mutation.no-op", format!("Every named vertex of object \"{object_id}\" already sits there.")).at([object_id.to_string()]);
        return protocol::MutationOutcome::new(LowpolyDiff::default()).absorb_messages(partial.into_iter().chain([no_op]));
    }
    protocol::MutationOutcome::new(crate::diff::diff_mesh_vertices(object_id.to_string(), rows, channels)).absorb_messages(partial)
}
//#endregion 🔖️Diff
