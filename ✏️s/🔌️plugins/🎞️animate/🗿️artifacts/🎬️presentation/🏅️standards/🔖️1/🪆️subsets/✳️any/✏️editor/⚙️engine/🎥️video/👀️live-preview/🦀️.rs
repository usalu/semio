//! 👀️ Live preview of the headless video engine: the preview outcome and the windowed, metadata-only and headless preview entry points. See [`super`].

use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::scene::Scene;
use crate::editor::animate::engine::scene::scene::{preview_scene_loop, SceneFrame};
use crate::editor::animate::engine::video::VideoError;
use std::io::Write;

/// 🪟️ Live preview outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreviewOutcome {
    FrameLimit,
    WindowClosed,
    MetadataOnly,
}

/// 🖥️ Previews a scene in a wgpu window when `preview-window` is enabled, else logs frame metadata.
pub async fn preview_scene_window<S: Scene>(mut scene: S, config: &AnimateConfig, max_frames: Option<u64>) -> Result<PreviewOutcome, VideoError> {
    scene.setup(config);
    {
        let outcome = preview_scene_window_metadata(&mut scene, max_frames);
        scene.tear_down();
        Ok(outcome)
    }
}


fn preview_scene_window_metadata<S: Scene>(scene: &mut S, max_frames: Option<u64>) -> PreviewOutcome {
    let max = max_frames.unwrap_or(120);
    let mut stderr = std::io::stderr();
    preview_scene_loop(scene, max, |frame: &SceneFrame| {
        let _ = writeln!(stderr, "[animate-preview] frame={} time={:.3}s mobjects={} section={:?}", frame.frame, frame.time, frame.mobject_count, frame.section);
    });
    PreviewOutcome::MetadataOnly
}

/// 🧪️ Headless preview used by CLI `--preview` flag.
pub async fn preview_scene_headless<S: Scene>(scene: S, config: &AnimateConfig, max_frames: Option<u64>) -> Result<PreviewOutcome, VideoError> {
    preview_scene_window(scene, config, max_frames).await
}

#[cfg(test)]
include!("../🧪️tests/🔬️preview-unit/🦀️.rs");
