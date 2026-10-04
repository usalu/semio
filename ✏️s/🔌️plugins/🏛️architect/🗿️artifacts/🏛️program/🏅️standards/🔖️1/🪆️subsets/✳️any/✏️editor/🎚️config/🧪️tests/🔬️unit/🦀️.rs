use super::*;

#[semio_framework_async_macros::async_test]
async fn a_snapshot_operation_replaces_the_whole_config_and_inverts_to_the_base() {
    let base = ArchitectConfig::default();
    let next = ArchitectConfig { search_query: "hall".into(), ..ArchitectConfig::default() };
    let operation = ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: next.clone() });
    assert_eq!(operation.diff(&base).diff(), &next);
    assert_eq!(operation.inverse(&base).expect("valid retained mutation inverse fixture"), vec![ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: base })]);
}

#[semio_framework_async_macros::async_test]
async fn an_empty_search_history_parses_to_no_queries() {
    assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
}
