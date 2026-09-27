//! 🎥️ The video tier of the raster module: RGBA8 frames → H.264 (AVC) access units → ISO-BMFF (MP4)
//! bytes, first-party and dependency-free on every target (including `wasm32-wasip2`).
//!
//! [`VideoEncoder`] is the interface every H.264 encoder answers to; [`AvcIntraPcmEncoder`] is the
//! first-party one — every picture an IDR whose macroblocks are all `I_PCM` (ITU-T H.264 §7.3.5), so the
//! encoder is exact (BT.601 limited-range 4:2:0 samples, no transform, no prediction) and every sample is
//! a sync sample. [`write_avc_mp4`] muxes any encoder's samples (this one's or a platform encoder's, e.g.
//! WebCodecs on the browser host) into a progressive MP4 (`moov` before `mdat`).
//!
//! The TypeScript twin is `🟦️.ts` beside this file; both answer `🧫️fixtures/🔣️.json` byte for byte.
//! <https://www.itu.int/rec/T-REC-H.264> · <https://www.iso.org/standard/83102.html> (ISO/IEC 14496-12) ·
//! <https://www.iso.org/standard/83529.html> (ISO/IEC 14496-15 `avcC`)

//#region 🔖️Parameters
/// 🎛️ The stream every encoder and the muxer agree on: picture size in pixels and frames per second.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoStreamParameters {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

/// 📏️ The largest picture edge the first-party encoder admits (H.264 level 5.2's frame-size ceiling).
pub const VIDEO_MAXIMUM_EDGE_PIXELS: u32 = 4096;

/// 🎞️ The fastest frame rate the tier admits.
pub const VIDEO_MAXIMUM_FPS: u32 = 120;

/// 🧾️ `profile_idc` of every stream the first-party encoder writes: Baseline (Annex A.2.1) with
/// `constraint_set0_flag` + `constraint_set1_flag` = Constrained Baseline.
pub const AVC_PROFILE_BASELINE: u8 = 66;

/// 🧾️ `constraint_set0_flag` and `constraint_set1_flag` set (Constrained Baseline).
pub const AVC_CONSTRAINED_BASELINE_FLAGS: u8 = 0xC0;

/// 📶️ H.264 Table A-1 rows the level is chosen from: `(level_idc, MaxFS macroblocks, MaxMBPS)`.
pub const AVC_LEVEL_LIMITS: [(u8, u32, u32); 16] = [
    (10, 99, 1_485),
    (11, 396, 3_000),
    (12, 396, 6_000),
    (13, 396, 11_880),
    (20, 396, 11_880),
    (21, 792, 19_800),
    (22, 1_620, 20_250),
    (30, 1_620, 40_500),
    (31, 3_600, 108_000),
    (32, 5_120, 216_000),
    (40, 8_192, 245_760),
    (41, 8_192, 245_760),
    (42, 8_704, 522_240),
    (50, 22_080, 589_824),
    (51, 36_864, 983_040),
    (52, 36_864, 2_073_600),
];

impl VideoStreamParameters {
    /// 🧱️ Picture width in 16×16 macroblocks.
    pub fn macroblock_width(&self) -> u32 {
        self.width.div_ceil(16)
    }

    /// 🧱️ Picture height in 16×16 macroblocks.
    pub fn macroblock_height(&self) -> u32 {
        self.height.div_ceil(16)
    }

    /// 📶️ The smallest Table A-1 level whose frame size and macroblock rate hold this stream (5.2 when none does).
    pub fn level_idc(&self) -> u8 {
        let frame = self.macroblock_width() * self.macroblock_height();
        let rate = frame.saturating_mul(self.fps);
        AVC_LEVEL_LIMITS.iter().find(|(_, max_frame, max_rate)| frame <= *max_frame && rate <= *max_rate).map_or(52, |(level, _, _)| *level)
    }

