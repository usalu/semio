use crate::change_anchor_a_s::ChangeAnchorAs;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorAs, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_a_s::ChangeAnchorAs> {
    Ok((|| {
    let Some(a) = base.anchors.iter().find(|a| a.id == payload.anchor_id) else { return vec![]; };
    vec![Op::from(ChangeAnchorAs { anchor_id: payload.anchor_id.clone(), new_value: a.a_s })]

    })())
}
