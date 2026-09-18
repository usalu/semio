//! 🧪️ Mount contract — every `#[path]` leaf this crate root mounts resolves to a real file, and
//! every mutation the dispatch enum declares owns a complete triad directory on disk. A stale mount
//! left behind by a renamed fixture directory is a compile error, never a silent hole.

use crate::mutations::KINDS;

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");

fn subset() -> std::path::PathBuf {
    std::path::Path::new(ROOT).join("🏅️standards/🔖️1/🪆️subsets/✳️any")
}

/// 🗂️ The on-disk directory name of each mutation kind, in `KINDS` order.
const DIRECTORIES: &[&str] = &[
    "🎲️change-seed",
    "📐️resize-grid",
    "📏️change-cell-size",
    "🔁️change-periodicity",
    "🌱️create-tile",
    "🗑️delete-tile",
    "⚖️change-tile-weight",
    "🎨️change-tile-media",
    "🚦️create-rule",
    "❌delete-rule",
    "📌️pin-cell",
    "📍️unpin-cell",
    "🕳️mask-cell",
    "🔳️unmask-cell",
];

#[test]
fn every_mutation_kind_owns_a_directory() {
    assert_eq!(KINDS.len(), DIRECTORIES.len());
    for (kind, directory) in KINDS.iter().zip(DIRECTORIES) {
        let root = subset().join("🧬️schema/🧬️mutations").join(directory);
        assert!(root.is_dir(), "{kind}: {} is missing", root.display());
        for leaf in ["🦀️.rs", "🔣️.json", "🧬️schema/🔣️.json", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs"] {
            assert!(root.join(leaf).is_file(), "{kind}: {leaf} is missing");
        }
    }
}

#[test]
fn every_mutation_kind_owns_at_least_one_fixture_quintet() {
    for (kind, directory) in KINDS.iter().zip(DIRECTORIES) {
        let root = subset().join("🧫️fixtures/🧬️mutations").join(directory);
        assert!(root.is_dir(), "{kind}: no fixture directory");
        let cases: Vec<_> = std::fs::read_dir(&root).expect("fixture directory reads").filter_map(Result::ok).filter(|entry| entry.path().is_dir()).collect();
        assert!(!cases.is_empty(), "{kind}: no fixture case");
        for case in cases {
            for leaf in ["📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🦠️mutation/🔣️.json", "🎯️outcome/🔣️.json", "🔺️diff/🔣️.json"] {
                assert!(case.path().join(leaf).is_file(), "{kind}/{:?}: {leaf} is missing", case.file_name());
            }
            let mounted = subset().join("🧬️schema/🧬️mutations").join(directory).join("🧪️tests").join(case.file_name()).join("🦀️.rs");
            assert!(mounted.is_file(), "{kind}/{:?}: the fixture case has no mounted test", case.file_name());
        }
    }
}

#[test]
fn every_schema_facet_states_all_five_language_leaves() {
    let schema = subset().join("🧬️schema");
    for facet in ["", "📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"] {
        let root = if facet.is_empty() { schema.clone() } else { schema.join(facet) };
        for leaf in ["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🔣️.json", "🛰️.proto"] {
            assert!(root.join(leaf).is_file(), "{facet}/{leaf} is missing");
        }
    }
}

#[test]
fn the_oracle_manifest_and_the_replay_triplet_exist() {
    assert!(subset().join("🔮️oracles/🔣️.json").is_file());
    for leaf in ["🥒️.feature", "🐍️.py", "🦀️.rs"] {
        assert!(subset().join("🧪️tests/🔲️mutate-grid2d-1").join(leaf).is_file(), "{leaf} is missing from the replay triplet");
    }
}
