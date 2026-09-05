#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio");
    assert!(text.len() > 8);
}
