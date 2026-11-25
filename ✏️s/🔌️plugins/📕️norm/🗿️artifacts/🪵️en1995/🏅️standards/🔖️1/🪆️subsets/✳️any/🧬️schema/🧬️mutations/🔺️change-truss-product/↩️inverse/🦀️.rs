use crate::change_truss_product::ChangeTrussProduct;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussProduct, base: &En1995Snapshot) -> Option<crate::change_truss_product::ChangeTrussProduct> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussProduct { truss_id: payload.truss_id.clone(), new_value: item.product }),
        None => None,
    }
}
