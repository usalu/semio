//! ↩️ Inverse for `UpdateCamera` — restores the captured BASE camera. The camera always exists (it
//! is not optional on the fixture), so this is unconditional, unlike the id-keyed leaves above.

use crate::mutations::{update_camera, Generation2dMutation};
use crate::Generation2dSnapshot;

pub fn inverse(_payload: &super::UpdateCamera, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    vec![update_camera(base.fixture.camera.clone())]
}
