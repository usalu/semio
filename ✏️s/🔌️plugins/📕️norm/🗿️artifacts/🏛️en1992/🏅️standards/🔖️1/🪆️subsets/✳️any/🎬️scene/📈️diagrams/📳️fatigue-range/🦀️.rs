//! 📳️ Fatigue stress range of the longitudinal bars: the linear-elastic state II steel stress of every bar under the characteristic and the quasi-permanent serviceability moment (a stand-in for the load cycle of EN 1992-1-1 6.8.3, the document holds no fatigue load model), set against the design resistance of straight bars at N* cycles from the registry; drawn as a colour per bar and as a diagram over the bar index.

use super::checks::{self, DesignEffects};
use super::{axis, base, stroke, titled, xy};
use super::kit::*;
use super::layout::{Class, Layout};
use super::mechanics;
use super::model::{En1992Snapshot, MemberKind, RcMember};
use super::reinforcement::bar_layout;
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

pub const FIELD: &str = "fatigue-ratio";
const TUBE: f64 = 1.3;

/// 📊️ The stress range of one bar in MPa with its layer, index in the layer and position in the section (m).
#[derive(Clone, Debug, PartialEq)]
pub struct BarRange {
    pub layer: usize,
    pub index: usize,
    pub y: f64,
    pub z: f64,
    pub range_mpa: f64,
}

/// 📐️ The ranges of all bars of a member and the resistance `Δσ_Rsk / γ_s,fat` in MPa.
#[derive(Clone, Debug, PartialEq)]
pub struct Ranges {
    pub bars: Vec<BarRange>,
    pub limit_mpa: f64,
}

fn stresses(sec: &checks::Sec, e: &DesignEffects, bars: &[(f64, f64)], height: f64) -> Option<Vec<f64>> {
    let el = sec.elastic(e.m_ed * 1.0e3, -e.n_ed, false)?;
    let sign = if e.m_ed >= 0.0 { 1.0 } else { -1.0 };
    let z_na = sign * (height / 2.0 - el.x / 1.0e3);
    let lever: Vec<f64> = bars.iter().map(|&(_, z)| (sign * (z_na - z)).max(0.0)).collect();
    let reference = lever.iter().copied().fold(0.0, f64::max);
    (reference > 1.0e-9).then(|| lever.iter().map(|l| el.sigma_s.abs() * l / reference).collect())
}

/// 🧮️ Δσ_s per bar between the quasi-permanent and the characteristic combination; the bar stress follows the lever arm to the neutral axis of the elastic state, the steel stress of the section refers to the lowest bar. `None` for upright members, without bars, without both combinations or when a state cannot be solved.
pub fn bar_ranges(document: &En1992Snapshot, m: &RcMember, width: f64) -> Option<Ranges> {
    let sec = mechanics::member_sec(document, m)?;
    let effects = checks::combine_member_actions(m, &document.action_factors());
    let (high, low) = (checks::governing(&effects, "sls_char")?, checks::governing(&effects, "sls_qp")?);
    let layout = bar_layout(m, width);
    let positions: Vec<(f64, f64)> = layout.iter().flat_map(|l| l.points.iter().copied()).collect();
    let (hi, lo) = (stresses(&sec, high, &positions, m.height)?, stresses(&sec, low, &positions, m.height)?);
    let bars = layout
        .iter()
        .flat_map(|l| l.points.iter().enumerate().map(move |(index, &(y, z))| (l.layer, index, y, z)))
        .zip(hi.iter().zip(&lo))
        .map(|((layer, index, y, z), (h, l))| BarRange { layer, index, y, z, range_mpa: (h - l).max(0.0) })
        .collect();
    let limit_mpa = checks::ndp(document.annex, "ndp.en1992.fatigue-straight") / checks::ndp(document.annex, "ndp.en1992.gamma-s-fat");
    Some(Ranges { bars, limit_mpa })
}

fn tube(c: &Frame, m: &RcMember, spec_id: &str, path: &str, bar: &BarRange, diameter: f64, ratio: f64) -> Spec {
    let (l, cv) = (m.span, m.cover);
    let (from_s, to_s) = if l - 2.0 * cv > 0.1 { (cv, l - cv) } else { (0.0, l) };
    let geometry = Geometry::Fastener(FastenerGeometry { kind: FastenerKind::Rebar { bend: None }, axis: Segment3 { from: at(c, from_s, bar.y, bar.z), to: at(c, to_s, bar.y, bar.z) }, diameter: diameter * TUBE, head: HeadShape::None, pattern: None, hole_clearance: None });
    let (en, de) = (format!("fatigue range {:.0} MPa of bar {} of layer {spec_id} ({:.0} % of the resistance)", bar.range_mpa, bar.index + 1, 100.0 * ratio), format!("Schwingbreite {:.0} MPa von Stab {} der Lage {spec_id} ({:.0} % der Beanspruchbarkeit)", bar.range_mpa, bar.index + 1, 100.0 * ratio));
    Spec::new(EntityKind::Fastener, format!("{}:fatigue-{spec_id}-{}", m.id, bar.index), path.to_string(), geometry, MaterialRole::Rebar, LAYER_FATIGUE, loc(&format!("{} fatigue range", m.label_en), &format!("{} Ermüdungsschwingbreite", m.label_de)), loc(&en, &de))
        .translucent(0.85)
        .channel(ColourChannel::Fields { active: FIELD.into(), fields: vec![ScalarField { id: FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Instance { values: vec![ratio] } }] })
}

