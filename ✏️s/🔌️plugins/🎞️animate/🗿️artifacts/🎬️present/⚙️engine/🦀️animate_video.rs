//! 🎥️ Headless video engine: Vello frame capture, partial-movie cache, FFmpeg encode.

pub mod cache {
    use crate::artifacts::present::engine::animate_video::VideoError;
    use framework_hash::hash_bytes;
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// 💾️ Partial-movie cache keyed by animation hash with LRU eviction.
    pub struct PartialMovieCache {
        root: PathBuf,
        entries: HashMap<String, PathBuf>,
        access_order: Vec<String>,
        max_entries: usize,
    }

    impl PartialMovieCache {
        /// 📂️ Opens or creates a cache directory.
        pub fn open(root: impl Into<PathBuf>) -> Result<Self, VideoError> {
            Self::open_with_limit(root, usize::MAX)
        }

        /// 📂️ Opens a cache directory enforcing `max_entries` LRU eviction.
        pub fn open_with_limit(root: impl Into<PathBuf>, max_entries: usize) -> Result<Self, VideoError> {
            let root = root.into();
            fs::create_dir_all(&root).map_err(VideoError::io("cache dir"))?;
            let mut entries = HashMap::new();
            let mut access_order = Vec::new();
            if let Ok(read) = fs::read_dir(&root) {
                for entry in read.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().is_some_and(|ext| ext == "mp4") {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            access_order.push(stem.to_string());
                            entries.insert(stem.to_string(), path);
                        }
                    }
                }
            }
            let mut cache = Self { root, entries, access_order, max_entries: max_entries.max(1) };
            cache.evict_if_needed()?;
            Ok(cache)
        }

        /// 🔍️ Returns a cached partial movie path and marks the entry recently used.
        pub fn get(&mut self, hash: &str) -> Option<&Path> {
            if self.entries.contains_key(hash) {
                self.touch(hash);
                self.entries.get(hash).map(PathBuf::as_path)
            } else {
                None
            }
        }

        /// 💾️ Registers a rendered partial movie and evicts oldest entries when over capacity.
        pub fn insert(&mut self, hash: String, path: PathBuf) -> Result<(), VideoError> {
            if !self.entries.contains_key(&hash) {
                self.access_order.push(hash.clone());
            } else {
                self.touch(&hash);
            }
            self.entries.insert(hash, path);
            self.evict_if_needed()
        }

        /// 📁️ Cache root directory.
        pub fn root(&self) -> &Path {
            &self.root
        }

        /// 🧾️ Records cache metadata on disk.
        pub fn write_index(&self) -> Result<(), VideoError> {
            let index_path = self.root.join("index.json");
            let payload = serde_json::to_string_pretty(&self.access_order).map_err(VideoError::json("cache index"))?;
            fs::write(index_path, payload).map_err(VideoError::io("cache index"))?;
            Ok(())
        }

        /// 🪪️ Hash helper for partial segments.
        pub fn segment_hash(animation_hash: &str, frame_start: u32, frame_end: u32) -> String {
            hash_bytes(format!("{animation_hash}:{frame_start}:{frame_end}").as_bytes())
        }

        /// 🧹️ Removes all cached partial movies from disk.
        pub fn flush(root: impl Into<PathBuf>) -> Result<usize, VideoError> {
            let root = root.into();
            if !root.exists() {
                return Ok(0);
            }
            let mut removed = 0usize;
            if let Ok(read) = fs::read_dir(&root) {
                for entry in read.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        fs::remove_file(&path).map_err(VideoError::io("cache flush file"))?;
                        removed += 1;
                    } else if path.is_dir() {
                        fs::remove_dir_all(&path).map_err(VideoError::io("cache flush dir"))?;
                        removed += 1;
                    }
                }
            }
            let index_path = root.join("index.json");
            if index_path.exists() {
                fs::remove_file(index_path).map_err(VideoError::io("cache flush index"))?;
            }
            Ok(removed)
        }

        fn touch(&mut self, hash: &str) {
            self.access_order.retain(|entry| entry != hash);
            self.access_order.push(hash.to_string());
        }

        fn evict_if_needed(&mut self) -> Result<(), VideoError> {
            while self.access_order.len() > self.max_entries {
                let oldest = self.access_order.first().cloned().ok_or(VideoError::CacheEvictionEmpty)?;
                self.access_order.remove(0);
                if let Some(path) = self.entries.remove(&oldest) {
                    if path.is_dir() {
                        let _ = fs::remove_dir_all(&path);
                    } else if path.exists() {
                        let _ = fs::remove_file(&path);
                    }
                    if let Some(parent) = path.parent() {
                        if parent != self.root && parent.exists() {
                            let _ = fs::remove_dir(parent);
                        }
                    }
                }
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::{SystemTime, UNIX_EPOCH};

        #[test]
        fn segment_hash_is_stable() {
            let a = PartialMovieCache::segment_hash("abc", 0, 10);
            let b = PartialMovieCache::segment_hash("abc", 0, 10);
            assert_eq!(a, b);
            assert_ne!(a, PartialMovieCache::segment_hash("abc", 0, 11));
        }

        #[test]
        fn lru_evicts_oldest_entry() {
            let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let root = std::env::temp_dir().join(format!("animate_cache_lru_{stamp}"));
            let _ = fs::remove_dir_all(&root);
            let mut cache = PartialMovieCache::open_with_limit(&root, 2).expect("open");
            let first = root.join("first.mp4");
            let second = root.join("second.mp4");
            let third = root.join("third.mp4");
            fs::write(&first, b"a").expect("first");
            fs::write(&second, b"b").expect("second");
            fs::write(&third, b"c").expect("third");
            cache.insert("first".into(), first.clone()).expect("insert first");
            cache.insert("second".into(), second.clone()).expect("insert second");
            cache.get("first");
            cache.insert("third".into(), third.clone()).expect("insert third");
            assert!(!cache.entries.contains_key("second"));
            assert!(cache.entries.contains_key("first"));
            assert!(cache.entries.contains_key("third"));
            let _ = fs::remove_dir_all(&root);
        }
    }
}

