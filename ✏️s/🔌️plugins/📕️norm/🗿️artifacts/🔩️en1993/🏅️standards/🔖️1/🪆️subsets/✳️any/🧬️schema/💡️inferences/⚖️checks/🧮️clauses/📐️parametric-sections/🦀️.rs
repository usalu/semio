//! 📐️ Section properties of parametric welded I, rectangular hollow and circular hollow sections with sharp corners (mm). See [EN 1993-1-1 §6.2.5](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Properties {
    pub area: f64,
    pub iy: f64,
    pub iz: f64,
    pub w_el_y: f64,
    pub w_pl_y: f64,
}

pub fn welded_i(h: f64, b: f64, tw: f64, tf: f64) -> Properties {
    let hw = h - 2.0 * tf;
    Properties {
        area: 2.0 * b * tf + hw * tw,
        iy: (b * h.powi(3) - (b - tw) * hw.powi(3)) / 12.0,
        iz: (2.0 * tf * b.powi(3) + hw * tw.powi(3)) / 12.0,
        w_el_y: (b * h.powi(3) - (b - tw) * hw.powi(3)) / (6.0 * h),
        w_pl_y: b * tf * (h - tf) + tw * hw * hw / 4.0,
    }
}

pub fn rhs(h: f64, b: f64, t: f64) -> Properties {
    let (hi, bi) = (h - 2.0 * t, b - 2.0 * t);
    Properties {
        area: b * h - bi * hi,
        iy: (b * h.powi(3) - bi * hi.powi(3)) / 12.0,
        iz: (h * b.powi(3) - hi * bi.powi(3)) / 12.0,
        w_el_y: (b * h.powi(3) - bi * hi.powi(3)) / (6.0 * h),
        w_pl_y: b * h * h / 4.0 - bi * hi * hi / 4.0,
    }
}

pub fn chs(d: f64, t: f64) -> Properties {
    let di = d - 2.0 * t;
    let i = PI / 64.0 * (d.powi(4) - di.powi(4));
    Properties { area: PI / 4.0 * (d * d - di * di), iy: i, iz: i, w_el_y: 2.0 * i / d, w_pl_y: (d.powi(3) - di.powi(3)) / 6.0 }
}

/// 📐️ Elastic and plastic section moduli (W_el,z, W_pl,z) of a sharp-corner rectangular hollow section about the minor axis.
pub fn rhs_minor_moduli(h: f64, b: f64, t: f64) -> (f64, f64) {
    let (hi, bi) = (h - 2.0 * t, b - 2.0 * t);
    ((h * b.powi(3) - hi * bi.powi(3)) / (6.0 * b), h * b * b / 4.0 - hi * bi * bi / 4.0)
}

/// 🌀️ Saint-Venant torsion constant I_t = π/32 (d⁴ − d_i⁴) of a circular hollow section.
pub fn chs_torsion(d: f64, t: f64) -> f64 {
    PI / 32.0 * (d.powi(4) - (d - 2.0 * t).powi(4))
}

/// 📏️ Largest relative deviation between two property sets over A, I_y, I_z, W_el,y and W_pl,y.
pub fn max_deviation(computed: &Properties, reference: &Properties) -> f64 {
    [(computed.area, reference.area), (computed.iy, reference.iy), (computed.iz, reference.iz), (computed.w_el_y, reference.w_el_y), (computed.w_pl_y, reference.w_pl_y)]
        .iter()
        .map(|(c, r)| ((c - r) / r).abs())
        .fold(0.0, f64::max)
}

/// 📐️ Section properties of an equal or unequal leg angle with sharp corners (legs `h` ≥ `b`, thickness `t`, mm): values about the principal axes u-u (major) and v-v (minor), the angle `alpha` (rad) from the axis parallel to the long leg (y-y) to u-u, the centroid from the heel corner and the Saint-Venant constant of the open thin-walled shape I_t = (h + b − t) t³/3. See [EN 1993-1-1 Figure 1.1](https://eurocodes.jrc.ec.europa.eu/).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AngleProperties {
    pub area: f64,
    pub y_c: f64,
    pub z_c: f64,
    pub iy: f64,
    pub iz: f64,
    pub iyz: f64,
    pub iu: f64,
    pub iv: f64,
    pub alpha: f64,
    pub w_el_u: f64,
    pub w_el_v: f64,
    pub w_pl_u: f64,
    pub w_pl_v: f64,
    pub it: f64,
}

fn angle_outline(h: f64, b: f64, t: f64) -> [(f64, f64); 6] {
    [(0.0, 0.0), (b, 0.0), (b, t), (t, t), (t, h), (0.0, h)]
}

fn clip_side(poly: &[(f64, f64)], axis: usize, cut: f64, above: bool) -> Vec<(f64, f64)> {
    let pick = |p: &(f64, f64)| if axis == 0 { p.0 } else { p.1 };
    let inside = |p: &(f64, f64)| if above { pick(p) >= cut } else { pick(p) <= cut };
    let mut out = Vec::new();
    for (i, p) in poly.iter().enumerate() {
        let q = &poly[(i + 1) % poly.len()];
        let (pi, qi) = (inside(p), inside(q));
        if pi {
            out.push(*p);
        }
        if pi != qi {
            let s = (cut - pick(p)) / (pick(q) - pick(p));
            out.push((p.0 + s * (q.0 - p.0), p.1 + s * (q.1 - p.1)));
        }
    }
    out
}

