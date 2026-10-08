//! 🔺️ `add-geometry` — sparse diff construction.

use super::AddGeometry;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805GeometryRows, Vdi3805GeometryEntry};

//#region 🔖️Diff
/// 🔺️ A duplicate id is `mutation.duplicate-id` — an id-keyed entity that already exists cannot be
/// "created" again.

pub fn diff(payload: &AddGeometry, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.geometry.contains_key(&payload.geometry.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A geometry with id \"{}\" already exists.", payload.geometry.id), [payload.geometry.id.clone()]);
    }
    protocol::MutationOutcome::new(Vdi3805Diff { geometry: Some(Vdi3805GeometryRows { added: vec![Vdi3805GeometryEntry { key: payload.geometry.id.clone(), value: payload.geometry.clone() }], ..Default::default() }), ..Default::default() })
}
