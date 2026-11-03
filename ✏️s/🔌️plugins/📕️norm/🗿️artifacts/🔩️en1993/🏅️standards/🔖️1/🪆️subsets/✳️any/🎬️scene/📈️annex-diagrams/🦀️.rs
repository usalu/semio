//! 📈️ Annex diagrams: the hoop buckling stress of a ring-stiffened shell over the ring spacing, the corrugated web shear resistance over the web thickness with its local and global branches, the clamped deck strip moments under the wheel patch and the free-flange moment of a purlin with and without the sheeting spring.

use super::checks::clauses::shell_stability;
use super::checks::clauses_annex::{corrugated_web, orthotropic_deck, purlin_free_flange, shell_pressure};
use super::checks::{clauses::cold_formed, AnnexParams};
use super::model::{BridgeFatigue, ColdFormedMember, En1993Snapshot, PlatedPanel, SiloShell};
use super::norm::CheckReport;
use super::scene::diagram_kit::Xy;
use super::scene::projection_kit::diagram_parts::{axis, column, stroke};
use super::scene::*;

const SAMPLES: usize = 40;
const E_MPA: f64 = 210_000.0;
const NU: f64 = 0.3;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn range(values: &[f64]) -> (f64, f64) {
    (values.iter().cloned().fold(f64::INFINITY, f64::min), values.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
}

fn polyline(points: &[Xy]) -> Vec<Vec2> {
    points.iter().map(|q| Vec2::new(q[0], q[1])).collect()
}

fn ring_spacing(d: &En1993Snapshot, s: &SiloShell) -> Option<Diagram2d> {
    let class = s.fabrication_class.as_str().chars().next()?;
    let l_ring = if s.ring_spacing > 0.0 { s.ring_spacing } else { s.length };
    if !(s.external_pressure > 0.0 && s.radius > 0.0 && s.thickness > 0.0 && s.fy > 0.0 && l_ring > 0.0) {
        return None;
    }
    let (r, t, fy, gm1) = (s.radius * 1e3, s.thickness * 1e3, s.fy / 1e6, AnnexParams::for_choice(d.annex).gamma_m1);
    let c_theta = if s.boundary_factor_hoop > 0.0 { s.boundary_factor_hoop } else { 1.0 };
    let sigma_ed = shell_pressure::hoop_stress(s.external_pressure / 1e6, r, t);
    let at = |l: f64| shell_stability::hoop_design(l * 1e3, r, t, fy, class, c_theta, gm1).sigma_rd;
    let spans: Vec<f64> = (0..=SAMPLES).map(|i| l_ring * (0.2 + 1.8 * i as f64 / SAMPLES as f64)).collect();
    let curve: Vec<Xy> = spans.iter().map(|l| [*l, at(*l)]).collect();
    let current = at(l_ring);
    let utilisation = sigma_ed / current.max(1e-9);
    Some(Diagram2d {
        id: format!("ring-spacing:{}", s.id),
        title: loc(&format!("Hoop buckling stress over the ring spacing – {}", s.id), &format!("Umfangsbeulspannung über dem Ringabstand – {}", s.id)),
        description: loc(
            &format!("σ_θ,Rd of the shell between the rings (EN 1993-1-6 LS3, class {class}) against the membrane stress p r/t = {sigma_ed:.1} MPa of the external pressure; the marker is the current spacing {l_ring:.2} m."),
            &format!("σ_θ,Rd der Schale zwischen den Ringen (EN 1993-1-6 LS3, Klasse {class}) gegen die Membranspannung p r/t = {sigma_ed:.1} MPa aus dem Außendruck; die Marke zeigt den aktuellen Abstand {l_ring:.2} m."),
        ),
        axes: vec![axis("Ring spacing l_r", "Ringabstand l_r", QuantityKind::Length, "m", AxisScale::Linear, Some(range(&spans))), axis("Hoop stress σ_θ", "Umfangsspannung σ_θ", QuantityKind::Stress, "MPa", AxisScale::Linear, Some((0.0, curve.iter().map(|q| q[1]).fold(sigma_ed, f64::max) * 1.1)))],
        series: vec![
            Series::Polyline { id: "sigma-rd".into(), label: loc("σ_θ,Rd", "σ_θ,Rd"), points: polyline(&curve), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "sigma-ed".into(), label: loc("σ_θ,Ed = p r/t", "σ_θ,Ed = p r/t"), axis: AxisRef::Y, value: sigma_ed, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "spacing".into(), at: Vec2::new(l_ring, current), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("l_r = {l_ring:.2} m: σ_θ,Rd = {current:.1} MPa"), &format!("l_r = {l_ring:.2} m: σ_θ,Rd = {current:.1} MPa")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-6.D.ext.{}", s.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Shell, &s.id, format!("siloShells[id={}]", s.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Ring spacing l_r", "Ringabstand l_r", "m", curve.iter().map(|q| q[0]).collect()), column("Hoop resistance σ_θ,Rd", "Umfangswiderstand σ_θ,Rd", "MPa", curve.iter().map(|q| q[1]).collect())] },
    })
}

