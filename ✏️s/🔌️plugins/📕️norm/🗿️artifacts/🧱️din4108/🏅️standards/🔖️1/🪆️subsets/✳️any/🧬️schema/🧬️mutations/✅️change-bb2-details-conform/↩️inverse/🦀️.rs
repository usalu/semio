//! ↩️ `change-bb2-details-conform` inverse.

use super::ChangeBb2DetailsConform;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeBb2DetailsConform(ChangeBb2DetailsConform { new_bb2_details_conform: base.bb2_details_conform })]
}
