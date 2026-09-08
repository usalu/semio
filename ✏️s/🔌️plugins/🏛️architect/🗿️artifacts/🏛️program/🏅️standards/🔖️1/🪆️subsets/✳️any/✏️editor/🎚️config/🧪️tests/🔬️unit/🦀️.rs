
use super::*;

#[semio_framework_async_macros::async_test]
async fn active_register_falls_back_to_elements() {
    assert_eq!(active_register(&ArchitectConfig::default()), "elements");
    assert_eq!(active_register(&ArchitectConfig { active_register: "risks".into(), ..ArchitectConfig::default() }), "risks");
}

#[semio_framework_async_macros::async_test]
async fn a_snapshot_operation_replaces_the_whole_config_and_inverts_to_the_base() {
    let base = ArchitectConfig::default();
    let next = ArchitectConfig { search_query: "hall".into(), ..ArchitectConfig::default() };
    let operation = ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: next.clone() });
    assert_eq!(operation.diff(&base).diff(), &next);
    assert_eq!(operation.inverse(&base), vec![ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: base })]);
}

#[semio_framework_async_macros::async_test]
async fn an_empty_active_report_parses_to_none() {
    assert!(parse_active_report(&ArchitectConfig::default()).is_none());
    assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
}
