use crate::remove_pile_group::RemovePileGroup;
use crate::insert_pile_group::InsertPileGroup;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(payload: &RemovePileGroup, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_pile_group::InsertPileGroup> {
    Ok((|| {
    match base.pile_groups.get(payload.index).cloned() {
        Some(pile_group) => vec![Op::from(InsertPileGroup { index: payload.index, pile_group })],
        None => Vec::new(),
    }

    })())
}
