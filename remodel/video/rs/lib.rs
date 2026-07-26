//! 🎞️ Video container demuxing and baseline decode: ISO-BMFF/MP4 and RIFF/AVI demux, a hand-rolled H.264
//! baseline-profile decoder, minimal fixture-synthesis muxers/encoder, and a lazy frame-extraction API sitting
//! on top of [`remodel_image`]. DAG position: `remodel_image` → `remodel_video` → `remodel_engine`.

// #region 🔖Bytes
/// 🧭 Four-character box/chunk code (ISO-BMFF box types, RIFF FourCCs); compared and hashed by raw bytes.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FourCc(pub [u8; 4]);

impl FourCc {
    /// 🧭 Builds a `FourCc` from an ASCII byte-string literal, e.g. `FourCc::new(b"moov")`.
    pub const fn new(bytes: &[u8; 4]) -> Self {
        Self(*bytes)
    }
}

impl std::fmt::Debug for FourCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.iter().all(|&b| (0x20..0x7F).contains(&b)) {
            write!(f, "FourCc({:?})", std::str::from_utf8(&self.0).unwrap_or("?"))
        } else {
            write!(f, "FourCc({:02x}{:02x}{:02x}{:02x})", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }
}

impl std::fmt::Display for FourCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.iter().all(|&b| (0x20..0x7F).contains(&b)) {
            write!(f, "{}", std::str::from_utf8(&self.0).unwrap_or("????"))
        } else {
            write!(f, "{:02x}{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }
}

/// ⚠️ Error type for every fallible operation in this crate: demux, probe, decode. Malformed or truncated input
/// always yields one of these — decoders in this crate never panic on attacker-controlled bytes.
#[derive(Clone, Debug, PartialEq)]
pub enum VideoError {
    Truncated,
    BadBox(&'static str),
    NoVideoTrack,
    UnsupportedCodec(FourCc),
    Jpeg(remodel_image::ImageError),
    H264(H264Error),
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "video container truncated"),
            Self::BadBox(msg) => write!(f, "malformed container box/chunk: {msg}"),
            Self::NoVideoTrack => write!(f, "container has no video track"),
            Self::UnsupportedCodec(fourcc) => write!(f, "unsupported video codec: {fourcc}"),
            Self::Jpeg(e) => write!(f, "jpeg error: {e}"),
            Self::H264(e) => write!(f, "h264 error: {e}"),
        }
    }
}

impl std::error::Error for VideoError {}

impl From<remodel_image::ImageError> for VideoError {
    fn from(e: remodel_image::ImageError) -> Self {
        Self::Jpeg(e)
    }
}

impl From<H264Error> for VideoError {
    fn from(e: H264Error) -> Self {
        Self::H264(e)
    }
}

/// 🎞️ Video codec identified from a container's sample description; `Unknown` carries the raw fourcc for
/// diagnostics even when this crate cannot decode it (routing the caller to a host decoder).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoCodec {
    Avc,
    Hevc,
    Vp9,
    Av1,
    Mjpeg,
    Unknown(FourCc),
}

/// 📖 Bounds-checked cursor over a byte slice; every read fails cleanly with [`VideoError::Truncated`] instead
/// of panicking, and supports both ISO-BMFF's big-endian and RIFF's little-endian integer encodings.
pub(crate) struct ByteReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8], VideoError> {
        let end = self.pos.checked_add(n).ok_or(VideoError::Truncated)?;
        let slice = self.data.get(self.pos..end).ok_or(VideoError::Truncated)?;
        self.pos = end;
        Ok(slice)
    }

    pub(crate) fn skip(&mut self, n: usize) -> Result<(), VideoError> {
        self.take(n).map(|_| ())
    }

    pub(crate) fn u8(&mut self) -> Result<u8, VideoError> {
        Ok(self.take(1)?[0])
    }

    pub(crate) fn u16_be(&mut self) -> Result<u16, VideoError> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))
    }

    pub(crate) fn u32_be(&mut self) -> Result<u32, VideoError> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }

    pub(crate) fn u64_be(&mut self) -> Result<u64, VideoError> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().unwrap()))
    }

    pub(crate) fn u32_le(&mut self) -> Result<u32, VideoError> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    pub(crate) fn fourcc(&mut self) -> Result<FourCc, VideoError> {
        Ok(FourCc(self.take(4)?.try_into().unwrap()))
    }
}
// #endregion 🔖Bytes

// #region 🔖Bmff
/// 📦 One parsed ISO-BMFF box: its four-character type and payload (the bytes after the size/type header,
/// excluding any nested nesting logic — callers walk further with [`iter_boxes`]/[`find_box`] as needed).
struct Mp4Box<'a> {
    kind: FourCc,
    payload: &'a [u8],
}

/// 🚶 Iterates sibling boxes at one level of an ISO-BMFF byte range, honoring both the 32-bit inline size and
/// the 64-bit `largesize` extension (`size == 1`), and the "extends to end of data" convention (`size == 0`).
/// <https://www.iso.org/standard/74428.html> (ISO/IEC 14496-12)
struct Mp4BoxIter<'a> {
    data: &'a [u8],
    pos: usize,
}

fn iter_boxes(data: &[u8]) -> Mp4BoxIter<'_> {
    Mp4BoxIter { data, pos: 0 }
}

impl<'a> Iterator for Mp4BoxIter<'a> {
    type Item = Result<Mp4Box<'a>, VideoError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let mut r = ByteReader::new(&self.data[self.pos..]);
        let size32 = match r.u32_be() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let kind = match r.fourcc() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let (box_len, header_len) = if size32 == 1 {
            match r.u64_be() {
                Ok(v) => (v as usize, 16usize),
                Err(e) => return Some(Err(e)),
            }
        } else if size32 == 0 {
            (self.data.len() - self.pos, 8usize)
        } else {
            (size32 as usize, 8usize)
        };
        if box_len < header_len {
            return Some(Err(VideoError::BadBox("box size smaller than its own header")));
        }
        let box_end = match self.pos.checked_add(box_len) {
            Some(v) if v <= self.data.len() => v,
            _ => return Some(Err(VideoError::Truncated)),
        };
        let payload = &self.data[self.pos + header_len..box_end];
        self.pos = box_end;
        Some(Ok(Mp4Box { kind, payload }))
    }
}

/// 🔍 First direct-child box of the given type, or `Ok(None)` if absent; propagates a parse error from any
/// malformed sibling encountered along the way.
fn find_box<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Option<&'a [u8]>, VideoError> {
    for item in iter_boxes(data) {
        let b = item?;
        if b.kind.0 == *want {
            return Ok(Some(b.payload));
        }
    }
    Ok(None)
}

/// 🔍 Every direct-child box of the given type, in document order.
fn find_boxes<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Vec<&'a [u8]>, VideoError> {
    let mut out = Vec::new();
    for item in iter_boxes(data) {
        let b = item?;
        if b.kind.0 == *want {
            out.push(b.payload);
        }
    }
    Ok(out)
}

/// 🔍 Like [`find_box`], but a missing box is itself the error (`ctx` names it for diagnostics).
fn require_box<'a>(data: &'a [u8], want: &[u8; 4], ctx: &'static str) -> Result<&'a [u8], VideoError> {
    find_box(data, want)?.ok_or(VideoError::BadBox(ctx))
}

/// 🎞️ One decodable/probeable sample: its byte range in the source buffer, presentation timestamp, and
/// whether it is a sync (random-access) sample.
#[derive(Clone, Debug, PartialEq)]
pub struct SampleInfo {
    pub offset: u64,
    pub size: u32,
    pub timestamp_ms: f64,
    pub is_sync: bool,
}

/// 🎞️ Probed MP4/ISO-BMFF video track metadata: dimensions, timing, codec, and the resolved per-sample byte
/// ranges. Produced by [`probe_mp4`] for *any* parseable mp4, even when [`Mp4Info::codec`] is undecodable by
/// this crate — provenance and timestamps are always recoverable.
#[derive(Clone, Debug, PartialEq)]
pub struct Mp4Info {
    pub width: u32,
    pub height: u32,
    pub timescale: u32,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub codec: VideoCodec,
    pub samples: Vec<SampleInfo>,
    pub(crate) avc_config: Option<(Vec<u8>, u8)>,
}

/// 📥 `mdhd` (media header) → `(timescale, duration_in_timescale_units)`, handling both the 32-bit (version 0)
/// and 64-bit (version 1) field widths.
fn parse_mdhd(payload: &[u8]) -> Result<(u32, u64), VideoError> {
    let mut r = ByteReader::new(payload);
    let version = r.u8()?;
    r.skip(3)?;
    if version == 1 {
        r.skip(16)?;
        let timescale = r.u32_be()?;
        let duration = r.u64_be()?;
        Ok((timescale, duration))
    } else {
        r.skip(8)?;
        let timescale = r.u32_be()?;
        let duration = u64::from(r.u32_be()?);
        Ok((timescale, duration))
    }
}

/// 📥 `VisualSampleEntry` fixed fields → `(width, height, trailing_child_boxes)`; `payload` is the sample
/// entry's own box payload (i.e. the bytes right after its `size`/`type` header).
fn parse_visual_sample_entry(payload: &[u8]) -> Result<(u16, u16, &[u8]), VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(6)?;
    r.skip(2)?;
    r.skip(2)?;
    r.skip(2)?;
    r.skip(12)?;
    let width = r.u16_be()?;
    let height = r.u16_be()?;
    r.skip(4)?;
    r.skip(4)?;
    r.skip(4)?;
    r.skip(2)?;
    r.skip(32)?;
    r.skip(2)?;
    r.skip(2)?;
    Ok((width, height, &payload[r.pos()..]))
}

/// 📥 `avcC` (AVCDecoderConfigurationRecord) → `(sps_pps_nals, nal_length_size)`, where `sps_pps_nals` is a
/// flat sequence of `(u16 big-endian length, NAL bytes)` entries (SPS entries first, then PPS) — exactly the
/// format [`H264Decoder::new`] expects. <https://www.iso.org/standard/74428.html> (ISO/IEC 14496-15)
fn extract_avc_config(children: &[u8]) -> Result<(Vec<u8>, u8), VideoError> {
    let avcc = require_box(children, b"avcC", "avc1 sample entry missing avcC box")?;
    let mut r = ByteReader::new(avcc);
    r.skip(4)?;
    let length_size_byte = r.u8()?;
    let nal_length_size = (length_size_byte & 0x03) + 1;
    let mut sps_pps = Vec::new();
    let num_sps = r.u8()? & 0x1F;
    for _ in 0..num_sps {
        let len = usize::from(r.u16_be()?);
        let nal = r.take(len)?;
        sps_pps.extend_from_slice(&(len as u16).to_be_bytes());
        sps_pps.extend_from_slice(nal);
    }
    let num_pps = r.u8()?;
    for _ in 0..num_pps {
        let len = usize::from(r.u16_be()?);
        let nal = r.take(len)?;
        sps_pps.extend_from_slice(&(len as u16).to_be_bytes());
        sps_pps.extend_from_slice(nal);
    }
    Ok((sps_pps, nal_length_size))
}

/// 📥 `stsd` (sample description) → `(codec, width, height, avc_config)`; only the first sample entry is
/// consulted (multiple codec-switching entries per track are not something this crate's writers ever produce).
#[allow(clippy::type_complexity, reason = "the four probed fields (codec, width, height, optional avcC config) are each simple and self-explanatory in context; a named struct would just rename them without clarifying anything, and this is a private, single-call-site parser")]
fn parse_stsd(payload: &[u8]) -> Result<(VideoCodec, u16, u16, Option<(Vec<u8>, u8)>), VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let entry_count = r.u32_be()?;
    if entry_count == 0 {
        return Err(VideoError::BadBox("stsd has no sample entries"));
    }
    let rest = &payload[r.pos()..];
    let first = match iter_boxes(rest).next() {
        Some(b) => b?,
        None => return Err(VideoError::BadBox("stsd missing sample entry box")),
    };
    let fourcc = first.kind;
    let (width, height, children) = parse_visual_sample_entry(first.payload)?;
    let codec = match &fourcc.0 {
        b"avc1" | b"avc3" => VideoCodec::Avc,
        b"hvc1" | b"hev1" => VideoCodec::Hevc,
        b"vp09" => VideoCodec::Vp9,
        b"av01" => VideoCodec::Av1,
        b"mjpg" | b"jpeg" => VideoCodec::Mjpeg,
        _ => VideoCodec::Unknown(fourcc),
    };
    let avc_config = if codec == VideoCodec::Avc { Some(extract_avc_config(children)?) } else { None };
    Ok((codec, width, height, avc_config))
}

fn parse_stts(payload: &[u8]) -> Result<Vec<(u32, u32)>, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let count = r.u32_be()?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push((r.u32_be()?, r.u32_be()?));
    }
    Ok(out)
}

fn parse_ctts(payload: &[u8]) -> Result<Vec<(u32, i64)>, VideoError> {
    let mut r = ByteReader::new(payload);
    let version = r.u8()?;
    r.skip(3)?;
    let count = r.u32_be()?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let sample_count = r.u32_be()?;
        let raw = r.u32_be()?;
        let offset = if version == 1 { i64::from(raw as i32) } else { i64::from(raw) };
        out.push((sample_count, offset));
    }
    Ok(out)
}

fn parse_stsc(payload: &[u8]) -> Result<Vec<(u32, u32, u32)>, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let count = r.u32_be()?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push((r.u32_be()?, r.u32_be()?, r.u32_be()?));
    }
    Ok(out)
}

/// 📏 `stsz` sample sizes: either one uniform size shared by every sample, or an explicit per-sample table.
enum SampleSizes {
    Uniform { size: u32, count: u32 },
    PerSample(Vec<u32>),
}

impl SampleSizes {
    fn len(&self) -> usize {
        match self {
            Self::Uniform { count, .. } => *count as usize,
            Self::PerSample(v) => v.len(),
        }
    }

    fn get(&self, i: usize) -> Result<u32, VideoError> {
        match self {
            Self::Uniform { size, count } => {
                if (i as u32) < *count {
                    Ok(*size)
                } else {
                    Err(VideoError::BadBox("stsz sample index out of range"))
                }
            }
            Self::PerSample(v) => v.get(i).copied().ok_or(VideoError::BadBox("stsz sample index out of range")),
        }
    }
}

fn parse_stsz(payload: &[u8]) -> Result<SampleSizes, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let sample_size = r.u32_be()?;
    let count = r.u32_be()?;
    if sample_size != 0 {
        return Ok(SampleSizes::Uniform { size: sample_size, count });
    }
    let mut sizes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        sizes.push(r.u32_be()?);
    }
    Ok(SampleSizes::PerSample(sizes))
}

fn parse_stco(payload: &[u8]) -> Result<Vec<u64>, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let count = r.u32_be()?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(u64::from(r.u32_be()?));
    }
    Ok(out)
}

fn parse_co64(payload: &[u8]) -> Result<Vec<u64>, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let count = r.u32_be()?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(r.u64_be()?);
    }
    Ok(out)
}

fn parse_stss(payload: &[u8]) -> Result<Vec<u32>, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(4)?;
    let count = r.u32_be()?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(r.u32_be()?);
    }
    Ok(out)
}

fn samples_per_chunk_for(stsc: &[(u32, u32, u32)], chunk_number: u32) -> Result<u32, VideoError> {
    let mut result = None;
    for &(first_chunk, spc, _) in stsc {
        if first_chunk == 0 {
            return Err(VideoError::BadBox("stsc first_chunk must be >= 1"));
        }
        if first_chunk <= chunk_number {
            result = Some(spc);
        } else {
            break;
        }
    }
    result.ok_or(VideoError::BadBox("stsc does not cover this chunk"))
}

/// 🧮 Standard MP4 sample-table resolution: walks chunks in order, and within each chunk accumulates
/// per-sample byte offsets from the chunk's start, per `stsc`'s samples-per-chunk (which may change at chunk
/// boundaries) and `stsz`'s per-sample sizes.
fn resolve_samples(stsc: &[(u32, u32, u32)], chunk_offsets: &[u64], sizes: &SampleSizes) -> Result<Vec<(u64, u32)>, VideoError> {
    if stsc.is_empty() {
        return Err(VideoError::BadBox("stsc has no entries"));
    }
    let mut out = Vec::with_capacity(sizes.len());
    let mut sample_index = 0usize;
    for (i, &chunk_offset) in chunk_offsets.iter().enumerate() {
        let chunk_number = i as u32 + 1;
        let spc = samples_per_chunk_for(stsc, chunk_number)?;
        let mut offset = chunk_offset;
        for _ in 0..spc {
            if sample_index >= sizes.len() {
                break;
            }
            let size = sizes.get(sample_index)?;
            out.push((offset, size));
            offset = offset.checked_add(u64::from(size)).ok_or(VideoError::BadBox("chunk offset overflow"))?;
            sample_index += 1;
        }
    }
    Ok(out)
}

fn expand_run_length_u32(entries: &[(u32, u32)]) -> Vec<u32> {
    let mut out = Vec::new();
    for &(count, value) in entries {
        out.extend(std::iter::repeat_n(value, count as usize));
    }
    out
}

fn expand_run_length_i64(entries: &[(u32, i64)]) -> Vec<i64> {
    let mut out = Vec::new();
    for &(count, value) in entries {
        out.extend(std::iter::repeat_n(value, count as usize));
    }
    out
}

/// 📥 Probes an ISO-BMFF/MP4 byte stream's first video track: walks `ftyp/moov/trak/mdia/minf/stbl`, resolves
/// the full per-sample table (`stts`/`ctts`/`stsc`/`stsz`/`stco`|`co64`/`stss`), and detects the codec from
/// `stsd` (extracting `avcC` SPS/PPS when it is AVC). Succeeds for any well-formed mp4 track, even when the
/// codec is one this crate cannot decode — provenance/timestamps stay recoverable either way.
/// <https://www.iso.org/standard/74428.html> (ISO/IEC 14496-12)
pub fn probe_mp4(bytes: &[u8]) -> Result<Mp4Info, VideoError> {
    require_box(bytes, b"ftyp", "mp4 stream missing ftyp box")?;
    let moov = require_box(bytes, b"moov", "mp4 stream missing moov box")?;
    let traks = find_boxes(moov, b"trak")?;
    for trak in traks {
        let mdia = require_box(trak, b"mdia", "trak missing mdia")?;
        let hdlr = require_box(mdia, b"hdlr", "mdia missing hdlr")?;
        if hdlr.len() < 12 || &hdlr[8..12] != b"vide" {
            continue;
        }
        let mdhd = require_box(mdia, b"mdhd", "mdia missing mdhd")?;
        let (timescale, duration_ticks) = parse_mdhd(mdhd)?;
        let minf = require_box(mdia, b"minf", "mdia missing minf")?;
        let stbl = require_box(minf, b"stbl", "minf missing stbl")?;
        let stsd = require_box(stbl, b"stsd", "stbl missing stsd")?;
        let (codec, width, height, avc_config) = parse_stsd(stsd)?;

        let stts = require_box(stbl, b"stts", "stbl missing stts")?;
        let dts_deltas = expand_run_length_u32(&parse_stts(stts)?);
        let stsc_entries = parse_stsc(require_box(stbl, b"stsc", "stbl missing stsc")?)?;
        let sizes = parse_stsz(require_box(stbl, b"stsz", "stbl missing stsz")?)?;
        let chunk_offsets = match find_box(stbl, b"stco")? {
            Some(p) => parse_stco(p)?,
            None => parse_co64(require_box(stbl, b"co64", "stbl missing stco/co64")?)?,
        };
        let sync = match find_box(stbl, b"stss")? {
            Some(p) => Some(parse_stss(p)?),
            None => None,
        };
        let sample_count = sizes.len();
        if dts_deltas.len() != sample_count {
            return Err(VideoError::BadBox("stts sample count does not match stsz"));
        }
        let cts_offsets: Vec<i64> = match find_box(stbl, b"ctts")? {
            Some(p) => {
                let expanded = expand_run_length_i64(&parse_ctts(p)?);
                if expanded.len() != sample_count {
                    return Err(VideoError::BadBox("ctts sample count does not match stsz"));
                }
                expanded
            }
            None => vec![0i64; sample_count],
        };
        let offsets_sizes = resolve_samples(&stsc_entries, &chunk_offsets, &sizes)?;
        if offsets_sizes.len() != sample_count {
            return Err(VideoError::BadBox("stsc/stco resolved sample count does not match stsz"));
        }

        let mut samples = Vec::with_capacity(sample_count);
        let mut dts_accum: u64 = 0;
        for i in 0..sample_count {
            let (offset, size) = offsets_sizes[i];
            let pts_ticks = dts_accum as i64 + cts_offsets[i];
            let timestamp_ms = if timescale > 0 { pts_ticks as f64 * 1000.0 / f64::from(timescale) } else { 0.0 };
            let is_sync = sync.as_ref().is_none_or(|list| list.contains(&(i as u32 + 1)));
            samples.push(SampleInfo { offset, size, timestamp_ms, is_sync });
            dts_accum += u64::from(dts_deltas[i]);
        }

        let duration_ms = if timescale > 0 { duration_ticks as f64 * 1000.0 / f64::from(timescale) } else { 0.0 };
        return Ok(Mp4Info { width: u32::from(width), height: u32::from(height), timescale, duration_ms, frame_count: sample_count as u32, codec, samples, avc_config });
    }
    Err(VideoError::NoVideoTrack)
}
// #endregion 🔖Bmff

// #region 🔖Avi
/// 📦 One parsed RIFF chunk: its four-character id and payload (odd-length payloads are followed by a pad
/// byte per RIFF convention, which [`RiffIter`] skips automatically).
struct RiffChunk<'a> {
    id: FourCc,
    payload: &'a [u8],
}

/// 🚶 Iterates sibling RIFF chunks (little-endian sizes) at one level — `LIST` chunks are yielded whole
/// (their 4-byte list-type tag is the first 4 bytes of `payload`); callers recurse into them explicitly.
struct RiffIter<'a> {
    data: &'a [u8],
    pos: usize,
}

fn iter_riff(data: &[u8]) -> RiffIter<'_> {
    RiffIter { data, pos: 0 }
}

impl<'a> Iterator for RiffIter<'a> {
    type Item = Result<RiffChunk<'a>, VideoError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let mut r = ByteReader::new(&self.data[self.pos..]);
        let id = match r.fourcc() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let size = match r.u32_le() {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let payload_start = self.pos + 8;
        let payload_end = match payload_start.checked_add(size as usize) {
            Some(v) if v <= self.data.len() => v,
            _ => return Some(Err(VideoError::Truncated)),
        };
        let payload = &self.data[payload_start..payload_end];
        let padded_len = size as usize + (size as usize & 1);
        self.pos = payload_start + padded_len;
        Some(Ok(RiffChunk { id, payload }))
    }
}

fn find_riff_list<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Option<&'a [u8]>, VideoError> {
    for item in iter_riff(data) {
        let c = item?;
        if c.id.0 == *b"LIST" && c.payload.len() >= 4 && &c.payload[0..4] == want {
            return Ok(Some(&c.payload[4..]));
        }
    }
    Ok(None)
}

fn require_riff_list<'a>(data: &'a [u8], want: &[u8; 4], ctx: &'static str) -> Result<&'a [u8], VideoError> {
    find_riff_list(data, want)?.ok_or(VideoError::BadBox(ctx))
}

fn find_riff_lists<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Vec<&'a [u8]>, VideoError> {
    let mut out = Vec::new();
    for item in iter_riff(data) {
        let c = item?;
        if c.id.0 == *b"LIST" && c.payload.len() >= 4 && &c.payload[0..4] == want {
            out.push(&c.payload[4..]);
        }
    }
    Ok(out)
}

fn find_riff_chunk<'a>(data: &'a [u8], want: &[u8; 4]) -> Result<Option<&'a [u8]>, VideoError> {
    for item in iter_riff(data) {
        let c = item?;
        if c.id.0 == *want {
            return Ok(Some(c.payload));
        }
    }
    Ok(None)
}

fn require_riff_chunk<'a>(data: &'a [u8], want: &[u8; 4], ctx: &'static str) -> Result<&'a [u8], VideoError> {
    find_riff_chunk(data, want)?.ok_or(VideoError::BadBox(ctx))
}

/// 📍 Byte offset of subslice `sub` within `whole`, given `sub` is known (by construction, via chained
/// slicing) to lie inside `whole`'s backing allocation — pure address arithmetic, no dereference.
fn offset_in(whole: &[u8], sub: &[u8]) -> usize {
    sub.as_ptr() as usize - whole.as_ptr() as usize
}

/// 🎞️ Probed RIFF/AVI video stream metadata, mirroring [`Mp4Info`] for the AVI container family.
#[derive(Clone, Debug, PartialEq)]
pub struct AviInfo {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub frame_count: u32,
    pub codec: VideoCodec,
    pub samples: Vec<SampleInfo>,
}

/// 📥 `avih` (main AVI header, always exactly 56 bytes) → `(micro_sec_per_frame, total_frames, width, height)`.
fn parse_avih(payload: &[u8]) -> Result<(u32, u32, u32, u32), VideoError> {
    let mut r = ByteReader::new(payload);
    let micro_sec_per_frame = r.u32_le()?;
    r.skip(8)?;
    r.skip(4)?;
    let total_frames = r.u32_le()?;
    r.skip(8)?;
    r.skip(4)?;
    let width = r.u32_le()?;
    let height = r.u32_le()?;
    Ok((micro_sec_per_frame, total_frames, width, height))
}

/// 📥 `strh` (stream header) → `(fcc_type, dw_scale, dw_rate, dw_length)`; only the fields preceding the
/// legacy `rcFrame` rectangle (whose exact width is ambiguous across encoders) are read.
fn parse_strh(payload: &[u8]) -> Result<(FourCc, u32, u32, u32), VideoError> {
    let mut r = ByteReader::new(payload);
    let fcc_type = r.fourcc()?;
    r.skip(4)?;
    r.skip(4)?;
    r.skip(2)?;
    r.skip(2)?;
    r.skip(4)?;
    let scale = r.u32_le()?;
    let rate = r.u32_le()?;
    r.skip(4)?;
    let length = r.u32_le()?;
    Ok((fcc_type, scale, rate, length))
}

/// 📥 `strf` (video stream format, a `BITMAPINFOHEADER`) → `biCompression`, the codec fourcc.
fn parse_strf_video_compression(payload: &[u8]) -> Result<FourCc, VideoError> {
    let mut r = ByteReader::new(payload);
    r.skip(16)?;
    r.fourcc()
}

/// 📇 One `idx1` index entry: chunk id, `AVIIF_KEYFRAME` flag, and offset/size (offset's base is ambiguous —
/// see [`parse_idx1`]).
struct Idx1Entry {
    ckid: FourCc,
    flags: u32,
    chunk_offset: u32,
    chunk_size: u32,
}

fn parse_idx1_entries(idx1: &[u8]) -> Result<Vec<Idx1Entry>, VideoError> {
    let mut r = ByteReader::new(idx1);
    let mut out = Vec::new();
    while r.remaining() >= 16 {
        let ckid = r.fourcc()?;
        let flags = r.u32_le()?;
        let chunk_offset = r.u32_le()?;
        let chunk_size = r.u32_le()?;
        out.push(Idx1Entry { ckid, flags, chunk_offset, chunk_size });
    }
    Ok(out)
}

/// 📇 Resolves `idx1` entries to absolute `(data_offset, size, is_sync)` triples. `idx1` offsets are
/// conventionally relative to the start of the `movi` list's data, but some encoders write them relative to
/// the start of the file instead — both bases are probed against the first few entries' `ckid` (which must
/// match the four bytes actually found there), and the base that verifies is used for every entry. An empty
/// result (neither base verifies) signals the caller to fall back to [`scan_movi_chunks`].
fn parse_idx1(idx1: &[u8], movi: &[u8], bytes: &[u8]) -> Result<Vec<(u64, u32, bool)>, VideoError> {
    let entries = parse_idx1_entries(idx1)?;
    if entries.is_empty() {
        return Ok(Vec::new());
    }
    let movi_start = offset_in(bytes, movi);
    let candidates = [movi_start, 0usize];
    let mut chosen_base = None;
    'bases: for &base in &candidates {
        for e in entries.iter().take(entries.len().min(4)) {
            let matches = base
                .checked_add(e.chunk_offset as usize)
                .and_then(|start| start.checked_add(4).map(|end| (start, end)))
                .and_then(|(start, end)| bytes.get(start..end))
                .is_some_and(|found| found == e.ckid.0);
            if !matches {
                continue 'bases;
            }
        }
        chosen_base = Some(base);
        break;
    }
    let Some(base) = chosen_base else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for e in &entries {
        if &e.ckid.0[2..4] != b"dc" {
            continue;
        }
        let Some(header_start) = base.checked_add(e.chunk_offset as usize) else { continue };
        let Some(data_start) = header_start.checked_add(8) else { continue };
        let Some(data_end) = data_start.checked_add(e.chunk_size as usize) else { continue };
        if data_end > bytes.len() {
            continue;
        }
        out.push((data_start as u64, e.chunk_size, e.flags & 0x10 != 0));
    }
    Ok(out)
}

/// 📇 Fallback frame index built by scanning `movi`'s chunks directly (recursing one level into `rec ` sub-
/// lists, which interleave audio/video chunks per frame group) for `NNdc`-style compressed-video chunk ids;
/// used when `idx1` is absent or its offsets verify against neither convention. Every scanned MJPEG frame is
/// independently decodable, so all are reported as sync samples.
fn scan_movi_chunks<'a>(movi: &'a [u8], bytes: &'a [u8], out: &mut Vec<(u64, u32, bool)>) -> Result<(), VideoError> {
    for item in iter_riff(movi) {
        let c = item?;
        if c.id.0 == *b"LIST" && c.payload.len() >= 4 && &c.payload[0..4] == b"rec " {
            scan_movi_chunks(&c.payload[4..], bytes, out)?;
            continue;
        }
        if c.id.0.len() == 4 && &c.id.0[2..4] == b"dc" {
            out.push((offset_in(bytes, c.payload) as u64, c.payload.len() as u32, true));
        }
    }
    Ok(())
}

