#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️high-strength-connection.dsl.semio");
    assert!(text.len() > 8);
}
