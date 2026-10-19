//! ↩️ Inverse of `SetStoreyCutHeight`: an absolute `SetStoreyCutHeight` back to the base cut height (an assigned null when the base had
//! none), none when the storey is absent.

use crate::standards::v1::subsets::any::schema::mutations::set_storey_cut_height::SetStoreyCutHeight;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetStoreyCutHeight, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_storey_cut_height::SetStoreyCutHeight>,
{
    match base.storeys.get(&payload.id) {
        Some(storey) => vec![Op::from(SetStoreyCutHeight { id: payload.id.clone(), cut_height: Assigned::new(storey.cut_height) })],
        None => Vec::new(),
    }
}
