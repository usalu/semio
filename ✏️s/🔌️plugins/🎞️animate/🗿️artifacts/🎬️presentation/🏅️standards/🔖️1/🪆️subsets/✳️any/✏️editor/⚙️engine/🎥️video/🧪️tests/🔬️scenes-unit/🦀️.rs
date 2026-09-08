mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn scene_for_hash_constructs() {
        let config = AnimateConfig::default().with_resolution(32, 32).with_frame_rate(15.0);
        let mut scene = scene_for_hash(config.clone(), "abc123");
        scene.setup(&config);
        scene.construct();
        assert!(!scene.mobjects().is_empty());
    }
}
