#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️aluminium-roof-purlin.dsl.semio");
    assert!(text.len() > 8);
}
