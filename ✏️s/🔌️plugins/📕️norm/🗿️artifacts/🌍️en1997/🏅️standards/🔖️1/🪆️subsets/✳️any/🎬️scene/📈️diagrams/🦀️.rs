//! 📈️ Diagrams: earth pressure versus depth, CPT and SPT profiles with derived parameters, bearing capacity factors and load-settlement curves, each with its table twin.

use super::checks::catalogue::foundations::{bearing_drained, BearingIn};
use super::checks::catalogue::walls_and_anchors;
use super::kit::{entity_ref, loc, tone_of};
use super::mechanics as mech;
use super::scene::diagram_kit::curves;
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column};
use super::scene::projection_kit::diagram_parts;
use super::model::En1997Snapshot;
use super::norm::CheckReport;
use super::{Ctx, SceneInput};

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, scale: AxisScale) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, scale, None)
}

fn stroke(tone: Tone, dash: Dash) -> Stroke {
    Stroke { width_px: 2.0, dash, tone }
}

fn points(raw: &[[f64; 2]]) -> Vec<Vec2> {
    raw.iter().map(|p| Vec2::new(p[0], p[1])).collect()
}

fn polyline(id: &str, en: &str, de: &str, raw: &[[f64; 2]], tone: Tone, dash: Dash) -> Series {
    Series::Polyline { id: id.into(), label: loc(en, de), points: points(raw), style: stroke(tone, dash) }
}

/// 🏷️ A marker of a diagram: tied to `check`, or typed `reference:<row>` when no check draws this point.
fn marker(id: String, at: [f64; 2], shape: MarkerShape, tone: Tone, label: LocalizedCopy, check: Option<String>) -> Marker {
    let id = if check.is_some() { id } else { format!("{REFERENCE_MARKER_KIND}:{id}") };
    Marker { id, at: Vec2::new(at[0], at[1]), shape, tone, label, utilisation: None, bound_check: check, binding: None }
}

const STEPS: usize = 10;
const SPENCER_SLICES: usize = 60;

fn earth_pressure(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.retaining_walls
        .iter()
        .filter(|w| w.height > 1e-6 && focus.is_none_or(|f| f == w.id))
        .map(|w| {
            let hw = mech::total_height(w);
            let depths: Vec<f64> = (0..=STEPS).map(|i| hw * i as f64 / STEPS as f64).collect();
            let active: Vec<[f64; 2]> = depths.iter().map(|z| [mech::driving_pressure(w, *z) / 1000.0, w.height - z]).collect();
            let toe: Vec<f64> = (0..=STEPS).map(|i| w.embedment * i as f64 / STEPS as f64).collect();
            let passive: Vec<[f64; 2]> = if w.embedment > 1e-6 { toe.iter().map(|z| [-mech::passive_pressure(w, *z) / 1000.0, -z]).collect() } else { Vec::new() };
            let net: Vec<[f64; 2]> = passive.iter().zip(&toe).map(|(p, z)| [mech::driving_pressure(w, w.height + z) / 1000.0 + p[0], -z]).collect();
            let (ea, lever_a) = mech::driving_resultant(w);
            let (ep, lever_p) = mech::passive_resultant(w);
            let earth_check = ctx.binding(&w.id, &["en1997.9.earthPressure."]).map(|b| b.check_id);
            let mut series = vec![polyline("active", "Driving earth pressure", "Treibender Erddruck", &active, Tone::Error, Dash::Solid)];
            if passive.len() >= 2 {
                series.push(polyline("passive", "Passive resistance", "Erdwiderstand", &passive, Tone::Info, Dash::Dashed));
                series.push(polyline("net", "Net pressure", "Resultierender Druck", &net, Tone::Warning, Dash::Dotted));
            }
            series.push(Series::Limit { id: "dredge".into(), label: loc("Dredge level", "Aushubsohle"), axis: AxisRef::Y, value: 0.0, style: stroke(Tone::Neutral, Dash::Dotted) });
            let mut markers = vec![marker(format!("{}:ea", w.id), [mech::driving_pressure(w, hw - lever_a) / 1000.0, -w.embedment + lever_a], MarkerShape::Triangle, Tone::Error, loc(&format!("Ea = {:.1} kN/m", ea / 1000.0), &format!("Ea = {:.1} kN/m", ea / 1000.0)), earth_check.clone())];
            if ep > 1.0 {
                markers.push(marker(format!("{}:ep", w.id), [-mech::passive_pressure(w, w.embedment - lever_p) / 1000.0, -w.embedment + lever_p], MarkerShape::Square, Tone::Info, loc(&format!("Ep = {:.1} kN/m", ep / 1000.0), &format!("Ep = {:.1} kN/m", ep / 1000.0)), earth_check.clone()));
            }
            let elevation: Vec<f64> = active.iter().map(|p| p[1]).collect();
            let table = DataTable {
                columns: vec![
                    column("Elevation", "Höhe", "m", elevation),
                    column("Driving pressure", "Treibender Druck", "kPa", active.iter().map(|p| p[0]).collect()),
                    column("Passive resistance", "Erdwiderstand", "kPa", depths.iter().map(|z| -mech::passive_pressure(w, (z - w.height).max(0.0)) / 1000.0).collect()),
                ],
            };
            Diagram2d {
                id: format!("earth-pressure:{}", w.id),
                title: loc(&format!("Earth pressure on {}", w.id), &format!("Erddruck auf {}", w.id)),
                description: loc(&format!("Driving pressure K = {:.3} and passive resistance Kp = {:.2} against elevation.", mech::k_driving(w), mech::k_passive(w)), &format!("Treibender Druck K = {:.3} und Erdwiderstand Kp = {:.2} über der Höhe.", mech::k_driving(w), mech::k_passive(w))),
                axes: vec![axis("Pressure", "Druck", QuantityKind::Pressure, "kPa", AxisScale::Linear), axis("Elevation", "Höhe", QuantityKind::Length, "m", AxisScale::Linear)],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &w.id, "retainingWalls")),
                legend: true,
                table_twin: table,
            }
        })
        .collect()
}

fn step_profile(d: &super::model::En1997Snapshot, value: impl Fn(&super::model::SoilLayer) -> f64) -> Vec<[f64; 2]> {
    d.layers.iter().flat_map(|l| [[value(l), -l.depth_top], [value(l), -l.depth_bottom]]).collect()
}

