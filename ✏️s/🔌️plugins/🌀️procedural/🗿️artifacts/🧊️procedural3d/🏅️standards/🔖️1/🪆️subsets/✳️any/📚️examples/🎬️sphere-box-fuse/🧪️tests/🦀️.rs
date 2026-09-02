#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🧪️sphere-box-fuse/🗣️.dsl.semio");
    assert!(text.len() > 8);
}
