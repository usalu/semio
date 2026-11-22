//! 💼️ The compact Job variant builds one typed owner under the existing ActorInput Source.
use super::{ActorInput, OriginalTurnFailure, RetainedCloneProgress, StepContext, ValueError, ValueRefusalKind, charge, invalid};
use semio_framework_actor::{OriginalJobSource, OriginalJobSourceWireHeader};
use std::mem::size_of;

pub(super) struct JobFrame {
    header: Option<OriginalJobSourceWireHeader>,
    maximum_bytes: u64,
    maximum_owned_bytes: usize,
    field: u8,
}

impl JobFrame {
    pub(super) fn select(wire: &[u8], maximum_bytes: u64, maximum_owned_bytes: usize) -> Option<Self> {
        wire.starts_with(&OriginalJobSourceWireHeader::MAGIC).then_some(Self {
            header: None, maximum_bytes, maximum_owned_bytes, field: 0,
        })
    }

    pub(super) fn advance(
        &mut self,
        wire: &[u8],
        out: &mut Option<ActorInput>,
        cx: &mut StepContext<'_>,
    ) -> Result<Result<bool, ValueError>, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if grant.maximum_depth < 3 { return Ok(Ok(false)); }

        let Some(header) = self.header else {
            let bytes = size_of::<Option<OriginalJobSourceWireHeader>>() + size_of::<Option<ActorInput>>() + OriginalJobSourceWireHeader::HEADER_BYTES;
            if !charge(cx, false, bytes)? { return Ok(Ok(false)); }
            let count = OriginalJobSourceWireHeader::HEADER_BYTES.min(wire.len());
            let header = match OriginalJobSourceWireHeader::read(&wire[..count], wire.len()) {
                Ok(header) => header,
                Err(error) => return Ok(Err(error)),
            };
            if wire.len() as u64 > self.maximum_bytes || wire.len() - count > self.maximum_owned_bytes {
                return Ok(Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "compact Job input exceeds its admitted extent")));
            }
            *out = Some(ActorInput::Job(OriginalJobSource {
                job: header.job,
                kind: String::new(),
                input: header.input_bytes.map(|_| Vec::new()),
                checkpoint: header.checkpoint_bytes.map(|_| Vec::new()),
            }));
            self.header = Some(header);
            return Ok(Ok(false));
        };

        let Some(ActorInput::Job(job)) = out.as_mut() else { return Err(invalid()); };
        if self.field == 3 { return Ok(Ok(true)); }
        let Some((start, length)) = header.field_range(self.field)? else {
            return self.finish_field(cx);
        };
        let mut target = match self.field {
            0 => Field::Kind(&mut job.kind),
            1 => Field::Bytes(job.input.as_mut().ok_or_else(invalid)?),
            2 => Field::Bytes(job.checkpoint.as_mut().ok_or_else(invalid)?),
            _ => return Err(invalid()),
        };
        let offset = target.len();
        if offset == length { return self.finish_field(cx); }

        if target.capacity() == 0 {
            let bytes = size_of::<Vec<u8>>();
            if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0
                || grant.maximum_capacity_bytes < length || grant.maximum_copy_bytes < bytes
            {
                return Ok(Ok(false));
            }
            cx.consume_fuel(1);
            if target.reserve(length).is_err() {
                return Ok(Err(ValueError::literal(ValueRefusalKind::AllocationFailed, "compact Job field allocation failed")));
            }
            cx.consume_retained(RetainedCloneProgress {
                copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: target.capacity(), ..Default::default()
            })?;
            return Ok(Ok(false));
        }

        let work_per_byte = if matches!(target, Field::Kind(_)) { 2 } else { 1 };
        let copy = size_of::<usize>();
        let count = (length - offset)
            .min(16_384)
            .min(grant.maximum_copy_bytes.saturating_sub(copy))
            .min(usize::try_from(cx.fuel_remaining().saturating_sub(1) / work_per_byte).unwrap_or(usize::MAX));
        if cx.should_yield() || cx.is_cancelled() || grant.maximum_items == 0 || count == 0 {
            return Ok(Ok(false));
        }
        let page = &wire[start + offset..start + offset + count];
        let actual = match &mut target {
            Field::Kind(kind) => {
                cx.consume_fuel(count as u64 * work_per_byte + 1);
                let text = match std::str::from_utf8(page) {
                    Ok(text) => text,
                    Err(error) if error.error_len().is_none() && offset + count < length => {
                        std::str::from_utf8(&page[..error.valid_up_to()]).map_err(|_| invalid())?
                    }
                    Err(_) => return Ok(Err(ValueError::literal(ValueRefusalKind::InvalidValue, "compact Job kind is invalid UTF-8"))),
                };
                kind.push_str(text);
                text.len()
            }
            Field::Bytes(bytes) => {
                cx.consume_fuel(count as u64 + 1);
                bytes.extend_from_slice(page);
                count
            }
        };
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: actual + copy, ..Default::default() })?;
        Ok(Ok(false))
    }

    fn finish_field(&mut self, cx: &mut StepContext<'_>) -> Result<Result<bool, ValueError>, OriginalTurnFailure> {
        if charge(cx, false, size_of::<u8>())? { self.field += 1; }
        Ok(Ok(self.field == 3))
    }
}

enum Field<'a> { Kind(&'a mut String), Bytes(&'a mut Vec<u8>) }

impl Field<'_> {
    fn len(&self) -> usize {
        match self { Self::Kind(value) => value.len(), Self::Bytes(value) => value.len() }
    }

    fn capacity(&self) -> usize {
        match self { Self::Kind(value) => value.capacity(), Self::Bytes(value) => value.capacity() }
    }

    fn reserve(&mut self, len: usize) -> Result<(), std::collections::TryReserveError> {
        match self { Self::Kind(value) => value.try_reserve_exact(len), Self::Bytes(value) => value.try_reserve_exact(len) }
    }
}