pub mod preview {
    use crate::artifacts::present::engine::animate_video::VideoError;
    use crate::artifacts::present::engine::animate_core::{preview_scene_loop, AnimateConfig, Scene, SceneFrame};
    use std::io::Write;

    /// 🪟️ Live preview outcome.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PreviewOutcome {
        FrameLimit,
        WindowClosed,
        MetadataOnly,
    }

    /// 🖥️ Previews a scene in a wgpu window when `preview-window` is enabled, else logs frame metadata.
    pub fn preview_scene_window<S: Scene>(mut scene: S, config: &AnimateConfig, max_frames: Option<u64>) -> Result<PreviewOutcome, VideoError> {
        scene.setup(config);
        #[cfg(feature = "preview-window")]
        {
            return preview_scene_window_winit(scene, config, max_frames);
        }
        #[cfg(not(feature = "preview-window"))]
        {
            let outcome = preview_scene_window_metadata(&mut scene, max_frames)?;
            scene.tear_down();
            Ok(outcome)
        }
    }

    #[cfg(feature = "preview-window")]
    fn preview_scene_window_winit<S: Scene>(mut scene: S, config: &AnimateConfig, max_frames: Option<u64>) -> Result<PreviewOutcome, VideoError> {
        use crate::artifacts::present::engine::animate_video::renderer::{CapturedFrame, VelloRenderer};
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use winit::application::ApplicationHandler;
        use winit::event::WindowEvent;
        use winit::event_loop::{ActiveEventLoop, EventLoop};
        use winit::window::{Window, WindowId};

        struct PreviewApp<S> {
            scene: S,
            config: AnimateConfig,
            max_frames: u64,
            frame_index: u64,
            renderer: Option<VelloRenderer>,
            window: Option<Arc<Window>>,
            closed: Arc<AtomicBool>,
            constructed: bool,
            error: Option<VideoError>,
        }

        impl<S: Scene> PreviewApp<S> {
            fn fail(&mut self, error: VideoError) {
                self.error = Some(error);
            }
        }

        impl<S: Scene> ApplicationHandler for PreviewApp<S> {
            fn resumed(&mut self, event_loop: &ActiveEventLoop) {
                if self.window.is_some() {
                    return;
                }
                let window = match event_loop.create_window(Window::default_attributes().with_title("Animate Preview").with_inner_size(winit::dpi::LogicalSize::new(self.config.width, self.config.height))) {
                    Ok(window) => Arc::new(window),
                    Err(err) => {
                        self.fail(VideoError::backend("preview window", err));
                        event_loop.exit();
                        return;
                    }
                };
                match VelloRenderer::new(self.config.width, self.config.height) {
                    Ok(renderer) => self.renderer = Some(renderer),
                    Err(err) => {
                        self.fail(err);
                        event_loop.exit();
                        return;
                    }
                }
                self.window = Some(window.clone());
                if !self.constructed {
                    self.scene.construct();
                    self.constructed = true;
                }
                window.request_redraw();
            }

            fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
                match event {
                    WindowEvent::CloseRequested => {
                        self.closed.store(true, Ordering::Relaxed);
                        event_loop.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        if self.frame_index >= self.max_frames {
                            event_loop.exit();
                            return;
                        }
                        self.scene.sample_frame(self.config.frame_duration());
                        let capture = CapturedFrame { time: self.scene.scene_time(), mobjects: self.scene.mobjects().values().map(|m| m.clone_box()).collect() };
                        if let Some(renderer) = self.renderer.as_mut() {
                            let _ = renderer.render_capture(&capture, self.scene.camera(), &self.config);
                        }
                        let frame = self.scene.render_frame_index(self.frame_index);
                        if let Some(window) = self.window.as_ref() {
                            window.set_title(&format!("Animate Preview — frame {} t={:.2}s mobjects={} section={:?}", frame.frame, frame.time, frame.mobject_count, frame.section));
                            if self.frame_index + 1 < self.max_frames {
                                window.request_redraw();
                            } else {
                                event_loop.exit();
                            }
                        }
                        self.frame_index += 1;
                    }
                    _ => {}
                }
            }
        }

        let max = max_frames.unwrap_or(300);
        let mut app = PreviewApp { scene, config: config.clone(), max_frames: max, frame_index: 0, renderer: None, window: None, closed: Arc::new(AtomicBool::new(false)), constructed: false, error: None };
        let event_loop = EventLoop::new().map_err(|err| VideoError::backend("preview event loop", err))?;
        event_loop.run_app(&mut app).map_err(|err| VideoError::backend("preview run", err))?;
        app.scene.tear_down();
        if let Some(error) = app.error {
            return Err(error);
        }
        if app.closed.load(Ordering::Relaxed) {
            Ok(PreviewOutcome::WindowClosed)
        } else if app.frame_index >= max {
            Ok(PreviewOutcome::FrameLimit)
        } else {
            Ok(PreviewOutcome::WindowClosed)
        }
    }

    fn preview_scene_window_metadata<S: Scene>(scene: &mut S, max_frames: Option<u64>) -> Result<PreviewOutcome, VideoError> {
        let max = max_frames.unwrap_or(120);
        let mut stderr = std::io::stderr();
        preview_scene_loop(scene, max, |frame: &SceneFrame| {
            let _ = writeln!(stderr, "[animate-preview] frame={} time={:.3}s mobjects={} section={:?}", frame.frame, frame.time, frame.mobject_count, frame.section);
        });
        Ok(PreviewOutcome::MetadataOnly)
    }

    /// 🧪️ Headless preview used by CLI `--preview` flag.
    pub fn preview_scene_headless<S: Scene>(scene: S, config: &AnimateConfig, max_frames: Option<u64>) -> Result<PreviewOutcome, VideoError> {
        preview_scene_window(scene, config, max_frames)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::present::engine::animate_core::{BasicScene, Camera, Scene, SectionList, Sobject, VSobject};
        use std::collections::HashMap;

        struct DemoScene {
            base: BasicScene,
        }

        impl DemoScene {
            fn new(config: AnimateConfig) -> Self {
                Self { base: BasicScene::new(config) }
            }
        }

        impl Scene for DemoScene {
            fn construct(&mut self) {
                self.add(Box::new(VSobject::new()));
                self.wait(0.05);
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
        fn preview_scene_window_metadata_runs() {
            let config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(30.0);
            let scene = DemoScene::new(config.clone());
            let outcome = preview_scene_headless(scene, &config, Some(2)).expect("preview");
            assert_eq!(outcome, PreviewOutcome::MetadataOnly);
        }
    }
}

pub mod render {
    use crate::artifacts::present::engine::animate_core::{compile_animations, interpolate_at, AnimateConfig, Animation, Camera, Scene, SectionList, Sobject, Wait};
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use crate::artifacts::present::engine::animate_video::cache::PartialMovieCache;
    use crate::artifacts::present::engine::animate_video::renderer::{frame_hash, CapturedFrame, VelloRenderer};
    use crate::artifacts::present::engine::animate_video::writer::{write_sections_srt, SceneFileWriter};
    use crate::artifacts::present::engine::animate_video::VideoError;

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

    /// 🎬️ Renders any `Scene` implementation to configured outputs.
    pub fn render_scene<S: Scene>(mut scene: S, config: AnimateConfig, formats: &[OutputFormat]) -> Result<OutputPaths, VideoError> {
        scene.setup(&config);
        let mut recorder = FrameRecorder { inner: scene, captures: Vec::new() };
        recorder.construct();
        recorder.tear_down();
        if recorder.captures.is_empty() {
            recorder.capture_now();
        }

        let sections = recorder.inner.sections().clone();
        let sections_path = config.output_dir.join("sections.json");
        fs::create_dir_all(&config.output_dir).map_err(VideoError::io("output dir"))?;
        fs::write(&sections_path, serde_json::to_string_pretty(&sections).map_err(VideoError::json("sections json"))?).map_err(VideoError::io("sections write"))?;

        let camera = recorder.inner.camera().clone();
        let mut renderer = VelloRenderer::new(config.width, config.height)?;
        let mut writer = SceneFileWriter::new(&config, formats)?;
        let mut cache = if config.cache.enabled { Some(PartialMovieCache::open_with_limit(config.cache.partial_movie_dir.clone(), config.cache.max_entries)?) } else { None };

        let mut current_hash = String::new();
        let mut current_partial: Option<PathBuf> = None;
        let mut last_pixels: Option<Vec<u8>> = None;

        for (frame_index, capture) in recorder.captures.iter().enumerate() {
            let hash = frame_hash(capture, &config);
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
            let pixels = renderer.render_capture(capture, &camera, &config)?;
            if let Some(ref partial) = current_partial {
                writer.write_frame_png(partial, &pixels, frame_index as u32)?;
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
        use crate::artifacts::present::engine::animate_core::{BasicScene, Scene, VSobject};
        use std::time::{SystemTime, UNIX_EPOCH};

        struct DemoScene {
            base: BasicScene,
        }

        impl DemoScene {
            fn new(config: AnimateConfig) -> Self {
                Self { base: BasicScene::new(config) }
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
            let config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(15.0).with_output_dir(&dir).with_media_dir(dir.join("media"));
            let scene = DemoScene::new(config.clone());
            let outputs = render_scene(scene, config, &[OutputFormat::LastFrame]).expect("render");
            let last = outputs.last_frame.expect("last frame path");
            assert!(last.exists());
        }
    }
}

pub mod renderer {
    use crate::artifacts::present::engine::animate_video::VideoError;
    use crate::artifacts::present::engine::animate_core::{AnimateConfig, Camera, Color, Sobject};
    use pollster::block_on;
    use vello::kurbo::Stroke as KurboStroke;
    use vello::peniko::Color as VelloColor;
    use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};

    /// 🖼️ Captured mobject state at one timeline sample.
    pub struct CapturedFrame {
        pub time: f64,
        pub mobjects: Vec<Box<dyn Sobject>>,
    }

    /// 🖌️ Headless Vello/wgpu renderer with static-background caching.
    pub struct VelloRenderer {
        device: wgpu::Device,
        queue: wgpu::Queue,
        renderer: Renderer,
        width: u32,
        height: u32,
        target_texture: wgpu::Texture,
        target_view: wgpu::TextureView,
        readback_buffer: wgpu::Buffer,
        static_cache: Option<StaticBackgroundCache>,
    }

    struct StaticBackgroundCache {
        hash: String,
        pixels: Vec<u8>,
    }

    impl VelloRenderer {
        /// 🏗️ Creates a headless wgpu + Vello renderer at `width` × `height`.
        pub fn new(width: u32, height: u32) -> Result<Self, VideoError> {
            let width = width.max(1);
            let height = height.max(1);
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..Default::default() });
            let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false }))
                .map_err(|err| VideoError::backend("no wgpu adapter available", format!("{err:?}")))?;
            let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                label: Some("animate_video"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            }))
            .map_err(|err| VideoError::backend("wgpu device", format!("{err:?}")))?;
            let renderer = Renderer::new(&device, RendererOptions { use_cpu: false, antialiasing_support: AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None })
                .map_err(|err| VideoError::backend("vello renderer", format!("{err:?}")))?;
            let (target_texture, target_view) = create_target_texture(&device, width, height);
            let readback_buffer =
                device.create_buffer(&wgpu::BufferDescriptor { label: Some("animate_video_readback"), size: u64::from(width * height * 4), usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
            Ok(Self { device, queue, renderer, width, height, target_texture, target_view, readback_buffer, static_cache: None })
        }

        /// 🖼️ Renders captured mobjects to RGBA8 pixels.
        pub fn render_capture(&mut self, capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) -> Result<Vec<u8>, VideoError> {
            let static_hash = static_layer_hash(capture, config);
            if self.static_cache.as_ref().is_some_and(|cache| cache.hash == static_hash) {
                return Ok(self.static_cache.as_ref().expect("cache").pixels.clone());
            }
            let scene = build_vello_scene(capture, camera, config);
            let background = color_to_vello_array(config.background);
            let pixels = self.render_scene_to_pixels(&scene, background)?;
            self.static_cache = Some(StaticBackgroundCache { hash: static_hash, pixels: pixels.clone() });
            Ok(pixels)
        }

        fn render_scene_to_pixels(&mut self, scene: &Scene, background: VelloColor) -> Result<Vec<u8>, VideoError> {
            let params = RenderParams { base_color: background, width: self.width, height: self.height, antialiasing_method: AaConfig::Area };
            self.renderer.render_to_texture(&self.device, &self.queue, scene, &self.target_view, &params).map_err(|err| VideoError::backend("vello render", format!("{err:?}")))?;
            read_pixels(&self.device, &self.queue, &self.target_texture, &self.readback_buffer, self.width, self.height)
        }
    }

    fn create_target_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("animate_video_target"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    fn build_vello_scene(capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) -> Scene {
        let mut scene = Scene::new();
        let view = scene_affine(camera, config.width, config.height);
        let mut indices: Vec<usize> = (0..capture.mobjects.len()).collect();
        indices.sort_by_key(|&i| (capture.mobjects[i].z_order(), capture.mobjects[i].id()));
        for i in indices {
            paint_mobject(&mut scene, capture.mobjects[i].as_ref(), view);
        }
        scene
    }

    fn scene_affine(camera: &Camera, width: u32, height: u32) -> vello::kurbo::Affine {
        let sx = width as f64 / camera.frame_width;
        let sy = height as f64 / camera.frame_height;
        vello::kurbo::Affine::new([sx, 0.0, 0.0, -sy, width as f64 * 0.5 - camera.frame_center.x() * sx, height as f64 * 0.5 + camera.frame_center.y() * sy]) * camera.transform.to_kurbo()
    }

    fn paint_mobject(scene: &mut Scene, mobj: &dyn Sobject, view: vello::kurbo::Affine) {
        let transform = view * mobj.transform().to_kurbo();
        let style = mobj.style();
        let opacity = mobj.effective_opacity();
        for path in mobj.paths() {
            let shape = path.to_kurbo();
            if let Some(fill) = style.fill {
                let color = fill.with_alpha(fill.a * style.fill_opacity * opacity);
                scene.fill(vello::peniko::Fill::NonZero, transform, color_to_vello_array(color_from_style(color)), None, &shape);
            }
            if let Some(stroke) = style.stroke {
                let color = stroke.with_alpha(stroke.a * style.stroke_opacity * opacity);
                let stroke_style = KurboStroke::new(style.stroke_width);
                scene.stroke(&stroke_style, transform, color_to_vello_array(color_from_style(color)), None, &shape);
            }
        }
    }

    fn color_to_vello_array(rgba: [f64; 4]) -> VelloColor {
        VelloColor::new([rgba[0] as f32, rgba[1] as f32, rgba[2] as f32, rgba[3] as f32])
    }

    fn color_from_style(color: Color) -> [f64; 4] {
        color.to_array()
    }

    pub(crate) fn static_layer_hash(capture: &CapturedFrame, config: &AnimateConfig) -> String {
        use framework_hash::{format_number_for_hash, hash_parts};
        let mut parts = vec![format_number_for_hash(config.background[0]), format_number_for_hash(config.background[1]), format_number_for_hash(config.background[2]), format_number_for_hash(config.background[3]), capture.mobjects.len().to_string()];
        for mobj in &capture.mobjects {
            parts.push(mobj.id().to_string());
            parts.push(mobj.z_order().to_string());
            parts.push(format_number_for_hash(mobj.opacity()));
            parts.push(format_number_for_hash(mobj.point_ratio()));
            let coeffs = mobj.transform().to_kurbo().as_coeffs();
            for c in coeffs {
                parts.push(format_number_for_hash(c));
            }
            parts.push(mobj.paths().len().to_string());
            for path in mobj.paths() {
                parts.push(path.elements().len().to_string());
            }
        }
        hash_parts(&parts)
    }

    pub(crate) fn frame_hash(capture: &CapturedFrame, config: &AnimateConfig) -> String {
        use framework_hash::{format_number_for_hash, hash_parts};
        hash_parts(&[format_number_for_hash(capture.time), static_layer_hash(capture, config)])
    }

    fn read_pixels(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture, readback_buffer: &wgpu::Buffer, width: u32, height: u32) -> Result<Vec<u8>, VideoError> {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("animate_video_readback") });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: readback_buffer, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4 * width), rows_per_image: Some(height) } },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        queue.submit(Some(encoder.finish()));
        let slice = readback_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        let _ = device.poll(wgpu::PollType::wait_indefinitely());
        receiver.recv().map_err(|_| VideoError::ReadbackChannelClosed)?.map_err(|err| VideoError::backend("map async", format!("{err:?}")))?;
        let data = slice.get_mapped_range();
        let pixels = data.to_vec();
        drop(data);
        readback_buffer.unmap();
        Ok(pixels)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::present::engine::animate_core::VSobject;

        #[test]
        fn vello_renderer_produces_rgba_buffer() {
            let config = AnimateConfig::default().with_resolution(64, 64);
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            let mut capture = CapturedFrame { time: 0.0, mobjects: vec![Box::new(VSobject::new())] };
            let mut renderer = VelloRenderer::new(config.width, config.height).expect("renderer");
            let pixels = renderer.render_capture(&capture, &camera, &config).expect("frame");
            assert_eq!(pixels.len(), 64 * 64 * 4);
            capture.mobjects.clear();
            let empty = renderer.render_capture(&capture, &camera, &config).expect("empty");
            assert_eq!(empty.len(), 64 * 64 * 4);
        }
    }
}

