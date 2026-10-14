use crate::change_pile_count::ChangePileCount;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &ChangePileCount, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_count::ChangePileCount> {
    Ok((|| {
    let count = base.piles.iter().find(|f| f.id == payload.id).map(|f| f.count).unwrap_or(payload.new_count);
    vec![Op::from(ChangePileCount { id: payload.id.clone(), new_count: count })]

    })())
}
