//! ↩️ `update-camera` inverse — self-inverse: the pre-state camera captured from `base` (the
//! fixture always has a camera, so this never has an empty-vec case).

use crate::artifacts::procedural3d::mutations::update_camera::mutation::UpdateCamera;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

pub fn inverse(_payload: &UpdateCamera, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    vec![Procedural3dMutation::UpdateCamera(UpdateCamera { camera: base.fixture.camera.clone() })]
}
