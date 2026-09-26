//! Inverse for `retire-geometry-object`.

use crate::mutations::introduce_geometry_object;
use crate::{Iso16757Mutation, Iso16757Snapshot};
use super::mutation::RetireGeometryObject;

pub fn inverse(payload: &RetireGeometryObject, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    let Some(obj) = base.geometry.objects.get(&payload.id) else {
        return Vec::new();
    };
    vec![Iso16757Mutation::IntroduceGeometryObject(introduce_geometry_object::mutation::IntroduceGeometryObject {
        geometry_object: obj.clone(),
    })]
}