fn profile(ctx: &Ctx, id: &str, en: &str, de: &str, qen: &str, qde: &str, quantity: QuantityKind, unit: &str, value: impl Fn(&super::model::SoilLayer) -> f64 + Copy) -> Diagram2d {
    let d = ctx.input.document;
    let raw = step_profile(d, value);
    let mut series = vec![polyline("profile", qen, qde, &raw, Tone::Info, Dash::Solid), Series::Limit { id: "groundwater".into(), label: loc("Groundwater table", "Grundwasserspiegel"), axis: AxisRef::Y, value: -d.groundwater_level, style: stroke(Tone::Neutral, Dash::Dashed) }];
    series.push(Series::Limit { id: "investigation".into(), label: loc("Investigation depth", "Erkundungstiefe"), axis: AxisRef::Y, value: -d.investigation_depth, style: stroke(Tone::Warning, Dash::Dotted) });
    let markers = d
        .layers
        .iter()
        .map(|l| {
            let mid = -(l.depth_top + l.depth_bottom) / 2.0;
            let text = format!("{}: phi' {:.0} deg, E_oed {:.0} MPa", l.id, l.phi_prime_deg, l.oedometric_modulus / 1e6);
            marker(format!("{}:derived", l.id), [value(l), mid], MarkerShape::Diamond, tone_of(&ctx.binding(&l.id, &[])), loc(&text, &text), ctx.binding(&l.id, &[]).map(|b| b.check_id))
        })
        .collect();
    let table = DataTable {
        columns: vec![
            column("Layer top", "Schichtoberkante", "m", d.layers.iter().map(|l| l.depth_top).collect()),
            column("Layer bottom", "Schichtunterkante", "m", d.layers.iter().map(|l| l.depth_bottom).collect()),
            column(qen, qde, unit, d.layers.iter().map(value).collect()),
            column("Derived friction angle", "Abgeleiteter Reibungswinkel", "°", d.layers.iter().map(|l| l.phi_prime_deg).collect()),
            column("Oedometric modulus", "Steifemodul", "MPa", d.layers.iter().map(|l| l.oedometric_modulus / 1e6).collect()),
        ],
    };
    Diagram2d {
        id: id.into(),
        title: loc(en, de),
        description: loc(&format!("{en} per layer with the derived parameters of each layer."), &format!("{de} je Schicht mit den abgeleiteten Kennwerten.")),
        axes: vec![axis(qen, qde, quantity, unit, AxisScale::Linear), axis("Elevation", "Höhe", QuantityKind::Length, "m", AxisScale::Linear)],
        series,
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: table,
    }
}

/// 🧱️ One soil profile diagram of the investigation points over the elevation: a polyline per point, the layer interfaces, the investigation depth and the depth z_a required below the deepest footing; recomputed from the document, so it follows every edit of a reading, a layer or a basis.
fn ground_profile(ctx: &Ctx, id: &str, en: &str, de: &str, unit: &str, quantity: QuantityKind, curves: Vec<(String, Vec<[f64; 2]>)>, characteristic: Vec<(String, Vec<[f64; 2]>, Dash)>) -> Diagram2d {
    let d = ctx.input.document;
    let mut series: Vec<Series> = curves.iter().enumerate().map(|(k, (name, raw))| polyline(name, name, name, raw, if k % 2 == 0 { Tone::Info } else { Tone::Neutral }, Dash::Solid)).collect();
    series.extend(characteristic.iter().map(|(name, raw, dash)| polyline(name, name, name, raw, Tone::Warning, *dash)));
    for l in &d.layers {
        series.push(Series::Limit { id: format!("layer:{}", l.id), label: loc(&format!("Bottom of {}", l.id), &format!("Unterkante {}", l.id)), axis: AxisRef::Y, value: -l.depth_bottom, style: stroke(Tone::Neutral, Dash::Dotted) });
    }
    let reached = super::checks::ground_derivation::reached_depth(d);
    series.push(Series::Limit { id: "investigation".into(), label: loc("Investigation depth", "Erkundungstiefe"), axis: AxisRef::Y, value: -reached, style: stroke(Tone::Warning, Dash::Dotted) });
    let required = d.footings.iter().map(|f| super::checks::part_2::min_investigation_depth_for_gk(f.width.max(f.length), d.geotechnical_category)).fold(0.0, f64::max);
    if required > 0.0 {
        series.push(Series::Limit { id: "required".into(), label: loc("Required investigation depth z_a", "Erforderliche Aufschlusstiefe z_a"), axis: AxisRef::Y, value: -required, style: stroke(if reached >= required { Tone::Info } else { Tone::Error }, Dash::Dashed) });
    }
    let flat: Vec<[f64; 2]> = curves.iter().flat_map(|(_, raw)| raw.iter().copied()).collect();
    Diagram2d {
        id: id.into(),
        title: loc(en, de),
        description: loc(&format!("{en} of every investigation point against the elevation with the layer interfaces, the reached and the required investigation depth and the characteristic value of each layer (dashed)."), &format!("{de} jedes Erkundungspunktes über der Höhe mit den Schichtgrenzen, der erreichten und der erforderlichen Aufschlusstiefe und dem charakteristischen Wert je Schicht (gestrichelt).")),
        axes: vec![axis(en, de, quantity, unit, AxisScale::Linear), axis("Elevation", "Höhe", QuantityKind::Length, "m", AxisScale::Linear)],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column(en, de, unit, flat.iter().map(|p| p[0]).collect()), column("Elevation", "Höhe", "m", flat.iter().map(|p| p[1]).collect())] },
    }
}

/// 🧱️ The soil profile diagrams q_c, N and c_u of the investigation points (GI-03, GI-06): the CPT readings, the SPT blow counts with (N1)60, and the c_u profile with the characteristic c_u of every layer that has a derivation basis or a cohesive soil.
fn ground_profiles(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    use super::checks::ground_derivation as gd;
    let d = ctx.input.document;
    if d.investigations.is_empty() || !(focus.is_none_or(|f| d.layers.iter().any(|l| l.id == f) || d.investigations.iter().any(|i| i.id == f))) {
        return Vec::new();
    }
    let of_kind = |kind: super::model::InvestigationKind| d.investigations.iter().filter(move |i| i.kind == kind);
    let mut out = Vec::new();
    let qc: Vec<(String, Vec<[f64; 2]>)> = of_kind(super::model::InvestigationKind::Cpt).map(|i| (i.id.clone(), i.readings.iter().map(|r| [r.measurement / 1e6, -r.depth]).collect())).filter(|(_, raw): &(String, Vec<[f64; 2]>)| raw.len() >= 2).collect();
    if !qc.is_empty() {
        out.push(ground_profile(ctx, "ground-profile:qc", "Cone resistance q_c profile", "Spitzendruck-q_c-Profil", "MPa", QuantityKind::Stress, qc, Vec::new()));
    }
    let mut n: Vec<(String, Vec<[f64; 2]>)> = Vec::new();
    for i in of_kind(super::model::InvestigationKind::Spt) {
        n.push((i.id.clone(), i.readings.iter().map(|r| [r.measurement, -r.depth]).collect()));
        n.push((format!("{} (N1)60", i.id), gd::n1_60_profile(d, i).iter().map(|p| [p.1, -p.0]).collect()));
    }
    n.retain(|(_, raw)| raw.len() >= 2);
    if !n.is_empty() {
        out.push(ground_profile(ctx, "ground-profile:n", "SPT blow count N profile", "Rammsondierung-N-Profil", "1", QuantityKind::Dimensionless, n, Vec::new()));
    }
    let cu: Vec<(String, Vec<[f64; 2]>)> = of_kind(super::model::InvestigationKind::Cpt).map(|i| (i.id.clone(), gd::cu_profile(d, i).iter().map(|p| [p.1 / 1000.0, -p.0]).collect())).filter(|(_, raw): &(String, Vec<[f64; 2]>)| raw.len() >= 2).collect();
    if !cu.is_empty() {
        let wanted = |l: &super::model::SoilLayer| d.characteristic_bases.iter().any(|b| b.layer_id == l.id && b.parameter == super::model::CharacteristicParameter::UndrainedStrength) || l.cohesion_undrained > 0.0;
        let characteristic = d
            .layers
            .iter()
            .filter(|l| wanted(l))
            .filter_map(|l| gd::derive_layer(d, l, super::model::CharacteristicParameter::UndrainedStrength).map(|x| (format!("{} c_u,k", l.id), vec![[x.characteristic / 1000.0, -l.depth_top], [x.characteristic / 1000.0, -l.depth_bottom]], if x.method == gd::Method::Statistical { Dash::Dashed } else { Dash::Dotted })))
            .collect();
        out.push(ground_profile(ctx, "ground-profile:cu", "Undrained shear strength c_u profile", "Undränierte-Scherfestigkeit-c_u-Profil", "kPa", QuantityKind::Stress, cu, characteristic));
    }
    out
}

