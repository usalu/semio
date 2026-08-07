#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio");
    assert!(text.len() > 8);
}
