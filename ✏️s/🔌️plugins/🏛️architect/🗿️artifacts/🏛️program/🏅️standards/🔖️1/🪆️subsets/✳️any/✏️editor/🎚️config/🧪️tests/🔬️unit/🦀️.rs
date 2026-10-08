use super::*;

#[semio_framework_async_macros::async_test]
async fn replace_config_diffs_only_the_fields_that_differ_and_inverts_to_the_base() {
    let base = ArchitectConfig { last_result_json: "[]".into(), ..ArchitectConfig::default() };
    let next = ArchitectConfig { search_query: "hall".into(), last_result_json: "[]".into(), ..ArchitectConfig::default() };
    let operation = ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: next.clone() });
    assert_eq!(operation.diff(&base).diff(), &ArchitectConfigDiff { search_query: Some("hall".into()), ..Default::default() }, "unchanged fields stay out of the diff");
    assert_eq!(protocol::apply_diff(operation.diff(&base).diff(), &base).expect("valid diff"), next);
    assert_eq!(operation.inverse(&base).expect("valid retained mutation inverse fixture"), vec![ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn the_config_diff_obeys_the_absorb_inverse_and_between_laws() {
    let base = ArchitectConfig { search_query: "a".into(), ..ArchitectConfig::default() };
    let first = ArchitectConfigDiff { search_query: Some("b".into()), last_result_json: Some("[1]".into()), ..Default::default() };
    let second = ArchitectConfigDiff { search_query: Some("c".into()), ..Default::default() };
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first.clone(), second).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &first).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<ArchitectConfig, ArchitectConfigDiff>(&base, &ArchitectConfig { search_history_json: "[]".into(), ..ArchitectConfig::default() }).await;
}

#[semio_framework_async_macros::async_test]
async fn an_empty_search_history_parses_to_no_queries() {
    assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
}
