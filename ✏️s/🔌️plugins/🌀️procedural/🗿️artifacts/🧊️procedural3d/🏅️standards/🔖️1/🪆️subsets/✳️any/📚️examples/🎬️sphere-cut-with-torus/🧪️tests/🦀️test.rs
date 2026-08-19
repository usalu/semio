#[test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️sphere-cut-with-torus.dsl.semio");
    assert!(text.len() > 8);
}
