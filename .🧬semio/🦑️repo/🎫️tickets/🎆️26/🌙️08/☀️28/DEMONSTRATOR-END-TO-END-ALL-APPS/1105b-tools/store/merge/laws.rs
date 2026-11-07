
//#region 🧪️PairMergeCursorLaws
/// 🧵️ The scenario of the pair-merge law: replica A (own alternative, uncommitted edit there) and the pair B wrote after editing the trunk.
async fn cursor_scenario(identity: &mut semio_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (DemoStore, ArtifactPackFiles) {
    let mut a = fresh("pair-merge", Some(0)).await;
    apply(&mut a, vec![DemoMutation::SetN(SetN { n: 1 })], identity).await;
    apply(&mut a, vec![DemoMutation::AddN(AddN { delta: 2 })], identity).await;
    let mut b = fresh("pair-merge", Some(0)).await;
    deliver(&mut b, a.event_log().expect("log")).await;
    let first = a.mutation_ops().expect("applied operations")[0].mutation_id.clone();
    test_support::dispatch_test_command(&mut a, ArtifactCommand::CreateAlternativeWithSupersede { name: "mine".into(), inputs: vec![SupersedeInput { target: first, replacement: Some(DemoMutation::SetN(SetN { n: 10 })) }] }).await.expect("a history edit as a new alternative");
    apply(&mut a, vec![DemoMutation::AddN(AddN { delta: 5 })], identity).await;
    apply(&mut b, vec![DemoMutation::AddN(AddN { delta: 100 })], identity).await;
    let written = print_document_pack(b.envelope()).await.expect("b's pair prints");
    (a, written)
}

async fn cursor_open(store: &DemoStore, pair: &ArtifactPackFiles) -> PersistedMergeCursor {
    let history = semio_os_spr::decode_history(&pair.spr, &semio_os_spr::DecodeOptions::default()).await.expect("the pair's history decodes");
    store.persisted_merge_open(&pair.pack, history).map_err(|(error, _)| error).expect("the pair is this document's own")
}

/// 🪜️ Drives a cursor to completion with a fixed per-step budget, answering the outcome and every step's reported work.
async fn cursor_drive(store: &mut DemoStore, cursor: &mut PersistedMergeCursor, budget: u64) -> (PairMerge, Vec<u64>) {
    let mut works = Vec::new();
    loop {
        let turn = store.persisted_merge_step(cursor, budget).await;
        let progress = turn.result.expect("a clean merge step");
        works.push(turn.work);
        assert!(turn.work > 0 || progress.phase == PersistedMergePhase::Complete, "every step that does not finish performs work: forward progress");
        if progress.phase == PersistedMergePhase::Complete { return (cursor.outcome(), works) }
        assert!(works.len() < 100_000, "the merge must terminate");
    }
}

fn event_ids(store: &DemoStore) -> Vec<String> {
    let mut ids: Vec<String> = store.event_log().expect("log").into_iter().map(|event| event.mutation_id.0).collect();
    ids.sort();
    ids
}

/// 🧲️ LAW: chunking never changes the merge. Whatever the per-step budget, the cursor takes exactly the events the one-call merge
/// takes, reports the same `PairMerge`, and leaves the same event log; a bigger budget never needs more steps.
#[semio_framework_async_macros::async_test]
async fn a_chunked_persisted_merge_equals_the_one_call_merge_for_every_budget() {
    const CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= CEILING); true };
    let mut identity = semio_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING, &mut observer).expect("identity authority");
    let (mut oracle, pair) = cursor_scenario(&mut identity).await;
    let expected = oracle.merge_persisted_pair(&pair.pack, &pair.spr).await.expect("the one-call merge");
    let mut steps = Vec::new();
    for budget in [1_u64, 2, 5, 13, 89, u64::MAX] {
        let (mut replica, _) = cursor_scenario(&mut identity).await;
        let mut cursor = cursor_open(&replica, &pair).await;
        let (outcome, works) = cursor_drive(&mut replica, &mut cursor, budget).await;
        assert_eq!(outcome, expected, "budget {budget}");
        assert_eq!(event_ids(&replica), event_ids(&oracle), "budget {budget}: same events");
        assert!(works.iter().all(|work| *work <= budget.max(PERSISTED_MERGE_PRICING.ingest_base + 4 * PERSISTED_MERGE_PRICING.decode_base + 64)), "budget {budget}: a step never overspends beyond its first indivisible unit: {works:?}");
        test_support::assert_live_equals_replay(&replica).await;
        steps.push(works.len());
    }
    assert!(steps.windows(2).all(|pair| pair[0] >= pair[1]), "a bigger budget never needs more steps: {steps:?}");
    assert_eq!(*steps.last().expect("steps"), 1, "an unbounded budget finishes in one step");
}

