#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio")).expect("../../📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio").leak();
    assert!(text.len() > 8);
}
