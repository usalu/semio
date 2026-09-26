//! Diff for `introduce-geometry-object`.

use super::mutation::IntroduceGeometryObject;
use crate::{Iso16757Diff, Iso16757Snapshot};

pub fn diff(payload: &IntroduceGeometryObject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.geometry.objects.contains_key(&payload.geometry_object.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("Geometry \"{}\" already exists.", payload.geometry_object.id),
            [payload.geometry_object.id.clone()],
        );
    }
    let mut geometry = base.geometry.clone();
    geometry.objects.insert(payload.geometry_object.id.clone(), payload.geometry_object.clone());
    protocol::MutationOutcome::new(Iso16757Diff { geometry: Some(geometry), ..Default::default() })
}
