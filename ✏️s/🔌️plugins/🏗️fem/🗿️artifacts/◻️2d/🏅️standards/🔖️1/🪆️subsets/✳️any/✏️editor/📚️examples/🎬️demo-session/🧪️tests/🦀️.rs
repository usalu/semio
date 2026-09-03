#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🎮️.cmd.semio");
    assert!(text.len() > 8);
}
