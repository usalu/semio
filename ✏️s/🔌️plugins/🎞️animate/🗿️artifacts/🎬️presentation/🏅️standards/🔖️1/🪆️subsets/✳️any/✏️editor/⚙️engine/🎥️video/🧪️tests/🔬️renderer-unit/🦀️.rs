mod tests {
    use super::*;
    use crate::editor::animate::engine::scene::sobject::VSobject;

    #[test]
    fn movie_cache_identifies_the_painted_picture_and_its_duration() {
        use crate::editor::animate::engine::geometry::geometry::square;
        let config = AnimateConfig::default().with_resolution(64, 64);
        let camera = Camera::new(4.0, 4.0);
        let picture = |color, size, time| CapturedFrame { time, mobjects: vec![square(size, geometry::Point::new(0.0, 0.0), color, None, 0.0).into()] };
        let first = picture(Color::RED, 1.0, 0.0);
        let key = static_layer_hash(&first, &camera, &config);
        assert_eq!(key, static_layer_hash(&picture(Color::RED, 1.0, 1.0), &camera, &config));
        assert_ne!(key, static_layer_hash(&picture(Color::BLUE, 1.0, 0.0), &camera, &config));
        assert_ne!(key, static_layer_hash(&picture(Color::RED, 0.5, 0.0), &camera, &config));
        assert_ne!(key, static_layer_hash(&first, &Camera::new(8.0, 4.0), &config));
        assert_ne!(partial_hash(&key, 1), partial_hash(&key, 4));
    }

    #[semio_framework_async_macros::async_test]
    async fn cached_renderer_updates_the_center_pixel_after_a_fill_change() {
        use crate::editor::animate::engine::geometry::geometry::square;
        let config = AnimateConfig::default().with_resolution(64, 64);
        let camera = Camera::new(4.0, 4.0);
        let mut renderer = VelloRenderer::new(64, 64).await.expect("renderer");
        for (color, expected) in [(Color::RED, [255, 0, 0, 255]), (Color::BLUE, [0, 0, 255, 255])] {
            let capture = CapturedFrame { time: 0.0, mobjects: vec![square(1.0, geometry::Point::new(0.0, 0.0), color, None, 0.0).into()] };
            let pixels = renderer.render_capture(&capture, &camera, &config).expect("painted frame");
            let center = (32 * 64 + 32) * 4;
            assert_eq!(&pixels[center..center + 4], &expected);
        }
    }

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
