//! ️ `change-material-designation` named scenario `upgrades-alloy-to-6082-t6`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn upgrades_alloy_to_6082_t6_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-material-designation applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-material-designation must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangeMaterialDesignation(crate::mutations::change_material_designation::ChangeMaterialDesignation {
        material_id: base.materials[0].id.clone(),
        new_designation: "aw6060-t6".into(),
    })
}