/// 🧮️ The Annex D factors N_q, N_c, N_gamma over `phi'` from 1 to 45 degrees.
pub fn bearing_factor_curves() -> [Vec<[f64; 2]>; 3] {
    let phis: Vec<f64> = (1..=45).map(f64::from).collect();
    [0, 1, 2].map(|k| phis.iter().map(|p| [*p, mech::bearing_factors(*p)[k]]).collect())
}

fn factors(ctx: &Ctx) -> Diagram2d {
    let d = ctx.input.document;
    let [nq, nc, ng] = bearing_factor_curves();
    let mut markers = Vec::new();
    for f in &d.footings {
        let phi = mech::layer_at(&d.layers, f.embedment).map_or(30.0, |l| l.phi_prime_deg).clamp(1.0, 45.0);
        let n = mech::bearing_factors(phi);
        for (tag, name, value) in [("nq", "Nq", n[0]), ("nc", "Nc", n[1]), ("ng", "Ngamma", n[2])] {
            markers.push(marker(format!("{}:{tag}", f.id), [phi, value], MarkerShape::Circle, tone_of(&ctx.binding(&f.id, &["en1997.6.5.bearing"])), loc(&format!("{} {name} = {value:.1} at phi' {phi:.1} deg", f.id), &format!("{} {name} = {value:.1} bei phi' {phi:.1} Grad", f.id)), ctx.binding(&f.id, &["en1997.6.5.bearing"]).map(|b| b.check_id)));
        }
    }
    let table = DataTable { columns: vec![column("Friction angle", "Reibungswinkel", "°", nq.iter().map(|p| p[0]).collect()), column("Nq", "Nq", "1", nq.iter().map(|p| p[1]).collect()), column("Nc", "Nc", "1", nc.iter().map(|p| p[1]).collect()), column("Ngamma", "Ngamma", "1", ng.iter().map(|p| p[1]).collect())] };
    Diagram2d {
        id: "bearing:factors".into(),
        title: loc("Bearing capacity factors", "Tragfähigkeitsbeiwerte"),
        description: loc("Annex D factors Nq, Nc and Ngamma against the friction angle with the footings' operating points.", "Beiwerte Nq, Nc und Ngamma nach Anhang D über dem Reibungswinkel mit den Arbeitspunkten der Fundamente."),
        axes: vec![axis("Friction angle", "Reibungswinkel", QuantityKind::Angle, "°", AxisScale::Linear), axis("Factor", "Beiwert", QuantityKind::Dimensionless, "1", AxisScale::Log10)],
        series: vec![polyline("nq", "Nq", "Nq", &nq, Tone::Info, Dash::Solid), polyline("nc", "Nc", "Nc", &nc, Tone::Warning, Dash::Dashed), polyline("ng", "Ngamma", "Ngamma", &ng, Tone::Error, Dash::Dotted)],
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: table,
    }
}

fn load_settlement(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.footings
        .iter()
        .filter(|f| f.width > 1e-6 && f.length > 1e-6 && focus.is_none_or(|x| x == f.id))
        .map(|f| {
            let actions = mech::governing_case(f).map(mech::actions).unwrap_or(mech::Actions { v: 0.0, h: 0.0, m: 0.0 });
            let q_work = mech::base_pressure(f, &actions);
            let layer = mech::layer_at(&d.layers, f.embedment);
            let gamma_over = layer.map_or(0.0, |l| l.gamma);
            let q_resist = layer
                .and_then(|l| {
                    let wet = d.groundwater_level < f.embedment;
                    let input = BearingIn { b: f.width, l: f.length, d: f.embedment, alpha_deg: f.base_inclination_deg, v: actions.v.max(1.0), hb: actions.h.abs(), hl: 0.0, e_b: mech::ecc(&actions), e_l: 0.0, phi_deg: l.phi_prime_deg, c: l.cohesion_effective, gamma_below: if wet { l.gamma_prime } else { l.gamma }, q_over: gamma_over * f.embedment, include_depth: false };
                    bearing_drained(&input).ok().map(|o| o.r_over_a)
                })
                .filter(|q| q.is_finite() && *q > 0.0);
            let top = q_resist.map_or(2.0 * q_work, |q| q.min(4.0 * q_work.max(1.0)));
            let net = |q: f64| (q - gamma_over * f.embedment).max(0.0);
            let unit = if q_work > 1e-9 { mech::settlement_at(f, &d.layers, net(q_work), 0.0, 0.0) / net(q_work).max(1e-9) } else { 0.0 };
            let curve: Vec<[f64; 2]> = curves::sample(|q| unit * net(q) * 1000.0, 0.0, top.max(1.0), 20).iter().map(|p| [p[0] / 1000.0, p[1]]).collect();
            let s_work = unit * net(q_work) * 1000.0;
            let mut series = vec![polyline("curve", "Settlement under base pressure", "Setzung unter Sohldruck", &curve, Tone::Info, Dash::Solid)];
            series.push(Series::Limit { id: "limit".into(), label: loc("Settlement limit", "Setzungsgrenze"), axis: AxisRef::Y, value: f.settlement_limit * 1000.0, style: stroke(Tone::Error, Dash::Dashed) });
            if let Some(q) = q_resist {
                series.push(Series::Limit { id: "resistance".into(), label: loc("Bearing resistance", "Grundbruchwiderstand"), axis: AxisRef::X, value: q / 1000.0, style: stroke(Tone::Warning, Dash::Dotted) });
            }
            let status = ctx.binding(&f.id, &["en1997.6.6"]);
            let table = DataTable { columns: vec![column("Base pressure", "Sohldruck", "kPa", curve.iter().map(|p| p[0]).collect()), column("Settlement", "Setzung", "mm", curve.iter().map(|p| p[1]).collect())] };
            Diagram2d {
                id: format!("load-settlement:{}", f.id),
                title: loc(&format!("Load-settlement of {}", f.id), &format!("Last-Setzung von {}", f.id)),
                description: loc("Oedometric settlement against base pressure with the working point, the settlement limit and the bearing resistance.", "Oedometrische Setzung über dem Sohldruck mit Arbeitspunkt, Setzungsgrenze und Grundbruchwiderstand."),
                axes: vec![axis("Base pressure", "Sohldruck", QuantityKind::Pressure, "kPa", AxisScale::Linear), axis("Settlement", "Setzung", QuantityKind::Length, "mm", AxisScale::Linear)],
                series,
                markers: vec![marker(format!("{}:work", f.id), [q_work / 1000.0, s_work], MarkerShape::Circle, tone_of(&status), loc(&format!("Working point {:.0} kPa, {:.1} mm", q_work / 1000.0, s_work), &format!("Arbeitspunkt {:.0} kPa, {:.1} mm", q_work / 1000.0, s_work)), status.as_ref().map(|s| s.check_id.clone()))],
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &f.id, "footings")),
                legend: true,
                table_twin: table,
            }
        })
        .collect()
}