    /// 🚦️ Refuses what no H.264 4:2:0 stream of this tier can carry.
    pub fn validate(&self) -> Result<(), VideoEncodeError> {
        if self.width == 0 || self.height == 0 || self.width > VIDEO_MAXIMUM_EDGE_PIXELS || self.height > VIDEO_MAXIMUM_EDGE_PIXELS {
            return Err(VideoEncodeError::Dimensions { width: self.width, height: self.height });
        }
        if self.width % 2 != 0 || self.height % 2 != 0 {
            return Err(VideoEncodeError::Dimensions { width: self.width, height: self.height });
        }
        if self.fps == 0 || self.fps > VIDEO_MAXIMUM_FPS {
            return Err(VideoEncodeError::FrameRate { fps: self.fps });
        }
        Ok(())
    }
}
//#endregion 🔖️Parameters

//#region 🔖️Error
/// 🚨️ Everything the video tier refuses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoEncodeError {
    Dimensions { width: u32, height: u32 },
    FrameRate { fps: u32 },
    FrameBytes { expected: usize, actual: usize },
    ContainerTooLarge { bytes: u64 },
    Empty,
    Cancelled,
}

impl std::fmt::Display for VideoEncodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dimensions { width, height } => write!(formatter, "video picture {width}x{height} is not an even size within 2..={VIDEO_MAXIMUM_EDGE_PIXELS}"),
            Self::FrameRate { fps } => write!(formatter, "video frame rate {fps} is outside 1..={VIDEO_MAXIMUM_FPS}"),
            Self::FrameBytes { expected, actual } => write!(formatter, "video frame carries {actual} RGBA bytes, the stream needs {expected}"),
            Self::ContainerTooLarge { bytes } => write!(formatter, "video media data of {bytes} bytes exceeds the 32-bit MP4 chunk offset range"),
            Self::Empty => formatter.write_str("video has no frames"),
            Self::Cancelled => formatter.write_str("video encoding was cancelled"),
        }
    }
}

impl std::error::Error for VideoEncodeError {}
//#endregion 🔖️Error

//#region 🔖️Bitstream
/// ✍️ MSB-first RBSP bit writer (§7.2 `u(n)`, `ue(v)`, `se(v)`, `rbsp_trailing_bits`).
struct BitWriter {
    bytes: Vec<u8>,
    current: u8,
    used: u8,
}

impl BitWriter {
    fn with_capacity(capacity: usize) -> Self {
        Self { bytes: Vec::with_capacity(capacity), current: 0, used: 0 }
    }

    fn bit(&mut self, bit: u32) {
        self.current = (self.current << 1) | (bit & 1) as u8;
        self.used += 1;
        if self.used == 8 {
            self.bytes.push(self.current);
            self.current = 0;
            self.used = 0;
        }
    }

    fn bits(&mut self, value: u32, count: u8) {
        for shift in (0..count).rev() {
            self.bit(value >> shift);
        }
    }

    fn ue(&mut self, value: u32) {
        let coded = u64::from(value) + 1;
        let length = 64 - coded.leading_zeros() as u8;
        for _ in 1..length {
            self.bit(0);
        }
        for shift in (0..length).rev() {
            self.bit((coded >> shift) as u32);
        }
    }

    fn se(&mut self, value: i32) {
        self.ue(if value > 0 { (2 * value - 1) as u32 } else { (-2 * value) as u32 });
    }

    fn aligned(&self) -> bool {
        self.used == 0
    }

    fn align_zero(&mut self) {
        while !self.aligned() {
            self.bit(0);
        }
    }

    fn byte(&mut self, value: u8) {
        if self.aligned() {
            self.bytes.push(value);
        } else {
            self.bits(u32::from(value), 8);
        }
    }

    fn trailing(mut self) -> Vec<u8> {
        self.bit(1);
        self.align_zero();
        self.bytes
    }
}

/// 🛡️ RBSP → NAL unit: header byte, then `emulation_prevention_three_byte` after every two zero bytes
/// followed by a byte ≤ 3 (§7.4.1).
fn nal_unit(nal_ref_idc: u8, nal_unit_type: u8, rbsp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(rbsp.len() + rbsp.len() / 64 + 1);
    out.push((nal_ref_idc << 5) | nal_unit_type);
    let mut zeros = 0;
    for &byte in rbsp {
        if zeros >= 2 && byte <= 3 {
            out.push(3);
            zeros = 0;
        }
        out.push(byte);
        zeros = if byte == 0 { zeros + 1 } else { 0 };
    }
    out
}
//#endregion 🔖️Bitstream

