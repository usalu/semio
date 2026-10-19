//! ↩️ Inverse of `RenameElement`: an absolute `RenameElement` back to the base name, none when the element is absent.

use crate::standards::v1::subsets::any::schema::mutations::rename_element::RenameElement;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RenameElement, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_element::RenameElement>,
{
    match semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::rename(base, &payload.id, &payload.name) {
        Some((current, _)) => vec![Op::from(RenameElement { id: payload.id.clone(), name: current })],
        None => Vec::new(),
    }
}
