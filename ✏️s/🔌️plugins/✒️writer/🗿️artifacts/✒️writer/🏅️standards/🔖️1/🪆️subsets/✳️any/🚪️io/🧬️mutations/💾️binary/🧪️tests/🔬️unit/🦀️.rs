
use super::*;
use crate::{WriterSnapshot, schema};

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
    assert_eq!(steps, 9, "four snapshot strings plus five child-reference strings retire independently");
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

fn drive_writer_edit(bytes: &[u8], cancel: semio_framework_job::CancelToken) -> Result<protocol::Edit<WriterMutation>, store::OwnedSchemaDecodeDiagnostic> {
    let mut source = writer_edit_source(bytes);
    let catalog: std::sync::Arc<dyn store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation>> = std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog);
    let decoder = WriterEditHistoryDecoder { catalog };
    let mut authority = store::ArtifactOwnedHistoryEntryDecoder::begin_entry(
        &decoder,
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        store::OwnedSchemaPath::field("value").expect("bounded Writer test path"),
        std::sync::Arc::new(UnusedWriterEditRetirementFactory),
    );
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
        match authority.accept_token(field.0, field.1, &source, &mut context) {
            Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending) => pending = Some(field),
            Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete) => {}
            Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                return authority.take_value().ok_or(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.test-value-missing", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT });
            }
            Ok(store::ArtifactEnvelopeFieldDecodeStep::RecordComplete) => unreachable!("entry authority never owns the wrapper record"),
            Err(diagnostic) => {
                while !authority.terminal_is_empty() {
                    authority.close_step(1, WRITER_ENVELOPE_FIELD_BYTES)?;
                }
                return Err(diagnostic);
            }
        }
    }
    Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.test-did-not-complete", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
}

#[semio_framework_async_macros::async_test]
async fn writer_edit_history_decoder_uses_begin_mutation_and_faults_malformed_input() {
    let edit = protocol::Edit {
        id: "edit-1".into(),
        actor: None,
        forwards: vec![WriterMutation::RenameWriter(schema::mutations::RenameWriter { new_id: "next".into() })],
        inverse: Vec::new(),
        mutation_meta: Vec::new(),
        description: None,
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
    assert!(matches!(drive_writer_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), semio_framework_job::StepOutcome::Fault(_)));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
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
    let mut store = store::ArtifactStore::<WriterSnapshot, WriterMutation>::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None }).await.expect("apply");
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

    let mut store = store::ArtifactStore::<WriterSnapshot, WriterMutation>::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None }).await.expect("apply");
    let edit: &Edit<WriterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
    store::os_store::test_support::assert_command_envelope_round_trip::<WriterSnapshot, WriterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}
//#endregion 🔖️CommandEnvelopeTests
