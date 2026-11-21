//! 🧳 Encodes the private sampled placement into a paid canonical Pack page owner.
use super::{
    plane_identity::{FACT_BYTES, PlaneFacts, PlaneIdentity, PlaneRole, PngPlanes},
    plane_pack::{MAX_ROWS_BYTES, SCHEMA, schema_hash},
};
use pack::{
    codec::{Crc32cCursor, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_END, KIND_MANIFEST, KIND_SYMBOLS, crc32c},
    format::{FOOTER_MAGIC, FOOTER_SIZE, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, HEADER_SIZE, MAGIC,
        OPTIONAL_CANONICAL, OPTIONAL_HAS_SCHEMA, REQUIRED_CHUNKED},
};
use semio_framework_hash::Hasher;
use semio_framework_pack_format::retained_encoding::{RetainedChunkEntry, prefix_byte, prefix_len, varint_byte, varint_len};
use semio_framework_value::{
    OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
    ValueRefusalKind, paged::{PagedBytes, PAGED_BYTES_CHUNK_BYTES},
    retained_clone::retained_clone_grant_admits, retirement::controlled::ControlledRetirement,
};
use semio_framework_value_persistent::list::{PagedListReservationFailure, PagedListReservationOutcome};

const CONTROL_BYTES: usize = 512;
const CONTROL_WORK: u64 = 3 * CONTROL_BYTES as u64;

fn fault(detail: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, detail)
}

struct Control {
    bytes: [u8; CONTROL_BYTES],
    len: usize,
}

impl Default for Control {
    fn default() -> Self { Self { bytes: [0; CONTROL_BYTES], len: 0 } }
}

impl Control {
    fn push(&mut self, input: &[u8]) {
        let end = self.len + input.len();
        self.bytes[self.len..end].copy_from_slice(input);
        self.len = end;
    }

    fn varint(&mut self, value: usize) {
        for index in 0..varint_len(value) { self.push(&[varint_byte(value, index)]); }
    }

    fn segment(&mut self, kind: u8, body: &[u8]) {
        let start = self.len;
        self.push(&[kind, 0]);
        self.varint(body.len());
        self.push(body);
        self.push(&crc32c(&self.bytes[start..self.len]).to_le_bytes());
    }

    fn header() -> Self {
        let mut out = Self::default();
        out.push(&MAGIC);
        out.push(&FORMAT_VERSION_MAJOR.to_le_bytes());
        out.push(&FORMAT_VERSION_MINOR.to_le_bytes());
        out.push(&REQUIRED_CHUNKED.to_le_bytes());
        out.push(&(OPTIONAL_CANONICAL | OPTIONAL_HAS_SCHEMA).to_le_bytes());
        out.push(&crc32c(&out.bytes[..out.len]).to_le_bytes());
        out.push(&[0; 8]);
        let mut symbols = Self::default();
        symbols.varint(1);
        symbols.varint(SCHEMA.len());
        symbols.push(SCHEMA.as_bytes());
        out.segment(KIND_SYMBOLS, &symbols.bytes[..symbols.len]);
        out
    }

    fn tail(at: usize, symbols: usize, entries: &[Option<RetainedChunkEntry>]) -> Result<Self, ValueError> {
        let mut table = Self::default();
        table.varint(entries.len());
        for entry in entries {
            let entry = entry.as_ref().ok_or_else(|| fault("PDF PNG Pack has an unfinished chunk index"))?;
            for index in 0..entry.raw_len() { table.push(&[entry.byte(index)?]); }
        }
        let mut out = Self::default();
        out.segment(KIND_CHUNK_TABLE, &table.bytes[..table.len]);
        let manifest_offset = at + out.len;
        let mut body = Self::default();
        body.varint(0);
        body.push(&schema_hash());
        for value in [0, 0, 0, HEADER_SIZE, symbols, at, out.len, 0, 0, 0, 0, entries.len(), 1] {
            body.varint(value);
        }
        out.segment(KIND_MANIFEST, &body.bytes[..body.len]);
        let manifest_len = at + out.len - manifest_offset;
        out.segment(KIND_END, &[]);
        let file_len = at + out.len + FOOTER_SIZE;
        let start = out.len;
        out.push(&FOOTER_MAGIC);
        out.push(&FORMAT_VERSION_MAJOR.to_le_bytes());
        out.push(&FORMAT_VERSION_MINOR.to_le_bytes());
        out.push(&REQUIRED_CHUNKED.to_le_bytes());
        for value in [manifest_offset, manifest_len, file_len] { out.push(&(value as u64).to_le_bytes()); }
        out.push(semio_framework_hash::hash(&[]).as_bytes());
        out.push(&0_u64.to_le_bytes());
        out.push(&crc32c(&out.bytes[start..out.len]).to_le_bytes());
        Ok(out)
    }
}

#[derive(Clone, Copy)]
enum Body { Facts, Color, Alpha, Rows }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Header, ChunkPrefix, ChunkBody, ChunkSuffix, Tail, Ready, Revoked }

struct Chunk {
    payload: usize,
    written: usize,
    segment_crc: Crc32cCursor,
    payload_crc: Crc32cCursor,
    hash: Hasher,
}