pub mod scenes {
    //! 🎬️ Built-in scenes resolved by content hash for present/video export.

    use crate::artifacts::present::engine::animate_core::{AnimateConfig, BasicScene, Camera, Scene, Section, SectionList, Sobject, VSobject};
    use std::collections::HashMap;

    /// 🧩️ Demo scene used when no bespoke scene is registered for a hash.
    pub struct HashDemoScene {
        base: BasicScene,
        hash: String,
    }

    impl HashDemoScene {
        pub fn new(config: AnimateConfig, hash: impl Into<String>) -> Self {
            Self { base: BasicScene::new(config), hash: hash.into() }
        }
    }

    impl Scene for HashDemoScene {
        fn construct(&mut self) {
            self.add(Box::new(VSobject::new()));
            let label = format!("scene-{}", &self.hash[..self.hash.len().min(8)]);
            self.sections_mut().push(Section::new(label, 0.0, 0.2));
            self.wait(0.2);
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

    /// 🔍️ Builds the default scene implementation for a scene hash.
    pub fn scene_for_hash(config: AnimateConfig, scene_hash: &str) -> HashDemoScene {
        HashDemoScene::new(config, scene_hash)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn scene_for_hash_constructs() {
            let config = AnimateConfig::default().with_resolution(32, 32).with_frame_rate(15.0);
            let mut scene = scene_for_hash(config.clone(), "abc123");
            scene.setup(&config);
            scene.construct();
            assert!(!scene.mobjects().is_empty());
        }
    }
}

pub mod writer {
    use crate::artifacts::present::engine::animate_video::render::OutputFormat;
    use crate::artifacts::present::engine::animate_video::VideoError;
    use crate::artifacts::present::engine::animate_core::{AnimateConfig, SectionList};
    use image::{ImageBuffer, Rgba};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    /// 🧹️ Clears partial-movie cache directories from config.
    pub fn flush_partial_movie_cache(config: &AnimateConfig) -> Result<usize, VideoError> {
        crate::artifacts::present::engine::animate_video::cache::PartialMovieCache::flush(&config.cache.partial_movie_dir)
    }

