mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::scene::{BasicStage, Scene};
    use crate::editor::animate::engine::scene::sobject::VSobject;

    struct DemoScene {
        base: BasicStage,
    }

    impl DemoScene {
        fn new(config: AnimateConfig) -> Self {
            Self { base: BasicStage::new(config) }
        }
    }

    impl Scene for DemoScene {
        fn construct(&mut self) {
            self.add(VSobject::new().into());
            self.wait(0.05);
        }
        fn stage(&self) -> &BasicStage {
            &self.base
        }

        fn stage_mut(&mut self) -> &mut BasicStage {
            &mut self.base
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_scene_window_metadata_runs() {
        let config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(30.0);
        let scene = DemoScene::new(config.clone());
        let outcome = preview_scene_headless(scene, &config, Some(2)).await.expect("preview");
        assert_eq!(outcome, PreviewOutcome::MetadataOnly);
    }
}
