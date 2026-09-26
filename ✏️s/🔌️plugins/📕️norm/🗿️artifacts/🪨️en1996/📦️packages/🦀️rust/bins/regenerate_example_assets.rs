//! 🖼️ Regenerate committed EN 1996 example DSL/pack assets.
fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    semio_s_artifact_norm_en1996::snapshot::text::write_committed_example_assets(&root)
        .unwrap_or_else(|e| panic!("write assets under {}: {e}", root.display()));
    println!("ok");
}