    /// 📝️ Writes section timings as an SRT subtitle sidecar.
    pub fn write_sections_srt(sections: &SectionList, path: &Path) -> Result<(), VideoError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(VideoError::io("subtitle dir"))?;
        }
        let mut body = String::new();
        for (index, section) in sections.sections.iter().enumerate() {
            body.push_str(&(index + 1).to_string());
            body.push('\n');
            body.push_str(&format_srt_timestamp(section.start_time));
            body.push_str(" --> ");
            body.push_str(&format_srt_timestamp(section.end_time.max(section.start_time)));
            body.push('\n');
            body.push_str(&section.name);
            body.push_str("\n\n");
        }
        fs::write(path, body).map_err(VideoError::io("subtitle write"))
    }

    fn format_srt_timestamp(seconds: f64) -> String {
        let total_ms = (seconds.max(0.0) * 1000.0).round() as u64;
        let hours = total_ms / 3_600_000;
        let minutes = (total_ms % 3_600_000) / 60_000;
        let secs = (total_ms % 60_000) / 1000;
        let millis = total_ms % 1000;
        format!("{hours:02}:{minutes:02}:{secs:02},{millis:03}")
    }

    /// 📝️ Partial-movie writer with FFmpeg concat and sidecar outputs.
    pub struct SceneFileWriter {
        config: AnimateConfig,
        formats: Vec<OutputFormat>,
        partial_root: PathBuf,
        partial_paths: Vec<PathBuf>,
        png_sequence_dir: Option<PathBuf>,
        file_stem: String,
    }

