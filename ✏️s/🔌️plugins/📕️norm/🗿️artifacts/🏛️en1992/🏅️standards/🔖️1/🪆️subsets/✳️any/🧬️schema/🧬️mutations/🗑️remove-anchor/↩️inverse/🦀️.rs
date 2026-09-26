use super::RemoveAnchor;
use crate::mutations::insert_anchor::InsertAnchor;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &RemoveAnchor, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some((index, anchor)) = base.anchors.iter().enumerate().find(|(_, a)| a.id == payload.anchor_id) else { return vec![]; };
    vec![En1992Mutation::InsertAnchor(InsertAnchor { index, anchor: anchor.clone() })]
}