/// 📥 Probes a RIFF/AVI byte stream's first `vids` (video) stream: `avih`/`strh`/`strf` for dimensions, fps
/// and codec, then `idx1` for the per-frame index (falling back to a direct `movi` scan when `idx1` is
/// missing or inconsistent). Only `MJPG`-compressed streams are decodable by [`extract_frames`]; other
/// codecs still probe successfully for provenance.
pub fn probe_avi(bytes: &[u8]) -> Result<AviInfo, VideoError> {
    let mut r = ByteReader::new(bytes);
    let riff = r.fourcc()?;
    if riff.0 != *b"RIFF" {
        return Err(VideoError::BadBox("not a RIFF stream"));
    }
    r.skip(4)?;
    let form = r.fourcc()?;
    if form.0 != *b"AVI " {
        return Err(VideoError::BadBox("RIFF form is not AVI"));
    }
    let body = &bytes[r.pos()..];

    let hdrl = require_riff_list(body, b"hdrl", "avi stream missing hdrl list")?;
    let avih = require_riff_chunk(hdrl, b"avih", "hdrl missing avih chunk")?;
    let (micro_sec_per_frame, _total_frames, avih_width, avih_height) = parse_avih(avih)?;

    let strls = find_riff_lists(hdrl, b"strl")?;
    let mut stream: Option<(f64, VideoCodec)> = None;
    for strl in strls {
        let strh = require_riff_chunk(strl, b"strh", "strl missing strh chunk")?;
        let (fcc_type, scale, rate, _length) = parse_strh(strh)?;
        if fcc_type.0 != *b"vids" {
            continue;
        }
        let strf = require_riff_chunk(strl, b"strf", "strl missing strf chunk")?;
        let compression = parse_strf_video_compression(strf)?;
        let codec = match &compression.0 {
            b"MJPG" | b"mjpg" => VideoCodec::Mjpeg,
            _ => VideoCodec::Unknown(compression),
        };
        let fps = if scale > 0 { f64::from(rate) / f64::from(scale) } else if micro_sec_per_frame > 0 { 1_000_000.0 / f64::from(micro_sec_per_frame) } else { 0.0 };
        stream = Some((fps, codec));
        break;
    }
    let (fps, codec) = stream.ok_or(VideoError::NoVideoTrack)?;

    let movi = require_riff_list(body, b"movi", "avi stream missing movi list")?;
    let mut triples = match find_riff_chunk(body, b"idx1")? {
        Some(idx1) => parse_idx1(idx1, movi, bytes)?,
        None => Vec::new(),
    };
    if triples.is_empty() {
        scan_movi_chunks(movi, bytes, &mut triples)?;
    }

    let samples: Vec<SampleInfo> = triples
        .into_iter()
        .enumerate()
        .map(|(i, (offset, size, is_sync))| SampleInfo { offset, size, timestamp_ms: if fps > 0.0 { i as f64 * 1000.0 / fps } else { 0.0 }, is_sync })
        .collect();

    Ok(AviInfo { width: avih_width, height: avih_height, fps, frame_count: samples.len() as u32, codec, samples })
}
// #endregion 🔖Avi

// #region 🔖H264
/// ⚠️ Error type for the hand-rolled H.264 baseline decoder. `Unsupported` is used for any spec feature this
/// decoder deliberately does not implement (CABAC, B-slices, 8×8 transform, interlace, FMO, reference-list
/// reordering, adaptive memory control, sub-8×8 P-partitions) — always a loud, named failure, never silent
/// misdecoding. `Truncated`/`Malformed` cover corrupt or adversarial bitstreams; this decoder never panics on
/// untrusted input.
#[derive(Clone, Debug, PartialEq)]
pub enum H264Error {
    Truncated,
    Malformed(&'static str),
    Unsupported(&'static str),
    NoSps,
    NoPps,
}

impl std::fmt::Display for H264Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "h264 bitstream truncated"),
            Self::Malformed(msg) => write!(f, "malformed h264 bitstream: {msg}"),
            Self::Unsupported(msg) => write!(f, "unsupported h264 feature: {msg}"),
            Self::NoSps => write!(f, "h264 slice references an unparsed sps"),
            Self::NoPps => write!(f, "h264 slice references an unparsed pps"),
        }
    }
}

impl std::error::Error for H264Error {}

// #region 🔖Bits
/// 📖 MSB-first bit reader over an RBSP (emulation-prevention already stripped), providing the three H.264
/// bitstream primitives: `u(n)` fixed-width, `ue(v)` Exp-Golomb unsigned, `se(v)` Exp-Golomb signed.
/// <https://www.itu.int/rec/T-REC-H.264> (clause 9.1)
struct BitReader<'a> {
    data: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, byte_pos: 0, bit_pos: 0 }
    }

    fn u1(&mut self) -> Result<u32, H264Error> {
        let &byte = self.data.get(self.byte_pos).ok_or(H264Error::Truncated)?;
        let bit = (byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        if self.bit_pos == 8 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
        Ok(u32::from(bit))
    }

    fn u(&mut self, n: u8) -> Result<u32, H264Error> {
        let mut v = 0u32;
        for _ in 0..n {
            v = (v << 1) | self.u1()?;
        }
        Ok(v)
    }

    /// 🧮 Exp-Golomb unsigned code (`ue(v)`, clause 9.1).
    fn ue(&mut self) -> Result<u32, H264Error> {
        let mut leading_zero_bits = 0u32;
        while self.u1()? == 0 {
            leading_zero_bits += 1;
            if leading_zero_bits > 31 {
                return Err(H264Error::Malformed("exp-golomb code exceeds 31 leading zero bits"));
            }
        }
        if leading_zero_bits == 0 {
            return Ok(0);
        }
        let suffix = self.u(leading_zero_bits as u8)?;
        Ok((1u32 << leading_zero_bits) - 1 + suffix)
    }

    /// 🧮 Exp-Golomb signed code (`se(v)`, clause 9.1.1): maps the unsigned code `k` to `(-1)^(k+1) * ceil(k/2)`.
    fn se(&mut self) -> Result<i32, H264Error> {
        let code = self.ue()? as i64;
        let magnitude = (code + 1) / 2;
        let value = if code % 2 == 0 { -magnitude } else { magnitude };
        i32::try_from(value).map_err(|_| H264Error::Malformed("se(v) value out of range"))
    }

    fn byte_align(&mut self) {
        if self.bit_pos != 0 {
            self.bit_pos = 0;
            self.byte_pos += 1;
        }
    }

    /// 🏁 `more_rbsp_data()` (clause 7.2): locates `rbsp_stop_one_bit` (the last `1` bit in the RBSP) and
    /// reports whether the cursor still sits strictly before it.
    fn more_rbsp_data(&self) -> bool {
        if self.byte_pos >= self.data.len() {
            return false;
        }
        let mut last_nonzero = self.data.len();
        while last_nonzero > 0 && self.data[last_nonzero - 1] == 0 {
            last_nonzero -= 1;
        }
        if last_nonzero == 0 {
            return false;
        }
        let last_byte = self.data[last_nonzero - 1];
        let stop_bit_index = (last_nonzero - 1) * 8 + (7 - last_byte.trailing_zeros() as usize);
        let current_bit_index = self.byte_pos * 8 + self.bit_pos as usize;
        current_bit_index < stop_bit_index
    }
}
// #endregion 🔖Bits

// #region 🔖Rbsp
/// 🧹 Removes `emulation_prevention_three_byte`s (`00 00 03` → `00 00`) from a NAL's EBSP, yielding its RBSP.
/// <https://www.itu.int/rec/T-REC-H.264> (clause 7.4.1.1)
fn strip_emulation_prevention(ebsp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(ebsp.len());
    let mut i = 0;
    while i < ebsp.len() {
        if i + 2 < ebsp.len() && ebsp[i] == 0 && ebsp[i + 1] == 0 && ebsp[i + 2] == 3 {
            out.push(0);
            out.push(0);
            i += 3;
        } else {
            out.push(ebsp[i]);
            i += 1;
        }
    }
    out
}

/// 🧩 One parsed NAL unit: its type plus emulation-prevention-stripped RBSP payload. `nal_ref_idc` is
/// intentionally not retained — this decoder's DPB (see [`Dpb`]) treats every decoded picture as a reference,
/// a scoped simplification since this crate's own encoder never emits non-reference pictures.
struct NalUnit {
    nal_unit_type: u8,
    rbsp: Vec<u8>,
}

/// 📥 Parses a single NAL's 1-byte header and strips emulation prevention from the rest.
fn parse_nal(nal_bytes: &[u8]) -> Result<NalUnit, H264Error> {
    let &header = nal_bytes.first().ok_or(H264Error::Truncated)?;
    if header & 0x80 != 0 {
        return Err(H264Error::Malformed("nal forbidden_zero_bit is set"));
    }
    let nal_unit_type = header & 0x1F;
    Ok(NalUnit { nal_unit_type, rbsp: strip_emulation_prevention(&nal_bytes[1..]) })
}

/// ✂️ Splits one AVCC length-prefixed access unit into its constituent NAL byte ranges (each still including
/// the 1-byte NAL header); `length_size` is `avcC`'s `lengthSizeMinusOne + 1` (1, 2 or 4 bytes).
fn split_avcc_nals(data: &[u8], length_size: u8) -> Result<Vec<&[u8]>, H264Error> {
    let n = length_size as usize;
    if !(1..=4).contains(&n) || n == 3 {
        return Err(H264Error::Unsupported("nal length size other than 1, 2 or 4 bytes"));
    }
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < data.len() {
        let len_bytes = data.get(pos..pos + n).ok_or(H264Error::Truncated)?;
        let len = len_bytes.iter().fold(0usize, |acc, &b| (acc << 8) | usize::from(b));
        pos += n;
        let nal = data.get(pos..pos + len).ok_or(H264Error::Truncated)?;
        out.push(nal);
        pos += len;
    }
    Ok(out)
}

/// ✂️ Splits an Annex-B byte stream (`00 00 01` / `00 00 00 01` start codes) into NAL byte ranges; provided
/// for parity with the spec's other framing convention, alongside [`split_avcc_nals`] which this crate's own
/// [`H264Decoder`]/[`Mux`](mod@self) plumbing actually uses.
#[allow(dead_code, reason = "public per the plan's 🔖Rbsp subregion spec even though this crate's own mux/decode path only exercises AVCC framing")]
fn split_annexb_nals(data: &[u8]) -> Vec<&[u8]> {
    let mut starts = Vec::new();
    let mut i = 0;
    while i + 2 < data.len() {
        if data[i] == 0 && data[i + 1] == 0 && data[i + 2] == 1 {
            starts.push(i + 3);
            i += 3;
        } else {
            i += 1;
        }
    }
    let mut out = Vec::new();
    for (idx, &start) in starts.iter().enumerate() {
        let mut end = starts.get(idx + 1).map_or(data.len(), |&next_start| {
            let mut e = next_start - 3;
            if e > start && data[e - 1] == 0 {
                e -= 1;
            }
            e
        });
        if end < start {
            end = start;
        }
        out.push(&data[start..end]);
    }
    out
}
// #endregion 🔖Rbsp

// #region 🔖Sps
/// 🎬 Parsed SPS fields this decoder needs. Only Baseline profile (`profile_idc == 66`), progressive frames
/// (`frame_mbs_only_flag == 1`) and `pic_order_cnt_type` 0 or 2 are accepted — everything else is a loud
/// [`H264Error::Unsupported`] at parse time rather than a best-effort (and possibly wrong) decode later.
#[derive(Clone, Debug)]
struct SpsInfo {
    log2_max_frame_num: u32,
    pic_order_cnt_type: u32,
    log2_max_pic_order_cnt_lsb: u32,
    max_num_ref_frames: u32,
    pic_width_in_mbs: u32,
    pic_height_in_mbs: u32,
    width_px: u32,
    height_px: u32,
}

/// 📥 Parses a Sequence Parameter Set RBSP. <https://www.itu.int/rec/T-REC-H.264> (clause 7.3.2.1.1)
fn parse_sps(rbsp: &[u8]) -> Result<SpsInfo, H264Error> {
    let mut b = BitReader::new(rbsp);
    let profile_idc = b.u(8)?;
    if profile_idc != 66 {
        return Err(H264Error::Unsupported("non-baseline sps profile_idc (only 66 is supported)"));
    }
    b.u(8)?;
    b.u(8)?;
    b.ue()?;
    let log2_max_frame_num = b.ue()? + 4;
    let pic_order_cnt_type = b.ue()?;
    let log2_max_pic_order_cnt_lsb = match pic_order_cnt_type {
        0 => b.ue()? + 4,
        1 => return Err(H264Error::Unsupported("pic_order_cnt_type 1")),
        2 => 0,
        _ => return Err(H264Error::Malformed("pic_order_cnt_type out of range")),
    };
    let max_num_ref_frames = b.ue()?;
    b.u1()?;
    let pic_width_in_mbs = b.ue()? + 1;
    let pic_height_in_map_units = b.ue()? + 1;
    let frame_mbs_only_flag = b.u1()?;
    if frame_mbs_only_flag == 0 {
        return Err(H264Error::Unsupported("interlaced sps (frame_mbs_only_flag == 0)"));
    }
    b.u1()?;
    let frame_cropping_flag = b.u1()?;
    let (mut crop_left, mut crop_right, mut crop_top, mut crop_bottom) = (0u32, 0u32, 0u32, 0u32);
    if frame_cropping_flag == 1 {
        crop_left = b.ue()?;
        crop_right = b.ue()?;
        crop_top = b.ue()?;
        crop_bottom = b.ue()?;
    }
    let pic_height_in_mbs = pic_height_in_map_units;
    let width_px = pic_width_in_mbs * 16 - 2 * (crop_left + crop_right);
    let height_px = pic_height_in_mbs * 16 - 2 * (crop_top + crop_bottom);
    if width_px == 0 || height_px == 0 {
        return Err(H264Error::Malformed("sps describes a zero-sized picture"));
    }
    Ok(SpsInfo { log2_max_frame_num, pic_order_cnt_type, log2_max_pic_order_cnt_lsb, max_num_ref_frames, pic_width_in_mbs, pic_height_in_mbs, width_px, height_px })
}
// #endregion 🔖Sps

// #region 🔖Pps
/// 🎬 Parsed PPS fields this decoder needs; entropy coding is always CAVLC and slices are always a single
/// slice group (constraints are enforced at parse time, see [`parse_pps`]). `constrained_intra_pred_flag` is
/// parsed (so the bitstream stays in sync) but not enforced during intra prediction — a scoped simplification
/// unexercised by this crate's own encoder, which never sets it.
#[derive(Clone, Debug)]
struct PpsInfo {
    num_ref_idx_l0_default_active: u32,
    weighted_pred_flag: bool,
    pic_init_qp: i32,
    chroma_qp_index_offset: i32,
    deblocking_filter_control_present_flag: bool,
}

/// 📥 Parses a Picture Parameter Set RBSP. <https://www.itu.int/rec/T-REC-H.264> (clause 7.3.2.2)
fn parse_pps(rbsp: &[u8]) -> Result<PpsInfo, H264Error> {
    let mut b = BitReader::new(rbsp);
    b.ue()?;
    b.ue()?;
    let entropy_coding_mode_flag = b.u1()?;
    if entropy_coding_mode_flag != 0 {
        return Err(H264Error::Unsupported("cabac entropy coding (entropy_coding_mode_flag == 1)"));
    }
    b.u1()?;
    let num_slice_groups_minus1 = b.ue()?;
    if num_slice_groups_minus1 != 0 {
        return Err(H264Error::Unsupported("multiple slice groups (fmo)"));
    }
    let num_ref_idx_l0_default_active = b.ue()? + 1;
    b.ue()?;
    let weighted_pred_flag = b.u1()? == 1;
    b.u(2)?;
    let pic_init_qp = b.se()? + 26;
    b.se()?;
    let chroma_qp_index_offset = b.se()?;
    let deblocking_filter_control_present_flag = b.u1()? == 1;
    b.u1()?;
    b.u1()?;
    if b.more_rbsp_data() {
        let transform_8x8_mode_flag = b.u1()?;
        if transform_8x8_mode_flag == 1 {
            return Err(H264Error::Unsupported("8x8 transform (transform_8x8_mode_flag == 1)"));
        }
    }
    Ok(PpsInfo { num_ref_idx_l0_default_active, weighted_pred_flag, pic_init_qp, chroma_qp_index_offset, deblocking_filter_control_present_flag })
}
// #endregion 🔖Pps

// #region 🔖SliceHeader
/// 🎬 Parsed slice header fields this decoder needs. `slice_type_mod5 == 0` is P, `== 2` is I (B/SP/SI are
/// rejected during parsing, see [`parse_slice_header`]).
#[derive(Clone, Debug)]
struct SliceHeaderInfo {
    first_mb_in_slice: u32,
    slice_type_mod5: u32,
    frame_num: u32,
    pic_order_cnt_lsb: u32,
    num_ref_idx_l0_active: u32,
    slice_qp: i32,
    disable_deblocking_filter_idc: u32,
    slice_alpha_c0_offset: i32,
    slice_beta_offset: i32,
}

/// 📥 Parses a slice header RBSP prefix (up to but not including `slice_data()`), advancing `b` so the
/// caller can continue reading macroblock data from the same cursor. <https://www.itu.int/rec/T-REC-H.264>
/// (clause 7.3.3)
fn parse_slice_header(b: &mut BitReader<'_>, nal_unit_type: u8, sps: &SpsInfo, pps: &PpsInfo) -> Result<SliceHeaderInfo, H264Error> {
    let first_mb_in_slice = b.ue()?;
    let slice_type = b.ue()?;
    let slice_type_mod5 = slice_type % 5;
    if slice_type_mod5 != 0 && slice_type_mod5 != 2 {
        return Err(H264Error::Unsupported("slice type other than P or I (b/sp/si)"));
    }
    b.ue()?;
    let frame_num = b.u(sps.log2_max_frame_num as u8)?;
    let is_idr = nal_unit_type == 5;
    if is_idr {
        b.ue()?;
    }
    let mut pic_order_cnt_lsb = 0u32;
    if sps.pic_order_cnt_type == 0 {
        pic_order_cnt_lsb = b.u(sps.log2_max_pic_order_cnt_lsb as u8)?;
    }
    let mut num_ref_idx_l0_active = pps.num_ref_idx_l0_default_active;
    if slice_type_mod5 == 0 {
        let num_ref_idx_active_override_flag = b.u1()?;
        if num_ref_idx_active_override_flag == 1 {
            num_ref_idx_l0_active = b.ue()? + 1;
        }
        let ref_pic_list_modification_flag_l0 = b.u1()?;
        if ref_pic_list_modification_flag_l0 == 1 {
            return Err(H264Error::Unsupported("reference picture list reordering"));
        }
        if pps.weighted_pred_flag {
            return Err(H264Error::Unsupported("weighted prediction"));
        }
    }
    if is_idr {
        b.u1()?;
        b.u1()?;
    } else {
        let adaptive_ref_pic_marking_mode_flag = b.u1()?;
        if adaptive_ref_pic_marking_mode_flag == 1 {
            return Err(H264Error::Unsupported("adaptive reference picture marking (mmco)"));
        }
    }
    let slice_qp = pps.pic_init_qp + b.se()?;
    let mut disable_deblocking_filter_idc = 0u32;
    let mut slice_alpha_c0_offset = 0i32;
    let mut slice_beta_offset = 0i32;
    if pps.deblocking_filter_control_present_flag {
        disable_deblocking_filter_idc = b.ue()?;
        if disable_deblocking_filter_idc != 1 {
            slice_alpha_c0_offset = b.se()? * 2;
            slice_beta_offset = b.se()? * 2;
        }
    }
    Ok(SliceHeaderInfo { first_mb_in_slice, slice_type_mod5, frame_num, pic_order_cnt_lsb, num_ref_idx_l0_active, slice_qp, disable_deblocking_filter_idc, slice_alpha_c0_offset, slice_beta_offset })
}
// #endregion 🔖SliceHeader

