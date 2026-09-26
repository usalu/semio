use crate::mutations::change_anchor_h_ef::ChangeAnchorHEf;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeAnchorHEf, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(a) = base.anchors.iter().find(|a| a.id == payload.anchor_id) else { return vec![]; };
    vec![En1992Mutation::ChangeAnchorHEf(ChangeAnchorHEf { anchor_id: payload.anchor_id.clone(), new_value: a.h_ef })]
}
