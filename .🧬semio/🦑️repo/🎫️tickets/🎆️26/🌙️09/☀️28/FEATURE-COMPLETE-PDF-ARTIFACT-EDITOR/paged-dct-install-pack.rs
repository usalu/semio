//! 🗃️ Canonical Pack event payload for installing one exact native PDF JPEG source.
use super::{dct_view::{PdfDctImageColor, PdfDctImageView}, PdfDctResourceIdentity};
use pack::{
    PackRefusal,
    codec::{ByteRange, ChunkId, CodecId, KIND_SYMBOLS, PackLimits},
    format::{Manifest, OPTIONAL_CANONICAL, OPTIONAL_HAS_SCHEMA, PackFile, PackIdentityChunkCursor,
        PackWriter, REQUIRED_CHUNKED, VerificationLevel, WriteOptions, encode_symbols},
};
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_pack_source::{PackSink, PackSource};
use semio_framework_value::{ValueRefusalKind, paged::{ByteReadSource, PAGED_BYTES_CHUNK_BYTES}};

const SCHEMA_NAME: &str = "s.stdio.pdf.dct-install.v1";
const JPEG_CAPACITY: usize = 4 * 1024 * 1024;
const CHUNK_BYTES: usize = 32 * 1024;
const MAX_CHUNKS: u64 = 1 + JPEG_CAPACITY.div_ceil(CHUNK_BYTES) as u64;
const LABEL_BYTES: usize = 42;
const EMPTY: ByteRange = ByteRange { offset: 0, len: 0 };

fn refusal(detail: &str) -> PackRefusal {
    PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "PDF DCT install payload", offset: 0, detail: detail.into() }
}

fn schema_hash() -> [u8; 32] {
    use semio_framework_hash::Sha256;
    let mut hash = Sha256::new();
    hash.update(b"s.stdio.pdf.dct-install.v1:length:u32le,width:u16le,height:u16le,components:u8,transform:0|1|255,digest:raw32,chunks:identity:32768");
    hash.finalize()
}

/// 🏷️ Immutable native facts authenticated by the enclosing event and checked on resource lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DctInstallFacts {
    pub dimensions: (u16, u16),
    pub color: PdfDctImageColor,
    pub digest: [u8; 32],
    pub length: usize,
}

impl DctInstallFacts {
    fn encode(self) -> [u8; LABEL_BYTES] {
        let mut out = [0; LABEL_BYTES];
        out[..4].copy_from_slice(&(self.length as u32).to_le_bytes());
        out[4..6].copy_from_slice(&self.dimensions.0.to_le_bytes());
        out[6..8].copy_from_slice(&self.dimensions.1.to_le_bytes());
        out[8] = if matches!(self.color, PdfDctImageColor::Gray { .. }) { 1 } else { 3 };
        out[9] = self.color.color_transform().map_or(255, |transform| transform as u8);
        out[10..].copy_from_slice(&self.digest);
        out
    }

    fn decode(raw: &[u8; LABEL_BYTES]) -> Result<Self, PackRefusal> {
        let length = u32::from_le_bytes(raw[..4].try_into().unwrap()) as usize;
        let width = u16::from_le_bytes(raw[4..6].try_into().unwrap());
        let height = u16::from_le_bytes(raw[6..8].try_into().unwrap());
        if !valid_extent((width, height), length) { return Err(refusal("invalid native length or dimensions")); }
        let transform = match raw[9] {
            0 => Some(false), 1 => Some(true), 255 => None,
            _ => return Err(refusal("invalid DCT transform")),
        };
        let color = match raw[8] {
            1 => PdfDctImageColor::Gray { transform },
            3 => PdfDctImageColor::Rgb { transform },
            _ => return Err(refusal("unsupported native colorspace")),
        };
        Ok(Self { dimensions: (width, height), color, digest: raw[10..].try_into().unwrap(), length })
    }

    /// 🪪️ Generates the same logical artifact reference as graph admission.
    pub fn reference(self) -> ArtifactRef { PdfDctResourceIdentity::reference_for_digest(self.digest) }
}

