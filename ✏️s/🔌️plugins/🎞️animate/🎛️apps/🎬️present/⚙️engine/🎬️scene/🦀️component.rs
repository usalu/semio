//! 🎞️ Animate app engine facet: 🎬️scene (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/🎬️scene`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod scene {
    //! 🎭️ Scene trait with construct/play/wait timeline and frame loop.

    use crate::apps::present::engine::animation::animation::{apply_parent_opacity_tree, compile_animations, interpolate_at, Animation, Wait};
    use crate::apps::present::engine::camera::camera::{Camera, MovingCamera, ThreeDCamera, ZoomedCamera};
    use crate::apps::present::engine::config::config::AnimateConfig;
    use crate::apps::present::engine::scene::section::SectionList;
    use crate::apps::present::engine::scene::sobject::Sobject;
    use crate::apps::present::engine::rate::updater::run_updaters;
    use std::collections::HashMap;

    /// 🎬️ User-authored animation scene contract.
    pub trait Scene {
        fn construct(&mut self);

        fn setup(&mut self, _config: &AnimateConfig) {}

        fn tear_down(&mut self) {}

        fn config(&self) -> &AnimateConfig;

        fn config_mut(&mut self) -> &mut AnimateConfig;

        fn camera(&self) -> &Camera;

        fn camera_mut(&mut self) -> &mut Camera;

        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>>;

        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>>;

        fn sections(&self) -> &SectionList;

        fn sections_mut(&mut self) -> &mut SectionList;

        fn scene_time(&self) -> f64;

        fn set_scene_time(&mut self, time: f64);

        fn add(&mut self, mobject: Box<dyn Sobject>) {
            let id = mobject.id();
            self.mobjects_mut().insert(id, mobject);
        }

        fn remove(&mut self, id: u64) {
            self.mobjects_mut().remove(&id);
        }

        fn play(&mut self, mut animation: Box<dyn Animation>) {
            let pending_introducers = if animation.is_introducer() { animation.get_all_mobjects() } else { Vec::new() };
            for id in &pending_introducers {
                debug_assert!(self.mobjects().contains_key(id), "introducer animation requires mobject {id} to exist in scene");
            }
            let remover_ids = if animation.is_remover() { animation.get_all_mobjects() } else { Vec::new() };
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
            for id in remover_ids {
                self.remove(id);
            }
        }

        fn begin_section(&mut self, name: impl Into<String>) {
            self.sections_mut().begin_section(name, false);
        }

        fn next_section(&mut self, name: impl Into<String>) {
            let t = self.scene_time();
            self.sections_mut().end_section(t);
            self.sections_mut().begin_section(name, false);
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
            let t = self.scene_time() + dt;
            self.set_scene_time(t);
            for m in self.mobjects_mut().values_mut() {
                apply_parent_opacity_tree(m.as_mut(), 1.0);
                run_updaters(m.as_mut(), dt);
            }
        }

        fn render_frame_index(&self, frame: u64) -> SceneFrame {
            SceneFrame { frame, time: frame as f64 / self.config().frame_rate, mobject_count: self.mobjects().len(), section: self.sections().find_at_time(self.scene_time()).map(|s| s.name.clone()) }
        }
    }

    /// 🖼️ Lightweight frame snapshot metadata for renderers.
    #[derive(Clone, Debug)]
    pub struct SceneFrame {
        pub frame: u64,
        pub time: f64,
        pub mobject_count: usize,
        pub section: Option<String>,
    }

    /// 🔁️ Interactive preview loop sampling construct timeline without encoding.
    pub fn preview_scene_loop<S: Scene>(scene: &mut S, max_frames: u64, mut on_frame: impl FnMut(&SceneFrame)) {
        let config = scene.config().clone();
        scene.setup(&config);
        let fps = config.frame_rate;
        let dt = 1.0 / fps.max(1.0);
        scene.construct();
        for frame in 0..max_frames {
            scene.sample_frame(dt);
            on_frame(&scene.render_frame_index(frame));
        }
        scene.tear_down();
    }

    /// 🏗️ Default scene implementation backing most user scenes.
    pub struct BasicStage {
        pub config: AnimateConfig,
        pub camera: Camera,
        pub mobjects: HashMap<u64, Box<dyn Sobject>>,
        pub sections: SectionList,
        pub scene_time: f64,
    }

    impl BasicStage {
        pub fn new(config: AnimateConfig) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 }
        }

        pub fn run_construct<S: Scene>(&mut self, scene: &mut S) {
            scene.setup(&self.config);
            scene.construct();
            scene.tear_down();
        }
    }

    impl Scene for BasicStage {
        fn construct(&mut self) {}

        fn config(&self) -> &AnimateConfig {
            &self.config
        }

        fn config_mut(&mut self) -> &mut AnimateConfig {
            &mut self.config
        }

        fn camera(&self) -> &Camera {
            &self.camera
        }

        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.camera
        }

        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            &self.mobjects
        }

        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            &mut self.mobjects
        }

        fn sections(&self) -> &SectionList {
            &self.sections
        }

        fn sections_mut(&mut self) -> &mut SectionList {
            &mut self.sections
        }

        fn scene_time(&self) -> f64 {
            self.scene_time
        }

        fn set_scene_time(&mut self, time: f64) {
            self.scene_time = time;
        }
    }

    /// 🧪️ Specialized scene for unit tests with fixed frame rate.
    pub struct TestScene {
        inner: BasicStage,
    }

    impl TestScene {
        pub fn new() -> Self {
            Self { inner: BasicStage::new(AnimateConfig::default().with_frame_rate(60.0)) }
        }
    }

    impl Default for TestScene {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Scene for TestScene {
        fn construct(&mut self) {}
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
    }

    /// 🎥️ Scene with a panning/zooming {@link MovingCamera}.
    pub struct MovingCameraScene {
        inner: BasicStage,
        pub moving_camera: MovingCamera,
    }

    impl MovingCameraScene {
        pub fn new(config: AnimateConfig) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { moving_camera: MovingCamera::new(camera.clone()), inner: BasicStage { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 } }
        }
    }

    impl Scene for MovingCameraScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            &self.moving_camera.camera
        }
        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.moving_camera.camera
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
        fn sample_frame(&mut self, dt: f64) {
            self.moving_camera.interpolate((self.scene_time() * self.config().frame_rate).fract());
            self.inner.sample_frame(dt);
        }
    }

    /// 🧊️ Scene using a perspective {@link ThreeDCamera}.
    pub struct ThreeDScene {
        inner: BasicStage,
        pub three_d_camera: ThreeDCamera,
    }

    impl ThreeDScene {
        pub fn new(config: AnimateConfig) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { three_d_camera: ThreeDCamera::new(camera.clone()), inner: BasicStage { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 } }
        }
    }

    impl Scene for ThreeDScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            &self.three_d_camera.camera
        }
        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.three_d_camera.camera
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
    }

    /// 🔍️ Scene with a picture-in-picture {@link ZoomedCamera} inset.
    pub struct ZoomedScene {
        inner: BasicStage,
        pub zoomed_camera: ZoomedCamera,
    }

    impl ZoomedScene {
        pub fn new(config: AnimateConfig, zoom_factor: f64) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { zoomed_camera: ZoomedCamera::new(camera.clone(), zoom_factor), inner: BasicStage { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 } }
        }
    }

    impl Scene for ZoomedScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            &self.zoomed_camera.camera
        }
        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.zoomed_camera.camera
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
    }

    /// ➡️ Scene for vector-field and linear-transformation animations.
    pub struct VectorScene {
        inner: BasicStage,
    }

    impl VectorScene {
        pub fn new(config: AnimateConfig) -> Self {
            Self { inner: BasicStage::new(config) }
        }
    }

    impl Scene for VectorScene {
        fn construct(&mut self) {}
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::apps::present::engine::scene::sobject::VSobject;

        struct DemoScene {
            base: TestScene,
        }

        impl DemoScene {
            fn new() -> Self {
                Self { base: TestScene::new() }
            }
        }

        impl Scene for DemoScene {
            fn construct(&mut self) {
                self.add(Box::new(VSobject::new()));
                self.wait(0.5);
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
        fn preview_loop_samples_frames() {
            let mut s = DemoScene::new();
            let mut frames = 0u64;
            preview_scene_loop(&mut s, 3, |_| {
                frames += 1;
            });
            assert_eq!(frames, 3);
        }
    }
}

pub mod section {
    //! 📑️ Named sections for partial movie output and navigation.

    use serde::{Deserialize, Serialize};

    /// 🏷️ Single named section within a scene timeline.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Section {
        pub name: String,
        pub start_time: f64,
        pub end_time: f64,
        pub skip_animations: bool,
    }

    impl Section {
        pub fn new(name: impl Into<String>, start_time: f64, end_time: f64) -> Self {
            Self { name: name.into(), start_time, end_time, skip_animations: false }
        }

        pub fn duration(&self) -> f64 {
            (self.end_time - self.start_time).max(0.0)
        }

        pub fn contains_time(&self, t: f64) -> bool {
            t >= self.start_time && t <= self.end_time
        }
    }

    /// 📚️ Ordered section list attached to a scene.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct SectionList {
        pub sections: Vec<Section>,
        open: Option<Section>,
    }

    impl SectionList {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn begin_section(&mut self, name: impl Into<String>, skip_animations: bool) {
            self.open = Some(Section { name: name.into(), start_time: 0.0, end_time: 0.0, skip_animations });
        }

        pub fn end_section(&mut self, end_time: f64) {
            if let Some(mut s) = self.open.take() {
                s.end_time = end_time;
                self.sections.push(s);
            }
        }

        pub fn push(&mut self, section: Section) {
            self.sections.push(section);
        }

        pub fn find_at_time(&self, t: f64) -> Option<&Section> {
            self.sections.iter().find(|s| s.contains_time(t))
        }

        pub fn names(&self) -> Vec<&str> {
            self.sections.iter().map(|s| s.name.as_str()).collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn section_duration_is_non_negative() {
            let s = Section::new("intro", 0.0, 2.5);
            assert!((s.duration() - 2.5).abs() < 1e-9);
        }

        #[test]
        fn section_list_tracks_open_close() {
            let mut list = SectionList::new();
            list.begin_section("main", false);
            list.end_section(10.0);
            assert_eq!(list.sections.len(), 1);
            assert_eq!(list.sections[0].name, "main");
        }
    }
}

