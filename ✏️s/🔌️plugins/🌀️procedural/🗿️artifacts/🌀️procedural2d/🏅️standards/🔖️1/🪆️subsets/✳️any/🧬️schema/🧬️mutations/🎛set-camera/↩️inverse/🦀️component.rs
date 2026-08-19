//! ↩️ Inverse for `UpdateCamera` — restores the captured BASE camera. The camera always exists (it
//! is not optional on the fixture), so this is unconditional, unlike the id-keyed leaves above.

use crate::artifacts::procedural2d::mutations::{update_camera, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub async fn inverse(_payload: &super::mutation::UpdateCamera, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    vec![update_camera(base.fixture.camera.clone())]
}
