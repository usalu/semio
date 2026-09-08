
use super::EngagementSubmit;
use crate::editor::writer::testkit::new_app;
use crate::editor::writer::{WriterCommand, WRITER_PLAY_WINDOW_KIND};
use semio_framework_plugin::{PluginApp, WindowMeasure};

#[semio_framework_async_macros::async_test]
async fn engagement_submit_parses_font_size() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("font 16".into()) }), &semio_framework_plugin::testkit::meta("local")).await.expect("submit");
    // Font size is ephemeral config state — no history entry.
    assert!(result.mutations.is_empty());
    let measures = app.window_measures().await;
    let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
    assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_parses_normalized_shell_drafts() {
    // The React shell PascalCases and strips separators from every draft before submitting it
    // (`normalizeEngagementActionText`), so "font 16" arrives as "Font16", "tab 4" as "Tab4",
    // and "line numbers" as "LineNumbers".
    let mut app = new_app().await;
    let before_toggle = app.window_engagements().await.get(WRITER_PLAY_WINDOW_KIND).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");

    app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("Font16".into()) }), &semio_framework_plugin::testkit::meta("local")).await.expect("font");
    app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("Tab4".into()) }), &semio_framework_plugin::testkit::meta("local")).await.expect("tab");
    app.dispatch_typed(WriterCommand::EngagementSubmit(EngagementSubmit { value: Some("LineNumbers".into()) }), &semio_framework_plugin::testkit::meta("local")).await.expect("line numbers");

    let measures = app.window_measures().await;
    let main = measures.get(WRITER_PLAY_WINDOW_KIND).expect("main measures");
    assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-font-size-measure" && *value == 16.0)));
    assert!(main.iter().any(|m| matches!(m, WindowMeasure::Slider { id, value, .. } if id == "writer-tab-size-measure" && *value == 4.0)));

    let after_toggle = app.window_engagements().await.get(WRITER_PLAY_WINDOW_KIND).and_then(|engagement| engagement.options.as_ref()).and_then(|options| options.first()).and_then(|option| option.pressed).expect("line-numbers pressed state");
    assert_eq!(after_toggle, !before_toggle);
}