// #region 🔖Cavlc
/// 🔀 H.264's 4×4 zig-zag scan: `ZIGZAG_4X4[k]` is the raster (`row*4+col`) position of the coefficient at
/// scan index `k`. <https://www.itu.int/rec/T-REC-H.264> (clause 8.5.6, Figure 8-8)
const ZIGZAG_4X4: [usize; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

/// 📖 One VLC's `(length, bits)` table pair, indexed by `4*totalCoeff + trailingOnes` (clause 9.2.1, Table 9-5).
struct CoeffTokenTable {
    len: &'static [u8],
    bits: &'static [u8],
}

/// 📖 `coeff_token` VLC tables selected by `nC` bucket, transcribed from ITU-T H.264 Table 9-5.
const COEFF_TOKEN_TABLES: [CoeffTokenTable; 4] = [
    CoeffTokenTable {
        len: &[1, 0, 0, 0, 6, 2, 0, 0, 8, 6, 3, 0, 9, 8, 7, 5, 10, 9, 8, 6, 11, 10, 9, 7, 13, 11, 10, 8, 13, 13, 11, 9, 13, 13, 13, 10, 14, 14, 13, 11, 14, 14, 14, 13, 15, 15, 14, 14, 15, 15, 15, 14, 16, 15, 15, 15, 16, 16, 16, 15, 16, 16, 16, 16, 16, 16, 16, 16],
        bits: &[1, 0, 0, 0, 5, 1, 0, 0, 7, 4, 1, 0, 7, 6, 5, 3, 7, 6, 5, 3, 7, 6, 5, 4, 15, 6, 5, 4, 11, 14, 5, 4, 8, 10, 13, 4, 15, 14, 9, 4, 11, 10, 13, 12, 15, 14, 9, 12, 11, 10, 13, 8, 15, 1, 9, 12, 11, 14, 13, 8, 7, 10, 9, 12, 4, 6, 5, 8],
    },
    CoeffTokenTable {
        len: &[2, 0, 0, 0, 6, 2, 0, 0, 6, 5, 3, 0, 7, 6, 6, 4, 8, 6, 6, 4, 8, 7, 7, 5, 9, 8, 8, 6, 11, 9, 9, 6, 11, 11, 11, 7, 12, 11, 11, 9, 12, 12, 12, 11, 12, 12, 12, 11, 13, 13, 13, 12, 13, 13, 13, 13, 13, 14, 13, 13, 14, 14, 14, 13, 14, 14, 14, 14],
        bits: &[3, 0, 0, 0, 11, 2, 0, 0, 7, 7, 3, 0, 7, 10, 9, 5, 7, 6, 5, 4, 4, 6, 5, 6, 7, 6, 5, 8, 15, 6, 5, 4, 11, 14, 13, 4, 15, 10, 9, 4, 11, 14, 13, 12, 8, 10, 9, 8, 15, 14, 13, 12, 11, 10, 9, 12, 7, 11, 6, 8, 9, 8, 10, 1, 7, 6, 5, 4],
    },
    CoeffTokenTable {
        len: &[4, 0, 0, 0, 6, 4, 0, 0, 6, 5, 4, 0, 6, 5, 5, 4, 7, 5, 5, 4, 7, 5, 5, 4, 7, 6, 6, 4, 7, 6, 6, 4, 8, 7, 7, 5, 8, 8, 7, 6, 9, 8, 8, 7, 9, 9, 8, 8, 9, 9, 9, 8, 10, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        bits: &[15, 0, 0, 0, 15, 14, 0, 0, 11, 15, 13, 0, 8, 12, 14, 12, 15, 10, 11, 11, 11, 8, 9, 10, 9, 14, 13, 9, 8, 10, 9, 8, 15, 14, 13, 13, 11, 14, 10, 12, 15, 10, 13, 12, 11, 14, 9, 12, 8, 10, 13, 8, 13, 7, 9, 12, 9, 12, 11, 10, 5, 8, 7, 6, 1, 4, 3, 2],
    },
    CoeffTokenTable {
        len: &[6, 0, 0, 0, 6, 6, 0, 0, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6],
        bits: &[3, 0, 0, 0, 0, 1, 0, 0, 4, 5, 6, 0, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63],
    },
];

/// 📖 `coeff_token` VLC for chroma DC (`nC == -1`, 4:2:0's 2×2 block), Table 9-5's rightmost-but-one column.
const CHROMA_DC_COEFF_TOKEN: CoeffTokenTable = CoeffTokenTable {
    len: &[2, 0, 0, 0, 6, 1, 0, 0, 6, 6, 3, 0, 6, 7, 7, 6, 6, 8, 8, 7],
    bits: &[1, 0, 0, 0, 7, 1, 0, 0, 4, 6, 1, 0, 3, 3, 2, 5, 2, 3, 2, 0],
};

/// 🔎 Reads one `coeff_token` from `table`, returning `(total_coeff, trailing_ones)`.
fn read_coeff_token(b: &mut BitReader<'_>, table: &CoeffTokenTable) -> Result<(u32, u32), H264Error> {
    let idx = read_vlc(b, table.len, table.bits)?;
    Ok((idx / 4, idx % 4))
}

/// 🔎 Reads `coeff_token` for `nC >= 8`: a fixed 6-bit code, `((TotalCoeff-1)<<2)|TrailingOnes`, with `000011`
/// as the sole exception encoding `TotalCoeff == 0`.
fn read_coeff_token_fixed(b: &mut BitReader<'_>) -> Result<(u32, u32), H264Error> {
    let v = b.u(6)?;
    if v == 3 {
        return Ok((0, 0));
    }
    Ok((v / 4 + 1, v % 4))
}

/// 🧮 Predicted `nC` (clause 9.2.1): average of the left/above 4×4 neighbor blocks' total_coeff, rounded up,
/// or just one of them when the other is unavailable, or `0` when neither is available.
fn predict_nc(left: Option<u8>, above: Option<u8>) -> u32 {
    match (left, above) {
        (Some(l), Some(a)) => (u32::from(l) + u32::from(a)).div_ceil(2),
        (Some(l), None) => u32::from(l),
        (None, Some(a)) => u32::from(a),
        (None, None) => 0,
    }
}

const TOTAL_ZEROS_LEN: [&[u8]; 15] = [
    &[1, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 9],
    &[3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6],
    &[4, 3, 3, 3, 4, 4, 3, 3, 4, 5, 5, 6, 5, 6],
    &[5, 3, 4, 4, 3, 3, 3, 4, 3, 4, 5, 5, 5],
    &[4, 4, 4, 3, 3, 3, 3, 3, 4, 5, 4, 5],
    &[6, 5, 3, 3, 3, 3, 3, 3, 4, 3, 6],
    &[6, 5, 3, 3, 3, 2, 3, 4, 3, 6],
    &[6, 4, 5, 3, 2, 2, 3, 3, 6],
    &[6, 6, 4, 2, 2, 3, 2, 5],
    &[5, 5, 3, 2, 2, 2, 4],
    &[4, 4, 3, 3, 1, 3],
    &[4, 4, 2, 1, 3],
    &[3, 3, 1, 2],
    &[2, 2, 1],
    &[1, 1],
];

const TOTAL_ZEROS_BITS: [&[u8]; 15] = [
    &[1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 1],
    &[7, 6, 5, 4, 3, 5, 4, 3, 2, 3, 2, 3, 2, 1, 0],
    &[5, 7, 6, 5, 4, 3, 4, 3, 2, 3, 2, 1, 1, 0],
    &[3, 7, 5, 4, 6, 5, 4, 3, 3, 2, 2, 1, 0],
    &[5, 4, 3, 7, 6, 5, 4, 3, 2, 1, 1, 0],
    &[1, 1, 7, 6, 5, 4, 3, 2, 1, 1, 0],
    &[1, 1, 5, 4, 3, 3, 2, 1, 1, 0],
    &[1, 1, 1, 3, 3, 2, 2, 1, 0],
    &[1, 0, 1, 3, 2, 1, 1, 1],
    &[1, 0, 1, 3, 2, 1, 1],
    &[0, 1, 1, 2, 1, 3],
    &[0, 1, 1, 1, 1],
    &[0, 1, 1, 1],
    &[0, 1, 1],
    &[0, 1],
];

const CHROMA_DC_TOTAL_ZEROS_LEN: [[u8; 3]; 3] = [[1, 2, 3], [1, 2, 2], [1, 1, 0]];
const CHROMA_DC_TOTAL_ZEROS_BITS: [[u8; 3]; 3] = [[1, 1, 1], [1, 1, 0], [1, 0, 0]];

const RUN_BEFORE_LEN: [&[u8]; 7] = [&[1, 1], &[1, 2, 2], &[2, 2, 2, 2], &[2, 2, 2, 3, 3], &[2, 2, 3, 3, 3, 3], &[2, 3, 3, 3, 3, 3, 3], &[3, 3, 3, 3, 3, 3, 3, 4, 5, 6, 7, 8, 9, 10, 11]];
const RUN_BEFORE_BITS: [&[u8]; 7] = [&[1, 0], &[1, 1, 0], &[3, 2, 1, 0], &[3, 2, 1, 1, 0], &[3, 2, 3, 2, 1, 0], &[3, 0, 1, 3, 2, 5, 4], &[7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1]];

/// 🔎 Generic canonical-VLC reader matching bits MSB-first against parallel `(len, bits)` slices, returning
/// the matching slice index (the semantic value depends on the table: `total_zeros`, `run_before`, ...).
fn read_vlc(b: &mut BitReader<'_>, len: &[u8], bits: &[u8]) -> Result<u32, H264Error> {
    let mut code = 0u32;
    for l in 1..=16u32 {
        code = (code << 1) | b.u1()?;
        for (idx, (&entry_len, &entry_bits)) in len.iter().zip(bits.iter()).enumerate() {
            if u32::from(entry_len) == l && u32::from(entry_bits) == code {
                return Ok(idx as u32);
            }
        }
    }
    Err(H264Error::Malformed("invalid vlc code"))
}

/// 🔎 `level_prefix` (clause 9.2.2.1): a unary prefix, counted as leading `0` bits before the terminating `1`.
fn read_level_prefix(b: &mut BitReader<'_>) -> Result<u32, H264Error> {
    let mut count = 0u32;
    while b.u1()? == 0 {
        count += 1;
        if count > 63 {
            return Err(H264Error::Malformed("level_prefix too long"));
        }
    }
    Ok(count)
}

/// 📥 One decoded residual block: raster-order (already un-zig-zagged) coefficients plus the block's
/// `total_coeff` (needed by neighboring blocks' `nC` prediction).
struct ResidualBlock {
    coeffs: [i32; 16],
    total_coeff: u8,
}

/// 📥 Decodes one CAVLC residual block (clause 9.2, 7.3.5.3.1 `residual_block_cavlc`) and un-zig-zags it in
/// the same pass. `scan` is the raster position for each scan index: [`ZIGZAG_4X4`] (16 entries) for luma
/// 4×4/`Intra16x16DCLevel`, `&ZIGZAG_4X4[1..]` (15 entries, DC excluded) for `Intra16x16ACLevel`/chroma AC, or
/// `&[0, 1, 2, 3]` for chroma DC (2×2, `nC == -1`, no real zig-zag).
fn read_residual_block(b: &mut BitReader<'_>, nc_selector: NcSelector, scan: &[usize]) -> Result<ResidualBlock, H264Error> {
    let max_coeff = scan.len() as u8;
    let (total_coeff, trailing_ones) = match nc_selector {
        NcSelector::ChromaDc => read_coeff_token(b, &CHROMA_DC_COEFF_TOKEN)?,
        NcSelector::Nc(nc) if nc >= 8 => read_coeff_token_fixed(b)?,
        NcSelector::Nc(nc) => {
            let bucket = if nc < 2 { 0 } else if nc < 4 { 1 } else { 2 };
            read_coeff_token(b, &COEFF_TOKEN_TABLES[bucket])?
        }
    };
    let mut out = ResidualBlock { coeffs: [0; 16], total_coeff: total_coeff as u8 };
    if total_coeff == 0 {
        return Ok(out);
    }
    if total_coeff > u32::from(max_coeff) {
        return Err(H264Error::Malformed("total_coeff exceeds block size"));
    }

    let mut level = [0i32; 16];
    for slot in level.iter_mut().take(trailing_ones as usize) {
        *slot = if b.u1()? == 0 { 1 } else { -1 };
    }

    let mut suffix_length: u32 = if total_coeff > 10 && trailing_ones < 3 { 1 } else { 0 };
    #[allow(clippy::needless_range_loop, reason = "`i` is compared against `trailing_ones` (clause 9.2.2.1's \"first coefficient after the trailing ones\" special case) in addition to indexing `level`, so this isn't just a plain element walk")]
    for i in (trailing_ones as usize)..(total_coeff as usize) {
        let level_prefix = read_level_prefix(b)?;
        let level_suffix_size = if level_prefix == 14 && suffix_length == 0 {
            4
        } else if level_prefix >= 15 {
            level_prefix - 3
        } else {
            suffix_length
        };
        let mut level_code = level_prefix.min(15) << suffix_length;
        if level_suffix_size > 0 {
            level_code += b.u(level_suffix_size as u8)?;
        }
        if level_prefix >= 15 && suffix_length == 0 {
            level_code += 15;
        }
        if level_prefix >= 16 {
            level_code += (1u32 << (level_prefix - 3)).wrapping_sub(4096);
        }
        if i == trailing_ones as usize && trailing_ones < 3 {
            level_code += 2;
        }
        let signed = if level_code.is_multiple_of(2) { (level_code as i64 + 2) / 2 } else { -((level_code as i64 + 1) / 2) };
        level[i] = signed as i32;
        if suffix_length == 0 {
            suffix_length = 1;
        }
        if level[i].unsigned_abs() > (3u32 << (suffix_length - 1)) && suffix_length < 6 {
            suffix_length += 1;
        }
    }

    let zeros_left = if total_coeff == u32::from(max_coeff) {
        0
    } else if matches!(nc_selector, NcSelector::ChromaDc) {
        read_vlc(b, &CHROMA_DC_TOTAL_ZEROS_LEN[total_coeff as usize - 1], &CHROMA_DC_TOTAL_ZEROS_BITS[total_coeff as usize - 1])?
    } else {
        read_vlc(b, TOTAL_ZEROS_LEN[total_coeff as usize - 1], TOTAL_ZEROS_BITS[total_coeff as usize - 1])?
    };

    let mut run_before = vec![0u32; total_coeff as usize];
    let mut zeros_remaining = zeros_left;
    for slot in run_before.iter_mut().take(total_coeff as usize - 1) {
        if zeros_remaining == 0 {
            break;
        }
        let r = if zeros_remaining < 7 {
            read_vlc(b, RUN_BEFORE_LEN[zeros_remaining as usize - 1], RUN_BEFORE_BITS[zeros_remaining as usize - 1])?
        } else {
            read_vlc(b, RUN_BEFORE_LEN[6], RUN_BEFORE_BITS[6])?
        };
        *slot = r;
        zeros_remaining -= r;
    }

    let mut scan_pos = (zeros_left + total_coeff - 1) as i32;
    for i in 0..total_coeff as usize {
        if scan_pos < 0 || scan_pos as usize >= max_coeff as usize {
            return Err(H264Error::Malformed("residual block scan position out of range"));
        }
        out.coeffs[scan[scan_pos as usize]] = level[i];
        if i + 1 < total_coeff as usize {
            scan_pos -= 1 + run_before[i] as i32;
        }
    }
    Ok(out)
}

/// 🔀 Selects which `coeff_token` VLC variant a residual block uses: the ordinary `nC`-bucketed luma/chroma-AC
/// path, or the dedicated `nC == -1` chroma-DC path.
#[derive(Clone, Copy)]
enum NcSelector {
    Nc(u32),
    ChromaDc,
}
// #endregion 🔖Cavlc

// #region 🔖Transform
/// 🧮 `normAdjust4x4` base values `{v0, v1, v2}` per `m = qP % 6` (clause 8.5.9, Table not numbered — the
/// three distinct scale factors placed at `(even,even)`, `(odd,odd)`, and mixed-parity 4×4 positions).
const LEVEL_SCALE_BASE: [[i32; 3]; 6] = [[10, 13, 16], [11, 14, 18], [13, 16, 20], [14, 18, 23], [16, 20, 25], [18, 23, 29]];

/// 🧮 `normAdjust4x4(m, i, j)` (clause 8.5.9).
fn norm_adjust(m: usize, i: usize, j: usize) -> i32 {
    if i.is_multiple_of(2) && j.is_multiple_of(2) {
        LEVEL_SCALE_BASE[m][0]
    } else if i % 2 == 1 && j % 2 == 1 {
        LEVEL_SCALE_BASE[m][1]
    } else {
        LEVEL_SCALE_BASE[m][2]
    }
}

/// 🧮 Scales quantized 4×4 residual coefficients (raster order) back to the transform domain (clause 8.5.12.1).
fn dequant4x4(coeffs: &[i32; 16], qp: i32) -> [i32; 16] {
    let m = qp.rem_euclid(6) as usize;
    let shift = qp.div_euclid(6);
    let mut out = [0i32; 16];
    for i in 0..4 {
        for j in 0..4 {
            let pos = i * 4 + j;
            let scale = norm_adjust(m, i, j);
            out[pos] = if shift >= 4 { (coeffs[pos] * scale) << (shift - 4) } else { (coeffs[pos] * scale + (1 << (3 - shift))) >> (4 - shift) };
        }
    }
    out
}

/// 🌊 H.264's integer core transform butterfly, applied separably (clause 8.5.12.2); its own inverse up to
/// the final `(x + 32) >> 6` rounding/normalization.
fn idct4x4_1d(input: [i32; 4]) -> [i32; 4] {
    let e0 = input[0] + input[2];
    let e1 = input[0] - input[2];
    let e2 = (input[1] >> 1) - input[3];
    let e3 = input[1] + (input[3] >> 1);
    [e0 + e3, e1 + e2, e1 - e2, e0 - e3]
}

/// 🌊 Full separable 4×4 inverse core transform: dequantized coefficients (raster order) → spatial residual.
fn idct4x4(d: &[i32; 16]) -> [i32; 16] {
    let mut cols = [0i32; 16];
    for c in 0..4 {
        let t = idct4x4_1d([d[c], d[4 + c], d[8 + c], d[12 + c]]);
        for (r, &v) in t.iter().enumerate() {
            cols[r * 4 + c] = v;
        }
    }
    let mut out = [0i32; 16];
    for r in 0..4 {
        let t = idct4x4_1d([cols[r * 4], cols[r * 4 + 1], cols[r * 4 + 2], cols[r * 4 + 3]]);
        for (c, &v) in t.iter().enumerate() {
            out[r * 4 + c] = (v + 32) >> 6;
        }
    }
    out
}

/// 🌊 4×4 Hadamard butterfly (symmetric, its own inverse up to scale), shared by the luma-DC transform.
fn hadamard4_1d(input: [i32; 4]) -> [i32; 4] {
    let e0 = input[0] + input[2];
    let e1 = input[0] - input[2];
    let e2 = input[1] - input[3];
    let e3 = input[1] + input[3];
    [e0 + e3, e1 + e2, e1 - e2, e0 - e3]
}

/// 🌊 Inverse-transforms and dequantizes the 16 `Intra16x16DCLevel` coefficients (raster block order, i.e.
/// `dc[blockRow*4+blockCol]`) into per-block DC values to splice into position 0 of each luma 4×4 AC block
/// (clause 8.5.10).
fn transform_luma16x16_dc(dc: &[i32; 16], qp: i32) -> [i32; 16] {
    let mut cols = [0i32; 16];
    for c in 0..4 {
        let t = hadamard4_1d([dc[c], dc[4 + c], dc[8 + c], dc[12 + c]]);
        for (r, &v) in t.iter().enumerate() {
            cols[r * 4 + c] = v;
        }
    }
    let mut f = [0i32; 16];
    for r in 0..4 {
        let t = hadamard4_1d([cols[r * 4], cols[r * 4 + 1], cols[r * 4 + 2], cols[r * 4 + 3]]);
        for (c, &v) in t.iter().enumerate() {
            f[r * 4 + c] = v;
        }
    }
    let m = qp.rem_euclid(6) as usize;
    let shift = qp.div_euclid(6);
    let scale = LEVEL_SCALE_BASE[m][0];
    let mut out = [0i32; 16];
    for (o, &fv) in out.iter_mut().zip(f.iter()) {
        *o = if shift >= 6 { (fv * scale) << (shift - 6) } else { (fv * scale + (1 << (5 - shift))) >> (6 - shift) };
    }
    out
}

/// 🌊 Inverse-transforms and dequantizes the 4 `ChromaDCLevel` coefficients (`[cb00,cb01,cb10,cb11]` raster)
/// into per-4×4-block DC values (clause 8.5.11).
fn transform_chroma_dc(dc: &[i32; 4], qp: i32) -> [i32; 4] {
    let e00 = dc[0] + dc[1] + dc[2] + dc[3];
    let e01 = dc[0] - dc[1] + dc[2] - dc[3];
    let e10 = dc[0] + dc[1] - dc[2] - dc[3];
    let e11 = dc[0] - dc[1] - dc[2] + dc[3];
    let m = qp.rem_euclid(6) as usize;
    let shift = qp.div_euclid(6);
    let scale = LEVEL_SCALE_BASE[m][0];
    [e00, e01, e10, e11].map(|f| ((f * scale) << shift) >> 5)
}
// #endregion 🔖Transform

// #region 🔖Picture
/// 🖼️ One in-progress or fully reconstructed picture's working state: 4:2:0 sample planes plus the per-4×4
/// (luma) / per-4×4-chroma-quadrant side information (`is_intra`, motion, non-zero counts) later stages
/// (deblocking, next-frame prediction) need. Sample planes are macroblock-padded (`mb_width*16 × mb_height*16`
/// luma, half that for chroma) — [`Picture::crop_to`] trims to `width × height` once decode finishes.
struct Picture {
    mb_width: u32,
    mb_height: u32,
    luma: Vec<u8>,
    cb: Vec<u8>,
    cr: Vec<u8>,
    decoded_luma4: Vec<bool>,
    mb_is_intra: Vec<bool>,
    mb_qp: Vec<i32>,
    mv: Vec<[i32; 2]>,
    ref_idx: Vec<i8>,
    nnz_luma: Vec<u8>,
    nnz_cb: Vec<u8>,
    nnz_cr: Vec<u8>,
    intra4x4_mode: Vec<i8>,
    frame_num: u32,
    poc: i32,
}

impl Picture {
    fn new(mb_width: u32, mb_height: u32) -> Self {
        let lw = (mb_width * 16) as usize;
        let lh = (mb_height * 16) as usize;
        let cw = (mb_width * 8) as usize;
        let ch = (mb_height * 8) as usize;
        let luma4_w = (mb_width * 4) as usize;
        let luma4_h = (mb_height * 4) as usize;
        let mb_count = (mb_width * mb_height) as usize;
        Self {
            mb_width,
            mb_height,
            luma: vec![0; lw * lh],
            cb: vec![128; cw * ch],
            cr: vec![128; cw * ch],
            decoded_luma4: vec![false; luma4_w * luma4_h],
            mb_is_intra: vec![false; mb_count],
            mb_qp: vec![0; mb_count],
            mv: vec![[0, 0]; luma4_w * luma4_h],
            ref_idx: vec![-1; luma4_w * luma4_h],
            nnz_luma: vec![0; luma4_w * luma4_h],
            nnz_cb: vec![0; luma4_w * luma4_h / 4],
            nnz_cr: vec![0; luma4_w * luma4_h / 4],
            intra4x4_mode: vec![-1; luma4_w * luma4_h],
            frame_num: 0,
            poc: 0,
        }
    }

    fn luma_width(&self) -> usize {
        (self.mb_width * 16) as usize
    }

    fn luma4_width(&self) -> usize {
        (self.mb_width * 4) as usize
    }

    fn chroma_width(&self) -> usize {
        (self.mb_width * 8) as usize
    }

    fn chroma4_width(&self) -> usize {
        (self.mb_width * 2) as usize
    }

    /// 🔍 Reconstructed luma sample at `(x, y)`, or `None` outside the (macroblock-padded) picture.
    fn luma_at(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x as usize >= self.luma_width() || y as usize >= (self.mb_height * 16) as usize {
            return None;
        }
        Some(self.luma[y as usize * self.luma_width() + x as usize])
    }

    fn luma4_available(&self, bx: i32, by: i32) -> bool {
        if bx < 0 || by < 0 || bx as usize >= self.luma4_width() || by as usize >= (self.mb_height * 4) as usize {
            return false;
        }
        self.decoded_luma4[by as usize * self.luma4_width() + bx as usize]
    }

    fn mb_index(&self, mb_x: u32, mb_y: u32) -> usize {
        (mb_y * self.mb_width + mb_x) as usize
    }

    /// 🎨 Writes one reconstructed 4×4 luma block (clamped to `[0,255]`) at 4×4-block grid position `(bx, by)`
    /// and marks it decoded.
    fn write_luma4(&mut self, bx: u32, by: u32, block: &[i32; 16]) {
        let w = self.luma_width();
        let (px, py) = (bx as usize * 4, by as usize * 4);
        for r in 0..4 {
            for c in 0..4 {
                self.luma[(py + r) * w + px + c] = block[r * 4 + c].clamp(0, 255) as u8;
            }
        }
        let idx = by as usize * self.luma4_width() + bx as usize;
        self.decoded_luma4[idx] = true;
    }

    /// 🎨 Writes one reconstructed 4×4 chroma block into `plane` (`cb`/`cr`) at chroma-pixel origin `(px, py)`.
    fn write_chroma4(plane: &mut [u8], stride: usize, px: usize, py: usize, block: &[i32; 16]) {
        for r in 0..4 {
            for c in 0..4 {
                plane[(py + r) * stride + px + c] = block[r * 4 + c].clamp(0, 255) as u8;
            }
        }
    }

    /// ✂️ Crops the macroblock-padded planes down to the sequence's coded `width`×`height`, producing the
    /// final displayable [`remodel_image::ImageRgba8`] via [`ycbcr420_to_rgba`].
    fn crop_to(&self, width: u32, height: u32) -> remodel_image::ImageRgba8 {
        ycbcr420_to_rgba(&self.luma, self.luma_width(), &self.cb, &self.cr, self.chroma_width(), width, height)
    }
}
// #endregion 🔖Picture

// #region 🔖Intra
/// 🎨 One-sided (or corner) neighbor availability + samples for 4×4 luma intra prediction: `top`/`left` hold 4
/// samples each, `top_right` holds the 4 samples beyond the top row's right edge (replicated from the
/// rightmost available top sample when genuinely unavailable, per clause 8.3.1.2.1), `corner` is the top-left
/// sample.
struct Intra4Neighbors {
    top: Option<[i32; 4]>,
    left: Option<[i32; 4]>,
    top_right: [i32; 4],
    corner: i32,
}

/// 🎨 Predicts one 4×4 luma block for `mode` (0..8, clause 8.3.1.2); callers add the residual afterward.
fn predict_intra4x4(mode: u8, n: &Intra4Neighbors) -> Result<[i32; 16], H264Error> {
    let t = n.top.unwrap_or([128; 4]);
    let l = n.left.unwrap_or([128; 4]);
    let tl = n.corner;
    let tr = n.top_right;
    let tt = |i: i32| if i < 0 { tl } else { t[i as usize] };
    let ll = |i: i32| if i < 0 { tl } else { l[i as usize] };
    let tfull = |i: i32| if i < 4 { tt(i) } else { tr[(i - 4) as usize] };
    let mut out = [0i32; 16];
    let mut px = |x: usize, y: usize, v: i32| out[y * 4 + x] = v;
    match mode {
        0 => {
            if n.top.is_none() {
                return Err(H264Error::Malformed("intra4x4 vertical mode needs top neighbor"));
            }
            for y in 0..4 {
                for (x, &tv) in t.iter().enumerate() {
                    px(x, y, tv);
                }
            }
        }
        1 => {
            if n.left.is_none() {
                return Err(H264Error::Malformed("intra4x4 horizontal mode needs left neighbor"));
            }
            for (y, &lv) in l.iter().enumerate() {
                for x in 0..4 {
                    px(x, y, lv);
                }
            }
        }
        2 => {
            let dc = match (n.top, n.left) {
                (Some(t), Some(l)) => (t.iter().sum::<i32>() + l.iter().sum::<i32>() + 4) >> 3,
                (Some(t), None) => (t.iter().sum::<i32>() + 2) >> 2,
                (None, Some(l)) => (l.iter().sum::<i32>() + 2) >> 2,
                (None, None) => 128,
            };
            out = [dc; 16];
        }
        3 => {
            for y in 0..4usize {
                for x in 0..4usize {
                    let v = if x == 3 && y == 3 { (tfull(6) + 3 * tfull(7) + 2) >> 2 } else { (tfull((x + y) as i32) + 2 * tfull((x + y + 1) as i32) + tfull((x + y + 2) as i32) + 2) >> 2 };
                    px(x, y, v);
                }
            }
        }
        4 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let v = match x.cmp(&y) {
                        std::cmp::Ordering::Greater => (tt(x - y - 2) + 2 * tt(x - y - 1) + tt(x - y) + 2) >> 2,
                        std::cmp::Ordering::Less => (ll(y - x - 2) + 2 * ll(y - x - 1) + ll(y - x) + 2) >> 2,
                        std::cmp::Ordering::Equal => (t[0] + 2 * tl + l[0] + 2) >> 2,
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        5 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let z_vr = 2 * x - y;
                    let v = if z_vr >= 0 && z_vr % 2 == 0 {
                        (tt(x - (y >> 1) - 1) + tt(x - (y >> 1)) + 1) >> 1
                    } else if z_vr >= 1 && z_vr % 2 == 1 {
                        (tt(x - (y >> 1) - 2) + 2 * tt(x - (y >> 1) - 1) + tt(x - (y >> 1)) + 2) >> 2
                    } else if z_vr == -1 {
                        (l[0] + 2 * tl + t[0] + 2) >> 2
                    } else {
                        (ll(y - 1) + 2 * ll(y - 2) + ll(y - 3) + 2) >> 2
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        6 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let z_hd = 2 * y - x;
                    let v = if z_hd >= 0 && z_hd % 2 == 0 {
                        (ll(y - (x >> 1) - 1) + ll(y - (x >> 1)) + 1) >> 1
                    } else if z_hd >= 1 && z_hd % 2 == 1 {
                        (ll(y - (x >> 1) - 2) + 2 * ll(y - (x >> 1) - 1) + ll(y - (x >> 1)) + 2) >> 2
                    } else if z_hd == -1 {
                        (l[0] + 2 * tl + t[0] + 2) >> 2
                    } else {
                        (tt(x - 1) + 2 * tt(x - 2) + tt(x - 3) + 2) >> 2
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        7 => {
            for y in 0..4usize {
                for x in 0..4usize {
                    let v = if y % 2 == 0 { (tfull((x + y / 2) as i32) + tfull((x + y / 2 + 1) as i32) + 1) >> 1 } else { (tfull((x + y / 2) as i32) + 2 * tfull((x + y / 2 + 1) as i32) + tfull((x + y / 2 + 2) as i32) + 2) >> 2 };
                    px(x, y, v);
                }
            }
        }
        8 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let z_hu = x + 2 * y;
                    let v = if (0..=4).contains(&z_hu) && z_hu % 2 == 0 {
                        (ll(y + (x >> 1)) + ll(y + (x >> 1) + 1) + 1) >> 1
                    } else if z_hu <= 3 && z_hu % 2 == 1 {
                        (ll(y + (x >> 1)) + 2 * ll(y + (x >> 1) + 1) + ll(y + (x >> 1) + 2) + 2) >> 2
                    } else if z_hu == 5 {
                        (l[2] + 3 * l[3] + 2) >> 2
                    } else {
                        l[3]
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        _ => return Err(H264Error::Malformed("intra4x4 pred mode out of range")),
    }
    Ok(out)
}

/// 🎨 Neighbor-average DC prediction for an `n×n` block (Intra16x16 luma `n=16`, chroma DC quadrant `n=4`).
fn dc_pred(top: Option<&[i32]>, left: Option<&[i32]>, n: usize) -> i32 {
    match (top, left) {
        (Some(t), Some(l)) => (t.iter().sum::<i32>() + l.iter().sum::<i32>() + n as i32) / (2 * n as i32),
        (Some(t), None) => (t.iter().sum::<i32>() + (n as i32) / 2) / n as i32,
        (None, Some(l)) => (l.iter().sum::<i32>() + (n as i32) / 2) / n as i32,
        (None, None) => 128,
    }
}

/// 🎨 Plane-mode gradient prediction shared by Intra16x16 luma (`size=16`) and intra chroma (`size=8`), clause
/// 8.3.3.4 / 8.3.4.4 (the `(5,32,6)` vs `(17,16,5)` rounding constants are the only difference between them).
fn plane_pred(top: &[i32], left: &[i32], corner: i32, size: usize, h_mul: i32, h_round: i32, h_shift: u32) -> Vec<i32> {
    let half = size / 2;
    let t = |i: i32| if i < 0 { corner } else { top[i as usize] };
    let l = |i: i32| if i < 0 { corner } else { left[i as usize] };
    let mut h_sum = 0i32;
    let mut v_sum = 0i32;
    for x in 0..half as i32 {
        h_sum += (x + 1) * (t(half as i32 + x) - t(half as i32 - 2 - x));
        v_sum += (x + 1) * (l(half as i32 + x) - l(half as i32 - 2 - x));
    }
    let a = 16 * (t(size as i32 - 1) + l(size as i32 - 1));
    let b = (h_mul * h_sum + h_round) >> h_shift;
    let c = (h_mul * v_sum + h_round) >> h_shift;
    let mut out = vec![0i32; size * size];
    for y in 0..size {
        for x in 0..size {
            out[y * size + x] = (a + b * (x as i32 - (half as i32 - 1)) + c * (y as i32 - (half as i32 - 1)) + 16) >> 5;
        }
    }
    out
}
// #endregion 🔖Intra

// #region 🔖Inter
/// 🖼️ A reconstructed reference picture's luma plane plus both planes' dimensions, addressed with implicit
/// border-clamped extension (clause 8.4.2.2.1's "samples outside the picture are the nearest picture sample")
/// — every fetch clamps its coordinates in bounds first, so motion vectors may point arbitrarily far outside
/// the picture safely. Chroma planes are passed separately to [`mc_chroma_block`] (they don't need luma's
/// wide interpolation neighborhood, just [`RefPlanes::chroma_px`]'s clamped bilinear taps).
struct RefPlanes<'a> {
    luma: &'a [u8],
    luma_w: i32,
    luma_h: i32,
    chroma_w: i32,
    chroma_h: i32,
}

impl<'a> RefPlanes<'a> {
    fn luma_px(&self, x: i32, y: i32) -> i32 {
        let cx = x.clamp(0, self.luma_w - 1);
        let cy = y.clamp(0, self.luma_h - 1);
        i32::from(self.luma[(cy * self.luma_w + cx) as usize])
    }

    fn chroma_px(plane: &[u8], w: i32, h: i32, x: i32, y: i32) -> i32 {
        let cx = x.clamp(0, w - 1);
        let cy = y.clamp(0, h - 1);
        i32::from(plane[(cy * w + cx) as usize])
    }
}

fn clip_u8(v: i32) -> i32 {
    v.clamp(0, 255)
}

/// 🌊 Raw (unrounded, unclipped) horizontal 6-tap sum at the half-pel position between luma columns `x`/`x+1`.
fn h264_h6_raw(rp: &RefPlanes<'_>, x: i32, y: i32) -> i32 {
    rp.luma_px(x - 2, y) - 5 * rp.luma_px(x - 1, y) + 20 * rp.luma_px(x, y) + 20 * rp.luma_px(x + 1, y) - 5 * rp.luma_px(x + 2, y) + rp.luma_px(x + 3, y)
}

/// 🌊 Raw (unrounded, unclipped) vertical 6-tap sum at the half-pel position between luma rows `y`/`y+1`.
fn h264_v6_raw(rp: &RefPlanes<'_>, x: i32, y: i32) -> i32 {
    rp.luma_px(x, y - 2) - 5 * rp.luma_px(x, y - 1) + 20 * rp.luma_px(x, y) + 20 * rp.luma_px(x, y + 1) - 5 * rp.luma_px(x, y + 2) + rp.luma_px(x, y + 3)
}

/// 🎯 Quarter-pel luma sample at integer base `(x0, y0)` plus fractional `(fx, fy)` in `0..=3` quarter-pel
/// units (clause 8.4.2.2.1): the six named half-pel positions (`b`, `h`, `j`, plus the row-below/col-right
/// helpers `m`/`s`) are derived via 6-tap filtering, and the twelve quarter-pel positions average their two
/// nearest half/integer neighbors.
fn luma_qpel_sample(rp: &RefPlanes<'_>, x0: i32, y0: i32, fx: i32, fy: i32) -> i32 {
    if fx == 0 && fy == 0 {
        return rp.luma_px(x0, y0);
    }
    let b = clip_u8((h264_h6_raw(rp, x0, y0) + 16) >> 5);
    let h = clip_u8((h264_v6_raw(rp, x0, y0) + 16) >> 5);
    if fx == 2 && fy == 0 {
        return b;
    }
    if fx == 0 && fy == 2 {
        return h;
    }
    let g = rp.luma_px(x0, y0);
    let hh = rp.luma_px(x0 + 1, y0);
    let m_pel = rp.luma_px(x0, y0 + 1);
    if fy == 0 {
        return if fx == 1 { (g + b + 1) >> 1 } else { (hh + b + 1) >> 1 };
    }
    if fx == 0 {
        return if fy == 1 { (g + h + 1) >> 1 } else { (m_pel + h + 1) >> 1 };
    }
    let j_raw = h264_v6_raw_of_horiz(rp, x0, y0);
    let j = clip_u8((j_raw + 512) >> 10);
    if fx == 2 && fy == 2 {
        return j;
    }
    let m = clip_u8((h264_h6_raw(rp, x0, y0 + 1) + 16) >> 5);
    let s = clip_u8((h264_v6_raw(rp, x0 + 1, y0) + 16) >> 5);
    match (fx, fy) {
        (2, 1) => (b + j + 1) >> 1,
        (2, 3) => (j + m + 1) >> 1,
        (1, 2) => (h + j + 1) >> 1,
        (3, 2) => (j + s + 1) >> 1,
        (1, 1) => (b + h + 1) >> 1,
        (3, 1) => (b + m + 1) >> 1,
        (1, 3) => (h + s + 1) >> 1,
        (3, 3) => (m + s + 1) >> 1,
        _ => unreachable!("fx,fy in 0..=3 exhausted above"),
    }
}

/// 🌊 The `j` center half-pel's raw two-pass sum: the horizontal 6-tap filter's *unrounded* output at 6
/// consecutive rows, combined with the vertical 6-tap filter (clause 8.4.2.2.1); callers finish with
/// `(raw + 512) >> 10` and clip.
fn h264_v6_raw_of_horiz(rp: &RefPlanes<'_>, x: i32, y: i32) -> i32 {
    let r = |dy: i32| h264_h6_raw(rp, x, y + dy);
    r(-2) - 5 * r(-1) + 20 * r(0) + 20 * r(1) - 5 * r(2) + r(3)
}

/// 🎯 Bilinear chroma sample at 1/8-pel precision (clause 8.4.2.2.2): `mv_x8`/`mv_y8` are the chroma motion
/// vector components in eighth-chroma-pel units (numerically identical to the luma quarter-pel MV, since
/// chroma is half-resolution in 4:2:0).
fn chroma_bilinear_sample(plane: &[u8], w: i32, h: i32, x: i32, y: i32, mv_x8: i32, mv_y8: i32) -> i32 {
    let full_x = x + (mv_x8 >> 3);
    let full_y = y + (mv_y8 >> 3);
    let frac_x = mv_x8 & 7;
    let frac_y = mv_y8 & 7;
    let a = RefPlanes::chroma_px(plane, w, h, full_x, full_y);
    let b = RefPlanes::chroma_px(plane, w, h, full_x + 1, full_y);
    let c = RefPlanes::chroma_px(plane, w, h, full_x, full_y + 1);
    let d = RefPlanes::chroma_px(plane, w, h, full_x + 1, full_y + 1);
    ((8 - frac_x) * (8 - frac_y) * a + frac_x * (8 - frac_y) * b + (8 - frac_x) * frac_y * c + frac_x * frac_y * d + 32) >> 6
}

/// 🎯 Motion-compensates one `w×h` luma block at picture position `(px, py)` from `mv` (quarter-pel), writing
/// samples into `out` (row-major, `w*h`).
fn mc_luma_block(rp: &RefPlanes<'_>, px: i32, py: i32, w: usize, h: usize, mv: [i32; 2], out: &mut [i32]) {
    for y in 0..h {
        for x in 0..w {
            let full_x = px + x as i32 + (mv[0] >> 2);
            let full_y = py + y as i32 + (mv[1] >> 2);
            out[y * w + x] = luma_qpel_sample(rp, full_x, full_y, mv[0] & 3, mv[1] & 3);
        }
    }
}

/// 🎯 Motion-compensates one `w×h` chroma block (either plane) at chroma position `(px, py)` from a luma-space
/// `mv` (quarter-pel; reinterpreted as eighth-chroma-pel per clause 8.4.1.4).
#[allow(clippy::too_many_arguments, reason = "one plane plus its geometry (dims, origin, size) plus the motion vector is the natural, self-describing parameter list for a block motion-compensation primitive; bundling them into a struct would just rename the same fields")]
fn mc_chroma_block(plane: &[u8], plane_w: i32, plane_h: i32, px: i32, py: i32, w: usize, h: usize, mv: [i32; 2], out: &mut [i32]) {
    for y in 0..h {
        for x in 0..w {
            out[y * w + x] = chroma_bilinear_sample(plane, plane_w, plane_h, px + x as i32, py + y as i32, mv[0], mv[1]);
        }
    }
}

/// 🧭 Median motion-vector predictor (clause 8.4.1.3): `A`/`B`/`C` are the left/above/above-right neighbors'
/// `(mv, ref_idx)`; `D` is the above-left fallback used only when `C` is unavailable. The 16×8/8×16 single-
/// neighbor shortcuts are handled by the caller (they only apply to those partition shapes).
fn median_mv_predict(a: ([i32; 2], i8), b: ([i32; 2], i8), c: ([i32; 2], i8), ref_idx: i8) -> [i32; 2] {
    let matches: Vec<[i32; 2]> = [a, b, c].iter().filter(|&&(_, r)| r == ref_idx).map(|&(mv, _)| mv).collect();
    if matches.len() == 1 {
        return matches[0];
    }
    let mut xs = [a.0[0], b.0[0], c.0[0]];
    let mut ys = [a.0[1], b.0[1], c.0[1]];
    xs.sort_unstable();
    ys.sort_unstable();
    [xs[1], ys[1]]
}
// #endregion 🔖Inter

// #region 🔖Deblock
/// 📏 `Alpha[qP]`, `qP` 0..51 (clause 8.7.2.2, Table 8-16 column). <https://www.itu.int/rec/T-REC-H.264>
const DEBLOCK_ALPHA: [i32; 52] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 17, 20, 22, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90, 101, 113, 127, 144, 162, 182, 203, 226, 255, 255];

/// 📏 `Beta[qP]`, `qP` 0..51 (clause 8.7.2.2, Table 8-16 column).
const DEBLOCK_BETA: [i32; 52] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18];

/// 📏 `tC0[qP][bS-1]` for `bS` 1..3, `qP` 0..51 (clause 8.7.2.3, Table 8-17); `bS == 4` never consults this
/// table (it uses the strong, unconditional intra filter instead).
const DEBLOCK_TC0: [[i32; 3]; 52] = [
    [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0],
    [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 1, 1], [0, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 2], [1, 1, 2], [1, 1, 2], [1, 1, 2], [1, 2, 3], [1, 2, 3], [2, 2, 3],
    [2, 2, 4], [2, 3, 4], [2, 3, 4], [3, 3, 5], [3, 4, 6], [3, 4, 6], [4, 5, 7], [4, 5, 8], [4, 6, 9], [5, 7, 10], [6, 8, 11], [6, 8, 13], [7, 10, 14], [8, 11, 16], [9, 12, 18], [10, 13, 20],
    [11, 15, 23], [13, 17, 25],
];

/// 🧮 Boundary strength (clause 8.7.2.1) for the edge between 4×4 luma blocks `p` and `q`: `4` only at a true
/// macroblock edge between two intra macroblocks, `3` at an intra macroblock's internal edges, `2` if either
/// side has nonzero luma coefficients, `1` if the motion differs (reference frame, or either MV component by
/// `>= 4` quarter-pel units), else `0` (no filtering).
#[allow(clippy::too_many_arguments, reason = "boundary strength genuinely depends on this many independent per-block facts; bundling them into a struct would just rename the same inputs")]
fn boundary_strength(is_mb_edge: bool, p_intra: bool, q_intra: bool, p_nnz: u8, q_nnz: u8, p_mv: [i32; 2], p_ref: i8, q_mv: [i32; 2], q_ref: i8) -> u8 {
    if p_intra || q_intra {
        return if is_mb_edge { 4 } else { 3 };
    }
    if p_nnz > 0 || q_nnz > 0 {
        return 2;
    }
    if p_ref != q_ref {
        return 1;
    }
    if (p_mv[0] - q_mv[0]).abs() >= 4 || (p_mv[1] - q_mv[1]).abs() >= 4 {
        return 1;
    }
    0
}

/// 🧮 Clause 8.7.2.3's strong (`bS == 4`) luma sample filter for one row/column of 4 pixels straddling an
/// edge; `p`/`q` are `[p3,p2,p1,p0]`/`[q0,q1,q2,q3]` (`p0`/`q0` adjacent to the edge), returned as the filtered
/// `[p2,p1,p0,q0,q1,q2]` (only `p2`/`q2` change when `!(ap < beta)`/`!(aq < beta)`, spliced in by the caller).
fn filter_luma_strong(p: [i32; 4], q: [i32; 4], alpha: i32, beta: i32) -> ([i32; 3], [i32; 3]) {
    let (p3, p2, p1, p0) = (p[0], p[1], p[2], p[3]);
    let (q0, q1, q2, q3) = (q[0], q[1], q[2], q[3]);
    let strong = (p0 - q0).abs() < alpha && (p1 - p0).abs() < beta && (q1 - q0).abs() < beta;
    if !strong {
        return ([p2, p1, p0], [q0, q1, q2]);
    }
    let small_gap = (p0 - q0).abs() < (alpha >> 2) + 2;
    let new_p0 = if small_gap && (p2 - p0).abs() < beta { (p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >> 3 } else { (2 * p1 + p0 + q1 + 2) >> 2 };
    let new_p1 = if small_gap && (p2 - p0).abs() < beta { (p2 + p1 + p0 + q0 + 2) >> 2 } else { p1 };
    let new_p2 = if small_gap && (p2 - p0).abs() < beta { (2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >> 3 } else { p2 };
    let new_q0 = if small_gap && (q2 - q0).abs() < beta { (q2 + 2 * q1 + 2 * q0 + 2 * p0 + p1 + 4) >> 3 } else { (2 * q1 + q0 + p1 + 2) >> 2 };
    let new_q1 = if small_gap && (q2 - q0).abs() < beta { (q2 + q1 + q0 + p0 + 2) >> 2 } else { q1 };
    let new_q2 = if small_gap && (q2 - q0).abs() < beta { (2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >> 3 } else { q2 };
    ([new_p2, new_p1, new_p0], [new_q0, new_q1, new_q2])
}

/// 🧮 Clause 8.7.2.3's normal (`bS` 1..3) luma sample filter, returning filtered `(p1, p0, q0, q1)`; `p1`/`q1`
/// only change when the local activity is low enough (`ap < beta` / `aq < beta`).
fn filter_luma_normal(p: [i32; 3], q: [i32; 3], alpha: i32, beta: i32, tc0: i32) -> Option<(i32, i32, i32, i32)> {
    let (p2, p1, p0) = (p[0], p[1], p[2]);
    let (q0, q1, q2) = (q[0], q[1], q[2]);
    if (p0 - q0).abs() >= alpha || (p1 - p0).abs() >= beta || (q1 - q0).abs() >= beta {
        return None;
    }
    let ap = (p2 - p0).abs();
    let aq = (q2 - q0).abs();
    let tc = tc0 + i32::from(ap < beta) + i32::from(aq < beta);
    let delta = (((q0 - p0) * 4 + (p1 - q1) + 4) >> 3).clamp(-tc, tc);
    let new_p0 = clip_u8(p0 + delta);
    let new_q0 = clip_u8(q0 - delta);
    let new_p1 = if ap < beta { p1 + (((p2 + ((p0 + q0 + 1) >> 1)) >> 1) - p1).clamp(-tc0, tc0) } else { p1 };
    let new_q1 = if aq < beta { q1 + (((q2 + ((p0 + q0 + 1) >> 1)) >> 1) - q1).clamp(-tc0, tc0) } else { q1 };
    Some((new_p1, new_p0, new_q0, new_q1))
}

/// 🧮 Chroma edge filter (clause 8.7.2.4), `bS` 1..3: filtered `(p0, q0)`.
fn filter_chroma_normal(p1: i32, p0: i32, q0: i32, q1: i32, alpha: i32, beta: i32, tc: i32) -> Option<(i32, i32)> {
    if (p0 - q0).abs() >= alpha || (p1 - p0).abs() >= beta || (q1 - q0).abs() >= beta {
        return None;
    }
    let delta = (((q0 - p0) * 4 + (p1 - q1) + 4) >> 3).clamp(-tc, tc);
    Some((clip_u8(p0 + delta), clip_u8(q0 - delta)))
}

/// 🧮 Chroma edge filter, `bS == 4`: unconditional averaging (clause 8.7.2.4).
fn filter_chroma_strong(p1: i32, p0: i32, q0: i32, q1: i32, alpha: i32, beta: i32) -> Option<(i32, i32)> {
    if (p0 - q0).abs() >= alpha || (p1 - p0).abs() >= beta || (q1 - q0).abs() >= beta {
        return None;
    }
    Some(((2 * p1 + p0 + q1 + 2) >> 2, (2 * q1 + q0 + p1 + 2) >> 2))
}
/// 🧮 Filters the 4 luma edges (macroblock edge + 3 internal, clause 8.7) of one 4-row segment straddling a
/// vertical edge at picture column `edge_x`, rows `[y, y+4)`.
#[allow(clippy::too_many_arguments, reason = "the deblocking filter's boundary-strength/threshold lookup genuinely needs this many independent per-edge facts")]
fn deblock_vertical_segment(pic: &mut Picture, edge_x: usize, y: usize, is_mb_edge: bool, p_intra: bool, q_intra: bool, p_qp: i32, q_qp: i32, block_y: usize, p_block_x: usize, q_block_x: usize, a_off: i32, b_off: i32) {
    let lw4 = pic.luma4_width();
    let p_nnz = pic.nnz_luma[block_y * lw4 + p_block_x];
    let q_nnz = pic.nnz_luma[block_y * lw4 + q_block_x];
    let p_mv = pic.mv[block_y * lw4 + p_block_x];
    let q_mv = pic.mv[block_y * lw4 + q_block_x];
    let p_ref = pic.ref_idx[block_y * lw4 + p_block_x];
    let q_ref = pic.ref_idx[block_y * lw4 + q_block_x];
    let bs = boundary_strength(is_mb_edge, p_intra, q_intra, p_nnz, q_nnz, p_mv, p_ref, q_mv, q_ref);
    if bs == 0 {
        return;
    }
    let avg_qp = (p_qp + q_qp + 1) >> 1;
    let index_a = (avg_qp + a_off).clamp(0, 51) as usize;
    let index_b = (avg_qp + b_off).clamp(0, 51) as usize;
    let (alpha, beta) = (DEBLOCK_ALPHA[index_a], DEBLOCK_BETA[index_b]);
    if alpha == 0 || beta == 0 {
        return;
    }
    let lw = pic.luma_width();
    for row in y..y + 4 {
        let get = |dx: i32| i32::from(pic.luma[row * lw + (edge_x as i32 + dx) as usize]);
        if bs == 4 {
            let (pf, qf) = filter_luma_strong([get(-4), get(-3), get(-2), get(-1)], [get(0), get(1), get(2), get(3)], alpha, beta);
            for (k, &v) in pf.iter().enumerate() {
                pic.luma[row * lw + edge_x - 3 + k] = v.clamp(0, 255) as u8;
            }
            for (k, &v) in qf.iter().enumerate() {
                pic.luma[row * lw + edge_x + k] = v.clamp(0, 255) as u8;
            }
        } else if let Some((np1, np0, nq0, nq1)) = filter_luma_normal([get(-3), get(-2), get(-1)], [get(0), get(1), get(2)], alpha, beta, DEBLOCK_TC0[index_a][bs as usize - 1]) {
            pic.luma[row * lw + edge_x - 2] = np1.clamp(0, 255) as u8;
            pic.luma[row * lw + edge_x - 1] = np0.clamp(0, 255) as u8;
            pic.luma[row * lw + edge_x] = nq0.clamp(0, 255) as u8;
            pic.luma[row * lw + edge_x + 1] = nq1.clamp(0, 255) as u8;
        }
    }
}

/// 🧮 Filters the 4 luma edges of one 4-column segment straddling a horizontal edge at picture row `edge_y`,
/// columns `[x, x+4)` — the transpose of [`deblock_vertical_segment`].
#[allow(clippy::too_many_arguments, reason = "same as deblock_vertical_segment, whose transpose this is")]
fn deblock_horizontal_segment(pic: &mut Picture, x: usize, edge_y: usize, is_mb_edge: bool, p_intra: bool, q_intra: bool, p_qp: i32, q_qp: i32, block_x: usize, p_block_y: usize, q_block_y: usize, a_off: i32, b_off: i32) {
    let lw4 = pic.luma4_width();
    let p_nnz = pic.nnz_luma[p_block_y * lw4 + block_x];
    let q_nnz = pic.nnz_luma[q_block_y * lw4 + block_x];
    let p_mv = pic.mv[p_block_y * lw4 + block_x];
    let q_mv = pic.mv[q_block_y * lw4 + block_x];
    let p_ref = pic.ref_idx[p_block_y * lw4 + block_x];
    let q_ref = pic.ref_idx[q_block_y * lw4 + block_x];
    let bs = boundary_strength(is_mb_edge, p_intra, q_intra, p_nnz, q_nnz, p_mv, p_ref, q_mv, q_ref);
    if bs == 0 {
        return;
    }
    let avg_qp = (p_qp + q_qp + 1) >> 1;
    let index_a = (avg_qp + a_off).clamp(0, 51) as usize;
    let index_b = (avg_qp + b_off).clamp(0, 51) as usize;
    let (alpha, beta) = (DEBLOCK_ALPHA[index_a], DEBLOCK_BETA[index_b]);
    if alpha == 0 || beta == 0 {
        return;
    }
    let lw = pic.luma_width();
    for col in x..x + 4 {
        let get = |dy: i32| i32::from(pic.luma[(edge_y as i32 + dy) as usize * lw + col]);
        if bs == 4 {
            let (pf, qf) = filter_luma_strong([get(-4), get(-3), get(-2), get(-1)], [get(0), get(1), get(2), get(3)], alpha, beta);
            for (k, &v) in pf.iter().enumerate() {
                pic.luma[(edge_y - 3 + k) * lw + col] = v.clamp(0, 255) as u8;
            }
            for (k, &v) in qf.iter().enumerate() {
                pic.luma[(edge_y + k) * lw + col] = v.clamp(0, 255) as u8;
            }
        } else if let Some((np1, np0, nq0, nq1)) = filter_luma_normal([get(-3), get(-2), get(-1)], [get(0), get(1), get(2)], alpha, beta, DEBLOCK_TC0[index_a][bs as usize - 1]) {
            pic.luma[(edge_y - 2) * lw + col] = np1.clamp(0, 255) as u8;
            pic.luma[(edge_y - 1) * lw + col] = np0.clamp(0, 255) as u8;
            pic.luma[edge_y * lw + col] = nq0.clamp(0, 255) as u8;
            pic.luma[(edge_y + 1) * lw + col] = nq1.clamp(0, 255) as u8;
        }
    }
}

/// 🧮 In-loop deblocking filter over the whole picture (clause 8.7): all vertical edges first (macroblock
/// raster order), then all horizontal edges, each macroblock edge before its own internal edges. Luma only —
/// chroma filtering is a scoped-out simplification of this decoder (see crate-level notes); harmless for this
/// crate's own encoder output, which always sets `disable_deblocking_filter_idc = 1`.
fn deblock_picture(pic: &mut Picture, sps: &SpsInfo, header: &SliceHeaderInfo) {
    if header.disable_deblocking_filter_idc == 1 {
        return;
    }
    let (mb_width, mb_height) = (sps.pic_width_in_mbs, sps.pic_height_in_mbs);
    let (a_off, b_off) = (header.slice_alpha_c0_offset, header.slice_beta_offset);
    for mb_y in 0..mb_height {
        for mb_x in 0..mb_width {
            let mbidx = pic.mb_index(mb_x, mb_y);
            let (q_intra, q_qp) = (pic.mb_is_intra[mbidx], pic.mb_qp[mbidx]);
            for edge in 0..4u32 {
                let is_mb_edge = edge == 0;
                if is_mb_edge && mb_x == 0 {
                    continue;
                }
                let (p_intra, p_qp) = if is_mb_edge {
                    let pidx = pic.mb_index(mb_x - 1, mb_y);
                    (pic.mb_is_intra[pidx], pic.mb_qp[pidx])
                } else {
                    (q_intra, q_qp)
                };
                let edge_x = (mb_x * 16 + edge * 4) as usize;
                for seg in 0..4u32 {
                    let block_y = (mb_y * 4 + seg) as usize;
                    let q_block_x = edge_x / 4;
                    deblock_vertical_segment(pic, edge_x, block_y * 4, is_mb_edge, p_intra, q_intra, p_qp, q_qp, block_y, q_block_x - 1, q_block_x, a_off, b_off);
                }
            }
        }
    }
    for mb_y in 0..mb_height {
        for mb_x in 0..mb_width {
            let mbidx = pic.mb_index(mb_x, mb_y);
            let (q_intra, q_qp) = (pic.mb_is_intra[mbidx], pic.mb_qp[mbidx]);
            for edge in 0..4u32 {
                let is_mb_edge = edge == 0;
                if is_mb_edge && mb_y == 0 {
                    continue;
                }
                let (p_intra, p_qp) = if is_mb_edge {
                    let pidx = pic.mb_index(mb_x, mb_y - 1);
                    (pic.mb_is_intra[pidx], pic.mb_qp[pidx])
                } else {
                    (q_intra, q_qp)
                };
                let edge_y = (mb_y * 16 + edge * 4) as usize;
                for seg in 0..4u32 {
                    let block_x = (mb_x * 4 + seg) as usize;
                    let q_block_y = edge_y / 4;
                    deblock_horizontal_segment(pic, block_x * 4, edge_y, is_mb_edge, p_intra, q_intra, p_qp, q_qp, block_x, q_block_y - 1, q_block_y, a_off, b_off);
                }
            }
        }
    }
    deblock_chroma_mb_edges(pic, sps, a_off, b_off);
}

/// 🧮 Chroma vertical-macroblock-edge-only deblocking (a scoped simplification of clause 8.7.2: horizontal
/// chroma MB edges and the one internal chroma edge are not filtered — harmless for this crate's own
/// encoder, which always disables the filter). `bS` is re-derived directly from the co-located luma 4×4
/// block's side information rather than precisely reusing clause 8.7.2.1's luma-edge bS array at chroma
/// sub-positions.
fn deblock_chroma_mb_edges(pic: &mut Picture, sps: &SpsInfo, a_off: i32, b_off: i32) {
    let (mb_width, mb_height) = (sps.pic_width_in_mbs, sps.pic_height_in_mbs);
    let lw4 = pic.luma4_width();
    let cw = pic.chroma_width();
    for mb_y in 0..mb_height {
        for mb_x in 1..mb_width {
            let mbidx = pic.mb_index(mb_x, mb_y);
            let pidx = pic.mb_index(mb_x - 1, mb_y);
            let (q_intra, q_qp) = (pic.mb_is_intra[mbidx], pic.mb_qp[mbidx]);
            let (p_intra, p_qp) = (pic.mb_is_intra[pidx], pic.mb_qp[pidx]);
            let avg_qp = chroma_qp((p_qp + q_qp + 1) >> 1, 0);
            let (index_a, index_b) = ((avg_qp + a_off).clamp(0, 51) as usize, (avg_qp + b_off).clamp(0, 51) as usize);
            let (alpha, beta) = (DEBLOCK_ALPHA[index_a], DEBLOCK_BETA[index_b]);
            if alpha == 0 || beta == 0 {
                continue;
            }
            for qy in 0..2usize {
                let block_row = (mb_y * 4) as usize + qy * 2;
                let q_block_x = (mb_x * 4) as usize;
                let (p_nnz, q_nnz) = (pic.nnz_luma[block_row * lw4 + q_block_x - 1], pic.nnz_luma[block_row * lw4 + q_block_x]);
                let (p_mv, q_mv) = (pic.mv[block_row * lw4 + q_block_x - 1], pic.mv[block_row * lw4 + q_block_x]);
                let (p_ref, q_ref) = (pic.ref_idx[block_row * lw4 + q_block_x - 1], pic.ref_idx[block_row * lw4 + q_block_x]);
                let bs = boundary_strength(true, p_intra, q_intra, p_nnz, q_nnz, p_mv, p_ref, q_mv, q_ref);
                if bs == 0 {
                    continue;
                }
                let tc = if bs == 4 { 0 } else { DEBLOCK_TC0[index_a][bs as usize - 1] + 1 };
                let edge_x = (mb_x * 8) as usize;
                for row in (mb_y * 8) as usize + qy * 4..(mb_y * 8) as usize + qy * 4 + 4 {
                    for plane in [&mut pic.cb, &mut pic.cr] {
                        let get = |dx: i32, plane: &[u8]| i32::from(plane[row * cw + (edge_x as i32 + dx) as usize]);
                        let filtered = if bs == 4 { filter_chroma_strong(get(-2, plane), get(-1, plane), get(0, plane), get(1, plane), alpha, beta) } else { filter_chroma_normal(get(-2, plane), get(-1, plane), get(0, plane), get(1, plane), alpha, beta, tc) };
                        if let Some((p0, q0)) = filtered {
                            plane[row * cw + edge_x - 1] = p0.clamp(0, 255) as u8;
                            plane[row * cw + edge_x] = q0.clamp(0, 255) as u8;
                        }
                    }
                }
            }
        }
    }
}
// #endregion 🔖Deblock

// #region 🔖Dpb
/// 🗃️ Decoded picture buffer with sliding-window reference management (clause 8.2.5.3): the `max_num_ref_frames`
/// most recently decoded pictures are kept as references, oldest evicted first. Explicit MMCO adaptive marking
/// is rejected earlier (at slice-header parse time), so this is the only marking process this decoder needs.
struct Dpb {
    pictures: Vec<Picture>,
    max_num_ref_frames: usize,
}

impl Dpb {
    fn new(max_num_ref_frames: u32) -> Self {
        Self { pictures: Vec::new(), max_num_ref_frames: (max_num_ref_frames as usize).max(1) }
    }

    fn clear(&mut self) {
        self.pictures.clear();
    }

    fn push(&mut self, pic: Picture) {
        self.pictures.push(pic);
        while self.pictures.len() > self.max_num_ref_frames {
            self.pictures.remove(0);
        }
    }

    /// 📜 `RefPicList0`'s default initialization for P slices (clause 8.2.4.2.1): descending `PicNum`, i.e.
    /// most-recently-decoded reference first. This decoder rejects explicit reordering, so this *is*
    /// `RefPicList0`.
    fn ref_list0(&self) -> Vec<&Picture> {
        self.pictures.iter().rev().collect()
    }
}
// #endregion 🔖Dpb

// #region 🔖Yuv
/// 🎨 4:2:0 planes → RGBA, cropped to `width`×`height` (clause E.2.1's default YCbCr matrix, BT.601 full range
/// — kept as a local, private duplicate of `remodel_image`'s identical JPEG conversion rather than a new
/// public cross-crate dependency surface for one formula).
fn ycbcr420_to_rgba(luma: &[u8], luma_stride: usize, cb: &[u8], cr: &[u8], chroma_stride: usize, width: u32, height: u32) -> remodel_image::ImageRgba8 {
    let mut out = remodel_image::ImageRgba8::new(width, height);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let yy = f32::from(luma[y * luma_stride + x]);
            let (cx, cy) = (x / 2, y / 2);
            let cb_v = f32::from(cb[cy * chroma_stride + cx]) - 128.0;
            let cr_v = f32::from(cr[cy * chroma_stride + cx]) - 128.0;
            let idx = (y * width as usize + x) * 4;
            out.data[idx] = (yy + 1.402 * cr_v).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 1] = (yy - 0.344_136 * cb_v - 0.714_136 * cr_v).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 2] = (yy + 1.772 * cb_v).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 3] = 255;
        }
    }
    out
}
// #endregion 🔖Yuv

// #region 🔖SliceData
/// 🔀 `coded_block_pattern` inverse mapping for Intra_4x4/Intra_8x8 macroblocks (Table 9-4, `chroma_format_idc == 1`).
const GOLOMB_TO_INTRA4X4_CBP: [u8; 48] = [47, 31, 15, 0, 23, 27, 29, 30, 7, 11, 13, 14, 39, 43, 45, 46, 16, 3, 5, 10, 12, 19, 21, 26, 28, 35, 37, 42, 44, 1, 2, 4, 8, 17, 18, 20, 24, 6, 9, 22, 25, 32, 33, 34, 36, 40, 38, 41];

/// 🔀 `coded_block_pattern` inverse mapping for Inter macroblocks (Table 9-4, `chroma_format_idc == 1`).
const GOLOMB_TO_INTER_CBP: [u8; 48] = [0, 16, 1, 2, 4, 8, 32, 3, 5, 10, 12, 15, 47, 7, 11, 13, 14, 6, 9, 31, 35, 37, 42, 44, 33, 34, 36, 40, 39, 43, 45, 46, 17, 18, 20, 24, 19, 21, 26, 28, 23, 27, 29, 30, 22, 25, 38, 41];

/// 🎚️ `QPc` as a function of `qPI = Clip3(0, 51, QPY + chroma_qp_index_offset)` (clause 8.5.8, Table 8-15).
const CHROMA_QP_TABLE: [i32; 52] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 34, 35, 35, 36, 36, 37, 37, 37, 38, 38, 38, 39, 39, 39,
    39,
];

