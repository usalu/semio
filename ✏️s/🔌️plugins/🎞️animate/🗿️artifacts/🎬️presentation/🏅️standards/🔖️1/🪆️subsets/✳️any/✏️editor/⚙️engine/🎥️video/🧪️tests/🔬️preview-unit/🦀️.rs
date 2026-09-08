mod tests {
    use super::*;
    use crate::editor::animate::engine::camera::camera::Camera;
    use crate::editor::animate::engine::scene::scene::{BasicStage, Scene};
    use crate::editor::animate::engine::scene::section::SectionList;
    use crate::editor::animate::engine::scene::sobject::{Sobjects, VSobject};
    use std::collections::HashMap;

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

    #[semio_framework_async_macros::async_test]
    async fn preview_scene_window_metadata_runs() {
        let config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(30.0);
        let scene = DemoScene::new(config.clone());
        let outcome = preview_scene_headless(scene, &config, Some(2)).await.expect("preview");
        assert_eq!(outcome, PreviewOutcome::MetadataOnly);
    }
}
