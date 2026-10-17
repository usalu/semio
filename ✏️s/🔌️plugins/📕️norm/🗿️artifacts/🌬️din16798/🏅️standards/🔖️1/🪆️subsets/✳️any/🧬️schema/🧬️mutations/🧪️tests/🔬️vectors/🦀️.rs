//! 🧫️ DIN EN 16798's committed mutation vectors (`🧪️tests/🧫️fixtures/🧬️mutations`) through production dispatch; the law table is
//! the shared `semio_s_mutation_vectors`.

use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn committed_vectors_hold() {
    let root = fixture_path!("../../../../🧪️tests/🧫️fixtures/🧬️mutations");
    semio_s_mutation_vectors::assert_committed_vectors::<semio_s_artifact_norm_din16798_model::Din16798Snapshot, semio_s_artifact_norm_din16798_mutations::Din16798Mutation>(&root, || {}).await;
}