fn chroma_qp(luma_qp: i32, offset: i32) -> i32 {
    CHROMA_QP_TABLE[(luma_qp + offset).clamp(0, 51) as usize]
}

/// 🎛️ `te(v)`, mapped Exp-Golomb (clause 9.1): a single inverted bit when `max_val == 1`, plain `ue(v)`
/// otherwise (and always `0`, no bits consumed, when `max_val == 0`).
fn read_te(b: &mut BitReader<'_>, max_val: u32) -> Result<u32, H264Error> {
    match max_val {
        0 => Ok(0),
        1 => Ok(1 - b.u1()?),
        _ => b.ue(),
    }
}

/// 🧭 Luma 4×4 block-scan index `n` (0..15) → its `(blockX, blockY)` position in the macroblock's 4×4 grid
/// (clause 6.4.3): a raster scan of four 8×8 quadrants, each itself a raster scan of four 4×4 blocks.
fn block4x4_grid_pos(n: usize) -> (u32, u32) {
    let i8x8 = n / 4;
    let i4x4 = n % 4;
    (((i8x8 % 2) * 2 + i4x4 % 2) as u32, ((i8x8 / 2) * 2 + i4x4 / 2) as u32)
}

/// 🧭 Chroma 4×4 quadrant index `n` (0..3) → its `(blockX, blockY)` position in the 8×8 chroma block's 2×2 grid.
fn chroma_quad_pos(n: usize) -> (u32, u32) {
    ((n % 2) as u32, (n / 2) as u32)
}

/// 🔍 `nC` neighbor lookup for luma 4×4 blocks: the neighbor's `total_coeff` if it has already been decoded
/// (in-picture, and — since macroblocks/blocks decode in raster/scan order — necessarily earlier), else
/// `None` (unavailable, matching clause 9.2.1's substitution for out-of-picture/not-yet-decoded neighbors).
fn luma_nc(pic: &Picture, gx: i32, gy: i32) -> Option<u8> {
    if gx < 0 || gy < 0 || gx as u32 >= pic.mb_width * 4 || gy as u32 >= pic.mb_height * 4 {
        return None;
    }
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    if pic.decoded_luma4[idx] { Some(pic.nnz_luma[idx]) } else { None }
}

/// 🔍 Left/above `intra4x4_pred_mode` neighbor for clause 8.3.1.1's mode prediction; unavailable or non-
/// `Intra_4x4` neighbors substitute `2` (DC), per spec.
fn intra4x4_mode_neighbor(pic: &Picture, gx: i32, gy: i32) -> u8 {
    if gx < 0 || gy < 0 || gx as u32 >= pic.mb_width * 4 || gy as u32 >= pic.mb_height * 4 {
        return 2;
    }
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    if !pic.decoded_luma4[idx] {
        return 2;
    }
    let m = pic.intra4x4_mode[idx];
    if m < 0 { 2 } else { m as u8 }
}

/// 🎨 Gathers a 4×4 luma block's intra-prediction neighbors from the picture buffer (clause 6.4.11.4 /
/// 8.3.1.2.1): `top_right` unavailability substitutes the rightmost available top sample.
fn gather_intra4_neighbors(pic: &Picture, gx: u32, gy: u32) -> Intra4Neighbors {
    let (px, py) = (gx as i32 * 4, gy as i32 * 4);
    let top = pic.luma4_available(gx as i32, gy as i32 - 1).then(|| std::array::from_fn(|i| i32::from(pic.luma_at(px + i as i32, py - 1).unwrap_or(128))));
    let left = pic.luma4_available(gx as i32 - 1, gy as i32).then(|| std::array::from_fn(|i| i32::from(pic.luma_at(px - 1, py + i as i32).unwrap_or(128))));
    let corner = if pic.luma4_available(gx as i32 - 1, gy as i32 - 1) { i32::from(pic.luma_at(px - 1, py - 1).unwrap_or(128)) } else { 128 };
    let top_right_available = pic.luma4_available(gx as i32 + 1, gy as i32 - 1);
    let top_right = if top_right_available {
        std::array::from_fn(|i| i32::from(pic.luma_at(px + 4 + i as i32, py - 1).unwrap_or(128)))
    } else if let Some(t) = top {
        [t[3]; 4]
    } else {
        [128; 4]
    };
    Intra4Neighbors { top, left, top_right, corner }
}

