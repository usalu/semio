//! ✒️ Partial-movie writer of the headless video engine: the scene file writer, the sections subtitle writer and the partial-movie cache flush. See [`super`].
//!
//! 📝️ Partial-movie writer. The FFmpeg subprocess path (`Command::new("ffmpeg")`, a real
//! CLAUDE.md "no external runtime dependency" violation confirmed by W0 recon) is deleted
//! outright — mp4 assembly now goes through stdio's real ISO-BMFF `encode_mp4`/`decode_mp4`
//! engine, and the gif sidecar through stdio's real `encode_gif` engine, both in-process, no
//! subprocess involved.

use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::section::SectionList;
use crate::editor::animate::engine::video::render::OutputFormat;
use crate::editor::animate::engine::video::VideoError;
use std::fs;
use std::path::{Path, PathBuf};

//#region 🔖️Mp4RawCodec
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::io::{decode_mp4, encode_mp4};
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Sample, Mp4Snapshot, Mp4Track};

/// 🧬️ Builds one partial segment's real `Mp4Snapshot` from captured RGBA8 frames — real ISO-
/// BMFF container structure via stdio's own `encode_mp4` below, never hand-rolled here.
fn build_raw_mp4_snapshot(width: u32, height: u32, frame_rate: f64, frames: &[Vec<u8>]) -> Mp4Snapshot {
    let timescale = (frame_rate.round() as u32).max(1);
    let samples: Vec<Mp4Sample> = frames.iter().map(|pixels| Mp4Sample { data: pixels.clone(), duration: 1, cts_offset: 0, sync: true }).collect();
    let codec = Mp4Codec::default();
    let track = Mp4Track { track_id: 1, timescale, codec, width, height, metadata: Default::default(), chunk_sample_counts: vec![samples.len() as u32], samples };
    Mp4Snapshot { tracks: vec![track], ..Mp4Snapshot::default() }
}
//#endregion 🔖️Mp4RawCodec

//#region 🔖️GifQuantize
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::io::encode_gif;
use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifDisposal, GifFrame, GifRgb, GifSnapshot};

const GIF_CUBE_LEVELS: [u8; 6] = [0, 51, 102, 153, 204, 255];
const GIF_TARGET_FPS: f64 = 15.0;
const GIF_MAX_WIDTH: u32 = 640;

