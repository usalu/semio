#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
use crate::editor::architect::modes::review::*;

#[semio_framework_async_macros::async_test]
async fn the_mode_declares_no_layout_of_its_own() {
    let definition = definition();
    assert_eq!(definition.id, ARCHITECT_MODE_REVIEW);
    assert!(definition.layout_id.is_none());
}
