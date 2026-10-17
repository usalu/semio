//! 📸️ Snapshot decode authority of the owned envelope catalog: the pack snapshot state machine and its fresh-envelope field authority. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dPackSnapshotState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

pub(super) struct Generation3dPackSnapshotAuthority {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub(super) state: Generation3dPackSnapshotState,
    pub(super) token: Option<store::os_store::owned_schema_decode::OwnedSchemaToken>,
    pub(super) relative: usize,
    pub(super) high: Option<u8>,
    pub(super) session: std::mem::ManuallyDrop<Option<semio_s_artifact_procedural_3d_io_binary::snapshot::Generation3dMountedPackSession>>,
    pub(super) value: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dPackSnapshotAuthority {
    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let refusal = move |error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.snapshot-close-demand", path).with_native(error);
        if let Some(session) = self.session.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: session.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refusal);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.value).map_err(refusal);
        }
        Ok(Default::default())
    }

    pub(super) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation3dPackSnapshotState::AwaitToken,
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

    pub(super) fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation3dPackSnapshotState::Published | Generation3dPackSnapshotState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }

    pub(super) fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot> for Generation3dPackSnapshotAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation3d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.snapshot-pack-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = generation3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(semio_s_artifact_procedural_3d_io_binary::snapshot::generation3d_mounted_pack_session(expected, maximum_items).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation3dPackSnapshotState::Ingest;
        }
        if self.state == Generation3dPackSnapshotState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation3d-envelope.snapshot-token-replayed", token.start));
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P3 mounted pack session retained").seal().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Generation3dPackSnapshotState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if let Some(exact) = self.session.as_mut().expect("P3 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))? {
                let step = self.session.as_mut().expect("P3 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-source-allocation-stalled", retained.start + self.relative as u64));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P3 mounted pack session retained").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-handback", retained.start + self.relative as u64))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::Drive {
            cx.set_stage("generation3d-retained-canonical-pack");
            if let Some(exact) = self.session.as_mut().expect("P3 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-retained-allocation", token.start))? {
                let step = self.session.as_mut().expect("P3 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-retained-allocation", token.start))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P3 mounted pack session retained").grant().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P3 mounted pack session retained").take().ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-handoff", token.start))?);
            self.session.as_mut().expect("P3 mounted pack session retained").request_cancel();
            self.state = Generation3dPackSnapshotState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P3 mounted pack session retained").next_retained_release_allocation_bytes().unwrap_or(0);
            if matches!(
                self.session.as_mut().expect("P3 mounted pack session retained").close_step(1, maximum_bytes).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-session-close", token.start))?,
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { .. }
            ) {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation3dPackSnapshotState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation3d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldTarget<Generation3dSnapshot>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, Generation3dPackSnapshotState::Ready) {
            return Err(self.diagnostic("generation3d-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Generation3dPackSnapshotState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let path = self.path;
        let diagnostic = move |code: &'static str, error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, path).with_native(error);
        let empty = RetainedCloneProgress::default();
        if self.session.is_none() && self.retirement.is_none() && self.value.is_none() {
            self.state = Generation3dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            session.request_cancel();
            self.state = Generation3dPackSnapshotState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation3d-envelope.snapshot-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted pack session refused its close")))? {
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.snapshot-retirement-fault", error))?;
            self.state = Generation3dPackSnapshotState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.snapshot-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation3dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation3dPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation3d pack snapshot authority reached Drop before publication or bounded retirement");
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
