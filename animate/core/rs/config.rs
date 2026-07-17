use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 🎛️ Global render and encode settings for animate engines.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimateConfig {
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub frame_rate: f64,
    pub output_dir: PathBuf,
    pub media_dir: PathBuf,
    pub file_stem: String,
    pub quality: QualityPreset,
    #[serde(default)]
    pub output_formats: Vec<OutputFormat>,
    #[serde(default)]
    pub transparent: bool,
    #[serde(default = "default_cache_partial_movies")]
    pub cache_partial_movies: bool,
    #[serde(default = "default_background")]
    pub background_color: [f32; 4],
    #[serde(default)]
    pub seed: u64,
}

fn default_cache_partial_movies() -> bool {
    true
}

fn default_background() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

impl Default for AnimateConfig {
    fn default() -> Self {
        Self {
            pixel_width: 854,
            pixel_height: 480,
            frame_rate: 15.0,
            output_dir: PathBuf::from("media"),
            media_dir: PathBuf::from("media"),
            file_stem: "scene".into(),
            quality: QualityPreset::Medium720p30,
            output_formats: vec![OutputFormat::Mp4, OutputFormat::LastFrame],
            transparent: false,
            cache_partial_movies: true,
            background_color: default_background(),
            seed: 0,
        }
    }
}

impl AnimateConfig {
    /// 📐 Applies a quality preset to resolution and frame rate.
    pub fn apply_quality(&mut self, quality: QualityPreset) {
        self.quality = quality;
        match quality {
            QualityPreset::Low480p15 => {
                self.pixel_width = 854;
                self.pixel_height = 480;
                self.frame_rate = 15.0;
            }
            QualityPreset::Medium720p30 => {
                self.pixel_width = 1280;
                self.pixel_height = 720;
                self.frame_rate = 30.0;
            }
            QualityPreset::High1080p60 => {
                self.pixel_width = 1920;
                self.pixel_height = 1080;
                self.frame_rate = 60.0;
            }
            QualityPreset::FourK2160p60 => {
                self.pixel_width = 3840;
                self.pixel_height = 2160;
                self.frame_rate = 60.0;
            }
        }
    }

    /// 📁 Returns the partial-movie workspace directory.
    pub fn partial_movie_dir(&self) -> PathBuf {
        self.media_dir.join("partial_movie_files")
    }
}

/// 🎚️ Named quality presets mirroring Manim `-ql` … `-qk`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QualityPreset {
    Low480p15,
    Medium720p30,
    High1080p60,
    FourK2160p60,
}

/// 📼 Encoded artifact kinds produced by the video engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputFormat {
    Mp4,
    Gif,
    PngSequence,
    LastFrame,
}
