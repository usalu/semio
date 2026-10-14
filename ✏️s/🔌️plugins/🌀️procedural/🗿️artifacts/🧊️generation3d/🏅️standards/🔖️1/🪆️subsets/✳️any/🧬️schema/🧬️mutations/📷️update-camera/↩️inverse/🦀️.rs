//! ↩️ `update-camera` inverse — self-inverse: the pre-state camera captured from `base` (the
//! fixture always has a camera, so this never has an empty-vec case).

use crate::update_camera::UpdateCamera;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

pub fn inverse<Op: From<UpdateCamera>>(_payload: &UpdateCamera, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Op::from(UpdateCamera { camera: base.host_snapshot.camera.clone() })]

    })())
}
