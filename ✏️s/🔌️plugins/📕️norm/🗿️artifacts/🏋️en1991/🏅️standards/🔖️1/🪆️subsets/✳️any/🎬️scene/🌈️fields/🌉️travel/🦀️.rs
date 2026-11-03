//! 🌉️ LM1 tandem travel: the tandem axle group of every notional lane (EN 1991-2, 4.3.2) crossing a simply supported span, as ten stops of the mid-span moment on an overlay (colour channel and snapping track) and as the envelope diagram of the influence line.

use super::checks::{part_2, part_3};
use super::model::{En1991Snapshot, StructureKind};
use super::scene::diagram_kit::{curves, Xy};
use super::scene::projection_kit::diagram_parts::{axis, column, stroke};
use super::scene::projection_kit::tracks;
use super::scene::*;

/// 🆔 Id of the overlay entity that carries the travel fields.
pub const OVERLAY: &str = "bridge:travel";
/// 🔢 Number of intervals of the travel; the track has one more stop.
pub const STOPS: usize = 10;
/// 🏷️ Id of the parameter track.
pub const TRACK: &str = "traffic-travel";
const ENVELOPE: &str = "travel@envelope";
const SAMPLES: usize = 96;

fn stop_id(k: usize) -> String {
    format!("travel@{k}")
}

fn lanes(d: &En1991Snapshot) -> Vec<u8> {
    if d.bridge_lanes.is_empty() {
        (1..=part_2::notional_lanes(d.bridge_lane_width.max(0.0)).0.min(3)).collect()
    } else {
        d.bridge_lanes.iter().map(|l| l.lm1_lane).filter(|n| (1..=3).contains(n)).collect()
    }
}

/// 🏋️ Sum of the tandem axle loads Q_ak (N) of all loaded lanes, placed side by side without transverse distribution.
pub fn axle_group_n(d: &En1991Snapshot) -> f64 {
    lanes(d).iter().map(|n| part_2::lm1_tandem_n(d.annex, *n)).sum()
}

pub use super::checks::part_2::{eta_moment, eta_reaction, tandem_effects as effects, tandem_envelope as envelope};

fn travel_length(span: f64, spacing: f64) -> f64 {
    span + spacing
}

/// 🌈️ Envelope of the travel: the largest mid-span moment (N·m) and left reaction (N); both are piecewise linear in the front axle position, so the maximum sits at one of the breakpoints.
pub fn envelope(axle_n: f64, span: f64, spacing: f64) -> (f64, f64) {
    [0.0, spacing, span / 2.0, span / 2.0 + spacing, span, span + spacing].iter().map(|x| effects(axle_n, *x, span, spacing)).fold((0.0, 0.0), |a, e| (f64::max(a.0, e.0), f64::max(a.1, e.1)))
}

fn field(id: String, nm: f64) -> ScalarField {
    ScalarField { id, unit: QuantityKind::Moment, samples: FieldSamples::Instance { values: vec![nm] } }
}

/// 🌈️ The colour channel of the overlay: the mid-span moment at each of the ten stops and the envelope (active).
pub fn channel(d: &En1991Snapshot) -> ColourChannel {
    let (axle, span, spacing) = (axle_group_n(d), d.bridge_span.max(1.0), part_2::lm1_axle_spacing_m());
    let mut fields: Vec<ScalarField> = (0..=STOPS).map(|k| field(stop_id(k), effects(axle, travel_length(span, spacing) * k as f64 / STOPS as f64, span, spacing).0)).collect();
    fields.push(field(ENVELOPE.into(), envelope(axle, span, spacing).0));
    ColourChannel::Fields { active: ENVELOPE.into(), fields }
}

/// 🧩 Gives the overlay entities their travel channel and seals them again.
pub fn apply(entities: &mut [SceneEntity], d: &En1991Snapshot) {
    for entity in entities.iter_mut().filter(|e| e.entity.id == OVERLAY) {
        entity.appearance.channel = channel(d);
        let sealed = entity.clone().sealed();
        *entity = sealed;
    }
}

