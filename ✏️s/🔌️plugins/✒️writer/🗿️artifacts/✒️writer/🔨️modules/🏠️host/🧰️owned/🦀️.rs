//! 🏠️ Artifact document-store and publication authorities.

use crate::op::WriterMutation;
use crate::schema;
use crate::WriterSnapshot;
use store::ArtifactEnvelopeMutationFieldTarget;
use semio_framework_value::retirement::OwnedValueRetirementFactory;

/// 🗃️ The framework-bounded document owner catalog, funded for a store no scheduler grants.
pub fn writer_document_store_owners() -> store::DocumentStoreOwners<WriterSnapshot, WriterMutation> {
    store::funded_bounded_artifact_store_owners::<WriterSnapshot, WriterMutation>().expect("funded Writer document owners")
}

pub type WriterDocumentStore = store::ArtifactStore<WriterSnapshot, WriterMutation>;

const WRITER_ENVELOPE_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// 🔐️ Opens a Writer store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` then refuses every `Apply`
/// (`edit history insertion requires its exact mutation retirement factory`) — a bare
/// `ArtifactStore::new` can be READ but never mutated, undone or closed. The app installs
/// [`writer_document_store_owners`] through `ArtifactEditor::build_document_store_owners`; every
/// standalone store goes through here instead. Mirrors `🕸️dag`'s `new_dag_store`.
pub async fn new_writer_store(envelope: WriterDocumentEnvelope, actor: protocol::ActorId) -> Result<OwnedWriterStore, store::VcsError> {
    let mut store = WriterDocumentStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(writer_document_store_owners()).unwrap_or_else(|(error, _owners)| panic!("{}", error.into_message()));
    Ok(OwnedWriterStore(store))
}

/// 🔚 A standalone Writer store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedWriterStore(WriterDocumentStore);

impl OwnedWriterStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            let demand = self.0.close_owned_demands(WRITER_ENVELOPE_FIELD_BYTES).expect("the Writer document store quotes its close");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(WRITER_ENVELOPE_FIELD_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            self.0.close_owned_step(grant).expect("Writer document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedWriterStore {
    type Target = WriterDocumentStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedWriterStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedWriterStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
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

impl store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot> for WriterSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
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

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth)
    }
    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        let empty = store::RetainedCloneProgress::default();
        if store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self) {
            return Ok(store::RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(self.diagnostic("writer-envelope.snapshot-retirement-depth", 0));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        if let WriterSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = WriterSnapshotDecodeState::Closing;
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ..empty }));
        }
        let child = store::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.retirement.is_some() {
            let step = store::artifact_retirement_box_close_step(&mut self.retirement, child).map_err(|_| self.diagnostic("writer-envelope.snapshot-retirement-fault", 0))?;
            if self.retirement.is_none() {
                self.state = WriterSnapshotDecodeState::Complete;
            }
            return Ok(store::RetainedCloneStep::Progress(step.progress()));
        }
        if self.value.is_some() {
            let step = store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, child).map_err(|_| self.diagnostic("writer-envelope.snapshot-retirement-admission", 0))?;
            self.state = WriterSnapshotDecodeState::Closing;
            return Ok(step);
        }
        self.state = WriterSnapshotDecodeState::Complete;
        Ok(store::RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, WriterSnapshotDecodeState::Published | WriterSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterSnapshotDecodeAuthority {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self)), "Writer snapshot decode reached Drop before publication or bounded retirement");
    }
}

