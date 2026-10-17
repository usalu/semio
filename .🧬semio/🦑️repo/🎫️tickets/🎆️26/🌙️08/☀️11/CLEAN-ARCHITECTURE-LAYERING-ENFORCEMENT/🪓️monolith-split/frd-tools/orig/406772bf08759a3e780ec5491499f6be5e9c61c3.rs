//! 🧫️ DIN V 18599's committed mutation vectors (`🧪️tests/🧫️fixtures/🧬️mutations`) through production dispatch; the law table is
//! the shared `semio_s_mutation_vectors`.

#[semio_framework_async_macros::async_test]
async fn committed_vectors_hold() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🧪️tests/🧫️fixtures/🧬️mutations");
    semio_s_mutation_vectors::assert_committed_vectors::<semio_s_artifact_norm_din18599_model::Din18599Snapshot, semio_s_artifact_norm_din18599_mutations::Din18599Mutation>(&root, || {}).await;
}
