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
        fn stage(&self) -> &BasicStage {
            self.base.stage()
        }

        fn stage_mut(&mut self) -> &mut BasicStage {
            self.base.stage_mut()
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