struct WriterMutationString {
    field_id: u16,
    authority: store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES>,
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
            splice: WriterSplicePayload::default(),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            published: false,
            terminal: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn finish_string(&mut self, field_id: u16, authority: &mut store::OwnedSchemaStringAuthority<WRITER_ENVELOPE_FIELD_BYTES>) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        if field_id == 1 {
            self.kind = Some(match authority.as_str() {
                Some("renameWriter") => WriterMutationKind::RenameWriter,
                Some("changeUri") => WriterMutationKind::ChangeUri,
                Some("changeLanguage") => WriterMutationKind::ChangeLanguage,
                Some("editText") => WriterMutationKind::EditText,
                Some("spliceText") => WriterMutationKind::SpliceText,
                _ => return Err(self.diagnostic("writer-envelope.unknown-mutation", 0)),
            });
            authority.cancel();
            return Ok(());
        }
        if (7..=10).contains(&field_id) {
            let slot = &mut self.splice.strings[usize::from(field_id - 7)];
            if slot.is_some() {
                return Err(self.diagnostic("writer-envelope.duplicate-mutation-payload", 0));
            }
            *slot = authority.take_string();
            return Ok(());
        }
        if self.payload_field.replace(field_id).is_some() {
            return Err(self.diagnostic("writer-envelope.duplicate-mutation-payload", 0));
        }
        *self.payload = authority.take_string();
        Ok(())
    }

    /// 🔢️ `SpliceText.start`: the one scalar field of the mutation record, a non-negative integer that fits `u32`.
    fn finish_start(&mut self, token: store::OwnedSchemaToken, source: &store::OwnedSchemaRecordCursor) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let mut digits = [0u8; 10];
        let length = usize::try_from(token.end - token.start).map_err(|_| self.diagnostic("writer-envelope.splice-start", token.start))?;
        if token.kind != store::OwnedSchemaTokenKind::Number || length == 0 || length > digits.len() || self.splice.start.is_some() {
            return Err(self.diagnostic("writer-envelope.splice-start", token.start));
        }
        source.copy_token_bytes(token, 0, &mut digits[..length]);
        let text = std::str::from_utf8(&digits[..length]).map_err(|_| self.diagnostic("writer-envelope.splice-start", token.start))?;
        self.splice.start = Some(text.parse::<u32>().map_err(|_| self.diagnostic("writer-envelope.splice-start", token.start))?);
        Ok(())
    }

    fn finish_record(&mut self) -> Result<(), store::OwnedSchemaDecodeDiagnostic> {
        let kind = self.kind.ok_or_else(|| self.diagnostic("writer-envelope.missing-mutation-kind", 0))?;
        let expected = match kind {
            WriterMutationKind::RenameWriter => 2,
            WriterMutationKind::ChangeUri => 3,
            WriterMutationKind::ChangeLanguage => 4,
            WriterMutationKind::EditText => 5,
            WriterMutationKind::SpliceText => {
                let [deleted, insert, before, after] = std::mem::take(&mut self.splice.strings);
                let (Some(start), Some(deleted), Some(insert), Some(before), Some(after), None) = (self.splice.start.take(), deleted, insert, before, after, self.payload_field) else {
                    return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0));
                };
                *self.value = Some(WriterMutation::SpliceText(schema::mutations::SpliceText { start, deleted, insert, before, after }));
                return Ok(());
            }
        };
        if self.payload_field != Some(expected) || self.splice.start.is_some() || self.splice.strings.iter().any(Option::is_some) {
            return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0));
        }
        let payload = self.payload.take().ok_or_else(|| self.diagnostic("writer-envelope.missing-mutation-payload", 0))?;
        *self.value = Some(match kind {
            WriterMutationKind::RenameWriter => WriterMutation::RenameWriter(schema::mutations::RenameWriter { new_id: payload }),
            WriterMutationKind::ChangeUri => WriterMutation::ChangeUri(schema::mutations::ChangeUri { new_uri: payload }),
            WriterMutationKind::ChangeLanguage => WriterMutation::ChangeLanguage(schema::mutations::ChangeLanguage { new_language_id: payload }),
            WriterMutationKind::EditText => WriterMutation::EditText(schema::mutations::EditText { text: payload }),
            WriterMutationKind::SpliceText => return Err(self.diagnostic("writer-envelope.mutation-payload-mismatch", 0)),
        });
        Ok(())
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<WriterMutation> for WriterMutationDecodeAuthority {
    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth)
    }

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
            store::OwnedSchemaNestedRecordStep::FieldToken { field_id: 6, token, .. } => {
                self.finish_start(token, source)?;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::TokenComplete)
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
        target: &mut dyn ArtifactEnvelopeMutationFieldTarget<WriterMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let value = self.value.take().ok_or_else(|| self.diagnostic("writer-envelope.mutation-not-ready", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.published = true;
        self.terminal = true;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        let empty = store::RetainedCloneProgress::default();
        if store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self) {
            return Ok(store::RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(self.diagnostic("writer-envelope.mutation-retirement-depth", 0));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        if let Some(mut active) = self.active.take() {
            active.authority.cancel();
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let child = store::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let fault = |this: &Self| this.diagnostic("writer-envelope.mutation-retirement-fault", 0);
        if self.retirement.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.retirement, child).map_err(|_| fault(self));
        }
        if self.value.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, child).map_err(|_| fault(self));
        }
        if let Some(index) = self.splice.strings.iter().position(Option::is_some) {
            return store::artifact_retirement_admit_owned(&mut self.splice.strings[index], &mut self.retirement, child).map_err(|_| fault(self));
        }
        if self.payload.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.payload, &mut self.retirement, child).map_err(|_| fault(self));
        }
        self.terminal = true;
        Ok(store::RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.payload.is_none() && self.splice.strings.iter().all(Option::is_none) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for WriterMutationDecodeAuthority {
    fn drop(&mut self) {
        // 🧯️ `std::thread::panicking()` FIRST, exactly like the framework's own
        // `store::ArtifactEnvelope::drop`: a Drop witness exists to catch a leak on a HEALTHY path.
        // Firing it while the thread is ALREADY unwinding turns a reported failure into
        // `panic in a destructor during cleanup` — a non-unwinding abort that kills the whole test
        // binary and hides the first, real failure (that is how one red test took this crate's
        // other 300 with it).
        assert!(std::thread::panicking() || (store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self)), "Writer mutation decode reached Drop before exact publication or bounded retirement");
    }
}