/// 🎨️ Fixed 6×6×6 color cube (216 entries), padded to a valid power-of-two 256-entry GCT —
/// this artifact's own schema documents trailing padding entries past the meaningful palette
/// as real, expected on-disk bytes, not a modeling gap. No scaling/quantization logic is
/// added to stdio's gif engine itself (out of scope per this plugin's extraction map) — this
/// stays local, own domain code.
fn gif_palette() -> Vec<GifRgb> {
    let mut palette = Vec::with_capacity(256);
    for &r in &GIF_CUBE_LEVELS {
        for &g in &GIF_CUBE_LEVELS {
            for &b in &GIF_CUBE_LEVELS {
                palette.push(GifRgb { r, g, b });
            }
        }
    }
    palette.resize(256, GifRgb::default());
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

fn rgba_to_gif_indices(pixels: &[u8]) -> Vec<u8> {
    pixels.as_chunks::<4>().0.iter().map(|p| nearest_cube_index(p[0], p[1], p[2])).collect()
}

/// 🎞️ Builds a real `GifSnapshot` from captured RGBA8 frames: own frame-rate decimation +
/// nearest-neighbor downscale (mirrors the old `fps=15,scale=640:-1` ffmpeg filter), own
/// fixed-cube color quantization, real GIF89a encode via stdio's `encode_gif` below.
fn build_gif_snapshot(width: u32, height: u32, frame_rate: f64, frames: &[Vec<u8>]) -> Option<GifSnapshot> {
    if frames.is_empty() || width == 0 || height == 0 {
        return None;
    }
    let step = ((frame_rate / GIF_TARGET_FPS).round() as usize).max(1);
    let dst_w = width.clamp(1, GIF_MAX_WIDTH);
    let dst_h = ((dst_w as f64 * height as f64 / width as f64).round() as u32).max(1);
    let delay_cs = ((100.0 * step as f64 / frame_rate.max(1.0)).round() as u16).max(1);
    let mut gif_frames = Vec::new();
    for pixels in frames.iter().step_by(step) {
        let scaled = if (dst_w, dst_h) == (width, height) { pixels.clone() } else { nearest_neighbor_scale(pixels, width, height, dst_w, dst_h) };
        let indices = rgba_to_gif_indices(&scaled);
        gif_frames.push(GifFrame { left: 0, top: 0, width: dst_w, height: dst_h, interlace: false, lct: None, indices, delay_cs, disposal: GifDisposal::RestoreToBackground, transparent_index: None, user_input: false, plain_text: None });
    }
    Some(GifSnapshot { width: dst_w, height: dst_h, gct: Some(GifColorTable { sorted: false, colors: gif_palette() }), loop_count: Some(0), frames: gif_frames, ..GifSnapshot::default() })
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

/// 📝️ Partial-movie writer: buffers captured RGBA8 frames in memory per partial, encodes each
/// partial to a real `.mp4` via stdio's mp4 engine on `finalize_partial`, and concatenates
/// partials by decoding+merging their samples and re-encoding — no FFmpeg, no PNG staging.
pub struct SceneFileWriter {
    config: AnimateConfig,
    formats: Vec<OutputFormat>,
    partial_root: PathBuf,
    partial_paths: Vec<PathBuf>,
    pending_frames: Vec<Vec<u8>>,
    png_sequence_dir: Option<PathBuf>,
    file_stem: String,
}

impl SceneFileWriter {
    /// 🏗️ Prepares writer directories from config.
    pub fn new(config: &AnimateConfig, formats: &[OutputFormat]) -> Result<Self, VideoError> {
        fs::create_dir_all(&config.output_dir).map_err(VideoError::io("output dir"))?;
        fs::create_dir_all(&config.media_dir).map_err(VideoError::io("media dir"))?;
        let partial_root = config.cache.partial_movie_dir.clone();
        fs::create_dir_all(&partial_root).map_err(VideoError::io("partial dir"))?;
        let png_sequence_dir = if formats.contains(&OutputFormat::PngSequence) {
            let dir = config.output_dir.join("frames");
            fs::create_dir_all(&dir).map_err(VideoError::io("png dir"))?;
            Some(dir)
        } else {
            None
        };
        Ok(Self { config: config.clone(), formats: formats.to_vec(), partial_root, partial_paths: Vec::new(), pending_frames: Vec::new(), png_sequence_dir, file_stem: "scene".into() })
    }

    /// 🎬️ Begins a new partial-movie segment for `hash`; the returned path is where the
    /// segment's own real `.mp4` (stdio-encoded) lands once `finalize_partial` runs.
    pub fn begin_partial(&mut self, hash: &str, frame_start: u32) -> Result<PathBuf, VideoError> {
        self.pending_frames.clear();
        Ok(self.partial_root.join(format!("{}_{frame_start}.mp4", &hash[..hash.len().min(12)])))
    }

    /// 🖼️ Buffers one captured RGBA8 frame for the open partial and, if `PngSequence` output
    /// was requested, writes it into the flat frame sequence — the real `image` crate PNG
    /// encoder, unrelated to and unchanged by the mp4/gif codec rewiring above.
    pub fn push_frame(&mut self, pixels: &[u8], frame_index: u32) -> Result<(), VideoError> {
        self.pending_frames.push(pixels.to_vec());
        if let Some(dir) = &self.png_sequence_dir {
            let path = dir.join(format!("{frame_index:06}.png"));
            write_png_file(&path, pixels, self.config.width, self.config.height)?;
        }
        Ok(())
    }

    /// ✅️ Encodes the buffered frames into a real `.mp4` at `partial_path` via stdio's mp4
    /// engine and tracks it for the final concat pass.
    pub fn finalize_partial(&mut self, partial_path: &Path) -> Result<PathBuf, VideoError> {
        let snapshot = build_raw_mp4_snapshot(self.config.width, self.config.height, self.config.frame_rate, &self.pending_frames);
        let bytes = encode_mp4(&snapshot);
        fs::write(partial_path, &bytes).map_err(VideoError::io("partial mp4 write"))?;
        self.partial_paths.push(partial_path.to_path_buf());
        self.pending_frames.clear();
        Ok(partial_path.to_path_buf())
    }

    /// ♻️ Reuses a cached partial without re-encoding.
    pub fn register_cached_partial(&mut self, path: &Path) {
        if path.exists() {
            self.partial_paths.push(path.to_path_buf());
        }
    }

    /// 🎞️ Concatenates partial `.mp4` segments (real decode → merge samples → encode via
    /// stdio's mp4 engine, never FFmpeg) and emits configured sidecar outputs.
    pub fn encode_outputs(&self, last_frame: Option<&[u8]>) -> Result<super::render::OutputPaths, VideoError> {
        let mut outputs = super::render::OutputPaths::default();
        let mut concatenated_frames: Vec<Vec<u8>> = Vec::new();
        if self.formats.contains(&OutputFormat::Mp4) && !self.partial_paths.is_empty() {
            let mp4 = self.config.output_dir.join(format!("{}.mp4", self.file_stem));
            concatenated_frames = concat_raw_partials(&self.partial_paths, &mp4, self.config.width, self.config.height, (self.config.frame_rate.round() as u32).max(1))?;
            outputs.mp4 = Some(mp4);
            // 🚧️ Audio muxing removed with the FFmpeg deletion: stdio's `Mp4Track` schema
            // only models video-handler (`vide`) tracks (see that schema's own doc comment)
            // — there is no honest way to mux `config.audio_track` into this container via
            // stdio today. Reported as a stdio_gap (see `w5a--report.md`); not hand-rolled.
        }
        if self.formats.contains(&OutputFormat::Gif) {
            if let Some(gif_snapshot) = build_gif_snapshot(self.config.width, self.config.height, self.config.frame_rate, &concatenated_frames) {
                if let Ok(bytes) = encode_gif(&gif_snapshot) {
                    let gif = self.config.output_dir.join(format!("{}.gif", self.file_stem));
                    fs::write(&gif, bytes).map_err(VideoError::io("gif write"))?;
                    outputs.gif = Some(gif);
                }
            }
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

/// 🎞️ Decodes+merges partial `.mp4` segments' raw-frame samples into one final `.mp4` at
/// `output` (real stdio decode/encode round trip, never FFmpeg) and returns the merged RGBA8
/// frames so the gif path can reuse them without a second decode pass.
fn concat_raw_partials(partials: &[PathBuf], output: &Path, width: u32, height: u32, timescale: u32) -> Result<Vec<Vec<u8>>, VideoError> {
    if partials.len() == 1 {
        fs::copy(&partials[0], output).map_err(VideoError::io("copy partial"))?;
        let bytes = fs::read(output).map_err(VideoError::io("read concatenated mp4"))?;
        let snapshot = decode_mp4(&bytes).map_err(|error| VideoError::backend("decode single partial mp4", error))?;
        return Ok(snapshot.tracks.into_iter().next().map(|t| t.samples.into_iter().map(|s| s.data).collect()).unwrap_or_default());
    }
    let mut all_samples: Vec<Mp4Sample> = Vec::new();
    let mut codec: Option<Mp4Codec> = None;
    for partial in partials {
        let bytes = fs::read(partial).map_err(VideoError::io("read partial mp4"))?;
        let snapshot = decode_mp4(&bytes).map_err(|error| VideoError::backend("decode partial mp4", error))?;
        if let Some(track) = snapshot.tracks.into_iter().next() {
            if codec.is_none() {
                codec = Some(track.codec);
            }
            all_samples.extend(track.samples);
        }
    }
    let frames: Vec<Vec<u8>> = all_samples.iter().map(|sample| sample.data.clone()).collect();
    let track = Mp4Track { track_id: 1, timescale, codec: codec.unwrap_or_default(), width, height, metadata: Default::default(), chunk_sample_counts: vec![all_samples.len() as u32], samples: all_samples };
    let snapshot = Mp4Snapshot { tracks: vec![track], ..Mp4Snapshot::default() };
    let bytes = encode_mp4(&snapshot);
    fs::write(output, &bytes).map_err(VideoError::io("write concatenated mp4"))?;
    Ok(frames)
}

#[cfg(test)]
include!("../🧪️tests/🔬️writer-unit/🦀️.rs");
