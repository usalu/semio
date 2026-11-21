//! 🗃️ Seals the paired PNG planes and typed placement rows in one canonical native Pack.

use super::plane_identity::{FACT_BYTES, PLANE_CAPACITY, PlaneFacts, PlaneIdentity, PlaneRole, PngPlanes};
use pack::{
    PackRefusal,
    codec::{ByteRange, ChunkId, CodecId, KIND_SYMBOLS, PackLimits},
    format::{Manifest, OPTIONAL_CANONICAL, OPTIONAL_HAS_SCHEMA, PackFile, PackIdentityChunkCursor,
        PackWriter, REQUIRED_CHUNKED, VerificationLevel, WriteOptions, encode_symbols},
};
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_hash::Sha256;
use semio_framework_pack_source::{PackSink, PackSource};
use semio_framework_value::{ValueRefusalKind, paged::{ByteReadSource, PAGED_BYTES_CHUNK_BYTES}};

pub(super) const SCHEMA: &str = "s.stdio.pdf.png-placement.v1";
const CHUNK_BYTES: usize = 32 * 1024;
const MAX_CHUNKS: usize = 2 + 2 * PLANE_CAPACITY.div_ceil(CHUNK_BYTES);
pub const MAX_ROWS_BYTES: usize = 8192;
const EMPTY: ByteRange = ByteRange { offset: 0, len: 0 };

pub(crate) fn invalid(detail: &str) -> PackRefusal {
    PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "PDF PNG paired plane placement", offset: 0, detail: detail.into() }
}

pub(super) fn schema_hash() -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"s.stdio.pdf.png-placement.v1:width:u32le,height:u32le,components:u8,alpha:u8,digest:raw32,plane-chunks:color-alpha:32768,rows:typed-pdf-placement-v1");
    hash.finalize()
}

/// 📦️ Borrows the native original event octets as an ordinary Pack source, without replay copying.
pub struct PagedPngPlacementPayload<'a>(pub &'a dyn ByteReadSource);

impl PackSource for PagedPngPlacementPayload<'_> {
    type Error = PackRefusal;

    async fn len(&self) -> u64 { self.0.byte_len() as u64 }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, PackRefusal> {
        let offset = usize::try_from(offset).map_err(|_| PackRefusal::Truncated(offset))?;
        let count = output.len().min(PAGED_BYTES_CHUNK_BYTES);
        Ok(self.0.read_bytes(offset, &mut output[..count]))
    }
}

/// 🧵️ Writes both original octet streams and the paired operation into one unpublished sink.
pub(crate) async fn seal<S: PackSink>(
    sink: S,
    planes: PngPlanes<'_>,
    facts: PlaneFacts,
    rows: &[u8],
    mut checkpoint: impl std::ops::AsyncFnMut(usize) -> Result<(), S::Error>,
) -> Result<S, S::Error> {
    if planes.shape() != facts.shape || rows.is_empty() || rows.len() > MAX_ROWS_BYTES {
        return Err(invalid("paired shape or placement row extent changed").into());
    }
    checkpoint(FACT_BYTES).await?;
    let config = WriteOptions { required_flags: REQUIRED_CHUNKED, optional_flags: OPTIONAL_CANONICAL | OPTIONAL_HAS_SCHEMA, codec: CodecId(0) };
    let mut writer = PackWriter::begin(sink, &config).await?;
    let name = SCHEMA.to_string();
    writer.write_segment(KIND_SYMBOLS, &encode_symbols(std::slice::from_ref(&name)).await).await?;
    let mut label = writer.begin_identity_chunk(FACT_BYTES).await?;
    label.write_fragment(&facts.encode()).await?;
    label.finish().await?;
    let mut identity = PlaneIdentity::new(facts.shape);
    let mut scratch = [0; PAGED_BYTES_CHUNK_BYTES];
    while let Some((role, offset)) = identity.position() {
        let mut remaining = (facts.shape.len(role) - offset).min(CHUNK_BYTES);
        let mut chunk = writer.begin_identity_chunk(remaining).await?;
        while remaining != 0 {
            let count = remaining.min(scratch.len());
            checkpoint(count).await?;
            if identity.read_one(planes, &mut scratch, count) != Some(count) {
                return Err(invalid("original plane extent changed").into());
            }
            chunk.write_fragment(&scratch[..count]).await?;
            remaining -= count;
        }
        chunk.finish().await?;
    }
    if identity.finish() != Some(facts) { return Err(invalid("paired source changed before sealing").into()); }
    let mut record = writer.begin_identity_chunk(rows.len()).await?;
    for fragment in rows.chunks(PAGED_BYTES_CHUNK_BYTES) {
        checkpoint(fragment.len()).await?;
        record.write_fragment(fragment).await?;
    }
    record.finish().await?;
    checkpoint(0).await?;
    writer.finish(&Manifest {
        schema_name: name, schema_hash: schema_hash(), doc_span: EMPTY, doc_frame_count: 0, symbols_span: EMPTY,
        chunk_table_span: EMPTY, field_index_span: EMPTY, uncompressed_body_len: 0, field_count: 0, chunk_count: 0, symbol_count: 0,
    }).await
}

/// 🧳️ Reopened native authority. Publication requires successful terminal plane verification.
pub struct PngPlacementArchive<S: PackSource> {
    pack: PackFile<S>,
    facts: PlaneFacts,
}

