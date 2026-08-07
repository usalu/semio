#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️high-consequence-office.dsl.semio");
    assert!(text.len() > 8);
}
