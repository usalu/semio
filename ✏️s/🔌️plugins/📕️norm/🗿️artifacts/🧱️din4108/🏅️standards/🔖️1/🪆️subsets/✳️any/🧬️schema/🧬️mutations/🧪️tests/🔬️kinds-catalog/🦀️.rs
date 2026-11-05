use crate::KINDS;

#[semio_framework_async_macros::async_test]
async fn kinds_are_unique_and_non_empty() {
    assert!(!KINDS.is_empty());
    let mut seen = std::collections::BTreeSet::new();
    for kind in KINDS {
        assert!(seen.insert(*kind), "duplicate {kind}");
    }
    semio_s_mutation_vectors::assert_mutation_catalogue(KINDS, &semio_framework_test_fixtures::fixture_path!("../../../../🔮️oracles/🔣️.json"));

}