fn valid_extent(dimensions: (u16, u16), length: usize) -> bool {
    let pixels = u64::from(dimensions.0) * u64::from(dimensions.1);
    (4..=JPEG_CAPACITY).contains(&length) && (1..=64 * 1024 * 1024).contains(&pixels)
}

/// 🧵️ Writes one install event payload to an unpublished sink under caller-supplied page checkpoints.
pub async fn write_dct_install_pack<S: PackSink>(
    sink: S,
    view: PdfDctImageView<'_>,
    mut checkpoint: impl std::ops::AsyncFnMut(usize) -> Result<(), S::Error>,
) -> Result<S, S::Error> {
    let source = view.encoded();
    let facts = DctInstallFacts { dimensions: view.dimensions(), color: view.color(), digest: view.digest(), length: source.byte_len() };
    if !valid_extent(facts.dimensions, facts.length) { return Err(refusal("invalid native length or dimensions").into()); }
    checkpoint(LABEL_BYTES).await?;
    let config = WriteOptions { required_flags: REQUIRED_CHUNKED, optional_flags: OPTIONAL_CANONICAL | OPTIONAL_HAS_SCHEMA, codec: CodecId(0) };
    let mut writer = PackWriter::begin(sink, &config).await?;
    let name = SCHEMA_NAME.to_string();
    writer.write_segment(KIND_SYMBOLS, &encode_symbols(std::slice::from_ref(&name)).await).await?;
    let mut header = writer.begin_identity_chunk(LABEL_BYTES).await?;
    header.write_fragment(&facts.encode()).await?;
    header.finish().await?;

    let mut identity = PdfDctResourceIdentity::new(facts.color.color_transform())
        .ok_or_else(|| refusal("unsupported native color transform"))?;
    let mut offset = 0;
    let mut fragment = [0; PAGED_BYTES_CHUNK_BYTES];
    while offset < facts.length {
        let size = (facts.length - offset).min(CHUNK_BYTES);
        let mut chunk = writer.begin_identity_chunk(size).await?;
        let end = offset + size;
        while offset < end {
            let count = (end - offset).min(fragment.len());
            checkpoint(count).await?;
            if source.read_bytes(offset, &mut fragment[..count]) != count {
                return Err(refusal("original JPEG source extent changed").into());
            }
            identity.append(&fragment[..count]);
            chunk.write_fragment(&fragment[..count]).await?;
            offset += count;
        }
        chunk.finish().await?;
    }
    if identity.digest() != facts.digest { return Err(refusal("original JPEG digest changed").into()); }
    checkpoint(0).await?;
    writer.finish(&Manifest {
        schema_name: name, schema_hash: schema_hash(), doc_span: EMPTY, doc_frame_count: 0, symbols_span: EMPTY,
        chunk_table_span: EMPTY, field_index_span: EMPTY, uncompressed_body_len: 0, field_count: 0, chunk_count: 0, symbol_count: 0,
    }).await
}

/// 📖️ Borrows a persisted paged operation payload as a normal Pack source.
pub struct PagedDctInstallPayload<'a>(pub &'a dyn ByteReadSource);

impl PackSource for PagedDctInstallPayload<'_> {
    type Error = PackRefusal;

    async fn len(&self) -> u64 { self.0.byte_len() as u64 }

    async fn read_at(&self, offset: u64, output: &mut [u8]) -> Result<usize, PackRefusal> {
        let offset = usize::try_from(offset).map_err(|_| PackRefusal::Truncated(offset))?;
        let limit = output.len().min(PAGED_BYTES_CHUNK_BYTES);
        Ok(self.0.read_bytes(offset, &mut output[..limit]))
    }
}

/// 🗂️ Retains only the native Pack catalog and immutable source authority for a stored install event.
pub struct NativeDctInstallPack<S: PackSource> {
    pack: PackFile<S>,
    facts: DctInstallFacts,
}

