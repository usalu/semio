//! 🎥 Headless video engine: Vello frame capture, partial-movie cache, FFmpeg encode.

pub mod cache;
pub mod preview;
pub mod render;
pub mod renderer;
pub mod scenes;
pub mod writer;

pub use cache::PartialMovieCache;
pub use preview::{preview_scene_headless, preview_scene_window, PreviewOutcome};
pub use render::{render_scene, OutputFormat, OutputPaths};
pub use renderer::VelloRenderer;
pub use scenes::scene_for_hash;
pub use writer::{flush_partial_movie_cache, write_sections_srt, SceneFileWriter};
