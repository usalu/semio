use super::EngagementSubmit;
use crate::editor::writer::unit_tests::context::new_app;
use crate::editor::writer::{WriterCommand, WRITER_PLAY_WINDOW_KIND};
use semio_framework_plugin::{PluginApp, ViewModel, ViewWindowInstance, WindowMeasure};

const WINDOW_ID: &str = "writer-main-test";

fn view() -> ViewModel {
    ViewModel { window_instances: vec![ViewWindowInstance { id: WINDOW_ID.into(), window_kind_id: WRITER_PLAY_WINDOW_KIND.into() }], ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_parses_font_size() {
    let mut app = new_app().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("font 16".into()) })).await;
    // Font size is ephemeral config state — no history entry.
    assert!(result.mutations.is_empty());
    let measures = app.window_measures(&view()).await;
    let main = measures.get(WINDOW_ID).expect("main measures");
    assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_parses_separatorless_drafts() {
    // The verb token is matched ignoring case and separators, so a name-token line that carries no
    // spaces ("Font16", "Tab4", "LineNumbers") reads the same as the spaced line above.
    let mut app = new_app().await;
    let before_toggle = app.window_engagements(&view()).await.get(WINDOW_ID).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");

    crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("Font16".into()) })).await;
    crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("Tab4".into()) })).await;
    crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("LineNumbers".into()) })).await;

    let measures = app.window_measures(&view()).await;
    let main = measures.get(WINDOW_ID).expect("main measures");
    assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
    assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-tab-size-measure" && *value == 4.0)));

    let after_toggle = app.window_engagements(&view()).await.get(WINDOW_ID).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");
    assert_eq!(after_toggle, !before_toggle);
}
