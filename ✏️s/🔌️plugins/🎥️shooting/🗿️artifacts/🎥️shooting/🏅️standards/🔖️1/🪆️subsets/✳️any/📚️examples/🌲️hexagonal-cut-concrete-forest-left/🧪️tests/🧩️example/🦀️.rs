#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn forest_left_example_parses_as_shooting_dsl() {
    use crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl;
    let text = include_str!("../../🖼️assets/🌲️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio");
    let snapshot = parse_dsl(text).expect("hexagonal-cut-concrete-forest-left example parses");
    assert_eq!(snapshot.active_asset_id, "forest-left");
    let asset = snapshot.assets.first().expect("forest asset");
    assert_eq!(asset.url, "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb");
}