//#region 🔖️Encoder
/// 🧩️ The decoder configuration a stream needs before its first sample: one SPS and one PPS NAL unit
/// (header byte included), exactly what `avcC` carries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AvcDecoderConfiguration {
    pub sps: Vec<u8>,
    pub pps: Vec<u8>,
}

/// 🎞️ One encoded access unit in AVCC framing (every NAL unit prefixed by its 4-byte big-endian length).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedVideoSample {
    pub data: Vec<u8>,
    pub sync: bool,
}

/// 🔌️ The interface every H.264 encoder of this tier answers to: the decoder configuration it writes
/// and one AVCC sample per RGBA8 frame. A platform encoder plugs in behind this trait; nothing outside
/// the tier names its types.
pub trait VideoEncoder {
    fn parameters(&self) -> VideoStreamParameters;
    fn configuration(&self) -> &AvcDecoderConfiguration;
    fn encode(&mut self, rgba: &[u8]) -> Result<EncodedVideoSample, VideoEncodeError>;
}

/// 🧱️ First-party H.264 encoder: every picture one IDR slice of `I_PCM` macroblocks.
pub struct AvcIntraPcmEncoder {
    parameters: VideoStreamParameters,
    configuration: AvcDecoderConfiguration,
    idr_pic_id: u32,
}

impl AvcIntraPcmEncoder {
    /// 🏗️ Validates `parameters` and writes the stream's SPS and PPS.
    pub fn new(parameters: VideoStreamParameters) -> Result<Self, VideoEncodeError> {
        parameters.validate()?;
        Ok(Self { configuration: AvcDecoderConfiguration { sps: sequence_parameter_set(&parameters), pps: picture_parameter_set() }, parameters, idr_pic_id: 0 })
    }
}

impl VideoEncoder for AvcIntraPcmEncoder {
    fn parameters(&self) -> VideoStreamParameters {
        self.parameters
    }

    fn configuration(&self) -> &AvcDecoderConfiguration {
        &self.configuration
    }

    fn encode(&mut self, rgba: &[u8]) -> Result<EncodedVideoSample, VideoEncodeError> {
        let expected = self.parameters.width as usize * self.parameters.height as usize * 4;
        if rgba.len() != expected {
            return Err(VideoEncodeError::FrameBytes { expected, actual: rgba.len() });
        }
        let slice = idr_pcm_slice(&self.parameters, rgba, self.idr_pic_id);
        self.idr_pic_id = (self.idr_pic_id + 1) % 2;
        let mut data = Vec::with_capacity(slice.len() + 4);
        data.extend_from_slice(&(slice.len() as u32).to_be_bytes());
        data.extend_from_slice(&slice);
        Ok(EncodedVideoSample { data, sync: true })
    }
}

fn sequence_parameter_set(parameters: &VideoStreamParameters) -> Vec<u8> {
    let (mb_width, mb_height) = (parameters.macroblock_width(), parameters.macroblock_height());
    let mut bits = BitWriter::with_capacity(32);
    bits.byte(AVC_PROFILE_BASELINE);
    bits.byte(AVC_CONSTRAINED_BASELINE_FLAGS);
    bits.byte(parameters.level_idc());
    bits.ue(0);
    bits.ue(0);
    bits.ue(2);
    bits.ue(1);
    bits.bit(0);
    bits.ue(mb_width - 1);
    bits.ue(mb_height - 1);
    bits.bit(1);
    bits.bit(1);
    let (crop_right, crop_bottom) = ((mb_width * 16 - parameters.width) / 2, (mb_height * 16 - parameters.height) / 2);
    if crop_right != 0 || crop_bottom != 0 {
        bits.bit(1);
        bits.ue(0);
        bits.ue(crop_right);
        bits.ue(0);
        bits.ue(crop_bottom);
    } else {
        bits.bit(0);
    }
    bits.bit(0);
    nal_unit(3, 7, &bits.trailing())
}

