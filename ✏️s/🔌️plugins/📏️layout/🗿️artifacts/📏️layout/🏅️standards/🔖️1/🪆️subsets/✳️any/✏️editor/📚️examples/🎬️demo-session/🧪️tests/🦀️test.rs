#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🎮️demo.cmd.semio");
    assert!(text.len() > 8);
}
