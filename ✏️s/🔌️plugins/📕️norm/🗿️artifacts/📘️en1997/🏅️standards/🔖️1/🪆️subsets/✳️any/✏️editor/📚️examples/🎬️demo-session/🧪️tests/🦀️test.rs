#[semio_framework_async_macros::async_test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🎮️demo.cmd.semio");
    assert!(text.len() > 8);
}
