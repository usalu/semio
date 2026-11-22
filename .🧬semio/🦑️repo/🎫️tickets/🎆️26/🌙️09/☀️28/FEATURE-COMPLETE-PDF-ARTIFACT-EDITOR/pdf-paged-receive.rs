//! 🫴️ Retains the authored PDF snapshot Pack envelope and Record over the original pages.

use pack::record::RetainedRecordReceiving;
use protocol::{OriginalWireSourceIdentity, PackLimits, RetainedWireByteSource};
use semio_framework_dsl_record::{BorrowedRecordSpec, RecordValue};
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant,
    RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll,
    RetirementDemand, ValueError, ValueRefusalKind, retained_clone::NativeAllocationIdentity,
};
use semio_framework_value_native_owned::native_decoding::{
    NativeDecodeControl, OriginalEffectReply, OriginalEffectReplyComplete, OriginalReceivingPrimary,
    owned::{NativeRecordKey, NativeSchemaKeySource, NativeSourceKeyReply, NativeSourceKeyReplyComplete, NativeSourceKeyValue,
        tree::{NativeScalarQuery, NativeSourceScalar, NativeTreeKind, NativeValueSource}},
};
use std::mem::size_of;
use store::semio_format::{Component, declared_binary_prefix_byte, declared_envelope_prefix_len};

enum State {
    Prefix(usize),
    AdmitRecord,
    Record(RetainedRecordReceiving),
    Closed,
}

pub(crate) struct PagedPdfPackRecord {
    source: OriginalWireSourceIdentity,
    owner: (NativeAllocationIdentity, Option<u64>),
    envelope: &'static str,
    prefix: usize,
    spec: BorrowedRecordSpec,
    limits: PackLimits,
    state: State,
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

fn move_demand<T>() -> RetirementDemand {
    RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() }
}

fn admits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0
        && grant.maximum_copy_bytes >= demand.copy_bytes
        && grant.maximum_capacity_bytes >= demand.capacity_bytes
        && grant.maximum_release_bytes >= demand.release_bytes
        && grant.maximum_depth >= demand.depth
}

fn receipt(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }
}

fn perform(effect: &mut RetainedAllocationEffect<'_>, work: u128) -> Result<(), ValueError> {
    let units = u64::try_from(work).map_err(|_| invalid("PDF snapshot work exceeds one turn"))?;
    if units == 0 || effect.performed(units)? {
        return Ok(());
    }
    Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF snapshot turn has no work grant"))
}

impl PagedPdfPackRecord {
    /// 🎟️ Records the exact Pack source and the actual decoder recipient before reading any byte.
    pub(crate) fn admit(
        source: &dyn RetainedWireByteSource,
        envelope: &'static str,
        spec: BorrowedRecordSpec,
        limits: PackLimits,
        native: &NativeDecodeControl<'_>,
        grant: RetainedCloneGrant,
    ) -> Result<Option<(Self, RetainedCloneProgress)>, ValueError> {
        let demand = move_demand::<Self>();
        if !admits(grant, demand) { return Ok(None); }
        let source = source.original_source_identity();
        let prefix = declared_envelope_prefix_len(envelope, Component::Pack, 1)?;
        if source.extent <= prefix || source.extent as u64 > limits.max_file_len {
            return Err(invalid("PDF snapshot Pack is outside its declared extent"));
        }
        let owner = (native.ledger_identity(), native.retirement_recipient_identity());
        Ok(Some((Self { source, owner, prefix, envelope, spec, limits, state: State::Prefix(0) }, receipt(demand))))
    }

    pub(crate) fn demands(&self, source: &dyn RetainedWireByteSource) -> Result<RetirementDemand, ValueError> {
        if source.original_source_identity() != self.source {
            return Err(invalid("PDF snapshot Pack page owner changed"));
        }
        match &self.state {
            State::Prefix(_) => Ok(move_demand::<u8>()),
            State::AdmitRecord => Ok(RetainedRecordReceiving::document_birth_demand()),
            State::Record(record) => record.demands(source),
            State::Closed => Ok(Default::default()),
        }
    }

    pub(crate) fn step(
        &mut self,
        source: &dyn RetainedWireByteSource,
        native: &mut NativeDecodeControl<'_>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let before = effect.performed_work();
        let result = self.advance(source, native, effect);
        RetainedWorkOutcome { result, work: effect.performed_work() - before }
    }

    fn advance(
        &mut self,
        source: &dyn RetainedWireByteSource,
        native: &mut NativeDecodeControl<'_>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, OriginalReceivingPrimary> {
        if (native.ledger_identity(), native.retirement_recipient_identity()) != self.owner {
            return Err(invalid("PDF snapshot Pack decoder recipient changed").into());
        }
        let demand = self.demands(source)?;
        let grant = effect.grant();
        if !admits(grant, demand) || effect.remaining_work_units()? == 0 { return Ok(Default::default()); }
        match &mut self.state {
            State::Prefix(index) => {
                native.checkpoint()?;
                perform(effect, 1)?;
                let expected = declared_binary_prefix_byte(self.envelope, Component::Pack, 1, *index)?;
                let actual = source.byte_at(*index).ok_or_else(|| invalid("PDF snapshot Pack prefix is missing a byte"))?;
                if expected != Some(actual) {
                    return Err(invalid("PDF snapshot Pack envelope differs from its authored identity").into());
                }
                *index += 1;
                if *index == self.prefix { self.state = State::AdmitRecord; }
                Ok(receipt(demand))
            }
            State::AdmitRecord => {
                native.checkpoint()?;
                let turn = RetainedRecordReceiving::admit_document(source, (self.prefix, self.source.extent), self.spec, self.limits.clone(), grant);
                perform(effect, turn.work)?;
                if let Some((record, progress)) = turn.result? {
                    self.state = State::Record(record);
                    return Ok(progress);
                }
                Ok(Default::default())
            }
            State::Record(record) => record.step(source, native, effect).result,
            State::Closed => Ok(Default::default()),
        }
    }

