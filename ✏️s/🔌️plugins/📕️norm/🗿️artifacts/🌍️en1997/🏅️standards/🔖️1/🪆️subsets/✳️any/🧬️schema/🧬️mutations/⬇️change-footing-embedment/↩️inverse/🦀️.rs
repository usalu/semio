use crate::change_footing_embedment::ChangeFootingEmbedment;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeFootingEmbedment, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footing_embedment::ChangeFootingEmbedment> {
    Ok((|| {
    let embedment = base.footings.iter().find(|f| f.id == payload.id).map(|f| f.embedment).unwrap_or(payload.new_embedment);
    vec![Op::from(ChangeFootingEmbedment { id: payload.id.clone(), new_embedment: embedment })]

    })())
}
