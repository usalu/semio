#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio");
    assert!(text.len() > 8);
}
