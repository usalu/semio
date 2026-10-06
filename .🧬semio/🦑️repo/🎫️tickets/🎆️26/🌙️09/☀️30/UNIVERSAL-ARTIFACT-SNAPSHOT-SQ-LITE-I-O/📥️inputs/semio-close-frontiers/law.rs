
/// 🧭️ Genuine superseded history and retained replay prefixes reach the same bounded Semio close cursor.
#[semio_framework_async_macros::async_test]
async fn supersession_and_prefix_owners_follow_the_exact_semio_close_cursor() {
    use crate::standards::v1::subsets::value::schema::mutations::{set_snapshot::SetSnapshot, SemioValueMutation};
    use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
    let case: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/close-frontiers/🔣️.json")).expect("independent JSON close corpus");
    let document = case["document"].as_str().unwrap();
    let seed = SemioValueSnapshot::default();
    let mut envelope = dsl::create_document_envelope::<SemioValueSnapshot, SemioValueMutation>(STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, document, seed.clone(), None);
    envelope.dialect = Some(subset_dialect("value"));
    let digest = *semio_framework_hash::hash(&seed.encode_pack()).as_bytes();
    let runtime = dsl::ArtifactStoreInitializationRuntime::new(document, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA, seed, digest);
    let mut store = dsl::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, <SemioValueSnapshot as dsl::MemberStoreOwner<SemioValueMutation>>::member_store_owners());
    for value in case["values"].as_array().unwrap() {
        let snapshot = SemioValueSnapshot { root: SemioValue::Str { value: value.as_str().unwrap().into() }, ..SemioValueSnapshot::default() };
        store.apply_one(store.generation(), SemioValueMutation::SetSnapshot(SetSnapshot { snapshot }), None, dsl::HistoryLane::Document).await.expect("real registered value operation");
    }
    let ids: Vec<_> = store.mutation_ops().expect("authored operation identities").into_iter().map(|operation| operation.mutation_id).collect();
    store.dispatch(dsl::ArtifactCommand::Supersede { scope: None, inputs: vec![dsl::SupersedeInput { target: ids[case["withdraw_index"].as_u64().unwrap() as usize].clone(), replacement: None }] }).await.expect("real authored withdrawal");
    assert_eq!(store.supersessions().len(), case["expected_supersessions"].as_u64().unwrap() as usize);
    let prefix = store.state_before(&ids[case["prefix_target_index"].as_u64().unwrap() as usize], &std::collections::BTreeMap::new()).expect("real retained prefix admission");
    assert!(matches!(&prefix.root, SemioValue::Str { value } if value == case["values"][1].as_str().unwrap()));
    drop(prefix);
    let max_items = case["max_items"].as_u64().unwrap() as usize;
    let max_bytes = case["max_bytes"].as_u64().unwrap() as usize;
    let mut reached = 0;
    let phases = case["phases"].as_array().unwrap();
    for _ in 0..case["max_steps"].as_u64().unwrap() {
        let witness = store.close_owned_phase_witness();
        if reached < phases.len() && witness.starts_with(&format!("semio/{}/", phases[reached].as_str().unwrap())) { reached += 1; }
        match store.close_owned_step(max_items, max_bytes).expect("bounded genuine Semio close") {
            dsl::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= max_items && released_bytes <= max_bytes),
            dsl::SnapshotRetirementStep::Blocked => panic!("[DEBUG] unique Semio close blocked at {witness}"),
            dsl::SnapshotRetirementStep::Complete => {
                assert!(store.close_owned_terminal_is_empty());
                assert!(store.close_owned_phase_witness().starts_with("semio/Complete/"));
                assert_eq!(reached, phases.len() - 1, "every literal owner phase precedes completion");
                println!("[DEBUG] genuine supersession/prefix Semio close reached terminal under {max_items}/{max_bytes}");
                return;
            }
        }
    }
    panic!("genuine Semio close did not converge");
}
