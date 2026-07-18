use animate_core::{
    compile_animations, interpolate_at, AnimateConfig, Animation, Camera, Scene, SectionList, Sobject, Wait,
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::cache::PartialMovieCache;
use crate::renderer::{frame_hash, CapturedFrame, VelloRenderer};
use crate::writer::SceneFileWriter;

/// 📼 Encoded artifact kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Mp4,
    Gif,
    PngSequence,
    LastFrame,
}

/// 📦 Paths to encoded artifacts.
#[derive(Clone, Debug, Default)]
pub struct OutputPaths {
    pub mp4: Option<PathBuf>,
    pub gif: Option<PathBuf>,
    pub png_dir: Option<PathBuf>,
    pub last_frame: Option<PathBuf>,
    pub sections: Option<PathBuf>,
}

/// 🎬 Renders any `Scene` implementation to configured outputs.
pub fn render_scene<S: Scene>(mut scene: S, config: AnimateConfig, formats: &[OutputFormat]) -> Result<OutputPaths, String> {
    scene.setup(&config);
    let mut recorder = FrameRecorder { inner: scene, captures: Vec::new() };
    recorder.construct();
    recorder.tear_down();
    if recorder.captures.is_empty() {
        recorder.capture_now();
    }

    let sections = recorder.inner.sections().clone();
    let sections_path = config.output_dir.join("sections.json");
    fs::create_dir_all(&config.output_dir).map_err(|err| format!("output dir: {err}"))?;
    fs::write(
        &sections_path,
        serde_json::to_string_pretty(&sections).map_err(|err| format!("sections json: {err}"))?,
    )
    .map_err(|err| format!("sections write: {err}"))?;

    let camera = recorder.inner.camera().clone();
    let mut renderer = VelloRenderer::new(config.width, config.height)?;
    let mut writer = SceneFileWriter::new(&config, formats)?;
    let mut cache = if config.cache.enabled {
        Some(PartialMovieCache::open(config.cache.partial_movie_dir.clone())?)
    } else {
        None
    };

    let mut current_hash = String::new();
    let mut current_partial: Option<PathBuf> = None;
    let mut last_pixels: Option<Vec<u8>> = None;

    for (frame_index, capture) in recorder.captures.iter().enumerate() {
        let hash = frame_hash(capture, &config);
        if hash != current_hash {
            if let Some(partial) = current_partial.take() {
                let encoded = writer.finalize_partial(&partial)?;
                if let Some(cache) = cache.as_mut() {
                    cache.insert(current_hash.clone(), encoded);
                }
            }
            if let Some(cache) = cache.as_ref() {
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
        let pixels = renderer.render_capture(capture, &camera, &config)?;
        if let Some(ref partial) = current_partial {
            writer.write_frame_png(partial, &pixels, frame_index as u32)?;
        }
        last_pixels = Some(pixels);
    }

    if let Some(partial) = current_partial {
        let encoded = writer.finalize_partial(&partial)?;
        if let Some(cache) = cache.as_mut() {
            cache.insert(current_hash, encoded);
            let _ = cache.write_index();
        }
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
        self.captures.push(CapturedFrame {
            time: self.inner.scene_time(),
            mobjects: self.inner.mobjects().values().map(|m| m.clone_box()).collect(),
        });
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

    fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
        self.inner.mobjects()
    }

    fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
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

    fn play(&mut self, mut animation: Box<dyn Animation>) {
        animation.begin();
        let duration = animation.duration().max(0.0);
        let steps = (duration * self.config().frame_rate).ceil() as u64;
        let steps = steps.max(1);
        for frame in 0..=steps {
            let alpha = frame as f64 / steps as f64;
            interpolate_at(self.mobjects_mut(), animation.as_mut(), alpha);
            self.sample_frame(self.config().frame_duration());
        }
        animation.finish();
    }

    fn wait(&mut self, seconds: f64) {
        self.play(Box::new(Wait::new(seconds)));
    }

    fn compile_and_play(&mut self, animations: Vec<Box<dyn Animation>>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use animate_core::{BasicScene, Scene, VSobject};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct DemoScene {
        base: BasicScene,
    }

    impl DemoScene {
        fn new(config: AnimateConfig) -> Self {
            Self {
                base: BasicScene::new(config),
            }
        }
    }

    impl Scene for DemoScene {
        fn construct(&mut self) {
            self.add(Box::new(VSobject::new()));
            self.wait(0.1);
        }
        fn config(&self) -> &AnimateConfig {
            self.base.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.base.config_mut()
        }
        fn camera(&self) -> &Camera {
            self.base.camera()
        }
        fn camera_mut(&mut self) -> &mut Camera {
            self.base.camera_mut()
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.base.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.base.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.base.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.base.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.base.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.base.set_scene_time(time);
        }
    }

    #[test]
    fn render_scene_writes_last_frame() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_render_test_{stamp}"));
        let config = AnimateConfig::default()
            .with_resolution(64, 64)
            .with_frame_rate(15.0)
            .with_output_dir(&dir)
            .with_media_dir(dir.join("media"));
        let scene = DemoScene::new(config.clone());
        let outputs = render_scene(scene, config, &[OutputFormat::LastFrame]).expect("render");
        let last = outputs.last_frame.expect("last frame path");
        assert!(last.exists());
    }
}
