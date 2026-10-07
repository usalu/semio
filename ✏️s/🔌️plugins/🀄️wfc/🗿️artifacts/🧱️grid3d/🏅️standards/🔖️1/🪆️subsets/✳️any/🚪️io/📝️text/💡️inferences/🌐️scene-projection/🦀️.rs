//! 🌐️ Native JSON encoding of owned grid scene projections.
use crate::schema::scene_internals::*;
use crate::schema::snapshot::Grid3dSnapshot;
use crate::schema::inferences::Grid3dAssignment;
pub fn grid_meshes_json(snapshot: &Grid3dSnapshot) -> String { semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&grid_meshes_value(snapshot))) }
pub fn grid_instances_json(snapshot: &Grid3dSnapshot) -> String { semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&grid_instances_value(snapshot))) }
pub fn preview_meshes_json(snapshot: &Grid3dSnapshot) -> String { semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&preview_meshes_value(snapshot))) }
pub fn preview_instances_json(snapshot: &Grid3dSnapshot, assignments: &[Grid3dAssignment]) -> String { semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&preview_instances_value(snapshot, assignments))) }
pub fn preview_instances_delta_json(snapshot: &Grid3dSnapshot, previous: &[Grid3dAssignment], next: &[Grid3dAssignment], revision: u64) -> String { semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&preview_instances_delta_value(snapshot, previous, next, revision))) }

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
