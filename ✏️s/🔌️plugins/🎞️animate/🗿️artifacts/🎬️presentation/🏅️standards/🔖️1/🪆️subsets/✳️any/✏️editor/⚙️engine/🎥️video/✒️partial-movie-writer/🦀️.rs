//! ✒️ Partial-movie writer of the headless video engine: the scene file writer, the sections subtitle writer and the partial-movie cache flush. See [`super`].
//!
//! 🎞️ The raster video tier encodes RGBA frames into H.264 and muxes the samples into MP4.
//! The stdio ISO-BMFF demuxer reads cached access units for concatenation.

use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::section::SectionList;
use crate::editor::animate::engine::video::render::OutputFormat;
use crate::editor::animate::engine::video::VideoError;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

//#region 🔖️AvcPartial
use semio_framework_raster::video::{write_avc_mp4, AvcPcmEncoder, EncodedVideoSample, VideoEncoder, VideoStreamParameters};
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::io::decode_mp4;
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Codec;

struct AvcPartial {
    encoder: AvcPcmEncoder,
    previous: Vec<u8>,
    samples: Vec<EncodedVideoSample>,
}

impl AvcPartial {
    fn new(parameters: VideoStreamParameters) -> Result<Self, VideoError> {
        let encoder = AvcPcmEncoder::new(parameters).map_err(|error| VideoError::backend("initialize video encoder", error))?;
        Ok(Self { encoder, previous: Vec::new(), samples: Vec::new() })
    }

    fn push(&mut self, pixels: &[u8]) -> Result<(), VideoError> {
        let sample = if !self.samples.is_empty() && self.previous == pixels { self.encoder.repeat() } else { self.encoder.encode(pixels) }
            .map_err(|error| VideoError::backend("encode video frame", error))?;
        if sample.sync {
            self.previous.clear();
            self.previous.extend_from_slice(pixels);
        }
        self.samples.push(sample);
        Ok(())
    }

    fn finish(&self) -> Result<Vec<u8>, VideoError> {
        write_avc_mp4(&self.encoder.parameters(), self.encoder.configuration(), &self.samples).map_err(|error| VideoError::backend("mux partial video", error))
    }
}
//#endregion 🔖️AvcPartial

//#region 🔖️GifQuantize
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::io::encode_gif;
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifDisposal, GifFrame, GifRgb, GifSnapshot};

const GIF_CUBE_LEVELS: [u8; 6] = [0, 51, 102, 153, 204, 255];
const GIF_TARGET_FPS: f64 = 15.0;
const GIF_MAX_WIDTH: u32 = 640;
const GIF_CUBE_COLORS: usize = 216;
const GIF_PALETTE_SIZE: usize = 256;
const GIF_PALETTE_SAMPLE_PIXELS: usize = 32_768;

/// 🎨️ The fixed 6×6×6 color cube with 40 entries available for exact authored colors.
fn gif_palette() -> Vec<GifRgb> {
    let mut palette = Vec::with_capacity(GIF_PALETTE_SIZE);
    for &r in &GIF_CUBE_LEVELS {
        for &g in &GIF_CUBE_LEVELS {
            for &b in &GIF_CUBE_LEVELS {
                palette.push(GifRgb { r, g, b });
            }
        }
    }
    palette.resize(GIF_PALETTE_SIZE, GifRgb::default());
    palette
}

fn gif_cube_level(value: u8) -> u32 {
    (u32::from(value) * 5 + 127) / 255
}

/// 🔎️ Direct arithmetic nearest-color index into `gif_palette()`'s fixed uniform cube (no
/// brute-force search needed since the cube's levels are evenly spaced).
fn nearest_cube_index(r: u8, g: u8, b: u8) -> u8 {
    (gif_cube_level(r) * 36 + gif_cube_level(g) * 6 + gif_cube_level(b)) as u8
}

/// 🔬️ Own simple nearest-neighbor spatial scaler — mirrors the old `scale=640:-1` ffmpeg
/// filter's target width. Stdio's gif engine has no scaling logic and shouldn't grow any per
/// this plugin's extraction-map recipe, so this stays local, own domain code.
fn nearest_neighbor_scale(pixels: &[u8], src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (dst_w * dst_h * 4) as usize];
    for y in 0..dst_h {
        let sy = (y * src_h) / dst_h.max(1);
        for x in 0..dst_w {
            let sx = (x * src_w) / dst_w.max(1);
            let si = ((sy * src_w + sx) * 4) as usize;
            let di = ((y * dst_w + x) * 4) as usize;
            out[di..di + 4].copy_from_slice(&pixels[si..si + 4]);
        }
    }
    out
}