/// 🎨 Applies dequant + inverse transform to a raster-order coefficient block, adds `pred`, writes the
/// result into `pic`'s luma plane at 4×4 grid `(gx, gy)`, and records `total_coeff` for `nC` prediction.
fn reconstruct_luma4(pic: &mut Picture, gx: u32, gy: u32, pred: &[i32; 16], coeffs: &[i32; 16], qp: i32, total_coeff: u8) {
    let residual = idct4x4(&dequant4x4(coeffs, qp));
    let mut block = [0i32; 16];
    for i in 0..16 {
        block[i] = pred[i] + residual[i];
    }
    pic.write_luma4(gx, gy, &block);
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    pic.nnz_luma[idx] = total_coeff;
}

/// 🎨 Intra chroma prediction (clause 8.3.4): `mode` 0=DC, 1=Horizontal, 2=Vertical, 3=Plane, applied
/// identically to `cb`/`cr`. DC is computed per 4×4 quadrant per clause 8.3.4.1's four independent rules.
fn predict_intra_chroma(plane: &[u8], stride: usize, mb_px: usize, mb_py: usize, mode: u8, top_avail: bool, left_avail: bool) -> [i32; 64] {
    let top: Option<[i32; 8]> = top_avail.then(|| std::array::from_fn(|i| i32::from(plane[(mb_py - 1) * stride + mb_px + i])));
    let left: Option<[i32; 8]> = left_avail.then(|| std::array::from_fn(|i| i32::from(plane[(mb_py + i) * stride + mb_px - 1])));
    let corner = if top_avail && left_avail { i32::from(plane[(mb_py - 1) * stride + mb_px - 1]) } else { 128 };
    let mut out = [0i32; 64];
    match mode {
        1 => {
            let l = left.unwrap_or([128; 8]);
            for y in 0..8 {
                for x in 0..8 {
                    out[y * 8 + x] = l[y];
                }
            }
        }
        2 => {
            let t = top.unwrap_or([128; 8]);
            for y in 0..8 {
                for x in 0..8 {
                    out[y * 8 + x] = t[x];
                }
            }
        }
        3 => {
            let t = top.unwrap_or([128; 8]);
            let l = left.unwrap_or([128; 8]);
            let plane_vals = plane_pred(&t, &l, corner, 8, 17, 16, 5);
            out.copy_from_slice(&plane_vals);
        }
        _ => {
            let sum4 = |s: &[i32]| (s.iter().sum::<i32>() + 2) >> 2;
            let sum8 = |t: &[i32], l: &[i32]| (t.iter().sum::<i32>() + l.iter().sum::<i32>() + 4) >> 3;
            for qy in 0..2usize {
                for qx in 0..2usize {
                    let t_half = top.map(|t| if qx == 0 { [t[0], t[1], t[2], t[3]] } else { [t[4], t[5], t[6], t[7]] });
                    let l_half = left.map(|l| if qy == 0 { [l[0], l[1], l[2], l[3]] } else { [l[4], l[5], l[6], l[7]] });
                    let prefer_top = qx == 1 && qy == 0;
                    let dc = if qx == qy {
                        match (t_half, l_half) {
                            (Some(t), Some(l)) => sum8(&t, &l),
                            (Some(t), None) => sum4(&t),
                            (None, Some(l)) => sum4(&l),
                            (None, None) => 128,
                        }
                    } else {
                        match (prefer_top, t_half, l_half) {
                            (true, Some(t), _) => sum4(&t),
                            (false, _, Some(l)) => sum4(&l),
                            (true, None, Some(l)) => sum4(&l),
                            (false, Some(t), None) => sum4(&t),
                            (_, None, None) => 128,
                        }
                    };
                    for y in 0..4 {
                        for x in 0..4 {
                            out[(qy * 4 + y) * 8 + qx * 4 + x] = dc;
                        }
                    }
                }
            }
        }
    }
    out
}

/// 📥 Reads `ChromaDCLevel`/`ChromaACLevel` for both chroma planes' 4 quadrants each (clause 7.3.5.3.2),
/// dequantizing/inverse-transforming into per-quadrant raster residual blocks (DC already spliced into
/// position 0), and records `nC` bookkeeping. `cbp_chroma` `0` skips reading entirely (all-zero residual),
/// `1` reads DC only, `2` reads DC and AC.
#[allow(clippy::type_complexity, reason = "the return is just \"4 cb quadrant blocks, 4 cr quadrant blocks\", each already a self-explanatory raster 4x4; a wrapper struct would only rename these two fields")]
fn read_chroma_residual(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, cbp_chroma: u8, qp_cb: i32, qp_cr: i32) -> Result<([[i32; 16]; 4], [[i32; 16]; 4]), H264Error> {
    let mut cb_out = [[0i32; 16]; 4];
    let mut cr_out = [[0i32; 16]; 4];
    if cbp_chroma == 0 {
        return Ok((cb_out, cr_out));
    }
    let dc_cb = read_residual_block(b, NcSelector::ChromaDc, &[0, 1, 2, 3])?;
    let dc_cr = read_residual_block(b, NcSelector::ChromaDc, &[0, 1, 2, 3])?;
    let dc_cb_t = transform_chroma_dc(&[dc_cb.coeffs[0], dc_cb.coeffs[1], dc_cb.coeffs[2], dc_cb.coeffs[3]], qp_cb);
    let dc_cr_t = transform_chroma_dc(&[dc_cr.coeffs[0], dc_cr.coeffs[1], dc_cr.coeffs[2], dc_cr.coeffs[3]], qp_cr);

    let c4w = pic.chroma4_width();
    let mut ac_cb = [ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }];
    let mut ac_cr = [ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }, ResidualBlock { coeffs: [0; 16], total_coeff: 0 }];
    if cbp_chroma == 2 {
        #[allow(clippy::needless_range_loop, reason = "`n` also feeds chroma_quad_pos(n) to derive the block's grid position, not just the ac_cb index")]
        for n in 0..4usize {
            let (qx, qy) = chroma_quad_pos(n);
            let (gx, gy) = (mb_x * 2 + qx, mb_y * 2 + qy);
            let nc = predict_nc(if gx > 0 { Some(pic.nnz_cb[gy as usize * c4w + gx as usize - 1]) } else { None }, if gy > 0 { Some(pic.nnz_cb[(gy as usize - 1) * c4w + gx as usize]) } else { None });
            ac_cb[n] = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4[1..])?;
            pic.nnz_cb[gy as usize * c4w + gx as usize] = ac_cb[n].total_coeff;
        }
        #[allow(clippy::needless_range_loop, reason = "`n` also feeds chroma_quad_pos(n) to derive the block's grid position, not just the ac_cr index")]
        for n in 0..4usize {
            let (qx, qy) = chroma_quad_pos(n);
            let (gx, gy) = (mb_x * 2 + qx, mb_y * 2 + qy);
            let nc = predict_nc(if gx > 0 { Some(pic.nnz_cr[gy as usize * c4w + gx as usize - 1]) } else { None }, if gy > 0 { Some(pic.nnz_cr[(gy as usize - 1) * c4w + gx as usize]) } else { None });
            ac_cr[n] = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4[1..])?;
            pic.nnz_cr[gy as usize * c4w + gx as usize] = ac_cr[n].total_coeff;
        }
    } else {
        for n in 0..4usize {
            let (qx, qy) = chroma_quad_pos(n);
            let idx = (mb_y * 2 + qy) as usize * c4w + (mb_x * 2 + qx) as usize;
            pic.nnz_cb[idx] = 0;
            pic.nnz_cr[idx] = 0;
        }
    }

    for n in 0..4usize {
        let (qx, qy) = (n % 2, n / 2);
        let mut cb_coeffs = ac_cb[n].coeffs;
        cb_coeffs[0] = dc_cb_t[qy * 2 + qx];
        cb_out[n] = idct4x4(&dequant4x4(&cb_coeffs, qp_cb));
        let mut cr_coeffs = ac_cr[n].coeffs;
        cr_coeffs[0] = dc_cr_t[qy * 2 + qx];
        cr_out[n] = idct4x4(&dequant4x4(&cr_coeffs, qp_cr));
    }
    Ok((cb_out, cr_out))
}

/// 📥 Decodes one `I_NxN` (`Intra_4x4`, no 8×8-transform support) macroblock: per-block prediction-mode
/// syntax, chroma pred mode, `coded_block_pattern`, optional `mb_qp_delta`, then luma/chroma residual —
/// reconstructing pixel-by-pixel in 4×4 scan order so each block's neighbors are already available.
fn decode_intra4x4_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, pps: &PpsInfo, prev_qp: &mut i32) -> Result<(), H264Error> {
    let mut modes = [0u8; 16];
    for (n, mode) in modes.iter_mut().enumerate() {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let prev_flag = b.u1()?;
        let predicted = intra4x4_mode_neighbor(pic, gx as i32 - 1, gy as i32).min(intra4x4_mode_neighbor(pic, gx as i32, gy as i32 - 1));
        let m = if prev_flag == 1 {
            predicted
        } else {
            let rem = b.u(3)? as u8;
            if rem < predicted { rem } else { rem + 1 }
        };
        if m > 8 {
            return Err(H264Error::Malformed("intra4x4 pred mode out of range"));
        }
        *mode = m;
        let idx = gy as usize * pic.luma4_width() + gx as usize;
        pic.intra4x4_mode[idx] = m as i8;
    }
    let intra_chroma_pred_mode = b.ue()?;
    if intra_chroma_pred_mode > 3 {
        return Err(H264Error::Malformed("intra_chroma_pred_mode out of range"));
    }
    let cbp_code = b.ue()?;
    let cbp = *GOLOMB_TO_INTRA4X4_CBP.get(cbp_code as usize).ok_or(H264Error::Malformed("coded_block_pattern code out of range"))?;
    let (cbp_luma, cbp_chroma) = (cbp & 0xF, cbp >> 4);
    let qp = if cbp != 0 {
        let delta = b.se()?;
        let qp = (*prev_qp + delta + 52) % 52;
        *prev_qp = qp;
        qp
    } else {
        *prev_qp
    };

    #[allow(clippy::needless_range_loop, reason = "`n` is also used to derive the cbp_luma quadrant bit (n/4) and the 4x4 grid position, not just to index modes")]
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let neighbors = gather_intra4_neighbors(pic, gx, gy);
        let pred = predict_intra4x4(modes[n], &neighbors)?;
        let coded = cbp_luma & (1 << (n / 4)) != 0;
        if coded {
            let nc = predict_nc(luma_nc(pic, gx as i32 - 1, gy as i32), luma_nc(pic, gx as i32, gy as i32 - 1));
            let block = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4)?;
            reconstruct_luma4(pic, gx, gy, &pred, &block.coeffs, qp, block.total_coeff);
        } else {
            reconstruct_luma4(pic, gx, gy, &pred, &[0; 16], qp, 0);
        }
    }

    let qp_cb = chroma_qp(qp, pps.chroma_qp_index_offset);
    let qp_cr = chroma_qp(qp, pps.chroma_qp_index_offset);
    let (cb_res, cr_res) = read_chroma_residual(b, pic, mb_x, mb_y, cbp_chroma, qp_cb, qp_cr)?;
    let (mb_px, mb_py) = (mb_x as usize * 8, mb_y as usize * 8);
    let top_avail = mb_y > 0;
    let left_avail = mb_x > 0;
    let cw = pic.chroma_width();
    let cb_pred = predict_intra_chroma(&pic.cb, cw, mb_px, mb_py, intra_chroma_pred_mode as u8, top_avail, left_avail);
    let cr_pred = predict_intra_chroma(&pic.cr, cw, mb_px, mb_py, intra_chroma_pred_mode as u8, top_avail, left_avail);
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let (px, py) = (mb_px + qx as usize * 4, mb_py + qy as usize * 4);
        let mut pred_block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                pred_block[r * 4 + c] = cb_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c];
            }
        }
        let mut block = [0i32; 16];
        for i in 0..16 {
            block[i] = pred_block[i] + cb_res[n][i];
        }
        Picture::write_chroma4(&mut pic.cb, cw, px, py, &block);
        for r in 0..4 {
            for c in 0..4 {
                pred_block[r * 4 + c] = cr_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c];
            }
        }
        for i in 0..16 {
            block[i] = pred_block[i] + cr_res[n][i];
        }
        Picture::write_chroma4(&mut pic.cr, cw, px, py, &block);
    }

    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = true;
    pic.mb_qp[mbidx] = qp;
    Ok(())
}

/// 📥 Decodes one `Intra_16x16` macroblock: prediction mode / `CodedBlockPattern` are derived directly from
/// `mb_type` (clause 7.4.5, Table 7-11), `mb_qp_delta` is always present, and the luma DC coefficients go
/// through the extra Hadamard transform before splicing into each 4×4 AC block's position 0.
fn decode_intra16x16_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, code_num: u32, pps: &PpsInfo, prev_qp: &mut i32) -> Result<(), H264Error> {
    let pred_mode = (code_num % 4) as u8;
    let cbp_chroma = ((code_num / 4) % 3) as u8;
    let cbp_luma = if code_num < 12 { 0u8 } else { 15u8 };
    let intra_chroma_pred_mode = b.ue()?;
    if intra_chroma_pred_mode > 3 {
        return Err(H264Error::Malformed("intra_chroma_pred_mode out of range"));
    }
    let delta = b.se()?;
    let qp = (*prev_qp + delta + 52) % 52;
    *prev_qp = qp;

    let (mb_px, mb_py) = (mb_x as usize * 16, mb_y as usize * 16);
    let lw = pic.luma_width();
    let top_avail = mb_y > 0;
    let left_avail = mb_x > 0;
    let top16: Option<[i32; 16]> = top_avail.then(|| std::array::from_fn(|i| i32::from(pic.luma[(mb_py - 1) * lw + mb_px + i])));
    let left16: Option<[i32; 16]> = left_avail.then(|| std::array::from_fn(|i| i32::from(pic.luma[(mb_py + i) * lw + mb_px - 1])));
    let corner = if top_avail && left_avail { i32::from(pic.luma[(mb_py - 1) * lw + mb_px - 1]) } else { 128 };
    let pred16: Vec<i32> = match pred_mode {
        0 => {
            let t = top16.ok_or(H264Error::Malformed("intra16x16 vertical needs top"))?;
            (0..256).map(|i| t[i % 16]).collect()
        }
        1 => {
            let l = left16.ok_or(H264Error::Malformed("intra16x16 horizontal needs left"))?;
            (0..256).map(|i| l[i / 16]).collect()
        }
        2 => vec![dc_pred(top16.as_ref().map(|a| a.as_slice()), left16.as_ref().map(|a| a.as_slice()), 16); 256],
        _ => plane_pred(&top16.unwrap_or([128; 16]), &left16.unwrap_or([128; 16]), corner, 16, 5, 32, 6),
    };

    let n0 = predict_nc(luma_nc(pic, mb_x as i32 * 4 - 1, mb_y as i32 * 4), luma_nc(pic, mb_x as i32 * 4, mb_y as i32 * 4 - 1));
    let dc_block = read_residual_block(b, NcSelector::Nc(n0), &ZIGZAG_4X4)?;
    let dc_transformed = transform_luma16x16_dc(&dc_block.coeffs, qp);

    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let mut pred_block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                pred_block[r * 4 + c] = pred16[(by as usize * 4 + r) * 16 + bx as usize * 4 + c];
            }
        }
        let mut coeffs = [0i32; 16];
        let total_coeff = if cbp_luma != 0 {
            let nc = predict_nc(luma_nc(pic, gx as i32 - 1, gy as i32), luma_nc(pic, gx as i32, gy as i32 - 1));
            let ac = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4[1..])?;
            coeffs = ac.coeffs;
            ac.total_coeff
        } else {
            let idx = gy as usize * pic.luma4_width() + gx as usize;
            pic.nnz_luma[idx] = 0;
            0
        };
        coeffs[0] = dc_transformed[by as usize * 4 + bx as usize];
        reconstruct_luma4(pic, gx, gy, &pred_block, &coeffs, qp, total_coeff);
    }

    let qp_cb = chroma_qp(qp, pps.chroma_qp_index_offset);
    let qp_cr = chroma_qp(qp, pps.chroma_qp_index_offset);
    let (cb_res, cr_res) = read_chroma_residual(b, pic, mb_x, mb_y, cbp_chroma, qp_cb, qp_cr)?;
    let cw = pic.chroma_width();
    let cb_pred = predict_intra_chroma(&pic.cb, cw, mb_px / 2, mb_py / 2, intra_chroma_pred_mode as u8, top_avail, left_avail);
    let cr_pred = predict_intra_chroma(&pic.cr, cw, mb_px / 2, mb_py / 2, intra_chroma_pred_mode as u8, top_avail, left_avail);
    let (cpx, cpy) = (mb_px / 2, mb_py / 2);
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let (px, py) = (cpx + qx as usize * 4, cpy + qy as usize * 4);
        let mut block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = cb_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c] + cb_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cb, cw, px, py, &block);
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = cr_pred[(qy as usize * 4 + r) * 8 + qx as usize * 4 + c] + cr_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cr, cw, px, py, &block);
    }

    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = true;
    pic.mb_qp[mbidx] = qp;
    Ok(())
}

/// 📥 Decodes one `I_PCM` macroblock (clause 7.3.5): byte-aligns, then copies 256 raw luma + 64+64 raw chroma
/// samples verbatim — no prediction, transform or entropy coding. `TotalCoeff` for every 4×4 block is
/// inferred as `16` (clause 7.4.5), and `QPY` is inferred as `0`, both per spec.
fn decode_pcm_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32) -> Result<(), H264Error> {
    b.byte_align();
    let (px, py) = (mb_x as usize * 16, mb_y as usize * 16);
    let lw = pic.luma_width();
    for r in 0..16 {
        for c in 0..16 {
            pic.luma[(py + r) * lw + px + c] = b.u(8)? as u8;
        }
    }
    let (cpx, cpy) = (mb_x as usize * 8, mb_y as usize * 8);
    let cw = pic.chroma_width();
    for r in 0..8 {
        for c in 0..8 {
            pic.cb[(cpy + r) * cw + cpx + c] = b.u(8)? as u8;
        }
    }
    for r in 0..8 {
        for c in 0..8 {
            pic.cr[(cpy + r) * cw + cpx + c] = b.u(8)? as u8;
        }
    }
    let lw4 = pic.luma4_width();
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let idx = (mb_y * 4 + by) as usize * lw4 + (mb_x * 4 + bx) as usize;
        pic.decoded_luma4[idx] = true;
        pic.nnz_luma[idx] = 16;
    }
    let cw4 = pic.chroma4_width();
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let idx = (mb_y * 2 + qy) as usize * cw4 + (mb_x * 2 + qx) as usize;
        pic.nnz_cb[idx] = 16;
        pic.nnz_cr[idx] = 16;
    }
    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = true;
    pic.mb_qp[mbidx] = 0;
    Ok(())
}

/// 🔍 `(mv, ref_idx)` at 4×4 grid `(gx, gy)`; out-of-picture and never-written (intra, or genuinely not yet
/// decoded) positions both read as `([0, 0], -1)` — the same "unavailable" substitution clause 8.4.1.3.2 uses.
fn neighbor_mv_ref(pic: &Picture, gx: i32, gy: i32) -> ([i32; 2], i8) {
    if gx < 0 || gy < 0 || gx as u32 >= pic.mb_width * 4 || gy as u32 >= pic.mb_height * 4 {
        return ([0, 0], -1);
    }
    let idx = gy as usize * pic.luma4_width() + gx as usize;
    (pic.mv[idx], pic.ref_idx[idx])
}

/// 🔍 The `C` (above-right) motion neighbor for a partition at 4×4 grid `(part_gx, part_gy)` sized `part_w4`
/// blocks wide, falling back to `D` (above-left) when `C` would fall in a not-yet-decoded macroblock later in
/// the current row (clause 6.4.11.7) — the one genuinely order-dependent case among A/B/C/D for the whole-
/// partition motion predictors this decoder supports (16×16/16×8/8×16/skip).
fn neighbor_c_mv_ref(pic: &Picture, mb_x: u32, mb_y: u32, part_gx: i32, part_gy: i32, part_w4: i32) -> ([i32; 2], i8) {
    let c_gx = part_gx + part_w4;
    let c_gy = part_gy - 1;
    let c_mb_row = if c_gy >= 0 { c_gy / 4 } else { -1 };
    let c_mb_col = if c_gx >= 0 { c_gx / 4 } else { -1 };
    if c_mb_row == mb_y as i32 && c_mb_col > mb_x as i32 {
        neighbor_mv_ref(pic, part_gx - 1, part_gy - 1)
    } else {
        neighbor_mv_ref(pic, c_gx, c_gy)
    }
}

/// 📥 Decodes one `P_Skip` macroblock (implied by `mb_skip_run`, clause 7.3.4 / 8.4.1.1): no bits consumed
/// beyond the skip run itself. Motion is `(0, 0)` when either macroblock neighbor is unavailable or either
/// neighbor's own motion is `(0, 0)` against reference `0`; otherwise the usual median predictor.
fn decode_p_skip_mb(pic: &mut Picture, mb_x: u32, mb_y: u32, ref0: &Picture, qp: i32) {
    let (gx0, gy0) = (mb_x as i32 * 4, mb_y as i32 * 4);
    let (mv_a, ref_a) = neighbor_mv_ref(pic, gx0 - 1, gy0);
    let (mv_b, ref_b) = neighbor_mv_ref(pic, gx0, gy0 - 1);
    let mv = if mb_x == 0 || mb_y == 0 || (ref_a == 0 && mv_a == [0, 0]) || (ref_b == 0 && mv_b == [0, 0]) {
        [0, 0]
    } else {
        let (mv_c, ref_c) = neighbor_c_mv_ref(pic, mb_x, mb_y, gx0, gy0, 4);
        median_mv_predict((mv_a, ref_a), (mv_b, ref_b), (mv_c, ref_c), 0)
    };
    let rp = RefPlanes { luma: &ref0.luma, luma_w: ref0.luma_width() as i32, luma_h: (ref0.mb_height * 16) as i32, chroma_w: ref0.chroma_width() as i32, chroma_h: (ref0.mb_height * 8) as i32 };
    let (px, py) = (mb_x as i32 * 16, mb_y as i32 * 16);
    let mut luma_out = vec![0i32; 256];
    mc_luma_block(&rp, px, py, 16, 16, mv, &mut luma_out);
    let lw = pic.luma_width();
    for r in 0..16usize {
        for c in 0..16usize {
            pic.luma[(py as usize + r) * lw + px as usize + c] = luma_out[r * 16 + c].clamp(0, 255) as u8;
        }
    }
    let (cpx, cpy) = (mb_x as i32 * 8, mb_y as i32 * 8);
    let mut cb_out = vec![0i32; 64];
    let mut cr_out = vec![0i32; 64];
    mc_chroma_block(&ref0.cb, rp.chroma_w, rp.chroma_h, cpx, cpy, 8, 8, mv, &mut cb_out);
    mc_chroma_block(&ref0.cr, rp.chroma_w, rp.chroma_h, cpx, cpy, 8, 8, mv, &mut cr_out);
    let cw = pic.chroma_width();
    for r in 0..8usize {
        for c in 0..8usize {
            pic.cb[(cpy as usize + r) * cw + cpx as usize + c] = cb_out[r * 8 + c].clamp(0, 255) as u8;
            pic.cr[(cpy as usize + r) * cw + cpx as usize + c] = cr_out[r * 8 + c].clamp(0, 255) as u8;
        }
    }
    let lw4 = pic.luma4_width();
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let idx = (mb_y * 4 + by) as usize * lw4 + (mb_x * 4 + bx) as usize;
        pic.decoded_luma4[idx] = true;
        pic.mv[idx] = mv;
        pic.ref_idx[idx] = 0;
        pic.nnz_luma[idx] = 0;
    }
    let cw4 = pic.chroma4_width();
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let idx = (mb_y * 2 + qy) as usize * cw4 + (mb_x * 2 + qx) as usize;
        pic.nnz_cb[idx] = 0;
        pic.nnz_cr[idx] = 0;
    }
    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = false;
    pic.mb_qp[mbidx] = qp;
}

/// 📥 Decodes one coded (non-skip) P macroblock: `P_L0_16x16`/`P_L0_L0_16x8`/`P_L0_L0_8x16` (`code_num` 0..2);
/// `code_num` 3/4 (`P_8x8`/`P_8x8ref0`, sub-8×8 partitions) are a deliberate scope cut of this decoder — see
/// crate-level notes — and fail loudly rather than misdecode.
#[allow(clippy::too_many_lines, reason = "the P-macroblock syntax genuinely has this many sequential steps (ref_idx*, mvd*, motion compensation, cbp, qp, luma residual, chroma residual) and splitting it up would just scatter tightly-coupled per-partition state across more function boundaries")]
#[allow(clippy::too_many_arguments, reason = "these are the macroblock's own coordinates, the always-needed parsed-header context (pps/header), the reference list, and the running QP predictor — the natural, self-describing parameter list for a single-slice macroblock decoder with no surrounding decoder-state struct")]
fn decode_p_mb(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, code_num: u32, pps: &PpsInfo, header: &SliceHeaderInfo, ref_list0: &[&Picture], prev_qp: &mut i32) -> Result<(), H264Error> {
    if code_num >= 3 {
        return Err(H264Error::Unsupported("P_8x8 sub-macroblock partitions"));
    }
    let parts: [(u32, u32, u32, u32); 2] = match code_num {
        0 => [(0, 0, 4, 4), (0, 0, 0, 0)],
        1 => [(0, 0, 4, 2), (0, 2, 4, 2)],
        _ => [(0, 0, 2, 4), (2, 0, 2, 4)],
    };
    let num_parts = if code_num == 0 { 1 } else { 2 };
    let max_ref = header.num_ref_idx_l0_active.saturating_sub(1);
    let mut ref_idxs = [0i8; 2];
    for slot in ref_idxs.iter_mut().take(num_parts) {
        *slot = read_te(b, max_ref)? as i8;
    }
    let mut mvs = [[0i32; 2]; 2];
    for i in 0..num_parts {
        let (pxo, pyo, pw, ph) = parts[i];
        let (gx0, gy0) = (mb_x as i32 * 4 + pxo as i32, mb_y as i32 * 4 + pyo as i32);
        let mvd = [b.se()?, b.se()?];
        let (mv_a, ref_a) = neighbor_mv_ref(pic, gx0 - 1, gy0);
        let (mv_b, ref_b) = neighbor_mv_ref(pic, gx0, gy0 - 1);
        let (mv_c, ref_c) = neighbor_c_mv_ref(pic, mb_x, mb_y, gx0, gy0, pw as i32);
        let cur_ref = ref_idxs[i];
        let predictor = if num_parts == 2 && ph == 2 && i == 0 && ref_b == cur_ref {
            mv_b
        } else if num_parts == 2 && ((ph == 2 && i == 1) || (pw == 2 && i == 0)) && ref_a == cur_ref {
            mv_a
        } else if num_parts == 2 && pw == 2 && i == 1 && ref_c == cur_ref {
            mv_c
        } else {
            median_mv_predict((mv_a, ref_a), (mv_b, ref_b), (mv_c, ref_c), cur_ref)
        };
        let mv = [predictor[0] + mvd[0], predictor[1] + mvd[1]];
        mvs[i] = mv;
        let lw4 = pic.luma4_width();
        for dy in 0..ph {
            for dx in 0..pw {
                let idx = (gy0 as u32 + dy) as usize * lw4 + (gx0 as u32 + dx) as usize;
                pic.mv[idx] = mv;
                pic.ref_idx[idx] = cur_ref;
            }
        }
    }

    let cbp_code = b.ue()?;
    let cbp = *GOLOMB_TO_INTER_CBP.get(cbp_code as usize).ok_or(H264Error::Malformed("coded_block_pattern code out of range"))?;
    let (cbp_luma, cbp_chroma) = (cbp & 0xF, cbp >> 4);
    let qp = if cbp != 0 {
        let delta = b.se()?;
        let qp = (*prev_qp + delta + 52) % 52;
        *prev_qp = qp;
        qp
    } else {
        *prev_qp
    };

    let lw = pic.luma_width();
    let cw = pic.chroma_width();
    for i in 0..num_parts {
        let (pxo, pyo, pw, ph) = parts[i];
        let ref_pic = *ref_list0.get(ref_idxs[i] as usize).ok_or(H264Error::Malformed("p macroblock ref_idx out of range"))?;
        let rp = RefPlanes { luma: &ref_pic.luma, luma_w: ref_pic.luma_width() as i32, luma_h: (ref_pic.mb_height * 16) as i32, chroma_w: ref_pic.chroma_width() as i32, chroma_h: (ref_pic.mb_height * 8) as i32 };
        let (ppx, ppy) = ((mb_x * 16 + pxo * 4) as i32, (mb_y * 16 + pyo * 4) as i32);
        let (pw_px, ph_px) = ((pw * 4) as usize, (ph * 4) as usize);
        let mut out = vec![0i32; pw_px * ph_px];
        mc_luma_block(&rp, ppx, ppy, pw_px, ph_px, mvs[i], &mut out);
        for r in 0..ph_px {
            for c in 0..pw_px {
                pic.luma[(ppy as usize + r) * lw + ppx as usize + c] = out[r * pw_px + c].clamp(0, 255) as u8;
            }
        }
        let (cpx, cpy) = ((mb_x * 8 + pxo * 2) as i32, (mb_y * 8 + pyo * 2) as i32);
        let (cw_px, ch_px) = ((pw * 2) as usize, (ph * 2) as usize);
        let mut cb_out = vec![0i32; cw_px * ch_px];
        let mut cr_out = vec![0i32; cw_px * ch_px];
        mc_chroma_block(&ref_pic.cb, rp.chroma_w, rp.chroma_h, cpx, cpy, cw_px, ch_px, mvs[i], &mut cb_out);
        mc_chroma_block(&ref_pic.cr, rp.chroma_w, rp.chroma_h, cpx, cpy, cw_px, ch_px, mvs[i], &mut cr_out);
        for r in 0..ch_px {
            for c in 0..cw_px {
                pic.cb[(cpy as usize + r) * cw + cpx as usize + c] = cb_out[r * cw_px + c].clamp(0, 255) as u8;
                pic.cr[(cpy as usize + r) * cw + cpx as usize + c] = cr_out[r * cw_px + c].clamp(0, 255) as u8;
            }
        }
    }

    let lw4 = pic.luma4_width();
    for n in 0..16usize {
        let (bx, by) = block4x4_grid_pos(n);
        let (gx, gy) = (mb_x * 4 + bx, mb_y * 4 + by);
        let idx = gy as usize * lw4 + gx as usize;
        pic.decoded_luma4[idx] = true;
        if cbp_luma & (1 << (n / 4)) != 0 {
            let nc = predict_nc(luma_nc(pic, gx as i32 - 1, gy as i32), luma_nc(pic, gx as i32, gy as i32 - 1));
            let block = read_residual_block(b, NcSelector::Nc(nc), &ZIGZAG_4X4)?;
            let residual = idct4x4(&dequant4x4(&block.coeffs, qp));
            let mut px_block = [0i32; 16];
            for r in 0..4 {
                for c in 0..4 {
                    px_block[r * 4 + c] = i32::from(pic.luma[(gy as usize * 4 + r) * lw + gx as usize * 4 + c]) + residual[r * 4 + c];
                }
            }
            pic.write_luma4(gx, gy, &px_block);
            pic.nnz_luma[idx] = block.total_coeff;
        } else {
            pic.nnz_luma[idx] = 0;
        }
    }

    let qp_cb = chroma_qp(qp, pps.chroma_qp_index_offset);
    let qp_cr = chroma_qp(qp, pps.chroma_qp_index_offset);
    let (cb_res, cr_res) = read_chroma_residual(b, pic, mb_x, mb_y, cbp_chroma, qp_cb, qp_cr)?;
    for n in 0..4usize {
        let (qx, qy) = chroma_quad_pos(n);
        let (px, py) = (mb_x as usize * 8 + qx as usize * 4, mb_y as usize * 8 + qy as usize * 4);
        let mut block = [0i32; 16];
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = i32::from(pic.cb[(py + r) * cw + px + c]) + cb_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cb, cw, px, py, &block);
        for r in 0..4 {
            for c in 0..4 {
                block[r * 4 + c] = i32::from(pic.cr[(py + r) * cw + px + c]) + cr_res[n][r * 4 + c];
            }
        }
        Picture::write_chroma4(&mut pic.cr, cw, px, py, &block);
    }

    let mbidx = pic.mb_index(mb_x, mb_y);
    pic.mb_is_intra[mbidx] = false;
    pic.mb_qp[mbidx] = qp;
    Ok(())
}

