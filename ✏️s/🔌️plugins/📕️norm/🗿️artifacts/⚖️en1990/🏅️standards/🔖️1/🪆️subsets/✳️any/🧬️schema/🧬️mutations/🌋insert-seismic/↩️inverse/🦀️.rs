use super::InsertSeismic; use crate::En1990Mutation; use crate::En1990Snapshot;
pub fn inverse(_payload: &InsertSeismic, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeSeismics(crate::standards::v1::subsets::any::schema::mutations::change_seismics::ChangeSeismics { new_seismics: base.seismics.clone() })]
}
