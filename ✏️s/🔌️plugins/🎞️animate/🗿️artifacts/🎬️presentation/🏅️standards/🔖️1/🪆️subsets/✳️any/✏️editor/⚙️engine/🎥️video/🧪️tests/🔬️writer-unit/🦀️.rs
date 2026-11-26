mod tests {
    use super::*;
    use crate::editor::animate::engine::video::render::OutputFormat;
    use semio_framework_raster::video::decode::decode_avc_mp4;
    use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::io::decode_gif;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_config() -> AnimateConfig {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_video_test_{stamp}"));
        let mut config = AnimateConfig::default().with_resolution(16, 16).with_output_dir(&dir).with_media_dir(dir.join("media"));
        config.cache.partial_movie_dir = dir.join("partials");
        config
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_writes_srt_from_sections() {
        let config = temp_config();
        let sections = SectionList::default();
        let path = config.output_dir.join("scene.srt");
        write_sections_srt(&sections, &path).expect("srt");
        assert!(path.exists());
    }

    /// 🎞️ A partial plays both repeated frames at the configured cadence.
    #[semio_framework_async_macros::async_test]
    async fn writer_buffers_frame_and_finalizes_a_real_decodable_mp4() {
        let config = temp_config();
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Mp4]).expect("writer");
        let partial = writer.begin_partial(0).expect("partial");
        let pixels = vec![255u8; 16 * 16 * 4];
        writer.push_frame(&pixels, 0).expect("frame");
        writer.push_frame(&pixels, 1).expect("frame");
        let encoded = writer.finalize_partial(&partial).expect("finalize");
        assert!(encoded.exists());
        let bytes = fs::read(&encoded).expect("read partial mp4");
        let video = decode_avc_mp4(&bytes, &mut |_, _| {}, &|| false).expect("decode the actual H.264 samples");
        assert_eq!((video.width, video.height, video.timescale), (16, 16, config.frame_rate.round() as u32));
        assert_eq!(video.frame_ticks, [1, 1]);
        assert_eq!(video.frame_picture, [0, 0]);
        assert_eq!(video.frame_rgba(0), Some(pixels.as_slice()));
        assert_eq!(video.frame_rgba(1), Some(pixels.as_slice()));
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_writes_png_sequence_frame() {
        let config = temp_config();
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::PngSequence]).expect("writer");
        let pixels = vec![255u8; 16 * 16 * 4];
        writer.push_frame(&pixels, 0).expect("frame");
        let frames_dir = config.output_dir.join("frames");
        assert!(frames_dir.join("000000.png").exists());
    }

    #[semio_framework_async_macros::async_test]
    async fn concatenated_movies_decode_each_partial_and_keep_gif_sidecars() {
        let config = temp_config();
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Mp4, OutputFormat::Gif]).expect("writer");
        let red = [255u8, 0, 0, 255].repeat(16 * 16);
        let blue = [0u8, 0, 255, 255].repeat(16 * 16);
        let first_partial = writer.begin_partial(0).expect("partial a");
        writer.push_frame(&red, 0).expect("frame");
        writer.push_frame(&red, 1).expect("repeated frame");
        writer.finalize_partial(&first_partial).expect("finalize a");
        let second_partial = writer.begin_partial(2).expect("partial b");
        writer.push_frame(&blue, 2).expect("frame");
        writer.finalize_partial(&second_partial).expect("finalize b");
        let outputs = writer.encode_outputs(None).expect("outputs");
        let bytes = fs::read(outputs.mp4.expect("mp4")).expect("read merged mp4");
        let video = decode_avc_mp4(&bytes, &mut |_, _| {}, &|| false).expect("decode merged mp4");
        assert_eq!(video.frame_ticks, [1, 1, 1]);
        assert_eq!(video.frame_picture, [0, 0, 1]);
        for (index, expected) in [red, blue].into_iter().enumerate() {
            let pixels = video.frame_rgba(index * 2).expect("decoded picture");
            assert!(pixels.iter().zip(expected).all(|(actual, expected)| actual.abs_diff(expected) <= 2));
        }
        let gif = fs::read(outputs.gif.expect("gif")).expect("read gif");
        assert_eq!(&gif[..6], b"GIF89a");
    }

    #[semio_framework_async_macros::async_test]
    async fn evicting_a_cached_partial_keeps_the_current_movie_complete() {
        use crate::editor::animate::engine::video::cache::PartialMovieLut;

        let config = temp_config();
        let root = &config.cache.partial_movie_dir;
        let dark = [0u8, 0, 0, 255].repeat(16 * 16);
        let light = [255u8; 16 * 16 * 4];
        let first_key = PartialMovieLut::segment_hash("dark", 0, 1);
        let second_key = PartialMovieLut::segment_hash("light", 1, 2);
        {
            let mut producer = SceneFileWriter::new(&config, &[OutputFormat::Mp4]).expect("writer");
            let partial = producer.begin_partial(0).expect("partial");
            producer.push_frame(&dark, 0).expect("frame");
            producer.finalize_partial(&partial).expect("finalize");
            PartialMovieLut::open_with_limit(root, 1).expect("cache").insert(first_key.clone(), &partial).expect("publish");
        }
        let mut cache = PartialMovieLut::open_with_limit(root, 1).expect("reopen");
        let first = cache.get(&first_key).expect("durable lookup");
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Mp4]).expect("writer");
        assert!(writer.register_cached_partial(&first).expect("pin cached movie"));
        let second = writer.begin_partial(1).expect("partial");
        writer.push_frame(&light, 1).expect("frame");
        writer.finalize_partial(&second).expect("finalize");
        cache.insert(second_key, &second).expect("evict first");
        assert!(!first.exists());
        assert!(!writer.register_cached_partial(&first).expect("evicted source becomes a cache miss"));
        let outputs = writer.encode_outputs(None).expect("concatenate the pinned partial");
        let bytes = fs::read(outputs.mp4.expect("mp4")).expect("read mp4");
        let video = decode_avc_mp4(&bytes, &mut |_, _| {}, &|| false).expect("decoded movie");
        assert_eq!(video.frame_count(), 2);
        assert_eq!(video.frame_rgba(0), Some(dark.as_slice()));
        assert_eq!(video.frame_rgba(1), Some(light.as_slice()));
    }

    #[semio_framework_async_macros::async_test]
    async fn gif_only_preserves_the_exact_owner_and_chip_colors() {
        let owner = [6u8, 26, 31];
        let chip = [17u8, 35, 40];
        let mut config = temp_config().with_resolution(800, 3);
        config.background = [6.0 / 255.0, 26.0 / 255.0, 31.0 / 255.0, 1.0];
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Gif]).expect("writer");
        let mut frame = [owner[0], owner[1], owner[2], 255].repeat(800 * 3);
        frame[200 * 4..400 * 4].copy_from_slice(&[chip[0], chip[1], chip[2], 255].repeat(200));
        writer.push_frame(&frame, 0).expect("frame");
        let outputs = writer.encode_outputs(None).expect("outputs");
        assert!(outputs.mp4.is_none());
        let bytes = fs::read(outputs.gif.expect("gif")).expect("read gif");
        let snapshot = decode_gif(&bytes).expect("decode gif");
        let picture = &snapshot.frames[0];
        assert_eq!((picture.width, picture.height), (640, 2));
        let palette = &picture.lct.as_ref().expect("exact colors").colors;
        for (x, y, [r, g, b]) in [(0, 0, owner), (200, 0, chip), (639, 1, owner)] {
            assert_eq!(palette[usize::from(picture.indices[y * 640 + x])], GifRgb { r, g, b });
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn nearest_neighbor_scale_downsizes_dimensions() {
        let src = vec![7u8; (8 * 8 * 4) as usize];
        let scaled = nearest_neighbor_scale(&src, 8, 8, 4, 4);
        assert_eq!(scaled.len(), 4 * 4 * 4);
    }
}
