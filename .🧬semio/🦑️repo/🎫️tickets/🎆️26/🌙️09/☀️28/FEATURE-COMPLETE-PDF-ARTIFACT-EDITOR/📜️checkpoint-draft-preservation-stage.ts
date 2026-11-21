/** 🪢 Adds the open-transaction and causal-gap case to the linked Check In law. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const fixtureFile = join(import.meta.dir, "🔣️checkpoint-atomic-store-law.json");
const oracleFile = join(import.meta.dir, "📜️checkpoint-atomic-store-oracle.py");
const lawFile = join(import.meta.dir, "📜️checkpoint-atomic-store-law.rs");
const fixture = JSON.parse(readFileSync(fixtureFile, "utf8"));
if (fixture.draft) throw Error("The local draft law is already staged");
fixture.draft = { transactionId: "checkpoint-local-draft", actor: "checkpoint-local-device", missingDependency: "unknown-authored-predecessor", value: 19,
    expected: { gapValue: 19, gapRevision: 0, mergedDraft: 19, abortedDraft: 7, committedEdits: 1 } };

const anchor = '    let committed = replay(&pair.pack, &pair.spr, &packed_group).await.expect("one verified authored edit replays");';
const test = readFileSync(lawFile, "utf8");
if (test.split(anchor).length !== 2) throw Error("The Check In law changed");
const checked = test.replace("use semio_os_spr::atomic_hub::{AtomicWalEdit, encode_checkpoint_replay};",
    "use semio_os_spr::atomic_hub::{AtomicWalEdit, CheckpointReplayGroup, decode_checkpoint_replay, encode_checkpoint_replay};")
    .replace(anchor, `    let draft = &fixture["draft"];
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

` + anchor);

const oracle = readFileSync(oracleFile, "utf8") + `
draft = fixture["draft"]
connection.execute("CREATE TABLE local_transaction (value INTEGER NOT NULL, revision INTEGER NOT NULL)")
connection.execute("INSERT INTO local_transaction VALUES (?, 0)", (draft["value"],))
gap_value, gap_revision = connection.execute("SELECT value, revision FROM local_transaction").fetchone()
merged_draft = connection.execute("""
SELECT coalesce((SELECT value FROM local_transaction),
                (SELECT sample FROM checkpoint_ledger ORDER BY sequence DESC LIMIT 1))
""").fetchone()[0]
connection.execute("DELETE FROM local_transaction")
aborted_draft, committed_edits = connection.execute("""
SELECT (SELECT sample FROM checkpoint_ledger ORDER BY sequence DESC LIMIT 1),
       count(DISTINCT authored_edit) FROM checkpoint_ledger
""").fetchone()
draft_reply = dict(zip(("gapValue", "gapRevision", "mergedDraft", "abortedDraft", "committedEdits"),
                       (gap_value, gap_revision, merged_draft, aborted_draft, committed_edits)))
assert draft_reply == draft["expected"], draft_reply
print("[DEBUG] SQLite verified the retained draft", draft_reply)
connection.close()
`;
writeFileSync(fixtureFile, JSON.stringify(fixture, null, 2) + "\n");
writeFileSync(lawFile, checked);
writeFileSync(oracleFile, oracle);
