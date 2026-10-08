//! 🧪️ CAD presence — the sparse `Set` mutation's concrete inverse sums to exactly the negative diff.
use super::{CadPresence, CadPresenceDiff, CadPresenceMutation};
use protocol::os_spr::protocol_laws::{assert_mutation_inverse_sum_law};
use protocol::Mutation;

fn peer() -> CadPresence {
    CadPresence { camera_zoom: 2.0, engagement_step: "Pick".into(), engagement_pane: Some("shape".into()), ..CadPresence::default() }
}

/// ⚖️ A set that changes several fields restores every one of them exactly.
#[semio_framework_async_macros::async_test]
async fn setting_several_fields_inverts_to_the_negative_diff() {
    let mutation = CadPresenceMutation::Set { presence: CadPresence { camera_position: [1.0, 2.0, 3.0], camera_fov: 35.0, engagement_pane: None, ..peer() } };
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}

/// ⚖️ A set that changes one field carries only that field in its diff.
#[semio_framework_async_macros::async_test]
async fn setting_one_field_carries_only_that_field() {
    let mutation = CadPresenceMutation::Set { presence: CadPresence { camera_zoom: 4.0, ..peer() } };
    let outcome = mutation.diff(&peer());
    assert_eq!(*outcome.diff(), CadPresenceDiff { camera_zoom: Some(4.0), ..Default::default() });
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}

