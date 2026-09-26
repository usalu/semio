//! Inverse for `change-exceptional-snow-north-german-lowlands`.
use super::ChangeExceptionalSnowNorthGermanLowlands;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeExceptionalSnowNorthGermanLowlands, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeExceptionalSnowNorthGermanLowlands(ChangeExceptionalSnowNorthGermanLowlands { new_exceptional_snow_north_german_lowlands: base.exceptional_snow_north_german_lowlands })]
}