pub type OriginalPack<const N: usize> = ControlledRetirement<PagedBytes<N>>;

pub struct RawPngPlacementPack<const N: usize> {
    output: Option<OriginalPack<N>>,
    facts: PlaneFacts,
    identity: Option<PlaneIdentity>,
    entries: [Option<RetainedChunkEntry>; 4],
    chunk_index: usize,
    chunk: Option<Chunk>,
    control: Control,
    copied_control: usize,
    symbols: usize,
    phase: Phase,
    rows: (usize, usize),
    revision: [u8; 32],
}

impl<const N: usize> RawPngPlacementPack<N> {
    pub fn birth_demand() -> RetirementDemand {
        RetirementDemand { copy_bytes: size_of::<Option<Self>>(), depth: 1, ..Default::default() }
    }

    /// 🎟️ Binds the bounded typed operation record and sampled plane digest to one exact revision.
    pub fn admit(
        result: &mut Option<Self>, facts: PlaneFacts, rows: &[u8], revision: [u8; 32],
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        let demand = Self::birth_demand();
        if result.is_some() || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(CONTROL_WORK)? {
            return Ok(Default::default());
        }
        let samples = facts.shape.len(PlaneRole::Color) + facts.shape.len(PlaneRole::Alpha);
        if samples > 4 * 4096 || rows.len() <= 32 || rows.len() > MAX_ROWS_BYTES || rows[..32] != revision
            || samples + rows.len() + 1024 > N
        {
            return Err(fault("PDF PNG sampled placement exceeds its admitted Pack extent"));
        }
        let control = Control::header();
        *result = Some(Self {
            output: Some(ControlledRetirement::new(PagedBytes::default()).map_err(|(error, _)| error)?),
            identity: Some(PlaneIdentity::new(facts.shape)), facts,
            entries: Default::default(), chunk_index: 0, chunk: None,
            copied_control: 0, symbols: control.len - HEADER_SIZE, control, phase: Phase::Header,
            rows: (rows.as_ptr() as usize, rows.len()), revision,
        });
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    pub fn is_ready(&self) -> bool { self.phase == Phase::Ready }

    pub fn revoke(&mut self) { self.phase = Phase::Revoked; }

    pub fn matches_control(&self, record: &[u8]) -> bool {
        self.rows == (record.as_ptr() as usize, record.len())
    }

    fn count(&self) -> usize { 3 + usize::from(self.facts.shape.has_alpha()) }

    fn body(&self) -> Body {
        match self.chunk_index {
            0 => Body::Facts,
            1 => Body::Color,
            2 if self.facts.shape.has_alpha() => Body::Alpha,
            _ => Body::Rows,
        }
    }

    fn body_len(&self) -> usize {
        match self.body() {
            Body::Facts => FACT_BYTES,
            Body::Color => self.facts.shape.len(PlaneRole::Color),
            Body::Alpha => self.facts.shape.len(PlaneRole::Alpha),
            Body::Rows => self.rows.1,
        }
    }

    fn transition(&mut self, at: usize) -> Result<(), ValueError> {
        self.copied_control = 0;
        self.control = Control::default();
        self.phase = match self.phase {
            Phase::Header | Phase::ChunkSuffix if self.chunk_index < self.count() => {
                let length = self.body_len();
                for index in 0..prefix_len(length, length, false) {
                    self.control.push(&[prefix_byte(KIND_CHUNK, length, length, false, index)]);
                }
                let mut crc = Crc32cCursor::new();
                crc.update_page(&self.control.bytes[..self.control.len]);
                self.chunk = Some(Chunk { payload: at + self.control.len, written: 0,
                    segment_crc: crc, payload_crc: Crc32cCursor::new(), hash: Hasher::new() });
                Phase::ChunkPrefix
            }
            Phase::ChunkPrefix => Phase::ChunkBody,
            Phase::ChunkBody => {
                let chunk = self.chunk.take().ok_or_else(|| fault("PDF PNG Pack lost its chunk cursor"))?;
                self.control.push(&chunk.segment_crc.finish().to_le_bytes());
                self.entries[self.chunk_index] = Some(RetainedChunkEntry {
                    offset: chunk.payload, length: chunk.written, crc: chunk.payload_crc.finish(),
                    hash: *chunk.hash.finalize().as_bytes(),
                });
                self.chunk_index += 1;
                Phase::ChunkSuffix
            }
            Phase::ChunkSuffix => {
                let facts = self.identity.take().and_then(PlaneIdentity::finish);
                if facts != Some(self.facts) { return Err(fault("PDF PNG Pack plane pair changed after digest")); }
                self.control = Control::tail(at, self.symbols, &self.entries[..self.count()])?;
                Phase::Tail
            }
            Phase::Tail => Phase::Ready,
            _ => return Err(fault("PDF PNG Pack phase cannot advance")),
        };
        Ok(())
    }

    fn validate(&mut self, live: &[u8; 32], canceled: bool) -> Result<(), ValueError> {
        if canceled || live != &self.revision { self.phase = Phase::Revoked; }
        if self.phase == Phase::Revoked {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "PDF PNG placement no longer owns its captured revision"));
        }
        Ok(())
    }

    /// 📥 Advances exactly one framing transition, reservation or paid fixed page prefix.
    pub fn advance(
        &mut self, planes: PngPlanes<'_>, rows: &[u8], live: &[u8; 32], canceled: bool,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.validate(live, canceled)?;
        if planes.shape() != self.facts.shape || (rows.as_ptr() as usize, rows.len()) != self.rows {
            return Err(fault("PDF PNG Pack borrow changed its plane shape or typed record identity"));
        }
        if self.phase == Phase::Ready { return Ok(Default::default()); }
        let grant = effect.grant();
        let kind = self.body();
        let length = self.body_len();
        let offset = self.chunk.as_ref().map_or(0, |chunk| chunk.written);
        let body = self.phase == Phase::ChunkBody;
        let remaining = if body { length - offset } else { self.control.len - self.copied_control };
        let output = self.output.as_mut().and_then(ControlledRetirement::original_mut)
            .ok_or_else(|| fault("PDF PNG Pack has no original destination"))?;
        if remaining == 0 {
            let quote = RetirementDemand { copy_bytes: size_of::<Control>() + size_of::<Option<Chunk>>(), depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(grant, quote) || !effect.performed(CONTROL_WORK)? { return Ok(Default::default()); }
            let at = output.len();
            self.transition(at)?;
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: quote.copy_bytes, ..Default::default() });
        }
        let bytes = output.retained_bytes();
        if !bytes.has_reserved_slot() {
            if !retained_clone_grant_admits(grant, output.byte_backing_demand()?) { return Ok(Default::default()); }
            return match output.retained_bytes_mut().reserve_one_funded(effect).result {
                Ok(PagedListReservationOutcome::Ready(receipt)) => Ok(receipt),
                Ok(PagedListReservationOutcome::Denied) => Ok(Default::default()),
                Err(PagedListReservationFailure::Work(error)) => Err(error),
                Err(PagedListReservationFailure::Allocation { error, .. }) => Err(ValueError::from(error.refusal())
                    .with_retained_progress(RetainedCloneProgress { retained_capacity_bytes: error.allocated_bytes, ..Default::default() })),
            };
        }
        let work = usize::try_from(effect.remaining_work_units()?).unwrap_or(usize::MAX);
        let count = remaining.min(bytes.capacity() - output.len()).min(PAGED_BYTES_CHUNK_BYTES)
            .min(grant.maximum_copy_bytes).min(work / 4);
        let demand = RetirementDemand { copy_bytes: count, depth: bytes.next_push_depth_demand().map_err(ValueError::from)?,
            ..Default::default() };
        if count == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(4 * count as u64)? {
            return Ok(Default::default());
        }
        let mut scratch = [0; PAGED_BYTES_CHUNK_BYTES];
        let bytes = &mut scratch[..count];
        if body {
            let role = match kind {
                Body::Facts => { bytes.copy_from_slice(&self.facts.encode()[offset..offset + count]); None }
                Body::Color => Some(PlaneRole::Color),
                Body::Alpha => Some(PlaneRole::Alpha),
                Body::Rows => { bytes.copy_from_slice(&rows[offset..offset + count]); None }
            };
            if let Some(role) = role {
                if planes.loan(role).read_bytes(offset, bytes) != count
                    || self.identity.as_mut().and_then(|identity| identity.push(role, bytes)).is_none()
                {
                    self.phase = Phase::Revoked;
                    return Err(fault("PDF PNG Pack source could not supply its declared plane window"));
                }
            }
            let chunk = self.chunk.as_mut().ok_or_else(|| fault("PDF PNG Pack lost its original chunk"))?;
            chunk.segment_crc.update_page(bytes);
            chunk.payload_crc.update_page(bytes);
            chunk.hash.update(bytes);
            chunk.written += count;
        } else {
            let start = self.copied_control;
            bytes.copy_from_slice(&self.control.bytes[start..start + count]);
            self.copied_control += count;
        }
        for &byte in bytes.iter() { output.push_reserved_byte(byte)?; }
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() })
    }

    /// 🎁 Moves the complete private Pack only to an empty, funded transaction owner.
    pub fn take(
        &mut self, live: &[u8; 32], canceled: bool, output: &mut Option<OriginalPack<N>>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<RetainedCloneProgress, ValueError> {
        self.validate(live, canceled)?;
        let demand = RetirementDemand { copy_bytes: 2 * size_of::<Option<OriginalPack<N>>>(), depth: 1, ..Default::default() };
        if !self.is_ready() || self.output.is_none() || output.is_some()
            || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)?
        { return Ok(Default::default()); }
        *output = self.output.take();
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.output.as_ref().map_or(Ok(Default::default()), |owner| owner.next_demand(body))
    }

    pub fn retire(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Option<OriginalRetirementPoll> {
        self.phase = Phase::Revoked;
        self.output.as_mut().filter(|owner| !owner.terminal_is_empty()).map(|owner| owner.step(body, effect))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(ControlledRetirement::terminal_is_empty)
    }
}
