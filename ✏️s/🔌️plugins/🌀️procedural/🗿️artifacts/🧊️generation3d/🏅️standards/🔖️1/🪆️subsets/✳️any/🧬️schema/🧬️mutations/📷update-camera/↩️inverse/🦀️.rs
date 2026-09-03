//! ↩️ `update-camera` inverse — self-inverse: the pre-state camera captured from `base` (the
//! fixture always has a camera, so this never has an empty-vec case).

use crate::artifacts::generation3d::mutations::update_camera::UpdateCamera;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;

pub fn inverse(_payload: &UpdateCamera, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::UpdateCamera(UpdateCamera { camera: base.fixture.camera.clone() })]
}