    pub(crate) fn record_is_ready(&self) -> bool {
        matches!(&self.state, State::Record(record) if record.is_ready())
    }

    /// 📬️ Transfers the schema-checked Record to its paid domain projector exactly once.
    pub(crate) fn take_record_into(
        &mut self,
        output: &mut Option<NamedPdfSnapshotRecord>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = move_demand::<Option<NamedPdfSnapshotRecord>>();
        let grant = effect.grant();
        if output.is_some() || !self.record_is_ready() || !admits(grant, demand) || effect.remaining_work_units()? == 0 {
            return Ok(Default::default());
        }
        perform(effect, 1)?;
        let State::Record(record) = &mut self.state else { unreachable!() };
        let mut original = None;
        record.take_into(&mut original, grant).result?;
        *output = original.map(|record| NamedPdfSnapshotRecord { record, spec: self.spec });
        Ok(receipt(demand))
    }

    pub(crate) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        match &self.state {
            State::Record(record) if !record.terminal_is_empty() => record.close_demands(body),
            State::Closed => Ok(Default::default()),
            _ => Ok(move_demand::<bool>()),
        }
    }

    pub(crate) fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let body = effect.grant().maximum_copy_bytes;
        let before = effect.performed_work();
        match &mut self.state {
            State::Record(record) if !record.terminal_is_empty() => {
                return match record.close_step(body, effect) {
                    RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                    RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                        result: turn.result.map(|step| RetainedCloneStep::Progress(step.progress())), work: turn.work,
                    }),
                };
            }
            State::Closed => return RetainedWorkPoll::Complete(RetainedWorkOutcome {
                result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0,
            }),
            _ => {}
        }
        let result = (|| {
            let demand = move_demand::<bool>();
            if !admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            perform(effect, 1)?;
            self.state = State::Closed;
            Ok(RetainedCloneStep::Complete(receipt(demand)))
        })();
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() - before })
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        matches!(self.state, State::Closed)
    }
}

impl Drop for PagedPdfPackRecord {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "PDF snapshot receiver abandoned paid custody");
    }
}

/// 🔖️ Names the canonical ordinal fields without building a second record or copying any value.
pub(crate) struct NamedPdfSnapshotRecord {
    record: RecordValue,
    spec: BorrowedRecordSpec,
}

impl semio_framework_value::retirement::RetireOwned for NamedPdfSnapshotRecord {
    type RetirementCursor = <RecordValue as semio_framework_value::retirement::RetireOwned>::RetirementCursor;

    fn retirement_seed(self) -> Self::RetirementCursor {
        self.record.retirement_seed()
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        self.record.retirement_birth_bytes()
    }

    fn controlled_retirement_supported() -> bool {
        RecordValue::controlled_retirement_supported()
    }
}

impl NamedPdfSnapshotRecord {
    /// 🧾️ Returns the same canonical Record for a caller that already owns its retirement.
    pub(crate) fn record(&self) -> &RecordValue {
        &self.record
    }
}

impl NativeValueSource for NamedPdfSnapshotRecord {
    fn query_schema_key<'reply, 'source>(
        &'source self,
        query: NativeSchemaKeySource,
        effect: &mut RetainedAllocationEffect<'_>,
        reply: NativeSourceKeyReply<'reply, 'source>,
    ) -> Result<NativeSourceKeyReplyComplete<'reply>, ValueError> {
        let index = match query {
            NativeSchemaKeySource::Record(index) | NativeSchemaKeySource::RecordName(index) => index,
            _ => return self.record.query_schema_key(query, effect, reply),
        };
        if effect.remaining_work_units()? < 1 || !effect.performed(1)? {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF snapshot key query has no work grant"));
        }
        Ok(reply.complete(self.record_name(index).map(NativeSourceKeyValue::Name)))
    }

    fn query_scalar<'reply>(
        &self,
        query: NativeScalarQuery,
        effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'reply, NativeSourceScalar, ValueError>,
    ) -> Result<OriginalEffectReplyComplete<'reply, NativeSourceScalar, ValueError>, ValueError> {
        self.record.query_scalar(query, effect, reply)
    }

    fn recover_scalar<'reply>(
        &self,
        query: NativeScalarQuery,
        effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'reply, NativeSourceScalar, ValueError>,
    ) -> Result<OriginalEffectReplyComplete<'reply, NativeSourceScalar, ValueError>, ValueError> {
        self.record.recover_scalar(query, effect, reply)
    }

    fn as_source(&self) -> &dyn NativeValueSource { self }
    fn tree_kind(&self) -> Option<NativeTreeKind> { self.record.tree_kind() }
    fn record_len(&self) -> Result<usize, ValueError> { self.record.record_len() }

    fn record_key(&self, index: usize) -> Result<NativeRecordKey<'_>, ValueError> {
        let NativeRecordKey::Ordinal(id) = self.record.record_key(index)? else {
            return Err(invalid("PDF snapshot root field no longer has its authored ordinal"));
        };
        let field = usize::from(id).checked_sub(1)
            .and_then(|ordinal| self.spec.fields.get(ordinal))
            .filter(|field| field.id == id)
            .ok_or_else(|| invalid("PDF snapshot root contains an undeclared field"))?;
        Ok(NativeRecordKey::Name(field.key))
    }

    fn record_child(&self, index: usize) -> Result<&dyn NativeValueSource, ValueError> {
        self.record.record_child(index)
    }
}
