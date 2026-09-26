//! Inverse for `change-hoisting-speed`.
use super::ChangeHoistingSpeed;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeHoistingSpeed, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeHoistingSpeed(ChangeHoistingSpeed { new_hoisting_speed: base.hoisting_speed })]
}
