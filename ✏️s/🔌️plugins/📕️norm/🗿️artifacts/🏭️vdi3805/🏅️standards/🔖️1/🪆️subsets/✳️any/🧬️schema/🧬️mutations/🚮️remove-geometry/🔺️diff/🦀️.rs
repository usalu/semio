//! 🔺️ `remove-geometry` — sparse diff construction.

use super::RemoveGeometry;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805GeometryRows};

//#region 🔖️Diff

pub fn diff(payload: &RemoveGeometry, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if !base.geometry.contains_key(&payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(Vdi3805GeometryRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
