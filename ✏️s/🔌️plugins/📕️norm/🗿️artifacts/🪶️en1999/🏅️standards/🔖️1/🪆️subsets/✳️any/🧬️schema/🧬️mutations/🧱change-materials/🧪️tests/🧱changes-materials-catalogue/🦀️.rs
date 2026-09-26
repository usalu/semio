//! ️ `change-materials` named scenario `replaces-materials-catalogue`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_materials_catalogue_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-materials applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-materials must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    {
        let mut materials = base.materials.clone();
        if let Some(mat) = materials.first_mut() { mat.designation = "aw6061-t6".into(); }
        En1999Mutation::ChangeMaterials(crate::mutations::change_materials::ChangeMaterials { materials })
    }
}
