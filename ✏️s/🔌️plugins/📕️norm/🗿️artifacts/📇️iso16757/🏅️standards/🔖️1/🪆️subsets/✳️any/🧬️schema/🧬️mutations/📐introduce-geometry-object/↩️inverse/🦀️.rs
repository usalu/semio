//! Inverse for `introduce-geometry-object`.

use crate::mutations::retire_geometry_object;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::IntroduceGeometryObject;

pub fn inverse(payload: &IntroduceGeometryObject, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    if base.geometry.objects.contains_key(&payload.geometry_object.id) {
        return Vec::new();
    }
    vec![Iso16757Mutation::RetireGeometryObject(retire_geometry_object::mutation::RetireGeometryObject {
        id: payload.geometry_object.id.clone(),
    })]
}
