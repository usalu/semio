use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_shared_table_window_kind() {
    let def = definition();
    assert_eq!(def.id, TableWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    let document = Din18599Snapshot::default();
    let _node = render(&document);
}
