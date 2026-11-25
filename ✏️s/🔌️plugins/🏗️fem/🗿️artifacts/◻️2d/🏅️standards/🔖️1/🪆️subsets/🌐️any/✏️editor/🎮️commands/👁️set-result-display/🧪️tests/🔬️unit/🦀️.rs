use super::*;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results;
use semio_plugin::HistoryView;
use semio_plugin::ViewModel;
use semio_plugin::ViewWindowInstance;

fn results_view() -> ViewModel {
    ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }
}

/// 👁️ LAW: the result display is WINDOW state — it publishes into the addressed window's own
/// configuration partition and never into the document.
#[semio_framework_async_macros::async_test]
async fn set_result_display_is_config_only() {
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("bundled demo loads"));
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle_window(&SetResultDisplay { source_id: Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").unwrap()), mode: "modal".into(), mode_index: 1, field: None, value: None, window_id: None }, &cfg, &results_view()).expect("the addressed results window accepts a display change");
    assert!(emit.artifact_mutations.is_empty(), "setResultDisplay must not emit a document VCS operation");
    assert_eq!(emit.window_config_mutations.iter().map(semio_plugin::WindowConfigMutation::window_id).collect::<Vec<_>>(), ["results-left"]);
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
    let source = semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").unwrap();
    assert_eq!(apply("sourceId", &source).expect("source").result_source_id.as_deref(), Some(source.as_str()));
    assert!(apply("sourceId", "dead").is_err(), "untyped source names cannot collapse physical namespaces");
    assert_eq!(apply("sourceId", "").expect("cleared source").result_source_id, None);
    assert_eq!(apply("mode", "buckling").expect("mode").result_mode, semio_s_fem_app_surface::ResultMode::Buckling);
    assert_eq!(apply("modeIndex", "2").expect("index").result_mode_index, 2);
    assert_eq!(apply("modeIndex", "3.0").expect("a number control's own spelling").result_mode_index, 3);
    assert!(apply("harmonic", "1").is_err(), "an unknown field is refused");
    assert!(apply("mode", "harmonic").is_err(), "an unknown mode is refused");
    assert!(apply("modeIndex", "second").is_err(), "an unparseable index is refused");
}

/// 🪞️ LAW: a panel control used while another pane holds focus still lands in the results window, keeps what the panel last showed (so the Mode select reads Modal, not Static) and publishes only the field it named.
#[semio_framework_async_macros::async_test]
async fn set_result_display_from_an_unfocused_panel_is_remembered_and_sparse() {
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let view = ViewModel {
        window_id: Some("results-shadow".into()),
        window_instances: vec![ViewWindowInstance { id: "results-shadow".into(), window_kind_id: results::WINDOW_KIND_ID.into() }],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let pick = |field: &str, value: &str| SetResultDisplay { source_id: None, mode: String::new(), mode_index: 0, field: Some(field.into()), value: Some(value.into()), window_id: Some("results-shadow".into()) };
    handle_window(&pick("mode", "modal"), &cfg, &view).expect("mode");
    assert_eq!(results::config::remembered("results-shadow").expect("remembered").result_mode, semio_s_fem_app_surface::ResultMode::Modal);
    handle_window(&pick("modeIndex", "2"), &cfg, &view).expect("index");
    let remembered = results::config::remembered("results-shadow").expect("remembered");
    assert_eq!((remembered.result_mode, remembered.result_mode_index), (semio_s_fem_app_surface::ResultMode::Modal, 2), "a later field keeps the earlier one");
    let sparse = results::config::addressed_fields("results-shadow", &remembered, Some("mode"));
    assert_eq!(sparse.window_id(), "results-shadow");
}