fn embedded_pressure(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.embedded_walls
        .iter()
        .filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6 && focus.is_none_or(|f| f == w.id))
        .map(|w| {
            let (kah, kph) = walls_and_anchors::horizontal_coefficients(w.phi_deg.max(1.0), w.delta_active_deg.max(0.0), w.delta_passive_deg.max(0.0));
            let total = w.retained_height + w.embedment;
            let depths: Vec<f64> = (0..=STEPS).map(|i| total * i as f64 / STEPS as f64).collect();
            let active: Vec<[f64; 2]> = depths.iter().map(|z| [kah * (w.backfill_gamma * z + w.surcharge) / 1000.0, w.retained_height - z]).collect();
            let passive: Vec<[f64; 2]> = depths.iter().filter(|z| **z > w.retained_height).map(|z| [-kph * w.backfill_gamma * (z - w.retained_height) / 1000.0, w.retained_height - z]).collect();
            let mut series = vec![polyline("active", "Active earth pressure", "Aktiver Erddruck", &active, Tone::Error, Dash::Solid)];
            if passive.len() >= 2 {
                series.push(polyline("passive", "Passive resistance", "Erdwiderstand", &passive, Tone::Info, Dash::Dashed));
            }
            series.push(Series::Limit { id: "dredge".into(), label: loc("Dredge level", "Aushubsohle"), axis: AxisRef::Y, value: 0.0, style: stroke(Tone::Neutral, Dash::Dotted) });
            let status = ctx.binding(&w.id, &[]);
            let markers = vec![marker(format!("{}:toe", w.id), [0.0, -w.embedment], MarkerShape::Square, tone_of(&status), loc(&format!("Toe, t = {:.2} m", w.embedment), &format!("Fußpunkt, t = {:.2} m", w.embedment)), status.as_ref().map(|s| s.check_id.clone()))];
            let table = DataTable { columns: vec![column("Elevation", "Höhe", "m", active.iter().map(|p| p[1]).collect()), column("Active pressure", "Aktiver Druck", "kPa", active.iter().map(|p| p[0]).collect())] };
            Diagram2d {
                id: format!("embedded-pressure:{}", w.id),
                title: loc(&format!("Earth pressure on {}", w.id), &format!("Erddruck auf {}", w.id)),
                description: loc(&format!("Characteristic active K_ah = {kah:.3} and passive K_ph = {kph:.2} pressure against elevation."), &format!("Charakteristischer aktiver (K_ah = {kah:.3}) und passiver (K_ph = {kph:.2}) Druck über der Höhe.")),
                axes: vec![axis("Pressure", "Druck", QuantityKind::Pressure, "kPa", AxisScale::Linear), axis("Elevation", "Höhe", QuantityKind::Length, "m", AxisScale::Linear)],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &w.id, "embeddedWalls")),
                legend: true,
                table_twin: table,
            }
        })
        .collect()
}

/// 💧 Exit gradient along the excavation surface behind the toe of an embedded wall, i(x) = ΔH/(π·√(x² + d²)), against the critical gradient γ′/γw.
fn seepage_gradient(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.embedded_walls
        .iter()
        .filter(|w| w.embedment > 1e-6 && w.retained_height - d.groundwater_level > 1e-6 && focus.is_none_or(|f| f == w.id))
        .filter_map(|w| {
            let layer = mech::layer_at(&d.layers, w.retained_height + 0.5 * w.embedment)?;
            let (head, critical) = (w.retained_height - d.groundwater_level, layer.gamma_prime / mech::GAMMA_W);
            let reach = 3.0 * w.embedment;
            let xs: Vec<f64> = (0..=STEPS).map(|i| reach * i as f64 / STEPS as f64).collect();
            let gradient: Vec<f64> = xs.iter().map(|x| super::checks::catalogue::hydraulics::exit_gradient_sheet_pile(head, w.embedment, *x)).collect();
            let raw: Vec<[f64; 2]> = xs.iter().zip(&gradient).map(|(x, i)| [*x, *i]).collect();
            let status = ctx.binding(&w.id, &["en1997.hyd-02"]);
            let series = vec![
                polyline("exit-gradient", "Exit gradient", "Austrittsgefälle", &raw, tone_of(&status), Dash::Solid),
                Series::Limit { id: "critical".into(), label: loc("Critical gradient", "Kritisches Gefälle"), axis: AxisRef::Y, value: critical, style: stroke(Tone::Error, Dash::Dashed) },
            ];
            let table = DataTable { columns: vec![column("Distance from the wall", "Abstand von der Wand", "m", xs), column("Exit gradient", "Austrittsgefälle", "1", gradient.clone()), column("Critical gradient", "Kritisches Gefälle", "1", vec![critical; gradient.len()])] };
            Some(Diagram2d {
                id: format!("seepage-gradient:{}", w.id),
                title: loc(&format!("Exit gradient at {}", w.id), &format!("Austrittsgefälle an {}", w.id)),
                description: loc(&format!("Head difference {head:.2} m over embedment {:.2} m: i(0) = {:.3} against i_crit = {critical:.3}.", w.embedment, gradient[0]), &format!("Potentialdifferenz {head:.2} m bei Einbindetiefe {:.2} m: i(0) = {:.3} gegen i_krit = {critical:.3}.", w.embedment, gradient[0])),
                axes: vec![axis("Distance", "Abstand", QuantityKind::Length, "m", AxisScale::Linear), axis("Gradient", "Gefälle", QuantityKind::Dimensionless, "1", AxisScale::Linear)],
                series,
                markers: Vec::new(),
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &w.id, "embeddedWalls")),
                legend: true,
                table_twin: table,
            })
        })
        .collect()
}

/// 💧 Hydrostatic pore pressure u(z) = γw·(z − h) below the piezometric level h of every water row, over the depth of the ground model, with the level as a marker.
fn pore_pressure_profile(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.layer_waters
        .iter()
        .filter(|w| focus.is_none_or(|f| f == w.id) && w.piezometric_depth >= 0.0)
        .map(|w| {
            let depth = ctx.layout.depth.max(w.piezometric_depth + 1.0);
            let zs: Vec<f64> = (0..=STEPS).map(|i| depth * i as f64 / STEPS as f64).collect();
            let pressure: Vec<f64> = zs.iter().map(|z| mech::GAMMA_W * (z - w.piezometric_depth).max(0.0) / 1000.0).collect();
            let raw: Vec<[f64; 2]> = zs.iter().zip(&pressure).map(|(z, u)| [*u, *z]).collect();
            let status = ctx.binding(&w.id, &[]);
            let table = DataTable { columns: vec![column("Depth", "Tiefe", "m", zs), column("Pore pressure", "Porenwasserdruck", "kPa", pressure)] };
            Diagram2d {
                id: format!("pore-pressure:{}", w.id),
                title: loc(&format!("Pore pressure of {}", w.id), &format!("Porenwasserdruck von {}", w.id)),
                description: loc(&format!("Hydrostatic pore pressure below the piezometric level at {:.2} m depth in layer {}.", w.piezometric_depth, w.layer_id), &format!("Hydrostatischer Porenwasserdruck unter dem Standrohrspiegel bei {:.2} m Tiefe in Schicht {}.", w.piezometric_depth, w.layer_id)),
                axes: vec![axis("Pore pressure", "Porenwasserdruck", QuantityKind::Pressure, "kPa", AxisScale::Linear), axis("Depth", "Tiefe", QuantityKind::Length, "m", AxisScale::Linear)],
                series: vec![polyline("pore-pressure", "Pore pressure u", "Porenwasserdruck u", &raw, tone_of(&status), Dash::Solid)],
                markers: vec![marker(format!("{}:level", w.id), [0.0, w.piezometric_depth], MarkerShape::Triangle, tone_of(&status), loc(&format!("Piezometric level {:.2} m", w.piezometric_depth), &format!("Standrohrspiegel {:.2} m", w.piezometric_depth)), status.as_ref().map(|s| s.check_id.clone()))],
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Surface, &w.id, "layerWaters")),
                legend: true,
                table_twin: table,
            }
        })
        .collect()
}

