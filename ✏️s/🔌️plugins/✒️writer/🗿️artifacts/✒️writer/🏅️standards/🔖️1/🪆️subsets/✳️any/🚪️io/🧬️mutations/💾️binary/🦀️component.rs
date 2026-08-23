//! ⚖️ Writer artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! This component only carries the artifact-facing `encode_op`/`decode_op` wrappers plus the op
//! text↔binary equivalence law and a whole-store round trip. The app's typed `WriterCommand` enum —
//! which used to share the old `📡️protocol` crate with this codec — is an EDITOR-surface concern, not
//! an artifact one: it now lives in the subset's `✏️editor/🦀️component.rs`, assembled from the
//! `🎮️commands/*` payload
//! modules by `semio_framework_plugin::app_commands!`.

use crate::artifacts::writer::op::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use protocol::{Mutation, MutationDiff, OpBinary};

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `WriterMutation` to its binary state-patch form.
pub async fn encode_op(operation: &WriterMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WriterMutation` from its binary state-patch form.
pub async fn decode_op(bytes: &[u8]) -> Result<WriterMutation, protocol::ProtocolError> {
    WriterMutation::decode_op(bytes)
}

//#region 🔖️OwnedEnvelopeCatalog
const WRITER_ENVELOPE_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

struct WriterSnapshotRetirement {
    value: std::mem::ManuallyDrop<Option<WriterSnapshot>>,
    phase: u8,
}

impl WriterSnapshotRetirement {
    fn take_field(value: &mut WriterSnapshot, phase: u8) -> &mut String {
        match phase {
            0 => &mut value.schema,
            1 => &mut value.id,
            2 => &mut value.language_id,
            3 => &mut value.uri,
            4 => &mut value.document.child_id,
            5 => &mut value.document.target.artifact_id,
            6 => &mut value.document.target.dialect.artifact_kind,
            7 => &mut value.document.target.dialect.standard,
            8 => &mut value.document.target.dialect.subset,
            _ => unreachable!("Writer snapshot retirement phase is validated"),
        }
    }
}

impl store::ErasedSnapshotRetirement for WriterSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        let Some(value) = self.value.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        if self.phase < 9 {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let field = Self::take_field(value, self.phase);
            if field.len() > maximum_bytes {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = field.len();
            drop(std::mem::take(field));
            self.phase += 1;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        drop(self.value.take());
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for WriterSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.value.is_none(), "Writer snapshot retirement reached Drop before every exact string and child-handle owner was terminal-empty");
    }
}

pub struct WriterSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<WriterSnapshot> for WriterSnapshotRetirementFactory {
    fn retire_owned(&self, value: WriterSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(WriterSnapshotRetirement { value: std::mem::ManuallyDrop::new(Some(value)), phase: 0 })
    }
}

struct WriterSnapshotRootRetirement {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<WriterSnapshot>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for WriterSnapshotRootRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(maximum_items.min(1), maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err("Writer snapshot root retirement reported Complete without terminal-empty authority".into()),
                step => Ok(step),
            };
        }
        let Some(owner) = self.owner.take() else { return Ok(store::SnapshotRetirementStep::Complete) };
        match std::sync::Arc::try_unwrap(owner) {
            Ok(value) => {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, value));
                Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
            }
            Err(owner) => {
                *self.owner = Some(owner);
                Ok(store::SnapshotRetirementStep::Blocked)
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterSnapshotRootRetirement {
    fn drop(&mut self) {
        assert!(self.owner.is_none() && self.retirement.is_none(), "Writer snapshot root retirement reached Drop before exact Arc handback and bounded field disposal");
    }
}

impl store::SnapshotRetirementFactory<WriterSnapshot> for WriterSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<WriterSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(WriterSnapshotRootRetirement { owner: std::mem::ManuallyDrop::new(Some(snapshot)), retirement: std::mem::ManuallyDrop::new(None) })
    }
}

struct WriterMutationRetirement {
    value: std::mem::ManuallyDrop<Option<WriterMutation>>,
    field_released: bool,
}

impl store::ErasedSnapshotRetirement for WriterMutationRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        let Some(value) = self.value.as_mut() else { return Ok(store::SnapshotRetirementStep::Complete) };
        if !self.field_released {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let field = match value {
                WriterMutation::RenameWriter(value) => &mut value.new_id,
                WriterMutation::ChangeUri(value) => &mut value.new_uri,
                WriterMutation::ChangeLanguage(value) => &mut value.new_language_id,
                WriterMutation::EditText(value) => &mut value.text,
            };
            if field.len() > maximum_bytes {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = field.len();
            drop(std::mem::take(field));
            self.field_released = true;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        drop(self.value.take());
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for WriterMutationRetirement {
    fn drop(&mut self) {
        assert!(self.value.is_none(), "Writer mutation retirement reached Drop before its exact string owner was terminal-empty");
    }
}

pub struct WriterMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<WriterMutation> for WriterMutationRetirementFactory {
    fn retire_owned(&self, value: WriterMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(WriterMutationRetirement { value: std::mem::ManuallyDrop::new(Some(value)), field_released: false })
    }
}

enum WriterSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<WRITER_ENVELOPE_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct WriterSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: WriterSnapshotDecodeState,
    value: std::mem::ManuallyDrop<Option<WriterSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl WriterSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: WriterSnapshotDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot> for WriterSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path };
        if matches!(self.state, WriterSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("writer-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = WriterSnapshotDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let WriterSnapshotDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("writer-envelope.snapshot-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("writer-envelope.snapshot-pack-missing", token.start))?;
                let value = <WriterSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("writer-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Writer snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = WriterSnapshotDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("writer-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<WriterSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, WriterSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("writer-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("writer-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = WriterSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let WriterSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = WriterSnapshotDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, value));
                self.state = WriterSnapshotDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = WriterSnapshotDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let path = self.path;
        let retirement = self.retirement.as_mut().expect("Writer snapshot retirement remains retained");
        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.snapshot-retirement-fault", offset: 0, line: 0, column: 0, path })? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = WriterSnapshotDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(self.diagnostic("writer-envelope.snapshot-retirement-false-terminal", 0)),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, WriterSnapshotDecodeState::Published | WriterSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self), "Writer snapshot decode reached Drop before publication or bounded retirement");
    }
}

const WRITER_MUTATION_FIELDS: &[store::OwnedSchemaFieldSpec] = &[
    store::OwnedSchemaFieldSpec { id: 1, key: "mutation", required: true },
    store::OwnedSchemaFieldSpec { id: 2, key: "newId", required: false },
    store::OwnedSchemaFieldSpec { id: 3, key: "newUri", required: false },
    store::OwnedSchemaFieldSpec { id: 4, key: "newLanguageId", required: false },
    store::OwnedSchemaFieldSpec { id: 5, key: "text", required: false },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum WriterMutationKind {
    RenameWriter,
    ChangeUri,
    ChangeLanguage,
    EditText,
}

struct WriterMutationString {
    field_id: u16,
    authority: store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES>,
}

struct WriterMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    cursor: store::OwnedSchemaNestedRecordCursor,
    active: Option<WriterMutationString>,
    kind: Option<WriterMutationKind>,
    payload_field: Option<u16>,
    payload: std::mem::ManuallyDrop<Option<String>>,
    value: std::mem::ManuallyDrop<Option<WriterMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    published: bool,
    terminal: bool,
}

impl WriterMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            cursor: store::OwnedSchemaNestedRecordCursor::try_new(store::OwnedSchemaRecordSpec { fields: WRITER_MUTATION_FIELDS }).expect("Writer mutation schema is a validated static catalog"),
            active: None,
            kind: None,
            payload_field: None,
            payload: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            published: false,
            terminal: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn finish_string(&mut self, field_id: u16, authority: &mut store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES>) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        if field_id == 1 {
            self.kind = Some(match authority.as_str() {
                Some("renameWriter") => WriterMutationKind::RenameWriter,
                Some("changeUri") => WriterMutationKind::ChangeUri,
                Some("changeLanguage") => WriterMutationKind::ChangeLanguage,
                Some("editText") => WriterMutationKind::EditText,
                _ => return Err(self.diagnostic("writer-envelope.unknown-mutation", 0)),
            });
            authority.cancel();
            return Ok(());
        }
        if self.payload_field.replace(field_id).is_some() {
            return Err(self.diagnostic("writer-envelope.duplicate-mutation-payload", 0));
        }
        *self.payload = authority.take_string();
        Ok(())
    }

    fn finish_record(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let kind = self.kind.ok_or_else(|| self.diagnostic("writer-envelope.missing-mutation-kind", 0))?;
        let expected = match kind {
            WriterMutationKind::RenameWriter => 2,
            WriterMutationKind::ChangeUri => 3,
            WriterMutationKind::ChangeLanguage => 4,
            WriterMutationKind::EditText => 5,
        };
        if self.payload_field != Some(expected) {
            return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0));
        }
        let payload = self.payload.take().ok_or_else(|| self.diagnostic("writer-envelope.missing-mutation-payload", 0))?;
        *self.value = Some(match kind {
            WriterMutationKind::RenameWriter => WriterMutation::RenameWriter(crate::artifacts::writer::schema::mutations::RenameWriter { new_id: payload }),
            WriterMutationKind::ChangeUri => WriterMutation::ChangeUri(crate::artifacts::writer::schema::mutations::ChangeUri { new_uri: payload }),
            WriterMutationKind::ChangeLanguage => WriterMutation::ChangeLanguage(crate::artifacts::writer::schema::mutations::ChangeLanguage { new_language_id: payload }),
            WriterMutationKind::EditText => WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: payload }),
        });
        Ok(())
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<WriterMutation> for WriterMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if let Some(mut active) = self.active.take() {
            return match active.authority.step(source, cx) {
                store::OwnedSchemaStringStep::Pending => {
                    self.active = Some(active);
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                store::OwnedSchemaStringStep::Complete => {
                    self.finish_string(active.field_id, &mut active.authority)?;
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                store::OwnedSchemaStringStep::Cancelled => Err(self.diagnostic("writer-envelope.mutation-string-cancelled", token.start)),
                store::OwnedSchemaStringStep::Fault(diagnostic) => Err(diagnostic),
            };
        }
        match self.cursor.accept(token, source) {
            store::OwnedSchemaNestedRecordStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete),
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal: true } => {
                let authority = store::OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, self.path).map_err(|token| self.diagnostic("writer-envelope.mutation-field-string", token.start))?;
                self.active = Some(WriterMutationString { field_id, authority });
                self.accept_token(token, true, source, cx)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { token, .. } => Err(self.diagnostic("writer-envelope.mutation-field-scalar", token.start)),
            store::OwnedSchemaNestedRecordStep::Complete => {
                self.finish_record()?;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaNestedRecordStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<WriterMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let value = self.value.take().ok_or_else(|| self.diagnostic("writer-envelope.mutation-not-ready", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.published = true;
        self.terminal = true;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(mut active) = self.active.take() {
            active.authority.cancel();
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| self.diagnostic("writer-envelope.mutation-retirement-fault", 0))? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    self.terminal = true;
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err(self.diagnostic("writer-envelope.mutation-retirement-false-terminal", 0)),
                step => Ok(step),
            };
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterMutationRetirementFactory, value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(payload) = self.payload.as_ref() {
            if payload.len() > maximum_bytes {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = payload.len();
            drop(self.payload.take());
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.payload.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self), "Writer mutation decode reached Drop before exact publication or bounded retirement");
    }
}

struct WriterMutationTarget {
    next_generation: u64,
    reservation: Option<store::ArtifactEnvelopeFieldReservation>,
    value: std::mem::ManuallyDrop<Option<WriterMutation>>,
}

impl WriterMutationTarget {
    fn new() -> Self {
        Self { next_generation: 0, reservation: None, value: std::mem::ManuallyDrop::new(None) }
    }
}

impl store::ArtifactEnvelopeMutationFieldTarget<WriterMutation> for WriterMutationTarget {
    fn reserve_mutation(&mut self) -> Result<store::ArtifactEnvelopeFieldReservation, store::OwnedSchemaDecodeDiagnostic> {
        if self.reservation.is_some() || self.value.is_some() {
            return Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.mutation-target-occupied", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.mutation-target-generation", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })?;
        let reservation =
            store::ArtifactEnvelopeFieldReservation::new(1, self.next_generation).ok_or(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.mutation-target-reservation", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })?;
        self.reservation = Some(reservation);
        Ok(reservation)
    }

    fn publish_mutation_reserved(&mut self, reservation: store::ArtifactEnvelopeFieldReservation, value: WriterMutation) {
        assert_eq!(self.reservation, Some(reservation), "Writer mutation publication requires its exact reservation");
        assert!(self.value.is_none(), "Writer mutation publication cannot replace an owner");
        self.reservation = None;
        *self.value = Some(value);
    }

    fn cancel_mutation_reservation(&mut self, reservation: store::ArtifactEnvelopeFieldReservation) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        if self.reservation != Some(reservation) {
            return Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.mutation-target-stale", offset: 0, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT });
        }
        self.reservation = None;
        Ok(())
    }
}

impl Drop for WriterMutationTarget {
    fn drop(&mut self) {
        assert!(self.reservation.is_none() && self.value.is_none(), "Writer mutation target reached Drop with a live reservation or value owner");
    }
}

enum WriterMutationArrayState {
    AwaitStart,
    Entries,
    Publishing,
    Complete,
    Closing,
}

struct WriterMutationArrayAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    catalog: std::sync::Arc<dyn store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation>>,
    mutation_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMutation>>,
    target: WriterMutationTarget,
    reservation: Option<store::ArtifactEnvelopeFieldReservation>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<WriterMutation>>>>,
    values: std::mem::ManuallyDrop<Option<Vec<WriterMutation>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    depth: usize,
    state: WriterMutationArrayState,
    taken: bool,
}

impl WriterMutationArrayAuthority {
    fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
        catalog: std::sync::Arc<dyn store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation>>,
        mutation_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMutation>>,
    ) -> Self {
        Self {
            operation,
            generation,
            path,
            catalog,
            mutation_factory,
            target: WriterMutationTarget::new(),
            reservation: None,
            active: std::mem::ManuallyDrop::new(None),
            values: std::mem::ManuallyDrop::new(Some(Vec::with_capacity(store::ARTIFACT_ENVELOPE_HISTORY_ITEMS))),
            retirement: std::mem::ManuallyDrop::new(None),
            depth: 0,
            state: WriterMutationArrayState::AwaitStart,
            taken: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn accept(&mut self, token: store::OwnedSchemaToken, terminal: bool, source: &store::OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if matches!(self.state, WriterMutationArrayState::Publishing) {
            let reservation = self.reservation.ok_or_else(|| self.diagnostic("writer-envelope.mutation-array-reservation-missing", token.start))?;
            let active = self.active.as_mut().ok_or_else(|| self.diagnostic("writer-envelope.mutation-array-owner-missing", token.start))?;
            return match active.publish_reserved(&mut self.target, reservation, cx)? {
                store::ArtifactEnvelopeFieldDecodeStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::ArtifactEnvelopeFieldDecodeStep::FieldComplete | store::ArtifactEnvelopeFieldDecodeStep::TokenComplete => {
                    self.reservation = None;
                    let value = self.target.value.take().ok_or_else(|| self.diagnostic("writer-envelope.mutation-array-value-missing", token.start))?;
                    let values = self.values.as_mut().ok_or_else(|| self.diagnostic("writer-envelope.mutation-array-values-missing", token.start))?;
                    if values.len() == values.capacity() {
                        *self.target.value = Some(value);
                        return Err(self.diagnostic("writer-envelope.mutation-array-capacity", token.start));
                    }
                    values.push(value);
                    if !active.terminal_is_empty() {
                        return Err(self.diagnostic("writer-envelope.mutation-array-live-after-publish", token.start));
                    }
                    drop(self.active.take());
                    self.state = WriterMutationArrayState::Entries;
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                store::ArtifactEnvelopeFieldDecodeStep::RecordComplete => Err(self.diagnostic("writer-envelope.mutation-array-record-terminal", token.start)),
            };
        }
        if let Some(active) = self.active.as_mut() {
            let entry_terminal = self.depth == 1 && token.kind == store::OwnedSchemaTokenKind::ObjectEnd;
            return match active.accept_token(token, entry_terminal, source, cx)? {
                store::ArtifactEnvelopeFieldDecodeStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::ArtifactEnvelopeFieldDecodeStep::FieldComplete if entry_terminal => {
                    self.depth = 0;
                    self.state = WriterMutationArrayState::Publishing;
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                store::ArtifactEnvelopeFieldDecodeStep::TokenComplete if !entry_terminal => {
                    match token.kind {
                        store::OwnedSchemaTokenKind::ObjectStart | store::OwnedSchemaTokenKind::ArrayStart => self.depth += 1,
                        store::OwnedSchemaTokenKind::ObjectEnd | store::OwnedSchemaTokenKind::ArrayEnd => self.depth -= 1,
                        _ => {}
                    }
                    Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                _ => Err(self.diagnostic("writer-envelope.mutation-array-terminal-discipline", token.start)),
            };
        }
        match self.state {
            WriterMutationArrayState::AwaitStart if token.kind == store::OwnedSchemaTokenKind::ArrayStart && !terminal => {
                self.state = WriterMutationArrayState::Entries;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            WriterMutationArrayState::Entries if token.kind == store::OwnedSchemaTokenKind::ArrayEnd && terminal => {
                self.state = WriterMutationArrayState::Complete;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            WriterMutationArrayState::Entries if token.kind == store::OwnedSchemaTokenKind::Comma => Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete),
            WriterMutationArrayState::Entries if token.kind == store::OwnedSchemaTokenKind::ObjectStart => {
                let reservation = self.target.reserve_mutation()?;
                self.reservation = Some(reservation);
                *self.active = Some(self.catalog.begin_mutation(self.operation, self.generation, self.path));
                self.accept(token, false, source, cx)
            }
            WriterMutationArrayState::AwaitStart => Err(self.diagnostic("writer-envelope.mutation-array-start", token.start)),
            WriterMutationArrayState::Entries => Err(self.diagnostic("writer-envelope.mutation-array-entry", token.start)),
            WriterMutationArrayState::Complete => Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete),
            WriterMutationArrayState::Publishing => unreachable!("publishing handled before token admission"),
            WriterMutationArrayState::Closing => Err(self.diagnostic("writer-envelope.mutation-array-closing", token.start)),
        }
    }

    fn take_values(&mut self) -> Option<Vec<WriterMutation>> {
        if !matches!(self.state, WriterMutationArrayState::Complete) || self.active.is_some() || self.reservation.is_some() || self.taken {
            return None;
        }
        self.taken = true;
        self.values.take()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        self.state = WriterMutationArrayState::Closing;
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(maximum_items, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    self.depth = 0;
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(self.diagnostic("writer-envelope.mutation-array-close-false-terminal", 0)),
                step => Ok(step),
            };
        }
        if let Some(reservation) = self.reservation.take() {
            self.target.cancel_mutation_reservation(reservation)?;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.target.value.take() {
            *self.retirement = Some(self.mutation_factory.retire_owned(value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| self.diagnostic("writer-envelope.mutation-array-retirement-fault", 0))? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                store::SnapshotRetirementStep::Complete => Err(self.diagnostic("writer-envelope.mutation-array-retirement-false-terminal", 0)),
                step => Ok(step),
            };
        }
        if let Some(values) = self.values.as_mut() {
            if let Some(value) = values.pop() {
                *self.retirement = Some(self.mutation_factory.retire_owned(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            drop(self.values.take());
        }
        self.taken = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.taken && self.active.is_none() && self.reservation.is_none() && self.target.reservation.is_none() && self.target.value.is_none() && self.values.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterMutationArrayAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Writer mutation array reached Drop before every exact mutation owner was published or cursor-retired");
    }
}

const WRITER_EDIT_FIELDS: &[store::OwnedSchemaFieldSpec] = &[
    store::OwnedSchemaFieldSpec { id: 1, key: "id", required: true },
    store::OwnedSchemaFieldSpec { id: 2, key: "actor", required: false },
    store::OwnedSchemaFieldSpec { id: 3, key: "forwards", required: true },
    store::OwnedSchemaFieldSpec { id: 4, key: "inverse", required: true },
    store::OwnedSchemaFieldSpec { id: 5, key: "mutationMeta", required: false },
    store::OwnedSchemaFieldSpec { id: 6, key: "description", required: false },
    store::OwnedSchemaFieldSpec { id: 7, key: "coalesceKey", required: false },
    store::OwnedSchemaFieldSpec { id: 8, key: "sequenceNumber", required: true },
    store::OwnedSchemaFieldSpec { id: 9, key: "startedAt", required: true },
    store::OwnedSchemaFieldSpec { id: 10, key: "finishedAt", required: false },
];

enum WriterEditActive {
    String { field_id: u16, authority: store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES> },
    Mutations { field_id: u16, authority: WriterMutationArrayAuthority },
    EmptyMetadata(store::OwnedSchemaEmptyArrayAuthority),
}

struct WriterEditHistoryAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    catalog: std::sync::Arc<dyn store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation>>,
    mutation_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<WriterMutation>>,
    retirement_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<protocol::Edit<WriterMutation>>>,
    cursor: store::OwnedSchemaNestedRecordCursor,
    active: Option<WriterEditActive>,
    strings: [std::mem::ManuallyDrop<Option<String>>; 6],
    forwards: std::mem::ManuallyDrop<Option<Vec<WriterMutation>>>,
    inverse: std::mem::ManuallyDrop<Option<Vec<WriterMutation>>>,
    sequence_number: Option<i32>,
    value: std::mem::ManuallyDrop<Option<protocol::Edit<WriterMutation>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    close_string: usize,
    terminal: bool,
}

impl WriterEditHistoryAuthority {
    fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
        catalog: std::sync::Arc<dyn store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation>>,
        retirement_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<protocol::Edit<WriterMutation>>>,
    ) -> Self {
        Self {
            operation,
            generation,
            path,
            catalog,
            mutation_factory: std::sync::Arc::new(WriterMutationRetirementFactory),
            retirement_factory,
            cursor: store::OwnedSchemaNestedRecordCursor::try_new(store::OwnedSchemaRecordSpec { fields: WRITER_EDIT_FIELDS }).expect("Writer edit schema is a validated static catalog"),
            active: None,
            strings: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            forwards: std::mem::ManuallyDrop::new(None),
            inverse: std::mem::ManuallyDrop::new(None),
            sequence_number: None,
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            close_string: 0,
            terminal: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn string_index(field_id: u16) -> Option<usize> {
        match field_id {
            1 => Some(0),
            2 => Some(1),
            6 => Some(2),
            7 => Some(3),
            9 => Some(4),
            10 => Some(5),
            _ => None,
        }
    }

    fn finish_record(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let id = self.strings[0].take().ok_or_else(|| self.diagnostic("writer-envelope.edit-id-missing", 0))?;
        let forwards = self.forwards.take().ok_or_else(|| self.diagnostic("writer-envelope.edit-forwards-missing", 0))?;
        let inverse = self.inverse.take().ok_or_else(|| self.diagnostic("writer-envelope.edit-inverse-missing", 0))?;
        let started_at = self.strings[4].take().ok_or_else(|| self.diagnostic("writer-envelope.edit-started-at-missing", 0))?;
        let sequence_number = self.sequence_number.ok_or_else(|| self.diagnostic("writer-envelope.edit-sequence-missing", 0))?;
        *self.value = Some(protocol::Edit {
            id,
            actor: self.strings[1].take(),
            forwards,
            inverse,
            mutation_meta: Vec::new(),
            description: self.strings[2].take(),
            coalesce_key: self.strings[3].take(),
            sequence_number,
            started_at,
            finished_at: self.strings[5].take(),
        });
        Ok(())
    }
}

impl store::ArtifactOwnedHistoryEntryAuthority<protocol::Edit<WriterMutation>> for WriterEditHistoryAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("writer-envelope.edit-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("writer-envelope.edit-cancelled", token.start));
        }
        if let Some(mut active) = self.active.take() {
            return match &mut active {
                WriterEditActive::String { field_id, authority } => match authority.step(source, cx) {
                    store::OwnedSchemaStringStep::Pending => {
                        self.active = Some(active);
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    store::OwnedSchemaStringStep::Complete => {
                        let index = Self::string_index(*field_id).ok_or_else(|| self.diagnostic("writer-envelope.edit-string-field", token.start))?;
                        *self.strings[index] = authority.take_string();
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    store::OwnedSchemaStringStep::Cancelled => Err(self.diagnostic("writer-envelope.edit-string-cancelled", token.start)),
                    store::OwnedSchemaStringStep::Fault(diagnostic) => Err(diagnostic),
                },
                WriterEditActive::Mutations { field_id, authority } => match authority.accept(token, _terminal, source, cx)? {
                    store::ArtifactEnvelopeFieldDecodeStep::FieldComplete => {
                        let values = authority.take_values().ok_or_else(|| self.diagnostic("writer-envelope.edit-mutation-values", token.start))?;
                        if *field_id == 3 {
                            *self.forwards = Some(values);
                        } else {
                            *self.inverse = Some(values);
                        }
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    step => {
                        self.active = Some(active);
                        Ok(step)
                    }
                },
                WriterEditActive::EmptyMetadata(authority) => match authority.accept(token, _terminal)? {
                    store::ArtifactEnvelopeFieldDecodeStep::FieldComplete => Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                    step => {
                        self.active = Some(active);
                        Ok(step)
                    }
                },
            };
        }
        match self.cursor.accept(token, source) {
            store::OwnedSchemaNestedRecordStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete),
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } if Self::string_index(field_id).is_some() => {
                if token.kind == store::OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 6 | 7 | 10) {
                    return Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete);
                }
                if !terminal {
                    return Err(self.diagnostic("writer-envelope.edit-string-scalar", token.start));
                }
                let authority = store::OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, self.path).map_err(|token| self.diagnostic("writer-envelope.edit-string", token.start))?;
                self.active = Some(WriterEditActive::String { field_id, authority });
                self.accept_token(token, true, source, cx)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } if matches!(field_id, 3 | 4) => {
                let authority = WriterMutationArrayAuthority::new(self.operation, self.generation, self.path, self.catalog.clone(), self.mutation_factory.clone());
                self.active = Some(WriterEditActive::Mutations { field_id, authority });
                self.accept_token(token, terminal, source, cx)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id: 5, token, terminal } => {
                self.active = Some(WriterEditActive::EmptyMetadata(store::OwnedSchemaEmptyArrayAuthority::new(self.path)));
                self.accept_token(token, terminal, source, cx)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id: 8, token, terminal: true } if token.kind == store::OwnedSchemaTokenKind::Number => {
                let mut bytes = [0u8; 64];
                let len = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
                if len == 0 || len > bytes.len() || source.copy_token_bytes(token, 0, &mut bytes[..len]) != len {
                    return Err(self.diagnostic("writer-envelope.edit-sequence-token", token.start));
                }
                self.sequence_number = std::str::from_utf8(&bytes[..len]).ok().and_then(|value| value.parse().ok());
                if self.sequence_number.is_none() {
                    return Err(self.diagnostic("writer-envelope.edit-sequence-value", token.start));
                }
                Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            store::OwnedSchemaNestedRecordStep::FieldToken { token, .. } => Err(self.diagnostic("writer-envelope.edit-field", token.start)),
            store::OwnedSchemaNestedRecordStep::Complete => {
                self.finish_record()?;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaNestedRecordStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn take_value(&mut self) -> Option<protocol::Edit<WriterMutation>> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            let step = match active {
                WriterEditActive::String { authority, .. } => {
                    authority.cancel();
                    store::SnapshotRetirementStep::Complete
                }
                WriterEditActive::Mutations { authority, .. } => authority.close_step(maximum_items, maximum_bytes)?,
                WriterEditActive::EmptyMetadata(_) => store::SnapshotRetirementStep::Complete,
            };
            if matches!(step, store::SnapshotRetirementStep::Complete) {
                drop(self.active.take());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            return match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| self.diagnostic("writer-envelope.edit-retirement-fault", 0))? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.retirement.take());
                    self.terminal = true;
                    Ok(store::SnapshotRetirementStep::Complete)
                }
                store::SnapshotRetirementStep::Complete => Err(self.diagnostic("writer-envelope.edit-retirement-false-terminal", 0)),
                step => Ok(step),
            };
        }
        if let Some(value) = self.value.take() {
            *self.retirement = Some(self.retirement_factory.retire_owned(value));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        for values in [&mut self.inverse, &mut self.forwards] {
            if let Some(value) = values.as_mut().and_then(Vec::pop) {
                *self.retirement = Some(self.mutation_factory.retire_owned(value));
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if values.as_ref().is_some_and(Vec::is_empty) {
                drop(values.take());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        while self.close_string < self.strings.len() {
            let index = self.close_string;
            self.close_string += 1;
            if let Some(value) = self.strings[index].as_ref() {
                if value.len() > maximum_bytes {
                    self.close_string -= 1;
                    return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                let released_bytes = value.len();
                drop(self.strings[index].take());
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
            }
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.forwards.is_none() && self.inverse.is_none() && self.value.is_none() && self.retirement.is_none() && self.strings.iter().all(|value| value.is_none())
    }
}

impl Drop for WriterEditHistoryAuthority {
    fn drop(&mut self) {
        assert!(store::ArtifactOwnedHistoryEntryAuthority::terminal_is_empty(self), "Writer edit decode reached Drop before exact publication or bounded retirement");
    }
}

struct WriterEditHistoryDecoder {
    catalog: std::sync::Arc<dyn store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation>>,
}

impl store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<WriterMutation>> for WriterEditHistoryDecoder {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: store::OwnedSchemaPath,
        retirement_factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<protocol::Edit<WriterMutation>>>,
    ) -> Box<dyn store::ArtifactOwnedHistoryEntryAuthority<protocol::Edit<WriterMutation>>> {
        Box::new(WriterEditHistoryAuthority::new(operation, generation, path, self.catalog.clone(), retirement_factory))
    }
}

struct WriterRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for WriterRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub struct WriterEnvelopeOwnedFieldCatalog;

impl store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation> for WriterEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<WriterSnapshot, WriterMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(WriterSnapshotRetirementFactory), std::sync::Arc::new(WriterMutationRetirementFactory), self.edit_history_decoder()))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot>> {
        Box::new(WriterSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<WriterMutation>> {
        Box::new(WriterMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(WriterRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<WriterMutation>>> {
        std::sync::Arc::new(WriterEditHistoryDecoder { catalog: std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog) })
    }
}

pub fn writer_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<WriterSnapshot, WriterMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(WriterSnapshotRetirementFactory), std::sync::Arc::new(WriterMutationRetirementFactory))
}

pub fn writer_document_store_owners() -> store::MemberStoreOwners<WriterSnapshot, WriterMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(WriterSnapshotRetirementFactory),
        std::sync::Arc::new(WriterSnapshotRetirementFactory),
        std::sync::Arc::new(WriterMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<WriterSnapshot, WriterMutation>::new()),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WriterStoreInitializationPhase {
    ValidateEnvelope,
    ValidateEditPair { left: usize, right: usize },
    CloneInitial { field: u8 },
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct WriterStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<WriterSnapshot, WriterMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<WriterSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<WriterSnapshot, WriterMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    initial: std::mem::ManuallyDrop<Option<WriterSnapshot>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: WriterStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl WriterStoreInitializationAuthority {
    fn new(
        envelope: store::ArtifactEnvelope<WriterSnapshot, WriterMutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Self {
        let empty_document = store::ArtifactChild::new(
            String::new(),
            store::os_io::ArtifactRef {
                artifact_id: String::new(),
                dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() },
            },
        );
        Self {
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            initial: std::mem::ManuallyDrop::new(Some(WriterSnapshot { schema: String::new(), id: String::new(), language_id: String::new(), uri: String::new(), document: empty_document })),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"writer.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase: WriterStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn initial_field(value: &WriterSnapshot, field: u8) -> &str {
        match field {
            0 => &value.schema,
            1 => &value.id,
            2 => &value.language_id,
            3 => &value.uri,
            4 => &value.document.child_id,
            5 => &value.document.target.artifact_id,
            6 => &value.document.target.dialect.artifact_kind,
            7 => &value.document.target.dialect.standard,
            8 => &value.document.target.dialect.subset,
            _ => unreachable!("Writer initial field cursor is validated"),
        }
    }

    fn initial_field_mut(value: &mut WriterSnapshot, field: u8) -> &mut String {
        match field {
            0 => &mut value.schema,
            1 => &mut value.id,
            2 => &mut value.language_id,
            3 => &mut value.uri,
            4 => &mut value.document.child_id,
            5 => &mut value.document.target.artifact_id,
            6 => &mut value.document.target.dialect.artifact_kind,
            7 => &mut value.document.target.dialect.standard,
            8 => &mut value.document.target.dialect.subset,
            _ => unreachable!("Writer initial field cursor is validated"),
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = WriterStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, WRITER_ENVELOPE_FIELD_BYTES)? {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= WRITER_ENVELOPE_FIELD_BYTES => Ok(true),
            store::SnapshotRetirementStep::Pending { .. } => Err("Writer store initializer retirement exceeded its exact grant".into()),
            store::SnapshotRetirementStep::Blocked => Ok(true),
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                drop(self.active.take());
                Ok(true)
            }
            store::SnapshotRetirementStep::Complete => Err("Writer store initializer retirement reported a false terminal".into()),
        }
    }

    fn pump_terminal_retirement(&mut self) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            match runtime.close_step(&WriterSnapshotRetirementFactory, 1, WRITER_ENVELOPE_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    return Ok(false);
                }
                store::SnapshotRetirementStep::Complete => return Err("Writer initialization runtime reported a false terminal".into()),
                _ => return Ok(false),
            }
        }
        if let Some(initial) = self.initial.take() {
            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, initial));
            return Ok(false);
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(writer_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(true)
                }
                store::SnapshotRetirementStep::Complete => Err("Writer initialization envelope retirement reported a false terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.initial.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<WriterSnapshot, WriterMutation> for WriterStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"writer-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, WriterStoreInitializationPhase::RetireCancelled | WriterStoreInitializationPhase::Cancelled) {
            self.phase = WriterStoreInitializationPhase::RetireCancelled;
        }
        if let Err(error) = self.pump_active() {
            self.fault = Some(error.into_bytes());
            self.phase = WriterStoreInitializationPhase::RetireFault;
        } else if self.active.is_some() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.phase {
            WriterStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"writer-store.initializer-envelope-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if envelope.schema != crate::artifacts::writer::WRITER_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > WRITER_ENVELOPE_FIELD_BYTES {
                    self.fail(b"writer-store.initializer-envelope-invalid");
                } else {
                    self.phase = WriterStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("validated Writer envelope remains retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = WriterStoreInitializationPhase::CloneInitial { field: 0 };
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = WriterStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id || envelope.vcs.edits[left].id.len() > WRITER_ENVELOPE_FIELD_BYTES {
                    self.fail(b"writer-store.initializer-duplicate-or-hostile-edit");
                } else {
                    self.phase = WriterStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::CloneInitial { field } => {
                if field == 9 {
                    let initial = self.initial.take().expect("Writer initial snapshot was built one field at a time");
                    let initial_digest = self.initial_digest.take().expect("Writer initial digest remains retained").finish();
                    let envelope = self.envelope.as_ref().expect("Writer envelope remains retained during runtime construction");
                    *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, initial_digest));
                    self.phase = WriterStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                }
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained during initial clone");
                let value = Self::initial_field(&envelope.vcs.initial_snapshot, field);
                if value.len() > WRITER_ENVELOPE_FIELD_BYTES {
                    self.fail(b"writer-store.initializer-initial-field-too-large");
                    return semio_framework_job::StepOutcome::Yield;
                }
                self.initial_digest.as_mut().expect("Writer initial digest remains retained").observe(value.as_bytes());
                *Self::initial_field_mut(self.initial.as_mut().expect("Writer initial target remains retained"), field) = value.to_string();
                self.phase = WriterStoreInitializationPhase::CloneInitial { field: field + 1 };
                cx.consume_fuel(value.len().max(1) as u64);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = WriterStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = WriterStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_mutation(id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = WriterStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp.clone());
                        self.phase = WriterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = WriterStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Writer runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = WriterStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"writer-store.initializer-applied-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let id = edit.id.clone();
                    let sequence_number = edit.sequence_number;
                    let started_at = edit.started_at.clone();
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"writer.edit");
                    digest.observe(id.as_bytes());
                    digest.observe(&sequence_number.to_be_bytes());
                    digest.observe(started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = WriterStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = WriterStoreInitializationPhase::FindApplied { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer applied edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = WriterStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let encoded = match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= WRITER_ENVELOPE_FIELD_BYTES => encoded,
                    _ => {
                        self.fail(b"writer-store.initializer-forward-encoding");
                        return semio_framework_job::StepOutcome::Yield;
                    }
                };
                self.edit_digest.as_mut().expect("Writer edit digest remains retained").observe(&encoded);
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Writer runtime current snapshot remains retained");
                let (diff, messages) = operation.diff(current).into_parts();
                if messages.iter().any(|message| message.level == protocol::Severity::Fatal) {
                    self.fail(b"writer-store.initializer-fatal-mutation");
                    return semio_framework_job::StepOutcome::Yield;
                }
                match diff.apply(current) {
                    Ok(next) => {
                        let previous = std::mem::replace(current, next);
                        *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterSnapshotRetirementFactory, previous));
                        self.phase = WriterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    Err(error) => {
                        self.fault = Some(error.to_string().into_bytes());
                        self.phase = WriterStoreInitializationPhase::RetireFault;
                    }
                }
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer applied edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = WriterStoreInitializationPhase::CommitApplied { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= WRITER_ENVELOPE_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Writer edit digest remains retained").observe(&encoded);
                        self.phase = WriterStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"writer-store.initializer-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer applied edit remains retained");
                let id = entry.id.clone();
                let actor = entry.actor.clone();
                let digest = self.edit_digest.take().expect("Writer applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = WriterStoreInitializationPhase::RetireFault;
                } else {
                    runtime.set_local_actor_id(actor);
                    self.phase = WriterStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = WriterStoreInitializationPhase::BuildCandidate;
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.as_ref().expect("Writer envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"writer-store.initializer-redo-edit-missing");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if edit.id == id {
                    let id = edit.id.clone();
                    let sequence_number = edit.sequence_number;
                    let started_at = edit.started_at.clone();
                    let mut digest = store::ArtifactStoreInitializationDigest::new(b"writer.edit");
                    digest.observe(id.as_bytes());
                    digest.observe(&sequence_number.to_be_bytes());
                    digest.observe(started_at.as_bytes());
                    *self.edit_digest = Some(digest);
                    self.phase = WriterStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                } else {
                    self.phase = WriterStoreInitializationPhase::FindRedo { position, scan: scan + 1 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer redo edit remains retained");
                let Some(operation) = entry.forwards.get(mutation) else {
                    self.phase = WriterStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= WRITER_ENVELOPE_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Writer redo digest remains retained").observe(&encoded);
                        self.phase = WriterStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"writer-store.initializer-redo-forward-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer redo edit remains retained");
                let Some(operation) = entry.inverse.get(mutation) else {
                    self.phase = WriterStoreInitializationPhase::CommitRedo { position, edit };
                    return semio_framework_job::StepOutcome::Yield;
                };
                match operation.encode_op() {
                    Ok(encoded) if encoded.len() <= WRITER_ENVELOPE_FIELD_BYTES => {
                        self.edit_digest.as_mut().expect("Writer redo digest remains retained").observe(&encoded);
                        self.phase = WriterStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(encoded.len().max(1) as u64);
                    }
                    _ => self.fail(b"writer-store.initializer-redo-inverse-encoding"),
                }
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::CommitRedo { position, edit } => {
                let id = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Writer redo edit remains retained").id.clone();
                let digest = self.edit_digest.take().expect("Writer redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Writer runtime remains retained").push_redo(id, digest) {
                    self.fault = Some(error.into_bytes());
                    self.phase = WriterStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = WriterStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 };
                }
                cx.consume_fuel(1);
                semio_framework_job::StepOutcome::Yield
            }
            WriterStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"writer-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                let envelope = self.envelope.take().expect("Writer envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Writer runtime remains retained until atomic store construction");
                let mut candidate = store::ArtifactStore::from_initialized_runtime(envelope, runtime, candidate_generation);
                candidate.install_member_store_owners_exact(writer_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = WriterStoreInitializationPhase::Complete;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: Vec::new() })
            }
            WriterStoreInitializationPhase::RetireCancelled | WriterStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement() {
                Ok(false) => semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == WriterStoreInitializationPhase::RetireCancelled {
                        self.phase = WriterStoreInitializationPhase::Cancelled;
                        semio_framework_job::StepOutcome::Cancelled
                    } else {
                        self.phase = WriterStoreInitializationPhase::Fault;
                        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: self.fault.take().unwrap_or_else(|| b"writer-store.initializer-fault".to_vec()) })
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_bytes());
                    semio_framework_job::StepOutcome::Yield
                }
            },
            WriterStoreInitializationPhase::Complete => semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: Vec::new(), output: Vec::new() }),
            WriterStoreInitializationPhase::Cancelled => semio_framework_job::StepOutcome::Cancelled,
            WriterStoreInitializationPhase::Fault => semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: self.fault.clone().unwrap_or_else(|| b"writer-store.initializer-fault".to_vec()) }),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<WriterSnapshot, WriterMutation>> {
        if self.phase != WriterStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.initial_digest.take());
        drop(self.edit_digest.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for WriterStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Writer store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn writer_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<WriterSnapshot, WriterMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<WriterSnapshot, WriterMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(WriterStoreInitializationAuthority::new(envelope, operation, generation)))
}
//#endregion 🔖️OwnedEnvelopeCatalog

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::{schema, WriterSnapshot};

    #[semio_framework_async_macros::async_test]
    async fn writer_snapshot_and_mutation_owners_retire_one_exact_field_per_grant() {
        let snapshot = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "deep", "plaintext", "writer://deep", "body");
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

        let hostile = WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: "x".repeat(WRITER_ENVELOPE_FIELD_BYTES) });
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
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(1),
                semio_framework_job::Generation(1),
                semio_framework_job::StepBudget::new(3, u64::MAX),
                cancel.clone(),
                semio_framework_job::default_now_ms,
                &mut preview_sequence,
            );
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
            forwards: vec![WriterMutation::RenameWriter(crate::artifacts::writer::schema::mutations::RenameWriter { new_id: "next".into() })],
            inverse: Vec::new(),
            mutation_meta: Vec::new(),
            description: None,
            coalesce_key: None,
            sequence_number: 1,
            started_at: "1".into(),
            finished_at: None,
        };
        let bytes = serde_json::to_vec(&serde_json::json!({ "value": edit })).expect("bounded Writer edit fixture");
        let decoded = drive_writer_edit(&bytes, semio_framework_job::root_cancel_token()).expect("Writer owns its retained edit and mutation decoders");
        assert_eq!(decoded, edit);
        assert!(drive_writer_edit(br#"{"value":{"id":"broken","forwards":[{"mutation":"unknown","newId":"x"}],"inverse":[],"sequenceNumber":1,"startedAt":"1"}}"#, semio_framework_job::root_cancel_token()).is_err());
        let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&WriterMutationRetirementFactory, decoded.forwards.into_iter().next().expect("decoded mutation owner"));
        while !retirement.terminal_is_empty() {
            retirement.close_step(1, WRITER_ENVELOPE_FIELD_BYTES).expect("decoded mutation closes");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: "hello".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    async fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
    }

    /// 🧬️ Reaches `jack_snapshot()` from `empty_writer_snapshot()` via the semantic vocabulary —
    /// `SetSnapshot` (whole-document replace) is banned, so what used to be one mutation is now the
    /// sequence of scalar mutations that actually differ between the two documents (`schema` is
    /// identical in both, so it gets no mutation). `EditText` mints its `document` handle from
    /// `base.id`/`base.language_id` at apply time, so it must run LAST, after `RenameWriter`/
    /// `ChangeLanguage` have already landed — otherwise its handle would target the wrong owner id.
    async fn jack_mutations() -> Vec<WriterMutation> {
        let jack = jack_snapshot();
        let text = crate::artifacts::writer::writer_text(&jack);
        vec![
            WriterMutation::RenameWriter(crate::artifacts::writer::schema::mutations::RenameWriter { new_id: jack.id }),
            WriterMutation::ChangeLanguage(crate::artifacts::writer::schema::mutations::ChangeLanguage { new_language_id: jack.language_id }),
            WriterMutation::ChangeUri(crate::artifacts::writer::schema::mutations::ChangeUri { new_uri: jack.uri }),
            WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_document_text_round_trips_through_the_store() {
        let mut store = store::ArtifactStore::<WriterSnapshot, WriterMutation>::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot"), jack_snapshot());
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `WriterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip law.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};

        let mut store = store::ArtifactStore::<WriterSnapshot, WriterMutation>::new(store::create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: jack_mutations(), description: None }).expect("apply");
        let edit: &Edit<WriterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<WriterSnapshot, WriterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_protocol_bytes_against_encoded_spr() {
        let operation = WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: "hello".into() });
        let bytes = encode_op(&operation).expect("encode op");
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
    }
}
