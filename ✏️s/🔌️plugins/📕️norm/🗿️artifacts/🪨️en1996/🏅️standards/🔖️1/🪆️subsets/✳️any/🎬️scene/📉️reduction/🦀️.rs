//! 📉️ Reduction: the reduction factor Φ of EN 1996-1-1 (6.1.2.2, Annex G) as a 3D distribution over the wall height: a ribbon on the front face whose depth is Φ·t, interpolated linearly between the three verified sections (top, mid-height, bottom), bound to the governing compression check.

use super::kit::*;
use super::layout::{Layout, EPS};
use super::scene::*;
use super::{profile, SceneInput};

const SAMPLES: usize = 12;
const STRIP_WIDTH: f64 = 0.5;
const OPACITY: f64 = 0.55;

/// 📐️ A quantity given at `[top, mid, bottom]` interpolated linearly at the relative height `u` (0 bottom, 1 top); the code verifies only these three sections.
pub fn along_height(v: [f64; 3], u: f64) -> f64 {
    let u = u.clamp(0.0, 1.0);
    if u >= 0.5 { v[1] + (v[0] - v[1]) * (u - 0.5) * 2.0 } else { v[2] + (v[1] - v[2]) * u * 2.0 }
}

/// 📐️ Φ at the relative height `u` (0 bottom, 1 top) from the verified sections `[top, mid, bottom]`.
pub fn phi_at(phi: [f64; 3], u: f64) -> f64 {
    along_height(phi, u)
}

/// 📐️ Normalised vertical stress distribution across a rectangular section of thickness `t` under a resultant at the eccentricity `e` from the axis, at the distance `s` from the axis towards the load side: linear `1 + 12 e s / t²` inside the middle third, triangular over the contact length `3 (t/2 − e)` beyond it; the mean over the thickness is 1 in both branches.
pub fn stress_shape(t: f64, e: f64, s: f64) -> f64 {
    let e = e.abs().min(0.5 * t);
    if e <= t / 6.0 {
        return 1.0 + 12.0 * e * s / (t * t);
    }
    let contact = (3.0 * (0.5 * t - e)).max(0.01 * t);
    2.0 * t / contact * ((s - (0.5 * t - contact)) / contact).clamp(0.0, 1.0)
}

/// 🧱️ The vertical stress σ over the design strength f_d at the relative height `u` and the distance `s` from the wall axis towards the load side: the mean stress N/A of the interpolated section force times the [`stress_shape`] of the interpolated eccentricity (top, resultant e_mk at mid-height, bottom).
pub fn stress_ratio(prof: &profile::Profile, t: f64, u: f64, s: f64) -> f64 {
    let n = along_height(prof.n, u);
    let e = along_height([prof.e_top + prof.e_init, prof.e_mk, prof.e_bottom + prof.e_init], u);
    n / prof.area.max(1e-9) * stress_shape(t, e, s) / prof.f_d.max(1.0)
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
        if l <= EPS || h <= EPS || t <= EPS {
            continue;
        }
        let Some(prof) = profile::governing(input.document, wall) else { continue };
        let width = STRIP_WIDTH.min(l);
        let x = (0.5 * (l - width), 0.5 * (l + width));
        let depth = |i: usize| phi_at(prof.phi, i as f64 / SAMPLES as f64) * t;
        let mut mesh = MeshBuilder::default();
        for i in 0..SAMPLES {
            mesh.frustum(pose, x, (-depth(i), 0.0), (-depth(i + 1), 0.0), (h * i as f64 / SAMPLES as f64, h * (i + 1) as f64 / SAMPLES as f64));
        }
        if mesh.is_empty() {
            continue;
        }
        let points = mesh.points().to_vec();
        let id = format!("{}:{PHI_TAG}", wall.id);
        let mut spec = Spec::new(EntityKind::Solid, id, mesh.solid(), MaterialRole::Neutral, LAYER_PATTERNS, loc("Reduction factor Φ over the height", "Abminderungsfaktor Φ über die Höhe"));
        spec.detail = loc(
            &format!("Φ_top = {:.3}, Φ_m = {:.3}, Φ_bottom = {:.3}; the depth of the ribbon is Φ·t, linear between the verified sections (EN 1996-1-1 6.1.2.2, Annex G)", prof.phi[0], prof.phi[1], prof.phi[2]),
            &format!("Φ_oben = {:.3}, Φ_m = {:.3}, Φ_unten = {:.3}; die Tiefe des Bandes ist Φ·t, linear zwischen den nachgewiesenen Schnitten (EN 1996-1-1 6.1.2.2, Anhang G)", prof.phi[0], prof.phi[1], prof.phi[2]),
        );
        spec.binding = binding_of(input.report, &super::walls::compression_id(&wall.id, &prof.load_case, prof.governing_station()));
        spec.opacity = OPACITY;
        spec.channel = Some(ColourChannel::Utilisation);
        spec.handles = super::handles::ids(input.document, layout, &spec.id);
        out.push(spec, &points);
        let xc = 0.5 * l;
        for (tag, u, value) in [("top", 1.0, prof.phi[0]), ("mid", 0.5, prof.phi[1]), ("bottom", 0.0, prof.phi[2])] {
            let from = pose.at(xc, 0.0, u * h);
            out.annotate(Annotation::Dimension { id: format!("{}:phi-{tag}", wall.id), from: from.clone(), to: pose.at(xc, -value * t, u * h), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Dimensionless, value), binding: None }, &from);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{phi_at, stress_shape};

    #[test]
    fn phi_is_exact_at_the_verified_sections_and_linear_between() {
        let phi = [0.8, 0.5, 0.7];
        assert_eq!((phi_at(phi, 1.0), phi_at(phi, 0.5), phi_at(phi, 0.0)), (0.8, 0.5, 0.7));
        assert!((phi_at(phi, 0.75) - 0.65).abs() < 1e-12 && (phi_at(phi, 0.25) - 0.6).abs() < 1e-12);
    }

    #[test]
    fn the_stress_shape_carries_the_whole_force_for_every_eccentricity() {
        let t = 0.365;
        for e in [0.0, 0.02, t / 6.0, 0.09, 0.14, 0.18] {
            let n = 4000;
            let mean: f64 = (0..n).map(|i| stress_shape(t, e, -0.5 * t + t * (i as f64 + 0.5) / n as f64)).sum::<f64>() / n as f64;
            assert!((mean - 1.0).abs() < 2e-3, "e = {e}: mean {mean}");
        }
    }

    #[test]
    fn the_stress_is_uniform_without_eccentricity_and_peaks_on_the_load_side() {
        let t = 0.3;
        assert!((stress_shape(t, 0.0, -0.1) - 1.0).abs() < 1e-12 && (stress_shape(t, 0.0, 0.1) - 1.0).abs() < 1e-12);
        assert!((stress_shape(t, t / 6.0, 0.5 * t) - 2.0).abs() < 1e-12 && stress_shape(t, t / 6.0, -0.5 * t).abs() < 1e-12);
        assert!(stress_shape(t, 0.12, 0.5 * t) > 2.0 && stress_shape(t, 0.12, -0.5 * t) == 0.0);
    }
}
