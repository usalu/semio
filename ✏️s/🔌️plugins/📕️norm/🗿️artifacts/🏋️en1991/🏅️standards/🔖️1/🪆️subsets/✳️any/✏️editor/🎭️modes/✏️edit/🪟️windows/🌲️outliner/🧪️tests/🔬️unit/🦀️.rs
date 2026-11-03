use crate::modes::edit::windows::outliner::*;
use crate::unit_tests::context;
use semio_framework_value::DslValue;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_OUTLINER);
    assert_eq!(definition().id, WINDOW_OUTLINER);
}

#[semio_framework_async_macros::async_test]
async fn lists_the_first_entity_of_the_document_with_a_select_row() {
    let mut app = context::app_with_registry().await;
    let tree = semio_framework_value::ToValue::to_value(&app.snapshot().expect("projection"));
    let id = semio_s_artifact_norm_en1991_scene::ENTITY_KINDS
        .iter()
        .find_map(|kind| match tree.get(kind.collection) {
            Some(DslValue::Array(items)) => items.first().and_then(|item| match item.get("id") {
                Some(DslValue::String(id)) => Some(id.clone()),
                _ => None,
            }),
            _ => None,
        })
        .expect("the reference document has an entity");
    let rendered = context::render(&mut app, BODY_OUTLINER).await;
    assert!(rendered.contains(&id) && rendered.contains("select"), "outliner lists {id}: {rendered}");
    context::close(&mut app);
}
