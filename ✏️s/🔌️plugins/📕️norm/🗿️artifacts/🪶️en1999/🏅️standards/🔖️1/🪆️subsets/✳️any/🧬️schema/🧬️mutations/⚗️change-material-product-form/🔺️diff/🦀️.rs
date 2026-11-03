//! 🔺️ `change-material-product-form` diff: patches `product_form` of the addressed material row.

use crate::change_material_product_form::ChangeMaterialProductForm;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MaterialsPatch, En1999MaterialsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeMaterialProductForm, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.materials.iter().find(|row| row.id == payload.material_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "material", Vec::<String>::new());
    };
    if r0.product_form == payload.new_product_form {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The material already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { materials: Some(En1999MaterialsRows::modification(&payload.material_id, En1999MaterialsPatch { product_form: Some(payload.new_product_form), ..Default::default() })), ..Default::default() })
}
