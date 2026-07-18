use crate::render::OutputFormat;
use animate_core::AnimateConfig;
use image::{ImageBuffer, Rgba};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 📝 Partial-movie writer with FFmpeg concat and sidecar outputs.
pub struct SceneFileWriter {
    config: AnimateConfig,
    formats: Vec<OutputFormat>,
    partial_root: PathBuf,
    partial_paths: Vec<PathBuf>,
    png_sequence_dir: Option<PathBuf>,
    file_stem: String,
}

impl SceneFileWriter {
    /// 🏗️ Prepares writer directories from config.
    pub fn new(config: &AnimateConfig, formats: &[OutputFormat]) -> Result<Self, String> {
        fs::create_dir_all(&config.output_dir).map_err(|err| format!("output dir: {err}"))?;
        fs::create_dir_all(&config.media_dir).map_err(|err| format!("media dir: {err}"))?;
        let partial_root = config.cache.partial_movie_dir.clone();
        fs::create_dir_all(&partial_root).map_err(|err| format!("partial dir: {err}"))?;
        let png_sequence_dir = if formats.contains(&OutputFormat::PngSequence) {
            let dir = config.output_dir.join("frames");
            fs::create_dir_all(&dir).map_err(|err| format!("png dir: {err}"))?;
            Some(dir)
        } else {
            None
        };
        Ok(Self {
            config: config.clone(),
            formats: formats.to_vec(),
            partial_root,
            partial_paths: Vec::new(),
            png_sequence_dir,
            file_stem: "scene".into(),
        })
    }

    /// 🎬 Begins a new partial movie directory for `hash`.
    pub fn begin_partial(&mut self, hash: &str, frame_start: u32) -> Result<PathBuf, String> {
        let dir = self.partial_root.join(format!("{}_{frame_start}", &hash[..hash.len().min(12)]));
        fs::create_dir_all(&dir).map_err(|err| format!("partial begin: {err}"))?;
        Ok(dir)
    }

    /// 🖼️ Writes one RGBA frame as PNG into a partial directory.
    pub fn write_frame_png(&mut self, partial_dir: &Path, pixels: &[u8], frame_index: u32) -> Result<(), String> {
        let path = partial_dir.join(format!("{frame_index:06}.png"));
        write_png_file(&path, pixels, self.config.width, self.config.height)?;
        if let Some(dir) = &self.png_sequence_dir {
            let global = dir.join(format!("{frame_index:06}.png"));
            fs::copy(&path, &global).map_err(|err| format!("png copy: {err}"))?;
        }
        Ok(())
    }

    /// ✅ Encodes a partial PNG directory to mp4 and tracks it for concat.
    pub fn finalize_partial(&mut self, partial_dir: &Path) -> Result<PathBuf, String> {
        let partial_mp4 = partial_dir.with_extension("mp4");
        run_ffmpeg(&[
            "-y",
            "-framerate",
            &format_number(self.config.frame_rate),
            "-i",
            &partial_dir.join("%06d.png").display().to_string(),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            &partial_mp4.display().to_string(),
        ])?;
        self.partial_paths.push(partial_mp4.clone());
        Ok(partial_mp4)
    }

    /// ♻️ Reuses a cached partial without re-encoding.
    pub fn register_cached_partial(&mut self, path: &Path) {
        if path.exists() {
            self.partial_paths.push(path.to_path_buf());
        }
    }

    /// 🎞️ Concatenates partial movies and emits configured sidecar outputs.
    pub fn encode_outputs(&self, last_frame: Option<&[u8]>) -> Result<super::render::OutputPaths, String> {
        let mut outputs = super::render::OutputPaths::default();
        if self.formats.contains(&OutputFormat::Mp4) && !self.partial_paths.is_empty() {
            let mp4 = self.config.output_dir.join(format!("{}.mp4", self.file_stem));
            concat_partials(&self.partial_paths, &mp4)?;
            if let Some(audio) = &self.config.audio_track {
                if audio.exists() {
                    let muxed = self.config.output_dir.join(format!("{}_with_audio.mp4", self.file_stem));
                    mux_audio_track(&mp4, audio, &muxed)?;
                    outputs.mp4 = Some(muxed);
                } else {
                    outputs.mp4 = Some(mp4);
                }
            } else {
                outputs.mp4 = Some(mp4);
            }
        }
        if self.formats.contains(&OutputFormat::Gif) {
            if let Some(mp4) = &outputs.mp4 {
                let gif = self.config.output_dir.join(format!("{}.gif", self.file_stem));
                run_ffmpeg(&[
                    "-y",
                    "-i",
                    &mp4.display().to_string(),
                    "-vf",
                    "fps=15,scale=640:-1:flags=lanczos",
                    &gif.display().to_string(),
                ])?;
                outputs.gif = Some(gif);
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

fn format_number(value: f64) -> String {
    use framework_hash::format_number_for_hash;
    format_number_for_hash(value)
}

fn write_png_file(path: &Path, pixels: &[u8], width: u32, height: u32) -> Result<(), String> {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, pixels.to_vec()).ok_or_else(|| "invalid rgba buffer".to_string())?;
    image.save(path).map_err(|err| format!("png write: {err}"))
}

fn concat_partials(partials: &[PathBuf], output: &Path) -> Result<(), String> {
    if partials.len() == 1 {
        fs::copy(&partials[0], output).map_err(|err| format!("copy partial: {err}"))?;
        return Ok(());
    }
    let list_path = output.with_extension("txt");
    let mut list = String::new();
    for partial in partials {
        list.push_str(&format!("file '{}'\n", partial.display()));
    }
    fs::write(&list_path, list).map_err(|err| format!("concat list: {err}"))?;
    run_ffmpeg(&[
        "-y",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        &list_path.display().to_string(),
        "-c",
        "copy",
        &output.display().to_string(),
    ])
}

fn mux_audio_track(video: &Path, audio: &Path, output: &Path) -> Result<(), String> {
    run_ffmpeg(&[
        "-y",
        "-i",
        &video.display().to_string(),
        "-i",
        &audio.display().to_string(),
        "-c:v",
        "copy",
        "-c:a",
        "aac",
        "-shortest",
        &output.display().to_string(),
    ])
}

fn run_ffmpeg(args: &[&str]) -> Result<(), String> {
    let status = Command::new("ffmpeg").args(args).status().map_err(|err| format!("ffmpeg spawn: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("ffmpeg failed with status {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::OutputFormat;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_config() -> AnimateConfig {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_video_test_{stamp}"));
        AnimateConfig::default()
            .with_resolution(16, 16)
            .with_output_dir(&dir)
            .with_media_dir(dir.join("media"))
    }

    #[test]
    fn writer_writes_png_frame() {
        let config = temp_config();
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::LastFrame]).expect("writer");
        let partial = writer.begin_partial("hash", 0).expect("partial");
        let pixels = vec![255u8; 16 * 16 * 4];
        writer.write_frame_png(&partial, &pixels, 0).expect("frame");
        assert!(partial.join("000000.png").exists());
    }
}
