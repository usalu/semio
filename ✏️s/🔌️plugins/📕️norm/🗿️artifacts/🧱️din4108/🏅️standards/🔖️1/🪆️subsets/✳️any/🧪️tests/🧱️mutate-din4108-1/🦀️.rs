//! Language-agnostic mutate suite — every mutation kind changes a leaf; inverse restores.

use crate::standards::v1::subsets::any::schema::mutations::{
    apply_din4108_mutation, decode_din4108_mutation_json, KINDS, Din4108Mutation,
};
use crate::standards::v1::subsets::any::schema::snapshot::decode_din4108_snapshot_json;
use std::path::PathBuf;

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🎫️fixtures/🧬️mutations")
}

fn kind_slug_from_dir(name: &str) -> String {
    // Strip leading non-ascii / emoji prefix until first 'change-'/'insert-'/'remove-'/'reorder-'
    for prefix in ["change-", "insert-", "remove-", "reorder-"] {
        if let Some(i) = name.find(prefix) {
            return name[i..].to_string();
        }
    }
    name.to_string()
}

#[semio_framework_async_macros::async_test]
async fn mutate_suite_every_kind_fixture_changes_leaf_and_inverse_restores() {
    let root = fixtures_root();
    assert!(root.is_dir(), "fixtures root missing: {}", root.display());
    let mut covered = std::collections::BTreeSet::new();
    for kind_dir in std::fs::read_dir(&root).expect("list kinds") {
        let kind_dir = kind_dir.expect("entry").path();
        if !kind_dir.is_dir() {
            continue;
        }
        let slug = kind_slug_from_dir(kind_dir.file_name().unwrap().to_string_lossy().as_ref());
        for case_dir in std::fs::read_dir(&kind_dir).expect("list cases") {
            let case_dir = case_dir.expect("case").path();
            let before_path = case_dir.join("📸️snapshot/⬅️before/🔣️.json");
            let after_path = case_dir.join("📸️snapshot/➡️after/🔣️.json");
            let mutation_path = case_dir.join("🦠️mutation/🔣️.json");
            if !before_path.is_file() || !after_path.is_file() || !mutation_path.is_file() {
                continue;
            }
            let before = decode_din4108_snapshot_json(&std::fs::read_to_string(&before_path).unwrap()).expect("before");
            let after_expected = decode_din4108_snapshot_json(&std::fs::read_to_string(&after_path).unwrap()).expect("after");
            let mutation = decode_din4108_mutation_json(&std::fs::read_to_string(&mutation_path).unwrap()).expect("mutation");
            assert_ne!(before, after_expected, "fixture after must differ from before ({})", case_dir.display());
            let (after_got, _) = apply_din4108_mutation(&before, &mutation).expect("apply");
            assert_eq!(after_got, after_expected, "apply mismatch in {}", case_dir.display());
            let restore_ops = Din4108Mutation::from_snapshot(&after_got, &before);
            let mut restored = after_got;
            for inv in &restore_ops {
                let (next, _) = apply_din4108_mutation(&restored, inv).expect("restore apply");
                restored = next;
            }
            assert_eq!(restored, before, "restore did not reinstate before for {}", case_dir.display());
            covered.insert(slug.clone());
        }
    }
    for kind in KINDS {
        assert!(
            covered.contains(*kind),
            "missing language-agnostic fixture coverage for mutation kind {kind}; covered={covered:?}"
        );
    }
    assert_eq!(covered.len(), KINDS.len());
}
