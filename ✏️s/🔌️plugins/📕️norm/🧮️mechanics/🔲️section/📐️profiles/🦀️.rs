//! 📐️ Outlines of the standard steel and aluminium profile families as `(outer, holes)` polygons in metres, y horizontal and z up, with root, toe and corner radii discretised in `n` points per quarter arc; the geometry follows the construction of the `sectionproperties` library so that the oracle cases of the Eurocode artifacts compare like with like.

use std::f64::consts::{FRAC_PI_2, PI};

pub type Outline = (Vec<[f64; 2]>, Vec<Vec<[f64; 2]>>);

/// 🌈️ Arc of `n` points around `c` with radius `r` from angle `theta`, counter-clockwise or clockwise over a quarter turn; a zero radius yields the centre only.
pub fn arc(c: [f64; 2], r: f64, theta: f64, n: usize, ccw: bool) -> Vec<[f64; 2]> {
    if r == 0.0 {
        return vec![c];
    }
    let sign = if ccw { 1.0 } else { -1.0 };
    (0..n).map(|i| {
        let t = theta + sign * i as f64 / n.saturating_sub(1).max(1) as f64 * FRAC_PI_2;
        [c[0] + r * t.cos(), c[1] + r * t.sin()]
    }).collect()
}

/// 🏗️ Doubly symmetric I or H section, depth `d`, width `b`, flange `tf`, web `tw`, root radius `r`.
pub fn i_section(d: f64, b: f64, tf: f64, tw: f64, r: f64, n: usize) -> Outline {
    let (xl, xr) = (b / 2.0 - tw / 2.0, b / 2.0 + tw / 2.0);
    let mut p = vec![[0.0, 0.0], [b, 0.0], [b, tf]];
    p.extend(arc([xr + r, tf + r], r, 1.5 * PI, n, false));
    p.extend(arc([xr + r, d - tf - r], r, PI, n, false));
    p.extend([[b, d - tf], [b, d], [0.0, d], [0.0, d - tf]]);
    p.extend(arc([xl - r, d - tf - r], r, FRAC_PI_2, n, false));
    p.extend(arc([xl - r, tf + r], r, 0.0, n, false));
    p.push([0.0, tf]);
    (p, Vec::new())
}

/// 🏗️ Mono-symmetric I section with top flange `[width, thickness]` and bottom flange `[width, thickness]`.
pub fn mono_i_section(d: f64, top: [f64; 2], bottom: [f64; 2], tw: f64, r: f64, n: usize) -> Outline {
    let ([bt, tft], [bb, tfb]) = (top, bottom);
    let c = bt.max(bb) / 2.0;
    let (xl, xr) = (c - tw / 2.0, c + tw / 2.0);
    let mut p = vec![[c - bb / 2.0, 0.0], [c + bb / 2.0, 0.0], [c + bb / 2.0, tfb]];
    p.extend(arc([xr + r, tfb + r], r, 1.5 * PI, n, false));
    p.extend(arc([xr + r, d - tft - r], r, PI, n, false));
    p.extend([[c + bt / 2.0, d - tft], [c + bt / 2.0, d], [c - bt / 2.0, d], [c - bt / 2.0, d - tft]]);
    p.extend(arc([xl - r, d - tft - r], r, FRAC_PI_2, n, false));
    p.extend(arc([xl - r, tfb + r], r, 0.0, n, false));
    p.push([c - bb / 2.0, tfb]);
    (p, Vec::new())
}

/// 🧲️ Parallel-flange channel with the web on the left, depth `d`, flange width `b`.
pub fn channel_section(d: f64, b: f64, tf: f64, tw: f64, r: f64, n: usize) -> Outline {
    let mut p = vec![[0.0, 0.0], [b, 0.0], [b, tf]];
    p.extend(arc([tw + r, tf + r], r, 1.5 * PI, n, false));
    p.extend(arc([tw + r, d - tf - r], r, PI, n, false));
    p.extend([[b, d - tf], [b, d], [0.0, d]]);
    (p, Vec::new())
}