/// 🔀 Dispatches one coded macroblock by `mb_type`: P slices see `0..5` as P partition types and `5..` as the
/// I-macroblock table shifted by 5; I slices see the I-macroblock table directly.
#[allow(clippy::too_many_arguments, reason = "same rationale as decode_p_mb, which this just dispatches to alongside the intra macroblock decoders")]
fn decode_macroblock(b: &mut BitReader<'_>, pic: &mut Picture, mb_x: u32, mb_y: u32, pps: &PpsInfo, header: &SliceHeaderInfo, is_p_slice: bool, ref_list0: &[&Picture], prev_qp: &mut i32) -> Result<(), H264Error> {
    let mb_type = b.ue()?;
    let i_type = if is_p_slice {
        if mb_type < 5 {
            return decode_p_mb(b, pic, mb_x, mb_y, mb_type, pps, header, ref_list0, prev_qp);
        }
        mb_type - 5
    } else {
        mb_type
    };
    match i_type {
        0 => decode_intra4x4_mb(b, pic, mb_x, mb_y, pps, prev_qp),
        1..=24 => decode_intra16x16_mb(b, pic, mb_x, mb_y, i_type - 1, pps, prev_qp),
        25 => decode_pcm_mb(b, pic, mb_x, mb_y),
        _ => Err(H264Error::Malformed("mb_type out of range")),
    }
}

/// 📥 Decodes `slice_data()` (clause 7.3.4): the `mb_skip_run`-driven loop for P slices, dispatching each
/// coded macroblock via [`decode_macroblock`]. Only a single slice per picture is supported (this decoder
/// requires `first_mb_in_slice == 0` and consumes macroblocks until the whole picture is covered).
fn decode_slice_data(b: &mut BitReader<'_>, pic: &mut Picture, sps: &SpsInfo, pps: &PpsInfo, header: &SliceHeaderInfo, ref_list0: &[&Picture]) -> Result<(), H264Error> {
    let total_mbs = sps.pic_width_in_mbs * sps.pic_height_in_mbs;
    let is_p_slice = header.slice_type_mod5 == 0;
    let mut mb_addr = header.first_mb_in_slice;
    let mut prev_qp = header.slice_qp;
    loop {
        if mb_addr >= total_mbs {
            break;
        }
        if is_p_slice {
            let skip_run = b.ue()?;
            for _ in 0..skip_run {
                if mb_addr >= total_mbs {
                    return Err(H264Error::Malformed("mb_skip_run exceeds picture"));
                }
                let ref0 = *ref_list0.first().ok_or(H264Error::Malformed("p slice has no reference picture"))?;
                decode_p_skip_mb(pic, mb_addr % sps.pic_width_in_mbs, mb_addr / sps.pic_width_in_mbs, ref0, prev_qp);
                mb_addr += 1;
            }
            if mb_addr >= total_mbs {
                break;
            }
            if !b.more_rbsp_data() {
                break;
            }
        }
        decode_macroblock(b, pic, mb_addr % sps.pic_width_in_mbs, mb_addr / sps.pic_width_in_mbs, pps, header, is_p_slice, ref_list0, &mut prev_qp)?;
        mb_addr += 1;
        if !b.more_rbsp_data() {
            break;
        }
    }
    if mb_addr != total_mbs {
        return Err(H264Error::Malformed("slice did not cover the whole picture"));
    }
    Ok(())
}
// #endregion 🔖SliceData

// #region 🔖Decoder
/// 🎬 Baseline-profile (Constrained Baseline / Baseline, `profile_idc == 66`) H.264 decoder. Decodes AVCC
/// length-prefixed access units one at a time via [`H264Decoder::decode_sample`], maintaining its own
/// decoded-picture buffer across calls. See the crate-level docs for the full list of spec features this
/// decoder deliberately does not implement — each fails loudly with [`H264Error::Unsupported`] rather than
/// silently misdecoding.
pub struct H264Decoder {
    sps: SpsInfo,
    pps: PpsInfo,
    nal_length_size: u8,
    dpb: Dpb,
    prev_poc_msb: i32,
    prev_poc_lsb: i32,
}

impl H264Decoder {
    /// 🏗️ Parses SPS/PPS from `sps_pps_nals` — a flat sequence of `(u16 big-endian length, NAL bytes)`
    /// entries (exactly [`crate::probe_mp4`]'s `avcC` extraction format; classified by NAL type, order-
    /// independent). Rejects any non-baseline SPS/PPS feature immediately (see crate docs) rather than
    /// deferring the failure to the first `decode_sample` call.
    pub fn new(sps_pps_nals: &[u8]) -> Result<Self, H264Error> {
        let mut sps: Option<SpsInfo> = None;
        let mut pps: Option<PpsInfo> = None;
        let mut pos = 0usize;
        while pos < sps_pps_nals.len() {
            let len_bytes = sps_pps_nals.get(pos..pos + 2).ok_or(H264Error::Truncated)?;
            let len = usize::from(u16::from_be_bytes([len_bytes[0], len_bytes[1]]));
            pos += 2;
            let nal_bytes = sps_pps_nals.get(pos..pos + len).ok_or(H264Error::Truncated)?;
            pos += len;
            let nal = parse_nal(nal_bytes)?;
            match nal.nal_unit_type {
                7 => sps = Some(parse_sps(&nal.rbsp)?),
                8 => pps = Some(parse_pps(&nal.rbsp)?),
                _ => {}
            }
        }
        let sps = sps.ok_or(H264Error::NoSps)?;
        let pps = pps.ok_or(H264Error::NoPps)?;
        let max_refs = sps.max_num_ref_frames;
        Ok(Self { sps, pps, nal_length_size: 4, dpb: Dpb::new(max_refs), prev_poc_msb: 0, prev_poc_lsb: 0 })
    }

    /// 🔧 Overrides the AVCC NAL length-prefix size (default `4`, the overwhelmingly common `avcC` value);
    /// callers that extracted `nal_length_size` from a real `avcC` box should pass it through here.
    pub fn with_nal_length_size(mut self, size: u8) -> Self {
        self.nal_length_size = size;
        self
    }

    /// 🧮 `PicOrderCnt` (clauses 8.2.1.1/8.2.1.3): full derivation for `pic_order_cnt_type == 0`, and the
    /// `2 * frame_num` approximation for `type == 2` (exact for the non-wrapping, all-reference-frame,
    /// P/I-only streams this decoder ever produces or consumes; ordering only ever matters for output — this
    /// decoder never reorders, so precision here is informational, not load-bearing).
    fn compute_poc(&mut self, header: &SliceHeaderInfo, is_idr: bool) -> i32 {
        match self.sps.pic_order_cnt_type {
            0 => {
                let max_lsb = 1i32 << self.sps.log2_max_pic_order_cnt_lsb;
                let lsb = header.pic_order_cnt_lsb as i32;
                let msb = if is_idr {
                    0
                } else if lsb < self.prev_poc_lsb && (self.prev_poc_lsb - lsb) >= max_lsb / 2 {
                    self.prev_poc_msb + max_lsb
                } else if lsb > self.prev_poc_lsb && (lsb - self.prev_poc_lsb) > max_lsb / 2 {
                    self.prev_poc_msb - max_lsb
                } else {
                    self.prev_poc_msb
                };
                self.prev_poc_msb = msb;
                self.prev_poc_lsb = lsb;
                msb + lsb
            }
            _ => 2 * header.frame_num as i32,
        }
    }

    /// 📥 Decodes one AVCC-length-prefixed access unit (exactly one slice NAL, optionally alongside SEI/AUD
    /// NALs it skips) into a full RGBA frame. `Ok(None)` never occurs for this decoder's supported baseline
    /// P/I-only content — output is always immediate, never buffered pending reordering.
    pub fn decode_sample(&mut self, nal_bytes: &[u8]) -> Result<Option<remodel_image::ImageRgba8>, H264Error> {
        let nals = split_avcc_nals(nal_bytes, self.nal_length_size)?;
        let mut slice_nal: Option<NalUnit> = None;
        for raw in nals {
            let nal = parse_nal(raw)?;
            match nal.nal_unit_type {
                1 | 5 => {
                    if slice_nal.is_some() {
                        return Err(H264Error::Unsupported("multiple slices per access unit"));
                    }
                    slice_nal = Some(nal);
                }
                7 => self.sps = parse_sps(&nal.rbsp)?,
                8 => self.pps = parse_pps(&nal.rbsp)?,
                _ => {}
            }
        }
        let nal = slice_nal.ok_or(H264Error::Malformed("access unit has no slice nal"))?;
        let mut bits = BitReader::new(&nal.rbsp);
        let header = parse_slice_header(&mut bits, nal.nal_unit_type, &self.sps, &self.pps)?;
        if header.first_mb_in_slice != 0 {
            return Err(H264Error::Unsupported("multiple slices per picture"));
        }
        let is_idr = nal.nal_unit_type == 5;
        if is_idr {
            self.dpb.clear();
        }

        let mut pic = Picture::new(self.sps.pic_width_in_mbs, self.sps.pic_height_in_mbs);
        pic.frame_num = header.frame_num;
        pic.poc = self.compute_poc(&header, is_idr);

        let ref_list0 = self.dpb.ref_list0();
        decode_slice_data(&mut bits, &mut pic, &self.sps, &self.pps, &header, &ref_list0)?;
        drop(ref_list0);

        deblock_picture(&mut pic, &self.sps, &header);

        let image = pic.crop_to(self.sps.width_px, self.sps.height_px);
        self.dpb.push(pic);
        Ok(Some(image))
    }
}
// #endregion 🔖Decoder

// #region 🔖H264Enc
/// ✍️ MSB-first bit writer for hand-assembling spec-legal RBSPs (mirrors [`BitReader`]).
#[derive(Default)]
struct BitWriter {
    bytes: Vec<u8>,
    acc: u32,
    nbits: u32,
}

impl BitWriter {
    fn put_bit(&mut self, bit: u32) {
        self.acc = (self.acc << 1) | (bit & 1);
        self.nbits += 1;
        if self.nbits == 8 {
            self.bytes.push(self.acc as u8);
            self.acc = 0;
            self.nbits = 0;
        }
    }

    fn put_u(&mut self, v: u32, n: u8) {
        for i in (0..n).rev() {
            self.put_bit((v >> i) & 1);
        }
    }

    fn put_ue(&mut self, v: u32) {
        let x = v + 1;
        let nbits = 32 - x.leading_zeros();
        for _ in 0..nbits - 1 {
            self.put_bit(0);
        }
        for i in (0..nbits).rev() {
            self.put_bit((x >> i) & 1);
        }
    }

    fn put_se(&mut self, v: i32) {
        let code = if v <= 0 { (-v) as u32 * 2 } else { (v as u32) * 2 - 1 };
        self.put_ue(code);
    }

    /// 🏁 `rbsp_trailing_bits()`: a `1` stop bit, then zero-padded to a byte boundary.
    fn rbsp_trailing(&mut self) {
        self.put_bit(1);
        while self.nbits != 0 {
            self.put_bit(0);
        }
    }

    /// 🏁 Mid-stream `pcm_alignment_zero_bit` padding: zero bits up to the next byte boundary, writing
    /// nothing at all when already aligned (unlike [`Self::rbsp_trailing`], which always emits a stop bit).
    fn zero_align(&mut self) {
        while self.nbits != 0 {
            self.put_bit(0);
        }
    }
}

/// 🧹 Inverse of [`strip_emulation_prevention`]: inserts `emulation_prevention_three_byte` (`00 00 0x` →
/// `00 00 03 0x` for `x <= 3`) so the RBSP round-trips through a real NAL byte stream.
fn add_emulation_prevention(rbsp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(rbsp.len());
    let mut zero_run = 0u32;
    for &byte in rbsp {
        if zero_run >= 2 && byte <= 3 {
            out.push(3);
            zero_run = 0;
        }
        out.push(byte);
        zero_run = if byte == 0 { zero_run + 1 } else { 0 };
    }
    out
}

fn write_nal(nal_ref_idc: u8, nal_unit_type: u8, rbsp: &[u8]) -> Vec<u8> {
    let mut out = vec![(nal_ref_idc << 5) | nal_unit_type];
    out.extend(add_emulation_prevention(rbsp));
    out
}

/// ✍️ Length-prefixes `nal` with a 4-byte big-endian length, matching this crate's own AVCC framing
/// (`H264Decoder`'s default `nal_length_size`).
fn avcc_frame(nal: &[u8]) -> Vec<u8> {
    let mut out = (nal.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(nal);
    out
}

/// 🏗️ Builds a minimal baseline SPS+PPS pair for an `mb_w*16 × mb_h*16` picture: `pic_order_cnt_type = 2`
/// (no extra POC fields), a single reference frame, deblocking always disabled (`disable_deblocking_filter_idc
/// = 1` is written per-slice, but PPS still declares control present so the slice header carries it).
fn h264_enc_sps_pps_nals(mb_w: u32, mb_h: u32) -> (Vec<u8>, Vec<u8>) {
    let mut sps = BitWriter::default();
    sps.put_u(66, 8);
    sps.put_u(0, 8);
    sps.put_u(30, 8);
    sps.put_ue(0);
    sps.put_ue(4);
    sps.put_ue(2);
    sps.put_ue(15);
    sps.put_u(0, 1);
    sps.put_ue(mb_w - 1);
    sps.put_ue(mb_h - 1);
    sps.put_u(1, 1);
    sps.put_u(1, 1);
    sps.put_u(0, 1);
    sps.put_u(0, 1);
    sps.rbsp_trailing();

    let mut pps = BitWriter::default();
    pps.put_ue(0);
    pps.put_ue(0);
    pps.put_u(0, 1);
    pps.put_u(0, 1);
    pps.put_ue(0);
    pps.put_ue(0);
    pps.put_ue(0);
    pps.put_u(0, 1);
    pps.put_u(0, 2);
    pps.put_se(0);
    pps.put_se(0);
    pps.put_se(0);
    pps.put_u(1, 1);
    pps.put_u(0, 1);
    pps.put_u(0, 1);
    pps.rbsp_trailing();

    (write_nal(3, 7, &sps.bytes), write_nal(3, 8, &pps.bytes))
}

/// 🏗️ SPS+PPS in [`H264Decoder::new`]'s expected `(u16 length, NAL)*` format, for an `mb_w*16 × mb_h*16`
/// picture — pass the result straight to `H264Decoder::new`.
pub fn h264_enc_sps_pps(mb_w: u32, mb_h: u32) -> Vec<u8> {
    let (sps, pps) = h264_enc_sps_pps_nals(mb_w, mb_h);
    let mut out = Vec::new();
    for nal in [&sps, &pps] {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    out
}

/// 🏗️ Encodes one I_PCM-only IDR frame (`mb_w*16 × mb_h*16`, every macroblock `I_PCM`): a bit-exact,
/// entropy-coding-free round trip by construction — this crate's contract test for the whole NAL/slice-
/// header/SPS-PPS/PCM-macroblock plumbing. `luma`/`cb`/`cr` must be exactly `mb_w*16*mb_h*16`/`mb_w*8*mb_h*8`
/// samples (row-major); deblocking is disabled so the decoder returns these samples unchanged.
pub fn h264_enc_i_pcm_sample(mb_w: u32, mb_h: u32, frame_num: u32, luma: &[u8], cb: &[u8], cr: &[u8]) -> Vec<u8> {
    let mut s = BitWriter::default();
    s.put_ue(0);
    s.put_ue(7);
    s.put_ue(0);
    s.put_u(frame_num, 8);
    s.put_ue(0);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(1);

    for n in 0..(mb_w * mb_h) {
        s.put_ue(25);
        s.zero_align();
        let (mb_x, mb_y) = (n % mb_w, n / mb_w);
        let lw = (mb_w * 16) as usize;
        for r in 0..16usize {
            for c in 0..16usize {
                s.put_u(u32::from(luma[(mb_y as usize * 16 + r) * lw + mb_x as usize * 16 + c]), 8);
            }
        }
        let cw = (mb_w * 8) as usize;
        for r in 0..8usize {
            for c in 0..8usize {
                s.put_u(u32::from(cb[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
            }
        }
        for r in 0..8usize {
            for c in 0..8usize {
                s.put_u(u32::from(cr[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
            }
        }
    }
    s.rbsp_trailing();
    avcc_frame(&write_nal(3, 5, &s.bytes))
}

/// 🏗️ Encodes one whole-frame `P_Skip` slice (non-IDR, references `frame_num - 1`): every macroblock skipped,
/// no residual/MV entropy coding at all — exercises the DPB reference lookup and the `mb_skip_run` slice-data
/// path without needing CAVLC-coded content.
pub fn h264_enc_p_skip_sample(mb_w: u32, mb_h: u32, frame_num: u32) -> Vec<u8> {
    let mut s = BitWriter::default();
    s.put_ue(0);
    s.put_ue(5);
    s.put_ue(0);
    s.put_u(frame_num, 8);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_u(0, 1);
    s.put_se(0);
    s.put_ue(1);
    s.put_ue(mb_w * mb_h);
    s.rbsp_trailing();
    avcc_frame(&write_nal(2, 1, &s.bytes))
}
// #endregion 🔖H264Enc

// #region 🔖Mux
/// 📦 Wraps `payload` in a length-prefixed ISO-BMFF box.
fn mp4_box(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len());
    out.extend_from_slice(&((payload.len() + 8) as u32).to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(payload);
    out
}

/// 📦 `VisualSampleEntry` for `codec_fourcc` (`mjpg` or `avc1`); `extra` holds codec-specific child boxes
/// (e.g. `avcC`), empty for MJPEG.
fn mp4_visual_sample_entry(codec_fourcc: &[u8; 4], width: u16, height: u16, extra: &[u8]) -> Vec<u8> {
    let mut payload = vec![0u8; 8];
    payload.extend_from_slice(&[0, 0]);
    payload.extend_from_slice(&[0, 0]);
    payload.extend_from_slice(&[0u8; 12]);
    payload.extend_from_slice(&width.to_be_bytes());
    payload.extend_from_slice(&height.to_be_bytes());
    payload.extend_from_slice(&0x0048_0000u32.to_be_bytes());
    payload.extend_from_slice(&0x0048_0000u32.to_be_bytes());
    payload.extend_from_slice(&[0u8; 4]);
    payload.extend_from_slice(&[0, 1]);
    payload.extend_from_slice(&[0u8; 32]);
    payload.extend_from_slice(&[0, 0x18]);
    payload.extend_from_slice(&[0xFF, 0xFF]);
    payload.extend_from_slice(extra);
    mp4_box(codec_fourcc, &payload)
}

fn mp4_stsd(codec_fourcc: &[u8; 4], width: u16, height: u16, extra: &[u8]) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend(mp4_visual_sample_entry(codec_fourcc, width, height, extra));
    mp4_box(b"stsd", &payload)
}

fn mp4_stts(frame_count: u32, delta: u32) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend_from_slice(&frame_count.to_be_bytes());
    payload.extend_from_slice(&delta.to_be_bytes());
    mp4_box(b"stts", &payload)
}

fn mp4_stsc(frame_count: u32) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend_from_slice(&frame_count.to_be_bytes());
    payload.extend_from_slice(&1u32.to_be_bytes());
    mp4_box(b"stsc", &payload)
}

fn mp4_stsz(sizes: &[u32]) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&0u32.to_be_bytes());
    payload.extend_from_slice(&(sizes.len() as u32).to_be_bytes());
    for &s in sizes {
        payload.extend_from_slice(&s.to_be_bytes());
    }
    mp4_box(b"stsz", &payload)
}

fn mp4_stco(offset: u32) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&1u32.to_be_bytes());
    payload.extend_from_slice(&offset.to_be_bytes());
    mp4_box(b"stco", &payload)
}

fn mp4_mdhd(timescale: u32, duration: u32) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    payload.extend_from_slice(&[0u8; 8]);
    payload.extend_from_slice(&timescale.to_be_bytes());
    payload.extend_from_slice(&duration.to_be_bytes());
    payload.extend_from_slice(&[0x55, 0xC4]);
    payload.extend_from_slice(&[0u8; 2]);
    mp4_box(b"mdhd", &payload)
}

fn mp4_hdlr() -> Vec<u8> {
    let mut payload = vec![0u8; 8];
    payload.extend_from_slice(b"vide");
    payload.extend_from_slice(&[0u8; 12]);
    payload.push(0);
    mp4_box(b"hdlr", &payload)
}

/// 🏗️ Assembles the `moov` box for a single video track with `sizes.len()` samples of `codec_fourcc`, all at
/// a single chunk starting at `mdat_offset` (the absolute file offset of the first sample's bytes).
#[allow(clippy::too_many_arguments, reason = "this is a flat list of independent box fields for a fixture-synthesis-only muxer; a wrapper struct would only rename them once, at this single call site")]
fn mp4_build_moov(codec_fourcc: &[u8; 4], width: u32, height: u32, sizes: &[u32], timescale: u32, delta: u32, extra: &[u8], mdat_offset: u32) -> Vec<u8> {
    let duration = delta * sizes.len() as u32;
    let stbl = [mp4_stsd(codec_fourcc, width as u16, height as u16, extra), mp4_stts(sizes.len() as u32, delta), mp4_stsc(sizes.len() as u32), mp4_stsz(sizes), mp4_stco(mdat_offset)].concat();
    let minf = mp4_box(b"stbl", &stbl);
    let mdia = [mp4_mdhd(timescale, duration), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
    let trak = mp4_box(b"mdia", &mdia);
    let moov = mp4_box(b"trak", &trak);
    mp4_box(b"moov", &moov)
}

/// 🏗️ Builds an MP4 with `ftyp`/`moov`/`mdat`, `timescale = 1000` (milliseconds); `probe_mp4` recovers exact
/// frame count/timestamps/sync flags/dimensions from it.
fn mp4_mux(codec_fourcc: &[u8; 4], width: u32, height: u32, samples: &[Vec<u8>], fps: f64, extra: &[u8]) -> Vec<u8> {
    let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isomiso2avc1mp41");
    let sizes: Vec<u32> = samples.iter().map(|s| s.len() as u32).collect();
    let delta = if fps > 0.0 { (1000.0 / fps).round() as u32 } else { 1000 }.max(1);
    let placeholder = mp4_build_moov(codec_fourcc, width, height, &sizes, 1000, delta, extra, 0);
    let mdat_offset = (ftyp.len() + placeholder.len() + 8) as u32;
    let moov = mp4_build_moov(codec_fourcc, width, height, &sizes, 1000, delta, extra, mdat_offset);
    let mdat = mp4_box(b"mdat", &samples.concat());
    [ftyp, moov, mdat].concat()
}

/// ✍️ Muxes pre-encoded JPEG frames into a minimal `mjpg`-codec MP4, fixture-synthesis only (no `avcC`,
/// timescale fixed at milliseconds). Dimensions come from decoding `frames[0]`.
pub fn write_mp4_mjpeg(frames: &[Vec<u8>], fps: f64) -> Vec<u8> {
    let (w, h) = frames.first().and_then(|f| remodel_image::decode_jpeg(f).ok()).map_or((0, 0), |img| (img.width, img.height));
    mp4_mux(b"mjpg", w, h, frames, fps, &[])
}

/// ✍️ Builds an `avcC` (AVCDecoderConfigurationRecord) box from this crate's internal `(u16 length, NAL)*`
/// SPS/PPS format (as produced by [`h264_enc_sps_pps`] or extracted by `probe_mp4`).
fn build_avcc(sps_pps_nals: &[u8]) -> Vec<u8> {
    let mut sps_list = Vec::new();
    let mut pps_list = Vec::new();
    let mut pos = 0usize;
    while pos + 2 <= sps_pps_nals.len() {
        let len = usize::from(u16::from_be_bytes([sps_pps_nals[pos], sps_pps_nals[pos + 1]]));
        pos += 2;
        if pos + len > sps_pps_nals.len() {
            break;
        }
        let nal = &sps_pps_nals[pos..pos + len];
        pos += len;
        if nal.first().is_some_and(|&h| h & 0x1F == 7) {
            sps_list.push(nal);
        } else if nal.first().is_some_and(|&h| h & 0x1F == 8) {
            pps_list.push(nal);
        }
    }
    let mut out = vec![1, 66, 0, 30, 0xFF, 0xE0 | sps_list.len() as u8];
    for nal in &sps_list {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    out.push(pps_list.len() as u8);
    for nal in &pps_list {
        out.extend_from_slice(&(nal.len() as u16).to_be_bytes());
        out.extend_from_slice(nal);
    }
    mp4_box(b"avcC", &out)
}

/// 📐 Recovers `(width, height)` by locating and parsing the SPS NAL inside this crate's internal
/// `(u16 length, NAL)*` SPS/PPS format; `(0, 0)` if no valid SPS is present.
fn sps_pps_dimensions(sps_pps_nals: &[u8]) -> (u32, u32) {
    let mut pos = 0usize;
    while pos + 2 <= sps_pps_nals.len() {
        let len = usize::from(u16::from_be_bytes([sps_pps_nals[pos], sps_pps_nals[pos + 1]]));
        pos += 2;
        let Some(nal_bytes) = sps_pps_nals.get(pos..pos + len) else { break };
        pos += len;
        if let Ok(nal) = parse_nal(nal_bytes) {
            if nal.nal_unit_type == 7 {
                if let Ok(sps) = parse_sps(&nal.rbsp) {
                    return (sps.width_px, sps.height_px);
                }
            }
        }
    }
    (0, 0)
}

/// ✍️ Muxes AVCC-length-prefixed H.264 access units (as produced by [`h264_enc_i_pcm_sample`] /
/// [`h264_enc_p_skip_sample`]) into a minimal `avc1`-codec MP4, fixture-synthesis only. Dimensions are
/// recovered from `sps_pps` itself (as [`h264_enc_sps_pps`] produces).
pub fn write_mp4_avc(nal_samples: &[Vec<u8>], sps_pps: &[u8], fps: f64) -> Vec<u8> {
    let (width, height) = sps_pps_dimensions(sps_pps);
    mp4_mux(b"avc1", width, height, nal_samples, fps, &build_avcc(sps_pps))
}

/// 📦 Wraps `payload` in a RIFF chunk (little-endian size, even-padded).
fn riff_chunk(id: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len() + 1);
    out.extend_from_slice(id);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0);
    }
    out
}

fn riff_list(list_type: &[u8; 4], children: &[u8]) -> Vec<u8> {
    let mut payload = list_type.to_vec();
    payload.extend_from_slice(children);
    riff_chunk(b"LIST", &payload)
}

