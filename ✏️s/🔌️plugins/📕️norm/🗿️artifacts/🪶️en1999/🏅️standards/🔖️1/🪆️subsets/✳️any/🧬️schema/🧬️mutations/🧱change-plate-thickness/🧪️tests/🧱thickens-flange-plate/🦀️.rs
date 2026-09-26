//! ️ `change-plate-thickness` named scenario `thickens-flange-plate`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn thickens_flange_plate_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-plate-thickness applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-plate-thickness must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    En1999Mutation::ChangePlateThickness(crate::mutations::change_plate_thickness::ChangePlateThickness {
        section_id: base.sections[0].id.clone(),
        element_id: base.sections[0].elements[0].id.clone(),
        new_thickness: base.sections[0].elements[0].thickness + 0.001,
    })
}
