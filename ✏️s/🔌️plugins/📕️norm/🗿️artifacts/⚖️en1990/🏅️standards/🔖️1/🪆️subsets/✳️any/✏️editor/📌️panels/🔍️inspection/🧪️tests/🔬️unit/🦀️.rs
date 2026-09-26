use super::*;
use crate::document::NormHost;
use crate::editor::en1990::En1990Family;

#[semio_framework_async_macros::async_test]
async fn inspection_renders_with_locale() {
    let host = NormHost::<En1990Family>::from_artifact(crate::En1990Snapshot::default());
    let _node = render(&host, None, semio_framework_plugin::Locale::En, None).expect("node assembly");
}
