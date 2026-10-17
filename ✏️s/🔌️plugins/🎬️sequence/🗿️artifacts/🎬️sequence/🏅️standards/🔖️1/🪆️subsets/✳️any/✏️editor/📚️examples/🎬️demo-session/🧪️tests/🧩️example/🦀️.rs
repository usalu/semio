use semio_framework_test_fixtures::fixture_text;

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = fixture_text!("../../../../✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio");
    assert!(text.len() > 8);
}
