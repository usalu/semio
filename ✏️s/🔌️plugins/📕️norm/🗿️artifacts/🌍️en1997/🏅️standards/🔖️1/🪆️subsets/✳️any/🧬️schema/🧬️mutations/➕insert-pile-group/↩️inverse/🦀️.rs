use crate::insert_pile_group::InsertPileGroup;
use crate::remove_pile_group::RemovePileGroup;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &InsertPileGroup, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_pile_group::RemovePileGroup> {
    Ok((|| {
    let at = payload.index.min(base.pile_groups.len());
    vec![Op::from(RemovePileGroup { index: at })]

    })())
}
