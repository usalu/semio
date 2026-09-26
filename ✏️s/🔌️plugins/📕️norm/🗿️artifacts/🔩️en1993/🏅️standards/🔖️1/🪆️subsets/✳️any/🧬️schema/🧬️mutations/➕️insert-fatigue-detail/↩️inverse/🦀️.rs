use super::InsertFatigueDetail;
use crate::mutations::{remove_fatigue_detail, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertFatigueDetail, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.fatigue_details.len());
    vec![En1993Mutation::RemoveFatigueDetail(remove_fatigue_detail::RemoveFatigueDetail { index: at })]
}
