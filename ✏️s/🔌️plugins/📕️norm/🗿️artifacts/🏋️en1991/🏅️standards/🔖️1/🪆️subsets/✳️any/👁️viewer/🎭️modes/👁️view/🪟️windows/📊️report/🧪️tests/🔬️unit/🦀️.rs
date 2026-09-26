use super::*;

#[semio_framework_async_macros::async_test]
async fn render_assembles() {
    let document = crate::En1991Snapshot::default();
    let _node = render(&document, semio_framework_plugin::Locale::En);
}
