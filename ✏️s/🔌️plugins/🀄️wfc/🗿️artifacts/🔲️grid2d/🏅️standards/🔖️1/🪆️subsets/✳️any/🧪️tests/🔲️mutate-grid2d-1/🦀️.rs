//! 🧪️ Subset-level oracle replay — the Rust half of the `🥒️.feature`/`🐍️.py`/`🦀️.rs` triplet. It
//! walks the committed fixture tree off disk (never a hardcoded case list) so a vector added to the
//! table cannot leave this half behind, and replays each one through the artifact's own algebra.

use crate::diff::Grid2dDiff;
use crate::mutations::{apply_grid2d_mutation, inverse_grid2d_mutation, Grid2dMutation, KINDS};
use crate::schema::snapshot::Grid2dSnapshot;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");

fn fixtures() -> std::path::PathBuf {
    std::path::Path::new(ROOT).join("🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
}

fn read(case: &std::path::Path, leaf: &str) -> String {
    std::fs::read_to_string(case.join(leaf)).unwrap_or_else(|error| panic!("{}: {error}", case.join(leaf).display()))
}

fn cases() -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut kinds: Vec<_> = std::fs::read_dir(fixtures()).expect("fixture root reads").filter_map(Result::ok).map(|entry| entry.path()).filter(|path| path.is_dir()).collect();
    kinds.sort();
    for kind in kinds {
        let mut scenarios: Vec<_> = std::fs::read_dir(&kind).expect("kind reads").filter_map(Result::ok).map(|entry| entry.path()).filter(|path| path.is_dir()).collect();
        scenarios.sort();
        out.extend(scenarios);
    }
    out
}

#[test]
fn the_committed_tree_covers_every_declared_kind() {
    assert_eq!(std::fs::read_dir(fixtures()).expect("fixture root reads").filter_map(Result::ok).filter(|entry| entry.path().is_dir()).count(), KINDS.len());
    assert!(!cases().is_empty());
}

#[test]
fn every_committed_vector_replays_forward_and_back() {
    for case in cases() {
        let label = case.display().to_string();
        let before: Grid2dSnapshot = dsl::json::from_json_str(&read(&case, "📸️snapshot/⬅️before/🔣️.json")).unwrap_or_else(|error| panic!("{label}: before decodes: {error}"));
        let after: Grid2dSnapshot = dsl::json::from_json_str(&read(&case, "📸️snapshot/➡️after/🔣️.json")).unwrap_or_else(|error| panic!("{label}: after decodes: {error}"));
        let mutation: Grid2dMutation = dsl::json::from_json_str(&read(&case, "🦠️mutation/🔣️.json")).unwrap_or_else(|error| panic!("{label}: mutation decodes: {error}"));
        let committed: Grid2dDiff = dsl::json::from_json_str(&read(&case, "🔺️diff/🔣️.json")).unwrap_or_else(|error| panic!("{label}: diff decodes: {error}"));

        let produced = <Grid2dMutation as protocol::Mutation<Grid2dSnapshot>>::diff(&mutation, &before);
        assert_eq!(produced.diff(), &committed, "{label}: produced diff differs from the committed one");

        let mut walked = before.clone();
        apply_grid2d_mutation(&mut walked, &mutation).unwrap_or_else(|error| panic!("{label}: forward applies: {error}"));
        assert_eq!(walked, after, "{label}: forward did not reach the committed after-snapshot");

        for step in inverse_grid2d_mutation(&before, &mutation) {
            apply_grid2d_mutation(&mut walked, &step).unwrap_or_else(|error| panic!("{label}: inverse step applies: {error}"));
        }
        assert_eq!(walked, before, "{label}: inverse did not restore the before-snapshot");
    }
}

#[test]
fn every_committed_vector_is_canonical_json() {
    for case in cases() {
        let label = case.display().to_string();
        for leaf in ["📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json"] {
            let text = read(&case, leaf);
            let decoded: Grid2dSnapshot = dsl::json::from_json_str(&text).unwrap_or_else(|error| panic!("{label}/{leaf}: decodes: {error}"));
            let reencoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&decoded)).expect("re-encodes");
            let original: serde_json::Value = serde_json::from_str(&text).expect("reparses");
            assert_eq!(reencoded, original, "{label}/{leaf}: committed JSON is not canonical");
        }
    }
}