fn corrugated_shear(d: &En1993Snapshot, p: &PlatedPanel) -> Option<Diagram2d> {
    if !(p.corrugation_flat > 0.0 && p.corrugation_depth > 0.0 && p.web_height > 0.0 && p.thickness > 0.0 && p.fy > 0.0) {
        return None;
    }
    let g = corrugated_web::Corrugation { a0: p.corrugation_flat * 1e3, a1: p.corrugation_projection * 1e3, d: p.corrugation_depth * 1e3 };
    let (hw, t, fy, gm1) = (p.web_height * 1e3, p.thickness * 1e3, p.fy / 1e6, AnnexParams::for_choice(d.annex).gamma_m1);
    let thicknesses: Vec<f64> = (0..=SAMPLES).map(|i| t * (0.5 + 1.5 * i as f64 / SAMPLES as f64)).collect();
    let at = |th: f64| corrugated_web::shear_resistance(&g, hw, th, fy, E_MPA, NU, gm1);
    let series_of = |pick: &dyn Fn(&corrugated_web::CorrugatedShear) -> f64| -> Vec<Vec2> { thicknesses.iter().map(|th| Vec2::new(*th, pick(&at(*th)) * (fy / 3.0_f64.sqrt()) * hw * th / gm1 / 1e3)).collect() };
    let total: Vec<Vec2> = thicknesses.iter().map(|th| Vec2::new(*th, at(*th).v_b_rd / 1e3)).collect();
    let local = series_of(&|r| corrugated_web::chi_local(r.lambda_local));
    let global = series_of(&|r| corrugated_web::chi_global(r.lambda_global));
    let current = at(t);
    let v_ed = p.shear_force.abs() / 1e3;
    let utilisation = if current.v_b_rd > 0.0 { v_ed * 1e3 / current.v_b_rd } else { 0.0 };
    let top = total.iter().chain(&local).chain(&global).map(|q| q.y).fold(v_ed, f64::max) * 1.1;
    Some(Diagram2d {
        id: format!("corrugated-shear:{}", p.id),
        title: loc(&format!("Corrugated web shear resistance – {}", p.id), &format!("Schubtragfähigkeit des Trapezstegs – {}", p.id)),
        description: loc(
            &format!("V_b,Rd over the web thickness (EN 1993-1-5 Annex D): the local branch χ_c,l and the global branch χ_c,g of the equivalent orthotropic plate; the smaller governs. At t = {t:.1} mm: χ_c = {:.3}, V_b,Rd = {:.1} kN.", current.chi, current.v_b_rd / 1e3),
            &format!("V_b,Rd über der Stegdicke (EN 1993-1-5 Anhang D): lokaler Ast χ_c,l und globaler Ast χ_c,g der äquivalenten orthotropen Platte; der kleinere maßgebend. Bei t = {t:.1} mm: χ_c = {:.3}, V_b,Rd = {:.1} kN.", current.chi, current.v_b_rd / 1e3),
        ),
        axes: vec![axis("Web thickness t_w", "Stegdicke t_w", QuantityKind::Length, "mm", AxisScale::Linear, Some(range(&thicknesses))), axis("Shear resistance V_b,Rd", "Schubtragfähigkeit V_b,Rd", QuantityKind::Force, "kN", AxisScale::Linear, Some((0.0, top)))],
        series: vec![
            Series::Polyline { id: "v-rd".into(), label: loc("V_b,Rd", "V_b,Rd"), points: total.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Polyline { id: "local".into(), label: loc("Local branch", "Lokaler Ast"), points: local, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
            Series::Polyline { id: "global".into(), label: loc("Global branch", "Globaler Ast"), points: global, style: stroke(Tone::Warning, 1.5, Dash::Dotted) },
            Series::Limit { id: "v-ed".into(), label: loc("V_Ed", "V_Ed"), axis: AxisRef::Y, value: v_ed, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "thickness".into(), at: Vec2::new(t, current.v_b_rd / 1e3), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("t_w = {t:.1} mm: V_b,Rd = {:.1} kN", current.v_b_rd / 1e3), &format!("t_w = {t:.1} mm: V_b,Rd = {:.1} kN", current.v_b_rd / 1e3)), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-5.D.shear.{}", p.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &p.id, format!("platedPanels[id={}]", p.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Web thickness t_w", "Stegdicke t_w", "mm", thicknesses.clone()), column("Shear resistance V_b,Rd", "Schubtragfähigkeit V_b,Rd", "kN", total.iter().map(|q| q.y).collect())] },
    })
}