/// 📉️ Settlement trough along the line through the first footing parallel to the x axis: the superposed dish of every footing in millimetres (negative downwards) with the footing centres marked.
fn settlement_trough(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    let footings: Vec<_> = d.footings.iter().filter(|f| f.width > 1e-6 && f.length > 1e-6).collect();
    if footings.len() < 2 || focus.is_some_and(|f| !footings.iter().any(|x| x.id == f)) {
        return Vec::new();
    }
    let origin = |f: &super::model::SpreadFoundation| ctx.layout.pose(&f.id).origin;
    let y = origin(footings[0]).y;
    let reach = |f: &&super::model::SpreadFoundation| 1.5 * f.width.max(f.length);
    let lo = footings.iter().map(|f| origin(f).x - reach(f)).fold(f64::MAX, f64::min);
    let hi = footings.iter().map(|f| origin(f).x + reach(f)).fold(f64::MIN, f64::max);
    let xs: Vec<f64> = (0..=4 * STEPS).map(|i| lo + (hi - lo) * i as f64 / (4 * STEPS) as f64).collect();
    let depth = |x: f64| -> f64 { footings.iter().map(|f| super::settlement::settlement_of(ctx, f, x, y)).sum() };
    let raw: Vec<[f64; 2]> = xs.iter().map(|x| [*x, -depth(*x) * 1000.0]).collect();
    let markers = footings
        .iter()
        .map(|f| {
            let o = origin(f);
            let s = depth(o.x) * 1000.0;
            marker(format!("{}:trough", f.id), [o.x, -s], MarkerShape::Triangle, tone_of(&ctx.binding(&f.id, &["en1997.6.6"])), loc(&format!("{}: s = {s:.1} mm", f.id), &format!("{}: s = {s:.1} mm", f.id)), ctx.binding(&f.id, &["en1997.6.6"]).map(|b| b.check_id))
        })
        .collect();
    let table = DataTable { columns: vec![column("Position x", "Lage x", "m", xs), column("Settlement (downwards negative)", "Setzung (nach unten negativ)", "mm", raw.iter().map(|p| p[1]).collect())] };
    vec![Diagram2d {
        id: "settlement:trough".into(),
        title: loc("Settlement trough", "Setzungsmulde"),
        description: loc(&format!("Superposed settlement along y = {y:.1} m for {} footings; the slope between two footing marks is their angular distortion.", footings.len()), &format!("Überlagerte Setzung entlang y = {y:.1} m für {} Fundamente; die Neigung zwischen zwei Fundamentmarken ist ihre Winkelverdrehung.", footings.len())),
        axes: vec![axis("Position", "Lage", QuantityKind::Length, "m", AxisScale::Linear), axis("Settlement", "Setzung", QuantityKind::Length, "mm", AxisScale::Linear)],
        series: vec![polyline("trough", "Settlement", "Setzung", &raw, Tone::Info, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: table,
    }]
}

/// 📉️ Live settlement trough under every footing: the elastic settlement (Love's flexible rectangle on the equivalent half-space, superposed with the neighbours) and the oedometric settlement at every point of the footing axis from 1.5 times its longer side left to right, the footing edges and centre marked, the settlement limit as a line (downwards negative). A pure function of the document, so it follows every footing edit and drag of the settlement unit.
fn settlement_profiles(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    let footings: Vec<_> = d.footings.iter().filter(|f| f.width > 1e-6 && f.length > 1e-6).collect();
    footings
        .iter()
        .filter(|f| focus.is_none_or(|x| x == f.id))
        .map(|f| {
            let pose = ctx.layout.pose(&f.id);
            let (reach, (sin, cos)) = (1.5 * f.width.max(f.length), pose.yaw.sin_cos());
            let along: Vec<f64> = (0..=4 * STEPS).map(|i| -reach + 2.0 * reach * i as f64 / (4 * STEPS) as f64).collect();
            let at = |t: f64| (pose.origin.x + t * cos, pose.origin.y + t * sin);
            let total = |t: f64, elastic: bool| -> f64 {
                let (x, y) = at(t);
                footings.iter().map(|g| if elastic { super::settlement::elastic_settlement_of(ctx, g, x, y) } else { super::settlement::settlement_of(ctx, g, x, y) }).sum::<f64>() * 1000.0
            };
            let elastic: Vec<[f64; 2]> = along.iter().map(|t| [*t, -total(*t, true)]).collect();
            let oedometric: Vec<[f64; 2]> = along.iter().map(|t| [*t, -total(*t, false)]).collect();
            let status = ctx.binding(&f.id, &["en1997.6.6"]);
            let tone = tone_of(&status);
            let centre = total(0.0, true);
            let markers = vec![
                marker(format!("{}:edge-left", f.id), [-f.width / 2.0, -total(-f.width / 2.0, true)], MarkerShape::Diamond, Tone::Neutral, loc("Footing edge", "Fundamentkante"), None),
                marker(format!("{}:edge-right", f.id), [f.width / 2.0, -total(f.width / 2.0, true)], MarkerShape::Diamond, Tone::Neutral, loc("Footing edge", "Fundamentkante"), None),
                marker(format!("{}:centre", f.id), [0.0, -centre], MarkerShape::Triangle, tone, loc(&format!("{}: s,el = {centre:.1} mm", f.id), &format!("{}: s,el = {centre:.1} mm", f.id)), status.as_ref().map(|s| s.check_id.clone())),
            ];
            let mut series = vec![polyline("elastic", "Elastic settlement", "Elastische Setzung", &elastic, Tone::Info, Dash::Solid), polyline("oedometric", "Oedometric settlement", "Ödometrische Setzung", &oedometric, Tone::Warning, Dash::Dashed)];
            if f.settlement_limit > 0.0 {
                series.push(Series::Limit { id: "limit".into(), label: loc("Settlement limit", "Setzungsgrenze"), axis: AxisRef::Y, value: -f.settlement_limit * 1000.0, style: stroke(Tone::Error, Dash::Dashed) });
            }
            let table = DataTable {
                columns: vec![
                    column("Position along the footing axis", "Position entlang der Fundamentachse", "m", along.clone()),
                    column("Elastic settlement (downwards negative)", "Elastische Setzung (nach unten negativ)", "mm", elastic.iter().map(|p| p[1]).collect()),
                    column("Oedometric settlement (downwards negative)", "Ödometrische Setzung (nach unten negativ)", "mm", oedometric.iter().map(|p| p[1]).collect()),
                ],
            };
            Diagram2d {
                id: format!("settlement-profile:{}", f.id),
                title: loc(&format!("Settlement trough under {}", f.id), &format!("Setzungsmulde unter {}", f.id)),
                description: loc("Elastic and oedometric settlement along the footing axis, superposed with the neighbouring footings; the footing edges are marked.", "Elastische und ödometrische Setzung entlang der Fundamentachse, mit den Nachbarfundamenten überlagert; die Fundamentkanten sind markiert."),
                axes: vec![axis("Position along the footing axis", "Position entlang der Fundamentachse", QuantityKind::Length, "m", AxisScale::Linear), axis("Settlement", "Setzung", QuantityKind::Length, "mm", AxisScale::Linear)],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &f.id, "footings")),
                legend: true,
                table_twin: table,
            }
        })
        .collect()
}

