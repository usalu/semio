mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::scene::{BasicStage, Scene};
    use crate::editor::animate::engine::scene::sobject::VSobject;
    use std::time::{SystemTime, UNIX_EPOCH};

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
    async fn render_scene_writes_last_frame() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_render_test_{stamp}"));
        let config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(15.0).with_output_dir(&dir).with_media_dir(dir.join("media"));
        let scene = DemoScene::new(config.clone());
        let outputs = render_scene(scene, &config, &[OutputFormat::LastFrame]).await.expect("render");
        let last = outputs.last_frame.expect("last frame path");
        assert!(last.exists());
    }
}
