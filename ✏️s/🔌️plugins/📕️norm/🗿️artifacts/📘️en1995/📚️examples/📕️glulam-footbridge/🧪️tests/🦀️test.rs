#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️glulam-footbridge.dsl.semio");
    assert!(text.len() > 8);
}
