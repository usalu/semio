//! 🔬️ Plated panel details in 3D: the transverse stiffener outline as an extruded section at both panel edges, the lateral pressure as an area load with arrows, and the plate deflection surface under it.

use super::kit::*;
use super::layout::Layout;
use super::mechanics as mech;
use super::model::PlatedPanel;
use super::scene::*;
use super::{handles, SceneInput};
use std::f64::consts::PI;

const YOUNG: f64 = 210.0e9;
const POISSON: f64 = 0.3;
const GRID: usize = 12;
const TERMS: usize = 7;
const SHOWN_DEFLECTION: f64 = 0.05;
const SHEAR_TERMS: usize = 8;

/// 🔬️ The centreline-width outline of the transverse stiffener in panel coordinates `(u along a, v up from the plate surface)`, counter-clockwise; `None` without a valid shape.
pub fn outline(p: &PlatedPanel) -> Option<Vec<Vec2>> {
    let (h, t, bf, tf) = (p.stiffener_depth, p.stiffener_thickness, p.stiffener_flange_width, p.stiffener_flange_thickness);
    if h <= 1e-6 || t <= 1e-6 {
        return None;
    }
    let q = |u: f64, v: f64| Vec2::new(u, v);
    let web = vec![q(-t / 2.0, 0.0), q(t / 2.0, 0.0), q(t / 2.0, h), q(-t / 2.0, h)];
    match p.stiffener_shape {
        semio_s_artifact_norm_en1993_model::StiffenerShape::Flat => Some(web),
        semio_s_artifact_norm_en1993_model::StiffenerShape::Tee if bf > 1e-6 && tf > 1e-6 && tf < h => Some(vec![q(-t / 2.0, 0.0), q(t / 2.0, 0.0), q(t / 2.0, h - tf / 2.0), q(bf / 2.0, h - tf / 2.0), q(bf / 2.0, h + tf / 2.0), q(-bf / 2.0, h + tf / 2.0), q(-bf / 2.0, h - tf / 2.0), q(-t / 2.0, h - tf / 2.0)]),
        semio_s_artifact_norm_en1993_model::StiffenerShape::Angle if bf > 1e-6 && tf > 1e-6 && tf < h => Some(vec![q(-t / 2.0, 0.0), q(t / 2.0, 0.0), q(t / 2.0, h - tf / 2.0), q(bf, h - tf / 2.0), q(bf, h + tf / 2.0), q(-t / 2.0, h + tf / 2.0)]),
        _ => None,
    }
}

fn stiffeners(input: &SceneInput, layout: &Layout, panel: &PlatedPanel, out: &mut Out) {
    let Some(points) = outline(panel) else { return };
    let d = input.document;
    let o = layout.origin(&panel.id);
    let base = o.z + panel.thickness / 2.0;
    let polygon = Polygon2::solid(points.iter().rev().map(|q| Vec2::new(-q.x, q.y)).collect());
    for (k, x) in [o.x, o.x + panel.a].into_iter().enumerate() {
        let (from, to) = (v3(x, o.y, base), v3(x, o.y + panel.b, base));
        let (shape_en, shape_de) = match panel.stiffener_shape {
            semio_s_artifact_norm_en1993_model::StiffenerShape::Tee => ("tee", "T-Profil"),
            semio_s_artifact_norm_en1993_model::StiffenerShape::Angle => ("angle", "Winkel"),
            semio_s_artifact_norm_en1993_model::StiffenerShape::Flat | semio_s_artifact_norm_en1993_model::StiffenerShape::None => ("flat bar", "Flachstahl"),
        };
        let size = format!("h = {:.0} mm, t = {:.0} mm", panel.stiffener_depth * 1e3, panel.stiffener_thickness * 1e3);
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: format!("{}:stiffener-{}", panel.id, k + 1),
                collection: "platedPanels",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[from.clone(), to.clone()]),
                    frame: member_frame(&from, &Vec3::Y),
                    section: SectionShape::Polygon { polygon: polygon.clone() },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Steel,
                binding: binding(input.report, &[panel.id.as_str()]),
                name: loc(&format!("Transverse stiffener {} of {}", k + 1, panel.id), &format!("Quersteife {} von {}", k + 1, panel.id)),
                detail: loc(&format!("{shape_en}, {size}"), &format!("{shape_de}, {size}")),
                layer: LAYER_PLATES,
                handles: handles::ids(d, layout, &panel.id),
                deformation: None,
            },
            &[from, to],
        );
    }
}

