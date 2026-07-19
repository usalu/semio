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

//#region 🔖Error
/// 🎬 Errors from headless video rendering, caching, and encoding.
#[derive(Debug, thiserror::Error)]
pub enum VideoError {
    /// 📁 A filesystem operation (create/read/write/remove) failed.
    #[error("{context}: {source}")]
    Io {
        context: &'static str,
        #[source]
        source: std::io::Error,
    },
    /// 🧾 JSON (de)serialization failed.
    #[error("{context}: {source}")]
    Json {
        context: &'static str,
        #[source]
        source: serde_json::Error,
    },
    /// 🎞️ FFmpeg exited with a non-zero status.
    #[error("ffmpeg failed with status {0}")]
    FfmpegStatus(std::process::ExitStatus),
    /// 🖼️ Pixel buffer length didn't match the declared RGBA8 dimensions.
    #[error("invalid rgba buffer")]
    InvalidRgbaBuffer,
    /// 🗑️ Cache eviction found an empty access order (invariant violation).
    #[error("cache eviction: empty order")]
    CacheEvictionEmpty,
    /// 📡 GPU readback channel closed before a result arrived.
    #[error("readback channel closed")]
    ReadbackChannelClosed,
    /// 🖥️ wgpu/vello/window subsystem failure, message from the underlying backend.
    #[error("{context}: {message}")]
    Backend { context: &'static str, message: String },
}

impl VideoError {
    /// 📁 Curries an io::Error mapper tagged with `context` for `.map_err(...)`.
    pub(crate) fn io(context: &'static str) -> impl Fn(std::io::Error) -> Self {
        move |source| Self::Io { context, source }
    }
    /// 🧾 Curries a serde_json::Error mapper tagged with `context` for `.map_err(...)`.
    pub(crate) fn json(context: &'static str) -> impl Fn(serde_json::Error) -> Self {
        move |source| Self::Json { context, source }
    }
    /// 🖥️ Builds a backend-failure variant from any Display/Debug-formatted foreign error.
    pub(crate) fn backend(context: &'static str, message: impl std::fmt::Display) -> Self {
        Self::Backend { context, message: message.to_string() }
    }
}
//#endregion 🔖Error
