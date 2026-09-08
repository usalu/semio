mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::sobject::VSobject;

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
            self.add(VSobject::new().into());
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
        fn mobjects(&self) -> &HashMap<u64, Sobjects> {
            self.base.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Sobjects> {
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