fn pressure(input: &SceneInput, layout: &Layout, panel: &PlatedPanel, out: &mut Out) {
    if panel.lateral_pressure <= 0.0 {
        return;
    }
    let o = layout.origin(&panel.id);
    let z = o.z + panel.thickness / 2.0;
    let ring = vec![v3(o.x, o.y, z), v3(o.x + panel.a, o.y, z), v3(o.x + panel.a, o.y + panel.b, z), v3(o.x, o.y + panel.b, z)];
    let text = format!("p = {:.1} kPa", panel.lateral_pressure / 1e3);
    let centre = v3(o.x + panel.a / 2.0, o.y + panel.b / 2.0, z);
    out.push(
        Spec {
            kind: EntityKind::Load,
            id: format!("{}:pressure-load", panel.id),
            collection: "platedPanels",
            geometry: Geometry::Load(LoadGlyph::Area { polygon: Polygon3 { outer: Ring3 { points: ring.clone() }, holes: Vec::new() }, direction: mul(&Vec3::Z, -1.0), intensity: Quantity::new(QuantityKind::Pressure, panel.lateral_pressure), case: LoadCaseRef { id: "lateral-pressure".into(), kind: LoadKind::Variable, psi: None } }),
            role: MaterialRole::Load { kind: LoadKind::Variable },
            binding: binding(input.report, &[panel.id.as_str()]),
            name: loc(&format!("Lateral pressure on {}", panel.id), &format!("Querdruck auf {}", panel.id)),
            detail: loc(&text, &text),
            layer: LAYER_LOADS,
            handles: handles::ids(input.document, layout, &panel.id),
            deformation: None,
        },
        &ring,
    );
    out.annotate(Annotation::Label { id: format!("{}:pressure-label", panel.id), anchor: centre.clone(), text: loc(&text, &text), tone: tone_of(&binding(input.report, &[panel.id.as_str()])) }, &centre);
}

/// 🌊️ Deflection of the simply supported panel under its uniform lateral pressure (Navier double series, odd terms), sampled on a regular grid in plane coordinates about the panel centre; `scale` magnifies the largest deflection to a twentieth of the shorter edge. `None` without pressure.
pub fn deflection(panel: &PlatedPanel) -> Option<DeformationOverlay> {
    if panel.lateral_pressure <= 0.0 || panel.a <= 1e-6 || panel.b <= 1e-6 || panel.thickness <= 1e-6 {
        return None;
    }
    let stiffness = YOUNG * panel.thickness.powi(3) / (12.0 * (1.0 - POISSON * POISSON));
    let w = |x: f64, y: f64| -> f64 {
        let mut sum = 0.0;
        for m in (1..=TERMS).step_by(2) {
            for n in (1..=TERMS).step_by(2) {
                let (mf, nf) = (m as f64, n as f64);
                let k = (mf / panel.a).powi(2) + (nf / panel.b).powi(2);
                sum += (mf * PI * x / panel.a).sin() * (nf * PI * y / panel.b).sin() / (mf * nf * k * k);
            }
        }
        16.0 * panel.lateral_pressure / (PI.powi(6) * stiffness) * sum
    };
    let surface: Vec<SurfaceSample> = (0..=GRID)
        .flat_map(|i| (0..=GRID).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (x, y) = (panel.a * i as f64 / GRID as f64, panel.b * j as f64 / GRID as f64);
            SurfaceSample { at: Vec2::new(x - panel.a / 2.0, y - panel.b / 2.0), delta: Vec3::new(0.0, 0.0, -w(x, y)) }
        })
        .collect();
    let top = surface.iter().fold(0.0_f64, |m, s| m.max(s.delta.z.abs()));
    (top > 0.0).then(|| DeformationOverlay { kind: DeformationKind::Deformed, index: 1, frequency: None, participation: 1.0, scale: (SHOWN_DEFLECTION * panel.a.min(panel.b) / top).max(1.0), samples: Vec::new(), surface, motion: Motion::Static, load_factor: None })
}

