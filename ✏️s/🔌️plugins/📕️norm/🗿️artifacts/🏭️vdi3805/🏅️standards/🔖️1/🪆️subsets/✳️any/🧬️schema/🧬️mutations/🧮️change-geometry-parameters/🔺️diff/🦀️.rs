//! 🔺️ `change-geometry-parameters` — sparse diff construction; missing id is
//! `mutation.target-missing`.

use super::ChangeGeometryParameters;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805GeometryRows, Vdi3805GeometryPatch};

//#region 🔖️Diff

pub fn diff(payload: &ChangeGeometryParameters, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    let Some(entry) = base.geometry.get(&payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Geometry \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if entry.parameters == payload.new_parameters {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Geometry \"{}\" already has these parameters.", payload.id));
    }
    protocol::MutationOutcome::new(Vdi3805Diff {
        geometry: Some(Vdi3805GeometryRows { modified: vec![Vdi3805GeometryPatch { key: payload.id.clone(), parameters: Some(payload.new_parameters.clone()), ..Default::default() }], ..Default::default() }),
        ..Default::default()
    })
}
