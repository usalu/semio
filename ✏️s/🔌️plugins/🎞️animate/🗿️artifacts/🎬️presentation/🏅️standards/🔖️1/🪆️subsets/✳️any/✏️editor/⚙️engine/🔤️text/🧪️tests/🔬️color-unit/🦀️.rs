mod tests {
    use super::*;

    #[test]
    fn lerp_midpoint_is_average() {
        let c = Color::BLACK.lerp(Color::WHITE, 0.5);
        assert!((c.r - 0.5).abs() < 1e-9);
    }

    #[test]
    fn gradient_samples_stops() {
        let g = Gradient::new(vec![(0.0, Color::RED), (1.0, Color::BLUE)]);
        let mid = g.sample(0.5);
        assert!(mid.r > 0.0 && mid.b > 0.0);
    }

    #[test]
    fn hex_parses_six_and_eight_digit_forms() {
        let rgb = Color::hex("#ff0000");
        assert!((rgb.r - 1.0).abs() < 1e-9);
        assert!((rgb.a - 1.0).abs() < 1e-9);
        let rgba = Color::hex("00ff0080");
        assert!((rgba.g - 1.0).abs() < 1e-9);
        assert!((rgba.a - 128.0 / 255.0).abs() < 1e-9);
    }

    #[test]
    fn hex_falls_back_to_black_on_invalid_length() {
        let bad = Color::hex("#abc");
        assert_eq!(bad, Color::BLACK);
    }

    #[test]
    fn named_color_covers_aliases_and_hex_fallback() {
        assert_eq!(named_color("WHITE"), Color::WHITE);
        assert_eq!(named_color("grey"), Color::GRAY);
        assert_eq!(named_color("gray"), Color::GRAY);
        assert_eq!(named_color("semio_blue"), Color::hex("#58C4DD"));
        assert_eq!(named_color("manim_blue"), Color::hex("#58C4DD"));
        assert_eq!(named_color("semio_green"), Color::hex("#83C167"));
        assert_eq!(named_color("manim_red"), Color::hex("#FC6255"));
        assert_eq!(named_color("manim_yellow"), Color::hex("#FFFF00"));
        assert_eq!(named_color("ff00ff"), Color::hex("ff00ff"));
    }

    #[test]
    fn gradient_edge_cases() {
        let empty = Gradient::new(vec![]);
        assert_eq!(empty.sample(0.5), Color::WHITE);
        let single = Gradient::new(vec![(0.3, Color::RED)]);
        assert_eq!(single.sample(0.0), Color::RED);
        assert_eq!(single.sample(1.0), Color::RED);
        let g = Gradient::new(vec![(0.2, Color::RED), (0.8, Color::BLUE)]);
        assert_eq!(g.sample(0.0), Color::RED);
        assert_eq!(g.sample(1.0), Color::BLUE);
    }

    #[test]
    fn gradient_new_sorts_unordered_stops() {
        let g = Gradient::new(vec![(1.0, Color::BLUE), (0.0, Color::RED)]);
        assert_eq!(g.stops[0].0, 0.0);
        assert_eq!(g.stops[1].0, 1.0);
    }

    #[test]
    fn with_alpha_and_to_array_roundtrip() {
        let c = Color::rgb(0.2, 0.4, 0.6).with_alpha(0.5);
        assert_eq!(c.to_array(), [0.2, 0.4, 0.6, 0.5]);
    }
}