    impl SceneFileWriter {
        /// 🏗️ Prepares writer directories from config.
        pub fn new(config: &AnimateConfig, formats: &[OutputFormat]) -> Result<Self, VideoError> {
            fs::create_dir_all(&config.output_dir).map_err(VideoError::io("output dir"))?;
            fs::create_dir_all(&config.media_dir).map_err(VideoError::io("media dir"))?;
            let partial_root = config.cache.partial_movie_dir.clone();
            fs::create_dir_all(&partial_root).map_err(VideoError::io("partial dir"))?;
            let png_sequence_dir = if formats.contains(&OutputFormat::PngSequence) {
                let dir = config.output_dir.join("frames");
                fs::create_dir_all(&dir).map_err(VideoError::io("png dir"))?;
                Some(dir)
            } else {
                None
            };
            Ok(Self { config: config.clone(), formats: formats.to_vec(), partial_root, partial_paths: Vec::new(), png_sequence_dir, file_stem: "scene".into() })
        }

        /// 🎬️ Begins a new partial movie directory for `hash`.
        pub fn begin_partial(&mut self, hash: &str, frame_start: u32) -> Result<PathBuf, VideoError> {
            let dir = self.partial_root.join(format!("{}_{frame_start}", &hash[..hash.len().min(12)]));
            fs::create_dir_all(&dir).map_err(VideoError::io("partial begin"))?;
            Ok(dir)
        }

