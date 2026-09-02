#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🧪️face-sweep-extrude/🗣️.dsl.semio");
    assert!(text.len() > 8);
}
