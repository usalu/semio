mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::scene::{BasicStage, Scene};
    use crate::editor::animate::engine::scene::sobject::{Sobject, Sobjects, VSobject};
    use semio_s_artifact_animate_presentation_examples::editor::animate::engine::rate::updater::{add_updater, Updater};
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
            let mut first: Sobjects = VSobject::new().into();
            let removed = Updater::new("remove", |object, _| object.set_opacity(0.0));
            let removed_id = removed.id;
            add_updater(&mut first, removed);
            add_updater(&mut first, Updater::new("fade", |object, dt| object.set_opacity(object.opacity() - dt)));
            first.updaters_mut().retain(|updater| updater.id != removed_id);
            self.add(first);
            self.wait(0.1);
            self.add(VSobject::new().into());
            self.wait(0.1);
        }

        fn stage(&self) -> &BasicStage {
            &self.base
        }

        fn stage_mut(&mut self) -> &mut BasicStage {
            &mut self.base
        }
    }

    #[test]
    fn captures_every_authored_sample_and_keeps_a_still_fallback() {
        let config = AnimateConfig::default().with_frame_rate(15.0);
        let recorded = capture_scene(DemoScene::new(config.clone()), &config);
        let sizes: Vec<_> = recorded.captures.iter().map(|frame| frame.mobjects.len()).collect();
        assert_eq!(sizes, [1, 1, 1, 2, 2, 2]);
        let first_id = recorded.captures[0].mobjects[0].id();
        for (index, frame) in recorded.captures.iter().enumerate() {
            let elapsed = (index + 1) as f64 * config.frame_duration();
            assert!((frame.time - elapsed).abs() < 1e-12);
            let first = frame.mobjects.iter().find(|object| object.id() == first_id).expect("first object after capture");
            assert!((first.opacity() - (1.0 - elapsed)).abs() < 1e-12);
        }
        let still = capture_scene(BasicStage::new(config.clone()), &config);
        assert_eq!(still.captures.len(), 1);
        assert_eq!(still.captures[0].time, 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_scene_writes_last_frame() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_render_test_{stamp}"));
        let mut config = AnimateConfig::default().with_resolution(64, 64).with_frame_rate(15.0).with_output_dir(&dir).with_media_dir(dir.join("media"));
        config.cache.partial_movie_dir = dir.join("partials");
        let scene = DemoScene::new(config.clone());
        let outputs = render_scene(scene, &config, &[OutputFormat::LastFrame]).await.expect("render");
        let last = outputs.last_frame.expect("last frame path");
        assert!(last.exists());
        fs::remove_dir_all(&dir).expect("remove render output");
    }
}