/// 🧲️ LAW: a merge cancelled between steps leaves exactly the prefix it took, the cursor retires without touching the store, and a
/// fresh cursor over the same pair completes to the one-call result (every ingest is atomic and idempotent).
#[semio_framework_async_macros::async_test]
async fn a_cancelled_persisted_merge_keeps_its_prefix_and_a_fresh_cursor_completes_it() {
    const CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= CEILING); true };
    let mut identity = semio_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING, &mut observer).expect("identity authority");
    let (mut oracle, pair) = cursor_scenario(&mut identity).await;
    oracle.merge_persisted_pair(&pair.pack, &pair.spr).await.expect("the one-call merge");
    let (mut replica, _) = cursor_scenario(&mut identity).await;
    let mut cursor = cursor_open(&replica, &pair).await;
    for _ in 0..4 { let turn = replica.persisted_merge_step(&mut cursor, 1).await; turn.result.expect("a clean merge step"); }
    let taken = cursor.outcome().merged;
    drop(cursor);
    test_support::assert_live_equals_replay(&replica).await;
    let mut fresh_cursor = cursor_open(&replica, &pair).await;
    let (outcome, _) = cursor_drive(&mut replica, &mut fresh_cursor, 7).await;
    assert_eq!(taken + outcome.merged, 1, "the prefix and the rest add up to the one event the pair offered");
    assert_eq!(event_ids(&replica), event_ids(&oracle));
}

/// 🧲️ LAW: a pair of another document is refused at open, nothing is parsed, and the history comes back to its owner.
#[semio_framework_async_macros::async_test]
async fn a_persisted_merge_of_another_document_is_refused_at_open_and_returns_its_history() {
    const CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= CEILING); true };
    let mut identity = semio_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING, &mut observer).expect("identity authority");
    let (replica, _) = cursor_scenario(&mut identity).await;
    let mut other = fresh("another-document", Some(0)).await;
    apply(&mut other, vec![DemoMutation::SetN(SetN { n: 3 })], &mut identity).await;
    let pair = print_document_pack(other.envelope()).await.expect("the other pair prints");
    let history = semio_os_spr::decode_history(&pair.spr, &semio_os_spr::DecodeOptions::default()).await.expect("history");
    let doc = history.doc_id.clone();
    match replica.persisted_merge_open(&pair.pack, history) {
        Err((VcsError::ValidationFailed(_), returned)) => assert_eq!(returned.doc_id, doc),
        _ => panic!("another document is not merged"),
    }
}

/// 🧲️ LAW: the store may change between steps. A local edit made mid-merge neither duplicates nor loses events, and the log still replays to the live state.
#[semio_framework_async_macros::async_test]
async fn a_local_edit_between_persisted_merge_steps_is_kept_and_nothing_is_duplicated() {
    const CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= CEILING); true };
    let mut identity = semio_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING, &mut observer).expect("identity authority");
    let (mut replica, pair) = cursor_scenario(&mut identity).await;
    let mut cursor = cursor_open(&replica, &pair).await;
    replica.persisted_merge_step(&mut cursor, 1).await.result.expect("a clean merge step");
    apply(&mut replica, vec![DemoMutation::AddN(AddN { delta: 7 })], &mut identity).await;
    let (outcome, _) = cursor_drive(&mut replica, &mut cursor, 3).await;
    assert_eq!(outcome.merged, 1);
    let ids = event_ids(&replica);
    let mut unique = ids.clone();
    unique.dedup();
    assert_eq!(ids, unique, "no event is held twice");
    test_support::assert_live_equals_replay(&replica).await;
}

/// 🧲️ LAW: the Rust price constant is the authored price list.
#[test]
fn the_persisted_merge_price_constant_is_the_authored_price_list() {
    let authored: serde_json::Value = serde_json::from_str(include_str!("../../⚙️engine/📥️lifted/🏪️store/🔀️persisted-merge-cursor/💱️pricing.json")).expect("pricing json");
    let field = |name: &str| authored[name].as_u64().expect(name);
    let price = PERSISTED_MERGE_PRICING;
    assert_eq!((price.decode_base, price.decode_per_operation, price.census_per_id, price.order_per_comparison, price.ingest_base, price.ingest_per_dependency), (field("decodeBase"), field("decodePerOperation"), field("censusPerId"), field("orderPerComparison"), field("ingestBase"), field("ingestPerDependency")));
    assert_eq!(price.preflight_decode, authored["preflightDecode"].as_bool().expect("preflightDecode"));
}
//#endregion 🧪️PairMergeCursorLaws
