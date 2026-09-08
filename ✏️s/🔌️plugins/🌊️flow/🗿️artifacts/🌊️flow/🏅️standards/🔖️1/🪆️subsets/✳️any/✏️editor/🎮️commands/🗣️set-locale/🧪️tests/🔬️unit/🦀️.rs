
use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app, render};
use crate::editor::flow::{FLOW_PLAY_BODY_DOCUMENT, FlowCommand};

#[semio_framework_async_macros::async_test]
async fn flow_labels_resolve_native_english_and_german() {
    let mut app = flow_app().await;
    let english = render(&mut app, FLOW_PLAY_BODY_DOCUMENT).await;
    assert!(english.contains("Widgets") && english.contains("Synapses"), "english labels: {english}");
    dispatch(&mut app, FlowCommand::SetLocale(SetLocale { value: "de-DE".into() })).await;
    let german = render(&mut app, FLOW_PLAY_BODY_DOCUMENT).await;
    assert!(german.contains("Synapsen"), "german labels: {german}");
}
