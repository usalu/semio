//! 🔑️ Mutation decode authority of the owned envelope catalog: the mutation record decode state machine, its field authority and the rejected conflict authority. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dMutationDecodeState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

pub(super) struct Generation3dMutationDecodeAuthority {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub(super) state: Generation3dMutationDecodeState,
    pub(super) token: Option<store::os_store::owned_schema_decode::OwnedSchemaToken>,
    pub(super) relative: usize,
    pub(super) high: Option<u8>,
    drive_ingress: bool,
    pub(super) session: std::mem::ManuallyDrop<Option<Generation3dMutationSession>>,
    pub(super) value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dMutationDecodeAuthority {
    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let refusal = move |error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.mutation-close-demand", path).with_native(error);
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
            state: Generation3dMutationDecodeState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            drive_ingress: false,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    pub(super) fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    pub(super) fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }

    pub(super) fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation3dMutationDecodeState::Published | Generation3dMutationDecodeState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation> for Generation3dMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation3d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.mutation-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation3d-envelope.mutation-pack-length", token.start))?;
            let maximum_items = generation3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation3d-envelope.mutation-item-authority", token.start))?;
            *self.session = Some(Generation3dMutationSession::new(expected, maximum_items).map_err(|_| self.diagnostic("generation3d-envelope.mutation-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation3dMutationDecodeState::Ingest;
        }
        if self.state == Generation3dMutationDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start));
            }
            if self.drive_ingress {
                if let Some(exact) = self.session.as_mut().expect("P3 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))? {
                    let (progressed, _) = self
                        .session
                        .as_mut()
                        .expect("P3 retained mutation session")
                        .reserve_retained_allocation(exact)
                        .map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))?;
                    if !progressed {
                        return Err(self.diagnostic("generation3d-envelope.mutation-retained-allocation-stalled", retained.start + self.relative as u64));
                    }
                    cx.consume_fuel(1);
                    return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                self.session.as_mut().expect("P3 retained mutation session").grant().map_err(|_| self.diagnostic("generation3d-envelope.mutation-ingress-malformed", retained.start + self.relative as u64))?;
                self.drive_ingress = !self.session.as_ref().expect("P3 retained mutation session").ingress_ready();
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation3d-envelope.mutation-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P3 retained mutation session").seal().map_err(|_| self.diagnostic("generation3d-envelope.mutation-seal", retained.end))?;
                self.state = Generation3dMutationDecodeState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation3d-envelope.mutation-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P3 retained mutation session").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation3d-envelope.mutation-handback", retained.start + self.relative as u64))?;
                self.drive_ingress = true;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::Drive {
            cx.set_stage("generation3d-retained-mutation");
            if let Some(exact) = self.session.as_mut().expect("P3 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", token.start))? {
                let (progressed, _) = self
                    .session
                    .as_mut()
                    .expect("P3 retained mutation session")
                    .reserve_retained_allocation(exact)
                    .map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", token.start))?;
                if !progressed {
                    return Err(self.diagnostic("generation3d-envelope.mutation-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P3 retained mutation session").grant().map_err(|_| self.diagnostic("generation3d-envelope.mutation-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P3 retained mutation session").take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-handoff", token.start))?);
            self.state = Generation3dMutationDecodeState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P3 retained mutation session").next_retained_release_allocation_bytes().unwrap_or(0);
            match self
                .session
                .as_mut()
                .expect("P3 retained mutation session")
                .close_step(1, maximum_bytes)
                .map_err(|_| self.diagnostic("generation3d-envelope.mutation-session-close", token.start))?
            {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { .. } => return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {}
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation3dMutationDecodeState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldTarget<Generation3dMutation>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.state != Generation3dMutationDecodeState::Ready {
            return Err(self.diagnostic("generation3d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Generation3dMutationDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
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
            self.state = Generation3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            self.state = Generation3dMutationDecodeState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation3d-envelope.mutation-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted mutation session refused its close")))? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.mutation-retirement-fault", error))?;
            self.state = Generation3dMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.mutation-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation3dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation3d mutation authority reached Drop before publication or terminal-empty close");
    }
}

pub(super) struct Generation3dRejectedConflictAuthority {
    pub(super) terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for Generation3dRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "generation3d-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
