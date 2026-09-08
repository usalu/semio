mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::sobject::VSobject;

    #[semio_framework_async_macros::async_test]
    async fn vello_renderer_produces_rgba_buffer() {
        let config = AnimateConfig::default().with_resolution(64, 64);
        let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
        let mut capture = CapturedFrame { time: 0.0, mobjects: vec![VSobject::new().into()] };
        let mut renderer = VelloRenderer::new(config.width, config.height).await.expect("renderer");
        let pixels = renderer.render_capture(&capture, &camera, &config).expect("frame");
        assert_eq!(pixels.len(), 64 * 64 * 4);
        capture.mobjects.clear();
        let empty = renderer.render_capture(&capture, &camera, &config).expect("empty");
        assert_eq!(empty.len(), 64 * 64 * 4);
    }
}
