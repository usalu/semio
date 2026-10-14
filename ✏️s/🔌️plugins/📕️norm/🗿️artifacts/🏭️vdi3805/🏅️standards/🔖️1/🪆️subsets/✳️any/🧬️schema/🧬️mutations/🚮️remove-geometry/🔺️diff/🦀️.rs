//! 🔺️ `remove-geometry` — sparse diff construction.

use crate::remove_geometry::RemoveGeometry;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_s_artifact_norm_vdi3805_diff::{Vdi3805Diff, Vdi3805GeometryRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveGeometry, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.geometry.contains_key(&payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(Vdi3805GeometryRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
