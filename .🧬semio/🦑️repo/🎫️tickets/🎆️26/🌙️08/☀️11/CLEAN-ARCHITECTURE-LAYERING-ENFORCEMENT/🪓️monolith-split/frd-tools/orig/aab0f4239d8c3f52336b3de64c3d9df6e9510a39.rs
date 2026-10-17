#[test]
fn primary_asset_is_nonempty() {
    let text = runtime_fixture_text("../../📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio");
    assert!(text.len() > 8);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
