//! 🌀️ Deflection: the indicative deflected shape of the primary structure under the characteristic wind and snow load of the site, computed by the shared mechanics kernel ([`mechanics`]).
//! The snapshot holds actions only, so the stiffness is a stated assumption: the wind column drifts `H / 500` and the snow beam sags `L / 250` under the characteristic load; the shape and the ramp (the load applied from 0 to 1) are exact for that stiffness.

use super::kit::*;
use super::mechanics::member_modes::{deflection, Curve, Ends, Loads, Props};
use super::model::En1991Snapshot;
use super::scene::projection_kit::member_frame;
use super::scene::*;
use super::snow::sk_of;
use super::wind::{cpe_candidates, qp_at};
use super::{checks, Ctx};
use checks::part_1_3;

pub const WIND_FRAME: &str = "building:wind-frame";
pub const SNOW_BEAM: &str = "building:snow-beam";
pub const WIND_DRIFT_RATIO: f64 = 500.0;
pub const SNOW_SAG_RATIO: f64 = 250.0;
const SNOW_MU_1: f64 = 0.8;
const E_MODULUS: f64 = 210.0e9;
const G_MODULUS: f64 = 81.0e9;
const DISPLAY_FRACTION: f64 = 0.05;
const MAX_SCALE: f64 = 500.0;

/// 🌬️ Net characteristic wind line load in N/m on the windward and leeward walls of the building: `q_p(H)·(c_pe,D − c_pe,E)·B`, `None` when the site or a zone is inadmissible.
pub fn wind_line_load(d: &En1991Snapshot) -> Option<f64> {
    let qp = qp_at(d, d.height)?;
    let first = |letter: char| cpe_candidates(d, letter).and_then(|c| c.first().map(|p| p.0));
    Some(qp * (first('D')? - first('E')?) * d.width)
}

/// ❄️ Characteristic snow load in N/m per metre of tributary width on the first roof (shape coefficient μ₁ = 0.8 for pitches up to 30°), `None` without a roof or a valid snow zone.
pub fn snow_line_load(d: &En1991Snapshot) -> Option<f64> {
    let roof = d.roofs.first()?;
    Some(part_1_3::roof_snow_pa(sk_of(d)?, SNOW_MU_1, roof.c_e, roof.c_t))
}

/// 📐️ The stiffness `EI` in N·m² that makes the cantilever of length `l` under the uniform load `w` drift `l / 500` at its tip: `w·l⁴ / (8·δ)`.
pub fn cantilever_stiffness(w: f64, l: f64) -> f64 {
    w * l.powi(4) / (8.0 * l / WIND_DRIFT_RATIO)
}

/// 📐️ The stiffness `EI` in N·m² that makes the simply supported span `l` under the uniform load `w` sag `l / 250` at midspan: `5·w·l⁴ / (384·δ)`.
pub fn beam_stiffness(w: f64, l: f64) -> f64 {
    5.0 * w * l.powi(4) / (384.0 * l / SNOW_SAG_RATIO)
}

fn props(length: f64, ei: f64) -> Props {
    Props { length, e: E_MODULUS, g: G_MODULUS, a: 1.0, iy: ei / E_MODULUS, iz: ei / E_MODULUS, j: ei / E_MODULUS }
}

/// 🧮️ The local deflection line of the kernel for the uniform load `w` (N/m, directed along local `-z`), `None` when the kernel rejects the model.
pub fn line(length: f64, ei: f64, ends: Ends, w: f64) -> Option<Curve> {
    deflection(&props(length, ei), ends, &Loads { udl: w, ..Default::default() }).ok()
}

fn overlay(curve: &Curve, direction: &Vec3, length: f64) -> Option<DeformationOverlay> {
    let samples: Vec<DisplacementSample> = curve.iter().map(|(at, u)| DisplacementSample { at: *at, delta: mul(direction, u[2]) }).collect();
    let peak = samples.iter().map(|s| s.delta.length()).fold(0.0_f64, f64::max);
    (peak > 0.0).then(|| DeformationOverlay { surface: Vec::new(), motion: Motion::Ramp, load_factor: Some(1.0), kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale: (DISPLAY_FRACTION * length / peak).clamp(1.0, MAX_SCALE), samples })
}

fn member(id: &str, start: Vec3, end: Vec3, frame: Frame, release: EndRelease, name: LocalizedCopy, detail: LocalizedCopy, binding: Option<UtilisationBinding>, overlay: Option<DeformationOverlay>, out: &mut Out) {
    out.push(
        Spec::new(
            EntityKind::Member,
            id,
            "@site",
            Geometry::Member(MemberGeometry {
                axis: polyline(&[start.clone(), end.clone()]),
                frame,
                section: SectionShape::parametric(ProfileKind::IShape, &[("h", 0.4), ("b", 0.2), ("tw", 0.01), ("tf", 0.016)]),
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: release,
                end_release: release,
                stations: Vec::new(),
            }),
            MaterialRole::Neutral,
            name,
            LAYER_ENVELOPE,
        )
        .bound(binding)
        .detail(detail)
        .deformed(overlay),
        &[start, end],
    );
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    if !out.own(global::DEFLECTION) {
        return;
    }
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    let pose = &layout.building;
    if d.height > 0.0 {
        if let Some(w) = wind_line_load(d).filter(|w| *w > 0.0) {
            let (start, end) = (pose.at(0.0, 0.0, 0.0), pose.at(0.0, 0.0, d.height));
            let frame = member_frame(&start, &Vec3::Z);
            let curve = line(d.height, cantilever_stiffness(w, d.height), Ends::Cantilever, w).map(|c| c.into_iter().map(|(at, u)| (at, [u[0], u[1], -u[2]])).collect::<Curve>());
            let overlay = curve.as_ref().and_then(|c| overlay(c, &pose.dir(&Vec3::X), d.height));
            let drift = d.height / WIND_DRIFT_RATIO * 1000.0;
            member(WIND_FRAME, start, end, frame, EndRelease::Rigid, loc("Wind-loaded column (indicative)", "Windbelastete Stütze (indikativ)"), bi(|de| format!("w = {} kN/m, {} for H/{WIND_DRIFT_RATIO:.0} = {} mm", num(w / 1000.0, 2, de), if de { "angenommene Steifigkeit" } else { "assumed stiffness" }, num(drift, 1, de))), binding_prefixed(report, &["en1991.1-4."]), overlay, out);
        }
        if let Some(w) = snow_line_load(d).filter(|w| *w > 0.0).filter(|_| d.width > 0.0) {
            let (start, end) = (pose.at(0.0, 0.0, d.height), pose.at(0.0, d.width, d.height));
            let frame = member_frame(&start, &pose.dir(&Vec3::Y));
            let curve = line(d.width, beam_stiffness(w, d.width), Ends::Pinned, w);
            let overlay = curve.as_ref().and_then(|c| overlay(c, &Vec3::Z, d.width));
            let sag = d.width / SNOW_SAG_RATIO * 1000.0;
            member(SNOW_BEAM, start, end, frame, EndRelease::Pinned, loc("Snow-loaded roof beam (indicative)", "Schneebelasteter Dachträger (indikativ)"), bi(|de| format!("s = {} kN/m, {} for L/{SNOW_SAG_RATIO:.0} = {} mm", num(w / 1000.0, 2, de), if de { "angenommene Steifigkeit" } else { "assumed stiffness" }, num(sag, 1, de))), binding_prefixed(report, &["en1991.1-3."]), overlay, out);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
