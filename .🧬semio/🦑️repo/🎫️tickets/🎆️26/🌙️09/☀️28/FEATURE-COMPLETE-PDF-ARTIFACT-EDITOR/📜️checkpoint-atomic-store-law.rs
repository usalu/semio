/// 🧾 Checks that Check In preserves the one original media edit and its later ordinary Undo.
#[semio_framework_async_macros::async_test]
async fn check_in_replay_preserves_one_authored_media_edit_and_its_undo() {
    use semio_os_spr::atomic_hub::{AtomicWalEdit, CheckpointReplayGroup, decode_checkpoint_replay, encode_checkpoint_replay};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️check-in-original-image.json")).unwrap();
    let schema = fixture["schema"].as_str().unwrap();
    let document = fixture["document"].as_str().unwrap();
    let baseline = fixture["baseline"].as_i64().unwrap() as i32;
    let codec = artifact_codec::<DemoSnapshot, DemoMutation>(schema);
    let replay = codec.reference().expect("authored cold codec reference").replay_envelopes;
    let genesis = create_document_envelope::<DemoSnapshot, DemoMutation>(
        schema, document, DemoSnapshot { n: Some(baseline) }, None,
        &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    );
    let pair = print_document_pack(&genesis).await.expect("genesis checkpoint");
    drop(genesis.into_owners());

    let mut author = fresh_demo_store().await;
    let mutations = fixture["values"].as_array().unwrap().iter()
        .map(|value| DemoMutation::SetN(SetN { n: value.as_i64().unwrap() as i32 })).collect();
    let transaction = semio_os_spr::TransactionRef {
        id: fixture["transaction"]["id"].as_str().unwrap().into(),
        tool: fixture["transaction"]["tool"].as_str().unwrap().into(),
    };
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Apply { mutations, transaction: Some(transaction) })
        .await.expect("original authored edit commits at the local Store");
    let original = author.event_log().expect("original event rows");
    let edit_id = author.envelope().vcs.edits.iter().next().unwrap().id.clone();
    let group = AtomicWalEdit { edit_id: edit_id.clone(), rows: original.len() };
    assert_eq!(original.len(), fixture["expected"]["rows"].as_u64().unwrap() as usize);

    let packed_group = encode_checkpoint_replay([(Some(&group), original.as_slice())]).unwrap();
    let draft = &fixture["draft"];
    let local_value = draft["value"].as_i64().unwrap() as i32;
    let transaction_id = draft["transactionId"].as_str().unwrap();
    let receiver_seed = create_document_envelope::<DemoSnapshot, DemoMutation>(
        schema, document, DemoSnapshot { n: Some(baseline) }, None,
        &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    );
    let actor = draft["actor"].as_str().unwrap();
    assert_ne!(author.local_actor_id().0.as_str(), actor);
    let mut receiver = ArtifactStore::new_with_actor(receiver_seed, semio_os_spr::ActorId(actor.into()),
        &DemoMemberStoreProducer::<DemoMutation>::new()).await;
    test_support::dispatch_test_command(&mut receiver, ArtifactCommand::AppendTransaction {
        mutations: vec![DemoMutation::SetN(SetN { n: local_value })],
        transaction: semio_os_spr::TransactionRef { id: transaction_id.into(), tool: "fixture.draft".into() },
    }).await.unwrap();
    let local_edit = receiver.envelope().open_transaction.as_ref().unwrap().edit_id.clone();
    let draft_generation = receiver.generation();
    let draft_revision = receiver.content_revision_now();

    let receive = |stream: &[u8]| {
        let mut commits = decode_checkpoint_replay(stream, document).unwrap().unwrap();
        assert_eq!(commits.len(), 1);
        let CheckpointReplayGroup::Original(group) = commits.pop().unwrap() else { panic!("original edit framing") };
        group
    };
    let mut waiting = original.clone();
    waiting[0].dependencies.push(semio_os_spr::MutationId(draft["missingDependency"].as_str().unwrap().into()));
    let gap = encode_checkpoint_replay([(Some(&group), waiting.as_slice())]).unwrap();
    assert!(matches!(receiver.ingest_original_atomic(&receive(&gap)).await.unwrap(),
        semio_os_store_engine::AtomicStoreReceive::AwaitingDependencies));
    assert_eq!(receiver.generation() - draft_generation, draft["expected"]["gapRevision"].as_u64().unwrap());
    assert_eq!(receiver.content_revision_now(), draft_revision);
    assert_eq!(receiver.snapshot().unwrap().n, Some(draft["expected"]["gapValue"].as_i64().unwrap() as i32));
    assert_eq!(receiver.envelope().open_transaction.as_ref().unwrap().edit_id, local_edit);
    assert!(!receiver.replay_edits().iter().any(|edit| edit.id == edit_id));

    assert!(matches!(receiver.ingest_original_atomic(&receive(&packed_group)).await.unwrap(),
        semio_os_store_engine::AtomicStoreReceive::Finished(report) if report.accepted));
    assert_eq!(receiver.envelope().open_transaction.as_ref().unwrap().edit_id, local_edit);
    assert_eq!(receiver.snapshot().unwrap().n, Some(draft["expected"]["mergedDraft"].as_i64().unwrap() as i32));
    test_support::dispatch_test_command(&mut receiver, ArtifactCommand::AbortTransaction { transaction_id: transaction_id.into() }).await.unwrap();
    assert!(receiver.envelope().open_transaction.is_none());
    assert_eq!(receiver.snapshot().unwrap().n, Some(draft["expected"]["abortedDraft"].as_i64().unwrap() as i32));
    assert_eq!(receiver.replay_edits().iter().filter(|edit| edit.id == edit_id).count(),
        draft["expected"]["committedEdits"].as_u64().unwrap() as usize);
    close_test_store(&mut receiver);

    let committed = replay(&pair.pack, &pair.spr, &packed_group).await.expect("one verified authored edit replays");
    let history = semio_os_spr::decode_history(&committed.1, &semio_os_spr::DecodeOptions::default()).await.unwrap();
    assert_eq!(history.edits.len(), fixture["expected"]["edits"].as_u64().unwrap() as usize);
    assert_eq!(history.edits.iter().next().unwrap().id, edit_id);
    let after = parse_document_pack::<DemoSnapshot, DemoMutation>(&committed.0, &committed.1,
        &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap();
    assert_eq!(after.into_snapshot().n, Some(fixture["expected"]["after"].as_i64().unwrap() as i32));

    let flat = semio_os_spr::encode_envelopes(&original);
    assert!(replay(&pair.pack, &pair.spr, &flat).await.is_err(), "original media rows cannot replay without the authored group");

    test_support::dispatch_test_command(&mut author, ArtifactCommand::Undo).await.expect("authored Undo");
    let ledger = author.event_log().expect("original plus ordinary history transition");
    let tail = &ledger[original.len()..];
    let full_wire = encode_checkpoint_replay([(Some(&group), original.as_slice()), (None, tail)]).unwrap();
    let full = replay(&pair.pack, &pair.spr, &full_wire).await.expect("group and Undo replay in ledger order");
    let resumed = replay(&committed.0, &committed.1, &semio_os_spr::encode_envelopes(tail)).await.unwrap();
    assert_eq!((full.0.as_slice(), full.1.as_slice()), (resumed.0.as_slice(), resumed.1.as_slice()));
    let undone = parse_document_pack::<DemoSnapshot, DemoMutation>(&full.0, &full.1,
        &semio_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap();
    assert_eq!(undone.into_snapshot().n, Some(fixture["expected"]["undo"].as_i64().unwrap() as i32));
    close_test_store(&mut author);
}