impl WriterSnapshotDecodeAuthority {
    fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
        if matches!(self.state, WriterSnapshotDecodeState::Decode(_)) {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, depth: 1, ..Default::default() });
        }
        let demand = match self.retirement.as_ref() {
            Some(owner) => store::artifact_retirement_box_demands(owner, maximum_copy_bytes),
            None => store::artifact_retirement_owned_birth_demands(&self.value),
        };
        demand.map_err(|_| self.diagnostic("writer-envelope.snapshot-retirement-demand", 0))
    }
}

impl WriterMutationDecodeAuthority {
    fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
        if self.active.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        let demand = if let Some(owner) = self.retirement.as_ref() {
            store::artifact_retirement_box_demands(owner, maximum_copy_bytes)
        } else if self.value.is_some() {
            store::artifact_retirement_owned_birth_demands(&self.value)
        } else if let Some(index) = self.splice.strings.iter().position(Option::is_some) {
            store::artifact_retirement_owned_birth_demands(&self.splice.strings[index])
        } else {
            store::artifact_retirement_owned_birth_demands(&self.payload)
        };
        demand.map_err(|_| self.diagnostic("writer-envelope.mutation-retirement-demand", 0))
    }
}


impl store::ArtifactEnvelopeOwnedFieldCatalog<WriterSnapshot, WriterMutation> for WriterEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<WriterSnapshot, WriterMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<WriterSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(OwnedValueRetirementFactory::<WriterSnapshot>::default()), std::sync::Arc::new(OwnedValueRetirementFactory::<WriterMutation>::default()), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<WriterSnapshot, WriterMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
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
        store::artifact_owned_spr_edit_history_decoder::<WriterSnapshot, WriterMutation>(std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(OwnedValueRetirementFactory::<WriterMutation>::default()))
    }
}

pub fn writer_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<WriterSnapshot, WriterMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(WriterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(OwnedValueRetirementFactory::<WriterSnapshot>::default()), std::sync::Arc::new(OwnedValueRetirementFactory::<WriterMutation>::default()))
}

pub type WriterDocumentEnvelope = store::ArtifactEnvelope<WriterSnapshot, WriterMutation>;

/// 🧹️ Closes one erased retirement to its terminal-empty witness, one self-funded exact grant per turn.
#[cfg(test)]
pub(crate) fn close_retirement(mut retirement: Box<dyn store::ErasedSnapshotRetirement>) {
    for _ in 0..1_000_000 {
        if retirement.terminal_is_empty() {
            return;
        }
        let demand = retirement.next_demand(WRITER_ENVELOPE_FIELD_BYTES).expect("the Writer retirement quotes its next turn");
        let grant = store::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(WRITER_ENVELOPE_FIELD_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = retirement.close_step(grant).expect("the Writer retirement closes within its exact grant");
        assert!(step.progress().fits(grant));
    }
    panic!("Writer retirement did not reach its terminal-empty witness")
}

/// 🧹️ Retires a fixture envelope that no store adopted, through the artifact's own decode owner bundle.
#[cfg(test)]
pub(crate) fn retire_writer_envelope(envelope: WriterDocumentEnvelope) {
    close_retirement(writer_envelope_decode_owner_bundle().retire_envelope(envelope));
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
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }
}

const WRITER_MUTATION_FIELDS: &[store::OwnedSchemaFieldSpec] = &[
    store::OwnedSchemaFieldSpec { id: 1, key: "mutation", required: true },
    store::OwnedSchemaFieldSpec { id: 2, key: "newId", required: false },
    store::OwnedSchemaFieldSpec { id: 3, key: "newUri", required: false },
    store::OwnedSchemaFieldSpec { id: 4, key: "newLanguageId", required: false },
    store::OwnedSchemaFieldSpec { id: 5, key: "text", required: false },
    store::OwnedSchemaFieldSpec { id: 6, key: "start", required: false },
    store::OwnedSchemaFieldSpec { id: 7, key: "deleted", required: false },
    store::OwnedSchemaFieldSpec { id: 8, key: "insert", required: false },
    store::OwnedSchemaFieldSpec { id: 9, key: "before", required: false },
    store::OwnedSchemaFieldSpec { id: 10, key: "after", required: false },
];

/// ✂️ `SpliceText`'s four string fields (ids 7–10) and its scalar `start` (id 6), gathered before the record completes.
#[derive(Default)]
struct WriterSplicePayload {
    start: Option<u32>,
    strings: [Option<String>; 4],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WriterMutationKind {
    RenameWriter,
    ChangeUri,
    ChangeLanguage,
    EditText,
    SpliceText,
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
    splice: WriterSplicePayload,
    value: std::mem::ManuallyDrop<Option<WriterMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    published: bool,
    terminal: bool,
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
        Err(store::OwnedSchemaDecodeDiagnostic { code: "writer-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(store::RetainedCloneStep::Complete(store::RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct WriterEnvelopeOwnedFieldCatalog;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