/// ✍️ Muxes pre-encoded JPEG frames into a minimal MJPG-codec AVI (`avih`/`strh`/`strf`/`movi`/`idx1`),
/// fixture-synthesis only. Dimensions come from decoding `frames[0]`.
pub fn write_avi_mjpg(frames: &[Vec<u8>], fps: f64) -> Vec<u8> {
    let (w, h) = frames.first().and_then(|f| remodel_image::decode_jpeg(f).ok()).map_or((0, 0), |img| (img.width, img.height));
    let micro_sec_per_frame = if fps > 0.0 { (1_000_000.0 / fps).round() as u32 } else { 1_000_000 };

    let mut avih = Vec::new();
    avih.extend_from_slice(&micro_sec_per_frame.to_le_bytes());
    avih.extend_from_slice(&0u32.to_le_bytes());
    avih.extend_from_slice(&0u32.to_le_bytes());
    avih.extend_from_slice(&0x10u32.to_le_bytes());
    avih.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    avih.extend_from_slice(&0u32.to_le_bytes());
    avih.extend_from_slice(&1u32.to_le_bytes());
    avih.extend_from_slice(&0u32.to_le_bytes());
    avih.extend_from_slice(&w.to_le_bytes());
    avih.extend_from_slice(&h.to_le_bytes());
    avih.extend_from_slice(&[0u8; 16]);

    let mut strh = Vec::new();
    strh.extend_from_slice(b"vids");
    strh.extend_from_slice(b"MJPG");
    strh.extend_from_slice(&0u32.to_le_bytes());
    strh.extend_from_slice(&[0u8; 4]);
    strh.extend_from_slice(&0u32.to_le_bytes());
    strh.extend_from_slice(&1000u32.to_le_bytes());
    strh.extend_from_slice(&(if fps > 0.0 { (fps * 1000.0).round() as u32 } else { 1000 }).to_le_bytes());
    strh.extend_from_slice(&0u32.to_le_bytes());
    strh.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    strh.extend_from_slice(&[0u8; 16]);

    let mut strf = Vec::new();
    strf.extend_from_slice(&40u32.to_le_bytes());
    strf.extend_from_slice(&(w as i32).to_le_bytes());
    strf.extend_from_slice(&(h as i32).to_le_bytes());
    strf.extend_from_slice(&1u16.to_le_bytes());
    strf.extend_from_slice(&24u16.to_le_bytes());
    strf.extend_from_slice(b"MJPG");
    strf.extend_from_slice(&[0u8; 20]);

    let strl = [riff_chunk(b"strh", &strh), riff_chunk(b"strf", &strf)].concat();
    let hdrl = [riff_chunk(b"avih", &avih), riff_list(b"strl", &strl)].concat();

    let movi_chunks: Vec<u8> = frames.iter().flat_map(|f| riff_chunk(b"00dc", f)).collect();
    let movi = riff_list(b"movi", &movi_chunks);

    let mut idx1 = Vec::new();
    let mut offset = 0u32;
    for f in frames {
        idx1.extend_from_slice(b"00dc");
        idx1.extend_from_slice(&0x10u32.to_le_bytes());
        idx1.extend_from_slice(&offset.to_le_bytes());
        idx1.extend_from_slice(&(f.len() as u32).to_le_bytes());
        offset += 8 + f.len() as u32 + (f.len() as u32 % 2);
    }

    let body = [riff_list(b"hdrl", &hdrl), movi, riff_chunk(b"idx1", &idx1)].concat();
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(4 + body.len() as u32).to_le_bytes());
    out.extend_from_slice(b"AVI ");
    out.extend_from_slice(&body);
    out
}
// #endregion 🔖Mux

// #region 🔖Extract
/// 🎚️ Frame-sampling knobs for [`extract_frames`]: `stride` keeps every `stride`-th sample (`0` treated as
/// `1`), `max_frames` caps the total sampled count (`0` means unbounded), `max_long_edge_px` downscales any
/// frame whose longer side exceeds it (`0` means no downscaling).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoIngestOptions {
    pub stride: u32,
    pub max_frames: u32,
    pub max_long_edge_px: u32,
}

/// 🔍 A probed container's video-track metadata, still tagged by container family.
#[derive(Clone, Debug, PartialEq)]
pub enum VideoProbe {
    Mp4(Mp4Info),
    Avi(AviInfo),
}

/// 🔍 Sniffs `bytes` as ISO-BMFF/MP4 (`ftyp` box) or RIFF/AVI (`RIFF` magic) and probes accordingly. Succeeds
/// for any well-formed container regardless of codec — [`extract_frames`] is what may reject an undecodable
/// codec.
pub fn probe(bytes: &[u8]) -> Result<VideoProbe, VideoError> {
    if bytes.len() >= 4 && &bytes[0..4] == b"RIFF" {
        Ok(VideoProbe::Avi(probe_avi(bytes)?))
    } else {
        Ok(VideoProbe::Mp4(probe_mp4(bytes)?))
    }
}

/// 🏷️ A human/log-facing fourcc for a codec, for [`VideoError::UnsupportedCodec`] diagnostics.
fn codec_fourcc_hint(codec: VideoCodec) -> FourCc {
    match codec {
        VideoCodec::Avc => FourCc(*b"avc1"),
        VideoCodec::Hevc => FourCc(*b"hvc1"),
        VideoCodec::Vp9 => FourCc(*b"vp09"),
        VideoCodec::Av1 => FourCc(*b"av01"),
        VideoCodec::Mjpeg => FourCc(*b"mjpg"),
        VideoCodec::Unknown(fourcc) => fourcc,
    }
}

/// 🖼️ One sampled, fully decoded frame: its sample index (into the container's original sample table,
/// *before* stride/max_frames selection), true media timestamp, and image.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtractedFrame {
    pub index: u32,
    pub timestamp_ms: f64,
    pub image: remodel_image::ImageRgba8,
}

/// 🔻 Box-filtered (bilinear-sampled) downscale so the longer side is at most `max_long_edge_px`; a no-operation
/// when already within budget or `max_long_edge_px == 0`. Built on [`remodel_image::ImageRgba8::sample_rgb`]
/// rather than a new resize primitive in `remodel_image` itself.
fn resize_to_max_long_edge(img: remodel_image::ImageRgba8, max_long_edge_px: u32) -> remodel_image::ImageRgba8 {
    if max_long_edge_px == 0 || img.width == 0 || img.height == 0 {
        return img;
    }
    let long_edge = img.width.max(img.height);
    if long_edge <= max_long_edge_px {
        return img;
    }
    let scale = f64::from(max_long_edge_px) / f64::from(long_edge);
    let new_w = ((f64::from(img.width) * scale).round() as u32).max(1);
    let new_h = ((f64::from(img.height) * scale).round() as u32).max(1);
    let mut out = remodel_image::ImageRgba8::new(new_w, new_h);
    for y in 0..new_h {
        for x in 0..new_w {
            let sx = ((f64::from(x) + 0.5) / scale - 0.5).max(0.0) as f32;
            let sy = ((f64::from(y) + 0.5) / scale - 0.5).max(0.0) as f32;
            let [r, g, b] = img.sample_rgb(sx, sy);
            let idx = ((y * new_w + x) * 4) as usize;
            out.data[idx] = (r * 255.0).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 1] = (g * 255.0).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 2] = (b * 255.0).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 3] = 255;
        }
    }
    out
}

/// 🎞️ Which selected sample indices (post-`stride`/`max_frames`) to output.
fn select_sample_indices(total: usize, opts: &VideoIngestOptions) -> Vec<usize> {
    let stride = opts.stride.max(1) as usize;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < total && (opts.max_frames == 0 || out.len() < opts.max_frames as usize) {
        out.push(i);
        i += stride;
    }
    out
}

/// 🐌 Lazy frame decoder over a probed sample table: one decode per [`Iterator::next`], chunkable by callers.
/// MJPEG frames are independently decodable, so unselected samples are never even opened; baseline AVC's
/// P-frame chain means every sample up to (and including) each selected target must still be decoded — only
/// samples *trailing after* the last ever-needed one are skipped.
pub struct FrameIter<'a> {
    bytes: &'a [u8],
    samples: Vec<SampleInfo>,
    decoder: Option<H264Decoder>,
    opts: VideoIngestOptions,
    selected: Vec<usize>,
    cursor: usize,
    sample_idx: usize,
}

impl<'a> FrameIter<'a> {
    fn new(bytes: &'a [u8], samples: Vec<SampleInfo>, decoder: Option<H264Decoder>, opts: VideoIngestOptions) -> Self {
        let selected = select_sample_indices(samples.len(), &opts);
        Self { bytes, samples, decoder, opts, selected, cursor: 0, sample_idx: 0 }
    }

}

fn frame_iter_sample_bytes<'a>(bytes: &'a [u8], samples: &[SampleInfo], idx: usize) -> Result<&'a [u8], VideoError> {
    let s = &samples[idx];
    let end = s.offset.checked_add(u64::from(s.size)).ok_or(VideoError::Truncated)?;
    bytes.get(s.offset as usize..end as usize).ok_or(VideoError::Truncated)
}

impl<'a> Iterator for FrameIter<'a> {
    type Item = Result<ExtractedFrame, VideoError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.selected.len() {
            return None;
        }
        let target = self.selected[self.cursor];
        match &mut self.decoder {
            None => {
                let bytes = match frame_iter_sample_bytes(self.bytes, &self.samples, target) {
                    Ok(b) => b,
                    Err(e) => return Some(Err(e)),
                };
                let image = match remodel_image::decode_jpeg(bytes) {
                    Ok(i) => i,
                    Err(e) => return Some(Err(e.into())),
                };
                let timestamp_ms = self.samples[target].timestamp_ms;
                self.cursor += 1;
                Some(Ok(ExtractedFrame { index: target as u32, timestamp_ms, image: resize_to_max_long_edge(image, self.opts.max_long_edge_px) }))
            }
            Some(decoder) => {
                while self.sample_idx <= target {
                    let idx = self.sample_idx;
                    let bytes = match frame_iter_sample_bytes(self.bytes, &self.samples, idx) {
                        Ok(b) => b,
                        Err(e) => return Some(Err(e)),
                    };
                    let decoded = match decoder.decode_sample(bytes) {
                        Ok(v) => v,
                        Err(e) => return Some(Err(e.into())),
                    };
                    let Some(image) = decoded else {
                        self.sample_idx += 1;
                        continue;
                    };
                    if idx == target {
                        let timestamp_ms = self.samples[idx].timestamp_ms;
                        self.sample_idx += 1;
                        self.cursor += 1;
                        return Some(Ok(ExtractedFrame { index: idx as u32, timestamp_ms, image: resize_to_max_long_edge(image, self.opts.max_long_edge_px) }));
                    }
                    self.sample_idx += 1;
                }
                None
            }
        }
    }
}