fn outside_gif_cube(color: [u8; 3]) -> bool {
    color.into_iter().any(|channel| channel % 51 != 0)
}

/// 🖍️ Reserves an exact owner color and the most frequent other colors in a bounded frame sample.
fn gif_exact_colors(pixels: &[[u8; 4]], owner: [u8; 3]) -> Vec<[u8; 3]> {
    let mut counts = HashMap::new();
    let stride = pixels.len().div_ceil(GIF_PALETTE_SAMPLE_PIXELS).max(1);
    for &[r, g, b, _] in pixels.iter().step_by(stride) {
        let color = [r, g, b];
        if color != owner && outside_gif_cube(color) {
            *counts.entry(color).or_insert(0usize) += 1;
        }
    }
    let mut exact: Vec<_> = outside_gif_cube(owner).then_some(owner).into_iter().collect();
    let remaining = GIF_PALETTE_SIZE - GIF_CUBE_COLORS - exact.len();
    let mut ranked: Vec<_> = counts.into_iter().map(|(color, count)| (std::cmp::Reverse(count), color)).collect();
    if ranked.len() > remaining {
        ranked.select_nth_unstable(remaining);
        ranked.truncate(remaining);
    }
    ranked.sort_unstable();
    exact.extend(ranked.into_iter().map(|(_, color)| color));
    exact
}

struct GifCapture {
    width: u32,
    height: u32,
    owner: [u8; 3],
    step: u32,
    delay_cs: u16,
    snapshot: GifSnapshot,
}

impl GifCapture {
    fn new(config: &AnimateConfig) -> Result<Self, VideoError> {
        if config.width == 0 || config.height == 0 {
            return Err(VideoError::backend("initialize gif", "frame dimensions must be positive"));
        }
        let width = config.width.clamp(1, GIF_MAX_WIDTH);
        let height = ((f64::from(width) * f64::from(config.height) / f64::from(config.width)).round() as u32).max(1);
        if u16::try_from(height).is_err() {
            return Err(VideoError::backend("initialize gif", "scaled GIF height exceeds 65,535 pixels"));
        }
        let step = ((config.frame_rate / GIF_TARGET_FPS).round() as u32).max(1);
        let delay_cs = ((100.0 * f64::from(step) / config.frame_rate.max(1.0)).round() as u16).max(1);
        let owner = std::array::from_fn(|index| (config.background[index].clamp(0.0, 1.0) * 255.0).round() as u8);
        let snapshot = GifSnapshot { width, height, gct: Some(GifColorTable { sorted: false, colors: gif_palette() }), loop_count: Some(0), ..GifSnapshot::default() };
        Ok(Self { width: config.width, height: config.height, owner, step, delay_cs, snapshot })
    }

    fn push(&mut self, pixels: &[u8], frame_index: u32) -> Result<(), VideoError> {
        if frame_index % self.step != 0 {
            return Ok(());
        }
        let expected = u64::from(self.width) * u64::from(self.height);
        if u64::try_from(pixels.len() / 4).ok() != Some(expected) || pixels.len() % 4 != 0 {
            return Err(VideoError::backend("capture gif frame", "RGBA byte count differs from frame dimensions"));
        }
        let (width, height) = (self.snapshot.width, self.snapshot.height);
        let scaled;
        let pixels = if (width, height) == (self.width, self.height) { pixels } else {
            scaled = nearest_neighbor_scale(pixels, self.width, self.height, width, height);
            &scaled
        };
        let pixels = pixels.as_chunks::<4>().0;
        let exact = gif_exact_colors(pixels, self.owner);
        let exact_indices: HashMap<_, _> = exact.iter().enumerate().map(|(index, color)| (*color, (GIF_CUBE_COLORS + index) as u8)).collect();
        let indices = pixels.iter().map(|&[r, g, b, _]| exact_indices.get(&[r, g, b]).copied().unwrap_or_else(|| nearest_cube_index(r, g, b))).collect();
        let lct = (!exact.is_empty()).then(|| {
            let mut colors = gif_palette();
            for (target, [r, g, b]) in colors[GIF_CUBE_COLORS..].iter_mut().zip(exact) {
                *target = GifRgb { r, g, b };
            }
            GifColorTable { sorted: false, colors }
        });
        self.snapshot.frames.push(GifFrame { left: 0, top: 0, width, height, interlace: false, lct, indices, delay_cs: self.delay_cs, disposal: GifDisposal::RestoreToBackground, transparent_index: None, user_input: false, plain_text: None });
        Ok(())
    }
}
//#endregion 🔖️GifQuantize

