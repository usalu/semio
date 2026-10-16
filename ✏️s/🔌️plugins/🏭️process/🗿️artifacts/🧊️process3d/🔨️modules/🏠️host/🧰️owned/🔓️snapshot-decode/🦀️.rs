//! 🔓️ process3d snapshot decode: the snapshot field decode authority and its state machine.

use super::*;

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Process3dSnapshot> for Process3dSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("process3d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("process3d-envelope.snapshot-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Process3dSnapshotDecodeState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("process3d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("process3d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = process3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("process3d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::binary::snapshot::process3d_mounted_pack_session(expected, maximum_items).map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Process3dSnapshotDecodeState::Ingest;
        }
        if self.state == Process3dSnapshotDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("process3d-envelope.snapshot-token-replayed", token.start));
            }
            let offset = retained.start + self.relative as u64;
            if offset + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("process3d-envelope.snapshot-pack-odd-hex", offset));
                }
                self.session().seal().map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Process3dSnapshotDecodeState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if self.reserve_pending("process3d-envelope.snapshot-source-allocation", offset, cx)? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("process3d-envelope.snapshot-pack-source", offset));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-pack-hex", offset))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session().admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("process3d-envelope.snapshot-pack-handback", offset))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Process3dSnapshotDecodeState::Drive {
            cx.set_stage("process3d-retained-canonical-pack");
            if self.reserve_pending("process3d-envelope.snapshot-retained-allocation", token.start, cx)? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session().grant().map_err(|_| self.diagnostic("process3d-envelope.snapshot-structural-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let value = self.session().take().ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-handoff-missing", token.start))?;
            *self.value = Some(value);
            self.session().request_cancel();
            self.state = Process3dSnapshotDecodeState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Process3dSnapshotDecodeState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session().next_retained_release_allocation_bytes().unwrap_or(0);
            if matches!(self.session().close_step(1, maximum_bytes).map_err(|_| self.diagnostic("process3d-envelope.snapshot-session-close", token.start))?, store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { .. }) {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Process3dSnapshotDecodeState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("process3d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldTarget<Process3dSnapshot>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.state != Process3dSnapshotDecodeState::Ready {
            return Err(self.diagnostic("process3d-envelope.snapshot-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("process3d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Process3dSnapshotDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(self.diagnostic("process3d-envelope.snapshot-close-depth", 0));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.session.is_some() {
            self.session().request_cancel();
            let step = self.session().close_step(1, grant.maximum_release_bytes).map_err(|_| self.diagnostic("process3d-envelope.snapshot-session-close", 0))?;
            self.state = Process3dSnapshotDecodeState::Closing;
            if let store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { released_items, released_bytes } = step {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty }));
            }
            drop(self.session.take());
            self.token = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
                return match semio_framework_value::retirement::admit_owned_retirement(value, child) {
                    Ok((owner, progress)) => {
                        *self.retirement = Some(owner);
                        self.state = Process3dSnapshotDecodeState::Closing;
                        if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes {
                            return Err(self.diagnostic("process3d-envelope.snapshot-retirement-birth", 0).with_progress(progress));
                        }
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                    Err((error, value)) => {
                        *self.value = Some(value);
                        Err(self.diagnostic("process3d-envelope.snapshot-retirement-fault", 0).with_native(error))
                    }
                };
            }
            self.state = Process3dSnapshotDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| self.diagnostic("process3d-envelope.snapshot-retirement-fault", 0).with_native(error))?;
        if self.retirement.is_none() {
            self.state = Process3dSnapshotDecodeState::Complete;
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, Process3dSnapshotDecodeState::Published | Process3dSnapshotDecodeState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for Process3dSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d snapshot field reached Drop before publication or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dSnapshotDecodeState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

/// 🧵️ The envelope's `snapshot` field: hex pack bytes streamed nibble by nibble into this
/// artifact's mounted canonical pack session, then the typed snapshot published exactly once.
pub(super) struct Process3dSnapshotDecodeAuthority {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub(super) state: Process3dSnapshotDecodeState,
    pub(super) token: Option<store::os_store::owned_schema_decode::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    session: std::mem::ManuallyDrop<Option<semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::binary::snapshot::Process3dMountedPackSession>>,
    pub(super) value: std::mem::ManuallyDrop<Option<Process3dSnapshot>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Process3dSnapshotDecodeAuthority {
    pub(super) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Process3dSnapshotDecodeState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    pub(super) fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let native = |error: semio_framework_value::ValueError| self.diagnostic("process3d-envelope.snapshot-close-demand", 0).with_native(error);
        if let Some(session) = self.session.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: session.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() });
        }
        if let Some(retirement) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body).map_err(native);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value).map_err(native);
        }
        Ok(Default::default())
    }

    fn session(&mut self) -> &mut semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::binary::snapshot::Process3dMountedPackSession {
        self.session.as_mut().expect("Process3d mounted pack session retained")
    }

    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }

    fn reserve_pending(&mut self, code: &'static str, offset: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let Some(exact) = self.session().next_retained_allocation_bytes().map_err(|_| self.diagnostic(code, offset))? else { return Ok(false) };
        let step = self.session().reserve_retained_allocation(exact).map_err(|_| self.diagnostic(code, offset))?;
        if !step.progressed {
            return Err(self.diagnostic("process3d-envelope.snapshot-allocation-stalled", offset));
        }
        cx.consume_fuel(1);
        Ok(true)
    }
}
