#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️hexagonal-mushroom-column.dsl.semio");
    assert!(text.len() > 8);
}
