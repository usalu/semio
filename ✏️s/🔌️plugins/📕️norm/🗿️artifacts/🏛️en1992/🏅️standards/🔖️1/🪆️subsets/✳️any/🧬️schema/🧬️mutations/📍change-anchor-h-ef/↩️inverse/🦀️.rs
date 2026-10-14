use crate::change_anchor_h_ef::ChangeAnchorHEf;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeAnchorHEf, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_anchor_h_ef::ChangeAnchorHEf> {
    Ok((|| {
    let Some(a) = base.anchors.iter().find(|a| a.id == payload.anchor_id) else { return vec![]; };
    vec![Op::from(ChangeAnchorHEf { anchor_id: payload.anchor_id.clone(), new_value: a.h_ef })]

    })())
}