/// 🌀️ First elastic shear buckling mode of the simply supported panel `a × b` by the Rayleigh-Ritz method on the sine basis (Timoshenko): `(k_τ, coefficients A_mn)` with `k_τ = τ_cr t b²/(π² D)` for `w = Σ A_mn sin(mπx/a) sin(nπy/b)`, the coefficient vector indexed `(m − 1) · SHEAR_TERMS + (n − 1)`. The stiffness is diagonal, the shear work couples modes of opposite parity in both directions, so the lowest positive eigenvalue of `K A = λ B A` is the buckling shear.
pub fn shear_ritz(a: f64, b: f64) -> Option<(f64, Vec<f64>)> {
    if a <= 1e-9 || b <= 1e-9 {
        return None;
    }
    let n = SHEAR_TERMS;
    let size = n * n;
    let (mut k, mut g) = (mech::linalg::Dense::zeros(size), mech::linalg::Dense::zeros(size));
    for (m, nn, p, q) in (1..=n).flat_map(|m| (1..=n).flat_map(move |nn| (1..=n).flat_map(move |p| (1..=n).map(move |q| (m, nn, p, q))))) {
        let (i, j) = ((m - 1) * n + nn - 1, (p - 1) * n + q - 1);
        if i == j {
            k.add(i, i, PI.powi(4) * a * b / 4.0 * ((m as f64 / a).powi(2) + (nn as f64 / b).powi(2)).powi(2));
        }
        if (m + p) % 2 == 1 && (nn + q) % 2 == 1 {
            let (mf, nf, pf, qf) = (m as f64, nn as f64, p as f64, q as f64);
            let value = 4.0 * mf * qf * pf * nf / ((pf * pf - mf * mf) * (nf * nf - qf * qf));
            g.add(i, j, value);
            g.add(j, i, value);
        }
    }
    let (lambda, coefficients) = mech::linalg::generalized_dense(&k, &g, 1, &mech::Control::none()).ok()?.into_iter().next()?;
    Some((lambda * b * b / (PI * PI), coefficients))
}

/// 🌀️ The shear buckling mode as a surface ghost: the Ritz shape normalised to a largest deflection of 1 about the panel centre, shown magnified to a twentieth of the shorter edge; `load_factor` is `α_cr = τ_cr / τ_Ed` of the design shear force (none without shear). `None` without a valid panel.
pub fn shear_mode(panel: &PlatedPanel) -> Option<DeformationOverlay> {
    if panel.thickness <= 1e-6 {
        return None;
    }
    let (k, coefficients) = shear_ritz(panel.a, panel.b)?;
    let w = |x: f64, y: f64| -> f64 { (1..=SHEAR_TERMS).flat_map(|m| (1..=SHEAR_TERMS).map(move |n| (m, n))).map(|(m, n)| coefficients[(m - 1) * SHEAR_TERMS + n - 1] * (m as f64 * PI * x / panel.a).sin() * (n as f64 * PI * y / panel.b).sin()).sum() };
    let grid: Vec<(f64, f64, f64)> = (0..=GRID).flat_map(|i| (0..=GRID).map(move |j| (i, j))).map(|(i, j)| (panel.a * i as f64 / GRID as f64, panel.b * j as f64 / GRID as f64, 0.0)).map(|(x, y, _)| (x, y, w(x, y))).collect();
    let top = grid.iter().fold(0.0_f64, |m, (_, _, v)| m.max(v.abs()));
    if top <= 0.0 {
        return None;
    }
    let sign = if grid.iter().max_by(|l, r| l.2.abs().total_cmp(&r.2.abs())).is_some_and(|c| c.2 < 0.0) { -1.0 } else { 1.0 };
    let surface = grid.iter().map(|(x, y, v)| SurfaceSample { at: Vec2::new(x - panel.a / 2.0, y - panel.b / 2.0), delta: Vec3::new(0.0, 0.0, -sign * v / top) }).collect();
    let d_plate = YOUNG * panel.thickness.powi(3) / (12.0 * (1.0 - POISSON * POISSON));
    let tau_cr = k * PI * PI * d_plate / (panel.thickness * panel.b * panel.b);
    let tau_ed = panel.shear_force.abs() / (panel.b * panel.thickness);
    Some(DeformationOverlay { kind: DeformationKind::Mode, index: 1, frequency: None, participation: 1.0, scale: SHOWN_DEFLECTION * panel.a.min(panel.b), samples: Vec::new(), surface, motion: Motion::Harmonic, load_factor: (tau_ed > 0.0).then_some(tau_cr / tau_ed) })
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for panel in input.document.plated_panels.iter().filter(|p| p.a > 1e-6 && p.b > 1e-6 && p.thickness > 1e-6) {
        if !out.own("platedPanels", &panel.id) {
            continue;
        }
        stiffeners(input, layout, panel, out);
        pressure(input, layout, panel, out);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
