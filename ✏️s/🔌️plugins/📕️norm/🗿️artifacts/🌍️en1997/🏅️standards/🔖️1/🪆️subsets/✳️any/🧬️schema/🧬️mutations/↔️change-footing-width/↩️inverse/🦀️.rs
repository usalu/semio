use crate::change_footing_width::ChangeFootingWidth;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangeFootingWidth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_footing_width::ChangeFootingWidth> {
    Ok((|| {
    let width = base.footings.iter().find(|f| f.id == payload.id).map(|f| f.width).unwrap_or(payload.new_width);
    vec![Op::from(ChangeFootingWidth { id: payload.id.clone(), new_width: width })]

    })())
}