pub mod sobject {
    //! 🧩️ Sobject trait, VSobject paths, groups, transforms, and layout helpers.

    use crate::apps::present::engine::text::color::Color;
    use crate::apps::present::engine::rate::updater::Updater;
    use kurbo::{ParamCurve, ParamCurveArclen, PathSeg, Shape};
    use geometry::{append_shape_to_path, bounding_box, polygon_centroid, Affine, BezPath, PathEl, Point, Vec2};


    fn next_id() -> u64 {
        ({ u64::from_str_radix(&blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex()[..8], 16).unwrap_or(1) })
    }

    /// 🎨️ Stroke and fill style for vector Sobjects.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Style {
        pub fill: Option<Color>,
        pub stroke: Option<Color>,
        pub fill_opacity: f64,
        pub stroke_opacity: f64,
        pub stroke_width: f64,
    }

    impl Default for Style {
        fn default() -> Self {
            Self { fill: Some(Color::WHITE), stroke: None, fill_opacity: 1.0, stroke_opacity: 1.0, stroke_width: 4.0 }
        }
    }

    /// 📐️ Axis-aligned bounds in scene space.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Bounds {
        pub min: Point,
        pub max: Point,
    }

    impl Bounds {
        pub fn center(self) -> Point {
            Point::new((self.min.x() + self.max.x()) / 2.0, (self.min.y() + self.max.y()) / 2.0)
        }

        pub fn width(self) -> f64 {
            self.max.x() - self.min.x()
        }

        pub fn height(self) -> f64 {
            self.max.y() - self.min.y()
        }

        pub fn empty() -> Self {
            Self { min: Point::ZERO, max: Point::ZERO }
        }
    }

    /// 🧬️ Base scene-graph object contract.
    pub trait Sobject: Send {
        fn id(&self) -> u64;
        fn name(&self) -> &str;
        fn set_name(&mut self, name: String);
        fn style(&self) -> &Style;
        fn style_mut(&mut self) -> &mut Style;
        fn opacity(&self) -> f64;
        fn set_opacity(&mut self, opacity: f64);
        fn effective_opacity(&self) -> f64;
        fn set_parent_opacity(&mut self, parent: f64);
        fn transform(&self) -> Affine;
        fn transform_mut(&mut self) -> &mut Affine;
        fn bounds(&self) -> Bounds;
        fn center(&self) -> Point {
            self.bounds().center()
        }
        fn shift(&mut self, delta: Vec2) {
            *self.transform_mut() = self.transform() * Affine::IDENTITY.translate(delta);
        }
        fn move_to(&mut self, point: Point) {
            let c = self.center();
            self.shift(point - c);
        }
        fn scale(&mut self, factor: f64) {
            let c = self.center();
            let t = Affine::IDENTITY.translate(c.to_vec2()) * Affine::IDENTITY.scale(factor) * Affine::IDENTITY.translate(-c.to_vec2());
            *self.transform_mut() = self.transform() * t;
        }
        fn rotate(&mut self, angle: f64) {
            let c = self.center();
            let t = Affine::IDENTITY.translate(c.to_vec2()) * Affine::IDENTITY.rotate(angle) * Affine::IDENTITY.translate(-c.to_vec2());
            *self.transform_mut() = self.transform() * t;
        }
        fn set_color(&mut self, color: Color) {
            self.style_mut().fill = Some(color);
            self.style_mut().stroke = Some(color);
        }
        fn set_fill(&mut self, color: Color) {
            self.style_mut().fill = Some(color);
        }
        fn set_stroke(&mut self, color: Color, width: f64) {
            self.style_mut().stroke = Some(color);
            self.style_mut().stroke_width = width;
        }
        fn paths(&self) -> Vec<BezPath>;
        fn children(&self) -> Vec<&dyn Sobject>;
        fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Sobject));
        fn add_child(&mut self, child: Box<dyn Sobject>);
        fn updaters(&self) -> &[Updater];
        fn updaters_mut(&mut self) -> &mut Vec<Updater>;
        fn save_state(&mut self);
        fn restore(&mut self);
        fn generate_target(&mut self);
        fn has_target(&self) -> bool;
        fn apply_target(&mut self);
        fn clone_box(&self) -> Box<dyn Sobject>;
        fn as_any(&self) -> &dyn std::any::Any;
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
        fn z_order(&self) -> i64 {
            0
        }
        fn set_z_order(&mut self, _z: i64) {}
        fn point_ratio(&self) -> f64 {
            1.0
        }
    }

    /// ✏️ Vector Sobject backed by kurbo paths and partial point reveal.
    #[derive(Clone)]
    pub struct VSobject {
        pub id: u64,
        pub name: String,
        pub paths: Vec<BezPath>,
        pub style: Style,
        pub opacity: f64,
        pub parent_opacity: f64,
        pub transform: Affine,
        pub point_ratio: f64,
        pub z_order: i64,
        pub saved: Option<VSobjectSnapshot>,
        pub target: Option<VSobjectSnapshot>,
        pub updaters: Vec<Updater>,
    }

    #[derive(Clone, Debug)]
    pub struct VSobjectSnapshot {
        paths: Vec<BezPath>,
        style: Style,
        opacity: f64,
        transform: Affine,
        point_ratio: f64,
    }

    impl VSobject {
        pub fn new() -> Self {
            Self { id: next_id(), name: String::new(), paths: Vec::new(), style: Style::default(), opacity: 1.0, parent_opacity: 1.0, transform: Affine::IDENTITY, point_ratio: 1.0, z_order: 0, saved: None, target: None, updaters: Vec::new() }
        }

        pub fn from_path(path: BezPath) -> Self {
            let mut s = Self::new();
            s.paths.push(path);
            s
        }

        pub fn from_shape<'a>(shape: impl Into<geometry::ShapeRef<'a>>) -> Self {
            let mut path = BezPath::new();
            append_shape_to_path(&mut path, shape, 0.01);
            Self::from_path(path)
        }

        pub fn set_paths(&mut self, paths: Vec<BezPath>) {
            self.paths = paths;
        }

        pub fn set_point_ratio(&mut self, ratio: f64) {
            self.point_ratio = ratio.clamp(0.0, 1.0);
        }

        fn snapshot(&self) -> VSobjectSnapshot {
            VSobjectSnapshot { paths: self.paths.clone(), style: self.style.clone(), opacity: self.opacity, transform: self.transform, point_ratio: self.point_ratio }
        }

        fn restore_snapshot(&mut self, snap: VSobjectSnapshot) {
            self.paths = snap.paths;
            self.style = snap.style;
            self.opacity = snap.opacity;
            self.transform = snap.transform;
            self.point_ratio = snap.point_ratio;
        }

        /// 🔀️ Linearly blend two snapshots into the live VSobject state.
        pub fn interpolate_snapshots(&mut self, from: &VSobjectSnapshot, to: &VSobjectSnapshot, t: f64) {
            let t = t.clamp(0.0, 1.0);
            self.opacity = lerp_f64(from.opacity, to.opacity, t);
            self.transform = lerp_affine(from.transform, to.transform, t);
            self.point_ratio = lerp_f64(from.point_ratio, to.point_ratio, t);
            self.style = if t < 0.5 { from.style.clone() } else { to.style.clone() };
            self.paths = interpolate_path_sets(&from.paths, &to.paths, t);
        }

        /// 🎯️ Blend from saved state toward the generated target.
        pub fn interpolate_saved_to_target(&mut self, t: f64) {
            if self.saved.is_none() {
                self.save_state();
            }
            if !self.has_target() {
                self.generate_target();
            }
            let from = self.saved.clone();
            let to = self.target.clone();
            if let (Some(from), Some(to)) = (from, to) {
                self.interpolate_snapshots(&from, &to, t);
            }
        }
    }

    fn lerp_f64(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }

    /// ✂️ Trim a path to a partial reveal ratio in [0,1].
    pub fn trim_path_at_ratio(path: &BezPath, ratio: f64) -> BezPath {
        let ratio = ratio.clamp(0.0, 1.0);
        if ratio >= 1.0 {
            return path.clone();
        }
        if ratio <= 0.0 {
            return BezPath::new();
        }
        let kurbo = path.to_kurbo();
        let segments: Vec<PathSeg> = kurbo.path_segments(0.25).collect();
        let total: f64 = segments.iter().map(|s| s.arclen(0.25)).sum();
        if total <= 1e-12 {
            return path.clone();
        }
        let mut remaining = total * ratio;
        let mut out_k = kurbo::BezPath::new();
        for seg in segments {
            let len = seg.arclen(0.25);
            if remaining >= len - 1e-9 {
                if out_k.is_empty() {
                    out_k.move_to(seg.start());
                }
                out_k.push(seg.as_path_el());
                remaining -= len;
            } else {
                let frac = (remaining / len).clamp(0.0, 1.0);
                let sub = seg.subsegment(0.0..frac);
                if out_k.is_empty() {
                    out_k.move_to(sub.start());
                }
                out_k.push(sub.as_path_el());
                break;
            }
        }
        bezpath_from_kurbo(&out_k)
    }

    fn bezpath_from_kurbo(k: &kurbo::BezPath) -> BezPath {
        let mut out = BezPath::new();
        for el in k.elements() {
            out.push(PathEl::from(*el));
        }
        out
    }

    fn interpolate_path_sets(from: &[BezPath], to: &[BezPath], t: f64) -> Vec<BezPath> {
        if from.is_empty() {
            return to.to_vec();
        }
        if to.is_empty() {
            return from.to_vec();
        }
        let count = from.len().max(to.len());
        (0..count)
            .map(|i| {
                let a = &from[i.min(from.len() - 1)];
                let b = &to[i.min(to.len() - 1)];
                interpolate_bezpaths(a, b, t)
            })
            .collect()
    }

    fn interpolate_bezpaths(from: &BezPath, to: &BezPath, t: f64) -> BezPath {
        let fa = resample_points(&sample_path_points(from, 32), 32);
        let tb = resample_points(&sample_path_points(to, 32), 32);
        let mut out = BezPath::new();
        for (i, (a, b)) in fa.iter().zip(tb.iter()).enumerate() {
            let p = Point::new(lerp_f64(a.x(), b.x(), t), lerp_f64(a.y(), b.y(), t));
            if i == 0 {
                out.move_to(p);
            } else {
                out.line_to(p);
            }
        }
        out
    }

    fn sample_path_points(path: &BezPath, samples: usize) -> Vec<Point> {
        let kurbo = path.to_kurbo();
        let segments: Vec<PathSeg> = kurbo.path_segments(0.25).collect();
        let total: f64 = segments.iter().map(|s| s.arclen(0.25)).sum();
        if total <= 1e-12 {
            return Vec::new();
        }
        let mut pts = Vec::with_capacity(samples);
        for i in 0..samples {
            let target = total * i as f64 / (samples - 1).max(1) as f64;
            let mut acc = 0.0;
            for (idx, seg) in segments.iter().enumerate() {
                let len = seg.arclen(0.25);
                if acc + len >= target || idx + 1 == segments.len() {
                    let local = ((target - acc) / len.max(1e-12)).clamp(0.0, 1.0);
                    let p = seg.eval(local);
                    pts.push(Point::new(p.x, p.y));
                    break;
                }
                acc += len;
            }
        }
        pts
    }

    fn resample_points(pts: &[Point], count: usize) -> Vec<Point> {
        if pts.is_empty() {
            return vec![Point::ZERO; count];
        }
        if count <= 1 {
            return vec![pts[0]];
        }
        (0..count)
            .map(|i| {
                let idx = i * (pts.len() - 1) / (count - 1);
                pts[idx]
            })
            .collect()
    }

    fn lerp_affine(a: Affine, b: Affine, t: f64) -> Affine {
        let ta = a.to_kurbo().as_coeffs();
        let tb = b.to_kurbo().as_coeffs();
        let t = t.clamp(0.0, 1.0);
        Affine::new([lerp_f64(ta[0], tb[0], t), lerp_f64(ta[1], tb[1], t), lerp_f64(ta[2], tb[2], t), lerp_f64(ta[3], tb[3], t), lerp_f64(ta[4], tb[4], t), lerp_f64(ta[5], tb[5], t)])
    }

    impl Default for VSobject {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Sobject for VSobject {
        fn id(&self) -> u64 {
            self.id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn set_name(&mut self, name: String) {
            self.name = name;
        }
        fn style(&self) -> &Style {
            &self.style
        }
        fn style_mut(&mut self) -> &mut Style {
            &mut self.style
        }
        fn opacity(&self) -> f64 {
            self.opacity
        }
        fn set_opacity(&mut self, opacity: f64) {
            self.opacity = opacity.clamp(0.0, 1.0);
        }
        fn effective_opacity(&self) -> f64 {
            self.opacity * self.parent_opacity
        }
        fn set_parent_opacity(&mut self, parent: f64) {
            self.parent_opacity = parent.clamp(0.0, 1.0);
        }
        fn transform(&self) -> Affine {
            self.transform
        }
        fn transform_mut(&mut self) -> &mut Affine {
            &mut self.transform
        }
        fn bounds(&self) -> Bounds {
            let mut pts = Vec::new();
            for path in &self.paths {
                for el in path.elements() {
                    if let Some(p) = el.as_ref_point() {
                        pts.push(self.transform * p);
                    }
                }
            }
            if let Some(bb) = bounding_box(&pts) {
                Bounds { min: Point::new(bb.min_x, bb.min_y), max: Point::new(bb.max_x, bb.max_y) }
            } else {
                Bounds::empty()
            }
        }
        fn paths(&self) -> Vec<BezPath> {
            let t = self.transform.to_kurbo();
            self.paths
                .iter()
                .map(|p| {
                    let trimmed = trim_path_at_ratio(p, self.point_ratio);
                    transform_bezpath(&trimmed, t)
                })
                .collect()
        }
        fn children(&self) -> Vec<&dyn Sobject> {
            Vec::new()
        }
        fn visit_children_mut(&mut self, _f: &mut dyn FnMut(&mut dyn Sobject)) {}
        fn add_child(&mut self, _child: Box<dyn Sobject>) {}
        fn updaters(&self) -> &[Updater] {
            &self.updaters
        }
        fn updaters_mut(&mut self) -> &mut Vec<Updater> {
            &mut self.updaters
        }
        fn save_state(&mut self) {
            self.saved = Some(self.snapshot());
        }
        fn restore(&mut self) {
            if let Some(s) = self.saved.take() {
                self.restore_snapshot(s);
            }
        }
        fn generate_target(&mut self) {
            self.target = Some(self.snapshot());
        }
        fn has_target(&self) -> bool {
            self.target.is_some()
        }
        fn apply_target(&mut self) {
            if let Some(t) = self.target.take() {
                self.restore_snapshot(t);
            }
        }
        fn clone_box(&self) -> Box<dyn Sobject> {
            Box::new(self.clone())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn z_order(&self) -> i64 {
            self.z_order
        }
        fn set_z_order(&mut self, z: i64) {
            self.z_order = z;
        }
        fn point_ratio(&self) -> f64 {
            self.point_ratio
        }
    }

    /// 📦️ Group of heterogeneous Sobjects.
    pub struct Group {
        pub id: u64,
        pub name: String,
        pub children: Vec<Box<dyn Sobject>>,
        pub style: Style,
        pub opacity: f64,
        pub parent_opacity: f64,
        pub transform: Affine,
        pub z_order: i64,
        pub saved: Option<GroupSnapshot>,
        pub target: Option<GroupSnapshot>,
        pub updaters: Vec<Updater>,
    }

    #[derive(Clone, Debug)]
    pub struct GroupSnapshot {
        opacity: f64,
        transform: Affine,
    }

    impl Group {
        pub fn new(children: Vec<Box<dyn Sobject>>) -> Self {
            Self { id: next_id(), name: String::new(), children, style: Style::default(), opacity: 1.0, parent_opacity: 1.0, transform: Affine::IDENTITY, z_order: 0, saved: None, target: None, updaters: Vec::new() }
        }

        pub fn empty() -> Self {
            Self::new(Vec::new())
        }

        fn propagate_parent_opacity(&mut self) {
            let eff = self.effective_opacity();
            for child in &mut self.children {
                child.set_parent_opacity(eff);
                if let Some(g) = child.as_any_mut().downcast_mut::<Group>() {
                    g.propagate_parent_opacity();
                }
            }
        }
    }

    impl Sobject for Group {
        fn id(&self) -> u64 {
            self.id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn set_name(&mut self, name: String) {
            self.name = name;
        }
        fn style(&self) -> &Style {
            &self.style
        }
        fn style_mut(&mut self) -> &mut Style {
            &mut self.style
        }
        fn opacity(&self) -> f64 {
            self.opacity
        }
        fn set_opacity(&mut self, opacity: f64) {
            self.opacity = opacity.clamp(0.0, 1.0);
            self.propagate_parent_opacity();
        }
        fn effective_opacity(&self) -> f64 {
            self.opacity * self.parent_opacity
        }
        fn set_parent_opacity(&mut self, parent: f64) {
            self.parent_opacity = parent.clamp(0.0, 1.0);
            self.propagate_parent_opacity();
        }
        fn transform(&self) -> Affine {
            self.transform
        }
        fn transform_mut(&mut self) -> &mut Affine {
            &mut self.transform
        }
        fn bounds(&self) -> Bounds {
            let mut min = Point::new(f64::INFINITY, f64::INFINITY);
            let mut max = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
            for child in &self.children {
                let b = child.bounds();
                if b.min.x().is_finite() {
                    min = Point::new(min.x().min(b.min.x()), min.y().min(b.min.y()));
                    max = Point::new(max.x().max(b.max.x()), max.y().max(b.max.y()));
                }
            }
            if min.x().is_finite() {
                Bounds { min, max }
            } else {
                Bounds::empty()
            }
        }
        fn paths(&self) -> Vec<BezPath> {
            self.children.iter().flat_map(|c| c.paths()).collect()
        }
        fn children(&self) -> Vec<&dyn Sobject> {
            self.children.iter().map(|c| c.as_ref()).collect()
        }
        fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Sobject)) {
            for child in &mut self.children {
                f(child.as_mut());
            }
        }
        fn add_child(&mut self, child: Box<dyn Sobject>) {
            self.children.push(child);
            self.propagate_parent_opacity();
        }
        fn updaters(&self) -> &[Updater] {
            &self.updaters
        }
        fn updaters_mut(&mut self) -> &mut Vec<Updater> {
            &mut self.updaters
        }
        fn save_state(&mut self) {
            for c in &mut self.children {
                c.save_state();
            }
            self.saved = Some(GroupSnapshot { opacity: self.opacity, transform: self.transform });
        }
        fn restore(&mut self) {
            for c in &mut self.children {
                c.restore();
            }
            if let Some(s) = self.saved.take() {
                self.opacity = s.opacity;
                self.transform = s.transform;
            }
        }
        fn generate_target(&mut self) {
            for c in &mut self.children {
                c.generate_target();
            }
            self.target = Some(GroupSnapshot { opacity: self.opacity, transform: self.transform });
        }
        fn has_target(&self) -> bool {
            self.target.is_some() || self.children.iter().any(|c| c.has_target())
        }
        fn apply_target(&mut self) {
            for c in &mut self.children {
                c.apply_target();
            }
            if let Some(t) = self.target.take() {
                self.opacity = t.opacity;
                self.transform = t.transform;
            }
        }
        fn clone_box(&self) -> Box<dyn Sobject> {
            Box::new(Group {
                id: next_id(),
                name: self.name.clone(),
                children: self.children.iter().map(|c| c.clone_box()).collect(),
                style: self.style.clone(),
                opacity: self.opacity,
                parent_opacity: self.parent_opacity,
                transform: self.transform,
                z_order: self.z_order,
                saved: None,
                target: None,
                updaters: self.updaters.clone(),
            })
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn z_order(&self) -> i64 {
            self.z_order
        }
        fn set_z_order(&mut self, z: i64) {
            self.z_order = z;
        }
    }

    /// ✏️ Vector-only group convenience wrapper.
    pub type VGroup = Group;

    pub fn vgroup(children: Vec<Box<dyn Sobject>>) -> VGroup {
        Group::new(children)
    }

    /// ↔ Place `mover` next to `anchor` along a direction.
    pub fn next_to(mover: &mut dyn Sobject, anchor: &dyn Sobject, direction: Vec2, buff: f64) {
        let mb = mover.bounds();
        let ab = anchor.bounds();
        let dir = if direction.hypot() < 1e-9 { Vec2::new(1.0, 0.0) } else { direction / direction.hypot() };
        let shift = if dir.x().abs() > dir.y().abs() {
            let edge = if dir.x() > 0.0 { ab.max.x() } else { ab.min.x() };
            let target = if dir.x() > 0.0 { edge + buff + mb.width() / 2.0 } else { edge - buff - mb.width() / 2.0 };
            Vec2::new(target - mb.center().x(), 0.0)
        } else {
            let edge = if dir.y() > 0.0 { ab.max.y() } else { ab.min.y() };
            let target = if dir.y() > 0.0 { edge + buff + mb.height() / 2.0 } else { edge - buff - mb.height() / 2.0 };
            Vec2::new(0.0, target - mb.center().y())
        };
        mover.shift(shift);
    }

    /// 📏️ Arrange children in a line.
    pub fn arrange(group: &mut Group, direction: Vec2, buff: f64) {
        if group.children.is_empty() {
            return;
        }
        let dir = if direction.hypot() < 1e-9 { Vec2::new(1.0, 0.0) } else { direction / direction.hypot() };
        let mut cursor = group.children[0].center();
        for child in group.children.iter_mut().skip(1) {
            let b = child.bounds();
            let step = if dir.x().abs() > dir.y().abs() { b.width() / 2.0 + buff } else { b.height() / 2.0 + buff };
            cursor = Point::new(cursor.x() + dir.x() * step, cursor.y() + dir.y() * step);
            child.move_to(cursor);
        }
    }

    /// 🎯️ Align `mover` to `anchor` along an edge or center.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AlignEdge {
        Left,
        Right,
        Up,
        Down,
        Center,
    }

    pub fn align_to(mover: &mut dyn Sobject, anchor: &dyn Sobject, edge: AlignEdge) {
        let mb = mover.bounds();
        let ab = anchor.bounds();
        let shift = match edge {
            AlignEdge::Left => Vec2::new(ab.min.x() - mb.min.x(), 0.0),
            AlignEdge::Right => Vec2::new(ab.max.x() - mb.max.x(), 0.0),
            AlignEdge::Up => Vec2::new(0.0, ab.max.y() - mb.max.y()),
            AlignEdge::Down => Vec2::new(0.0, ab.min.y() - mb.min.y()),
            AlignEdge::Center => anchor.center() - mover.center(),
        };
        mover.shift(shift);
    }

    pub fn center_of_points(points: &[Point]) -> Point {
        if points.is_empty() {
            Point::ZERO
        } else {
            polygon_centroid(points)
        }
    }

    fn transform_bezpath(path: &BezPath, affine: kurbo::Affine) -> BezPath {
        let mut k = path.to_kurbo();
        k.apply_affine(affine);
        let mut out = BezPath::new();
        for el in k.elements() {
            out.push((*el).into());
        }
        out
    }

    trait PathElPoint {
        fn as_ref_point(&self) -> Option<Point>;
    }

    impl PathElPoint for PathEl {
        fn as_ref_point(&self) -> Option<Point> {
            match self {
                PathEl::MoveTo(p) | PathEl::LineTo(p) => Some(*p),
                PathEl::QuadTo(p, _) | PathEl::CurveTo(p, _, _) => Some(*p),
                PathEl::ClosePath => None,
            }
        }
    }

    trait PointVec2 {
        fn to_vec2(self) -> Vec2;
    }

    impl PointVec2 for Point {
        fn to_vec2(self) -> Vec2 {
            Vec2::new(self.x(), self.y())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use geometry::Circle;

        #[test]
        fn vobject_has_finite_bounds() {
            let dot = VSobject::from_shape(&Circle::new(Point::new(0.0, 0.0), 1.0));
            let b = dot.bounds();
            assert!(b.max.x() > b.min.x());
        }

        #[test]
        fn parent_opacity_multiplies() {
            let mut v = VSobject::new();
            v.set_opacity(0.5);
            v.set_parent_opacity(0.5);
            assert!((v.effective_opacity() - 0.25).abs() < 1e-9);
        }

        #[test]
        fn group_propagates_parent_opacity() {
            let mut g = Group::new(vec![Box::new(VSobject::new())]);
            g.set_opacity(0.5);
            assert!((g.children[0].effective_opacity() - 0.5).abs() < 1e-9);
        }

        fn square_vobj(center: Point, half: f64) -> VSobject {
            let mut path = BezPath::new();
            path.move_to(Point::new(center.x() - half, center.y() - half));
            path.line_to(Point::new(center.x() + half, center.y() - half));
            path.line_to(Point::new(center.x() + half, center.y() + half));
            path.line_to(Point::new(center.x() - half, center.y() + half));
            path.close_path();
            VSobject::from_path(path)
        }

        #[test]
        fn next_to_places_mover_right_of_anchor() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            let mut mover = square_vobj(Point::ZERO, 0.5);
            next_to(&mut mover, &anchor, Vec2::new(1.0, 0.0), 0.2);
            let b = mover.bounds();
            assert!((b.min.x() - 1.2).abs() < 1e-6);
        }

        #[test]
        fn next_to_places_mover_below_anchor() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            let mut mover = square_vobj(Point::ZERO, 0.5);
            next_to(&mut mover, &anchor, Vec2::new(0.0, -1.0), 0.2);
            let b = mover.bounds();
            assert!((b.max.y() - (-1.2)).abs() < 1e-6);
        }

        #[test]
        fn next_to_zero_direction_defaults_to_right() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            let mut mover = square_vobj(Point::ZERO, 0.5);
            next_to(&mut mover, &anchor, Vec2::new(0.0, 0.0), 0.2);
            let b = mover.bounds();
            assert!((b.min.x() - 1.2).abs() < 1e-6);
        }

        #[test]
        fn arrange_lays_children_along_direction() {
            let children: Vec<Box<dyn Sobject>> = vec![Box::new(square_vobj(Point::ZERO, 0.5)), Box::new(square_vobj(Point::ZERO, 0.5)), Box::new(square_vobj(Point::ZERO, 0.5))];
            let mut g = Group::new(children);
            arrange(&mut g, Vec2::new(1.0, 0.0), 0.5);
            assert!((g.children[0].center().x() - 0.0).abs() < 1e-6);
            assert!((g.children[1].center().x() - 1.0).abs() < 1e-6);
            assert!((g.children[2].center().x() - 2.0).abs() < 1e-6);
        }

        #[test]
        fn arrange_on_empty_group_is_noop() {
            let mut g = Group::empty();
            arrange(&mut g, Vec2::new(1.0, 0.0), 0.5);
            assert!(g.children.is_empty());
        }

        #[test]
        fn align_to_all_edges() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            for edge in [AlignEdge::Left, AlignEdge::Right, AlignEdge::Up, AlignEdge::Down, AlignEdge::Center] {
                let mut mover = square_vobj(Point::new(5.0, 5.0), 0.5);
                align_to(&mut mover, &anchor, edge);
                let b = mover.bounds();
                match edge {
                    AlignEdge::Left => assert!((b.min.x() - (-1.0)).abs() < 1e-6),
                    AlignEdge::Right => assert!((b.max.x() - 1.0).abs() < 1e-6),
                    AlignEdge::Up => assert!((b.max.y() - 1.0).abs() < 1e-6),
                    AlignEdge::Down => assert!((b.min.y() - (-1.0)).abs() < 1e-6),
                    AlignEdge::Center => {
                        assert!(b.center().x().abs() < 1e-6);
                        assert!(b.center().y().abs() < 1e-6);
                    }
                }
            }
        }

        #[test]
        fn center_of_points_empty_is_zero() {
            assert_eq!(center_of_points(&[]), Point::ZERO);
        }

        #[test]
        fn center_of_points_nonempty_matches_centroid() {
            let pts = [Point::new(-1.0, -1.0), Point::new(1.0, -1.0), Point::new(1.0, 1.0), Point::new(-1.0, 1.0)];
            let c = center_of_points(&pts);
            assert!(c.x().abs() < 1e-9);
            assert!(c.y().abs() < 1e-9);
        }

        #[test]
        fn trim_path_at_ratio_boundary_and_partial() {
            let mut path = BezPath::new();
            path.move_to(Point::new(0.0, 0.0));
            path.line_to(Point::new(10.0, 0.0));
            let full = trim_path_at_ratio(&path, 1.0);
            assert_eq!(full.elements().len(), path.elements().len());
            let none = trim_path_at_ratio(&path, 0.0);
            assert!(none.elements().is_empty());
            let half = trim_path_at_ratio(&path, 0.5);
            assert!(!half.elements().is_empty());
            let over = trim_path_at_ratio(&path, 2.0);
            assert_eq!(over.elements().len(), path.elements().len());
        }
    }
}
