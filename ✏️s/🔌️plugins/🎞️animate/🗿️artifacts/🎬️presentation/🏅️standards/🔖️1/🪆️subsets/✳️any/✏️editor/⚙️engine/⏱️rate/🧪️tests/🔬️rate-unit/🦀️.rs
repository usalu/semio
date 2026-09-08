mod tests {
    use super::*;

    #[test]
    fn endpoints_are_zero_and_one() {
        for f in [linear as RateFunc, smooth, ease_in_out_cubic, ease_out_bounce] {
            assert!((f(0.0) - 0.0).abs() < 1e-6);
            assert!((f(1.0) - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn map_child_alpha_splits_interval() {
        assert_eq!(map_child_alpha(0.25, 0.0, 0.5), 0.5);
        assert_eq!(map_child_alpha(0.75, 0.5, 1.0), 0.5);
    }

    #[test]
    fn map_child_alpha_degenerate_interval_is_step() {
        assert_eq!(map_child_alpha(0.4, 0.5, 0.5), 0.0);
        assert_eq!(map_child_alpha(0.6, 0.5, 0.5), 1.0);
        assert_eq!(map_child_alpha(0.5, 0.7, 0.2), 1.0);
    }

    #[test]
    fn simple_easing_functions_are_monotonic_within_range() {
        for f in [rush_from as RateFunc, slow_into, running_start, lingering] {
            assert!((f(0.0) - 0.0).abs() < 1e-6, "expected 0 at t=0");
            assert!((f(1.0) - 1.0).abs() < 1e-6, "expected 1 at t=1");
            let mid = f(0.5);
            assert!((0.0..=1.0).contains(&mid));
        }
    }

    #[test]
    fn rush_into_overshoots_past_one_at_t_equals_one() {
        assert!((rush_into(0.0) - 0.0).abs() < 1e-9);
        assert!((rush_into(1.0) - 2.0).abs() < 1e-9);
        assert!(rush_into(0.5) < 1.0);
    }

    #[test]
    fn double_smooth_covers_both_branches() {
        assert!(double_smooth(0.25) > 0.0 && double_smooth(0.25) < 0.5);
        assert!(double_smooth(0.75) > 0.5 && double_smooth(0.75) < 1.0);
        assert!((double_smooth(0.0) - 0.0).abs() < 1e-9);
        assert!((double_smooth(1.0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn there_and_back_returns_to_start() {
        assert!((there_and_back(0.0) - 0.0).abs() < 1e-9);
        assert!((there_and_back(0.5) - 1.0).abs() < 1e-6);
        assert!((there_and_back(1.0) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn there_and_back_with_pause_has_flat_middle() {
        assert!((there_and_back_with_pause(0.0, 0.4) - 0.0).abs() < 1e-9);
        assert!((there_and_back_with_pause(0.5, 0.4) - 1.0).abs() < 1e-9);
        assert!((there_and_back_with_pause(1.0, 0.4) - 0.0).abs() < 1e-6);
        let clamped = there_and_back_with_pause(0.5, 1.5);
        assert!((0.0..=1.0).contains(&clamped));
    }

    #[test]
    fn wiggle_oscillates_around_linear() {
        let w0 = wiggle(0.0, 2.0);
        let w1 = wiggle(1.0, 2.0);
        assert!((w0 - 0.0).abs() < 1e-6);
        assert!((w1 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn exponential_decay_approaches_one() {
        assert!((exponential_decay(0.0, 1.0) - 0.0).abs() < 1e-9);
        assert!(exponential_decay(1.0, 0.01) > 0.999);
        assert!(exponential_decay(2.0, 1.0) == exponential_decay(1.0, 1.0), "t is clamped to [0,1]");
        let clamped_half_life = exponential_decay(0.5, -1.0);
        assert!((0.0..=1.0).contains(&clamped_half_life));
    }

    #[test]
    fn sine_family_endpoints() {
        for f in [ease_in_sine as RateFunc, ease_out_sine, ease_in_out_sine] {
            assert!((f(0.0) - 0.0).abs() < 1e-6);
            assert!((f(1.0) - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn power_family_both_branches_of_in_out() {
        for f in [ease_in_out_quad as RateFunc, ease_in_out_cubic, ease_in_out_quart, ease_in_out_quint] {
            let low = f(0.25);
            let high = f(0.75);
            assert!(low < 0.5, "low half should stay under midpoint");
            assert!(high > 0.5, "high half should exceed midpoint");
            assert!((f(0.0) - 0.0).abs() < 1e-9);
            assert!((f(1.0) - 1.0).abs() < 1e-9);
        }
        for f in [ease_in_quad as RateFunc, ease_out_quad, ease_in_cubic, ease_out_cubic, ease_in_quart, ease_out_quart, ease_in_quint, ease_out_quint] {
            assert!((f(0.0) - 0.0).abs() < 1e-9);
            assert!((f(1.0) - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn exp_family_handles_boundary_and_branches() {
        assert_eq!(ease_in_exp(0.0), 0.0);
        assert!(ease_in_exp(1.0) > 0.99);
        assert_eq!(ease_out_exp(1.0), 1.0);
        assert!(ease_out_exp(0.0).abs() < 1e-9);
        assert_eq!(ease_in_out_exp(0.0), 0.0);
        assert_eq!(ease_in_out_exp(1.0), 1.0);
        assert!(ease_in_out_exp(0.25) < 0.5);
        assert!(ease_in_out_exp(0.75) > 0.5);
    }

    #[test]
    fn circ_family_both_branches() {
        for f in [ease_in_circ as RateFunc, ease_out_circ] {
            assert!((f(0.0) - 0.0).abs() < 1e-6);
            assert!((f(1.0) - 1.0).abs() < 1e-6);
        }
        assert!(ease_in_out_circ(0.25) < 0.5);
        assert!(ease_in_out_circ(0.75) > 0.5);
    }

    #[test]
    fn back_family_overshoots() {
        assert!(ease_in_back(0.1) < 0.0, "ease-in-back should dip negative early");
        assert!(ease_out_back(0.9) > 1.0, "ease-out-back should overshoot past one");
        assert!(ease_in_out_back(0.25) != ease_in_out_back(0.75));
    }

    #[test]
    fn elastic_family_boundary_and_branches() {
        for f in [ease_in_elastic as fn(f64) -> f64, ease_out_elastic, ease_in_out_elastic] {
            assert_eq!(f(0.0), 0.0);
            assert_eq!(f(1.0), 1.0);
        }
        let low = ease_in_out_elastic(0.25);
        let high = ease_in_out_elastic(0.75);
        assert!(low.is_finite() && high.is_finite());
    }

    #[test]
    fn bounce_family_all_segments() {
        let samples = [0.05, 0.3, 0.55, 0.9];
        for t in samples {
            let v = ease_out_bounce(t);
            assert!((0.0..=1.0).contains(&v), "ease_out_bounce({t}) out of range: {v}");
        }
        assert!((ease_out_bounce(0.0) - 0.0).abs() < 1e-9);
        assert!((ease_out_bounce(1.0) - 1.0).abs() < 1e-9);
        assert!((ease_in_bounce(0.0) - 0.0).abs() < 1e-9);
        assert!((ease_in_bounce(1.0) - 1.0).abs() < 1e-9);
        assert!(ease_in_out_bounce(0.25) < 0.5);
        assert!(ease_in_out_bounce(0.75) > 0.5);
    }

    #[test]
    fn rate_functions_clamp_out_of_range_input() {
        assert_eq!(linear(-1.0), 0.0);
        assert_eq!(linear(2.0), 1.0);
        assert_eq!(smooth(-5.0), smooth(0.0));
        assert_eq!(smooth(5.0), smooth(1.0));
    }
}