fn lab_envelopes(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.lab_tests
        .iter()
        .filter(|t| t.kind != semio_s_artifact_norm_en1997_model::LabTestKind::Oedometer && t.points.len() >= 2 && focus.is_none_or(|f| f == t.id || f == t.layer_id))
        .map(|t| {
            let xs: Vec<f64> = t.points.iter().map(|p| p.normal_stress / 1000.0).collect();
            let ys: Vec<f64> = t.points.iter().map(|p| p.shear_stress / 1000.0).collect();
            let (a, b) = walls_and_anchors_regression(&xs, &ys);
            let top = xs.iter().copied().fold(0.0, f64::max) * 1.1;
            let line = [[0.0, a], [top, a + b * top]];
            let status = ctx.binding(&t.id, &[]);
            let mut series = vec![
                Series::Scatter { id: "points".into(), label: loc("Test results", "Versuchsergebnisse"), points: xs.iter().zip(&ys).map(|(x, y)| ScatterPoint { at: Vec2::new(*x, *y), tone: Tone::Neutral, label: None }).collect() },
                polyline("envelope", "Regression envelope", "Regressionsgerade", &line, tone_of(&status), Dash::Solid),
            ];
            if let Some(layer) = d.layers.iter().find(|l| l.id == t.layer_id) {
                let slope = layer.phi_prime_deg.to_radians().tan();
                series.push(polyline("layer", "Layer parameters", "Schichtparameter", &[[0.0, layer.cohesion_effective / 1000.0], [top, layer.cohesion_effective / 1000.0 + slope * top]], Tone::Warning, Dash::Dashed));
            }
            let bound = super::checks::catalogue::investigation::regression_lower_bound(&xs, &ys);
            let mut columns = vec![column("Stress", "Spannung", "kPa", xs.clone()), column("Strength", "Festigkeit", "kPa", ys.clone())];
            if let Some((_, _, _, _, _, a_k, b_k)) = bound {
                series.push(polyline("characteristic", "Characteristic lower bound (95 %)", "Charakteristische Untergrenze (95 %)", &[[0.0, a_k], [top, a_k + b_k * top]], Tone::Error, Dash::Dotted));
                columns.push(column("Characteristic lower bound", "Charakteristische Untergrenze", "kPa", xs.iter().map(|x| a_k + b_k * x).collect()));
            }
            let phi = b.atan().to_degrees();
            let table = DataTable { columns };
            Diagram2d {
                id: format!("lab-envelope:{}", t.id),
                title: loc(&format!("Strength envelope of {}", t.id), &format!("Festigkeitseinhüllende von {}", t.id)),
                description: loc(&format!("Regression through {} points: phi' = {phi:.1} deg, c' = {a:.1} kPa; dashed: the parameters of layer {}.", xs.len(), t.layer_id), &format!("Regression durch {} Punkte: phi' = {phi:.1} Grad, c' = {a:.1} kPa; gestrichelt: Parameter der Schicht {}.", xs.len(), t.layer_id)),
                axes: vec![axis("Normal stress", "Normalspannung", QuantityKind::Pressure, "kPa", AxisScale::Linear), axis("Shear strength", "Scherfestigkeit", QuantityKind::Pressure, "kPa", AxisScale::Linear)],
                series,
                markers: Vec::new(),
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &t.id, "labTests")),
                legend: true,
                table_twin: table,
            }
        })
        .collect()
}

fn walls_and_anchors_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    super::checks::catalogue::investigation::regression(x, y)
}

const PILE_LOADS: usize = 24;

/// 🪵️ Load-settlement curve of every pile from the catalogue model PILE-09: the head settlement (mobilisation of shaft and base on the bilinear curves plus the elastic shortening, E_c of the pile) against the head load up to 98 % of the sum of shaft and base capacity, with the capacity as a limit line and the serviceability working point.
fn pile_load_settlement(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    use super::checks::catalogue::piles::{pile_elastic_modulus_pa, resistance_settlement};
    use super::checks::part_1::{base_resistance_n, shaft_resistance_n};
    let d = ctx.input.document;
    d.piles
        .iter()
        .filter(|p| p.diameter > 1e-6 && p.length > 1e-6 && pile_elastic_modulus_pa(p) > 0.0 && focus.is_none_or(|x| x == p.id))
        .filter_map(|p| {
            let area = std::f64::consts::PI * p.diameter * p.diameter / 4.0;
            let (rs, rb, ea) = (shaft_resistance_n(p.alpha_s, p.diameter, p.unit_shaft_resistance, p.length), base_resistance_n(p.unit_base_resistance, area), pile_elastic_modulus_pa(p) * area);
            let refs = (0.01 * p.diameter, 0.10 * p.diameter);
            let settle = |n: f64| resistance_settlement(n, rs, rb, ea, p.length, refs).map(|(mobilised, elastic)| (mobilised + elastic) * 1000.0);
            let capacity = rs + rb;
            let curve: Vec<[f64; 2]> = (0..=PILE_LOADS).filter_map(|k| {
                let n = 0.98 * capacity * k as f64 / PILE_LOADS as f64;
                settle(n).map(|s| [n / 1000.0, s])
            }).collect();
            (curve.len() >= 2).then(|| {
                let working = (p.compression_permanent + p.compression_variable) / f64::from(p.count.max(1));
                let s_work = settle(working);
                let status = ctx.binding(&p.id, &["en1997.pile-09"]);
                let markers = s_work.map(|s| marker(format!("{}:work", p.id), [working / 1000.0, s], MarkerShape::Circle, tone_of(&status), loc(&format!("Working load {:.0} kN, {s:.1} mm", working / 1000.0), &format!("Arbeitslast {:.0} kN, {s:.1} mm", working / 1000.0)), status.as_ref().map(|s| s.check_id.clone()))).into_iter().collect();
                Diagram2d {
                    id: format!("pile-load-settlement:{}", p.id),
                    title: loc(&format!("Load-settlement of pile {}", p.id), &format!("Last-Setzung von Pfahl {}", p.id)),
                    description: loc("Pile head settlement against head load from the bilinear mobilisation of shaft and base resistance plus the elastic shortening, with the sum of the capacities and the serviceability working point.", "Pfahlkopfsetzung über der Kopflast aus bilinearer Mobilisierung von Mantel- und Fußwiderstand plus elastischer Stauchung, mit der Summe der Tragfähigkeiten und dem Gebrauchslastpunkt."),
                    axes: vec![axis("Head load", "Kopflast", QuantityKind::Force, "kN", AxisScale::Linear), axis("Settlement", "Setzung", QuantityKind::Length, "mm", AxisScale::Linear)],
                    series: vec![
                        polyline("curve", "Pile head settlement", "Pfahlkopfsetzung", &curve, Tone::Info, Dash::Solid),
                        Series::Limit { id: "capacity".into(), label: loc("Shaft plus base capacity", "Mantel- plus Fußwiderstand"), axis: AxisRef::X, value: capacity / 1000.0, style: stroke(Tone::Warning, Dash::Dotted) },
                    ],
                    markers,
                    annotations: Vec::new(),
                    focus: Some(entity_ref(EntityKind::Group, &p.id, "piles")),
                    legend: true,
                    table_twin: DataTable { columns: vec![column("Head load", "Kopflast", "kN", curve.iter().map(|q| q[0]).collect()), column("Settlement", "Setzung", "mm", curve.iter().map(|q| q[1]).collect())] },
                }
            })
        })
        .collect()
}

