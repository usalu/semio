//! Diff for `introduce-geometry-object`.

use super::mutation::IntroduceGeometryObject;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757GeometryObjectsRows, Iso16757GeometryObjectsEntry};

pub fn diff(payload: &IntroduceGeometryObject, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.geometry.objects.contains_key(&payload.geometry_object.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("Geometry \"{}\" already exists.", payload.geometry_object.id),
            [payload.geometry_object.id.clone()],
        );
    }
    protocol::MutationOutcome::new(Iso16757Diff { geometry_objects: Some(Iso16757GeometryObjectsRows { added: vec![Iso16757GeometryObjectsEntry { key: payload.geometry_object.id.clone(), value: payload.geometry_object.clone() }], ..Default::default() }), ..Default::default() })
}
