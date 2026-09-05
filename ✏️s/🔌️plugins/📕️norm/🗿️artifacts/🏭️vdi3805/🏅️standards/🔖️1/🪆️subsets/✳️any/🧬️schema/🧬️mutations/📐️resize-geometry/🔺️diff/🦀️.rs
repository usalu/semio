//! 🔺️ `resize-geometry` — sparse diff construction; missing id is `mutation.target-missing`, a
//! non-finite or inverted (max < min) extent is `mutation.invariant`.

use super::ResizeGeometry;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ResizeGeometry, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let b = payload.new_bbox;
    if ![b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z].into_iter().all(f64::is_finite) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bounding box values must be finite numbers.", [payload.id.clone()]);
    }
    if b.max_x < b.min_x || b.max_y < b.min_y || b.max_z < b.min_z {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Bounding box max must not be less than min on any axis.", [payload.id.clone()]);
    }
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.bbox == payload.new_bbox {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Geometry \"{}\" already has this bounding box.", payload.id));
    }
    let mut geometry = base.geometry.clone();
    if let Some(entry) = geometry.get_mut(&payload.id) {
        entry.bbox = payload.new_bbox;
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(geometry), ..Default::default() })
}
//#endregion 🔖️Diff
