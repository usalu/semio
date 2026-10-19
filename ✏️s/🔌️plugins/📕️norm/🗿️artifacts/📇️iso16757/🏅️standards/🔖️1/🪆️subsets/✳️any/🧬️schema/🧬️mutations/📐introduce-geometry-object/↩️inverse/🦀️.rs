//! ↪️ Inverse for `introduce-geometry-object`.

use crate::retire_geometry_object;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::introduce_geometry_object::mutation::IntroduceGeometryObject;

pub fn inverse<Op>(payload: &IntroduceGeometryObject, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::retire_geometry_object::mutation::RetireGeometryObject> {
    Ok((|| {
    if base.geometry.objects.contains_key(&payload.geometry_object.id) {
        return Vec::new();
    }
    vec![Op::from(retire_geometry_object::mutation::RetireGeometryObject {
        id: payload.geometry_object.id.clone(),
    })]

    })())
}
