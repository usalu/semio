use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips_the_reference_fixture() {
    store::os_store::test_support::assert_dsl_round_trip(&crate::reference_fixture());
}

#[semio_framework_async_macros::async_test]
async fn bundled_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(REFERENCE_EXAMPLE_TEXT).expect("parse bundled example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}


#[semio_framework_async_macros::async_test]
async fn regenerate_example_dsl_assets_when_env_set() {
    if std::env::var_os("SEMIO_VDI_REGEN").is_none() {
        return;
    }
    use crate::standards::v1::subsets::any::schema::snapshot::encode_vdi3805_dsl;
    use crate::{conforming_valve_dataset, nonconforming_valve_dataset};
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets = root.join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
    let demo = assets.join("🎬️demo/🗣️.dsl.semio");
    let non = assets.join("🌶️nonconforming/🗣️.dsl.semio");
    std::fs::write(&demo, encode_vdi3805_dsl(&conforming_valve_dataset())).expect("write demo dsl");
    std::fs::write(&non, encode_vdi3805_dsl(&nonconforming_valve_dataset())).expect("write nonconforming dsl");
    eprintln!("[DEBUG] regenerated {} and {}", demo.display(), non.display());
}

#[semio_framework_async_macros::async_test]
async fn regenerate_mutation_fixtures_when_env_set() {
    if std::env::var_os("SEMIO_VDI_REGEN_FIXTURES").is_none() {
        return;
    }
    use crate::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
    use protocol::{Mutation, MutationDiff};
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixtures = root.join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let mut regenerated = 0usize;
    for kind_dir in std::fs::read_dir(&fixtures).expect("mutations fixtures") {
        let kind_dir = kind_dir.expect("kind entry").path();
        if !kind_dir.is_dir() {
            continue;
        }
        // `change-limits` is unmounted (import-only SecurityLimits — not snapshot state).
        if kind_dir.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.contains("change-limits")) {
            continue;
        }
        for case_dir in std::fs::read_dir(&kind_dir).expect("case dirs") {
            let case_dir = case_dir.expect("case entry").path();
            if !case_dir.is_dir() {
                continue;
            }
            let before_path = case_dir.join("📸️snapshot/⬅️before/🔣️.json");
            let after_path = case_dir.join("📸️snapshot/➡️after/🔣️.json");
            let mutation_path = case_dir.join("🦠️mutation/🔣️.json");
            let diff_path = case_dir.join("🔺️diff/🔣️.json");
            if !before_path.is_file() || !mutation_path.is_file() {
                continue;
            }
            let before_text = std::fs::read_to_string(&before_path).expect("read before");
            let mut before: Vdi3805Snapshot = serde_json::from_str(&before_text).unwrap_or_else(|e| panic!("decode before {}: {e}", before_path.display()));
            // Canonicalize before
            let before_canon = serde_json::to_string_pretty(&before).expect("encode before") + "\n";
            std::fs::write(&before_path, &before_canon).expect("write before");
            before = serde_json::from_str(&before_canon).expect("redecode before");

            let mutation_text = std::fs::read_to_string(&mutation_path).expect("read mutation");
            let mutation: Vdi3805Mutation = serde_json::from_str(&mutation_text).unwrap_or_else(|e| panic!("decode mutation {}: {e}", mutation_path.display()));
            let mutation_canon = serde_json::to_string_pretty(&mutation).expect("encode mutation") + "\n";
            std::fs::write(&mutation_path, &mutation_canon).expect("write mutation");

            let raised = Mutation::<Vdi3805Snapshot>::diff(&mutation, &before);
            let diff = raised.diff().clone();
            let after = MutationDiff::<Vdi3805Snapshot>::apply(&diff, &before).unwrap_or_else(|e| panic!("apply {}: {e}", case_dir.display()));
            let diff_canon = serde_json::to_string_pretty(&diff).expect("encode diff") + "\n";
            let after_canon = serde_json::to_string_pretty(&after).expect("encode after") + "\n";
            std::fs::write(&diff_path, &diff_canon).expect("write diff");
            std::fs::write(&after_path, &after_canon).expect("write after");
            regenerated += 1;
            eprintln!("[DEBUG] regenerated fixture {}", case_dir.display());
        }
    }
    eprintln!("[DEBUG] regenerated {regenerated} mutation fixtures");
    assert!(regenerated > 0, "expected to regenerate at least one fixture");
}
