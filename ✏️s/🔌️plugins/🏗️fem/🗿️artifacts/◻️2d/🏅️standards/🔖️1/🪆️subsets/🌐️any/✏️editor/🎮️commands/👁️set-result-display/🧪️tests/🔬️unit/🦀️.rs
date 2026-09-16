use super::*;
use crate::editor::fem2d::modes::edit::windows::results;
use semio_framework_plugin::{HistoryView, ViewModel, ViewWindowInstance};

fn results_view() -> ViewModel {
    ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() }],
        ..Default::default()
    }
}

/// 👁️ LAW: the result display is WINDOW state — it publishes into the addressed window's own
/// configuration partition and never into the document.
#[semio_framework_async_macros::async_test]
async fn set_result_display_is_config_only() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle_window(&SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1, field: None, value: None, window_id: None }, &cfg, &results_view()).expect("the addressed results window accepts a display change");
    assert!(emit.artifact_mutations.is_empty(), "setResultDisplay must not emit a document VCS operation");
    assert_eq!(emit.window_config_mutations.iter().map(semio_framework_plugin::WindowConfigMutation::window_id).collect::<Vec<_>>(), ["results-left"]);
    assert!(handle(&SetResultDisplay { source_id: None, mode: "static".into(), mode_index: 0, field: None, value: None, window_id: None }, &doc, &cfg).is_err(), "without a window there is nothing to address");
}

/// 🎛️ LAW: the `{field, value}` vocabulary a persistent panel select speaks changes ONE field and
/// leaves the rest of the window's display state alone.
#[semio_framework_async_macros::async_test]
async fn set_result_display_applies_one_named_field() {
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let apply = |field: &str, value: &str| {
        let mut next = results::config::current(&cfg);
        apply_field(&mut next, field, value).map(|()| next)
    };
    assert_eq!(apply("sourceId", "dead").expect("source").result_source_id.as_deref(), Some("dead"));
    assert_eq!(apply("sourceId", "").expect("cleared source").result_source_id, None);
    assert_eq!(apply("mode", "buckling").expect("mode").result_mode, crate::app_surface::ResultMode::Buckling);
    assert_eq!(apply("modeIndex", "2").expect("index").result_mode_index, 2);
    assert_eq!(apply("modeIndex", "3.0").expect("a number control's own spelling").result_mode_index, 3);
    assert!(apply("harmonic", "1").is_err(), "an unknown field is refused");
    assert!(apply("mode", "harmonic").is_err(), "an unknown mode is refused");
    assert!(apply("modeIndex", "second").is_err(), "an unparseable index is refused");
}
