use super::*;
use crate::{schema, WriterSnapshot};

#[semio_framework_async_macros::async_test]
async fn writer_snapshot_and_mutation_owners_retire_one_exact_field_per_grant() {
    let snapshot = crate::writer_snapshot_with_text("writer.document", "deep", "plaintext", "writer://deep", "body");
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, snapshot);
    assert_eq!(retirement.close_step(0, usize::MAX).expect("zero grant is truthful"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    let mut steps = 0;
    while !retirement.terminal_is_empty() {
        let step = retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES).expect("one Writer field retires");
        if matches!(step, store::SnapshotRetirementStep::Pending { released_items: 1, .. }) {
            steps += 1;
        }
    }
    assert_eq!(steps, 10, "five snapshot strings (schema, id, languageId, uri, text) plus five child-reference strings retire independently");
    drop(retirement);

    let hostile = WriterMutation::EditText(schema::mutations::EditText { text: "x".repeat(WRITER_ENVELOPE_FIELD_BYTES) });
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterMutationRetirementFactory, hostile);
    assert_eq!(retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES - 1).expect("under-credit preserves the exact mutation"), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES).expect("exact byte credit releases the string"), store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: WRITER_ENVELOPE_FIELD_BYTES });
    assert_eq!(retirement.close_step(1, 0).expect("empty mutation shell is shallow"), store::SnapshotRetirementStep::Complete);
    assert!(retirement.terminal_is_empty());
    drop(retirement);
}

struct UnusedWriterEditRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<protocol::Edit<WriterMutation>> for UnusedWriterEditRetirementFactory {
    fn retire_owned(&self, _value: protocol::Edit<WriterMutation>) -> Box<dyn store::ErasedSnapshotRetirement> {
        panic!("successful Writer edit fixtures take the exact decoded owner")
    }
}

fn writer_edit_source(bytes: &[u8]) -> store::OwnedSchemaRecordCursor {
    const FIELDS: &[store::OwnedSchemaFieldSpec] = &[store::OwnedSchemaFieldSpec { id: 1, key: "value", required: true }];
    let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: bytes.len() }).expect("exact Writer edit page credits");
    pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(bytes).expect("bounded Writer edit page")).unwrap_or_else(|_| panic!("pre-admitted Writer edit page"));
    pages.seal().expect("sealed Writer edit source");
    let tokens = store::OwnedSchemaTokenCursor::try_new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), pages).unwrap_or_else(|_| panic!("sealed Writer edit tokens"));
    store::OwnedSchemaRecordCursor::try_new(store::OwnedSchemaRecordSpec { fields: FIELDS }, tokens).unwrap_or_else(|_| panic!("valid Writer edit wrapper schema"))
}

/// 🧹️ Retires one entry authority (and the mutation array beneath it) to its exact terminal-empty
/// witness. Both assert in `Drop` unless they got there, and a Drop panic raised while the test is
/// already unwinding is a `panic in a destructor during cleanup` → SIGABRT that kills the whole test
/// binary — which is why every exit path of `drive_writer_edit` below goes through here, including
/// the refusal paths a malformed fixture takes.
fn retire_writer_edit_authority(authority: &mut dyn store::ArtifactOwnedHistoryEntryAuthority<protocol::Edit<WriterMutation>>) {
    for _ in 0..100_000 {
        if authority.terminal_is_empty() {
            return;
        }
        authority.close_step(1, WRITER_ENVELOPE_FIELD_BYTES).expect("a Writer edit authority retires through bounded close steps");
    }
    panic!("Writer edit authority did not reach its terminal-empty witness within its bounded close ladder")
}

fn drive_writer_edit(bytes: &[u8], cancel: semio_framework_job::CancelToken) -> Result<protocol::Edit<WriterMutation>, store::OwnedSchemaDecodeDiagnostic> {
    let mut source = writer_edit_source(bytes);
    let decoder = store::ArtifactEnvelopeOwnedFieldCatalog::<WriterSnapshot, WriterMutation>::edit_history_decoder(&WriterEnvelopeOwnedFieldCatalog);
    let mut authority = store::ArtifactOwnedHistoryEntryDecoder::begin_entry(
        decoder.as_ref(),
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        store::OwnedSchemaPath::field("value").expect("bounded Writer test path"),
        std::sync::Arc::new(UnusedWriterEditRetirementFactory),
    );
    let outcome = drive_writer_edit_tokens(&mut source, authority.as_mut(), cancel);
    retire_writer_edit_authority(authority.as_mut());
    outcome
}