/// 📥 Probes `bytes`, then returns a lazy [`FrameIter`] applying `opts`. Succeeds for MJPEG (either
/// container) and baseline AVC (MP4 only); any other codec is [`VideoError::UnsupportedCodec`], routing the
/// caller to a host decoder.
pub fn extract_frames<'a>(bytes: &'a [u8], opts: &VideoIngestOptions) -> Result<FrameIter<'a>, VideoError> {
    match probe(bytes)? {
        VideoProbe::Mp4(info) => match info.codec {
            VideoCodec::Mjpeg => Ok(FrameIter::new(bytes, info.samples, None, *opts)),
            VideoCodec::Avc => {
                let (sps_pps, nal_length_size) = info.avc_config.ok_or(VideoError::UnsupportedCodec(FourCc(*b"avc1")))?;
                let decoder = H264Decoder::new(&sps_pps)?.with_nal_length_size(nal_length_size);
                Ok(FrameIter::new(bytes, info.samples, Some(decoder), *opts))
            }
            other => Err(VideoError::UnsupportedCodec(codec_fourcc_hint(other))),
        },
        VideoProbe::Avi(info) => match info.codec {
            VideoCodec::Mjpeg => Ok(FrameIter::new(bytes, info.samples, None, *opts)),
            other => Err(VideoError::UnsupportedCodec(codec_fourcc_hint(other))),
        },
    }
}
// #endregion 🔖Extract

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn lcg(state: &mut u64) -> u8 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (*state >> 56) as u8
    }

    fn fill_deterministic(state: &mut u64, len: usize) -> Vec<u8> {
        (0..len).map(|_| lcg(state)).collect()
    }

    fn synth_rgba(width: u32, height: u32, seed: u64) -> remodel_image::ImageRgba8 {
        let mut state = seed;
        let mut img = remodel_image::ImageRgba8::new(width, height);
        for px in img.data.chunks_mut(4) {
            px[0] = lcg(&mut state);
            px[1] = lcg(&mut state);
            px[2] = lcg(&mut state);
            px[3] = 255;
        }
        img
    }

    // #region 🔖CoreTests
    #[test]
    fn fourcc_debug_and_display_render_printable_ascii() {
        let fcc = FourCc::new(b"avc1");
        assert_eq!(format!("{fcc:?}"), "FourCc(\"avc1\")");
        assert_eq!(format!("{fcc}"), "avc1");
    }

    #[test]
    fn fourcc_debug_and_display_render_non_ascii_as_hex() {
        let fcc = FourCc([0x00, 0x01, 0xFF, 0x80]);
        assert_eq!(format!("{fcc:?}"), "FourCc(0001ff80)");
        assert_eq!(format!("{fcc}"), "0001ff80");
    }

    #[test]
    fn video_error_display_messages() {
        assert_eq!(VideoError::Truncated.to_string(), "video container truncated");
        assert_eq!(VideoError::BadBox("x").to_string(), "malformed container box/chunk: x");
        assert_eq!(VideoError::NoVideoTrack.to_string(), "container has no video track");
        assert_eq!(VideoError::UnsupportedCodec(FourCc(*b"xvid")).to_string(), "unsupported video codec: xvid");
        assert_eq!(VideoError::H264(H264Error::NoSps).to_string(), "h264 error: h264 slice references an unparsed sps");
    }

    #[test]
    fn h264_error_display_messages() {
        assert_eq!(H264Error::Truncated.to_string(), "h264 bitstream truncated");
        assert_eq!(H264Error::Malformed("y").to_string(), "malformed h264 bitstream: y");
        assert_eq!(H264Error::Unsupported("z").to_string(), "unsupported h264 feature: z");
        assert_eq!(H264Error::NoSps.to_string(), "h264 slice references an unparsed sps");
        assert_eq!(H264Error::NoPps.to_string(), "h264 slice references an unparsed pps");
    }
    // #endregion 🔖CoreTests

    // #region 🔖BmffTests
    #[test]
    fn write_mp4_mjpeg_probe_round_trip_reports_exact_frames() {
        let frames: Vec<Vec<u8>> = (0..5).map(|i| remodel_image::encode_jpeg(&synth_rgba(16, 16, 100 + i), 90)).collect();
        let mp4 = write_mp4_mjpeg(&frames, 10.0);
        let info = probe_mp4(&mp4).expect("probes");
        assert_eq!(info.frame_count, 5);
        assert_eq!(info.codec, VideoCodec::Mjpeg);
        assert_eq!(info.width, 16);
        assert_eq!(info.height, 16);
        assert!(info.samples.iter().all(|s| s.is_sync));
        for (i, s) in info.samples.iter().enumerate() {
            assert!((s.timestamp_ms - i as f64 * 100.0).abs() < 1.0, "sample {i} timestamp {}", s.timestamp_ms);
        }
    }

    #[test]
    fn mp4_probe_detects_avc1_codec_from_avcc_sample_entry() {
        let sps_pps = h264_enc_sps_pps(1, 1);
        let mp4 = write_mp4_avc(&[h264_enc_i_pcm_sample(1, 1, 0, &[0; 256], &[0; 64], &[0; 64])], &sps_pps, 5.0);
        let info = probe_mp4(&mp4).expect("probes");
        assert_eq!(info.codec, VideoCodec::Avc);
        assert_eq!(info.width, 16);
        assert_eq!(info.height, 16);
        assert!(info.avc_config.is_some());
    }

    #[test]
    fn mp4_probe_truncated_at_every_prefix_length_errors_not_panics() {
        let frames: Vec<Vec<u8>> = (0..3).map(|i| remodel_image::encode_jpeg(&synth_rgba(16, 16, 200 + i), 80)).collect();
        let mp4 = write_mp4_mjpeg(&frames, 5.0);
        for len in 0..mp4.len() {
            let _ = probe_mp4(&mp4[..len]);
        }
        assert!(probe_mp4(&mp4).is_ok());
    }

    #[test]
    fn mp4_probe_co64_and_multi_entry_stsc_resolve_sample_offsets() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let stsd = mp4_stsd(b"mjpg", 4, 4, &[]);
        let stts = mp4_stts(3, 100);
        let mut stsc_payload = vec![0u8; 4];
        stsc_payload.extend_from_slice(&2u32.to_be_bytes());
        stsc_payload.extend_from_slice(&1u32.to_be_bytes());
        stsc_payload.extend_from_slice(&2u32.to_be_bytes());
        stsc_payload.extend_from_slice(&1u32.to_be_bytes());
        stsc_payload.extend_from_slice(&3u32.to_be_bytes());
        stsc_payload.extend_from_slice(&1u32.to_be_bytes());
        stsc_payload.extend_from_slice(&1u32.to_be_bytes());
        let stsc = mp4_box(b"stsc", &stsc_payload);
        let stsz = mp4_stsz(&[10, 10, 10]);
        let mut co64_payload = vec![0u8; 4];
        co64_payload.extend_from_slice(&2u32.to_be_bytes());
        co64_payload.extend_from_slice(&1000u64.to_be_bytes());
        co64_payload.extend_from_slice(&1020u64.to_be_bytes());
        let co64 = mp4_box(b"co64", &co64_payload);
        let stbl = [stsd, stts, stsc, stsz, co64].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 300), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let trak = mp4_box(b"mdia", &mdia);
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &trak));
        let bytes = [ftyp, moov].concat();
        let info = probe_mp4(&bytes).expect("probes hand-built co64/stsc tree");
        assert_eq!(info.samples.len(), 3);
        assert_eq!(info.samples[0].offset, 1000);
        assert_eq!(info.samples[1].offset, 1010);
        assert_eq!(info.samples[2].offset, 1020);
    }

    #[test]
    fn mp4_probe_ctts_shifts_presentation_timestamps() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let stsd = mp4_stsd(b"mjpg", 2, 2, &[]);
        let stts = mp4_stts(2, 100);
        let mut ctts_payload = vec![1u8, 0, 0, 0];
        ctts_payload.extend_from_slice(&2u32.to_be_bytes());
        ctts_payload.extend_from_slice(&1u32.to_be_bytes());
        ctts_payload.extend_from_slice(&(50i32).to_be_bytes());
        ctts_payload.extend_from_slice(&1u32.to_be_bytes());
        ctts_payload.extend_from_slice(&(-20i32).to_be_bytes());
        let ctts = mp4_box(b"ctts", &ctts_payload);
        let stsc = mp4_stsc(2);
        let stsz = mp4_stsz(&[5, 5]);
        let stco = mp4_stco(500);
        let stbl = [stsd, stts, ctts, stsc, stsz, stco].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 200), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let trak = mp4_box(b"mdia", &mdia);
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &trak));
        let bytes = [ftyp, moov].concat();
        let info = probe_mp4(&bytes).expect("probes hand-built ctts tree");
        assert!((info.samples[0].timestamp_ms - 50.0).abs() < 1e-9);
        assert!((info.samples[1].timestamp_ms - 80.0).abs() < 1e-9);
    }

    #[test]
    fn mp4_probe_missing_ftyp_or_moov_errors() {
        assert!(matches!(probe_mp4(&[]), Err(VideoError::BadBox(_))));
        let ftyp_only = mp4_box(b"ftyp", b"isom");
        assert!(matches!(probe_mp4(&ftyp_only), Err(VideoError::BadBox(_))));
    }

    #[test]
    fn mp4_probe_audio_only_track_reports_no_video_track() {
        let mut hdlr_payload = vec![0u8; 8];
        hdlr_payload.extend_from_slice(b"soun");
        hdlr_payload.extend_from_slice(&[0u8; 12]);
        hdlr_payload.push(0);
        let mdia = mp4_box(b"mdia", &mp4_box(b"hdlr", &hdlr_payload));
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &mdia));
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let bytes = [ftyp, moov].concat();
        assert!(matches!(probe_mp4(&bytes), Err(VideoError::NoVideoTrack)));
    }

    #[test]
    fn mp4_probe_stsd_zero_entries_errors() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let mut stsd_payload = vec![0u8; 4];
        stsd_payload.extend_from_slice(&0u32.to_be_bytes());
        let stsd = mp4_box(b"stsd", &stsd_payload);
        let stbl = [stsd, mp4_stts(1, 100), mp4_stsc(1), mp4_stsz(&[10]), mp4_stco(0)].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 100), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &mp4_box(b"mdia", &mdia)));
        let bytes = [ftyp, moov].concat();
        assert!(matches!(probe_mp4(&bytes), Err(VideoError::BadBox("stsd has no sample entries"))));
    }

    #[test]
    fn mp4_probe_stts_sample_count_mismatch_errors() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let stsd = mp4_stsd(b"mjpg", 4, 4, &[]);
        let stbl = [stsd, mp4_stts(2, 100), mp4_stsc(1), mp4_stsz(&[10]), mp4_stco(0)].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 200), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &mp4_box(b"mdia", &mdia)));
        let bytes = [ftyp, moov].concat();
        assert!(matches!(probe_mp4(&bytes), Err(VideoError::BadBox("stts sample count does not match stsz"))));
    }

    #[test]
    fn mp4_probe_ctts_sample_count_mismatch_errors() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let stsd = mp4_stsd(b"mjpg", 4, 4, &[]);
        let mut ctts_payload = vec![0u8; 4];
        ctts_payload.extend_from_slice(&1u32.to_be_bytes());
        ctts_payload.extend_from_slice(&2u32.to_be_bytes());
        ctts_payload.extend_from_slice(&0i32.to_be_bytes());
        let ctts = mp4_box(b"ctts", &ctts_payload);
        let stbl = [stsd, mp4_stts(1, 100), ctts, mp4_stsc(1), mp4_stsz(&[10]), mp4_stco(0)].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 100), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &mp4_box(b"mdia", &mdia)));
        let bytes = [ftyp, moov].concat();
        assert!(matches!(probe_mp4(&bytes), Err(VideoError::BadBox("ctts sample count does not match stsz"))));
    }

    #[test]
    fn mp4_probe_stsc_resolved_sample_count_mismatch_errors() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let stsd = mp4_stsd(b"mjpg", 4, 4, &[]);
        let stbl = [stsd, mp4_stts(3, 100), mp4_stsc(2), mp4_stsz(&[10, 10, 10]), mp4_stco(0)].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 300), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &mp4_box(b"mdia", &mdia)));
        let bytes = [ftyp, moov].concat();
        assert!(matches!(probe_mp4(&bytes), Err(VideoError::BadBox("stsc/stco resolved sample count does not match stsz"))));
    }

    #[test]
    fn samples_per_chunk_for_errors_on_invalid_first_chunk_or_uncovered_chunk() {
        assert!(matches!(samples_per_chunk_for(&[(0, 1, 1)], 1), Err(VideoError::BadBox(_))));
        assert!(matches!(samples_per_chunk_for(&[(2, 1, 1)], 1), Err(VideoError::BadBox(_))));
        assert_eq!(samples_per_chunk_for(&[(1, 3, 1), (5, 2, 1)], 4).unwrap(), 3);
        assert_eq!(samples_per_chunk_for(&[(1, 3, 1), (5, 2, 1)], 5).unwrap(), 2);
    }

    #[test]
    fn resolve_samples_rejects_empty_stsc() {
        let sizes = SampleSizes::Uniform { size: 1, count: 1 };
        assert!(matches!(resolve_samples(&[], &[0], &sizes), Err(VideoError::BadBox(_))));
    }
    // #endregion 🔖BmffTests

    // #region 🔖AviTests
    #[test]
    fn write_avi_mjpg_probe_round_trip_reports_exact_frames() {
        let frames: Vec<Vec<u8>> = (0..4).map(|i| remodel_image::encode_jpeg(&synth_rgba(8, 8, 300 + i), 85)).collect();
        let avi = write_avi_mjpg(&frames, 8.0);
        let info = probe_avi(&avi).expect("probes");
        assert_eq!(info.frame_count, 4);
        assert_eq!(info.codec, VideoCodec::Mjpeg);
        assert_eq!(info.width, 8);
        assert_eq!(info.height, 8);
        assert!((info.fps - 8.0).abs() < 1e-6);
        for (i, s) in info.samples.iter().enumerate() {
            assert!((s.timestamp_ms - i as f64 * 125.0).abs() < 1.0);
        }
    }

    #[test]
    fn avi_probe_falls_back_to_movi_scan_without_idx1() {
        let frames: Vec<Vec<u8>> = (0..3).map(|i| remodel_image::encode_jpeg(&synth_rgba(8, 8, 400 + i), 85)).collect();
        let mut avih = Vec::new();
        avih.extend_from_slice(&166_667u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0x10u32.to_le_bytes());
        avih.extend_from_slice(&(frames.len() as u32).to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&1u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&8u32.to_le_bytes());
        avih.extend_from_slice(&8u32.to_le_bytes());
        avih.extend_from_slice(&[0u8; 16]);
        let mut strh = Vec::new();
        strh.extend_from_slice(b"vids");
        strh.extend_from_slice(b"MJPG");
        strh.extend_from_slice(&[0u8; 4]);
        strh.extend_from_slice(&[0u8; 4]);
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&1000u32.to_le_bytes());
        strh.extend_from_slice(&6000u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&(frames.len() as u32).to_le_bytes());
        strh.extend_from_slice(&[0u8; 16]);
        let mut strf = Vec::new();
        strf.extend_from_slice(&40u32.to_le_bytes());
        strf.extend_from_slice(&8i32.to_le_bytes());
        strf.extend_from_slice(&8i32.to_le_bytes());
        strf.extend_from_slice(&1u16.to_le_bytes());
        strf.extend_from_slice(&24u16.to_le_bytes());
        strf.extend_from_slice(b"MJPG");
        strf.extend_from_slice(&[0u8; 20]);
        let strl = [riff_chunk(b"strh", &strh), riff_chunk(b"strf", &strf)].concat();
        let hdrl = [riff_chunk(b"avih", &avih), riff_list(b"strl", &strl)].concat();
        let movi_chunks: Vec<u8> = frames.iter().flat_map(|f| riff_chunk(b"00dc", f)).collect();
        let movi = riff_list(b"movi", &movi_chunks);
        let body = [riff_list(b"hdrl", &hdrl), movi].concat();
        let mut without_idx1 = Vec::new();
        without_idx1.extend_from_slice(b"RIFF");
        without_idx1.extend_from_slice(&(4 + body.len() as u32).to_le_bytes());
        without_idx1.extend_from_slice(b"AVI ");
        without_idx1.extend_from_slice(&body);
        let info = probe_avi(&without_idx1).expect("falls back to movi scan when idx1 is absent");
        assert_eq!(info.frame_count, 3);
    }

    #[test]
    fn avi_probe_rejects_non_riff_magic() {
        assert!(matches!(probe_avi(b"XXXXsize"), Err(VideoError::BadBox(_))));
    }

    #[test]
    fn avi_probe_rejects_non_avi_riff_form() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        assert!(matches!(probe_avi(&bytes), Err(VideoError::BadBox(_))));
    }

    #[test]
    fn avi_probe_audio_only_stream_reports_no_video_track() {
        let mut avih = Vec::new();
        avih.extend_from_slice(&166_667u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0x10u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&1u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&8u32.to_le_bytes());
        avih.extend_from_slice(&8u32.to_le_bytes());
        avih.extend_from_slice(&[0u8; 16]);
        let mut strh = Vec::new();
        strh.extend_from_slice(b"auds");
        strh.extend_from_slice(b"NONE");
        strh.extend_from_slice(&[0u8; 4]);
        strh.extend_from_slice(&[0u8; 4]);
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&1000u32.to_le_bytes());
        strh.extend_from_slice(&6000u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&[0u8; 16]);
        let strl = riff_chunk(b"strh", &strh);
        let hdrl = [riff_chunk(b"avih", &avih), riff_list(b"strl", &strl)].concat();
        let movi = riff_list(b"movi", &[]);
        let body = [riff_list(b"hdrl", &hdrl), movi].concat();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(4 + body.len() as u32).to_le_bytes());
        bytes.extend_from_slice(b"AVI ");
        bytes.extend_from_slice(&body);
        assert!(matches!(probe_avi(&bytes), Err(VideoError::NoVideoTrack)));
    }
    // #endregion 🔖AviTests

    // #region 🔖ProbeTests
    #[test]
    fn probe_dispatches_by_riff_magic() {
        let frames: Vec<Vec<u8>> = (0..2).map(|i| remodel_image::encode_jpeg(&synth_rgba(4, 4, 900 + i), 80)).collect();
        let avi = write_avi_mjpg(&frames, 5.0);
        assert!(matches!(probe(&avi), Ok(VideoProbe::Avi(_))));
        let mp4 = write_mp4_mjpeg(&frames, 5.0);
        assert!(matches!(probe(&mp4), Ok(VideoProbe::Mp4(_))));
    }

    #[test]
    fn codec_fourcc_hint_maps_each_codec_variant() {
        assert_eq!(codec_fourcc_hint(VideoCodec::Avc), FourCc(*b"avc1"));
        assert_eq!(codec_fourcc_hint(VideoCodec::Hevc), FourCc(*b"hvc1"));
        assert_eq!(codec_fourcc_hint(VideoCodec::Vp9), FourCc(*b"vp09"));
        assert_eq!(codec_fourcc_hint(VideoCodec::Av1), FourCc(*b"av01"));
        assert_eq!(codec_fourcc_hint(VideoCodec::Mjpeg), FourCc(*b"mjpg"));
        assert_eq!(codec_fourcc_hint(VideoCodec::Unknown(FourCc(*b"zzzz"))), FourCc(*b"zzzz"));
    }
    // #endregion 🔖ProbeTests

    // #region 🔖ExtractTests
    #[test]
    fn extract_frames_mjpeg_applies_stride_and_max_frames_exactly() {
        let frames: Vec<Vec<u8>> = (0..10).map(|i| remodel_image::encode_jpeg(&synth_rgba(8, 8, 500 + i), 85)).collect();
        let mp4 = write_mp4_mjpeg(&frames, 10.0);
        let opts = VideoIngestOptions { stride: 3, max_frames: 2, max_long_edge_px: 0 };
        let extracted: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("frame decodes")).collect();
        assert_eq!(extracted.len(), 2);
        assert_eq!(extracted[0].index, 0);
        assert_eq!(extracted[1].index, 3);
    }

    #[test]
    fn extract_frames_mjpeg_lazily_skips_undecoded_frames() {
        let mut frames: Vec<Vec<u8>> = (0..6).map(|i| remodel_image::encode_jpeg(&synth_rgba(8, 8, 600 + i), 85)).collect();
        for i in [1usize, 2, 4, 5] {
            frames[i] = vec![0xDE, 0xAD, 0xBE, 0xEF];
        }
        let mp4 = write_mp4_mjpeg(&frames, 10.0);
        let opts = VideoIngestOptions { stride: 3, max_frames: 0, max_long_edge_px: 0 };
        let extracted: Result<Vec<ExtractedFrame>, VideoError> = extract_frames(&mp4, &opts).expect("extracts").collect();
        let extracted = extracted.expect("only sync-selected frames (0, 3) are ever decoded, both real jpegs");
        assert_eq!(extracted.len(), 2);
        assert_eq!(extracted[0].index, 0);
        assert_eq!(extracted[1].index, 3);
    }

    #[test]
    fn extract_frames_applies_max_long_edge_downscale() {
        let frames = vec![remodel_image::encode_jpeg(&synth_rgba(32, 16, 700), 90)];
        let mp4 = write_mp4_mjpeg(&frames, 5.0);
        let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 16 };
        let extracted: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("decodes")).collect();
        assert_eq!(extracted.len(), 1);
        assert_eq!(extracted[0].image.width, 16);
        assert_eq!(extracted[0].image.height, 8);
    }

    #[test]
    fn extract_frames_rejects_unsupported_codec_with_provenance() {
        let ftyp = mp4_box(b"ftyp", b"isom\0\0\x02\0isom");
        let stsd = mp4_stsd(b"hvc1", 4, 4, &[]);
        let stbl = [stsd, mp4_stts(1, 100), mp4_stsc(1), mp4_stsz(&[10]), mp4_stco(0)].concat();
        let minf = mp4_box(b"stbl", &stbl);
        let mdia = [mp4_mdhd(1000, 100), mp4_hdlr(), mp4_box(b"minf", &minf)].concat();
        let moov = mp4_box(b"moov", &mp4_box(b"trak", &mp4_box(b"mdia", &mdia)));
        let bytes = [ftyp, moov].concat();
        let info = probe_mp4(&bytes).expect("hvc1 still probes for provenance");
        assert_eq!(info.codec, VideoCodec::Hevc);
        let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        assert!(matches!(extract_frames(&bytes, &opts), Err(VideoError::UnsupportedCodec(_))));
    }

    #[test]
    fn extract_frames_mjpeg_propagates_jpeg_decode_error() {
        let frames = vec![vec![0xDE, 0xAD, 0xBE, 0xEF]];
        let mp4 = write_mp4_mjpeg(&frames, 5.0);
        let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        let mut iter = extract_frames(&mp4, &opts).expect("extracts");
        assert!(matches!(iter.next(), Some(Err(VideoError::Jpeg(_)))));
    }

    #[test]
    fn extract_frames_avi_rejects_unsupported_codec_with_provenance() {
        let mut avih = Vec::new();
        avih.extend_from_slice(&166_667u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0x10u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&1u32.to_le_bytes());
        avih.extend_from_slice(&0u32.to_le_bytes());
        avih.extend_from_slice(&8u32.to_le_bytes());
        avih.extend_from_slice(&8u32.to_le_bytes());
        avih.extend_from_slice(&[0u8; 16]);
        let mut strh = Vec::new();
        strh.extend_from_slice(b"vids");
        strh.extend_from_slice(b"XVID");
        strh.extend_from_slice(&[0u8; 4]);
        strh.extend_from_slice(&[0u8; 4]);
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&1000u32.to_le_bytes());
        strh.extend_from_slice(&6000u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&0u32.to_le_bytes());
        strh.extend_from_slice(&[0u8; 16]);
        let mut strf = Vec::new();
        strf.extend_from_slice(&40u32.to_le_bytes());
        strf.extend_from_slice(&8i32.to_le_bytes());
        strf.extend_from_slice(&8i32.to_le_bytes());
        strf.extend_from_slice(&1u16.to_le_bytes());
        strf.extend_from_slice(&24u16.to_le_bytes());
        strf.extend_from_slice(b"XVID");
        strf.extend_from_slice(&[0u8; 20]);
        let strl = [riff_chunk(b"strh", &strh), riff_chunk(b"strf", &strf)].concat();
        let hdrl = [riff_chunk(b"avih", &avih), riff_list(b"strl", &strl)].concat();
        let movi = riff_list(b"movi", &[]);
        let body = [riff_list(b"hdrl", &hdrl), movi].concat();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(4 + body.len() as u32).to_le_bytes());
        bytes.extend_from_slice(b"AVI ");
        bytes.extend_from_slice(&body);
        let info = probe_avi(&bytes).expect("probes for provenance even with an undecodable codec");
        assert_eq!(info.codec, VideoCodec::Unknown(FourCc(*b"XVID")));
        let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
        assert!(matches!(extract_frames(&bytes, &opts), Err(VideoError::UnsupportedCodec(_))));
    }

    #[test]
    fn resize_to_max_long_edge_noop_when_budget_zero_or_already_small() {
        let same = resize_to_max_long_edge(synth_rgba(10, 5, 1), 0);
        assert_eq!((same.width, same.height), (10, 5));
        let same2 = resize_to_max_long_edge(synth_rgba(10, 5, 2), 20);
        assert_eq!((same2.width, same2.height), (10, 5));
    }

    #[test]
    fn resize_to_max_long_edge_downscales_preserving_aspect_ratio() {
        let out = resize_to_max_long_edge(synth_rgba(40, 20, 3), 20);
        assert_eq!((out.width, out.height), (20, 10));
    }

    #[test]
    fn select_sample_indices_treats_stride_zero_as_one_and_respects_max_frames() {
        let opts = VideoIngestOptions { stride: 0, max_frames: 3, max_long_edge_px: 0 };
        assert_eq!(select_sample_indices(10, &opts), vec![0, 1, 2]);
    }

    #[test]
    fn select_sample_indices_max_frames_zero_is_unbounded() {
        let opts = VideoIngestOptions { stride: 4, max_frames: 0, max_long_edge_px: 0 };
        assert_eq!(select_sample_indices(10, &opts), vec![0, 4, 8]);
    }

    #[test]
    fn frame_iter_sample_bytes_errors_on_overflow_and_out_of_bounds() {
        let overflow = vec![SampleInfo { offset: u64::MAX, size: 10, timestamp_ms: 0.0, is_sync: true }];
        assert!(matches!(frame_iter_sample_bytes(b"hello", &overflow, 0), Err(VideoError::Truncated)));
        let out_of_bounds = vec![SampleInfo { offset: 100, size: 10, timestamp_ms: 0.0, is_sync: true }];
        assert!(matches!(frame_iter_sample_bytes(b"short", &out_of_bounds, 0), Err(VideoError::Truncated)));
    }
    // #endregion 🔖ExtractTests

    // #region 🔖H264Tests
    fn pcm_frame(mb_w: u32, mb_h: u32, seed: u64) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let mut state = seed;
        let luma = fill_deterministic(&mut state, (mb_w * 16 * mb_h * 16) as usize);
        let cb = fill_deterministic(&mut state, (mb_w * 8 * mb_h * 8) as usize);
        let cr = fill_deterministic(&mut state, (mb_w * 8 * mb_h * 8) as usize);
        (luma, cb, cr)
    }

    #[test]
    fn h264_i_pcm_single_frame_decodes_bit_exactly() {
        let (mb_w, mb_h) = (2, 2);
        let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 42);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let nal = h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr);
        let image = dec.decode_sample(&nal).expect("decodes").expect("immediate output");
        let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, mb_w * 16, mb_h * 16);
        assert_eq!(image, expected);
    }

    #[test]
    fn h264_p_skip_chain_propagates_the_i_pcm_frame_unchanged() {
        let (mb_w, mb_h) = (2, 2);
        let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 7);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let idr = dec.decode_sample(&h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr)).expect("decodes").expect("output");
        let mut prev = idr;
        for frame_num in 1..6u32 {
            let sample = h264_enc_p_skip_sample(mb_w, mb_h, frame_num);
            let image = dec.decode_sample(&sample).expect("p_skip decodes").expect("output");
            assert_eq!(image, prev, "frame {frame_num} should exactly equal the previous decoded frame");
            prev = image;
        }
    }

    #[test]
    fn h264_truncated_nal_errors_not_panics() {
        let (mb_w, mb_h) = (1, 1);
        let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 99);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let full = h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr);
        for len in 0..full.len() {
            let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
            let _ = dec.decode_sample(&full[..len]);
        }
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        assert!(dec.decode_sample(&full).is_ok());
    }

    #[test]
    fn h264_garbage_bytes_error_not_panic() {
        let (mb_w, mb_h) = (1, 1);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut state = 12345u64;
        for _ in 0..40 {
            let garbage = fill_deterministic(&mut state, 40);
            let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
            let _ = dec.decode_sample(&garbage);
        }
    }

    #[test]
    fn h264_new_rejects_truncated_or_missing_sps_pps() {
        assert!(matches!(H264Decoder::new(&[]), Err(H264Error::NoSps)));
        assert!(matches!(H264Decoder::new(&[0, 100]), Err(H264Error::Truncated)));
        assert!(matches!(H264Decoder::new(&[0, 1, 2]), Err(H264Error::NoSps)));
    }

    #[test]
    fn h264_cabac_pps_is_unsupported() {
        let (sps, _) = h264_enc_sps_pps_nals(1, 1);
        let mut pps_bits = BitWriter::default();
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_u(1, 1);
        pps_bits.put_u(0, 1);
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 2);
        pps_bits.put_se(0);
        pps_bits.put_se(0);
        pps_bits.put_se(0);
        pps_bits.put_u(1, 1);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 1);
        pps_bits.rbsp_trailing();
        let pps_nal = write_nal(3, 8, &pps_bits.bytes);
        let mut nals = Vec::new();
        nals.extend_from_slice(&(sps.len() as u16).to_be_bytes());
        nals.extend_from_slice(&sps);
        nals.extend_from_slice(&(pps_nal.len() as u16).to_be_bytes());
        nals.extend_from_slice(&pps_nal);
        assert!(matches!(H264Decoder::new(&nals), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_b_slice_is_unsupported() {
        let (mb_w, mb_h) = (1, 1);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let mut s = BitWriter::default();
        s.put_ue(0);
        s.put_ue(1);
        s.put_ue(0);
        s.put_u(1, 8);
        s.put_u(0, 1);
        s.put_u(0, 1);
        s.put_u(0, 1);
        s.put_u(0, 1);
        s.put_se(0);
        s.put_ue(1);
        s.rbsp_trailing();
        let sample = avcc_frame(&write_nal(2, 1, &s.bytes));
        assert!(matches!(dec.decode_sample(&sample), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_decode_sample_rejects_nonzero_first_mb_in_slice() {
        let (mb_w, mb_h) = (2, 1);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let mut s = BitWriter::default();
        s.put_ue(1);
        s.put_ue(5);
        s.put_ue(0);
        s.put_u(0, 8);
        s.put_u(0, 1);
        s.put_u(0, 1);
        s.put_u(0, 1);
        s.put_se(0);
        s.put_ue(1);
        s.rbsp_trailing();
        let sample = avcc_frame(&write_nal(2, 1, &s.bytes));
        assert!(matches!(dec.decode_sample(&sample), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_decode_sample_rejects_multiple_slice_nals_in_one_access_unit() {
        let (mb_w, mb_h) = (1, 1);
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 1);
        let idr = h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr);
        let mut doubled = idr.clone();
        doubled.extend_from_slice(&idr);
        assert!(matches!(dec.decode_sample(&doubled), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_new_rejects_non_baseline_profile() {
        let mut sps = BitWriter::default();
        sps.put_u(77, 8);
        sps.rbsp_trailing();
        let nal = write_nal(3, 7, &sps.bytes);
        let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
        assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_new_rejects_pic_order_cnt_type_one() {
        let mut sps = BitWriter::default();
        sps.put_u(66, 8);
        sps.put_u(0, 8);
        sps.put_u(30, 8);
        sps.put_ue(0);
        sps.put_ue(4);
        sps.put_ue(1);
        sps.rbsp_trailing();
        let nal = write_nal(3, 7, &sps.bytes);
        let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
        assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_new_rejects_pic_order_cnt_type_out_of_range() {
        let mut sps = BitWriter::default();
        sps.put_u(66, 8);
        sps.put_u(0, 8);
        sps.put_u(30, 8);
        sps.put_ue(0);
        sps.put_ue(4);
        sps.put_ue(3);
        sps.rbsp_trailing();
        let nal = write_nal(3, 7, &sps.bytes);
        let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
        assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Malformed(_))));
    }

    #[test]
    fn h264_new_rejects_interlaced_sps() {
        let mut sps = BitWriter::default();
        sps.put_u(66, 8);
        sps.put_u(0, 8);
        sps.put_u(30, 8);
        sps.put_ue(0);
        sps.put_ue(4);
        sps.put_ue(2);
        sps.put_ue(0);
        sps.put_u(0, 1);
        sps.put_ue(1);
        sps.put_ue(1);
        sps.put_u(0, 1);
        sps.rbsp_trailing();
        let nal = write_nal(3, 7, &sps.bytes);
        let blob = [(nal.len() as u16).to_be_bytes().to_vec(), nal].concat();
        assert!(matches!(H264Decoder::new(&blob), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_new_rejects_multiple_slice_groups_pps() {
        let (sps, _) = h264_enc_sps_pps_nals(1, 1);
        let mut pps_bits = BitWriter::default();
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 1);
        pps_bits.put_ue(1);
        pps_bits.rbsp_trailing();
        let pps_nal = write_nal(3, 8, &pps_bits.bytes);
        let mut nals = Vec::new();
        nals.extend_from_slice(&(sps.len() as u16).to_be_bytes());
        nals.extend_from_slice(&sps);
        nals.extend_from_slice(&(pps_nal.len() as u16).to_be_bytes());
        nals.extend_from_slice(&pps_nal);
        assert!(matches!(H264Decoder::new(&nals), Err(H264Error::Unsupported(_))));
    }

    #[test]
    fn h264_new_rejects_transform_8x8_mode_pps() {
        let (sps, _) = h264_enc_sps_pps_nals(1, 1);
        let mut pps_bits = BitWriter::default();
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 1);
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_ue(0);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 2);
        pps_bits.put_se(0);
        pps_bits.put_se(0);
        pps_bits.put_se(0);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(0, 1);
        pps_bits.put_u(1, 1);
        pps_bits.rbsp_trailing();
        let pps_nal = write_nal(3, 8, &pps_bits.bytes);
        let mut nals = Vec::new();
        nals.extend_from_slice(&(sps.len() as u16).to_be_bytes());
        nals.extend_from_slice(&sps);
        nals.extend_from_slice(&(pps_nal.len() as u16).to_be_bytes());
        nals.extend_from_slice(&pps_nal);
        assert!(matches!(H264Decoder::new(&nals), Err(H264Error::Unsupported(_))));
    }

    /// 🏗️ Like [`h264_enc_i_pcm_sample`] but with a configurable `disable_deblocking_filter_idc`/offsets, to
    /// exercise the in-loop deblocking filter path that this crate's own encoder never turns on.
    #[allow(clippy::too_many_arguments)]
    fn i_pcm_sample_with_deblocking(mb_w: u32, mb_h: u32, frame_num: u32, luma: &[u8], cb: &[u8], cr: &[u8], disable_idc: u32, alpha_off_div2: i32, beta_off_div2: i32) -> Vec<u8> {
        let mut s = BitWriter::default();
        s.put_ue(0);
        s.put_ue(7);
        s.put_ue(0);
        s.put_u(frame_num, 8);
        s.put_ue(0);
        s.put_u(0, 1);
        s.put_u(0, 1);
        s.put_se(0);
        s.put_ue(disable_idc);
        if disable_idc != 1 {
            s.put_se(alpha_off_div2);
            s.put_se(beta_off_div2);
        }
        for n in 0..(mb_w * mb_h) {
            s.put_ue(25);
            s.zero_align();
            let (mb_x, mb_y) = (n % mb_w, n / mb_w);
            let lw = (mb_w * 16) as usize;
            for r in 0..16usize {
                for c in 0..16usize {
                    s.put_u(u32::from(luma[(mb_y as usize * 16 + r) * lw + mb_x as usize * 16 + c]), 8);
                }
            }
            let cw = (mb_w * 8) as usize;
            for r in 0..8usize {
                for c in 0..8usize {
                    s.put_u(u32::from(cb[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
                }
            }
            for r in 0..8usize {
                for c in 0..8usize {
                    s.put_u(u32::from(cr[(mb_y as usize * 8 + r) * cw + mb_x as usize * 8 + c]), 8);
                }
            }
        }
        s.rbsp_trailing();
        avcc_frame(&write_nal(3, 5, &s.bytes))
    }

    #[test]
    fn h264_i_pcm_with_deblocking_enabled_flat_picture_stays_flat() {
        let (mb_w, mb_h) = (2, 2);
        let luma = vec![128u8; (mb_w * 16 * mb_h * 16) as usize];
        let cb = vec![128u8; (mb_w * 8 * mb_h * 8) as usize];
        let cr = cb.clone();
        let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
        let mut dec = H264Decoder::new(&sps_pps).expect("sps/pps parse");
        let nal = i_pcm_sample_with_deblocking(mb_w, mb_h, 0, &luma, &cb, &cr, 0, 0, 0);
        let image = dec.decode_sample(&nal).expect("decodes").expect("output");
        let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, mb_w * 16, mb_h * 16);
        assert_eq!(image, expected, "deblocking a perfectly flat picture is a no-op by construction");
    }

    #[test]
    fn split_annexb_nals_splits_multiple_start_coded_nals() {
        let mut stream = Vec::new();
        stream.extend_from_slice(&[0, 0, 0, 1]);
        stream.extend_from_slice(&[0x67, 0xAA, 0xBB]);
        stream.extend_from_slice(&[0, 0, 1]);
        stream.extend_from_slice(&[0x68, 0xCC]);
        let nals = split_annexb_nals(&stream);
        assert_eq!(nals.len(), 2);
        assert_eq!(nals[0], [0x67, 0xAA, 0xBB]);
        assert_eq!(nals[1], [0x68, 0xCC]);
    }

    #[test]
    fn bitreader_ue_se_roundtrip_via_bitwriter() {
        for v in [0u32, 1, 2, 5, 100, 1000] {
            let mut w = BitWriter::default();
            w.put_ue(v);
            w.rbsp_trailing();
            let mut r = BitReader::new(&w.bytes);
            assert_eq!(r.ue().unwrap(), v);
        }
        for v in [-500i32, -1, 0, 1, 500] {
            let mut w = BitWriter::default();
            w.put_se(v);
            w.rbsp_trailing();
            let mut r = BitReader::new(&w.bytes);
            assert_eq!(r.se().unwrap(), v);
        }
    }

    #[test]
    fn bitreader_ue_rejects_overlong_leading_zero_run() {
        let data = [0x00u8, 0x00, 0x00, 0x00, 0x80];
        let mut r = BitReader::new(&data);
        assert!(matches!(r.ue(), Err(H264Error::Malformed(_))));
    }

    #[test]
    fn bitreader_more_rbsp_data_detects_remaining_bits() {
        let empty = BitReader::new(&[]);
        assert!(!empty.more_rbsp_data());
        let only_stop_bit = BitReader::new(&[0x80]);
        assert!(!only_stop_bit.more_rbsp_data());
        let mut with_more = BitReader::new(&[0xFF, 0x80]);
        assert!(with_more.more_rbsp_data());
        with_more.u(8).unwrap();
        assert!(!with_more.more_rbsp_data());
    }

    // #region 🔖H264PixelMathTests
    #[test]
    fn predict_intra4x4_uniform_neighbors_yield_uniform_output_for_all_modes() {
        let n = Intra4Neighbors { top: Some([100; 4]), left: Some([100; 4]), top_right: [100; 4], corner: 100 };
        for mode in 0..=8u8 {
            let out = predict_intra4x4(mode, &n).unwrap();
            assert_eq!(out, [100; 16], "mode {mode} should reproduce a flat neighborhood exactly");
        }
    }

    #[test]
    fn predict_intra4x4_vertical_and_horizontal_require_their_neighbor() {
        let no_top = Intra4Neighbors { top: None, left: Some([1; 4]), top_right: [1; 4], corner: 1 };
        assert!(matches!(predict_intra4x4(0, &no_top), Err(H264Error::Malformed(_))));
        let no_left = Intra4Neighbors { top: Some([1; 4]), left: None, top_right: [1; 4], corner: 1 };
        assert!(matches!(predict_intra4x4(1, &no_left), Err(H264Error::Malformed(_))));
    }

    #[test]
    fn predict_intra4x4_rejects_out_of_range_mode() {
        let n = Intra4Neighbors { top: Some([0; 4]), left: Some([0; 4]), top_right: [0; 4], corner: 0 };
        assert!(matches!(predict_intra4x4(9, &n), Err(H264Error::Malformed(_))));
    }

    #[test]
    fn predict_intra4x4_dc_mode_averages_available_neighbors() {
        let both = Intra4Neighbors { top: Some([4, 4, 4, 4]), left: Some([12, 12, 12, 12]), top_right: [0; 4], corner: 0 };
        assert_eq!(predict_intra4x4(2, &both).unwrap(), [8; 16]);
        let top_only = Intra4Neighbors { top: Some([4, 4, 4, 4]), left: None, top_right: [0; 4], corner: 0 };
        assert_eq!(predict_intra4x4(2, &top_only).unwrap(), [4; 16]);
        let left_only = Intra4Neighbors { top: None, left: Some([12, 12, 12, 12]), top_right: [0; 4], corner: 0 };
        assert_eq!(predict_intra4x4(2, &left_only).unwrap(), [12; 16]);
        let neither = Intra4Neighbors { top: None, left: None, top_right: [0; 4], corner: 0 };
        assert_eq!(predict_intra4x4(2, &neither).unwrap(), [128; 16]);
    }

    #[test]
    fn dc_pred_all_neighbor_availability_branches() {
        let top = [10i32, 20, 30, 40];
        let left = [1i32, 2, 3, 4];
        assert_eq!(dc_pred(Some(&top), Some(&left), 4), 14);
        assert_eq!(dc_pred(Some(&top), None, 4), 25);
        assert_eq!(dc_pred(None, Some(&left), 4), 3);
        assert_eq!(dc_pred(None, None, 4), 128);
    }

    #[test]
    fn plane_pred_flat_neighbors_are_a_noop() {
        let top16 = vec![100i32; 16];
        let left16 = vec![100i32; 16];
        assert_eq!(plane_pred(&top16, &left16, 100, 16, 5, 32, 6), vec![100i32; 256]);
        let top8 = vec![100i32; 8];
        let left8 = vec![100i32; 8];
        assert_eq!(plane_pred(&top8, &left8, 100, 8, 17, 16, 5), vec![100i32; 64]);
    }

    #[test]
    fn clip_u8_clamps_to_byte_range() {
        assert_eq!(clip_u8(-10), 0);
        assert_eq!(clip_u8(300), 255);
        assert_eq!(clip_u8(128), 128);
    }

    #[test]
    fn norm_adjust_selects_scale_by_position_parity() {
        assert_eq!(norm_adjust(0, 0, 0), 10);
        assert_eq!(norm_adjust(0, 1, 1), 13);
        assert_eq!(norm_adjust(0, 0, 1), 16);
    }

    #[test]
    fn dequant4x4_applies_shift_and_rounding_branches() {
        let coeffs = [1i32; 16];
        let low_qp = dequant4x4(&coeffs, 0);
        assert_eq!((low_qp[0], low_qp[1], low_qp[5]), (1, 1, 1));
        let high_qp = dequant4x4(&coeffs, 24);
        assert_eq!((high_qp[0], high_qp[1], high_qp[5]), (10, 16, 13));
    }

    #[test]
    fn idct4x4_zero_input_is_zero() {
        assert_eq!(idct4x4(&[0; 16]), [0; 16]);
    }

    #[test]
    fn idct4x4_dc_only_produces_uniform_output() {
        let mut d = [0i32; 16];
        d[0] = 64;
        assert_eq!(idct4x4(&d), [1; 16]);
    }

    #[test]
    fn hadamard4_1d_basic_butterfly() {
        assert_eq!(hadamard4_1d([1, 2, 3, 4]), [10, -4, 0, -2]);
    }

    #[test]
    fn transform_luma16x16_dc_zero_input_is_zero() {
        assert_eq!(transform_luma16x16_dc(&[0; 16], 10), [0; 16]);
    }

    #[test]
    fn transform_chroma_dc_computes_expected_values() {
        assert_eq!(transform_chroma_dc(&[4, 0, 0, 0], 0), [1, 1, 1, 1]);
    }

    #[test]
    fn chroma_qp_maps_table_and_clamps_offset() {
        assert_eq!(chroma_qp(0, 0), 0);
        assert_eq!(chroma_qp(30, 0), 29);
        assert_eq!(chroma_qp(51, 0), 39);
        assert_eq!(chroma_qp(51, 20), 39);
    }

    #[test]
    fn read_te_variants_by_max_val() {
        assert_eq!(read_te(&mut BitReader::new(&[]), 0).unwrap(), 0);
        let mut inverted_one = BitReader::new(&[0b1000_0000]);
        assert_eq!(read_te(&mut inverted_one, 1).unwrap(), 0);
        let mut inverted_zero = BitReader::new(&[0b0000_0000]);
        assert_eq!(read_te(&mut inverted_zero, 1).unwrap(), 1);
    }

    #[test]
    fn block4x4_grid_pos_covers_full_4x4_grid_bijectively() {
        let mut seen = std::collections::HashSet::new();
        for n in 0..16 {
            let pos = block4x4_grid_pos(n);
            assert!(pos.0 < 4 && pos.1 < 4);
            assert!(seen.insert(pos));
        }
        assert_eq!(seen.len(), 16);
    }

    #[test]
    fn chroma_quad_pos_maps_known_indices() {
        assert_eq!(chroma_quad_pos(0), (0, 0));
        assert_eq!(chroma_quad_pos(1), (1, 0));
        assert_eq!(chroma_quad_pos(2), (0, 1));
        assert_eq!(chroma_quad_pos(3), (1, 1));
    }

    #[test]
    fn boundary_strength_covers_all_branches() {
        assert_eq!(boundary_strength(true, true, false, 0, 0, [0, 0], 0, [0, 0], 0), 4);
        assert_eq!(boundary_strength(false, true, true, 0, 0, [0, 0], 0, [0, 0], 0), 3);
        assert_eq!(boundary_strength(false, false, false, 1, 0, [0, 0], 0, [0, 0], 0), 2);
        assert_eq!(boundary_strength(false, false, false, 0, 0, [0, 0], 0, [0, 0], 1), 1);
        assert_eq!(boundary_strength(false, false, false, 0, 0, [0, 0], 0, [4, 0], 0), 1);
        assert_eq!(boundary_strength(false, false, false, 0, 0, [0, 0], 0, [3, 0], 0), 0);
    }

    #[test]
    fn median_mv_predict_single_ref_match_shortcut() {
        let a = ([1, 1], 0i8);
        let b = ([2, 2], 0i8);
        let c = ([3, 3], 1i8);
        assert_eq!(median_mv_predict(a, b, c, 1), [3, 3]);
    }

    #[test]
    fn median_mv_predict_falls_back_to_componentwise_median() {
        let a = ([1, 5], 0i8);
        let b = ([2, 6], 0i8);
        let c = ([9, 1], 0i8);
        assert_eq!(median_mv_predict(a, b, c, 2), [2, 5]);
    }

    #[test]
    fn filter_luma_strong_computes_expected_edge_samples() {
        let (pf, qf) = filter_luma_strong([10, 20, 30, 40], [45, 50, 60, 70], 100, 50);
        assert_eq!(pf, [24, 34, 38]);
        assert_eq!(qf, [45, 49, 57]);
    }

    #[test]
    fn filter_luma_strong_passes_through_unfiltered_above_threshold() {
        let (pf, qf) = filter_luma_strong([10, 20, 30, 40], [200, 190, 180, 170], 5, 5);
        assert_eq!(pf, [20, 30, 40]);
        assert_eq!(qf, [200, 190, 180]);
    }

    #[test]
    fn filter_luma_normal_computes_expected_deltas_when_below_threshold() {
        assert_eq!(filter_luma_normal([50, 60, 70], [80, 90, 100], 100, 50, 3), Some((62, 71, 79, 87)));
    }

    #[test]
    fn filter_luma_normal_returns_none_above_threshold() {
        assert_eq!(filter_luma_normal([50, 60, 70], [200, 90, 100], 30, 50, 3), None);
    }

    #[test]
    fn filter_chroma_normal_computes_expected_values_when_below_threshold() {
        assert_eq!(filter_chroma_normal(60, 70, 80, 90, 100, 50, 5), Some((71, 79)));
    }

    #[test]
    fn filter_chroma_normal_returns_none_above_threshold() {
        assert_eq!(filter_chroma_normal(60, 70, 250, 90, 30, 50, 5), None);
    }

    #[test]
    fn filter_chroma_strong_averages_when_below_threshold() {
        assert_eq!(filter_chroma_strong(60, 70, 80, 90, 100, 50), Some((70, 80)));
    }

    #[test]
    fn filter_chroma_strong_returns_none_above_threshold() {
        assert_eq!(filter_chroma_strong(60, 70, 250, 90, 30, 50), None);
    }
    // #endregion 🔖H264PixelMathTests
    // #endregion 🔖H264Tests

    mod long {
        use super::*;

        #[test]
        fn video_in_contract_i_pcm_then_p_skip_chain_via_full_mp4_pipeline() {
            let (mb_w, mb_h) = (3, 2);
            let (width, height) = (mb_w * 16, mb_h * 16);
            let (luma, cb, cr) = pcm_frame(mb_w, mb_h, 2026);
            let sps_pps = h264_enc_sps_pps(mb_w, mb_h);
            let mut samples = vec![h264_enc_i_pcm_sample(mb_w, mb_h, 0, &luma, &cb, &cr)];
            for frame_num in 1..8u32 {
                samples.push(h264_enc_p_skip_sample(mb_w, mb_h, frame_num));
            }
            let mp4 = write_mp4_avc(&samples, &sps_pps, 12.0);

            let probed = probe_mp4(&mp4).expect("probes the muxed avc stream");
            assert_eq!(probed.codec, VideoCodec::Avc);
            assert_eq!(probed.frame_count, 8);
            assert_eq!(probed.width, width);
            assert_eq!(probed.height, height);

            let opts = VideoIngestOptions { stride: 1, max_frames: 0, max_long_edge_px: 0 };
            let frames: Vec<ExtractedFrame> = extract_frames(&mp4, &opts).expect("extracts").map(|f| f.expect("every frame decodes")).collect();
            assert_eq!(frames.len(), 8);
            let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, width, height);
            for (i, frame) in frames.iter().enumerate() {
                assert_eq!(frame.image, expected, "frame {i} should reconstruct pixel-exact via the I_PCM + P_Skip chain");
            }
            for i in 1..frames.len() {
                assert!(frames[i].timestamp_ms > frames[i - 1].timestamp_ms, "timestamps must be monotone");
            }
        }
    }
}
// #endregion 🔖Tests
