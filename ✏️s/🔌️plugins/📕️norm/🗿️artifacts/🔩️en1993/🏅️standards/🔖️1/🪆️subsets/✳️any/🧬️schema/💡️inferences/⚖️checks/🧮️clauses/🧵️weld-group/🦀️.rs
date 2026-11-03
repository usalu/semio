//! 🧵️ Weld groups under in-plane shear and torsion by the elastic vector method with the simplified weld resistance of EN 1993-1-8 4.5.3.3 (mm, N). See [EN 1993-1-8 §4.5](https://eurocodes.jrc.ec.europa.eu/).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeldPattern {
    Line,
    TwoLines,
    Box,
}

/// 📐️ Line properties per unit throat thickness of a symmetric group of width `b` and depth `d` about its centroid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupProperties {
    pub length: f64,
    pub i_polar: f64,
    pub x_max: f64,
    pub y_max: f64,
}

pub fn group_properties(pattern: WeldPattern, b: f64, d: f64) -> GroupProperties {
    match pattern {
        WeldPattern::Line => GroupProperties { length: d, i_polar: d.powi(3) / 12.0, x_max: 0.0, y_max: d / 2.0 },
        WeldPattern::TwoLines => GroupProperties { length: 2.0 * d, i_polar: 2.0 * d.powi(3) / 12.0 + 2.0 * d * (b / 2.0).powi(2), x_max: b / 2.0, y_max: d / 2.0 },
        WeldPattern::Box => GroupProperties {
            length: 2.0 * (b + d),
            i_polar: 2.0 * b * (d / 2.0).powi(2) + 2.0 * d.powi(3) / 12.0 + 2.0 * d * (b / 2.0).powi(2) + 2.0 * b.powi(3) / 12.0,
            x_max: b / 2.0,
            y_max: d / 2.0,
        },
    }
}

/// 🧮️ Largest resultant force per unit length (N/mm) at the far corner for the shear force `f` acting along the depth at the eccentricity `e` from the centroid.
pub fn max_force_per_length(p: &GroupProperties, f: f64, e: f64) -> f64 {
    let t = f * e;
    let direct = f / p.length;
    let along_depth = direct + t * p.x_max / p.i_polar;
    let along_width = t * p.y_max / p.i_polar;
    along_depth.hypot(along_width)
}

/// 🧭️ Force per unit length (N/mm) `(q_x, q_y)` at the weld point `(x, y)` from the centroid: direct share F/L_w along the depth plus the torsion T = F e perpendicular to the radius, T r/I_p.
pub fn vector_at(p: &GroupProperties, f: f64, e: f64, x: f64, y: f64) -> (f64, f64) {
    let t = f * e;
    (-t * y / p.i_polar, f / p.length + t * x / p.i_polar)
}

/// 🧭️ Points on the weld lines of the group (mm, from the centroid), `n` per line: one vertical line, two vertical lines at ±b/2, or the box ring.
pub fn sample_points(pattern: WeldPattern, b: f64, d: f64, n: usize) -> Vec<(f64, f64)> {
    let n = n.max(2);
    let along = |x0: f64, y0: f64, x1: f64, y1: f64| (0..n).map(move |i| (x0 + (x1 - x0) * i as f64 / (n - 1) as f64, y0 + (y1 - y0) * i as f64 / (n - 1) as f64));
    match pattern {
        WeldPattern::Line => along(0.0, -d / 2.0, 0.0, d / 2.0).collect(),
        WeldPattern::TwoLines => along(-b / 2.0, -d / 2.0, -b / 2.0, d / 2.0).chain(along(b / 2.0, -d / 2.0, b / 2.0, d / 2.0)).collect(),
        WeldPattern::Box => along(-b / 2.0, -d / 2.0, -b / 2.0, d / 2.0).chain(along(b / 2.0, -d / 2.0, b / 2.0, d / 2.0)).chain(along(-b / 2.0, -d / 2.0, b / 2.0, -d / 2.0)).chain(along(-b / 2.0, d / 2.0, b / 2.0, d / 2.0)).collect(),
    }
}

/// 🧱️ Simplified design resistance per unit length F_w,Rd = f_u/√3/(β_w γ_M2) a (4.5.3.3).
pub fn resistance_per_length(a: f64, fu: f64, beta_w: f64, gm2: f64) -> f64 {
    fu / 3.0_f64.sqrt() / (beta_w * gm2) * a
}
