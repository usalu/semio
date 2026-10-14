//! Inverse for `retire-geometry-object`.

use crate::introduce_geometry_object;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use crate::retire_geometry_object::mutation::RetireGeometryObject;

pub fn inverse<Op>(payload: &RetireGeometryObject, base: &Iso16757Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::introduce_geometry_object::mutation::IntroduceGeometryObject> {
    Ok((|| {
    let Some(obj) = base.geometry.objects.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(introduce_geometry_object::mutation::IntroduceGeometryObject {
        geometry_object: obj.clone(),
    })]

    })())
}