/// 🎬️ Adds a tinted shell around every longitudinal bar of the beams and slabs whose bars show a stress range; the tint is `Δσ_s / (Δσ_Rsk / γ_s,fat)`.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for m in &input.document.members {
        if !out.wants(&m.id) || m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let Some(pose) = layout.member(&m.id).filter(|p| p.class != Class::Upright) else { continue };
        let width = if m.kind == MemberKind::FlatSlab { m.span } else { m.width };
        let Some(ranges) = bar_ranges(input.document, m, width).filter(|r| r.limit_mpa > 0.0 && r.bars.iter().any(|b| b.range_mpa > 0.0)) else { continue };
        field_legend(&mut out.legends, FIELD, loc("Fatigue stress range over the resistance", "Ermüdungsschwingbreite zur Beanspruchbarkeit"));
        for bar in &ranges.bars {
            let layer = &m.longitudinal[bar.layer];
            let path = format!("members[id={}].longitudinal[id={}]", m.id, layer.id);
            let (from_s, to_s) = if m.span - 2.0 * m.cover > 0.1 { (m.cover, m.span - m.cover) } else { (0.0, m.span) };
            let spec = tube(&pose.frame, m, &layer.id, &path, bar, layer.diameter, bar.range_mpa / ranges.limit_mpa);
            out.push(spec, &[at(&pose.frame, from_s, bar.y, bar.z), at(&pose.frame, to_s, bar.y, bar.z)]);
        }
    }
}

/// 📈️ Diagram `fatigue-range:<member>`: Δσ_s per bar against the resistance `Δσ_Rsk / γ_s,fat`.
pub fn diagram(document: &En1992Snapshot, m: &RcMember, width: f64, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    let ranges = bar_ranges(document, m, width).filter(|r| r.bars.iter().any(|b| b.range_mpa > 0.0))?;
    let points: Vec<[f64; 2]> = ranges.bars.iter().enumerate().map(|(i, b)| [(i + 1) as f64, b.range_mpa]).collect();
    let peak = ranges.bars.iter().map(|b| b.range_mpa).fold(0.0, f64::max);
    let at_peak = ranges.bars.iter().position(|b| b.range_mpa == peak).map_or(1.0, |i| (i + 1) as f64);
    let tone = if ranges.limit_mpa > 0.0 { super::scene::utilisation_tone(peak / ranges.limit_mpa) } else { Tone::Neutral };
    let mut g = base(
        m,
        "fatigue-range",
        titled(m, "Fatigue stress range of the bars", "Ermüdungsschwingbreite der Stäbe"),
        loc("Steel stress range between the quasi-permanent and the characteristic serviceability moment per longitudinal bar (state II, lever arm to the neutral axis) against Δσ_Rsk / γ_s,fat of straight bars from the registry. An indicative stand-in: the document holds no fatigue load model.", "Stahlspannungsschwingbreite zwischen dem quasi-ständigen und dem charakteristischen Gebrauchstauglichkeitsmoment je Längsstab (Zustand II, Hebelarm zur Nulllinie) gegen Δσ_Rsk / γ_s,fat gerader Stäbe aus dem Register. Ein Richtwert: das Dokument enthält kein Ermüdungslastmodell."),
        vec![axis("Bar", "Stab", QuantityKind::Dimensionless, "1"), axis("Stress range Δσs", "Spannungsschwingbreite Δσs", QuantityKind::Stress, "MPa")],
        kind,
        path,
    );
    g.series.push(Series::Limit { id: "resistance".into(), label: loc("Δσ_Rsk / γ_s,fat", "Δσ_Rsk / γ_s,fat"), axis: AxisRef::Y, value: ranges.limit_mpa, style: stroke(1.0, Dash::Dashed, Tone::Warning) });
    g.series.push(Series::Polyline { id: "range".into(), label: loc("Δσs per bar", "Δσs je Stab"), points: xy(&points), style: stroke(2.0, Dash::Solid, tone) });
    g.markers.push(Marker { id: "reference:peak".into(), at: Vec2::new(at_peak, peak), shape: MarkerShape::Triangle, tone, label: loc(&format!("Δσs,max = {peak:.0} MPa"), &format!("Δσs,max = {peak:.0} MPa")), utilisation: Some(peak / ranges.limit_mpa), bound_check: None, binding: None });
    g.table_twin = DataTable { columns: vec![column("Bar", "Stab", "1", points.iter().map(|p| p[0]).collect()), column("Stress range", "Schwingbreite", "MPa", points.iter().map(|p| p[1]).collect()), column("Height z", "Höhe z", "mm", ranges.bars.iter().map(|b| b.z * 1000.0).collect())] };
    Some(g)
}
