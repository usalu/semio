use super::*;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn replace_presence_diffs_only_the_fields_that_differ_and_inverts_to_the_base() {
    let base = ArchitectPresence::default();
    let next = ArchitectPresence { active_register: "risks".into(), adjacency_kind_filter: Some(AdjacencyKind::Preferred), ..ArchitectPresence::default() };
    let operation = ArchitectPresenceMutation::ReplacePresence(ReplacePresence { presence: next.clone() });
    assert_eq!(operation.diff(&base).diff(), &ArchitectPresenceDiff { active_register: Some("risks".into()), adjacency_kind_filter: Some(AdjacencyKindFilterSet { value: Some(AdjacencyKind::Preferred) }), ..Default::default() });
    assert_eq!(protocol::apply_diff(operation.diff(&base).diff(), &base).expect("valid diff"), next);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
    let clearing = ArchitectPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&clearing, &next).await;
    assert_eq!(clearing.diff(&next).diff().adjacency_kind_filter, Some(AdjacencyKindFilterSet { value: None }), "clearing the filter is an explicit edit, not an absent one");
}

#[semio_framework_async_macros::async_test]
async fn the_presence_diff_obeys_the_absorb_inverse_and_between_laws() {
    let base = ArchitectPresence::default();
    let first = ArchitectPresenceDiff { active_register: Some("risks".into()), graph_camera_zoom: Some(2.0), ..Default::default() };
    let second = ArchitectPresenceDiff { active_register: Some("users".into()), adjacency_kind_filter: Some(AdjacencyKindFilterSet { value: Some(AdjacencyKind::Preferred) }), ..Default::default() };
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first.clone(), second).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &first).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<ArchitectPresence, ArchitectPresenceDiff>(&base, &ArchitectPresence { graph_camera_x: 4.0, ..ArchitectPresence::default() }).await;
}
