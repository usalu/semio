use super::ChangePileCount;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangePileCount, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let count = base.piles.iter().find(|f| f.id == payload.id).map(|f| f.count).unwrap_or(payload.new_count);
    vec![En1997Mutation::ChangePileCount(ChangePileCount { id: payload.id.clone(), new_count: count })]
}