fn deck_strip(bf: &BridgeFatigue) -> Option<Diagram2d> {
    if !(bf.deck_thickness > 0.0 && bf.wheel_load > 0.0 && bf.wheel_patch > 0.0 && bf.trough_spacing > bf.trough_width && bf.trough_width > 0.0) {
        return None;
    }
    let (t, surfacing) = (bf.deck_thickness * 1e3, bf.surfacing_thickness * 1e3);
    let span = (bf.trough_spacing - bf.trough_width) * 1e3;
    let c = orthotropic_deck::patch_side(bf.wheel_patch * 1e3, surfacing, t);
    let strip = orthotropic_deck::deck_strip(bf.wheel_load, c, span, t);
    let w = bf.wheel_load / (c * c);
    let loaded = c.min(span);
    let (x1, x2) = ((span - loaded) / 2.0, (span + loaded) / 2.0);
    let reaction = w * loaded / 2.0;
    let free = |x: f64| if x <= x1 { reaction * x } else if x <= x2 { reaction * x - w * (x - x1).powi(2) / 2.0 } else { reaction * x - w * loaded * (x - (x1 + x2) / 2.0) };
    let stations: Vec<f64> = (0..=SAMPLES).map(|i| span * i as f64 / SAMPLES as f64).collect();
    let moments: Vec<f64> = stations.iter().map(|x| free(*x) - strip.m_hogging).collect();
    let curve: Vec<Vec2> = stations.iter().zip(&moments).map(|(x, m)| Vec2::new(*x, *m)).collect();
    let (lo, hi) = range(&moments);
    Some(Diagram2d {
        id: format!("deck-strip:{}", bf.id),
        title: loc(&format!("Deck plate strip under the wheel – {}", bf.id), &format!("Fahrbahnblechstreifen unter dem Rad – {}", bf.id)),
        description: loc(
            &format!("Bending moment per unit width of the strip clamped between the trough walls (clear span {span:.0} mm) under Q_w = {:.1} kN on a {c:.0} mm patch: hogging {:.0} N·mm/mm at the welds, sagging {:.0} N·mm/mm at midspan; screening model without longitudinal load spreading.", bf.wheel_load / 1e3, strip.m_hogging, strip.m_sagging),
            &format!("Biegemoment je Breiteneinheit des zwischen den Trogwänden eingespannten Streifens (lichte Weite {span:.0} mm) unter Q_w = {:.1} kN auf {c:.0} mm Aufstandsfläche: Stützmoment {:.0} N·mm/mm an den Nähten, Feldmoment {:.0} N·mm/mm in Feldmitte; Abschätzung ohne Lastausbreitung in Längsrichtung.", bf.wheel_load / 1e3, strip.m_hogging, strip.m_sagging),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "mm", AxisScale::Linear, Some((0.0, span))), axis("Moment M", "Moment M", QuantityKind::Force, "N·mm/mm", AxisScale::Linear, Some((lo * 1.2, hi * 1.2)))],
        series: vec![Series::Polyline { id: "moment".into(), label: loc("M(x)", "M(x)"), points: curve.clone(), style: stroke(Tone::Info, 2.5, Dash::Solid) }],
        markers: vec![
            Marker { id: "hogging".into(), at: Vec2::new(0.0, -strip.m_hogging), shape: MarkerShape::Circle, tone: Tone::Warning, label: loc(&format!("Hogging {:.0}", strip.m_hogging), &format!("Stützmoment {:.0}", strip.m_hogging)), utilisation: None, bound_check: Some(format!("en1993.2.9.5.deck.{}", bf.id)), binding: None },
            Marker { id: "sagging".into(), at: Vec2::new(span / 2.0, strip.m_sagging), shape: MarkerShape::Circle, tone: Tone::Info, label: loc(&format!("Sagging {:.0}", strip.m_sagging), &format!("Feldmoment {:.0}", strip.m_sagging)), utilisation: None, bound_check: Some(format!("en1993.2.9.5.deck.{}", bf.id)), binding: None },
        ],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &bf.id, format!("bridgeFatigue[id={}]", bf.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "mm", stations), column("Moment M", "Moment M", "N·mm/mm", moments)] },
    })
}