fn drive_writer_edit_tokens(
    source: &mut store::OwnedSchemaRecordCursor,
    authority: &mut dyn store::ArtifactOwnedHistoryEntryAuthority<protocol::Edit<WriterMutation>>,
    cancel: semio_framework_job::CancelToken,
) -> Result<protocol::Edit<WriterMutation>, store::OwnedSchemaDecodeDiagnostic> {
    let mut pending = None;
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(3, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let field = match pending.take() {
            Some(field) => field,
            None => match source.step(&mut context) {
                store::OwnedSchemaRecordStep::Pending => continue,
                store::OwnedSchemaRecordStep::FieldToken { field_id: 1, token, terminal } => (token, terminal),
                store::OwnedSchemaRecordStep::Fault(diagnostic) => return Err(diagnostic),
                store::OwnedSchemaRecordStep::Cancelled => return Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.test-source-cancelled", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT }),
                store::OwnedSchemaRecordStep::Complete => break,
                store::OwnedSchemaRecordStep::FieldToken { .. } => unreachable!("single Writer wrapper field"),
            },
        };
        match authority.accept_token(field.0, field.1, source, &mut context) {
            Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending) => pending = Some(field),
            Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete) => {}
            Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                return authority.take_value().ok_or(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.test-value-missing", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT });
            }
            Ok(store::ArtifactEnvelopeFieldDecodeStep::RecordComplete) => unreachable!("entry authority never owns the wrapper record"),
            Err(diagnostic) => return Err(diagnostic),
        }
    }
    Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.test-did-not-complete", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
}

#[semio_framework_async_macros::async_test]
async fn writer_edit_history_decoder_uses_begin_mutation_and_faults_malformed_input() {
    let edit = protocol::Edit { line: None,
        id: "edit-1".into(),
        actor: None,
        forwards: vec![WriterMutation::RenameWriter(schema::mutations::RenameWriter { new_id: "next".into() })],
        inverse: Vec::new(),
        mutation_meta: Vec::new(),
        description: None, verb: None,
        coalesce_key: None,
        sequence_number: 1,
        started_at: "1".into(),
        finished_at: None,
    };
    let bytes = dsl::os_pack::json::to_json_string(&dsl::json!({ "value": edit })).into_bytes();
    let decoded = drive_writer_edit(&bytes, semio_framework_job::root_cancel_token()).expect("Writer owns its retained edit and mutation decoders");
    assert_eq!(decoded, edit);
    assert!(drive_writer_edit(br#"{"value":{"id":"broken","forwards":[{"mutation":"unknown","newId":"x"}],"inverse":[],"sequenceNumber":1,"startedAt":"1"}}"#, semio_framework_job::root_cancel_token()).is_err());
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterMutationRetirementFactory, decoded.forwards.into_iter().next().expect("decoded mutation owner"));
    while !retirement.terminal_is_empty() {
        retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES).expect("decoded mutation closes");
    }
}

fn empty_writer_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> WriterStoreInitializationAuthority {
    let envelope = store::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, "writer-retained-load", schema::empty_writer_snapshot(), None);
    WriterStoreInitializationAuthority::new(envelope, operation, generation)
}

fn drive_writer_initializer(authority: &mut WriterStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..10_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
        if outcome.is_terminal() {
            return outcome;
        }
    }
    panic!("Writer retained initializer did not reach a bounded terminal")
}

fn close_writer_candidate(mut candidate: store::ArtifactStore<WriterSnapshot, WriterMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<WriterSnapshot, WriterMutation>::new();
    for _ in 0..10_000 {
        match disposer.close_step(&mut candidate, 1, WRITER_ENVELOPE_FIELD_BYTES).expect("Writer candidate close step") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= WRITER_ENVELOPE_FIELD_BYTES);
            }
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh Writer candidate close unexpectedly blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh Writer candidate close unexpectedly awaited input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(disposer.terminal_is_empty(&candidate));
                drop(disposer);
                drop(candidate);
                return;
            }
        }
    }
    panic!("Writer candidate did not reach terminal-empty close")
}

#[test]
fn writer_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally() {
    let operation = semio_framework_job::OperationId(401);
    let generation = semio_framework_job::Generation(9);
    let mut authority = empty_writer_initializer(operation, generation);
    assert!(matches!(drive_writer_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Writer candidate");
    assert_eq!(candidate.generation_now(), 10);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_writer_candidate(candidate);
}

#[test]
fn writer_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
    let operation = semio_framework_job::OperationId(402);
    let generation = semio_framework_job::Generation(11);
    let mut cancelled = empty_writer_initializer(operation, generation);
    semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
    assert!(matches!(drive_writer_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let mut stale = empty_writer_initializer(operation, generation);
    let fault = drive_writer_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1));
    assert!(matches!(fault, semio_framework_job::StepOutcome::Fault(_)));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
    close_writer_step_outcome(fault);
}

/// 🧹️ A `Fault` outcome carries its detail as a retained job payload, which has no implicit Drop
/// release: it leaves only through its own one-page close ladder to the terminal-empty witness.
fn close_writer_step_outcome(mut outcome: semio_framework_job::StepOutcome) {
    for _ in 0..10_000 {
        if outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) == semio_framework_job::JobPayloadCloseStep::Complete {
            assert!(outcome.terminal_is_empty());
            return;
        }
    }
    panic!("Writer initializer outcome did not reach its terminal-empty witness")
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = WriterMutation::EditText(schema::mutations::EditText { text: "hello".into() });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

/// ✍️ Hand-built representative document — used across the artifact's own component tests.
fn jack_snapshot() -> WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
}

