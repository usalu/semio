//! 🎭 Scene trait with construct/play/wait timeline and frame loop.

use crate::animation::{apply_parent_opacity_tree, compile_animations, interpolate_at, Animation, Wait};
use crate::camera::Camera;
use crate::config::AnimateConfig;
use crate::section::SectionList;
use crate::sobject::Sobject;
use crate::updater::run_updaters;
use std::collections::HashMap;

/// 🎬 User-authored animation scene contract.
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
        animation.begin();
        let duration = animation.duration().max(0.0);
        let steps = (duration * self.config().frame_rate).ceil() as u64;
        let steps = steps.max(1);
        for frame in 0..=steps {
            let alpha = frame as f64 / steps as f64;
            interpolate_at(animation.as_mut(), alpha);
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
        let t = self.scene_time() + dt;
        self.set_scene_time(t);
        for m in self.mobjects_mut().values_mut() {
            apply_parent_opacity_tree(m.as_mut(), 1.0);
            run_updaters(m.as_mut(), dt);
        }
    }

    fn render_frame_index(&self, frame: u64) -> SceneFrame {
        SceneFrame {
            frame,
            time: frame as f64 / self.config().frame_rate,
            mobject_count: self.mobjects().len(),
        }
    }
}

/// 🖼️ Lightweight frame snapshot metadata for renderers.
#[derive(Clone, Debug)]
pub struct SceneFrame {
    pub frame: u64,
    pub time: f64,
    pub mobject_count: usize,
}

/// 🏗️ Default scene implementation backing most user scenes.
pub struct BasicScene {
    pub config: AnimateConfig,
    pub camera: Camera,
    pub mobjects: HashMap<u64, Box<dyn Sobject>>,
    pub sections: SectionList,
    pub scene_time: f64,
}

impl BasicScene {
    pub fn new(config: AnimateConfig) -> Self {
        let camera = Camera::new(
            config.width as f64 / 100.0,
            config.height as f64 / 100.0,
        );
        Self {
            config,
            camera,
            mobjects: HashMap::new(),
            sections: SectionList::new(),
            scene_time: 0.0,
        }
    }

    pub fn run_construct<S: Scene>(&mut self, scene: &mut S) {
        scene.setup(&self.config);
        scene.construct();
        scene.tear_down();
    }
}

impl Scene for BasicScene {
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

/// 🧪 Specialized scene for unit tests with fixed frame rate.
pub struct TestScene {
    inner: BasicScene,
}

impl TestScene {
    pub fn new() -> Self {
        Self {
            inner: BasicScene::new(AnimateConfig::default().with_frame_rate(60.0)),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sobject::VSobject;

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
    fn scene_construct_adds_mobject() {
        let mut s = DemoScene::new();
        s.construct();
        assert_eq!(s.mobjects().len(), 1);
    }
}
