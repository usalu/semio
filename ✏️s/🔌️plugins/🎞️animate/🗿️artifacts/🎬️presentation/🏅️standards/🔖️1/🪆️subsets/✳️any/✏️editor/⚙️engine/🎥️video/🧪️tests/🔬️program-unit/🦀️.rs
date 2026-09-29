mod tests {
    use super::*;
    use crate::editor::animate::engine::config::config::QualityPreset;
    use crate::editor::animate::engine::geometry::geometry::square;
    use crate::editor::animate::engine::scene::sobject::Sobjects;
    use crate::editor::animate::engine::text::color::Color;
    use crate::editor::animate::engine::video::render::capture_scene;
    use crate::editor::animate::engine::video::scenes::scene_for_hash;

    fn built(config: &AnimateConfig, frames: &[CapturedFrame], camera: &Camera) -> VideoRenderProgram {
        let mut builder = VideoProgramBuilder::new(config);
        for frame in frames {
            builder.push_capture(frame, camera, config);
        }
        builder.finish()
    }

    /// ⚖️ LAW: a demo scene (an empty mobject held for 0.2 s) is ONE still scene played for every captured frame, at the
    /// Medium preset's picture size and rate, and the kernel admits the program.
    #[test]
    fn a_demo_scene_is_one_still_scene_for_every_captured_frame() {
        let config = AnimateConfig::from_quality(QualityPreset::Medium);
        let capture = capture_scene(scene_for_hash(config.clone(), "abc123"), &config);
        let program = built(&config, &capture.captures, &capture.camera);
        assert_eq!(program.validate(), Ok(()));
        assert_eq!(program.frame_count(), capture.captures.len() as u64);
        assert_eq!(capture.captures.len(), 4, "0.2 s at 15 fps samples frames 0..=3");
        assert_eq!((program.width, program.height, program.fps), (1280, 720, 15));
        assert_eq!((program.scenes.len(), program.timeline.len()), (1, 1));
    }

    /// ⚖️ LAW: identical geometry is one path row, identical frames one scene, consecutive identical frames one run, and a
    /// mobject at the scene origin lands at the picture centre.
    #[test]
    fn painted_mobjects_share_paths_scenes_and_runs() {
        let config = AnimateConfig::from_quality(QualityPreset::Medium);
        let camera = capture_scene(scene_for_hash(config.clone(), "camera"), &config).camera;
        let frame = |color: Color| CapturedFrame { time: 0.0, mobjects: vec![Sobjects::from(square(1.0, geometry::Point::new(0.0, 0.0), color, None, 0.0))] };
        let frames = [frame(Color::RED), frame(Color::RED), frame(Color::BLUE), frame(Color::RED)];
        let program = built(&config, &frames, &camera);
        assert_eq!(program.validate(), Ok(()));
        assert_eq!(program.paths.len(), 1, "one geometry, one path row");
        assert_eq!(program.scenes.len(), 2, "red and blue");
        assert_eq!(program.timeline, vec![VideoRenderRun { scene: 0, frames: 2 }, VideoRenderRun { scene: 1, frames: 1 }, VideoRenderRun { scene: 0, frames: 1 }]);
        match &program.scenes[0].ops[0] {
            VideoRenderOp::Fill { color, transform, .. } => {
                assert_eq!(*color, [1.0, 0.0, 0.0, 1.0]);
                assert_eq!([transform[4], transform[5]], [640.0, 360.0], "scene origin → picture centre");
            }
            other => panic!("a filled square paints a fill first, got {other:?}"),
        }
    }
}
