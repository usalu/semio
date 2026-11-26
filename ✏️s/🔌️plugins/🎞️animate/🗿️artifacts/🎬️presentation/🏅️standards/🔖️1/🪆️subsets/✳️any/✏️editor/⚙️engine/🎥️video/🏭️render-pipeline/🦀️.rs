//! 🏭️ Render pipeline of the headless video engine: the output formats and paths, scene capture and the frame recorder that renders a scene to disk. See [`super`].

use crate::editor::animate::engine::camera::camera::Camera;
use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::scene::{CapturedFrame, Scene};
use crate::editor::animate::engine::scene::section::SectionList;
use std::fs;
use std::path::PathBuf;

use crate::editor::animate::engine::video::cache::PartialMovieLut;
use crate::editor::animate::engine::video::renderer::{partial_hash, static_layer_hash, VelloRenderer};
use crate::editor::animate::engine::video::writer::{write_sections_srt, SceneFileWriter};
use crate::editor::animate::engine::video::VideoError;

/// 📼️ Encoded artifact kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Mp4,
    Gif,
    PngSequence,
    LastFrame,
}

/// 📦️ Paths to encoded artifacts.
#[derive(Clone, Debug, Default)]
pub struct OutputPaths {
    pub mp4: Option<PathBuf>,
    pub gif: Option<PathBuf>,
    pub png_dir: Option<PathBuf>,
    pub last_frame: Option<PathBuf>,
    pub sections: Option<PathBuf>,
}

/// 🎞️ Every frame a `Scene` plays, captured target-neutrally (no GPU, no file system): the mobject state at each
/// timeline sample, the scene's final camera and its sections.
pub struct SceneCapture {
    pub captures: Vec<CapturedFrame>,
    pub camera: Camera,
    pub sections: SectionList,
}

/// 🎬️ Plays `scene` under `config` and captures one [`CapturedFrame`] per timeline sample (one still frame when the
/// scene plays nothing).
pub fn capture_scene<S: Scene>(mut scene: S, config: &AnimateConfig) -> SceneCapture {
    let captures = scene.record_timeline(config);
    SceneCapture { sections: scene.sections().clone(), camera: scene.camera().clone(), captures }
}

/// 🎬️ Renders any `Scene` implementation to configured outputs.
pub async fn render_scene<S: Scene>(scene: S, config: &AnimateConfig, formats: &[OutputFormat]) -> Result<OutputPaths, VideoError> {
    let recorded = capture_scene(scene, config);
    let sections = recorded.sections;
    let sections_path = config.output_dir.join("sections.json");
    fs::create_dir_all(&config.output_dir).map_err(VideoError::io("output dir"))?;
    let sections_value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&sections));
    fs::write(&sections_path, semio_framework_pack_json::to_string_pretty(&sections_value)).map_err(VideoError::io("sections write"))?;

    let camera = recorded.camera;
    let mut renderer = VelloRenderer::new(config.width, config.height).await?;
    let mut writer = SceneFileWriter::new(config, formats)?;
    let cache_movies = formats.contains(&OutputFormat::Mp4) && !formats.contains(&OutputFormat::Gif)
        && !formats.contains(&OutputFormat::PngSequence) && !formats.contains(&OutputFormat::LastFrame);
    let mut cache = if config.cache.enabled && cache_movies { Some(PartialMovieLut::open_with_limit(config.cache.partial_movie_dir.clone(), config.cache.max_entries)?) } else { None };

    let mut last_pixels = None;
    let mut frames = recorded.captures.iter().enumerate().map(|(index, capture)| (index, capture, static_layer_hash(capture, &camera, config))).peekable();
    while let Some((first, capture, picture)) = frames.next() {
        let mut end = first + 1;
        while frames.next_if(|(_, _, next)| *next == picture).is_some() {
            end += 1;
        }
        let key = partial_hash(&picture, end - first);
        if let Some(cached) = cache.as_mut().and_then(|cache| cache.get(&key)) {
            if writer.register_cached_partial(&cached)? {
                continue;
            }
        }
        let partial = writer.begin_partial(first as u32)?;
        let pixels = renderer.render_with_hash(capture, &camera, config, picture)?;
        for index in first..end {
            writer.push_frame(&pixels, index as u32)?;
        }
        let encoded = writer.finalize_partial(&partial)?;
        if let Some(cache) = cache.as_mut() {
            cache.insert(key, &encoded)?;
        }
        last_pixels = Some(pixels);
    }
    if let Some(subtitles_path) = &config.subtitles_path {
        write_sections_srt(&sections, subtitles_path)?;
    }

    let mut outputs = writer.encode_outputs(last_pixels.as_deref())?;
    outputs.sections = Some(sections_path);
    Ok(outputs)
}

#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
include!("../🧪️tests/🔬️render-unit/🦀️.rs");
