#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio");
    assert!(text.len() > 8);
}