/// 🔱️ Tee section with the flange `b` x `tf` on top and the stem `tw` down to the origin, root radius `r`.
pub fn tee_section(d: f64, b: f64, tf: f64, tw: f64, r: f64, n: usize) -> Outline {
    let (xl, xr) = (b / 2.0 - tw / 2.0, b / 2.0 + tw / 2.0);
    let mut p = vec![[xl, 0.0], [xr, 0.0]];
    p.extend(arc([xr + r, d - tf - r], r, PI, n, false));
    p.extend([[b, d - tf], [b, d], [0.0, d], [0.0, d - tf]]);
    p.extend(arc([xl - r, d - tf - r], r, FRAC_PI_2, n, false));
    (p, Vec::new())
}

/// 📐️ Angle with legs `b` (horizontal) and `d` (vertical), thickness `t`, root radius `rr` and toe radius `rt` (at most `t`).
pub fn angle_section(d: f64, b: f64, t: f64, rr: f64, rt: f64, n: usize) -> Outline {
    let full = (t - rt).abs() < 1e-12 * t.abs().max(1e-300);
    let mut p = vec![[0.0, 0.0]];
    if !full {
        p.push([b, 0.0]);
    }
    p.extend(arc([b - rt, t - rt], rt, 0.0, n, true));
    p.extend(arc([t + rr, t + rr], rr, 1.5 * PI, n, false));
    p.extend(arc([t - rt, d - rt], rt, 0.0, n, true));
    if !full {
        p.push([0.0, d]);
    }
    (p, Vec::new())
}

/// 🔲️ Rectangular hollow section, depth `d`, width `b`, wall `t`, outer corner radius `r_out` and inner radius `max(r_out - t, 0)`.
pub fn rectangular_hollow_section(d: f64, b: f64, t: f64, r_out: f64, n: usize) -> Outline {
    let r_in = (r_out - t).max(0.0);
    let mut outer = arc([r_out, r_out], r_out, PI, n, true);
    outer.extend(arc([b - r_out, r_out], r_out, 1.5 * PI, n, true));
    outer.extend(arc([b - r_out, d - r_out], r_out, 0.0, n, true));
    outer.extend(arc([r_out, d - r_out], r_out, FRAC_PI_2, n, true));
    let mut inner = arc([t + r_in, t + r_in], r_in, PI, n, true);
    inner.extend(arc([b - t - r_in, t + r_in], r_in, 1.5 * PI, n, true));
    inner.extend(arc([b - t - r_in, d - t - r_in], r_in, 0.0, n, true));
    inner.extend(arc([t + r_in, d - t - r_in], r_in, FRAC_PI_2, n, true));
    (outer, vec![inner])
}

/// ⭕️ Elliptical hollow section with outer axes `d_x` x `d_y` (a circle when equal), wall `t`, `n` points around.
pub fn elliptical_hollow_section(d_x: f64, d_y: f64, t: f64, n: usize) -> Outline {
    let ring = |ax: f64, ay: f64| -> Vec<[f64; 2]> { (0..n).map(|i| { let a = 2.0 * PI * i as f64 / n as f64; [ax * a.cos(), ay * a.sin()] }).collect() };
    (ring(d_x / 2.0, d_y / 2.0), vec![ring(d_x / 2.0 - t, d_y / 2.0 - t)])
}

/// ⭕️ Circular hollow section with outer diameter `d`, wall `t`, `n` points around.
pub fn circular_hollow_section(d: f64, t: f64, n: usize) -> Outline {
    elliptical_hollow_section(d, d, t, n)
}

/// ▭️ Solid rectangle `b` x `d`.
pub fn rectangle(b: f64, d: f64) -> Outline {
    (vec![[0.0, 0.0], [b, 0.0], [b, d], [0.0, d]], Vec::new())
}