/// 🎞️ The snapping track over the travel stops, present when the scene holds the overlay.
pub fn track(entities: &[SceneEntity]) -> Option<ParameterTrack> {
    entities.iter().any(|e| e.entity.id == OVERLAY).then_some(())?;
    let stops = (0..=STOPS).map(|k| (format!("Tandem at {} % of the travel", k * 100 / STOPS), format!("Tandem bei {} % der Fahrt", k * 100 / STOPS), Some(stop_id(k)), None)).collect();
    tracks::indexed(TRACK, TrackKind::Combination, ("Tandem travel", "Tandemfahrt"), stops)
}

/// 📈️ Mid-span moment and left reaction against the position of the front axle, with the envelope as limit line and the governing position as marker (a bridge only).
pub fn diagram(d: &En1991Snapshot) -> Option<Diagram2d> {
    if d.structure_kind != StructureKind::Bridge {
        return None;
    }
    let (axle, span, spacing) = (axle_group_n(d), d.bridge_span.max(1.0), part_2::lm1_axle_spacing_m());
    let length = travel_length(span, spacing);
    let moment: Vec<Xy> = curves::sample(|x| effects(axle, x, span, spacing).0 / 1000.0, 0.0, length, SAMPLES);
    let reaction: Vec<Xy> = curves::sample(|x| effects(axle, x, span, spacing).1 / 1000.0, 0.0, length, SAMPLES);
    let top = moment.iter().copied().fold([0.0, 0.0], |best, p| if p[1] > best[1] { p } else { best });
    let polyline = |id: &str, en: &str, de: &str, points: &[Xy], tone: Tone| Series::Polyline { id: id.into(), label: LocalizedCopy::new(en, de), points: points.iter().map(|p| Vec2::new(p[0], p[1])).collect(), style: stroke(tone, 2.5, Dash::Solid) };
    Some(Diagram2d {
        id: "bridge:travel".into(),
        title: LocalizedCopy::new("LM1 tandem travel: moment envelope", "LM1-Tandemfahrt: Momentenhüllende"),
        description: LocalizedCopy::new(
            "Mid-span moment and left support reaction while the LM1 tandem group of all loaded lanes crosses a simply supported span; the dashed line is the envelope (EN 1991-2, 4.3.2; lanes side by side, no transverse distribution, UDL not included).",
            "Feldmoment und linke Auflagerkraft während die LM1-Tandemgruppe aller belasteten Fahrstreifen einen Einfeldträger überquert; die gestrichelte Linie ist die Hüllende (EN 1991-2, 4.3.2; Fahrstreifen nebeneinander, ohne Querverteilung, ohne Gleichlast).",
        ),
        axes: vec![axis("Front axle position x", "Position der Vorderachse x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, length))), axis("Mid-span moment M", "Feldmoment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
        series: vec![
            polyline("travel-moment", "M(x) at mid-span", "M(x) in Feldmitte", &moment, Tone::Info),
            Series::Limit { id: "travel-envelope".into(), label: LocalizedCopy::new("Envelope M_max", "Hüllende M_max"), axis: AxisRef::Y, value: envelope(axle, span, spacing).0 / 1000.0, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
        ],
        markers: vec![Marker {
            id: "travel-governing".into(),
            at: Vec2::new(top[0], top[1]),
            shape: MarkerShape::Circle,
            tone: Tone::Info,
            label: LocalizedCopy::new(format!("Governing position x = {:.2} m, M = {:.0} kNm", top[0], top[1]), format!("Maßgebende Stellung x = {:.2} m, M = {:.0} kNm", top[0], top[1]).replace('.', ",")),
            utilisation: None,
            bound_check: Some("en1991.2.lm1-tandem".into()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, "bridge", "site")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Front axle position x", "Position der Vorderachse x", "m", moment.iter().map(|p| p[0]).collect()), column("Mid-span moment M", "Feldmoment M", "kN·m", moment.iter().map(|p| p[1]).collect()), column("Left reaction A", "Linke Auflagerkraft A", "kN", reaction.iter().map(|p| p[1]).collect())] },
    })
}

const SECTIONS: usize = 40;

/// 🛣️ Line load (N/m) of the LM1 uniform loads of all lanes and of the remaining area of the carriageway.
pub fn udl_n_m(d: &En1991Snapshot) -> f64 {
    let widths: Vec<f64> = d.bridge_lanes.iter().map(|l| l.width).collect();
    let (count, lane_width, remaining) = part_2::lane_layout(d.bridge_lane_width.max(0.0), &widths);
    let lanes: f64 = if d.bridge_lanes.is_empty() { (1..=count).map(|n| part_2::lm1_udl_pa(d.annex, n) * lane_width).sum() } else { d.bridge_lanes.iter().map(|l| part_2::lm1_udl_pa(d.annex, l.lm1_lane) * l.width).sum() };
    lanes + part_2::remaining_area_udl_pa(d.annex) * remaining
}

/// 🌈️ Envelope of the LM1 tandem group and the uniform loads at one section of the span: the largest bending moment (N·m), the largest and the smallest shear force (N).
pub fn section_envelope(d: &En1991Snapshot, section: f64) -> (f64, f64, f64) {
    let (axle, span, spacing) = (axle_group_n(d), d.bridge_span.max(1.0), part_2::lm1_axle_spacing_m());
    let (tandem, udl) = (part_2::tandem_section_envelope(axle, span, spacing, section), part_2::udl_section_envelope(udl_n_m(d), span, section));
    (tandem.0 + udl.0, tandem.1 + udl.1, tandem.2 + udl.2)
}

/// 📈️ Moment and shear envelopes along the span while the LM1 tandem group sweeps the deck (influence lines of the section, uniform loads added); both diagrams are projected from the document, so they follow every drag of the span, the lanes and the loads (a bridge only).
pub fn envelope_diagrams(d: &En1991Snapshot) -> Vec<Diagram2d> {
    if d.structure_kind != StructureKind::Bridge {
        return Vec::new();
    }
    let (axle, span, spacing) = (axle_group_n(d), d.bridge_span.max(1.0), part_2::lm1_axle_spacing_m());
    let moment_tandem: Vec<Xy> = curves::sample(|x| part_2::tandem_section_envelope(axle, span, spacing, x).0 / 1000.0, 0.0, span, SECTIONS);
    let moment: Vec<Xy> = curves::sample(|x| section_envelope(d, x).0 / 1000.0, 0.0, span, SECTIONS);
    let shear_max: Vec<Xy> = curves::sample(|x| section_envelope(d, x).1 / 1000.0, 0.0, span, SECTIONS);
    let shear_min: Vec<Xy> = curves::sample(|x| section_envelope(d, x).2 / 1000.0, 0.0, span, SECTIONS);
    let top = moment.iter().copied().fold([0.0, 0.0], |best, p| if p[1] > best[1] { p } else { best });
    let polyline = |id: &str, en: &str, de: &str, points: &[Xy], tone: Tone, dash: Dash| Series::Polyline { id: id.into(), label: LocalizedCopy::new(en, de), points: points.iter().map(|p| Vec2::new(p[0], p[1])).collect(), style: stroke(tone, 2.5, dash) };
    let position = || axis("Section x", "Schnitt x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, span)));
    let note = LocalizedCopy::new("(EN 1991-2, 4.3.2; influence lines of a simply supported span, lanes side by side, no transverse distribution)", "(EN 1991-2, 4.3.2; Einflusslinien eines Einfeldträgers, Fahrstreifen nebeneinander, ohne Querverteilung)");
    let describe = |en: &str, de: &str| LocalizedCopy::new(format!("{en} {}", note.en), format!("{de} {}", note.de));
    let columns = |en: &str, de: &str, unit: &str, series: &[&[Xy]]| { std::iter::once(column("Section x", "Schnitt x", "m", moment.iter().map(|p| p[0]).collect())).chain(series.iter().map(|points| column(en, de, unit, points.iter().map(|p| p[1]).collect()))).collect::<Vec<_>>() };
    let focus = Some(EntityRef::new(EntityKind::Solid, "bridge", "site"));
    vec![
        Diagram2d {
            id: "bridge:envelope-moment".into(),
            title: LocalizedCopy::new("LM1 moment envelope along the span", "LM1-Momentenhüllende entlang der Stützweite"),
            description: describe("Largest bending moment at every section while the tandem group sweeps the span: tandem alone and with the uniform loads", "Größtes Biegemoment in jedem Schnitt während die Tandemgruppe die Stützweite überfährt: Tandem allein und mit den Gleichlasten"),
            axes: vec![position(), axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
            series: vec![polyline("envelope-m-tandem", "M_max tandem", "M_max Tandem", &moment_tandem, Tone::Info, Dash::Dashed), polyline("envelope-m-total", "M_max tandem + UDL", "M_max Tandem + Gleichlast", &moment, Tone::Warning, Dash::Solid)],
            markers: vec![Marker { id: "envelope-m-governing".into(), at: Vec2::new(top[0], top[1]), shape: MarkerShape::Circle, tone: Tone::Warning, label: LocalizedCopy::new(format!("M_max = {:.0} kNm at x = {:.2} m", top[1], top[0]), format!("M_max = {:.0} kNm bei x = {:.2} m", top[1], top[0]).replace('.', ",")), utilisation: None, bound_check: Some("en1991.2.lm1-tandem".into()), binding: None }],
            annotations: Vec::new(),
            focus: focus.clone(),
            legend: true,
            table_twin: DataTable { columns: columns("M_max", "M_max", "kN·m", &[&moment_tandem, &moment]) },
        },
        Diagram2d {
            id: "bridge:envelope-shear".into(),
            title: LocalizedCopy::new("LM1 shear envelope along the span", "LM1-Querkrafthüllende entlang der Stützweite"),
            description: describe("Largest and smallest shear force at every section while the tandem group sweeps the span, with the uniform loads", "Größte und kleinste Querkraft in jedem Schnitt während die Tandemgruppe die Stützweite überfährt, mit den Gleichlasten"),
            axes: vec![position(), axis("Shear force V", "Querkraft V", QuantityKind::Force, "kN", AxisScale::Linear, None)],
            series: vec![polyline("envelope-v-max", "V_max", "V_max", &shear_max, Tone::Warning, Dash::Solid), polyline("envelope-v-min", "V_min", "V_min", &shear_min, Tone::Info, Dash::Solid)],
            markers: Vec::new(),
            annotations: Vec::new(),
            focus,
            legend: true,
            table_twin: DataTable { columns: columns("V", "V", "kN", &[&shear_max, &shear_min]) },
        },
    ]
}

/// 🏗️ Moment and shear envelopes along the runway girder while the crane travels (EN 1991-3, 2.7): two wheels of the design load φ₂·Q_r at the wheel base cross a simply supported span, the diagrams are projected from the document and follow every drag of the runway length, the wheel base and the classes (a crane only).
pub fn crane_envelope_diagrams(d: &En1991Snapshot) -> Vec<Diagram2d> {
    let Some(wheel) = part_3::design_vertical_wheel_n(&d.crane_class, &d.hoist_class, d.hoisting_speed).filter(|_| d.crane_claimed) else { return Vec::new() };
    let (span, base) = (d.crane_runway_length.max(1.0), d.crane_wheel_base.max(0.1));
    let moment: Vec<Xy> = curves::sample(|x| part_2::tandem_section_envelope(wheel, span, base, x).0 / 1000.0, 0.0, span, SECTIONS);
    let shear_max: Vec<Xy> = curves::sample(|x| part_2::tandem_section_envelope(wheel, span, base, x).1 / 1000.0, 0.0, span, SECTIONS);
    let shear_min: Vec<Xy> = curves::sample(|x| part_2::tandem_section_envelope(wheel, span, base, x).2 / 1000.0, 0.0, span, SECTIONS);
    let top = moment.iter().copied().fold([0.0, 0.0], |best, p| if p[1] > best[1] { p } else { best });
    let polyline = |id: &str, en: &str, de: &str, points: &[Xy], tone: Tone| Series::Polyline { id: id.into(), label: LocalizedCopy::new(en, de), points: points.iter().map(|p| Vec2::new(p[0], p[1])).collect(), style: stroke(tone, 2.5, Dash::Solid) };
    let position = || axis("Section x", "Schnitt x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, span)));
    let note = LocalizedCopy::new("(EN 1991-3, 2.7; influence lines of a simply supported runway girder, two wheels at the wheel base, no continuity)", "(EN 1991-3, 2.7; Einflusslinien eines Einfeld-Kranbahnträgers, zwei Räder im Radstand, ohne Durchlaufwirkung)");
    let describe = |en: &str, de: &str| LocalizedCopy::new(format!("{en} {}", note.en), format!("{de} {}", note.de));
    let columns = |en: &str, de: &str, unit: &str, series: &[&[Xy]]| { std::iter::once(column("Section x", "Schnitt x", "m", moment.iter().map(|p| p[0]).collect())).chain(series.iter().map(|points| column(en, de, unit, points.iter().map(|p| p[1]).collect()))).collect::<Vec<_>>() };
    let focus = Some(EntityRef::new(EntityKind::Member, "crane", "site"));
    vec![
        Diagram2d {
            id: "crane:envelope-moment".into(),
            title: LocalizedCopy::new("Crane runway moment envelope", "Momentenhüllende der Kranbahn"),
            description: describe("Largest bending moment of the runway girder at every section while the crane travels along it", "Größtes Biegemoment des Kranbahnträgers in jedem Schnitt während der Kran darüber fährt"),
            axes: vec![position(), axis("Bending moment M", "Biegemoment M", QuantityKind::Moment, "kN·m", AxisScale::Linear, None)],
            series: vec![polyline("envelope-m", "M_max", "M_max", &moment, Tone::Warning)],
            markers: vec![Marker { id: "crane-envelope-m-governing".into(), at: Vec2::new(top[0], top[1]), shape: MarkerShape::Circle, tone: Tone::Warning, label: LocalizedCopy::new(format!("M_max = {:.0} kNm at x = {:.2} m", top[1], top[0]), format!("M_max = {:.0} kNm bei x = {:.2} m", top[1], top[0]).replace('.', ",")), utilisation: None, bound_check: Some("en1991.3.crane-runway".into()), binding: None }],
            annotations: Vec::new(),
            focus: focus.clone(),
            legend: true,
            table_twin: DataTable { columns: columns("M_max", "M_max", "kN·m", &[&moment]) },
        },
        Diagram2d {
            id: "crane:envelope-shear".into(),
            title: LocalizedCopy::new("Crane runway shear envelope", "Querkrafthüllende der Kranbahn"),
            description: describe("Largest and smallest shear force of the runway girder at every section while the crane travels along it", "Größte und kleinste Querkraft des Kranbahnträgers in jedem Schnitt während der Kran darüber fährt"),
            axes: vec![position(), axis("Shear force V", "Querkraft V", QuantityKind::Force, "kN", AxisScale::Linear, None)],
            series: vec![polyline("envelope-v-max", "V_max", "V_max", &shear_max, Tone::Warning), polyline("envelope-v-min", "V_min", "V_min", &shear_min, Tone::Info)],
            markers: Vec::new(),
            annotations: Vec::new(),
            focus,
            legend: true,
            table_twin: DataTable { columns: columns("V", "V", "kN", &[&shear_max, &shear_min]) },
        },
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
