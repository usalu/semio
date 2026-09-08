
use super::*;
use crate::editor::writer::testkit::{app_with_jack, new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_document_tree_for_jack() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    let node = app.render(WRITER_PLAY_BODY_ARTIFACT, Some(&crate::dsl::jack_example_json()), &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("render JSON");
    assert!(json.contains("\"type\":\"tree\""));
    assert!(json.contains("Query"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_and_its_children_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.children.len(), 2);
    assert!(definition.children.iter().all(|child| child.body_key.as_deref() == Some(WRITER_PLAY_BODY_ARTIFACT)));
}

/// 🌳️ The AST section only appears for `jack`-language documents (see `render`'s early return for
/// any other language) — load the jack fixture first.
#[semio_framework_async_macros::async_test]
async fn document_lists_the_ast_section_for_jack_documents() {
    let mut app = app_with_jack().await;
    assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).await.contains("writer-play-document.ast"));
}

/// 📄️ A non-jack (default/plaintext) document renders the plain id/language fallback section
/// instead of the AST tree.
#[semio_framework_async_macros::async_test]
async fn document_falls_back_to_a_plain_section_for_non_jack_documents() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).await.contains("writer-document"));
}
