#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️box-shell-preview.dsl.semio");
    assert!(text.len() > 8);
}
