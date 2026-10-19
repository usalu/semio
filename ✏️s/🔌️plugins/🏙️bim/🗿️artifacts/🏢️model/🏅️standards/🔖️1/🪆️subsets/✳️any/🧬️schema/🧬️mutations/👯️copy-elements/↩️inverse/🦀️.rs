//! ↩️ Inverse of `CopyElements`: one `DeleteElements` of the minted ids of the copied elements; the cascade removes their openings and
//! their property and classification entries, which is exactly what the copy created. Empty when the copy is refused.

use semio_s_artifact_bim_model_mut_any_85::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements;
use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::Map;
use crate::standards::v1::subsets::any::schema::mutations::copy_elements::CopyElements;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CopyElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_85::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements>,
{
    let map = Map::Translate(payload.vector);
    if !map.finite() {
        return Vec::new();
    }
    match semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::duplicate(base, &payload.ids, &payload.prefix, &[map]) {
        Ok(built) if !built.roots.is_empty() => vec![Op::from(DeleteElements { ids: built.roots })],
        _ => Vec::new(),
    }
}
