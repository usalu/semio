
use super::*;
use crate::editor::layout::LayoutCommand;

#[semio_framework_async_macros::async_test]
async fn set_locale_is_host_pushed_with_bare_wire_keyword() {
    let command = LayoutCommand::SetLocale(SetLocale { value: "de-DE".into() });
    assert!(protocol::OpText::print_op(&command).starts_with("locale "), "wire keyword must stay bare 'locale'");
}
