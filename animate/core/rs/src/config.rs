//! ⚙️ Global animation configuration, quality presets, and cache paths.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// 🎞️ Output quality preset mirroring Manim quality flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityPreset {
    Low,
    Medium,
    High,
    FourK,
    Production,
}

impl QualityPreset {
    pub fn frame_rate(self) -> f64 {
        match self {
            Self::Low | Self::Medium => 15.0,
            Self::High | Self::FourK | Self::Production => 60.0,
        }
    }

    pub fn resolution(self) -> (u32, u32) {
        match self {
            Self::Low => (854, 480),
            Self::Medium => (1280, 720),
            Self::High => (1920, 1080),
            Self::FourK => (3840, 2160),
            Self::Production => (2560, 1440),
        }
    }

    pub fn pixel_height(self) -> u32 {
        self.resolution().1
    }
}

/// 💾 Cache settings for partial movies and hashed assets.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub max_entries: usize,
    pub partial_movie_dir: PathBuf,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self { enabled: true, max_entries: 10_000, partial_movie_dir: PathBuf::from("partial_movie_files") }
    }
}

/// 🎬 Root configuration for animate scenes and renderers.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimateConfig {
    pub quality: QualityPreset,
    pub frame_rate: f64,
    pub width: u32,
    pub height: u32,
    pub media_dir: PathBuf,
    pub output_dir: PathBuf,
    pub cache: CacheConfig,
    pub background: [f64; 4],
    pub audio_track: Option<PathBuf>,
    pub subtitles_path: Option<PathBuf>,
}

impl Default for AnimateConfig {
    fn default() -> Self {
        Self::from_quality(QualityPreset::High)
    }
}

impl AnimateConfig {
    pub fn from_quality(quality: QualityPreset) -> Self {
        let (width, height) = quality.resolution();
        Self {
            quality,
            frame_rate: quality.frame_rate(),
            width,
            height,
            media_dir: PathBuf::from("media"),
            output_dir: PathBuf::from("output"),
            cache: CacheConfig::default(),
            background: [0.0, 0.0, 0.0, 1.0],
            audio_track: None,
            subtitles_path: None,
        }
    }

    pub fn with_frame_rate(mut self, frame_rate: f64) -> Self {
        self.frame_rate = frame_rate.max(1.0);
        self
    }

    pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
        self.width = width.max(1);
        self.height = height.max(1);
        self
    }

    pub fn with_output_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.output_dir = path.as_ref().to_path_buf();
        self
    }

    pub fn with_media_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.media_dir = path.as_ref().to_path_buf();
        self
    }

    pub fn with_audio_track(mut self, path: impl AsRef<Path>) -> Self {
        self.audio_track = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_subtitles_path(mut self, path: impl AsRef<Path>) -> Self {
        self.subtitles_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn frame_duration(&self) -> f64 {
        1.0 / self.frame_rate
    }

    pub fn aspect_ratio(self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_presets_have_expected_resolution() {
        assert_eq!(QualityPreset::High.resolution(), (1920, 1080));
        assert_eq!(QualityPreset::FourK.resolution(), (3840, 2160));
    }

    #[test]
    fn config_frame_duration_matches_rate() {
        let cfg = AnimateConfig::default().with_frame_rate(30.0);
        assert!((cfg.frame_duration() - 1.0 / 30.0).abs() < 1e-9);
    }
}
