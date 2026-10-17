//! 🏭️ Render pipeline of the headless video engine: the output formats and paths, scene capture and the frame recorder that renders a scene to disk. See [`super`].

use crate::editor::animate::engine::animation::animation::{compile_animations, interpolate_at, Animation, Animations, Wait};
use crate::editor::animate::engine::camera::camera::Camera;
use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::scene::Scene;
use crate::editor::animate::engine::scene::section::SectionList;
use crate::editor::animate::engine::scene::sobject::{Sobject, Sobjects};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::editor::animate::engine::video::cache::PartialMovieLut;
use crate::editor::animate::engine::video::renderer::{frame_hash, CapturedFrame, VelloRenderer};
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
    scene.setup(config);
    let mut recorder = FrameRecorder { inner: scene, captures: Vec::new() };
    recorder.construct();
    recorder.tear_down();
    if recorder.captures.is_empty() {
        recorder.capture_now();
    }
    SceneCapture { sections: recorder.inner.sections().clone(), camera: recorder.inner.camera().clone(), captures: recorder.captures }
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
    let mut cache = if config.cache.enabled { Some(PartialMovieLut::open_with_limit(config.cache.partial_movie_dir.clone(), config.cache.max_entries)?) } else { None };

    let mut current_hash = String::new();
    let mut current_partial: Option<PathBuf> = None;
    let mut last_pixels: Option<Vec<u8>> = None;

    for (frame_index, capture) in recorded.captures.iter().enumerate() {
        let hash = frame_hash(capture, config);
        if hash != current_hash {
            if let Some(partial) = current_partial.take() {
                let encoded = writer.finalize_partial(&partial)?;
                if let Some(cache) = cache.as_mut() {
                    cache.insert(current_hash.clone(), encoded)?;
                }
            }
            if let Some(cache) = cache.as_mut() {
                if let Some(cached) = cache.get(&hash) {
                    writer.register_cached_partial(cached);
                    current_hash = hash;
                    current_partial = None;
                    continue;
                }
            }
            current_hash = hash.clone();
            current_partial = Some(writer.begin_partial(&hash, frame_index as u32)?);
        }
        let pixels = renderer.render_capture(capture, &camera, config)?;
        if current_partial.is_some() {
            writer.push_frame(&pixels, frame_index as u32)?;
        }
        last_pixels = Some(pixels);
    }

    if let Some(partial) = current_partial {
        let encoded = writer.finalize_partial(&partial)?;
        if let Some(cache) = cache.as_mut() {
            cache.insert(current_hash, encoded)?;
            let _ = cache.write_index();
        }
    }

    if let Some(subtitles_path) = &config.subtitles_path {
        write_sections_srt(&sections, subtitles_path)?;
    }

    let mut outputs = writer.encode_outputs(last_pixels.as_deref())?;
    outputs.sections = Some(sections_path);
    Ok(outputs)
}

struct FrameRecorder<S> {
    inner: S,
    captures: Vec<CapturedFrame>,
}

impl<S: Scene> FrameRecorder<S> {
    fn capture_now(&mut self) {
        self.captures.push(CapturedFrame { time: self.inner.scene_time(), mobjects: self.inner.mobjects().values().map(|m| m.clone_box()).collect() });
    }
}

impl<S: Scene> Scene for FrameRecorder<S> {
    fn construct(&mut self) {
        self.inner.construct();
    }

    fn setup(&mut self, config: &AnimateConfig) {
        self.inner.setup(config);
    }

    fn tear_down(&mut self) {
        self.inner.tear_down();
    }

    fn config(&self) -> &AnimateConfig {
        self.inner.config()
    }

    fn config_mut(&mut self) -> &mut AnimateConfig {
        self.inner.config_mut()
    }

    fn camera(&self) -> &Camera {
        self.inner.camera()
    }

    fn camera_mut(&mut self) -> &mut Camera {
        self.inner.camera_mut()
    }

    fn mobjects(&self) -> &HashMap<u64, Sobjects> {
        self.inner.mobjects()
    }

    fn mobjects_mut(&mut self) -> &mut HashMap<u64, Sobjects> {
        self.inner.mobjects_mut()
    }

    fn sections(&self) -> &SectionList {
        self.inner.sections()
    }

    fn sections_mut(&mut self) -> &mut SectionList {
        self.inner.sections_mut()
    }

    fn scene_time(&self) -> f64 {
        self.inner.scene_time()
    }

    fn set_scene_time(&mut self, time: f64) {
        self.inner.set_scene_time(time);
    }

    fn play(&mut self, mut animation: Animations) {
        animation.begin();
        let duration = animation.duration().max(0.0);
        let steps = (duration * self.config().frame_rate).ceil() as u64;
        let steps = steps.max(1);
        for frame in 0..=steps {
            let alpha = frame as f64 / steps as f64;
            interpolate_at(self.mobjects_mut(), &mut animation, alpha);
            self.sample_frame(self.config().frame_duration());
        }
        animation.finish();
    }

    fn wait(&mut self, seconds: f64) {
        self.play(Wait::new(seconds).into());
    }

    fn compile_and_play(&mut self, animations: Vec<Animations>) {
        let _durations = compile_animations(&animations);
        for anim in animations {
            self.play(anim);
        }
    }

    fn sample_frame(&mut self, dt: f64) {
        self.inner.sample_frame(dt);
        self.capture_now();
    }
}

#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
include!("../🧪️tests/🔬️render-unit/🦀️.rs");
