//! 🧪️ `regen_assets` — moved out of `📸️snapshot/🦀️.rs` into its canonical test implementation.
use super::{compliant_demo, encode_en1997_dsl, encode_en1997_pack, noncompliant_demo};

#[test]
fn regen_example_assets_when_env_set() {
    if std::env::var("SEMIO_REGEN_EN1997_ASSETS").ok().as_deref() != Some("1") {
        return;
    }
    let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    assert!(assets.is_dir(), "assets dir missing: {}", assets.display());
    for (name, snap) in [
        ("🎬️demo", compliant_demo()),
        ("🏗compliant", compliant_demo()),
        ("🚨noncompliant", noncompliant_demo()),
    ] {
        let dir = assets.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("🗣️.dsl.semio"), encode_en1997_dsl(&snap)).unwrap();
        std::fs::write(dir.join("📦️.pack.semio"), encode_en1997_pack(&snap)).unwrap();
    }
}