fn picture_parameter_set() -> Vec<u8> {
    let mut bits = BitWriter::with_capacity(8);
    bits.ue(0);
    bits.ue(0);
    bits.bit(0);
    bits.bit(0);
    bits.ue(0);
    bits.ue(0);
    bits.ue(0);
    bits.bit(0);
    bits.bits(0, 2);
    bits.se(0);
    bits.se(0);
    bits.se(0);
    bits.bit(1);
    bits.bit(0);
    bits.bit(0);
    nal_unit(3, 8, &bits.trailing())
}

/// 🎨️ BT.601 limited-range luma of one RGB pixel (integer form, identical in the TypeScript twin).
pub fn bt601_luma(r: i32, g: i32, b: i32) -> i32 {
    ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16
}

/// 🎨️ BT.601 limited-range blue-difference chroma of one RGB pixel.
pub fn bt601_cb(r: i32, g: i32, b: i32) -> i32 {
    ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128
}

/// 🎨️ BT.601 limited-range red-difference chroma of one RGB pixel.
pub fn bt601_cr(r: i32, g: i32, b: i32) -> i32 {
    ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128
}

fn idr_pcm_slice(parameters: &VideoStreamParameters, rgba: &[u8], idr_pic_id: u32) -> Vec<u8> {
    let (width, height) = (parameters.width as usize, parameters.height as usize);
    let (mb_width, mb_height) = (parameters.macroblock_width() as usize, parameters.macroblock_height() as usize);
    let pixel = |x: usize, y: usize| {
        let index = (y.min(height - 1) * width + x.min(width - 1)) * 4;
        (i32::from(rgba[index]), i32::from(rgba[index + 1]), i32::from(rgba[index + 2]))
    };
    let mut bits = BitWriter::with_capacity(mb_width * mb_height * 386 + 16);
    bits.ue(0);
    bits.ue(7);
    bits.ue(0);
    bits.bits(0, 4);
    bits.ue(idr_pic_id);
    bits.bit(0);
    bits.bit(0);
    bits.se(0);
    bits.ue(1);
    let mut cb = [0u8; 64];
    let mut cr = [0u8; 64];
    for mb_y in 0..mb_height {
        for mb_x in 0..mb_width {
            bits.ue(25);
            bits.align_zero();
            for y in 0..16 {
                for x in 0..16 {
                    let (r, g, b) = pixel(mb_x * 16 + x, mb_y * 16 + y);
                    bits.byte(bt601_luma(r, g, b) as u8);
                }
            }
            for y in 0..8 {
                for x in 0..8 {
                    let (mut sum_cb, mut sum_cr) = (0, 0);
                    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                        let (r, g, b) = pixel(mb_x * 16 + x * 2 + dx, mb_y * 16 + y * 2 + dy);
                        sum_cb += bt601_cb(r, g, b);
                        sum_cr += bt601_cr(r, g, b);
                    }
                    cb[y * 8 + x] = ((sum_cb + 2) >> 2) as u8;
                    cr[y * 8 + x] = ((sum_cr + 2) >> 2) as u8;
                }
            }
            for value in cb.iter().chain(cr.iter()) {
                bits.byte(*value);
            }
        }
    }
    nal_unit(3, 5, &bits.trailing())
}
//#endregion 🔖️Encoder

//#region 🔖️Mp4Writer
fn put_box(out: &mut Vec<u8>, kind: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&((body.len() + 8) as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
}

fn boxed(kind: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len() + 8);
    put_box(&mut out, kind, body);
    out
}

fn full_box(kind: &[u8; 4], version_flags: u32, body: &[u8]) -> Vec<u8> {
    let mut inner = version_flags.to_be_bytes().to_vec();
    inner.extend_from_slice(body);
    boxed(kind, &inner)
}

fn be32(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|value| value.to_be_bytes()).collect()
}

const UNITY_MATRIX: [u32; 9] = [0x0001_0000, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000];

