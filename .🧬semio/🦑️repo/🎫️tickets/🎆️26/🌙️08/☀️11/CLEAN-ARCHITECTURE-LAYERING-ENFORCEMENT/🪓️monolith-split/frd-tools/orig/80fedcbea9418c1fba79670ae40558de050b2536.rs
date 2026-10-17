#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn forest_left_example_parses_as_shooting_dsl() {
    use crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl;
    let text = runtime_fixture_text("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🌲️hexagonal-cut/🗣️.dsl.semio");
    let snapshot = parse_dsl(text).expect("hexagonal-cut-concrete-forest-left example parses");
    assert_eq!(snapshot.active_asset_id, "forest-left");
    let asset = snapshot.assets.first().expect("forest asset");
    assert_eq!(asset.url, "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
