//! ↩️ `update-camera` inverse — self-inverse: the pre-state camera captured from `base` (the
//! fixture always has a camera, so this never has an empty-vec case).

use crate::standards::v1::subsets::any::schema::mutations::update_camera::UpdateCamera;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

pub fn inverse(_payload: &UpdateCamera, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::UpdateCamera(UpdateCamera { camera: base.fixture.camera.clone() })]
}
