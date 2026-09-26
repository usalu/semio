//! Inverse for `change-snow-zone`.
use super::ChangeSnowZone;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeSnowZone, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeSnowZone(ChangeSnowZone { new_snow_zone: base.snow_zone.clone() })]
}
