use super::ChangeFootingWidth;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangeFootingWidth, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let width = base.footings.iter().find(|f| f.id == payload.id).map(|f| f.width).unwrap_or(payload.new_width);
    vec![En1997Mutation::ChangeFootingWidth(ChangeFootingWidth { id: payload.id.clone(), new_width: width })]
}