impl<S: PackSource> NativeDctInstallPack<S> {
    /// 🔍️ Looks up the typed graph reference before admitting any original JPEG bytes into a consumer.
    pub async fn open(source: S, reference: &ArtifactRef) -> Result<Self, S::Error> {
        let expected = PdfDctResourceIdentity::parse_reference(reference)
            .ok_or_else(|| refusal("invalid logical DCT lookup key"))?;
        let limits = PackLimits { max_file_len: (JPEG_CAPACITY + CHUNK_BYTES) as u64,
            max_segment_len: CHUNK_BYTES as u64, max_symbols: 1, max_depth: 2,
            max_items: MAX_CHUNKS, max_total_alloc: CHUNK_BYTES as u64 };
        let pack = PackFile::open_manifest(source, &limits, VerificationLevel::Full).await?;
        let manifest = pack.manifest().ok_or_else(|| refusal("missing native install manifest"))?;
        if manifest.schema_name != SCHEMA_NAME || manifest.schema_hash != schema_hash()
            || manifest.doc_span.len != 0 || manifest.doc_frame_count != 0 || manifest.field_index_span.len != 0
            || manifest.field_count != 0 || manifest.uncompressed_body_len != 0 || manifest.symbol_count != 1
            || pack.chunk_decoded_len(ChunkId(0))? != LABEL_BYTES as u64
        {
            return Err(refusal("invalid install Pack schema or metadata extent").into());
        }
        let mut label = [0; LABEL_BYTES];
        let mut cursor = pack.identity_chunk_cursor(ChunkId(0), VerificationLevel::Full)?;
        if cursor.read_fragment(&mut label).await? != label.len() || cursor.read_fragment(&mut label[..1]).await? != 0 {
            return Err(refusal("invalid install metadata chunk").into());
        }
        let facts = DctInstallFacts::decode(&label)?;
        if facts.digest != expected || pack.chunk_count() != 1 + facts.length.div_ceil(CHUNK_BYTES) as u64 {
            return Err(refusal("native resource key or original chunk count differs").into());
        }
        Ok(Self { pack, facts })
    }

    /// 📐️ Lends decoded metadata without reading the native codestream.
    pub fn facts(&self) -> DctInstallFacts { self.facts }

    /// 🪜️ Creates a bounded verifier over the same persistent Pack source.
    pub fn verify(&self) -> DctInstallReader<'_, S> {
        DctInstallReader { source: self, chunk: None, index: 1, offset: 0,
            identity: PdfDctResourceIdentity::new(self.facts.color.color_transform()).expect("decoded native transform"),
            complete: false }
    }
}

/// 🛡️ Original bytes become eligible for publication only after the entire DCT identity verifies.
pub struct DctInstallReader<'a, S: PackSource> {
    source: &'a NativeDctInstallPack<S>,
    chunk: Option<PackIdentityChunkCursor<'a, S>>,
    index: u32,
    offset: usize,
    identity: PdfDctResourceIdentity,
    complete: bool,
}

impl<S: PackSource> DctInstallReader<'_, S> {
    pub fn complete(&self) -> bool { self.complete }

    /// 📥️ Lends at most one source fragment or verifies one chunk boundary under the caller's grant.
    pub async fn step(&mut self, output: &mut [u8]) -> Result<usize, S::Error> {
        if self.complete || output.is_empty() { return Ok(0); }
        if self.chunk.is_none() {
            if self.offset == self.source.facts.length {
                if self.identity.digest() != self.source.facts.digest {
                    return Err(refusal("stored DCT source differs from native identity").into());
                }
                self.complete = true;
                return Ok(0);
            }
            let size = (self.source.facts.length - self.offset).min(CHUNK_BYTES);
            let cursor = self.source.pack.identity_chunk_cursor(ChunkId(self.index), VerificationLevel::Full)?;
            if cursor.len() != size as u64 { return Err(refusal("original DCT Pack chunk has noncanonical extent").into()); }
            self.chunk = Some(cursor);
        }
        let limit = output.len().min(PAGED_BYTES_CHUNK_BYTES);
        let received = self.chunk.as_mut().expect("admitted native chunk").read_fragment(&mut output[..limit]).await?;
        if received == 0 {
            self.chunk = None;
            self.index += 1;
        } else {
            self.identity.append(&output[..received]);
            self.offset += received;
        }
        Ok(received)
    }
}
