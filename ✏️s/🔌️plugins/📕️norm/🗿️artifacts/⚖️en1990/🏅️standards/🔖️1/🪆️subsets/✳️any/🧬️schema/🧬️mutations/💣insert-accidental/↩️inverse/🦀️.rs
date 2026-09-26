use super::InsertAccidental; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &InsertAccidental, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeAccidentals(crate::standards::v1::subsets::any::schema::mutations::change_accidentals::ChangeAccidentals { new_accidentals: base.accidentals.clone() })]
}
