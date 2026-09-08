
use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn search_finds_reception_element() {
    let hits = search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Reception".into()], ..Default::default() }, None, None);
    assert!(hits.iter().any(|h| h.name == "Reception"));
}

#[semio_framework_async_macros::async_test]
async fn search_history_records_query() {
    let mut history = Vec::new();
    search_plugin(&sample_plugin(), &SearchQuery { keywords: vec!["Waiting".into()], ..Default::default() }, None, Some(&mut history));
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].keywords, vec!["Waiting".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn entity_kind_filter_limits_registers() {
    let hits = search_plugin(&sample_plugin(), &SearchQuery { entity_kinds: vec!["elements".into()], ..Default::default() }, None, None);
    assert!(hits.iter().all(|h| h.register == "elements"));
}
