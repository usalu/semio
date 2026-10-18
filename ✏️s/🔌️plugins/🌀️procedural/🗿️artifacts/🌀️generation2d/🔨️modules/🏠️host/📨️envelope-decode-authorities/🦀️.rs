//! 📨️ Envelope decode authorities: the snapshot, mutation and owned field catalog decode implementations of the artifact envelope with their drops. See [`super`].

use super::*;

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot> for Generation2dPackSnapshotAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation2d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation2d-envelope.snapshot-pack-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dPackSnapshotState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation2d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation2d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = generation2d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(semio_s_artifact_procedural_2d_io_binary::snapshot::generation2d_mounted_pack_session(expected, maximum_items).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation2dPackSnapshotState::Ingest;
        }
        if self.state == Generation2dPackSnapshotState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation2d-envelope.snapshot-token-replayed", token.start));
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation2d-envelope.snapshot-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P2 mounted pack session retained").seal().map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Generation2dPackSnapshotState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if let Some(exact) = self.session.as_mut().expect("P2 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation2d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))? {
                let step = self.session.as_mut().expect("P2 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation2d-envelope.snapshot-source-allocation-stalled", retained.start + self.relative as u64));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation2d-envelope.snapshot-pack-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P2 mounted pack session retained").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-handback", retained.start + self.relative as u64))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dPackSnapshotState::Drive {
            cx.set_stage("generation2d-retained-canonical-pack");
            if let Some(exact) = self.session.as_mut().expect("P2 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation2d-envelope.snapshot-retained-allocation", token.start))? {
                let step = self.session.as_mut().expect("P2 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-retained-allocation", token.start))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation2d-envelope.snapshot-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P2 mounted pack session retained").grant().map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P2 mounted pack session retained").take().ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-pack-handoff", token.start))?);
            self.session.as_mut().expect("P2 mounted pack session retained").request_cancel();
            self.state = Generation2dPackSnapshotState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dPackSnapshotState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P2 mounted pack session retained").next_retained_release_allocation_bytes().unwrap_or(0);
            if matches!(
                self.session.as_mut().expect("P2 mounted pack session retained").close_step(1, maximum_bytes).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-session-close", token.start))?,
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { .. }
            ) {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation2dPackSnapshotState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation2d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldTarget<Generation2dSnapshot>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, Generation2dPackSnapshotState::Ready) {
            return Err(self.diagnostic("generation2d-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Generation2dPackSnapshotState::Published;
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
            self.state = Generation2dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            session.request_cancel();
            self.state = Generation2dPackSnapshotState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation2d-envelope.snapshot-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted pack session refused its close")))? {
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation2d-envelope.snapshot-retirement-fault", error))?;
            self.state = Generation2dPackSnapshotState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation2d-envelope.snapshot-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation2dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation2dPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation2d pack snapshot authority reached Drop before publication or bounded retirement");
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Generation2dMutation> for Generation2dMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation2d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation2d-envelope.mutation-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dMutationDecodeState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation2d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation2d-envelope.mutation-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation2d-envelope.mutation-pack-length", token.start))?;
            let maximum_items = generation2d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation2d-envelope.mutation-item-authority", token.start))?;
            *self.session = Some(Generation2dMutationSession::new(expected, maximum_items).map_err(|_| self.diagnostic("generation2d-envelope.mutation-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation2dMutationDecodeState::Ingest;
        }
        if self.state == Generation2dMutationDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation2d-envelope.mutation-token-replayed", token.start));
            }
            if self.drive_ingress {
                if let Some(exact) = self.session.as_mut().expect("P2 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation2d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))? {
                    let (progressed, _) = self
                        .session
                        .as_mut()
                        .expect("P2 retained mutation session")
                        .reserve_retained_allocation(exact)
                        .map_err(|_| self.diagnostic("generation2d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))?;
                    if !progressed {
                        return Err(self.diagnostic("generation2d-envelope.mutation-retained-allocation-stalled", retained.start + self.relative as u64));
                    }
                    cx.consume_fuel(1);
                    return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                self.session.as_mut().expect("P2 retained mutation session").grant().map_err(|_| self.diagnostic("generation2d-envelope.mutation-ingress-malformed", retained.start + self.relative as u64))?;
                self.drive_ingress = !self.session.as_ref().expect("P2 retained mutation session").ingress_ready();
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation2d-envelope.mutation-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P2 retained mutation session").seal().map_err(|_| self.diagnostic("generation2d-envelope.mutation-seal", retained.end))?;
                self.state = Generation2dMutationDecodeState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation2d-envelope.mutation-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P2 retained mutation session").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation2d-envelope.mutation-handback", retained.start + self.relative as u64))?;
                self.drive_ingress = true;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dMutationDecodeState::Drive {
            cx.set_stage("generation2d-retained-mutation");
            if let Some(exact) = self.session.as_mut().expect("P2 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation2d-envelope.mutation-retained-allocation", token.start))? {
                let (progressed, _) = self
                    .session
                    .as_mut()
                    .expect("P2 retained mutation session")
                    .reserve_retained_allocation(exact)
                    .map_err(|_| self.diagnostic("generation2d-envelope.mutation-retained-allocation", token.start))?;
                if !progressed {
                    return Err(self.diagnostic("generation2d-envelope.mutation-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P2 retained mutation session").grant().map_err(|_| self.diagnostic("generation2d-envelope.mutation-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P2 retained mutation session").take().ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-handoff", token.start))?);
            self.state = Generation2dMutationDecodeState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dMutationDecodeState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P2 retained mutation session").next_retained_release_allocation_bytes().unwrap_or(0);
            match self
                .session
                .as_mut()
                .expect("P2 retained mutation session")
                .close_step(1, maximum_bytes)
                .map_err(|_| self.diagnostic("generation2d-envelope.mutation-session-close", token.start))?
            {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { .. } => return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {}
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation2dMutationDecodeState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation2d-envelope.mutation-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldTarget<Generation2dMutation>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.state != Generation2dMutationDecodeState::Ready {
            return Err(self.diagnostic("generation2d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Generation2dMutationDecodeState::Published;
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
            self.state = Generation2dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            self.state = Generation2dMutationDecodeState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation2d-envelope.mutation-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted mutation session refused its close")))? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation2d-envelope.mutation-retirement-fault", error))?;
            self.state = Generation2dMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation2d-envelope.mutation-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation2dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation2dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation2d mutation authority reached Drop before publication or terminal-empty close");
    }
}

/// 📦️ Installs Generation2d's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn generation2d_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Generation2dSnapshot, Generation2dMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(Generation2dEnvelopeOwnedFieldCatalog), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dSnapshot>::default()), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dMutation>::default()))
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<Generation2dSnapshot, Generation2dMutation> for Generation2dEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<Generation2dSnapshot, Generation2dMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dMutation>::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<Generation2dSnapshot, Generation2dMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot>> {
        Box::new(Generation2dPackSnapshotAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Generation2dMutation>> {
        Box::new(Generation2dMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(Generation2dRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<Generation2dMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation2dMutation>::default()))
    }
}