/// 🧬️ Reaches `jack_snapshot()` from `empty_writer_snapshot()` via the semantic vocabulary —
/// `SetSnapshot` (whole-document replace) is banned, so what used to be one mutation is now the
/// sequence of scalar mutations that actually differ between the two documents (`schema` is
/// identical in both, so it gets no mutation). `EditText` mints its `document` handle from
/// `base.id`/`base.language_id` at apply time, so it must run LAST, after `RenameWriter`/
/// `ChangeLanguage` have already landed — otherwise its handle would target the wrong owner id.
fn jack_mutations() -> Vec<WriterMutation> {
    let jack = jack_snapshot();
    let text = crate::writer_text(&jack);
    vec![
        WriterMutation::RenameWriter(schema::mutations::RenameWriter { new_id: jack.id }),
        WriterMutation::ChangeLanguage(schema::mutations::ChangeLanguage { new_language_id: jack.language_id }),
        WriterMutation::ChangeUri(schema::mutations::ChangeUri { new_uri: jack.uri }),
        WriterMutation::EditText(schema::mutations::EditText { text }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn writer_document_text_round_trips_through_the_store() {
    // 🔐️ Through the owner-installing constructor: a bare `ArtifactStore::new` installs no catalog
    // and `reserve_edit_history_slot` then refuses every `Apply`
    // (`edit history insertion requires its exact mutation retirement factory`).
    let mut store = new_writer_store(store::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, "writer", schema::empty_writer_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None, transaction: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot"), jack_snapshot());
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `WriterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
/// existing pack round-trip law.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    // 🔐️ Through the owner-installing constructor: a bare `ArtifactStore::new` installs no catalog
    // and `reserve_edit_history_slot` then refuses every `Apply`
    // (`edit history insertion requires its exact mutation retirement factory`).
    let mut store = new_writer_store(store::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, "writer", schema::empty_writer_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None, transaction: None }).await.expect("apply");
    let edit: &Edit<WriterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<WriterSnapshot, WriterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests

/// 🧯️ LAW: a Drop witness must never turn a REPORTED failure into a process abort. See the identical
/// law in `🎞️animate`'s own binary leaf for the full reasoning: a witness that fires while the thread
/// is already unwinding from a test's own failed assertion becomes a `panic in a destructor during
/// cleanup` — a NON-unwinding abort that kills the whole test binary and hides the first, real
/// failure. Every witness in this file therefore checks `std::thread::panicking()` first, the shape
/// `store::ArtifactEnvelope::drop` already uses.
///
/// This law can only pass when the guard is there: without it the panic below aborts the process
/// instead of being caught here.
#[semio_framework_async_macros::async_test]
async fn a_live_owner_dropped_during_a_panic_unwinds_instead_of_aborting() {
    let decoder = store::ArtifactEnvelopeOwnedFieldCatalog::<WriterSnapshot, WriterMutation>::edit_history_decoder(&WriterEnvelopeOwnedFieldCatalog);
    let authority = store::ArtifactOwnedHistoryEntryDecoder::begin_entry(
        decoder.as_ref(),
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        store::OwnedSchemaPath::field("value").expect("bounded Writer test path"),
        std::sync::Arc::new(UnusedWriterEditRetirementFactory),
    );
    assert!(!authority.terminal_is_empty(), "a freshly begun entry authority is NOT terminal-empty, or this law proves nothing");
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _live = authority;
        panic!("a test failure while an undrained edit authority is still live");
    }));
    std::panic::set_hook(previous);
    assert!(outcome.is_err(), "the fixture panic must reach this caller as an unwind");
}

//#region 🔖️HubTailInitialization
/// 🔁️ The hub's catch-up tail of STEP 8 (ticket 26/09/23 C12, captured from hub 7800): a replica folds each operation as a remote edit
/// named after its mutation id. Parsed from the fixture's verbatim wire fields.
fn hub_tail_envelope(value: &serde_json::Value) -> protocol::MutationEnvelope {
    let text = |node: &serde_json::Value| node.as_str().expect("hub tail fixture string").to_string();
    let bytes = |node: &serde_json::Value| node["payload"].as_array().expect("hub tail payload bytes").iter().map(|byte| u8::try_from(byte.as_u64().expect("hub tail byte")).expect("hub tail byte fits u8")).collect::<Vec<u8>>();
    let number = |node: &serde_json::Value| node.as_u64().expect("hub tail timestamp field");
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(text(&value["mutation_id"])),
        document_id: protocol::ArtifactId(text(&value["document_id"])),
        actor: protocol::ActorId(text(&value["actor"])),
        dependencies: value["dependencies"].as_array().expect("hub tail dependencies").iter().map(|id| protocol::MutationId(text(id))).collect(),
        observed: value["observed"].as_str().map(|id| protocol::MutationId(id.to_string())),
        target: value["target"].as_array().expect("hub tail target").iter().map(text).collect(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(text(&value["diff"]["schema"])), payload: bytes(&value["diff"]) },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(text(&value["inverse"]["schema"])), payload: bytes(&value["inverse"]) },
        timestamp: protocol::HybridLogicalTimestamp { actor: number(&value["timestamp"]["actor"]), physical_ms: number(&value["timestamp"]["physical_ms"]), logical: number(&value["timestamp"]["logical"]) },
        transaction: None, verb: None,
    }
}

