//! 🎥️ Headless video engine: Vello frame capture, partial-movie cache, FFmpeg encode. Relocated
//! verbatim from the deleted artifact-tree `⚙️engine/🎥️video` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).

#[path = "💾️partial-movie-cache/🦀️.rs"]
pub mod cache;

#[path = "👀️live-preview/🦀️.rs"]
pub mod preview;

#[path = "🏭️render-pipeline/🦀️.rs"]
pub mod render;

#[path = "🎨️vello-renderer/🦀️.rs"]
pub mod renderer;

pub use semio_s_artifact_animate_presentation_tests::editor::animate::engine::video::scenes;

#[path = "✒️partial-movie-writer/🦀️.rs"]
pub mod writer;

#[path = "🔧️video-render-program/🦀️.rs"]
pub mod program;

pub use cache::PartialMovieLut;
pub use preview::{preview_scene_headless, preview_scene_window, PreviewOutcome};
pub use program::VideoProgramBuilder;
pub use render::{capture_scene, render_scene, OutputFormat, OutputPaths, SceneCapture};
pub use renderer::VelloRenderer;
pub use scenes::scene_for_hash;
pub use writer::{flush_partial_movie_cache, write_sections_srt, SceneFileWriter};

//#region 🔖️Error
/// 🎬️ Errors from headless video rendering, caching, and encoding.
#[derive(Debug)]
pub enum VideoError {
    /// 📁️ A filesystem operation (create/read/write/remove) failed.
    Io { context: &'static str, source: std::io::Error },
    /// 🗑️ Cache eviction found an empty access order (invariant violation).
    CacheEvictionEmpty,
    /// 📡️ GPU readback channel closed before a result arrived.
    ReadbackChannelClosed,
    /// 🖥️ wgpu/vello/window subsystem failure, message from the underlying backend.
    Backend { context: &'static str, message: String },
}

impl std::fmt::Display for VideoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { context, source } => write!(formatter, "{context}: {source}"),
            Self::CacheEvictionEmpty => formatter.write_str("cache eviction: empty order"),
            Self::ReadbackChannelClosed => formatter.write_str("readback channel closed"),
            Self::Backend { context, message } => write!(formatter, "{context}: {message}"),
        }
    }
}

impl std::error::Error for VideoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::CacheEvictionEmpty | Self::ReadbackChannelClosed | Self::Backend { .. } => None,
        }
    }
}

impl VideoError {
    /// 📁️ Curries an io::Error mapper tagged with `context` for `.map_err(...)`.
    pub(crate) fn io(context: &'static str) -> impl Fn(std::io::Error) -> Self {
        move |source| Self::Io { context, source }
    }
    /// 🖥️ Builds a backend-failure variant from any Display/Debug-formatted foreign error.
    pub(crate) fn backend(context: &'static str, message: impl std::fmt::Display) -> Self {
        Self::Backend { context, message: message.to_string() }
    }
}
//#endregion 🔖️Error