        /// 🖼️ Writes one RGBA frame as PNG into a partial directory.
        pub fn write_frame_png(&mut self, partial_dir: &Path, pixels: &[u8], frame_index: u32) -> Result<(), VideoError> {
            let path = partial_dir.join(format!("{frame_index:06}.png"));
            write_png_file(&path, pixels, self.config.width, self.config.height)?;
            if let Some(dir) = &self.png_sequence_dir {
                let global = dir.join(format!("{frame_index:06}.png"));
                fs::copy(&path, &global).map_err(VideoError::io("png copy"))?;
            }
            Ok(())
        }

        /// ✅️ Encodes a partial PNG directory to mp4 and tracks it for concat.
        pub fn finalize_partial(&mut self, partial_dir: &Path) -> Result<PathBuf, VideoError> {
            let partial_mp4 = partial_dir.with_extension("mp4");
            run_ffmpeg(&["-y", "-framerate", &format_number(self.config.frame_rate), "-i", &partial_dir.join("%06d.png").display().to_string(), "-c:v", "libx264", "-pix_fmt", "yuv420p", &partial_mp4.display().to_string()])?;
            self.partial_paths.push(partial_mp4.clone());
            Ok(partial_mp4)
        }

        /// ♻️ Reuses a cached partial without re-encoding.
        pub fn register_cached_partial(&mut self, path: &Path) {
            if path.exists() {
                self.partial_paths.push(path.to_path_buf());
            }
        }