/// 🧹️ Clears partial-movie cache directories from config.
pub fn flush_partial_movie_cache(config: &AnimateConfig) -> Result<usize, VideoError> {
    crate::editor::animate::engine::video::cache::PartialMovieLut::flush(&config.cache.partial_movie_dir)
}

/// 📝️ Writes section timings as an SRT subtitle sidecar.
pub fn write_sections_srt(sections: &SectionList, path: &Path) -> Result<(), VideoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(VideoError::io("subtitle dir"))?;
    }
    let mut body = String::new();
    for (index, section) in sections.sections.iter().enumerate() {
        body.push_str(&(index + 1).to_string());
        body.push('\n');
        body.push_str(&format_srt_timestamp(section.start_time));
        body.push_str(" --> ");
        body.push_str(&format_srt_timestamp(section.end_time.max(section.start_time)));
        body.push('\n');
        body.push_str(&section.name);
        body.push_str("\n\n");
    }
    fs::write(path, body).map_err(VideoError::io("subtitle write"))
}

fn format_srt_timestamp(seconds: f64) -> String {
    let total_ms = (seconds.max(0.0) * 1000.0).round() as u64;
    let hours = total_ms / 3_600_000;
    let minutes = (total_ms % 3_600_000) / 60_000;
    let secs = (total_ms % 60_000) / 1000;
    let millis = total_ms % 1000;
    format!("{hours:02}:{minutes:02}:{secs:02},{millis:03}")
}

/// 📝️ Encodes captured RGBA8 frames into independently playable partial movies and
/// concatenates their access units without re-encoding.
pub struct SceneFileWriter {
    config: AnimateConfig,
    formats: Vec<OutputFormat>,
    scratch: super::cache::MovieScratch,
    partial_paths: Vec<PathBuf>,
    video_parameters: Option<VideoStreamParameters>,
    pending_video: Option<AvcPartial>,
    gif: Option<GifCapture>,
    png_sequence_dir: Option<PathBuf>,
    file_stem: String,
}

impl SceneFileWriter {
    /// 🏗️ Prepares writer directories from config.
    pub fn new(config: &AnimateConfig, formats: &[OutputFormat]) -> Result<Self, VideoError> {
        fs::create_dir_all(&config.output_dir).map_err(VideoError::io("output dir"))?;
        fs::create_dir_all(&config.media_dir).map_err(VideoError::io("media dir"))?;
        let png_sequence_dir = if formats.contains(&OutputFormat::PngSequence) {
            let dir = config.output_dir.join("frames");
            fs::create_dir_all(&dir).map_err(VideoError::io("png dir"))?;
            Some(dir)
        } else {
            None
        };
        let video_parameters = formats.contains(&OutputFormat::Mp4).then_some(VideoStreamParameters {
            width: config.width,
            height: config.height,
            fps: (config.frame_rate.round() as u32).max(1),
        });
        let pending_video = video_parameters.map(AvcPartial::new).transpose()?;
        let gif = formats.contains(&OutputFormat::Gif).then(|| GifCapture::new(config)).transpose()?;
        let scratch = super::cache::MovieScratch::new(&config.cache.partial_movie_dir)?;
        Ok(Self { config: config.clone(), formats: formats.to_vec(), scratch, partial_paths: Vec::new(), video_parameters, pending_video, gif, png_sequence_dir, file_stem: "scene".into() })
    }

    /// 🎬️ Begins a partial movie in the current render's private directory.
    pub fn begin_partial(&mut self, frame_start: u32) -> Result<PathBuf, VideoError> {
        self.pending_video = self.video_parameters.map(AvcPartial::new).transpose()?;
        Ok(self.scratch.file(format!("frame-{frame_start}.mp4")))
    }

    /// 🖼️ Encodes one captured RGBA8 frame for the open partial and writes its requested PNG.
    pub fn push_frame(&mut self, pixels: &[u8], frame_index: u32) -> Result<(), VideoError> {
        if let Some(video) = &mut self.pending_video {
            video.push(pixels)?;
        }
        if let Some(gif) = &mut self.gif {
            gif.push(pixels, frame_index)?;
        }
        if let Some(dir) = &self.png_sequence_dir {
            let path = dir.join(format!("{frame_index:06}.png"));
            write_png_file(&path, pixels, self.config.width, self.config.height)?;
        }
        Ok(())
    }

