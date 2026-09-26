use super::ChangePileLength;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangePileLength, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let length = base.piles.iter().find(|f| f.id == payload.id).map(|f| f.length).unwrap_or(payload.new_length);
    vec![En1997Mutation::ChangePileLength(ChangePileLength { id: payload.id.clone(), new_length: length })]
}
