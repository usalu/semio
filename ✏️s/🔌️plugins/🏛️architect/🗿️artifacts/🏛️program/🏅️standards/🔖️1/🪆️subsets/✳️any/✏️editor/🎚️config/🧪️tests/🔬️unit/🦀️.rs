use super::*;

#[semio_framework_async_macros::async_test]
async fn set_config_diffs_only_the_named_fields_that_differ_and_inverts_to_their_base_values() {
    let base = ArchitectConfig { last_result_json: "[]".into(), ..ArchitectConfig::default() };
    let operation = ArchitectConfigMutation::SetConfig(SetConfig { search_query: Some("hall".into()), last_result_json: Some("[]".into()), ..SetConfig { search_query: None, search_history_json: None, last_result_json: None, last_analysis_json: None } });
    assert_eq!(operation.diff(&base).diff(), &ArchitectConfigDiff { search_query: Some("hall".into()), ..Default::default() }, "unchanged and unnamed fields stay out of the diff");
    assert_eq!(protocol::apply_diff(operation.diff(&base).diff(), &base).expect("valid diff"), ArchitectConfig { search_query: "hall".into(), last_result_json: "[]".into(), ..ArchitectConfig::default() });
    let inverse = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![ArchitectConfigMutation::SetConfig(SetConfig { search_query: Some(String::new()), search_history_json: None, last_result_json: None, last_analysis_json: None })], "the inverse sets exactly the touched field back to its base value");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
}

#[semio_framework_async_macros::async_test]
async fn the_snapshot_helper_names_only_the_differing_fields() {
    let base = ArchitectConfig::default();
    assert!(snapshot(&base, base.clone()).is_empty());
    let next = ArchitectConfig { search_query: "hall".into(), ..ArchitectConfig::default() };
    assert_eq!(snapshot(&base, next), vec![ArchitectConfigMutation::SetConfig(SetConfig { search_query: Some("hall".into()), search_history_json: None, last_result_json: None, last_analysis_json: None })]);
}

#[semio_framework_async_macros::async_test]
async fn the_config_diff_obeys_the_absorb_and_inverse_laws() {
    let base = ArchitectConfig { search_query: "a".into(), ..ArchitectConfig::default() };
    let first = ArchitectConfigDiff { search_query: Some("b".into()), last_result_json: Some("[1]".into()), ..Default::default() };
    let second = ArchitectConfigDiff { search_query: Some("c".into()), ..Default::default() };
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first.clone(), second).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &first).await;
}

#[semio_framework_async_macros::async_test]
async fn an_empty_search_history_parses_to_no_queries() {
    assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
}