/// 🌀️ Slip-circle diagrams of every slope with a sliding mass: the Bishop forces per slice (driving W sin α, mobilised shear T, effective normal N') along the slip surface, and the factor of safety against the shifted circle centre (x and z, radius fixed) with the current centre marked; both follow the circle the scene draws, so they update live while the centre grips are dragged.
fn slip_circle(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    use super::checks::catalogue::slopes as slope_checks;
    let d = ctx.input.document;
    d.slopes
        .iter()
        .filter(|s| focus.is_none_or(|f| f == s.id) && s.height > 1e-6 && s.angle_deg > 1.0 && s.angle_deg < 89.0 && s.length > 1e-6)
        .flat_map(|s| {
            let Some((circle0, tan_phi, c)) = super::slopes::proto(d, s) else { return Vec::new() };
            let Some((circle, _)) = super::slopes::current_circle(s, &circle0, tan_phi, c) else { return Vec::new() };
            let Some(analysis) = slope_checks::bishop_forces(&circle, tan_phi, c, mech::SLICES) else { return Vec::new() };
            let status = ctx.binding(&s.id, &["en1997.11"]).or_else(|| ctx.binding(&s.id, &[]));
            let focus_ref = Some(entity_ref(EntityKind::Solid, &s.id, "slopes"));
            let kn = |v: f64| v / 1000.0;
            let xs: Vec<f64> = analysis.slices.iter().map(|k| k.slice.x).collect();
            let (driving, shear, normal): (Vec<f64>, Vec<f64>, Vec<f64>) = analysis.slices.iter().fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, k| {
                acc.0.push(kn(k.driving));
                acc.1.push(kn(k.shear));
                acc.2.push(kn(k.normal));
                acc
            });
            let along = |values: &[f64]| xs.iter().zip(values).map(|(x, v)| [*x, *v]).collect::<Vec<[f64; 2]>>();
            let forces = Diagram2d {
                id: format!("slip-forces:{}", s.id),
                title: loc(&format!("Slice forces of {}", s.id), &format!("Lamellenkräfte von {}", s.id)),
                description: loc(
                    &format!("Simplified Bishop at F = {:.2}: driving W sin(alpha), mobilised shear T and effective normal force N' per slice (characteristic values, kN per metre of section); the sums of driving and mobilised shear are equal.", analysis.factor),
                    &format!("Vereinfachtes Bishop-Verfahren bei F = {:.2}: treibendes W sin(alpha), mobilisierte Schubkraft T und effektive Normalkraft N' je Lamelle (charakteristische Werte, kN je Meter Schnitt); die Summen von treibender und mobilisierter Schubkraft sind gleich.", analysis.factor),
                ),
                axes: vec![axis("Slice position x", "Lamellenlage x", QuantityKind::Length, "m", AxisScale::Linear), axis("Force per metre", "Kraft je Meter", QuantityKind::ForcePerLength, "kN/m", AxisScale::Linear)],
                series: vec![
                    polyline("driving", "Driving W sin(alpha)", "Treibend W sin(alpha)", &along(&driving), Tone::Error, Dash::Solid),
                    polyline("shear", "Mobilised shear T", "Mobilisierte Schubkraft T", &along(&shear), tone_of(&status), Dash::Dashed),
                    polyline("normal", "Effective normal N'", "Effektive Normalkraft N'", &along(&normal), Tone::Info, Dash::Dotted),
                ],
                markers: Vec::new(),
                annotations: Vec::new(),
                focus: focus_ref.clone(),
                legend: true,
                table_twin: DataTable { columns: vec![column("Slice position x", "Lamellenlage x", "m", xs.clone()), column("Driving W sin(alpha)", "Treibend W sin(alpha)", "kN/m", driving), column("Mobilised shear T", "Mobilisierte Schubkraft T", "kN/m", shear), column("Effective normal N'", "Effektive Normalkraft N'", "kN/m", normal)] },
            };
            let reach = 0.5 * s.height.max(1.0);
            let (along_x, along_z) = slope_checks::centre_sweep(&circle, tan_phi, c, mech::SLICES, reach, 10);
            let finite = |sweep: &[(f64, Option<f64>)]| sweep.iter().filter_map(|(shift, f)| f.filter(|f| f.is_finite() && *f > 0.0).map(|f| [*shift, f])).collect::<Vec<[f64; 2]>>();
            let (fx, fz) = (finite(&along_x), finite(&along_z));
            let paired: Vec<(f64, f64, f64)> = along_x.iter().zip(&along_z).filter_map(|((shift, a), (_, b))| Some((*shift, a.filter(|f| f.is_finite())?, b.filter(|f| f.is_finite())?))).collect();
            let centre = Diagram2d {
                id: format!("slip-centre:{}", s.id),
                title: loc(&format!("Factor of safety against the circle centre of {}", s.id), &format!("Sicherheitsfaktor über dem Kreismittelpunkt von {}", s.id)),
                description: loc(
                    &format!("Bishop factor of safety when the centre ({:.1}, {:.1}) m moves along x or along z with the radius {:.1} m fixed; the marker is the current circle, F = 1 the limit state.", circle.xc, circle.yc, circle.r),
                    &format!("Bishop-Sicherheitsfaktor, wenn der Mittelpunkt ({:.1}, {:.1}) m bei festem Radius {:.1} m längs x oder z wandert; die Marke ist der aktuelle Kreis, F = 1 der Grenzzustand.", circle.xc, circle.yc, circle.r),
                ),
                axes: vec![axis("Centre shift", "Mittelpunktverschiebung", QuantityKind::Length, "m", AxisScale::Linear), axis("Factor of safety F", "Sicherheitsfaktor F", QuantityKind::Dimensionless, "1", AxisScale::Linear)],
                series: vec![
                    polyline("shift-x", "Centre shifted along x", "Mittelpunkt längs x", &fx, Tone::Info, Dash::Solid),
                    polyline("shift-z", "Centre shifted along z", "Mittelpunkt längs z", &fz, Tone::Neutral, Dash::Dashed),
                    Series::Limit { id: "limit".into(), label: loc("Limit state F = 1", "Grenzzustand F = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, Dash::Dashed) },
                ],
                markers: vec![marker(format!("{}:circle", s.id), [0.0, analysis.factor], MarkerShape::Diamond, tone_of(&status), loc(&format!("Current circle F = {:.2}", analysis.factor), &format!("Aktueller Kreis F = {:.2}", analysis.factor)), status.as_ref().map(|s| s.check_id.clone()))],
                annotations: Vec::new(),
                focus: focus_ref.clone(),
                legend: true,
                table_twin: DataTable { columns: vec![column("Centre shift", "Mittelpunktverschiebung", "m", paired.iter().map(|p| p.0).collect()), column("F along x", "F längs x", "1", paired.iter().map(|p| p.1).collect()), column("F along z", "F längs z", "1", paired.iter().map(|p| p.2).collect())] },
            };
            let spencer = slope_checks::slices(&circle, SPENCER_SLICES).and_then(|sl| {
                let pairs: Vec<(f64, f64, f64)> = (-12..=18).filter_map(|i| { let (m, f) = slope_checks::spencer_pair(&sl, tan_phi, c, 0.05 * f64::from(i))?; Some(((0.05 * f64::from(i)).atan().to_degrees(), m, f)) }).collect();
                let (factor, lambda) = slope_checks::spencer(&circle, tan_phi, c, SPENCER_SLICES)?;
                let theta = lambda.atan().to_degrees();
                Some(Diagram2d {
                    id: format!("spencer-equilibrium:{}", s.id),
                    title: loc(&format!("Spencer equilibrium of {}", s.id), &format!("Spencer-Gleichgewicht von {}", s.id)),
                    description: loc(
                        &format!("Factor from moment equilibrium F_m and from horizontal force equilibrium F_f against the constant interslice force inclination θ; they cross at θ = {theta:.1} deg and F = {factor:.2}, where both equilibria hold (θ = 0 gives the simplified Bishop and a Janbu type factor)."),
                        &format!("Sicherheitsfaktor aus Momentengleichgewicht F_m und aus horizontalem Kräftegleichgewicht F_f über der konstanten Neigung θ der Lamellenkräfte; sie schneiden sich bei θ = {theta:.1} Grad und F = {factor:.2}, dort gelten beide Gleichgewichte (θ = 0 liefert das vereinfachte Bishop-Verfahren und einen Janbu-artigen Faktor)."),
                    ),
                    axes: vec![axis("Interslice inclination θ", "Neigung θ der Lamellenkräfte", QuantityKind::Angle, "°", AxisScale::Linear), axis("Factor of safety F", "Sicherheitsfaktor F", QuantityKind::Dimensionless, "1", AxisScale::Linear)],
                    series: vec![
                        polyline("moment", "Moment equilibrium F_m", "Momentengleichgewicht F_m", &pairs.iter().map(|p| [p.0, p.1]).collect::<Vec<_>>(), Tone::Info, Dash::Solid),
                        polyline("force", "Force equilibrium F_f", "Kräftegleichgewicht F_f", &pairs.iter().map(|p| [p.0, p.2]).collect::<Vec<_>>(), Tone::Warning, Dash::Dashed),
                        Series::Limit { id: "limit".into(), label: loc("Limit state F = 1", "Grenzzustand F = 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, Dash::Dashed) },
                    ],
                    markers: vec![marker(format!("{}:spencer", s.id), [theta, factor], MarkerShape::Diamond, tone_of(&status), loc(&format!("Spencer F = {factor:.2} at {theta:.1} deg"), &format!("Spencer F = {factor:.2} bei {theta:.1} Grad")), status.as_ref().map(|s| s.check_id.clone()))],
                    annotations: Vec::new(),
                    focus: focus_ref.clone(),
                    legend: true,
                    table_twin: DataTable { columns: vec![column("Interslice inclination", "Neigung der Lamellenkräfte", "°", pairs.iter().map(|p| p.0).collect()), column("F from moments", "F aus Momenten", "1", pairs.iter().map(|p| p.1).collect()), column("F from forces", "F aus Kräften", "1", pairs.iter().map(|p| p.2).collect())] },
                })
            });
            vec![forces, centre].into_iter().chain(spencer).collect()
        })
        .collect()
}

#[path = "🗺️plans/🦀️.rs"]
mod plans;

/// 📈️ Every diagram of the document; with `focus` set to an entity id only the diagrams of that entity (a layer shows the ground profiles and factors, a footing its factors and load-settlement curve).
pub fn diagrams(document: &En1997Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1997, document.annex))
}

fn diagrams_unbased(document: &En1997Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let placements: Vec<super::layout::Placement> = document.placements.iter().map(|p| super::layout::Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    let layout = super::layout::Layout::of(document, &placements);
    let handles = Vec::new();
    let input = SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } };
    let ctx = Ctx { input: &input, layout: &layout, handles: &handles };
    let is = |ids: &mut dyn Iterator<Item = &String>| focus.is_none_or(|f| ids.any(|id| id == f));
    let ground = is(&mut document.layers.iter().map(|l| &l.id));
    let footing = is(&mut document.footings.iter().map(|f| &f.id));
    let mut out = earth_pressure(&ctx, focus);
    out.extend(embedded_pressure(&ctx, focus));
    out.extend(lab_envelopes(&ctx, focus));
    out.extend(seepage_gradient(&ctx, focus));
    out.extend(settlement_trough(&ctx, focus));
    out.extend(settlement_profiles(&ctx, focus));
    out.extend(pore_pressure_profile(&ctx, focus));
    out.extend(slip_circle(&ctx, focus));
    out.extend(plans::uplift_stack(&ctx, focus));
    out.extend(plans::anchor_layout(&ctx, focus));
    out.extend(plans::pile_group_plan(&ctx, focus));
    out.extend(plans::pile_group_raft(&ctx, focus));
    out.extend(plans::consolidation_time(&ctx, focus));
    out.extend(plans::head_profile(&ctx, focus));
    out.extend(plans::family_outcomes(&ctx, focus));
    out.extend(ground_profiles(&ctx, focus));
    if ground && !document.layers.is_empty() {
        out.push(profile(&ctx, "profile:cpt", "CPT profile", "Drucksondierprofil", "Cone resistance", "Spitzendruck", QuantityKind::Stress, "MPa", |l| l.cpt_qc / 1e6));
        out.push(profile(&ctx, "profile:spt", "SPT profile", "Rammsondierprofil", "SPT blow count", "Schlagzahl SPT", QuantityKind::Dimensionless, "1", |l| l.spt_n));
    }
    if ground || footing {
        out.push(factors(&ctx));
    }
    out.extend(load_settlement(&ctx, focus));
    out.extend(pile_load_settlement(&ctx, focus));
    out.extend(case_diagrams(document, report, focus));
    out
}

/// 🗂️ The default utilisation diagram of every design case (or of the focused case) with its table twin.
fn case_diagrams(document: &En1997Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    document
        .cases
        .iter()
        .filter(|case| focus.is_none_or(|f| f == case.id))
        .filter_map(|case| super::scene::projection_kit::design_case::case_diagram(&case.id, &loc(&case.label_en, &case.label_de), report, None))
        .collect()
}
