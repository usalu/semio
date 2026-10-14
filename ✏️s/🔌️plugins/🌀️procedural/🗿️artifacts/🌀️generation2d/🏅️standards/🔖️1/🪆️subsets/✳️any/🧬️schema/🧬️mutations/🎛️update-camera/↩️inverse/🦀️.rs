//! ↩️ Inverse for `UpdateCamera` — restores the captured BASE camera. The camera always exists (it
//! is not optional on the fixture), so this is unconditional, unlike the id-keyed leaves above.

use crate::update_camera;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<UpdateCamera>>(_payload: &super::UpdateCamera, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![update_camera(base.host_snapshot.camera.clone())]

    })())
}
