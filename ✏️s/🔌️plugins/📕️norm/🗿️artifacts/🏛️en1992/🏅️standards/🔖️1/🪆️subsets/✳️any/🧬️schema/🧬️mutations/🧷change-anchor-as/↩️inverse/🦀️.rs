use crate::mutations::change_anchor_a_s::ChangeAnchorAs;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeAnchorAs, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(a) = base.anchors.iter().find(|a| a.id == payload.anchor_id) else { return vec![]; };
    vec![En1992Mutation::ChangeAnchorAs(ChangeAnchorAs { anchor_id: payload.anchor_id.clone(), new_value: a.a_s })]
}
