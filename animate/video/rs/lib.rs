//! 🎥 Headless video engine: Vello frame capture, partial-movie cache, FFmpeg encode.

pub mod cache;
pub mod render;
pub mod renderer;
pub mod writer;

pub use cache::PartialMovieCache;
pub use render::{render_scene, OutputPaths};
pub use renderer::VelloRenderer;
pub use writer::SceneFileWriter;