impl<S: PackSource> PngPlacementArchive<S> {
    pub async fn open(source: S, reference: &ArtifactRef) -> Result<Self, S::Error> {
        let limits = PackLimits { max_file_len: (2 * PLANE_CAPACITY + 2 * CHUNK_BYTES) as u64,
            max_segment_len: CHUNK_BYTES as u64, max_symbols: 1, max_depth: 2,
            max_items: MAX_CHUNKS as u64, max_total_alloc: (2 * CHUNK_BYTES) as u64 };
        let pack = PackFile::open_manifest(source, &limits, VerificationLevel::Full).await?;
        let manifest = pack.manifest().ok_or_else(|| invalid("paired placement manifest is missing"))?;
        if manifest.schema_name != SCHEMA || manifest.schema_hash != schema_hash()
            || manifest.doc_span.len != 0 || manifest.doc_frame_count != 0 || manifest.field_index_span.len != 0
            || manifest.field_count != 0 || manifest.uncompressed_body_len != 0 || manifest.symbol_count != 1
            || pack.chunk_decoded_len(ChunkId(0))? != FACT_BYTES as u64
        {
            return Err(invalid("paired placement manifest differs from its declared schema").into());
        }
        let mut encoded = [0; FACT_BYTES];
        let mut header = pack.identity_chunk_cursor(ChunkId(0), VerificationLevel::Full)?;
        if header.read_fragment(&mut encoded).await? != FACT_BYTES || header.read_fragment(&mut [0]).await? != 0 {
            return Err(invalid("paired native label changed extent").into());
        }
        let facts = PlaneFacts::decode(encoded).ok_or_else(|| invalid("paired plane shape is invalid"))?;
        let count = [PlaneRole::Color, PlaneRole::Alpha].into_iter().map(|role| facts.shape.len(role).div_ceil(CHUNK_BYTES)).sum::<usize>();
        if !facts.matches(PlaneRole::Color, reference) || pack.chunk_count() != (2 + count) as u64 {
            return Err(invalid("native reference or chunk roster differs").into());
        }
        Ok(Self { pack, facts })
    }

    pub fn facts(&self) -> PlaneFacts { self.facts }

    pub(crate) async fn operation(&self) -> Result<Vec<u8>, S::Error> {
        let id = ChunkId((self.pack.chunk_count() - 1) as u32);
        let length = self.pack.chunk_decoded_len(id)?;
        if length == 0 || length > MAX_ROWS_BYTES as u64 {
            return Err(invalid("paired placement operation exceeds its control extent").into());
        }
        let mut record = vec![0; length as usize];
        let mut reader = self.pack.identity_chunk_cursor(id, VerificationLevel::Full)?;
        for part in record.chunks_mut(PAGED_BYTES_CHUNK_BYTES) {
            if reader.read_fragment(part).await? != part.len() {
                return Err(invalid("paired placement operation truncated").into());
            }
        }
        if reader.read_fragment(&mut [0]).await? != 0 {
            return Err(invalid("paired placement operation has a trailing suffix").into());
        }
        Ok(record)
    }

    /// 🔎️ Lends an immutable one-window cursor to a private simultaneous color/alpha consumer.
    pub fn planes(&self) -> ArchivedPlaneCursor<'_, S> {
        ArchivedPlaneCursor { archive: self, id: 1, chunk: None,
            identity: Some(PlaneIdentity::new(self.facts.shape)), verified: false }
    }
}

pub struct ArchivedPlaneCursor<'a, S: PackSource> {
    archive: &'a PngPlacementArchive<S>,
    id: u32,
    chunk: Option<PackIdentityChunkCursor<'a, S>>,
    identity: Option<PlaneIdentity>,
    verified: bool,
}

impl<S: PackSource> ArchivedPlaneCursor<'_, S> {
    pub fn verified(&self) -> bool { self.verified }

    pub(crate) fn verified_for(&self, archive: &PngPlacementArchive<S>) -> bool {
        self.verified && std::ptr::eq(self.archive, archive)
    }

    /// 📤️ Returns one role-specific source prefix, or settles one boundary.
    pub async fn step(&mut self, output: &mut [u8]) -> Result<Option<(PlaneRole, usize)>, S::Error> {
        if output.is_empty() || self.verified { return Ok(None); }
        let identity = self.identity.as_mut().ok_or_else(|| invalid("plane proof already refused"))?;
        let expected = identity.position();
        if self.chunk.is_none() {
            let Some((role, offset)) = expected else {
                let identity = self.identity.take().expect("started PNG identity");
                if identity.finish() != Some(self.archive.facts) {
                    return Err(invalid("stored paired planes do not match the semantic digest").into());
                }
                self.verified = true;
                return Ok(None);
            };
            let extent = (self.archive.facts.shape.len(role) - offset).min(CHUNK_BYTES);
            let cursor = self.archive.pack.identity_chunk_cursor(ChunkId(self.id), VerificationLevel::Full)?;
            if cursor.len() != extent as u64 { return Err(invalid("native plane chunk has noncanonical extent").into()); }
            self.chunk = Some(cursor);
        }
        let count = output.len().min(PAGED_BYTES_CHUNK_BYTES);
        let read = self.chunk.as_mut().expect("admitted PNG chunk").read_fragment(&mut output[..count]).await?;
        if read == 0 {
            self.chunk = None;
            self.id += 1;
            return Ok(None);
        }
        let (role, _) = expected.ok_or_else(|| invalid("plane chunk exceeds the expected body"))?;
        identity.push(role, &output[..read]).ok_or_else(|| invalid("native plane chunk crossed its authorized role"))?;
        Ok(Some((role, read)))
    }
}