/// 🧩️ `avcC` — AVCDecoderConfigurationRecord (ISO/IEC 14496-15 §5.3.3.1) for one SPS and one PPS with
/// 4-byte NAL lengths.
pub fn avc_decoder_configuration_record(configuration: &AvcDecoderConfiguration) -> Vec<u8> {
    let sps = &configuration.sps;
    let mut record = vec![1, sps.get(1).copied().unwrap_or(0), sps.get(2).copied().unwrap_or(0), sps.get(3).copied().unwrap_or(0), 0xFF, 0xE1];
    record.extend_from_slice(&(sps.len() as u16).to_be_bytes());
    record.extend_from_slice(sps);
    record.push(1);
    record.extend_from_slice(&(configuration.pps.len() as u16).to_be_bytes());
    record.extend_from_slice(&configuration.pps);
    record
}

/// 🔤️ `avc1.PPCCLL` — the RFC 6381 codec string of a configuration (profile, constraint flags, level).
pub fn avc_codec_string(configuration: &AvcDecoderConfiguration) -> String {
    let sps = &configuration.sps;
    format!("avc1.{:02X}{:02X}{:02X}", sps.get(1).copied().unwrap_or(0), sps.get(2).copied().unwrap_or(0), sps.get(3).copied().unwrap_or(0))
}

/// ⏱️ Movie duration in milliseconds of `frames` frames at `fps` (rounded half up), what `mvhd`/`tkhd` state.
pub fn video_duration_milliseconds(frames: u32, fps: u32) -> u32 {
    ((u64::from(frames) * 1000 + u64::from(fps) / 2) / u64::from(fps.max(1))) as u32
}

fn moov(parameters: &VideoStreamParameters, configuration: &AvcDecoderConfiguration, samples: &[EncodedVideoSample], chunk_offset: u32) -> Vec<u8> {
    let frames = samples.len() as u32;
    let duration_ms = video_duration_milliseconds(frames, parameters.fps);
    let mut mvhd = be32(&[0, 0, 1000, duration_ms, 0x0001_0000]);
    mvhd.extend_from_slice(&[0x01, 0x00, 0, 0]);
    mvhd.extend_from_slice(&[0; 8]);
    mvhd.extend_from_slice(&be32(&UNITY_MATRIX));
    mvhd.extend_from_slice(&[0; 24]);
    mvhd.extend_from_slice(&2u32.to_be_bytes());
    let mut tkhd = be32(&[0, 0, 1, 0, duration_ms, 0, 0]);
    tkhd.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
    tkhd.extend_from_slice(&be32(&UNITY_MATRIX));
    tkhd.extend_from_slice(&be32(&[parameters.width << 16, parameters.height << 16]));
    let mut mdhd = be32(&[0, 0, parameters.fps, frames]);
    mdhd.extend_from_slice(&[0x55, 0xC4, 0, 0]);
    let mut hdlr = be32(&[0]);
    hdlr.extend_from_slice(b"vide");
    hdlr.extend_from_slice(&[0; 12]);
    hdlr.extend_from_slice(b"VideoHandler\0");
    let vmhd = full_box(b"vmhd", 1, &[0; 8]);
    let dinf = boxed(b"dinf", &full_box(b"dref", 0, &[&1u32.to_be_bytes()[..], &full_box(b"url ", 1, &[])].concat()));
    let mut avc1 = vec![0, 0, 0, 0, 0, 0, 0, 1];
    avc1.extend_from_slice(&[0; 16]);
    avc1.extend_from_slice(&(parameters.width as u16).to_be_bytes());
    avc1.extend_from_slice(&(parameters.height as u16).to_be_bytes());
    avc1.extend_from_slice(&be32(&[0x0048_0000, 0x0048_0000, 0]));
    avc1.extend_from_slice(&1u16.to_be_bytes());
    avc1.extend_from_slice(&[0; 32]);
    avc1.extend_from_slice(&[0x00, 0x18, 0xFF, 0xFF]);
    avc1.extend_from_slice(&boxed(b"avcC", &avc_decoder_configuration_record(configuration)));
    let stsd = full_box(b"stsd", 0, &[&1u32.to_be_bytes()[..], &boxed(b"avc1", &avc1)].concat());
    let stts = full_box(b"stts", 0, &be32(&[1, frames, 1]));
    let sync: Vec<u32> = samples.iter().enumerate().filter(|(_, sample)| sample.sync).map(|(index, _)| index as u32 + 1).collect();
    let stss = if sync.len() == samples.len() { Vec::new() } else { full_box(b"stss", 0, &[be32(&[sync.len() as u32]), be32(&sync)].concat()) };
    let stsc = full_box(b"stsc", 0, &be32(&[1, 1, frames, 1]));
    let sizes: Vec<u32> = samples.iter().map(|sample| sample.data.len() as u32).collect();
    let stsz = full_box(b"stsz", 0, &[be32(&[0, frames]), be32(&sizes)].concat());
    let stco = full_box(b"stco", 0, &be32(&[1, chunk_offset]));
    let stbl = boxed(b"stbl", &[stsd, stts, stss, stsc, stsz, stco].concat());
    let minf = boxed(b"minf", &[vmhd, dinf, stbl].concat());
    let mdia = boxed(b"mdia", &[full_box(b"mdhd", 0, &mdhd), full_box(b"hdlr", 0, &hdlr), minf].concat());
    let trak = boxed(b"trak", &[full_box(b"tkhd", 3, &tkhd), mdia].concat());
    boxed(b"moov", &[full_box(b"mvhd", 0, &mvhd), trak].concat())
}

