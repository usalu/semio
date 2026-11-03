//! 🏠️ EN 1993-1-3 purlins restrained by sheeting: the free flange with the lip and a fifth of the web, the lateral spring of the sheeting and the free-flange moment of a simply supported span on that spring (mm, N, MPa). See [EN 1993-1-3 Annex E](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreeFlange {
    pub area: f64,
    pub inertia: f64,
    pub y_max: f64,
}

/// 🏠️ Free flange of thickness `t`, width `b`, lip `c` and a fifth of the web `hw`: area, second moment about the vertical centroidal axis and extreme fibre distance.
pub fn free_flange(t: f64, b: f64, c: f64, hw: f64) -> FreeFlange {
    let side = c + hw / 5.0;
    let area = t * (b + side);
    let y_c = t * (c - hw / 5.0) * b / 2.0 / area;
    let inertia = t * b.powi(3) / 12.0 + t * side * (b / 2.0).powi(2) - area * y_c * y_c;
    FreeFlange { area, inertia, y_max: b / 2.0 + y_c.abs() }
}

/// 🏠️ Lateral spring of the sheeting acting on the free flange, K = C_D/h² per unit length.
pub fn lateral_spring(c_d: f64, h: f64) -> f64 {
    c_d / (h * h)
}

/// 🏠️ Moment at `x` of a simply supported beam of bending stiffness `ei` and span `l` on a continuous spring `k` under the uniform load `q`: Σ over odd n of 4q/(nπ) sin(nπx/L) EI a²/(EI a⁴ + K) with a = nπ/L.
pub fn moment_on_spring(q: f64, l: f64, x: f64, ei: f64, k: f64) -> f64 {
    (0..200).map(|j| {
        let n = (2 * j + 1) as f64;
        let a = n * PI / l;
        4.0 * q / (n * PI) * (n * PI * x / l).sin() * ei * a * a / (ei * a.powi(4) + k)
    }).sum()
}

/// 🏠️ Largest absolute moment over the half span (41 stations), which is at midspan without a spring and moves towards the supports for a stiff spring.
pub fn peak_moment_on_spring(q: f64, l: f64, ei: f64, k: f64) -> f64 {
    (0..=40).map(|i| moment_on_spring(q, l, l * i as f64 / 80.0, ei, k).abs()).fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EI: f64 = 210000.0 * 2.0e5;

    #[test]
    fn without_a_spring_the_moment_is_q_l_squared_over_eight() {
        let m = moment_on_spring(0.5, 6000.0, 3000.0, EI, 0.0);
        assert!((m - 0.5 * 6000.0 * 6000.0 / 8.0).abs() < 1e-6 * m);
        assert!((peak_moment_on_spring(0.5, 6000.0, EI, 0.0) - m).abs() < 1e-9 * m);
    }

    #[test]
    fn the_series_reproduces_the_finite_difference_solution() {
        assert!((moment_on_spring(0.5, 6000.0, 3000.0, EI, 0.01) - 488_196.86).abs() < 0.5);
        assert!((moment_on_spring(0.5, 6000.0, 3000.0, EI, 0.1) - 22_308.62).abs() < 0.5);
        assert!((moment_on_spring(0.5, 6000.0, 3000.0, EI, 10.0) - 13.8139).abs() < 1e-3);
    }

    #[test]
    fn the_spring_relieves_the_free_flange() {
        let moments: Vec<f64> = [0.0, 0.01, 0.1, 10.0].iter().map(|k| peak_moment_on_spring(0.5, 6000.0, EI, *k)).collect();
        assert!(moments.windows(2).all(|w| w[0] > w[1]));
    }

    #[test]
    fn a_stiff_spring_reverses_the_midspan_moment_and_the_peak_follows() {
        assert!(moment_on_spring(0.5, 6000.0, 3000.0, EI, 1.0) < 0.0);
        assert!(peak_moment_on_spring(0.5, 6000.0, EI, 1.0) >= 1889.0);
    }

    #[test]
    fn free_flange_of_a_symmetric_lip_and_web_fifth_is_centred() {
        let f = free_flange(2.0, 60.0, 20.0, 100.0);
        assert!((f.area - 2.0 * (60.0 + 40.0)).abs() < 1e-12);
        assert!((f.y_max - 30.0).abs() < 1e-12);
        assert!((f.inertia - (2.0 * 60.0_f64.powi(3) / 12.0 + 2.0 * 40.0 * 900.0)).abs() < 1e-9);
    }

    #[test]
    fn free_flange_with_a_longer_lip_shifts_its_centroid_to_the_lip() {
        let f = free_flange(2.0, 60.0, 40.0, 100.0);
        let side = 60.0;
        let area = 2.0 * (60.0 + side);
        let y_c = 2.0 * 20.0 * 30.0 / area;
        assert!((f.y_max - (30.0 + y_c)).abs() < 1e-12);
        assert!((f.inertia - (2.0 * 216000.0 / 12.0 + 2.0 * side * 900.0 - area * y_c * y_c)).abs() < 1e-9);
    }

    #[test]
    fn lateral_spring_divides_by_the_square_of_the_depth() {
        assert!((lateral_spring(4000.0, 100.0) - 0.4).abs() < 1e-12);
    }
}