        /// 🎞️ Concatenates partial movies and emits configured sidecar outputs.
        pub fn encode_outputs(&self, last_frame: Option<&[u8]>) -> Result<super::render::OutputPaths, VideoError> {
            let mut outputs = super::render::OutputPaths::default();
            if self.formats.contains(&OutputFormat::Mp4) && !self.partial_paths.is_empty() {
                let mp4 = self.config.output_dir.join(format!("{}.mp4", self.file_stem));
                concat_partials(&self.partial_paths, &mp4)?;
                if let Some(audio) = &self.config.audio_track {
                    if audio.exists() {
                        let muxed = self.config.output_dir.join(format!("{}_with_audio.mp4", self.file_stem));
                        mux_audio_track(&mp4, audio, &muxed)?;
                        outputs.mp4 = Some(muxed);
                    } else {
                        outputs.mp4 = Some(mp4);
                    }
                } else {
                    outputs.mp4 = Some(mp4);
                }
            }
            if self.formats.contains(&OutputFormat::Gif) {
                if let Some(mp4) = &outputs.mp4 {
                    let gif = self.config.output_dir.join(format!("{}.gif", self.file_stem));
                    run_ffmpeg(&["-y", "-i", &mp4.display().to_string(), "-vf", "fps=15,scale=640:-1:flags=lanczos", &gif.display().to_string()])?;
                    outputs.gif = Some(gif);
                }
            }
            if self.formats.contains(&OutputFormat::LastFrame) {
                if let Some(pixels) = last_frame {
                    let png = self.config.output_dir.join(format!("{}.png", self.file_stem));
                    write_png_file(&png, pixels, self.config.width, self.config.height)?;
                    outputs.last_frame = Some(png);
                }
            }
            outputs.png_dir = self.png_sequence_dir.clone();
            Ok(outputs)
        }
    }

    fn format_number(value: f64) -> String {
        use framework_hash::format_number_for_hash;
        format_number_for_hash(value)
    }