fn purlin_flange(d: &En1993Snapshot, c: &ColdFormedMember) -> Option<Diagram2d> {
    if !(c.purlin_span > 0.0 && c.purlin_load > 0.0 && c.web_height > 0.0 && c.flange_width > 0.0 && c.thickness > 0.0 && c.fy > 0.0) {
        return None;
    }
    let (t, b, lip, hw) = (c.thickness * 1e3, c.flange_width * 1e3, c.lip_length * 1e3, c.web_height * 1e3);
    let (l, q_h) = (c.purlin_span * 1e3, c.purlin_lateral_ratio * c.purlin_load / 1e3);
    let flange = purlin_free_flange::free_flange(t, b, lip, hw);
    let c_d = if c.sheeting_stiffness > 0.0 { cold_formed::c_d_series(c.sheeting_stiffness, cold_formed::c_d_web_bending(E_MPA, t, hw)) } else { 0.0 };
    let k = purlin_free_flange::lateral_spring(c_d, hw);
    let ei = E_MPA * flange.inertia;
    let stations: Vec<f64> = (0..=SAMPLES).map(|i| l * i as f64 / SAMPLES as f64).collect();
    let with: Vec<f64> = stations.iter().map(|x| purlin_free_flange::moment_on_spring(q_h, l, *x, ei, k) / 1e3).collect();
    let without: Vec<f64> = stations.iter().map(|x| purlin_free_flange::moment_on_spring(q_h, l, *x, ei, 0.0) / 1e3).collect();
    let metres: Vec<f64> = stations.iter().map(|x| x / 1e3).collect();
    let m_rd = flange.inertia / flange.y_max * c.fy / 1e6 / AnnexParams::for_choice(d.annex).gamma_m0 / 1e3;
    let peak = with.iter().fold(0.0_f64, |a, m| a.max(m.abs()));
    let utilisation = peak / m_rd.max(1e-9);
    let (lo, hi) = range(&without);
    Some(Diagram2d {
        id: format!("purlin-flange:{}", c.id),
        title: loc(&format!("Free flange moment of the purlin – {}", c.id), &format!("Moment des freien Gurts der Pfette – {}", c.id)),
        description: loc(
            &format!("Lateral bending moment of the free flange along the span {:.1} m under k_h q = {:.0} N/m: without restraint and on the sheeting spring K = C_D/h² = {k:.4} N/mm² (EN 1993-1-3 Annex E model, simply supported); the resistance of the flange is {m_rd:.1} N·m.", c.purlin_span, q_h * 1e3),
            &format!("Seitliches Biegemoment des freien Gurts entlang der Spannweite {:.1} m unter k_h q = {:.0} N/m: ohne Halterung und auf der Blechfeder K = C_D/h² = {k:.4} N/mm² (Modell EN 1993-1-3 Anhang E, Einfeldträger); die Gurtbeanspruchbarkeit beträgt {m_rd:.1} N·m.", c.purlin_span, q_h * 1e3),
        ),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, l / 1e3))), axis("Moment M_fz", "Moment M_fz", QuantityKind::Moment, "N·m", AxisScale::Linear, Some((lo.min(-m_rd) * 1.1, hi.max(m_rd) * 1.1)))],
        series: vec![
            Series::Polyline { id: "unrestrained".into(), label: loc("Without sheeting", "Ohne Blech"), points: metres.iter().zip(&without).map(|(x, m)| Vec2::new(*x, *m)).collect(), style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
            Series::Polyline { id: "restrained".into(), label: loc("On the sheeting spring", "Auf der Blechfeder"), points: metres.iter().zip(&with).map(|(x, m)| Vec2::new(*x, *m)).collect(), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Limit { id: "m-rd".into(), label: loc("M_fz,Rd", "M_fz,Rd"), axis: AxisRef::Y, value: m_rd, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![Marker { id: "peak".into(), at: Vec2::new(l / 2e3, with[SAMPLES / 2]), shape: MarkerShape::Circle, tone: tone(utilisation), label: loc(&format!("Peak {peak:.1} N·m"), &format!("Maximum {peak:.1} N·m")), utilisation: Some(utilisation), bound_check: Some(format!("en1993.1-3.E.purlin.{}", c.id)), binding: None }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &c.id, format!("coldFormedMembers[id={}]", c.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", metres), column("Without sheeting", "Ohne Blech", "N·m", without), column("On the sheeting spring", "Auf der Blechfeder", "N·m", with)] },
    })
}

/// 📈️ The diagrams of the annex blocks; `focus` limits them to one entity id.
pub(crate) fn annex_diagrams(d: &En1993Snapshot, _report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let wanted = |id: &str| focus.is_none_or(|f| f == id);
    let mut out = Vec::new();
    for s in d.silo_shells.iter().filter(|s| wanted(&s.id)) {
        out.extend(ring_spacing(d, s));
    }
    for p in d.plated_panels.iter().filter(|p| wanted(&p.id)) {
        out.extend(corrugated_shear(d, p));
    }
    for b in d.bridge_fatigue.iter().filter(|b| wanted(&b.id)) {
        out.extend(deck_strip(b));
    }
    for c in d.cold_formed_members.iter().filter(|c| wanted(&c.id)) {
        out.extend(purlin_flange(d, c));
    }
    out
}