/// 📦️ Muxes AVCC `samples` (one frame each, in presentation order) into a progressive MP4: `ftyp`,
/// `moov` (one `avc1` video track, timescale = fps, one sample per tick, all samples in one chunk),
/// then `mdat`.
pub fn write_avc_mp4(parameters: &VideoStreamParameters, configuration: &AvcDecoderConfiguration, samples: &[EncodedVideoSample]) -> Result<Vec<u8>, VideoEncodeError> {
    parameters.validate()?;
    if samples.is_empty() {
        return Err(VideoEncodeError::Empty);
    }
    let media_bytes: u64 = samples.iter().map(|sample| sample.data.len() as u64).sum();
    let ftyp = boxed(b"ftyp", &[&b"isom"[..], &0x200u32.to_be_bytes(), b"isom", b"iso2", b"avc1", b"mp41"].concat());
    let header_bytes = ftyp.len() as u64 + moov(parameters, configuration, samples, 0).len() as u64 + 8;
    if header_bytes + media_bytes > u64::from(u32::MAX) {
        return Err(VideoEncodeError::ContainerTooLarge { bytes: media_bytes });
    }
    let moov = moov(parameters, configuration, samples, header_bytes as u32);
    let mut out = Vec::with_capacity((header_bytes + media_bytes) as usize);
    out.extend_from_slice(&ftyp);
    out.extend_from_slice(&moov);
    out.extend_from_slice(&((media_bytes + 8) as u32).to_be_bytes());
    out.extend_from_slice(b"mdat");
    for sample in samples {
        out.extend_from_slice(&sample.data);
    }
    Ok(out)
}
//#endregion 🔖️Mp4Writer

//#region 🔖️Pipeline
/// 🎬️ One run of identical frames: an RGBA8 picture shown for `frames` consecutive frames.
pub struct VideoFrameRun<'a> {
    pub rgba: &'a [u8],
    pub frames: u32,
}

/// 📈️ Encodes `runs` with `encoder` and muxes the result, reporting `(frames done, frames total)` after
/// every frame and stopping with [`VideoEncodeError::Cancelled`] as soon as `cancelled` answers true.
pub fn encode_video_runs(encoder: &mut dyn VideoEncoder, runs: &[VideoFrameRun<'_>], progress: &mut dyn FnMut(u32, u32), cancelled: &dyn Fn() -> bool) -> Result<Vec<u8>, VideoEncodeError> {
    let total: u32 = runs.iter().map(|run| run.frames).sum();
    let mut samples = Vec::with_capacity(total as usize);
    for run in runs {
        for _ in 0..run.frames {
            if cancelled() {
                return Err(VideoEncodeError::Cancelled);
            }
            samples.push(encoder.encode(run.rgba)?);
            progress(samples.len() as u32, total);
        }
    }
    write_avc_mp4(&encoder.parameters(), encoder.configuration(), &samples)
}
//#endregion 🔖️Pipeline

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
