//! 🧷️ Interface slip arrows of the EN 1994 scene: one arrow per sampled stud station on the steel-slab interface, pointing the way the slab slips against the steel, its length and colour following the slip `δ(x)` against the capacity δ_uk = 6 mm (EN 1994-1-1, 6.6.1.1, 6.6.2). The values are those of the `slip-distribution` diagram.

use super::beams::local_frame;
use super::diagrams::{slip_at, slip_inputs, SLIP_CAPACITY_MM};
use super::facts::BeamFacts;
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::SceneInput;

const STUD_PREFIXES: [&str; 3] = ["en1994.6.6.", "en1994.2.6.8.3.stud.", "en1994.6.2.1.3."];
pub(crate) const MAX_ARROWS: usize = 12;
const SHAFT: f64 = 0.01;
const MIN_SLIP_MM: f64 = 1e-3;

/// 📏️ The arrow length in m: the slip over the capacity, times 0.8 stud pitches, never below a visible stub.
pub(crate) fn arrow_length(slip_mm: f64, pitch: f64) -> f64 {
    (slip_mm.abs() / SLIP_CAPACITY_MM * 0.8 * pitch).max(0.1 * pitch)
}

/// 🧷️ The sampled arrows of one beam as `(x, signed slip in mm)`; stations without slip are left out.
pub(crate) fn arrows(f: &BeamFacts) -> Vec<(f64, f64)> {
    let (peak, p_rk) = slip_inputs(f);
    let stations = f.station_xs();
    let count = stations.len().min(MAX_ARROWS);
    (0..count).map(|k| stations[k * stations.len() / count]).map(|x| (x, slip_at(f, peak, p_rk, x))).filter(|(_, slip)| slip.abs() > MIN_SLIP_MM).collect()
}

/// 🧷️ Adds the slip arrows of every composite beam.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for beam in &input.document.beams {
        if !out.wants(&beam.id) {
            continue;
        }
        let Some(pose) = layout.pose(&beam.id) else { continue };
        let f = BeamFacts::of(input.document, beam);
        if f.l_total <= 1e-6 || f.steel.height_m <= 1e-6 {
            continue;
        }
        let gov = binding(input.report, &beam.id, &STUD_PREFIXES);
        let pitch = f.station_pitch();
        for (k, (x, slip)) in arrows(&f).into_iter().enumerate() {
            let length = arrow_length(slip, pitch);
            let frame = local_frame(pose, (x, 0.0, 0.0), (1.0_f64.copysign(slip), 0.0, 0.0), (0.0, 1.0, 0.0));
            let end = add(&frame.origin, &mul(&frame.x, length));
            let geometry = Geometry::Member(MemberGeometry {
                axis: polyline(&[frame.origin.clone(), end.clone()]),
                frame: frame.clone(),
                section: steel_shape_rect(),
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: EndRelease::Rigid,
                end_release: EndRelease::Rigid,
                stations: Vec::new(),
            });
            let ratio = slip.abs() / SLIP_CAPACITY_MM;
            let share = gov.as_ref().map_or(0.0, |g| ratio / g.ratio.max(1e-9));
            let name = loc(&format!("Slip arrow {} of {}: δ = {slip:+.2} mm", k + 1, beam.id), &format!("Schlupfpfeil {} von {}: δ = {slip:+.2} mm", k + 1, beam.id));
            let detail = loc(
                &format!("Interface slip {:.2} mm at x = {x:.2} m, {:.0} % of the slip capacity δ_uk = 6 mm (EN 1994-1-1, 6.6.1.1)", slip.abs(), ratio * 100.0),
                &format!("Fugenschlupf {:.2} mm bei x = {x:.2} m, {:.0} % der Schlupfkapazität δ_uk = 6 mm (EN 1994-1-1, 6.6.1.1)", slip.abs(), ratio * 100.0),
            );
            let spec = Spec::new(EntityKind::Member, format!("{}:slip-{}", beam.id, k + 1), "beams", geometry, MaterialRole::Steel, name, LAYER_SLIP).bound(scaled(&gov, share)).detailed(detail);
            out.push(spec, &[frame.origin, end]);
        }
    }
}

fn steel_shape_rect() -> SectionShape {
    SectionShape::parametric(ProfileKind::Rect, &[("h", SHAFT), ("b", SHAFT)])
}