/// 🌱️ LAW: a document a replica folded from the hub's tail — beside its own local edits, STEP 8's mixed shape — initializes again through this store initializer. Every folded operation is
/// an edit whose id IS its only operation's mutation id — one causal node, which SeedHistory used to seed twice and refuse as
/// `duplicate mutation id` (collab-e2e STEP 8: the reload after a Check In showed "Document restore failed … initializer-failed").
/// The initialized candidate must be the folded document.
#[semio_framework_async_macros::async_test]
async fn a_document_folded_from_the_hub_tail_initializes_again() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json")).expect("hub tail fixture decodes");
    let document_id = fixture["documentId"].as_str().expect("hub tail document id");
    let tail = fixture["envelopes"].as_array().expect("hub tail envelopes");
    assert_eq!(tail.len(), 17, "the captured tail: four edits, a Check In, eleven two-author edits, a second Check In");
    let mut folded = new_writer_store(store::create_document_envelope(crate::WRITER_DOCUMENT_SCHEMA, document_id, schema::empty_writer_snapshot(), None)).await.expect("valid writer store fixture");
    folded.dispatch(store::ArtifactCommand::Apply { mutations: vec![schema::mutations::edit_text("user1 typed before the hub tail".to_string())], description: None, transaction: None }).await.expect("a locally authored edit before the tail");
    for value in tail {
        folded.ingest_remote(hub_tail_envelope(value)).await.expect("every operation and transition of the hub tail folds");
    }
    folded.dispatch(store::ArtifactCommand::Apply { mutations: vec![schema::mutations::edit_text("user1 typed after the hub tail".to_string())], description: None, transaction: None }).await.expect("a locally authored edit after the tail");
    let hub_actors: std::collections::BTreeSet<&str> = tail.iter().map(|value| value["actor"].as_str().expect("hub tail actor")).collect();
    let edits = &folded.envelope().vcs.edits;
    let from_hub = edits.iter().filter(|edit| edit.actor.as_deref().is_some_and(|actor| hub_actors.contains(actor))).count();
    assert_eq!((from_hub, edits.len() - from_hub), (15, 2), "STEP 8's mixed shape: the hub's 15 operations beside this replica's own 2 edits");
    assert!(edits.iter().all(|edit| edit.mutation_meta.first().and_then(|meta| meta.mutation_id.as_ref()).is_some_and(|id| id.0 == edit.id)), "every edit — folded or authored here — is named after its own operation, the shape SeedHistory must seed once");
    let live = folded.snapshot().expect("folded writer snapshot");
    let files = store::print_document_pack(folded.envelope()).await.expect("print the folded document pack");
    let parsed: store::ParsedDocumentText<WriterSnapshot, WriterMutation> = store::parse_document_pack(&files.pack, &files.spr).await.expect("parse the folded document pack");
    let operation = semio_framework_job::OperationId(403);
    let generation = semio_framework_job::Generation(13);
    let mut authority = WriterStoreInitializationAuthority::new(parsed.into_envelope(), operation, generation);
    let outcome = drive_writer_initializer(&mut authority, operation, generation);
    assert!(matches!(outcome, semio_framework_job::StepOutcome::Complete(_)), "the folded hub tail must initialize, not fault (duplicate mutation id)");
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Writer candidate");
    assert_eq!(candidate.snapshot().expect("initialized writer snapshot"), live, "the initialized document is the folded one");
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_writer_candidate(candidate);
}
//#endregion 🔖️HubTailInitialization
