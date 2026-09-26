//! Inverse for `change-fatigue-detail`.
use super::ChangeFatigueDetail;
use crate::{En1994Mutation, En1994Snapshot};
pub fn inverse(_payload: &ChangeFatigueDetail, base: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::ChangeFatigueDetail(ChangeFatigueDetail { new_fatigue_detail: base.fatigue_detail.clone() })]
}
