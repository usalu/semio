#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️rectangle-extrude-volume.dsl.semio");
    assert!(text.len() > 8);
}