    /// ✅️ Muxes the pending access units and tracks the saved partial movie.
    pub fn finalize_partial(&mut self, partial_path: &Path) -> Result<PathBuf, VideoError> {
        if let Some(video) = &self.pending_video {
            let bytes = video.finish()?;
            fs::write(partial_path, bytes).map_err(VideoError::io("partial mp4 write"))?;
            self.partial_paths.push(partial_path.to_path_buf());
            self.pending_video = None;
        }
        Ok(partial_path.to_path_buf())
    }

    /// ♻️ Pins a cached movie until this render finishes assembling its output.
    pub fn register_cached_partial(&mut self, path: &Path) -> Result<bool, VideoError> {
        let pinned = self.scratch.file(format!("cached-{}.mp4", self.partial_paths.len()));
        match fs::hard_link(path, &pinned).or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                Err(error)
            } else {
                fs::copy(path, &pinned).map(|_| ())
            }
        }) {
            Ok(()) => {
                self.partial_paths.push(pinned);
                Ok(true)
            }
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
            Err(error) => Err(VideoError::io("pin cached mp4")(error)),
        }
    }

    /// 🎞️ Concatenates partial movie segments and emits configured sidecar outputs.
    pub fn encode_outputs(&self, last_frame: Option<&[u8]>) -> Result<super::render::OutputPaths, VideoError> {
        let mut outputs = super::render::OutputPaths::default();
        if let Some(parameters) = self.video_parameters.filter(|_| !self.partial_paths.is_empty()) {
            let bytes = concat_avc_partials(&self.partial_paths, parameters)?;
            let mp4 = self.config.output_dir.join(format!("{}.mp4", self.file_stem));
            fs::write(&mp4, bytes).map_err(VideoError::io("write concatenated mp4"))?;
            outputs.mp4 = Some(mp4);
        }
        if let Some(gif) = self.gif.as_ref().filter(|gif| !gif.snapshot.frames.is_empty()) {
            let bytes = encode_gif(&gif.snapshot).map_err(|error| VideoError::backend("encode gif", error))?;
            let path = self.config.output_dir.join(format!("{}.gif", self.file_stem));
            fs::write(&path, bytes).map_err(VideoError::io("gif write"))?;
            outputs.gif = Some(path);
        }
        if self.formats.contains(&OutputFormat::LastFrame) {
            if let Some(pixels) = last_frame {
                let png = self.config.output_dir.join(format!("{}.png", self.file_stem));
                write_png_file(&png, pixels, self.config.width, self.config.height)?;
                outputs.last_frame = Some(png);
            }
        }
        outputs.png_dir = self.png_sequence_dir.clone();
        Ok(outputs)
    }
}

fn write_png_file(path: &Path, pixels: &[u8], width: u32, height: u32) -> Result<(), VideoError> {
    let image = semio_framework_pixels::RasterImage { width, height, pixels: pixels.to_vec() };
    let encoded = semio_framework_pixels::encode_png(&image).map_err(|err| VideoError::backend("png encode", err))?;
    fs::write(path, encoded).map_err(|err| VideoError::backend("png write", err))
}

fn concat_avc_partials(partials: &[PathBuf], parameters: VideoStreamParameters) -> Result<Vec<u8>, VideoError> {
    let encoder = AvcPcmEncoder::new(parameters).map_err(|error| VideoError::backend("initialize video encoder", error))?;
    let configuration = encoder.configuration();
    let expected_codec = Mp4Codec::avc(vec![configuration.sps.clone()], vec![configuration.pps.clone()], 4, None);
    let mut samples = Vec::new();
    for partial in partials {
        let bytes = fs::read(partial).map_err(VideoError::io("read partial mp4"))?;
        let snapshot = decode_mp4(&bytes).map_err(|error| VideoError::backend("demux partial mp4", error))?;
        let track = snapshot.tracks.into_iter().next().ok_or_else(|| VideoError::backend("demux partial mp4", "missing video track"))?;
        if track.codec != expected_codec || track.width != parameters.width || track.height != parameters.height || track.timescale != parameters.fps
            || !track.samples.first().is_some_and(|sample| sample.sync) || track.samples.iter().any(|sample| sample.duration != 1 || sample.cts_offset != 0)
        {
            return Err(VideoError::backend("concatenate partial mp4", "partial video settings or timing differ"));
        }
        if partials.len() == 1 {
            return Ok(bytes);
        }
        samples.extend(track.samples.into_iter().map(|sample| EncodedVideoSample { data: sample.data, sync: sample.sync }));
    }
    write_avc_mp4(&parameters, configuration, &samples).map_err(|error| VideoError::backend("mux concatenated video", error))
}

#[cfg(test)]
include!("../🧪️tests/🔬️writer-unit/🦀️.rs");
