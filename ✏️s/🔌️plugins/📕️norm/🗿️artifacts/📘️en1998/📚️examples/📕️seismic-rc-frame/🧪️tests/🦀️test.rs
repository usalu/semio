#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️seismic-rc-frame.dsl.semio");
    assert!(text.len() > 8);
}
