//! 📈 Rate functions mapping linear time α ∈ [0,1] to eased progress.

use mathematical_geometry::clamp_f64;

/// 📐 Easing function signature used by animations.
pub type RateFunc = fn(f64) -> f64;

pub fn linear(t: f64) -> f64 {
    clamp01(t)
}

pub fn smooth(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * (3.0 - 2.0 * t)
}

pub fn rush_into(t: f64) -> f64 {
    let t = clamp01(t);
    2.0 * t * t
}

pub fn rush_from(t: f64) -> f64 {
    let t = clamp01(t);
    2.0 * t - t * t
}

pub fn slow_into(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * t
}

pub fn double_smooth(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn there_and_back(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        smooth(t * 2.0)
    } else {
        smooth(2.0 - t * 2.0)
    }
}

pub fn there_and_back_with_pause(t: f64, pause_ratio: f64) -> f64 {
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

pub fn running_start(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * (2.0 - t)
}

pub fn wiggle(t: f64, num_wiggles: f64) -> f64 {
    let t = clamp01(t);
    t + (std::f64::consts::TAU * num_wiggles * t).sin() / (std::f64::consts::TAU * num_wiggles).max(1.0)
}

pub fn lingering(t: f64) -> f64 {
    let t = clamp01(t);
    1.0 - (1.0 - t).powi(3)
}

pub fn exponential_decay(t: f64, half_life: f64) -> f64 {
    let t = clamp01(t);
    1.0 - 0.5_f64.powf(t / half_life.max(1e-9))
}

pub fn ease_in_sine(t: f64) -> f64 {
    1.0 - ((clamp01(t) * std::f64::consts::FRAC_PI_2).cos())
}

pub fn ease_out_sine(t: f64) -> f64 {
    (clamp01(t) * std::f64::consts::FRAC_PI_2).sin()
}

pub fn ease_in_out_sine(t: f64) -> f64 {
    -(0.5 * (std::f64::consts::PI * clamp01(t)).cos() - 0.5)
}

pub fn ease_in_quad(t: f64) -> f64 {
    let t = clamp01(t);
    t * t
}

pub fn ease_out_quad(t: f64) -> f64 {
    let t = clamp01(t);
    1.0 - (1.0 - t) * (1.0 - t)
}

pub fn ease_in_out_quad(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

pub fn ease_in_cubic(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * t
}

pub fn ease_out_cubic(t: f64) -> f64 {
    let t = clamp01(t);
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_in_out_cubic(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_in_quart(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * t * t
}

pub fn ease_out_quart(t: f64) -> f64 {
    let t = clamp01(t);
    1.0 - (1.0 - t).powi(4)
}

pub fn ease_in_out_quart(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

pub fn ease_in_quint(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * t * t * t
}

pub fn ease_out_quint(t: f64) -> f64 {
    let t = clamp01(t);
    1.0 - (1.0 - t).powi(5)
}

pub fn ease_in_out_quint(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

pub fn ease_in_exp(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 {
        0.0
    } else {
        2.0_f64.powf(10.0 * t - 10.0)
    }
}

pub fn ease_out_exp(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f64.powf(-10.0 * t)
    }
}

pub fn ease_in_out_exp(t: f64) -> f64 {
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

pub fn ease_in_circ(t: f64) -> f64 {
    let t = clamp01(t);
    1.0 - (1.0 - t * t).sqrt()
}

pub fn ease_out_circ(t: f64) -> f64 {
    let t = clamp01(t);
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

pub fn ease_in_out_circ(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

pub fn ease_in_back(t: f64) -> f64 {
    const C: f64 = 1.70158;
    let t = clamp01(t);
    (C + 1.0) * t * t * t - C * t * t
}

pub fn ease_out_back(t: f64) -> f64 {
    const C: f64 = 1.70158;
    let t = clamp01(t);
    1.0 + (C + 1.0) * (t - 1.0).powi(3) + C * (t - 1.0).powi(2)
}

pub fn ease_in_out_back(t: f64) -> f64 {
    const C: f64 = 1.70158 * 1.525;
    let t = clamp01(t);
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C + 1.0) * 2.0 * t - C)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C + 1.0) * (t * 2.0 - 2.0) + C) + 2.0) / 2.0
    }
}

pub fn ease_in_elastic(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    -(2.0_f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * std::f64::consts::TAU / 3.0).sin()
}

pub fn ease_out_elastic(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * std::f64::consts::TAU / 3.0).sin() + 1.0
}

pub fn ease_in_out_elastic(t: f64) -> f64 {
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

pub fn ease_in_bounce(t: f64) -> f64 {
    1.0 - ease_out_bounce(1.0 - clamp01(t))
}

pub fn ease_out_bounce(t: f64) -> f64 {
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

pub fn ease_in_out_bounce(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
    }
}

pub fn map_child_alpha(parent_alpha: f64, start: f64, end: f64) -> f64 {
    if end <= start {
        return if parent_alpha >= end { 1.0 } else { 0.0 };
    }
    clamp01((parent_alpha - start) / (end - start))
}

fn clamp01(t: f64) -> f64 {
    clamp_f64(t, 0.0, 1.0)
}

#[cfg(test)]
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
}
