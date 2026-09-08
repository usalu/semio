mod tests {
    use super::*;
    use crate::editor::animate::engine::video::render::OutputFormat;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_config() -> AnimateConfig {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_video_test_{stamp}"));
        AnimateConfig::default().with_resolution(16, 16).with_output_dir(&dir).with_media_dir(dir.join("media"))
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_writes_srt_from_sections() {
        let config = temp_config();
        let sections = SectionList::default();
        let path = config.output_dir.join("scene.srt");
        write_sections_srt(&sections, &path).expect("srt");
        assert!(path.exists());
    }

    /// 🌉️ Scenario (c) — animate's e2e acceptance scenario: animate → semio/video → real mp4,
    /// "playable" operationalized as "decodes clean via the real codec we wrote" per the master
    /// plan's own framing. `decode_mp4` succeeding on real written bytes IS the box-walk proof
    /// (ISO-BMFF is a nested box tree — a truncated/malformed box tree is a hard decode error,
    /// never a silent partial result, per stdio's own mp4 engine); the assertions below add the
    /// explicit track/duration invariants (real `ftyp` header, sample-accurate total track
    /// duration in timescale ticks, byte-exact frame payload) on top of that.
    #[semio_framework_async_macros::async_test]
    async fn writer_buffers_frame_and_finalizes_a_real_decodable_mp4() {
        let config = temp_config();
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Mp4]).expect("writer");
        let partial = writer.begin_partial("hash", 0).expect("partial");
        let pixels = vec![255u8; 16 * 16 * 4];
        writer.push_frame(&pixels, 0).expect("frame");
        writer.push_frame(&pixels, 1).expect("frame");
        let encoded = writer.finalize_partial(&partial).expect("finalize");
        assert!(encoded.exists());
        let bytes = fs::read(&encoded).expect("read partial mp4");
        // 🌉️ `decode_mp4` walks the real nested ISO-BMFF box tree (ftyp/moov/trak/mdat/...) to
        // produce this snapshot -- a bogus or truncated box tree is a hard `Err` here, never a
        // silently-partial result, so this `expect` succeeding IS the box-walk assertion.
        let snapshot = decode_mp4(&bytes).expect("decode real mp4 bytes: box-walk must succeed clean");
        assert!(!snapshot.ftyp.major_brand.is_empty(), "real ftyp box must have survived the box-walk with a non-empty major_brand");
        assert_eq!(snapshot.tracks.len(), 1, "track-count invariant: exactly one video track");
        let track = &snapshot.tracks[0];
        assert!(track.timescale > 0, "timescale invariant: a real track always carries a positive timescale");
        assert_eq!(track.samples.len(), 2, "sample-count invariant: exactly the 2 pushed frames");
        let total_duration_ticks: u64 = track.samples.iter().map(|sample| sample.duration as u64).sum();
        assert_eq!(total_duration_ticks, 2, "duration invariant: 2 frames * 1 tick/frame == 2 total timescale ticks");
        assert!((total_duration_ticks as f64 / track.timescale as f64) > 0.0, "duration invariant: real-world track duration (ticks / timescale) must be positive");
        assert_eq!(track.samples[0].data, pixels, "byte-exact frame payload must survive the real mp4 round trip");
        assert_eq!(track.samples[1].data, pixels);
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
    async fn concat_raw_partials_merges_sample_counts_and_stays_decodable() {
        let config = temp_config();
        let mut writer = SceneFileWriter::new(&config, &[OutputFormat::Mp4]).expect("writer");
        let pixels = vec![128u8; 16 * 16 * 4];
        let first_partial = writer.begin_partial("a", 0).expect("partial a");
        writer.push_frame(&pixels, 0).expect("frame");
        writer.finalize_partial(&first_partial).expect("finalize a");
        let second_partial = writer.begin_partial("b", 1).expect("partial b");
        writer.push_frame(&pixels, 1).expect("frame");
        writer.finalize_partial(&second_partial).expect("finalize b");
        let output = config.output_dir.join("scene.mp4");
        let frames = concat_raw_partials(&writer.partial_paths, &output, config.width, config.height, 16).expect("concat");
        assert_eq!(frames.len(), 2);
        let bytes = fs::read(&output).expect("read merged mp4");
        let snapshot = decode_mp4(&bytes).expect("decode merged mp4");
        assert_eq!(snapshot.tracks[0].samples.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_gif_snapshot_quantizes_and_downscales() {
        let frames = vec![[255u8, 0, 0, 255].repeat(64 * 64)];
        let snapshot = build_gif_snapshot(64, 64, 15.0, &frames).expect("gif snapshot");
        assert_eq!(snapshot.frames.len(), 1);
        assert_eq!(snapshot.width, 64);
        assert_eq!(snapshot.frames[0].indices.len(), (snapshot.width * snapshot.height) as usize);
        assert_eq!(snapshot.gct.as_ref().map(|t| t.colors.len()), Some(256));
        let bytes = encode_gif(&snapshot).expect("real gif encode");
        assert!(!bytes.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn nearest_neighbor_scale_downsizes_dimensions() {
        let src = vec![7u8; (8 * 8 * 4) as usize];
        let scaled = nearest_neighbor_scale(&src, 8, 8, 4, 4);
        assert_eq!(scaled.len(), 4 * 4 * 4);
    }
}
