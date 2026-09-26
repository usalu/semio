use super::ChangeFootingEmbedment;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangeFootingEmbedment, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let embedment = base.footings.iter().find(|f| f.id == payload.id).map(|f| f.embedment).unwrap_or(payload.new_embedment);
    vec![En1997Mutation::ChangeFootingEmbedment(ChangeFootingEmbedment { id: payload.id.clone(), new_embedment: embedment })]
}