fn area_centroid(poly: &[(f64, f64)]) -> (f64, f64, f64) {
    let (mut a, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for (i, p) in poly.iter().enumerate() {
        let q = &poly[(i + 1) % poly.len()];
        let cross = p.0 * q.1 - q.0 * p.1;
        a += cross;
        cx += (p.0 + q.0) * cross;
        cy += (p.1 + q.1) * cross;
    }
    if a.abs() < 1e-300 {
        return (0.0, 0.0, 0.0);
    }
    (a / 2.0, cx / (3.0 * a), cy / (3.0 * a))
}

fn plastic_modulus(poly: &[(f64, f64)], axis: usize) -> f64 {
    let total = area_centroid(poly).0.abs();
    let pick = |p: &(f64, f64)| if axis == 0 { p.0 } else { p.1 };
    let (mut lo, mut hi) = (poly.iter().map(pick).fold(f64::INFINITY, f64::min), poly.iter().map(pick).fold(f64::NEG_INFINITY, f64::max));
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if area_centroid(&clip_side(poly, axis, mid, true)).0.abs() > total / 2.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let cut = 0.5 * (lo + hi);
    let (a_up, up_x, up_y) = area_centroid(&clip_side(poly, axis, cut, true));
    let (a_dn, dn_x, dn_y) = area_centroid(&clip_side(poly, axis, cut, false));
    let (up, dn) = if axis == 0 { (up_x, dn_x) } else { (up_y, dn_y) };
    a_up.abs() * (up - cut).abs() + a_dn.abs() * (cut - dn).abs()
}

/// 📐️ Properties of an angle section by the shoelace polygon integrals, the principal-axis rotation and the half-area plastic neutral axes (bisection); `h` is the long leg, `b` the short leg, `t` the thickness.
pub fn angle(h: f64, b: f64, t: f64) -> AngleProperties {
    let outline = angle_outline(h, b, t);
    let (area, y_c, z_c) = area_centroid(&outline);
    let (mut iy, mut iz, mut iyz) = (0.0, 0.0, 0.0);
    for (i, p) in outline.iter().enumerate() {
        let q = &outline[(i + 1) % outline.len()];
        let (x0, y0, x1, y1) = (p.0 - y_c, p.1 - z_c, q.0 - y_c, q.1 - z_c);
        let cross = x0 * y1 - x1 * y0;
        iy += cross * (y0 * y0 + y0 * y1 + y1 * y1) / 12.0;
        iz += cross * (x0 * x0 + x0 * x1 + x1 * x1) / 12.0;
        iyz += cross * (x0 * y1 + 2.0 * x0 * y0 + 2.0 * x1 * y1 + x1 * y0) / 24.0;
    }
    let about_u = |a: f64| iy * a.cos().powi(2) + iz * a.sin().powi(2) - iyz * (2.0 * a).sin();
    let mut alpha = 0.5 * (-2.0 * iyz).atan2(iy - iz);
    if about_u(alpha) < iy + iz - about_u(alpha) {
        alpha += std::f64::consts::FRAC_PI_2;
    }
    if alpha > std::f64::consts::FRAC_PI_2 {
        alpha -= std::f64::consts::PI;
    }
    let (iu, iv) = (about_u(alpha), iy + iz - about_u(alpha));
    let rotated: Vec<(f64, f64)> = outline.iter().map(|p| {
        let (y, z) = (p.0 - y_c, p.1 - z_c);
        (y * alpha.cos() + z * alpha.sin(), -y * alpha.sin() + z * alpha.cos())
    }).collect();
    let v_max = rotated.iter().map(|p| p.1.abs()).fold(0.0, f64::max);
    let u_max = rotated.iter().map(|p| p.0.abs()).fold(0.0, f64::max);
    AngleProperties { area, y_c, z_c, iy, iz, iyz, iu, iv, alpha, w_el_u: iu / v_max, w_el_v: iv / u_max, w_pl_u: plastic_modulus(&rotated, 1), w_pl_v: plastic_modulus(&rotated, 0), it: (h + b - t) * t.powi(3) / 3.0 }
}

/// 📐️ The principal-axis [`Properties`] of an angle: I_y and I_z are the principal I_u and I_v, the moduli those about u-u (the convention of the steel section row of kind `Angle`).
pub fn angle_principal(h: f64, b: f64, t: f64) -> Properties {
    let p = angle(h, b, t);
    Properties { area: p.area, iy: p.iu, iz: p.iv, w_el_y: p.w_el_u, w_pl_y: p.w_pl_u }
}

#[cfg(test)]
mod angle_tests {
    use super::*;

    const ANGLE_ORACLE: [(f64, f64, f64, [f64; 8]); 3] = [
        (100.0, 65.0, 8.0, [1256.000000000001, 1466399.7025995713, 251794.5097146593, -157.17872401640003, 21690.00450624167, 7085.495276210156, 37534.051633358984, 14743.437207670224]),
        (100.0, 100.0, 10.0, [1900.000000000001, 2865833.333333334, 734254.3859649161, -135.0, 40529.00367500896, 18100.41991413465, 63875.312567184796, 32202.82132653735]),
        (150.0, 90.0, 12.0, [2735.999999999998, 7011878.547006284, 1049252.8214147547, -160.05420483452355, 70126.56243483313, 20552.92558240696, 121390.139068052, 43871.73927080554]),
    ];
    const CHS_ORACLE: [(f64, f64, f64, f64, f64, f64); 3] = [
        (168.3, 6.3, 3205.02197878802, 10525747.577816918, 125083.15600495442, 165320.920852083),
        (114.3, 4.0, 1385.5141064484637, 2108960.0742407087, 36902.18852564675, 48656.37130843337),
        (60.3, 2.9, 522.7395243866956, 215750.52756367688, 7155.9047284801645, 9557.174180462524),
    ];

    fn close(a: f64, b: f64, tol: f64) -> bool {
        ((a - b) / b).abs() <= tol
    }

    #[test]
    fn angles_match_the_sectionproperties_oracle() {
        for (h, b, t, o) in ANGLE_ORACLE {
            let p = angle(h, b, t);
            assert!(close(p.area, o[0], 1e-9), "A {} {}", p.area, o[0]);
            assert!(close(p.iu, o[1], 1e-6), "I_u {} {}", p.iu, o[1]);
            assert!(close(p.iv, o[2], 1e-6), "I_v {} {}", p.iv, o[2]);
            let d = (p.alpha - o[3].to_radians()).rem_euclid(std::f64::consts::PI);
            assert!(d.min(std::f64::consts::PI - d) < 1e-9, "alpha {} {}", p.alpha.to_degrees(), o[3]);
            assert!(close(p.w_el_u, o[4], 1e-6), "W_el,u {} {}", p.w_el_u, o[4]);
            assert!(close(p.w_el_v, o[5], 1e-6), "W_el,v {} {}", p.w_el_v, o[5]);
            assert!(close(p.w_pl_u, o[6], 1e-5), "W_pl,u {} {}", p.w_pl_u, o[6]);
            assert!(close(p.w_pl_v, o[7], 1e-5), "W_pl,v {} {}", p.w_pl_v, o[7]);
        }
    }

    #[test]
    fn the_equal_angle_has_the_principal_axis_at_45_degrees() {
        let p = angle(100.0, 100.0, 10.0);
        assert!((p.alpha.abs() - std::f64::consts::FRAC_PI_4).abs() < 1e-9 || (p.alpha.abs() - 3.0 * std::f64::consts::FRAC_PI_4).abs() < 1e-9);
        assert!((p.iy - p.iz).abs() / p.iy < 1e-12);
        assert!((p.iu + p.iv - p.iy - p.iz).abs() / (p.iy + p.iz) < 1e-12);
    }

    #[test]
    fn the_thin_walled_torsion_constant_is_within_four_percent_of_the_finite_element_value() {
        for (h, b, t, fe) in [(100.0, 65.0, 8.0, 26_287.432_540_168_753), (100.0, 100.0, 10.0, 62_022.187_666_814_774), (150.0, 90.0, 12.0, 128_608.487_875_492_31)] {
            assert!(close(angle(h, b, t).it, fe, 0.04), "I_t {h} {b} {t}");
        }
    }

    #[test]
    fn circular_hollow_sections_match_the_sectionproperties_oracle() {
        for (d, t, area, i, w_el, w_pl) in CHS_ORACLE {
            let p = chs(d, t);
            assert!(close(p.area, area, 2e-3), "A {d}");
            assert!(close(p.iy, i, 3e-3), "I {d}");
            assert!(close(p.w_el_y, w_el, 3e-3), "W_el {d}");
            assert!(close(p.w_pl_y, w_pl, 3e-3), "W_pl {d}");
        }
    }
}

/// 📐️ Shift e_N,v (mm, along the minor principal axis direction v) of the centroid of the effective angle under uniform compression: the free tips of the long leg (`rho_h`) and the short leg (`rho_b`) lose the width (1 − ρ)(leg − t) (EN 1993-1-5 Table 4.2 outstand).
pub fn angle_tip_shift(h: f64, b: f64, t: f64, rho_h: f64, rho_b: f64) -> f64 {
    let p = angle(h, b, t);
    let (d_h, d_b) = ((1.0 - rho_h) * (h - t).max(0.0), (1.0 - rho_b) * (b - t).max(0.0));
    let v_of = |y: f64, z: f64| -(y - p.y_c) * p.alpha.sin() + (z - p.z_c) * p.alpha.cos();
    let removed = t * d_h * v_of(t / 2.0, h - d_h / 2.0) + t * d_b * v_of(b - d_b / 2.0, t / 2.0);
    -removed / (p.area - t * (d_h + d_b)).max(1e-12)
}
