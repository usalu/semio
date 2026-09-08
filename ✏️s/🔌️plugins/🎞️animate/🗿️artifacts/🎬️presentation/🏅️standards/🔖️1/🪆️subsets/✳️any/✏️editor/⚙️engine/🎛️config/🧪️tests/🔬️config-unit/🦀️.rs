mod tests {
    use super::*;

    #[test]
    fn quality_presets_have_expected_resolution() {
        assert_eq!(QualityPreset::High.resolution(), (1920, 1080));
        assert_eq!(QualityPreset::FourK.resolution(), (3840, 2160));
    }

    #[test]
    fn config_frame_duration_matches_rate() {
        let cfg = AnimateConfig::default().with_frame_rate(30.0);
        assert!((cfg.frame_duration() - 1.0 / 30.0).abs() < 1e-9);
    }

    #[test]
    fn all_quality_presets_report_frame_rate_and_resolution() {
        assert_eq!(QualityPreset::Low.frame_rate(), 15.0);
        assert_eq!(QualityPreset::Medium.frame_rate(), 15.0);
        assert_eq!(QualityPreset::High.frame_rate(), 60.0);
        assert_eq!(QualityPreset::FourK.frame_rate(), 60.0);
        assert_eq!(QualityPreset::Production.frame_rate(), 60.0);
        assert_eq!(QualityPreset::Low.resolution(), (854, 480));
        assert_eq!(QualityPreset::Medium.resolution(), (1280, 720));
        assert_eq!(QualityPreset::Production.resolution(), (2560, 1440));
        assert_eq!(QualityPreset::High.pixel_height(), 1080);
    }

    #[test]
    fn config_builder_methods_apply() {
        let cfg = AnimateConfig::from_quality(QualityPreset::Low).with_resolution(0, 0).with_output_dir("out").with_media_dir("media2").with_audio_track("track.wav").with_subtitles_path("subs.srt");
        assert_eq!(cfg.width, 1);
        assert_eq!(cfg.height, 1);
        assert_eq!(cfg.output_dir, PathBuf::from("out"));
        assert_eq!(cfg.media_dir, PathBuf::from("media2"));
        assert_eq!(cfg.audio_track, Some(PathBuf::from("track.wav")));
        assert_eq!(cfg.subtitles_path, Some(PathBuf::from("subs.srt")));
    }

    #[test]
    fn config_with_frame_rate_clamps_to_minimum() {
        let cfg = AnimateConfig::default().with_frame_rate(-5.0);
        assert_eq!(cfg.frame_rate, 1.0);
    }

    #[test]
    fn config_aspect_ratio_and_default_cache() {
        let cfg = AnimateConfig::from_quality(QualityPreset::Medium);
        assert!(cfg.cache.enabled);
        assert_eq!(cfg.cache.max_entries, 10_000);
        assert!((cfg.aspect_ratio() - 1280.0 / 720.0).abs() < 1e-9);
    }
}
