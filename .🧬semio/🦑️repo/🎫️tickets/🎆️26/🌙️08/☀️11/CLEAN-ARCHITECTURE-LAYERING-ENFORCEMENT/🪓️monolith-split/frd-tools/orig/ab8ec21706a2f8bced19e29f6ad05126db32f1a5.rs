//! 🧬️ The TypeScript bindings rendered from the owned schema metadata equal the checked-in generated file.
use super::*;

#[test]
fn exports_typescript_bindings() {
    crate::schema_metadata::validate().unwrap();
    let rendered = crate::schema_metadata::render_typescript();
    if let Some(path) = std::env::var_os("SEMIO_TYPEGEN_OUT") {
        std::fs::write(path, &rendered).unwrap();
    } else {
        assert_eq!(rendered, runtime_fixture_text("../../🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"));
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