    fn write_png_file(path: &Path, pixels: &[u8], width: u32, height: u32) -> Result<(), VideoError> {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, pixels.to_vec()).ok_or(VideoError::InvalidRgbaBuffer)?;
        image.save(path).map_err(|err| VideoError::backend("png write", err))
    }

    fn concat_partials(partials: &[PathBuf], output: &Path) -> Result<(), VideoError> {
        if partials.len() == 1 {
            fs::copy(&partials[0], output).map_err(VideoError::io("copy partial"))?;
            return Ok(());
        }
        let list_path = output.with_extension("txt");
        let mut list = String::new();
        for partial in partials {
            list.push_str(&format!("file '{}'\n", partial.display()));
        }
        fs::write(&list_path, list).map_err(VideoError::io("concat list"))?;
        run_ffmpeg(&["-y", "-f", "concat", "-safe", "0", "-i", &list_path.display().to_string(), "-c", "copy", &output.display().to_string()])
    }

    fn mux_audio_track(video: &Path, audio: &Path, output: &Path) -> Result<(), VideoError> {
        run_ffmpeg(&["-y", "-i", &video.display().to_string(), "-i", &audio.display().to_string(), "-c:v", "copy", "-c:a", "aac", "-shortest", &output.display().to_string()])
    }

    fn run_ffmpeg(args: &[&str]) -> Result<(), VideoError> {
        let status = Command::new("ffmpeg").args(args).status().map_err(VideoError::io("ffmpeg spawn"))?;
        if status.success() {
            Ok(())
        } else {
            Err(VideoError::FfmpegStatus(status))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::present::engine::animate_video::render::OutputFormat;
        use std::time::{SystemTime, UNIX_EPOCH};

        fn temp_config() -> AnimateConfig {
            let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let dir = std::env::temp_dir().join(format!("animate_video_test_{stamp}"));
            AnimateConfig::default().with_resolution(16, 16).with_output_dir(&dir).with_media_dir(dir.join("media"))
        }

        #[test]
        fn writer_writes_srt_from_sections() {
            let config = temp_config();
            let sections = crate::artifacts::present::engine::animate_core::SectionList::default();
            let path = config.output_dir.join("scene.srt");
            write_sections_srt(&sections, &path).expect("srt");
            assert!(path.exists());
        }

        #[test]
        fn writer_writes_png_frame() {
            let config = temp_config();
            let mut writer = SceneFileWriter::new(&config, &[OutputFormat::LastFrame]).expect("writer");
            let partial = writer.begin_partial("hash", 0).expect("partial");
            let pixels = vec![255u8; 16 * 16 * 4];
            writer.write_frame_png(&partial, &pixels, 0).expect("frame");
            assert!(partial.join("000000.png").exists());
        }
    }
}

pub use cache::PartialMovieCache;
pub use preview::{preview_scene_headless, preview_scene_window, PreviewOutcome};
pub use render::{render_scene, OutputFormat, OutputPaths};
pub use renderer::VelloRenderer;
pub use scenes::scene_for_hash;
pub use writer::{flush_partial_movie_cache, write_sections_srt, SceneFileWriter};

//#region 🔖️Error
/// 🎬️ Errors from headless video rendering, caching, and encoding.
#[derive(Debug, thiserror::Error)]
pub enum VideoError {
    /// 📁️ A filesystem operation (create/read/write/remove) failed.
    #[error("{context}: {source}")]
    Io {
        context: &'static str,
        #[source]
        source: std::io::Error,
    },
    /// 🧾️ JSON (de)serialization failed.
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
    /// 📡️ GPU readback channel closed before a result arrived.
    #[error("readback channel closed")]
    ReadbackChannelClosed,
    /// 🖥️ wgpu/vello/window subsystem failure, message from the underlying backend.
    #[error("{context}: {message}")]
    Backend { context: &'static str, message: String },
}

impl VideoError {
    /// 📁️ Curries an io::Error mapper tagged with `context` for `.map_err(...)`.
    pub(crate) fn io(context: &'static str) -> impl Fn(std::io::Error) -> Self {
        move |source| Self::Io { context, source }
    }
    /// 🧾️ Curries a serde_json::Error mapper tagged with `context` for `.map_err(...)`.
    pub(crate) fn json(context: &'static str) -> impl Fn(serde_json::Error) -> Self {
        move |source| Self::Json { context, source }
    }
    /// 🖥️ Builds a backend-failure variant from any Display/Debug-formatted foreign error.
    pub(crate) fn backend(context: &'static str, message: impl std::fmt::Display) -> Self {
        Self::Backend { context, message: message.to_string() }
    }
}
//#endregion 🔖️Error
