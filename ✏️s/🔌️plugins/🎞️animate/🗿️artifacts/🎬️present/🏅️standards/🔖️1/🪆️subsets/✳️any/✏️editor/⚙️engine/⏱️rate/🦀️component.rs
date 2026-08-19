//! 🎞️ Animate app engine facet: ⏱️rate (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/⏱️rate`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod rate {
    //! 📈️ Rate functions mapping linear time α ∈ [0,1] to eased progress.

    use geometry::clamp_f64;

    /// 📐️ Easing function signature used by animations.
    pub type RateFunc = fn(f64) -> f64;

    pub async fn linear(t: f64) -> f64 {
        clamp01(t)
    }

    pub async fn smooth(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * (3.0 - 2.0 * t)
    }

    pub async fn rush_into(t: f64) -> f64 {
        let t = clamp01(t);
        2.0 * t * t
    }

    pub async fn rush_from(t: f64) -> f64 {
        let t = clamp01(t);
        2.0 * t - t * t
    }

    pub async fn slow_into(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t
    }

    pub async fn double_smooth(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    pub async fn there_and_back(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            smooth(t * 2.0)
        } else {
            smooth(2.0 - t * 2.0)
        }
    }

    pub async fn there_and_back_with_pause(t: f64, pause_ratio: f64) -> f64 {
        let t = clamp01(t);
        let pause = pause_ratio.clamp(0.0, 0.9);
        let edge = (1.0 - pause) / 2.0;
        if t < edge {
            smooth(t / edge)
        } else if t < edge + pause {
            1.0
        } else {
            smooth(1.0 - (t - edge - pause) / edge)
        }
    }

    pub async fn running_start(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * (2.0 - t)
    }

    pub async fn wiggle(t: f64, num_wiggles: f64) -> f64 {
        let t = clamp01(t);
        t + (std::f64::consts::TAU * num_wiggles * t).sin() / (std::f64::consts::TAU * num_wiggles).max(1.0)
    }

    pub async fn lingering(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(3)
    }

    pub async fn exponential_decay(t: f64, half_life: f64) -> f64 {
        let t = clamp01(t);
        1.0 - 0.5_f64.powf(t / half_life.max(1e-9))
    }

    pub async fn ease_in_sine(t: f64) -> f64 {
        1.0 - ((clamp01(t) * std::f64::consts::FRAC_PI_2).cos())
    }

    pub async fn ease_out_sine(t: f64) -> f64 {
        (clamp01(t) * std::f64::consts::FRAC_PI_2).sin()
    }

    pub async fn ease_in_out_sine(t: f64) -> f64 {
        -(0.5 * (std::f64::consts::PI * clamp01(t)).cos() - 0.5)
    }

    pub async fn ease_in_quad(t: f64) -> f64 {
        let t = clamp01(t);
        t * t
    }

    pub async fn ease_out_quad(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t) * (1.0 - t)
    }

    pub async fn ease_in_out_quad(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }

    pub async fn ease_in_cubic(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t
    }

    pub async fn ease_out_cubic(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(3)
    }

    pub async fn ease_in_out_cubic(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    pub async fn ease_in_quart(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t * t
    }

    pub async fn ease_out_quart(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(4)
    }

    pub async fn ease_in_out_quart(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
        }
    }

    pub async fn ease_in_quint(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t * t * t
    }

    pub async fn ease_out_quint(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(5)
    }

    pub async fn ease_in_out_quint(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
        }
    }

    pub async fn ease_in_exp(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 {
            0.0
        } else {
            2.0_f64.powf(10.0 * t - 10.0)
        }
    }

    pub async fn ease_out_exp(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2.0_f64.powf(-10.0 * t)
        }
    }

    pub async fn ease_in_out_exp(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            2.0_f64.powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
        }
    }

    pub async fn ease_in_circ(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t * t).sqrt()
    }

    pub async fn ease_out_circ(t: f64) -> f64 {
        let t = clamp01(t);
        (1.0 - (t - 1.0).powi(2)).sqrt()
    }

    pub async fn ease_in_out_circ(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
        } else {
            ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
        }
    }

    pub async fn ease_in_back(t: f64) -> f64 {
        const C: f64 = 1.70158;
        let t = clamp01(t);
        (C + 1.0) * t * t * t - C * t * t
    }

    pub async fn ease_out_back(t: f64) -> f64 {
        const C: f64 = 1.70158;
        let t = clamp01(t);
        1.0 + (C + 1.0) * (t - 1.0).powi(3) + C * (t - 1.0).powi(2)
    }

    pub async fn ease_in_out_back(t: f64) -> f64 {
        const C: f64 = 1.70158 * 1.525;
        let t = clamp01(t);
        if t < 0.5 {
            ((2.0 * t).powi(2) * ((C + 1.0) * 2.0 * t - C)) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((C + 1.0) * (t * 2.0 - 2.0) + C) + 2.0) / 2.0
        }
    }

    pub async fn ease_in_elastic(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        -(2.0_f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * std::f64::consts::TAU / 3.0).sin()
    }

    pub async fn ease_out_elastic(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * std::f64::consts::TAU / 3.0).sin() + 1.0
    }

    pub async fn ease_in_out_elastic(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        if t < 0.5 {
            -((2.0_f64.powf(20.0 * t - 10.0)) * ((20.0 * t - 11.125) * std::f64::consts::TAU / 4.5).sin()) / 2.0
        } else {
            (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * std::f64::consts::TAU / 4.5).sin()) / 2.0 + 1.0
        }
    }

    pub async fn ease_in_bounce(t: f64) -> f64 {
        1.0 - ease_out_bounce(1.0 - clamp01(t))
    }

    pub async fn ease_out_bounce(t: f64) -> f64 {
        let t = clamp01(t);
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;
        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            let t = t - 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            let t = t - 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }

    pub async fn ease_in_out_bounce(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
        }
    }

    pub async fn map_child_alpha(parent_alpha: f64, start: f64, end: f64) -> f64 {
        if end <= start {
            return if parent_alpha >= end { 1.0 } else { 0.0 };
        }
        clamp01((parent_alpha - start) / (end - start))
    }

    async fn clamp01(t: f64) -> f64 {
        clamp_f64(t, 0.0, 1.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        async fn endpoints_are_zero_and_one() {
            for f in [linear as RateFunc, smooth, ease_in_out_cubic, ease_out_bounce] {
                assert!((f(0.0) - 0.0).abs() < 1e-6);
                assert!((f(1.0) - 1.0).abs() < 1e-6);
            }
        }

        #[test]
        async fn map_child_alpha_splits_interval() {
            assert_eq!(map_child_alpha(0.25, 0.0, 0.5), 0.5);
            assert_eq!(map_child_alpha(0.75, 0.5, 1.0), 0.5);
        }

        #[test]
        async fn map_child_alpha_degenerate_interval_is_step() {
            assert_eq!(map_child_alpha(0.4, 0.5, 0.5), 0.0);
            assert_eq!(map_child_alpha(0.6, 0.5, 0.5), 1.0);
            assert_eq!(map_child_alpha(0.5, 0.7, 0.2), 1.0);
        }

        #[test]
        async fn simple_easing_functions_are_monotonic_within_range() {
            for f in [rush_from as RateFunc, slow_into, running_start, lingering] {
                assert!((f(0.0) - 0.0).abs() < 1e-6, "expected 0 at t=0");
                assert!((f(1.0) - 1.0).abs() < 1e-6, "expected 1 at t=1");
                let mid = f(0.5);
                assert!((0.0..=1.0).contains(&mid));
            }
        }

        #[test]
        async fn rush_into_overshoots_past_one_at_t_equals_one() {
            assert!((rush_into(0.0) - 0.0).abs() < 1e-9);
            assert!((rush_into(1.0) - 2.0).abs() < 1e-9);
            assert!(rush_into(0.5) < 1.0);
        }

        #[test]
        async fn double_smooth_covers_both_branches() {
            assert!(double_smooth(0.25) > 0.0 && double_smooth(0.25) < 0.5);
            assert!(double_smooth(0.75) > 0.5 && double_smooth(0.75) < 1.0);
            assert!((double_smooth(0.0) - 0.0).abs() < 1e-9);
            assert!((double_smooth(1.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        async fn there_and_back_returns_to_start() {
            assert!((there_and_back(0.0) - 0.0).abs() < 1e-9);
            assert!((there_and_back(0.5) - 1.0).abs() < 1e-6);
            assert!((there_and_back(1.0) - 0.0).abs() < 1e-9);
        }

        #[test]
        async fn there_and_back_with_pause_has_flat_middle() {
            assert!((there_and_back_with_pause(0.0, 0.4) - 0.0).abs() < 1e-9);
            assert!((there_and_back_with_pause(0.5, 0.4) - 1.0).abs() < 1e-9);
            assert!((there_and_back_with_pause(1.0, 0.4) - 0.0).abs() < 1e-6);
            let clamped = there_and_back_with_pause(0.5, 1.5);
            assert!((0.0..=1.0).contains(&clamped));
        }

        #[test]
        async fn wiggle_oscillates_around_linear() {
            let w0 = wiggle(0.0, 2.0);
            let w1 = wiggle(1.0, 2.0);
            assert!((w0 - 0.0).abs() < 1e-6);
            assert!((w1 - 1.0).abs() < 1e-6);
        }

        #[test]
        async fn exponential_decay_approaches_one() {
            assert!((exponential_decay(0.0, 1.0) - 0.0).abs() < 1e-9);
            assert!(exponential_decay(1.0, 0.01) > 0.999);
            assert!(exponential_decay(2.0, 1.0) == exponential_decay(1.0, 1.0), "t is clamped to [0,1]");
            let clamped_half_life = exponential_decay(0.5, -1.0);
            assert!((0.0..=1.0).contains(&clamped_half_life));
        }

        #[test]
        async fn sine_family_endpoints() {
            for f in [ease_in_sine as RateFunc, ease_out_sine, ease_in_out_sine] {
                assert!((f(0.0) - 0.0).abs() < 1e-6);
                assert!((f(1.0) - 1.0).abs() < 1e-6);
            }
        }

        #[test]
        async fn power_family_both_branches_of_in_out() {
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
        async fn exp_family_handles_boundary_and_branches() {
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
        async fn circ_family_both_branches() {
            for f in [ease_in_circ as RateFunc, ease_out_circ] {
                assert!((f(0.0) - 0.0).abs() < 1e-6);
                assert!((f(1.0) - 1.0).abs() < 1e-6);
            }
            assert!(ease_in_out_circ(0.25) < 0.5);
            assert!(ease_in_out_circ(0.75) > 0.5);
        }

        #[test]
        async fn back_family_overshoots() {
            assert!(ease_in_back(0.1) < 0.0, "ease-in-back should dip negative early");
            assert!(ease_out_back(0.9) > 1.0, "ease-out-back should overshoot past one");
            assert!(ease_in_out_back(0.25) != ease_in_out_back(0.75));
        }

        #[test]
        async fn elastic_family_boundary_and_branches() {
            for f in [ease_in_elastic as fn(f64) -> f64, ease_out_elastic, ease_in_out_elastic] {
                assert_eq!(f(0.0), 0.0);
                assert_eq!(f(1.0), 1.0);
            }
            let low = ease_in_out_elastic(0.25);
            let high = ease_in_out_elastic(0.75);
            assert!(low.is_finite() && high.is_finite());
        }

        #[test]
        async fn bounce_family_all_segments() {
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
        async fn rate_functions_clamp_out_of_range_input() {
            assert_eq!(linear(-1.0), 0.0);
            assert_eq!(linear(2.0), 1.0);
            assert_eq!(smooth(-5.0), smooth(0.0));
            assert_eq!(smooth(5.0), smooth(1.0));
        }
    }
}

pub mod updater {
    //! 🔄️ Runtime updaters, value trackers, and always-redraw helpers.

    use crate::editor::animate::engine::scene::sobject::Sobject;
    use std::sync::{Arc, Mutex};

    /// 🎚️ Scalar animated parameter with get/set hooks.
    #[derive(Clone)]
    pub struct ValueTracker {
        pub value: Arc<Mutex<f64>>,
    }

    impl ValueTracker {
        pub async fn new(value: f64) -> Self {
            Self { value: Arc::new(Mutex::new(value)) }
        }

        pub async fn get(&self) -> f64 {
            *self.value.lock().unwrap_or_else(|e| e.into_inner())
        }

        pub async fn set(&self, value: f64) {
            *self.value.lock().unwrap_or_else(|e| e.into_inner()) = value;
        }

        pub async fn increment(&self, delta: f64) {
            let mut v = self.value.lock().unwrap_or_else(|e| e.into_inner());
            *v += delta;
        }
    }

    /// 🔁️ Per-frame mutation callback attached to an Sobject.
    #[derive(Clone)]
    pub struct Updater {
        pub id: u64,
        pub name: String,
        pub active: bool,
        pub dt_scale: f64,
        callback: Arc<dyn Fn(&mut dyn Sobject, f64) + Send + Sync>,
    }


    impl Updater {
        pub async fn new<F>(name: impl Into<String>, callback: F) -> Self
        where
            F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
        {
            Self { id: ({ u64::from_str_radix(&blake3::hash(concat!(file!(), line!()).as_bytes()).to_hex()[..8], 16).unwrap_or(1) }), name: name.into(), active: true, dt_scale: 1.0, callback: Arc::new(callback) }
        }

        pub async fn invoke(&self, target: &mut dyn Sobject, dt: f64) {
            if self.active {
                (self.callback)(target, dt * self.dt_scale);
            }
        }
    }

    /// ➕️ Attach an updater to an Sobject.
    pub async fn add_updater(target: &mut dyn Sobject, updater: Updater) {
        target.updaters_mut().push(updater);
    }

    /// ♾️ Attach an updater that runs every frame.
    pub async fn always<F>(target: &mut dyn Sobject, name: impl Into<String>, f: F)
    where
        F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
    {
        add_updater(target, Updater::new(name, f));
    }

    /// 🎯️ Attach an updater driven by a ValueTracker.
    pub async fn f_always<F>(target: &mut dyn Sobject, tracker: &ValueTracker, name: impl Into<String>, f: F)
    where
        F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
    {
        let t = tracker.clone();
        add_updater(
            target,
            Updater::new(name, move |obj, dt| {
                let _ = t.get();
                f(obj, dt);
            }),
        );
    }

    /// 🔃️ Rebuild an Sobject every frame from a factory closure.
    pub async fn always_redraw<F>(target: &mut dyn Sobject, name: impl Into<String>, factory: F)
    where
        F: Fn() -> Box<dyn Sobject> + Send + Sync + 'static,
    {
        let factory = Arc::new(factory);
        add_updater(
            target,
            Updater::new(name, move |obj, _dt| {
                let fresh = factory();
                if let Some(v) = obj.as_any_mut().downcast_mut::<crate::editor::animate::engine::scene::sobject::VSobject>() {
                    if let Some(fv) = fresh.as_any().downcast_ref::<crate::editor::animate::engine::scene::sobject::VSobject>() {
                        v.paths = fv.paths.clone();
                        v.style = fv.style.clone();
                        v.transform = fv.transform;
                    }
                }
            }),
        );
    }

    /// 🏃️ Run all updaters on a scene object tree.
    pub async fn run_updaters(target: &mut dyn Sobject, dt: f64) {
        let updaters: Vec<Updater> = target.updaters().to_vec();
        for u in updaters {
            u.invoke(target, dt);
        }
        target.visit_children_mut(&mut |child| run_updaters(child, dt));
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::editor::animate::engine::scene::sobject::VSobject;
        use geometry::BezPath;

        #[test]
        async fn value_tracker_mutates() {
            let t = ValueTracker::new(1.0);
            t.increment(2.0);
            assert!((t.get() - 3.0).abs() < 1e-9);
        }

        #[test]
        async fn updater_runs_on_object() {
            let mut v = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            add_updater(
                &mut v,
                Updater::new("mark", move |_o, _dt| {
                    *f.lock().unwrap() = true;
                }),
            );
            run_updaters(&mut v, 1.0 / 60.0);
            assert!(*flag.lock().unwrap());
        }

        #[test]
        async fn inactive_updater_does_not_invoke_callback() {
            let mut v = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            let mut u = Updater::new("mark", move |_o, _dt| {
                *f.lock().unwrap() = true;
            });
            u.active = false;
            u.invoke(&mut v, 1.0 / 60.0);
            assert!(!*flag.lock().unwrap());
        }

        #[test]
        async fn always_helper_attaches_updater_that_runs() {
            let mut v = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            always(&mut v, "always-mark", move |_o, _dt| {
                *f.lock().unwrap() = true;
            });
            assert_eq!(v.updaters().len(), 1);
            run_updaters(&mut v, 1.0 / 60.0);
            assert!(*flag.lock().unwrap());
        }

        #[test]
        async fn f_always_helper_reads_tracker_and_runs() {
            let mut v = VSobject::new();
            let tracker = ValueTracker::new(2.0);
            let flag: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
            let f = Arc::clone(&flag);
            f_always(&mut v, &tracker, "f-always-mark", move |_o, _dt| {
                *f.lock().unwrap() = 1.0;
            });
            tracker.set(9.0);
            run_updaters(&mut v, 1.0 / 60.0);
            assert!((*flag.lock().unwrap() - 1.0_f64).abs() < 1e-9);
        }

        #[test]
        async fn always_redraw_rebuilds_paths_from_factory() {
            let mut v = VSobject::new();
            always_redraw(&mut v, "redraw", || {
                let mut fresh = VSobject::new();
                fresh.paths.push(BezPath::new());
                Box::new(fresh) as Box<dyn Sobject>
            });
            assert!(v.paths.is_empty());
            run_updaters(&mut v, 1.0 / 60.0);
            assert_eq!(v.paths.len(), 1);
        }

        #[test]
        async fn run_updaters_recurses_into_group_children() {
            let mut child = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            add_updater(
                &mut child,
                Updater::new("child-mark", move |_o, _dt| {
                    *f.lock().unwrap() = true;
                }),
            );
            let mut group = crate::editor::animate::engine::scene::sobject::Group::new(vec![Box::new(child)]);
            run_updaters(&mut group, 1.0 / 60.0);
            assert!(*flag.lock().unwrap());
        }
    }
}
