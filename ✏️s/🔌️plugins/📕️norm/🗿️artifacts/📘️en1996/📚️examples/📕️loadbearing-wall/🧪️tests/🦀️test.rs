#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️loadbearing-wall.dsl.semio");
    assert!(text.len() > 8);
}
