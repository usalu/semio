//! 📈️ Diagrams: the peak velocity pressure profile q_p(z) with the wind faces, c_pe against the loaded area, the snow shape coefficients μ₁, μ₂ and μ₃, the fire curves (standard, external,
//! hydrocarbon, parametric), the traffic influence line with the tandem axles, and the silo pressure profiles.

use super::kit::{owner_of, Owner};
use super::model::{DerailmentCase, DynamicElement, En1991Snapshot, FireMode, InternalExplosion, StructureKind};
use super::norm::CheckReport;
use super::scene::diagram_kit::{curves, Xy};
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column, stroke, axis};
use super::thermal_fire::gas_temperature_k;
use super::wind::{cpe_candidates, qp_at};
use super::traffic::{RAIL_AXLES, RAIL_AXLE_SPACING, RAIL_LEAD, SW0_GAP, SW0_LENGTH, SW2_GAP, SW2_LENGTH};
use super::{checks, silo};
use super::wave3;
use super::wave3_rows;
use checks::{part_1_2, part_1_3, part_1_4, part_1_5, part_1_6, part_1_7, part_2, part_3, part_4};

#[path = "🧷️row-curves/🦀️.rs"]
mod row_curves;

#[path = "📊️outcomes/🦀️.rs"]
mod outcomes;

const SAMPLES: usize = 48;
const SNOW_MU2_CAP: f64 = 1.6;
const PARAMETRIC_T_LIM_H: f64 = 20.0 / 60.0;

fn loc(en: &str, de: &str) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

fn p(point: Xy) -> Vec2 {
    Vec2::new(point[0], point[1])
}

fn line(id: &str, en: &str, de: &str, points: Vec<Xy>, tone: Tone, width: f64, dash: Dash) -> Series {
    Series::Polyline { id: id.to_string(), label: loc(en, de), points: points.into_iter().map(p).collect(), style: stroke(tone, width, dash) }
}

fn utilisation_tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn utilisation_of(report: &CheckReport, check_id: &str) -> Option<f64> {
    report.checks.iter().find(|c| c.id == check_id && c.utilization.is_finite()).map(|c| c.utilization.max(0.0))
}

fn marker(report: &CheckReport, id: &str, at: Xy, check_id: Option<String>, label: LocalizedCopy, binding: Option<FieldBinding>) -> Marker {
    let utilisation = check_id.as_deref().and_then(|c| utilisation_of(report, c));
    Marker { id: id.to_string(), at: p(at), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label, utilisation, bound_check: check_id, binding }
}

fn bound(path: String, unit: &str, k: f64, step: f64) -> FieldBinding {
    FieldBinding { path, map: Affine { base: 0.0, k }, unit: unit.to_string(), quantise: Some(Quantise::Round { step }) }
}

fn qp_profile(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    qp_at(d, d.height)?;
    let z_max = d.wind_faces.iter().map(|f| f.z).fold((d.height * 1.5).max(10.0), |m, z| m.max(z * 1.1));
    let curve = curves::sample(|z| qp_at(d, z).unwrap_or(0.0) / 1000.0, 0.0, z_max, SAMPLES);
    let markers = d
        .wind_faces
        .iter()
        .filter_map(|f| {
            let q = qp_at(d, f.z)? / 1000.0;
            Some(marker(report, &format!("face-{}", f.id), [f.z, q], Some(format!("en1991.1-4.wind.{}", f.id)), loc(&format!("{}: z = {:.2} m, q_p = {q:.3} kN/m²", f.id, f.z), &format!("{}: z = {:.2} m, q_p = {q:.3} kN/m²", f.id, f.z).replace('.', ",")), Some(bound(format!("windFaces[id={}].z", f.id), "m", 1.0, 0.1))))
        })
        .collect();
    Some(Diagram2d {
        id: "profile:qp".into(),
        title: loc("Peak velocity pressure profile q_p(z)", "Böengeschwindigkeitsdruck-Profil q_p(z)"),
        description: loc("Peak velocity pressure over the height above ground for the wind zone, terrain category and orography of the site (EN 1991-1-4, 4.5); the markers are the reference heights of the wind faces.", "Böengeschwindigkeitsdruck über der Höhe über Grund für Windzone, Geländekategorie und Orographie des Standorts (EN 1991-1-4, 4.5); die Marker sind die Bezugshöhen der Windflächen."),
        axes: vec![axis("Height z", "Höhe z", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, z_max))), axis("Peak velocity pressure q_p", "Böengeschwindigkeitsdruck q_p", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)],
        series: vec![
            line("qp", "q_p(z)", "q_p(z)", curve.clone(), Tone::Info, 2.5, Dash::Solid),
            Series::Limit { id: "building-height".into(), label: loc("Building height h", "Gebäudehöhe h"), axis: AxisRef::X, value: d.height, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) },
        ],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, "building", "site")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Height z", "Höhe z", "m", curve.iter().map(|q| q[0]).collect()), column("Peak velocity pressure q_p", "Böengeschwindigkeitsdruck q_p", "kN/m²", curve.iter().map(|q| q[1]).collect())] },
    })
}

fn cpe_area(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.wind_faces.is_empty() {
        return None;
    }
    let areas = curves::log_space(0.5, 20.0, SAMPLES);
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut columns = vec![column("Loaded area A", "Lasteinzugsfläche A", "m²", areas.clone())];
    for f in &d.wind_faces {
        let user: Vec<Xy> = areas.iter().map(|a| [*a, part_1_4::area_cpe(f.c_pe1, f.c_pe10, *a)]).collect();
        columns.push(column(&format!("c_pe {}", f.id), &format!("c_pe {}", f.id), "1", user.iter().map(|q| q[1]).collect()));
        series.push(line(&format!("cpe-{}", f.id), &format!("{} (c_pe,1 = {:.2}, c_pe,10 = {:.2})", f.id, f.c_pe1, f.c_pe10), &format!("{} (c_pe,1 = {:.2}, c_pe,10 = {:.2})", f.id, f.c_pe1, f.c_pe10), user, Tone::Info, 2.0, Dash::Solid));
        if let Some(&(c10, c1)) = f.zone.chars().next().and_then(|z| cpe_candidates(d, z.to_ascii_uppercase())).as_ref().and_then(|c| c.first()) {
            let table: Vec<Xy> = areas.iter().map(|a| [*a, part_1_4::area_cpe(c1, c10, *a)]).collect();
            columns.push(column(&format!("Table c_pe {}", f.id), &format!("Tabelle c_pe {}", f.id), "1", table.iter().map(|q| q[1]).collect()));
            series.push(line(&format!("table-{}", f.id), &format!("Zone {} (Table 7.1 to 7.4)", f.zone), &format!("Zone {} (Tabellen 7.1 bis 7.4)", f.zone), table, Tone::Neutral, 1.0, Dash::Dashed));
        }
        let area = f.loaded_area.clamp(0.5, 20.0);
        markers.push(marker(report, &format!("face-{}", f.id), [area, part_1_4::area_cpe(f.c_pe1, f.c_pe10, area)], Some(format!("en1991.1-4.cpe10.{}", f.id)), loc(&format!("{}: A = {:.1} m²", f.id, f.loaded_area), &format!("{}: A = {:.1} m²", f.id, f.loaded_area).replace('.', ",")), Some(bound(format!("windFaces[id={}].loadedArea", f.id), "m2", 1.0, 0.1))));
    }
    Some(Diagram2d {
        id: "cpe:area".into(),
        title: loc("External pressure coefficient c_pe over the loaded area", "Außendruckbeiwert c_pe über der Lasteinzugsfläche"),
        description: loc("c_pe(A) = c_pe,1 − (c_pe,1 − c_pe,10) log₁₀ A between 1 m² and 10 m² (EN 1991-1-4, 7.1.2); dashed: the tabulated values of the zone.", "c_pe(A) = c_pe,1 − (c_pe,1 − c_pe,10) log₁₀ A zwischen 1 m² und 10 m² (EN 1991-1-4, 7.1.2); gestrichelt: die Tabellenwerte der Zone."),
        axes: vec![axis("Loaded area A", "Lasteinzugsfläche A", QuantityKind::Area, "m²", AxisScale::Log10, Some((0.5, 20.0))), axis("External pressure coefficient c_pe", "Außendruckbeiwert c_pe", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, "building", "site")),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn snow_mu(d: &En1991Snapshot, report: &CheckReport) -> Option<Vec<Diagram2d>> {
    if d.roofs.is_empty() {
        return None;
    }
    let mu1 = curves::sample_with_breaks(part_1_3::mu_1, 0.0, 60.0, SAMPLES, &[30.0]);
    let mu2: Vec<Xy> = mu1.iter().map(|q| [q[0], part_1_3::mu_2(q[0]).min(SNOW_MU2_CAP)]).collect();
    let markers = d
        .roofs
        .iter()
        .map(|r| marker(report, &format!("roof-{}", r.id), [r.pitch_deg.clamp(0.0, 60.0), part_1_3::mu_1(r.pitch_deg)], Some(format!("en1991.1-3.snow.{}", r.id)), loc(&format!("{}: α = {:.1}°, μ₁ = {:.2}", r.id, r.pitch_deg, part_1_3::mu_1(r.pitch_deg)), &format!("{}: α = {:.1}°, μ₁ = {:.2}", r.id, r.pitch_deg, part_1_3::mu_1(r.pitch_deg)).replace('.', ",")), Some(bound(format!("roofs[id={}].pitchDeg", r.id), "deg", 1.0, 0.5))))
        .collect();
    let first = Diagram2d {
        id: "snow:mu".into(),
        title: loc("Snow load shape coefficients μ₁ and μ₂", "Schneelast-Formbeiwerte μ₁ und μ₂"),
        description: loc("μ₁ of pitched roofs and μ₂ of the valleys of multi-span roofs over the roof pitch α (EN 1991-1-3, 5.3); the markers are the roofs of the document.", "μ₁ geneigter Dächer und μ₂ der Kehlen mehrfeldriger Dächer über der Dachneigung α (EN 1991-1-3, 5.3); die Marker sind die Dächer des Dokuments."),
        axes: vec![axis("Roof pitch α", "Dachneigung α", QuantityKind::Angle, "°", AxisScale::Linear, Some((0.0, 60.0))), axis("Shape coefficient μ", "Formbeiwert μ", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)],
        series: vec![line("mu1", "μ₁(α)", "μ₁(α)", mu1.clone(), Tone::Info, 2.5, Dash::Solid), line("mu2", "μ₂(α), valley of multi-span roofs", "μ₂(α), Kehle mehrfeldriger Dächer", mu2.clone(), Tone::Neutral, 1.5, Dash::Dashed)],
        markers,
        annotations: Vec::new(),
        focus: d.roofs.first().map(|r| EntityRef::new(EntityKind::Surface, &r.id, format!("roofs[id={}]", r.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Roof pitch α", "Dachneigung α", "°", mu1.iter().map(|q| q[0]).collect()), column("μ₁", "μ₁", "1", mu1.iter().map(|q| q[1]).collect()), column("μ₂", "μ₂", "1", mu2.iter().map(|q| q[1]).collect())] },
    };
    let mu3: Vec<Xy> = curves::sample(|r| part_1_3::mu_3(r, 1.0), 0.0, 0.2, SAMPLES);
    let second = Diagram2d {
        id: "snow:mu3".into(),
        title: loc("Snow load shape coefficient μ₃ of cylindrical roofs", "Schneelast-Formbeiwert μ₃ zylindrischer Dächer"),
        description: loc("μ₃ = 0.2 + 10 h/b ≤ 2.0 over the rise ratio h/b of a cylindrical roof (EN 1991-1-3, 5.3.5).", "μ₃ = 0,2 + 10 h/b ≤ 2,0 über dem Stichverhältnis h/b eines Tonnendachs (EN 1991-1-3, 5.3.5)."),
        axes: vec![axis("Rise ratio h/b", "Stichverhältnis h/b", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 0.2))), axis("Shape coefficient μ₃", "Formbeiwert μ₃", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)],
        series: vec![line("mu3", "μ₃(h/b)", "μ₃(h/b)", mu3.clone(), Tone::Info, 2.5, Dash::Solid), Series::Limit { id: "mu3-cap".into(), label: loc("Upper limit 2.0", "Obergrenze 2,0"), axis: AxisRef::Y, value: 2.0, style: stroke(Tone::Neutral, 1.0, Dash::Dashed) }],
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: first.focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("Rise ratio h/b", "Stichverhältnis h/b", "1", mu3.iter().map(|q| q[0]).collect()), column("μ₃", "μ₃", "1", mu3.iter().map(|q| q[1]).collect())] },
    };
    Some(vec![first, second])
}

fn fire_curves(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.fire_mode == FireMode::None {
        return None;
    }
    let t_max = (d.fire_duration / 60.0 * 1.2).max(120.0);
    let times: Vec<f64> = curves::sample(|t| t, 0.0, t_max, SAMPLES).into_iter().map(|q| q[0]).collect();
    let nominal: [(&str, &str, &str, fn(f64) -> f64, super::model::part_1_2::FireCurve); 3] = [
        ("fire-standard", "Standard (ISO 834)", "Einheits-Temperaturzeitkurve (ISO 834)", part_1_2::standard_gas_temp_c, super::model::part_1_2::FireCurve::Standard),
        ("fire-external", "External fire curve", "Außenbrandkurve", part_1_2::external_gas_temp_c, super::model::part_1_2::FireCurve::External),
        ("fire-hydrocarbon", "Hydrocarbon curve", "Hydrocarbonkurve", part_1_2::hydrocarbon_gas_temp_c, super::model::part_1_2::FireCurve::Hydrocarbon),
    ];
    let mut series = Vec::new();
    let mut columns = vec![column("Time t", "Zeit t", "min", times.clone())];
    for (id, en, de, f, curve) in nominal {
        let selected = d.fire_mode == FireMode::Nominal && d.fire_curve == curve;
        let points: Vec<Xy> = times.iter().map(|t| [*t, f(*t)]).collect();
        columns.push(column(en, de, "°C", points.iter().map(|q| q[1]).collect()));
        series.push(line(id, en, de, points, if selected { Tone::Error } else { Tone::Neutral }, if selected { 2.5 } else { 1.0 }, Dash::Solid));
    }
    if let Some(qfd) = part_1_2::design_fire_load_j_m2(d.fire_load_basis, d.fire_load_density_qf, d.annex, d.fire_occupancy, d.fire_compartment_area, 1.0) {
        let opening = (d.fire_opening_factor > 1e-9).then_some(d.fire_opening_factor);
        let points: Vec<Xy> = times.iter().map(|t| [*t, part_1_2::parametric_gas_temp_k(t * 60.0, d.fire_compartment_area, d.fire_compartment_height, d.fire_thermal_inertia, qfd, opening, PARAMETRIC_T_LIM_H).0 - 273.15]).collect();
        columns.push(column("Parametric fire (Annex A)", "Parametrische Brandkurve (Anhang A)", "°C", points.iter().map(|q| q[1]).collect()));
        let selected = d.fire_mode == FireMode::Parametric;
        series.push(line("fire-parametric", "Parametric fire (Annex A)", "Parametrische Brandkurve (Anhang A)", points, if selected { Tone::Error } else { Tone::Neutral }, if selected { 2.5 } else { 1.0 }, Dash::Dashed));
    }
    let mut markers: Vec<Marker> = gas_temperature_k(d)
        .map(|theta| marker(report, "duration", [d.fire_duration / 60.0, theta - 273.15], Some("en1991.1-2.gas-temperature".into()), loc(&format!("t = {:.0} min, θ_g = {:.0} °C", d.fire_duration / 60.0, theta - 273.15), &format!("t = {:.0} min, θ_g = {:.0} °C", d.fire_duration / 60.0, theta - 273.15)), Some(bound("fireDuration".into(), "s", 60.0, 60.0))))
        .into_iter()
        .collect();
    if d.fire_mode == FireMode::Parametric {
        if let Some(qfd) = part_1_2::design_fire_load_j_m2(d.fire_load_basis, d.fire_load_density_qf, d.annex, d.fire_occupancy, d.fire_compartment_area, 1.0) {
            let opening = (d.fire_opening_factor > 1e-9).then_some(d.fire_opening_factor);
            let fire = part_1_2::parametric_gas_temp_k(d.fire_duration, d.fire_compartment_area, d.fire_compartment_height, d.fire_thermal_inertia, qfd, opening, PARAMETRIC_T_LIM_H).1;
            let (minutes, peak) = (fire.t_max_h * 60.0, fire.theta_max_c);
            markers.push(marker(
                report,
                "theta-max",
                [minutes, peak],
                Some("en1991.1-2.gas-temperature".into()),
                loc(&format!("θ_max = {peak:.0} °C at t_max = {minutes:.0} min (O = {:.4} m½, b = {:.0} J/m²s½K, q_t,d = {:.0} MJ/m²)", fire.o, d.fire_thermal_inertia, fire.q_td_mj), &format!("θ_max = {peak:.0} °C bei t_max = {minutes:.0} min (O = {:.4} m½, b = {:.0} J/m²s½K, q_t,d = {:.0} MJ/m²)", fire.o, d.fire_thermal_inertia, fire.q_td_mj).replace('.', ",")),
                None,
            ));
        }
    }
    Some(Diagram2d {
        id: "fire:curves".into(),
        title: loc("Temperature-time curves of the fire", "Temperatur-Zeit-Kurven des Brandes"),
        description: loc("Standard, external and hydrocarbon curves (EN 1991-1-2, 3.2) and the parametric curve of the compartment (Annex A); the selected curve is highlighted, the marker is the design duration, the second marker the maximum of the parametric curve. The window grip of the compartment edits the opening factor O, the wall grip the thermal inertia b; q_f,d follows the chosen fire load basis.", "Einheits-, Außenbrand- und Hydrocarbonkurve (EN 1991-1-2, 3.2) und die parametrische Kurve des Brandabschnitts (Anhang A); die gewählte Kurve ist hervorgehoben, der Marker ist die Bemessungsdauer, der zweite Marker das Maximum der parametrischen Kurve. Der Fenstergriff des Brandabschnitts ändert den Öffnungsfaktor O, der Wandgriff die Wärmeträgheit b; q_f,d folgt der gewählten Brandlastgrundlage."),
        axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "min", AxisScale::Linear, Some((0.0, t_max))), axis("Gas temperature θ_g", "Gastemperatur θ_g", QuantityKind::Temperature, "°C", AxisScale::Linear, None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, "fire", "site")),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn influence(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.structure_kind != StructureKind::Bridge {
        return None;
    }
    let span = d.bridge_span.max(1.0);
    let eta = |x: f64| x.min(span - x).max(0.0) / 2.0;
    let line_points: Vec<Xy> = curves::sample_with_breaks(eta, 0.0, span, SAMPLES, &[span / 2.0]);
    let axles = [d.bridge_tandem_x - 0.6, d.bridge_tandem_x + 0.6];
    let lane = d.bridge_lane.max(1);
    let markers: Vec<Marker> = axles.iter().enumerate().map(|(i, x)| marker(report, &format!("axle-{i}"), [*x, eta(*x)], Some("en1991.2.lm1-tandem".into()), loc(&format!("Axle {}: x = {x:.2} m, η = {:.3} m", i + 1, eta(*x)), &format!("Achse {}: x = {x:.2} m, η = {:.3} m", i + 1, eta(*x)).replace('.', ",")), None)).collect();
    let moment: f64 = axles.iter().map(|x| part_2::lm1_tandem_n(d.annex, lane) * eta(*x)).sum();
    Some(Diagram2d {
        id: "bridge:influence".into(),
        title: loc("Mid-span moment influence line of the bridge", "Momenteneinflusslinie in Feldmitte der Brücke"),
        description: loc("Influence line η of the mid-span bending moment of a simply supported span with the two axles of the LM1 tandem (EN 1991-2, 4.3.2).", "Einflusslinie η des Feldmoments eines Einfeldträgers mit den beiden Achsen des LM1-Tandems (EN 1991-2, 4.3.2)."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, span))), axis("Influence ordinate η", "Einflussordinate η", QuantityKind::Length, "m", AxisScale::Linear, None)],
        series: vec![line("influence-moment", "η_M(x) at x = L/2", "η_M(x) bei x = L/2", line_points.clone(), Tone::Info, 2.5, Dash::Solid)],
        markers,
        annotations: vec![DiagramNote { id: "tandem-moment".into(), at: Vec2::new(span / 2.0, span / 8.0), text: loc(&format!("M = Q_ak Σ η = {:.0} kNm", moment / 1000.0), &format!("M = Q_ak Σ η = {:.0} kNm", moment / 1000.0)), tone: Tone::Info }, DiagramNote { id: "tandem-moment-max".into(), at: Vec2::new(span / 2.0, span / 16.0), text: loc(&format!("M_max (both axles, worst position) = {:.0} kNm", part_2::lm1_tandem_moment_nm(d.annex, lane, span) / 1000.0), &format!("M_max (beide Achsen, ungünstigste Stellung) = {:.0} kNm", part_2::lm1_tandem_moment_nm(d.annex, lane, span) / 1000.0)), tone: Tone::Info }],
        focus: Some(EntityRef::new(EntityKind::Solid, "bridge", "site")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", line_points.iter().map(|q| q[0]).collect()), column("Influence ordinate η_M", "Einflussordinate η_M", "m", line_points.iter().map(|q| q[1]).collect())] },
    })
}

/// 🚆 Mid-span moment influence line of the railway bridge with the LM71 axle group and the moments of the assumed LM71, SW/0 and SW/2 loads (EN 1991-2, 6.3).
fn rail_influence(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.structure_kind != StructureKind::Bridge {
        return None;
    }
    let span = d.bridge_span.max(1.0);
    let mid = span / 2.0;
    let eta = |x: f64| x.min(span - x).max(0.0) / 2.0;
    let curve: Vec<Xy> = curves::sample_with_breaks(eta, 0.0, span, SAMPLES, &[mid]);
    let axles: Vec<f64> = (0..RAIL_AXLES).map(|j| mid + (j as f64 - (RAIL_AXLES as f64 - 1.0) / 2.0) * RAIL_AXLE_SPACING).collect();
    let markers: Vec<Marker> = axles.iter().enumerate().map(|(j, x)| marker(report, &format!("axle-{j}"), [*x, eta(*x)], Some("en1991.2.rail.lm71-axle".into()), loc(&format!("LM71 axle {}: x = {x:.2} m, η = {:.3} m", j + 1, eta(*x)), &format!("LM71-Achse {}: x = {x:.2} m, η = {:.3} m", j + 1, eta(*x)).replace('.', ",")), None)).collect();
    let area = |a: f64, b: f64| {
        let (a, b) = (a.clamp(0.0, mid), b.clamp(0.0, mid));
        if b <= a { 0.0 } else { span / 4.0 * (b - a) - (b * b - a * a) / 4.0 }
    };
    let group = (RAIL_AXLES as f64 - 1.0) / 2.0 * RAIL_AXLE_SPACING + RAIL_LEAD;
    let lm71 = d.assumed_rail_lm71 * axles.iter().map(|x| eta(*x)).sum::<f64>() + d.assumed_rail_lm71_udl * 2.0 * (area(0.0, mid) - area(0.0, group));
    let sw = |q: f64, length: f64, gap: f64| q * 2.0 * area(gap / 2.0, gap / 2.0 + length);
    let (sw0, sw2) = (sw(d.assumed_rail_sw0, SW0_LENGTH, SW0_GAP), sw(d.assumed_rail_sw2, SW2_LENGTH, SW2_GAP));
    let note = |id: &str, at: f64, en: String, de: String| DiagramNote { id: id.into(), at: Vec2::new(mid, at), text: loc(&en, &de), tone: Tone::Info };
    Some(Diagram2d {
        id: "rail:influence".into(),
        title: loc("Rail mid-span moment influence line", "Momenteneinflusslinie in Feldmitte der Eisenbahnbrücke"),
        description: loc("Influence line η of the mid-span bending moment of a simply supported span with the four LM71 axles and the moments of the assumed LM71, SW/0 and SW/2 loads (EN 1991-2, 6.3; values unverified).", "Einflusslinie η des Feldmoments eines Einfeldträgers mit den vier LM71-Achsen und den Momenten der angesetzten Lasten LM71, SW/0 und SW/2 (EN 1991-2, 6.3; Werte ungeprüft)."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, span))), axis("Influence ordinate η", "Einflussordinate η", QuantityKind::Length, "m", AxisScale::Linear, None)],
        series: vec![line("influence-moment", "η_M(x) at x = L/2", "η_M(x) bei x = L/2", curve.clone(), Tone::Info, 2.5, Dash::Solid)],
        markers,
        annotations: vec![
            note("lm71-moment", span / 8.0, format!("M_LM71 = {:.0} kNm", lm71 / 1000.0), format!("M_LM71 = {:.0} kNm", lm71 / 1000.0)),
            note("sw0-moment", span / 10.0, format!("M_SW/0 = {:.0} kNm", sw0 / 1000.0), format!("M_SW/0 = {:.0} kNm", sw0 / 1000.0)),
            note("sw2-moment", span / 16.0, format!("M_SW/2 = {:.0} kNm", sw2 / 1000.0), format!("M_SW/2 = {:.0} kNm", sw2 / 1000.0)),
        ],
        focus: Some(EntityRef::new(EntityKind::Solid, "bridge", "site")),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", curve.iter().map(|q| q[0]).collect()), column("Influence ordinate η_M", "Einflussordinate η_M", "m", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🛣️ LM1 per lane: the tandem axle load Q_ik of the lane number of every `bridgeLanes` row against the assumed tandem load (EN 1991-2, Table 4.2).
fn lane_loads(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.structure_kind != StructureKind::Bridge || d.bridge_lanes.is_empty() {
        return None;
    }
    let mut rows: Vec<(f64, f64)> = d.bridge_lanes.iter().map(|l| (f64::from(l.lm1_lane), part_2::lm1_tandem_n(d.annex, l.lm1_lane) / 1000.0)).collect();
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    let highest = rows.iter().map(|r| r.0).fold(1.0, f64::max);
    let assumed = d.assumed_bridge_tandem / 1000.0;
    let markers: Vec<Marker> = d
        .bridge_lanes
        .iter()
        .map(|l| {
            let q = part_2::lm1_tandem_n(d.annex, l.lm1_lane) / 1000.0;
            marker(report, &format!("lane-{}", l.id), [f64::from(l.lm1_lane), q], Some(format!("en1991.2.lm1-lane.{}", l.id)), loc(&format!("{}: lane number {}, Q_ik = {q:.0} kN", l.id, l.lm1_lane), &format!("{}: Fahrstreifennummer {}, Q_ik = {q:.0} kN", l.id, l.lm1_lane)), Some(bound(format!("bridgeLanes[id={}].lm1Lane", l.id), "1", 1.0, 1.0)))
        })
        .collect();
    Some(Diagram2d {
        id: "bridge:lane-lm1".into(),
        title: loc("LM1 tandem load per lane", "LM1-Tandemlast je Fahrstreifen"),
        description: loc("Tandem axle load Q_ik of the LM1 lane numbers of the bridge lanes against the assumed tandem load of the document (EN 1991-2, 4.3.2, Table 4.2).", "Tandemachslast Q_ik der LM1-Fahrstreifennummern der Brückenfahrstreifen gegen die angesetzte Tandemlast des Dokuments (EN 1991-2, 4.3.2, Tabelle 4.2)."),
        axes: vec![axis("LM1 lane number", "LM1-Fahrstreifennummer", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, highest + 0.5))), axis("Tandem axle load Q_ik", "Tandemachslast Q_ik", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![
            line("required", "Q_ik of the lane number", "Q_ik der Fahrstreifennummer", rows.iter().map(|r| [r.0, r.1]).collect(), Tone::Info, 2.0, Dash::Solid),
            line("assumed", "Assumed tandem load", "Angesetzte Tandemlast", vec![[0.5, assumed], [highest + 0.5, assumed]], Tone::Warning, 1.5, Dash::Dashed),
        ],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, "bridge", "site")),
        legend: true,
        table_twin: DataTable { columns: vec![column("LM1 lane number", "LM1-Fahrstreifennummer", "1", rows.iter().map(|r| r.0).collect()), column("Required tandem axle load Q_ik", "Erforderliche Tandemachslast Q_ik", "kN", rows.iter().map(|r| r.1).collect())] },
    })
}

/// ⏱️ Construction stage timeline: the return period assumed for the climatic actions of every stage against the one the duration requires (EN 1991-1-6, Table 3.1, unverified).
fn stage_timeline(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.construction_stages.is_empty() {
        return None;
    }
    let mut start = 0.0;
    let mut assumed: Vec<Xy> = Vec::new();
    let mut required: Vec<Xy> = Vec::new();
    let mut markers: Vec<Marker> = Vec::new();
    for stage in &d.construction_stages {
        let end = start + stage.duration_days.max(0.0);
        let need = part_1_6::stage_return_period_years(d.annex, stage.duration_days);
        assumed.extend([[start, stage.return_years], [end, stage.return_years]]);
        if let Some(years) = need {
            required.extend([[start, years], [end, years]]);
        }
        let (need_en, need_de) = need.map_or(("awaiting values".to_string(), "Werte ausstehend".to_string()), |years| (format!("{years:.0} a"), format!("{years:.0} a")));
        markers.push(marker(report, &format!("stage-{}", stage.id), [(start + end) / 2.0, stage.return_years], Some(format!("en1991.1-6.stage-return-period.{}", stage.id)), loc(&format!("{}: {:.0} d, T = {:.1} a (required {need_en})", stage.id, stage.duration_days, stage.return_years), &format!("{}: {:.0} d, T = {:.1} a (erforderlich {need_de})", stage.id, stage.duration_days, stage.return_years)), Some(bound(format!("constructionStages[id={}].returnYears", stage.id), "a", 1.0, 0.5))));
        start = end;
    }
    let with_required = required.len() == assumed.len();
    let mut series = vec![line("assumed", "Assumed return period", "Angesetzte Wiederkehrperiode", assumed.clone(), Tone::Info, 2.0, Dash::Solid)];
    let mut columns = vec![column("Time", "Zeit", "d", assumed.iter().map(|q| q[0]).collect()), column("Assumed return period T", "Angesetzte Wiederkehrperiode T", "a", assumed.iter().map(|q| q[1]).collect())];
    if with_required {
        series.push(line("required", "Required return period", "Erforderliche Wiederkehrperiode", required.clone(), Tone::Warning, 1.5, Dash::Dashed));
        columns.push(column("Required return period T", "Erforderliche Wiederkehrperiode T", "a", required.iter().map(|q| q[1]).collect()));
    }
    Some(Diagram2d {
        id: "construction:stage-timeline".into(),
        title: loc("Construction stage timeline", "Zeitleiste der Bauzustände"),
        description: loc("Return period of the climatic actions assumed for each construction stage against the return period its duration requires (EN 1991-1-6, 3.3, Table 3.1; recommended values, unverified).", "Für jeden Bauzustand angesetzte Wiederkehrperiode der klimatischen Einwirkungen gegen die Wiederkehrperiode, die seine Dauer erfordert (EN 1991-1-6, 3.3, Tabelle 3.1; empfohlene Werte, ungeprüft)."),
        axes: vec![axis("Time", "Zeit", QuantityKind::Time, "d", AxisScale::Linear, Some((0.0, start.max(1.0)))), axis("Return period T", "Wiederkehrperiode T", QuantityKind::Time, "a", AxisScale::Linear, None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: d.construction_stages.first().map(|s| EntityRef::new(EntityKind::Load, &s.id, format!("constructionStages[id={}]", s.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn silo_pressure(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if !d.silo_claimed || !matches!(d.silo_kind.as_str(), "silo" | "tank") {
        return None;
    }
    let tank = d.silo_kind == "tank";
    let height = d.silo_height.max(0.5);
    let kpa = |f: &dyn Fn(f64) -> f64| curves::sample(|z| f(z) / 1000.0, 0.0, height, SAMPLES);
    let horizontal = kpa(&|z| silo::horizontal_pa(d, z));
    let mut series = vec![line("p-h", if tank { "Hydrostatic pressure" } else { "Horizontal pressure p_h" }, if tank { "Hydrostatischer Druck" } else { "Horizontaldruck p_h" }, horizontal.clone(), Tone::Info, 2.5, Dash::Solid)];
    let mut columns = vec![column("Depth z", "Tiefe z", "m", horizontal.iter().map(|q| q[0]).collect()), column("Horizontal pressure p_h", "Horizontaldruck p_h", "kN/m²", horizontal.iter().map(|q| q[1]).collect())];
    if !tank {
        let friction = kpa(&|z| part_4::wall_friction_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, z));
        let vertical = kpa(&|z| part_4::janssen_vertical_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, z));
        let patch = kpa(&|z| part_4::patch_pressure_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, z, d.annex));
        let discharge = kpa(&|z| part_4::discharge_horizontal_max_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, z));
        columns.push(column("Discharge pressure p_he", "Entleerungsdruck p_he", "kN/m²", discharge.iter().map(|q| q[1]).collect()));
        series.push(line("p-he", "Discharge pressure p_he", "Entleerungsdruck p_he", discharge, Tone::Warning, 1.5, Dash::Dashed));
        for (id, en, de, points, dash) in [("p-w", "Wall friction p_w", "Wandreibung p_w", friction, Dash::Solid), ("p-v", "Vertical pressure p_v", "Vertikaldruck p_v", vertical, Dash::Dotted), ("p-patch", "Patch load p_patch", "Patchlast p_patch", patch, Dash::Dashed)] {
            columns.push(column(en, de, "kN/m²", points.iter().map(|q| q[1]).collect()));
            series.push(line(id, en, de, points, Tone::Neutral, 1.5, dash));
        }
    }
    let at = horizontal.last().copied().unwrap_or([height, 0.0]);
    let check = if tank { "en1991.4.tank" } else { "en1991.4.silo-pressure" };
    Some(Diagram2d {
        id: "silo:pressure".into(),
        title: loc("Silo and tank pressure profiles", "Silo- und Tank-Druckprofile"),
        description: loc("Janssen horizontal pressure, wall friction, vertical pressure and patch load of a silo, or the hydrostatic pressure of a tank, over the depth below the top (EN 1991-4, 5.2 and 7).", "Janssen-Horizontaldruck, Wandreibung, Vertikaldruck und Patchlast eines Silos beziehungsweise hydrostatischer Druck eines Tanks über der Tiefe unter der Oberkante (EN 1991-4, 5.2 und 7)."),
        axes: vec![axis("Depth z below the top", "Tiefe z unter Oberkante", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, height))), axis("Pressure", "Druck", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)],
        series,
        markers: vec![marker(report, "height", at, Some(check.into()), loc(&format!("H = {height:.1} m, p_h = {:.1} kN/m²", at[1]), &format!("H = {height:.1} m, p_h = {:.1} kN/m²", at[1]).replace('.', ",")), Some(bound("siloHeight".into(), "m", 1.0, 0.1)))],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Shell, "silo", "site")),
        legend: true,
        table_twin: DataTable { columns },
    })
}

const PSI_ACTIONS: [(&str, &str, &str); 2] = [("snow", "Snow (H ≤ 1000 m)", "Schnee (H ≤ 1000 m)"), ("wind", "Wind", "Wind")];

/// 🎛️ The combination chart: ψ₀, ψ₁ and ψ₂ of the imposed-load categories of the floors and of snow and wind, which scale the actions of the combinations of EN 1990 (6.4.3); floor markers bind to the imposed-load check of their floor.
fn psi_chart(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let mut rows: Vec<(String, LocalizedCopy, f64, f64, f64, Option<String>)> = Vec::new();
    for floor in &d.floors {
        let letter = floor.category.trim().chars().next().map(|c| c.to_ascii_uppercase().to_string()).unwrap_or_default();
        if letter.is_empty() || rows.iter().any(|r| r.0 == letter) {
            continue;
        }
        let (psi0, psi1, psi2) = checks::part_1_1::psi_factors(&letter, d.annex);
        rows.push((letter.clone(), loc(&format!("Category {letter}"), &format!("Kategorie {letter}")), psi0, psi1, psi2, Some(format!("en1991.1-1.imposed.{}", floor.id))));
    }
    rows.extend(PSI_ACTIONS.iter().map(|(key, en, de)| {
        let (p0, p1, p2) = checks::part_1_1::psi_factors_of_key(key, d.annex);
        (key.to_string(), loc(en, de), p0, p1, p2, None)
    }));
    if rows.len() < 2 {
        return None;
    }
    let index = |i: usize| (i + 1) as f64;
    let factor = |name: &str, tone: Tone, pick: fn(&(String, LocalizedCopy, f64, f64, f64, Option<String>)) -> f64| Series::Scatter { id: name.to_string(), label: loc(&format!("ψ{}", &name[3..]), &format!("ψ{}", &name[3..])), points: rows.iter().enumerate().map(|(i, r)| ScatterPoint { at: Vec2::new(index(i), pick(r)), tone, label: Some(loc(&format!("{}: ψ{} = {:.1}", r.1.en, &name[3..], pick(r)), &format!("{}: ψ{} = {}", r.1.de, &name[3..], format!("{:.1}", pick(r)).replace('.', ",")))) }).collect() };
    let series = vec![factor("psi0", Tone::Info, |r| r.2), factor("psi1", Tone::Warning, |r| r.3), factor("psi2", Tone::Neutral, |r| r.4)];
    let markers = rows.iter().enumerate().filter_map(|(i, r)| r.5.as_ref().map(|check| marker(report, &format!("category-{}", r.0), [index(i), r.2], Some(check.clone()), loc(&format!("{}: ψ0 = {:.1}", r.1.en, r.2), &format!("{}: ψ0 = {}", r.1.de, format!("{:.1}", r.2).replace('.', ","))), None))).collect();
    let notes = rows.iter().enumerate().map(|(i, r)| DiagramNote { id: format!("category-note-{}", r.0), at: Vec2::new(index(i), -0.06), text: r.1.clone(), tone: Tone::Neutral }).collect();
    let columns = vec![column("Category index", "Kategorieindex", "1", (0..rows.len()).map(index).collect()), column("ψ0", "ψ0", "1", rows.iter().map(|r| r.2).collect()), column("ψ1", "ψ1", "1", rows.iter().map(|r| r.3).collect()), column("ψ2", "ψ2", "1", rows.iter().map(|r| r.4).collect())];
    Some(Diagram2d {
        id: "psi:factors".into(),
        title: loc("Combination factors ψ", "Kombinationsbeiwerte ψ"),
        description: loc("Combination factors ψ0 (combination value), ψ1 (frequent value) and ψ2 (quasi-permanent value) of the imposed-load categories of the floors and of snow and wind (EN 1990, Table A1.1); they scale the variable actions of the combinations (EN 1990, 6.4.3) and the permanent-duration share of the load.", "Kombinationsbeiwerte ψ0 (Kombinationswert), ψ1 (häufiger Wert) und ψ2 (quasi-ständiger Wert) der Nutzlastkategorien der Decken sowie von Schnee und Wind (EN 1990, Tabelle A1.1); sie skalieren die veränderlichen Einwirkungen der Kombinationen (EN 1990, 6.4.3) und deren ständigen Lastanteil."),
        axes: vec![axis("Category", "Kategorie", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, rows.len() as f64 + 0.5))), axis("Factor ψ", "Beiwert ψ", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((-0.1, 1.1)))],
        series,
        markers,
        annotations: notes,
        focus: None,
        legend: true,
        table_twin: DataTable { columns },
    })
}


/// 🌉 Horizontal wind force per metre of a bridge deck against the height above ground (EN 1991-1-4, 8.3.2): the profile q_p(z) c_f,x d_tot of the first case with the required and the assumed force of every case.
fn bridge_wind_force(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.wind_bridges.iter().filter_map(|r| wave3::wind_face(r).map(|(_, _, depth_tot)| (r, depth_tot))).collect();
    let (first, depth_tot) = *rows.first()?;
    qp_at(d, first.height_above_ground)?;
    let z_max = rows.iter().map(|r| r.0.height_above_ground).fold(10.0, f64::max) * 1.5;
    let c_fx = part_1_4::bridge_c_fx0(first.deck_width, depth_tot);
    let curve = curves::sample(|z| qp_at(d, z.max(1.0)).unwrap_or(0.0) * c_fx * depth_tot / 1000.0, 0.0, z_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for (row, depth_tot) in &rows {
        let Some(qp) = qp_at(d, row.height_above_ground) else { continue };
        let required = part_1_4::bridge_force_x_n_m(qp, part_1_4::bridge_c_fx0(row.deck_width, *depth_tot), *depth_tot) / 1000.0;
        let check = format!("en1991.1-4.bridge-wind.{}", row.id);
        markers.push(marker(report, &format!("required-{}", row.id), [row.height_above_ground, required], Some(check), loc(&format!("{}: F_W,x = {required:.2} kN/m", row.id), &format!("{}: F_W,x = {required:.2} kN/m", row.id)), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.height_above_ground, row.assumed_force / 1000.0], Some(check), loc(&format!("{}: assumed {:.2} kN/m", row.id, row.assumed_force / 1000.0), &format!("{}: angenommen {:.2} kN/m", row.id, row.assumed_force / 1000.0)), Some(bound(format!("windBridges[id={}].assumedForce", row.id), "N", 1000.0, 100.0))));
    }
    Some(Diagram2d { id: "bridge:wind-force".into(),
        title: loc("Wind force on the bridge deck", "Windkraft auf den Brückenüberbau"), description: loc("Horizontal wind force per metre F_W,x = q_p(z) c_f,x d_tot against the height above ground; c_f,x0 and the depth increments are unverified registry values (EN 1991-1-4, 8.3).", "Horizontale Windkraft je Meter F_W,x = q_p(z) c_f,x d_tot über der Höhe über Grund; c_f,x0 und die Höhenzuschläge sind ungeprüfte Registerwerte (EN 1991-1-4, 8.3)."),
        axes: vec![axis("Height above ground z_e", "Höhe über Grund z_e", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, z_max))), axis("Wind force F_W,x", "Windkraft F_W,x", QuantityKind::ForcePerLength, "kN/m", AxisScale::Linear, None)], series: vec![line("profile", "F_W,x(z) of the first case", "F_W,x(z) des ersten Falls", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Surface, &first.id, format!("windBridges[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Height above ground", "Höhe über Grund", "m", curve.iter().map(|q| q[0]).collect()), column("Wind force F_W,x", "Windkraft F_W,x", "kN/m", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 〰️ Structural factor c_s c_d against the element height (EN 1991-1-4, Annex B) with the required and the assumed value of every slender element.
fn dynamic_cscd(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let first = d.dynamic_elements.iter().find(|r| wave3::dynamic_values(d, r).is_some())?;
    let (vb, terrain) = (part_1_4::design_vb(d.annex, d.wind_zone, d.en_vb)?, part_1_4::TerrainCategory::from_u8(d.terrain_category)?);
    let h_max = d.dynamic_elements.iter().map(|r| r.height).fold(20.0, f64::max) * 1.5;
    let curve = curves::sample(|h| if part_1_4::cscd_simplified(h) { 1.0 } else { part_1_4::structural_factor(h, first.width, terrain, vb, first.log_decrement, if first.first_frequency > 0.0 { first.first_frequency } else { 46.0 / h })[8] }, 5.0, h_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &d.dynamic_elements {
        let Some((required, ..)) = wave3::dynamic_values(d, row) else { continue };
        markers.push(marker(report, &format!("required-{}", row.id), [row.height, required], Some(format!("en1991.1-4.dyn-cscd.{}", row.id)), loc(&format!("{}: required c_s c_d = {required:.3}", row.id), &format!("{}: erforderliches c_s c_d = {required:.3}", row.id)), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.height, row.assumed_cscd], Some(format!("en1991.1-4.dyn-cscd.{}", row.id)), loc(&format!("{}: assumed c_s c_d = {:.3}", row.id, row.assumed_cscd), &format!("{}: angenommenes c_s c_d = {:.3}", row.id, row.assumed_cscd)), Some(bound(format!("dynamicElements[id={}].assumedCscd", row.id), "1", 1.0, 0.01))));
    }
    Some(Diagram2d { id: "dynamic:cscd".into(),
        title: loc("Structural factor c_s c_d", "Strukturbeiwert c_s c_d"), description: loc("c_s c_d of the first slender element over the height h (Annex B, n₁ = 46/h unless given) with the required and the assumed value of every element; below the registry height the procedure is skipped and c_s c_d = 1.", "c_s c_d des ersten schlanken Bauteils über der Höhe h (Anhang B, n₁ = 46/h, sofern nicht angegeben) mit dem erforderlichen und dem angenommenen Wert jedes Bauteils; unter der Registerhöhe entfällt das Verfahren, c_s c_d = 1."),
        axes: vec![axis("Height h", "Höhe h", QuantityKind::Length, "m", AxisScale::Linear, Some((5.0, h_max))), axis("Structural factor c_s c_d", "Strukturbeiwert c_s c_d", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)], series: vec![line("cscd", "c_s c_d(h) of the first element", "c_s c_d(h) des ersten Bauteils", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &first.id, format!("dynamicElements[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Height h", "Höhe h", "m", curve.iter().map(|q| q[0]).collect()), column("Structural factor c_s c_d", "Strukturbeiwert c_s c_d", "1", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🌀 Structural factor against the first natural frequency (EN 1991-1-4, Annex B): c_s c_d of the first slender element at its height over n₁, with the required and the assumed value of every element at its own frequency.
fn dynamic_cscd_frequency(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.dynamic_elements.iter().filter_map(|r| wave3::dynamic_values(d, r).map(|v| (r, v))).collect();
    let (first, _) = *rows.first()?;
    let (vb, terrain) = (part_1_4::design_vb(d.annex, d.wind_zone, d.en_vb)?, part_1_4::TerrainCategory::from_u8(d.terrain_category)?);
    let n_of = |r: &DynamicElement| if r.first_frequency > 0.0 { r.first_frequency } else { 46.0 / r.height };
    let n_max = rows.iter().map(|(r, _)| n_of(r)).fold(1.0, f64::max) * 2.0;
    let simplified = part_1_4::cscd_simplified(first.height);
    let curve = curves::sample(|n| if simplified { 1.0 } else { part_1_4::structural_factor(first.height, first.width, terrain, vb, first.log_decrement, n.max(0.01))[8] }, 0.05, n_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for (row, (required, ..)) in &rows {
        let n = n_of(row);
        markers.push(marker(report, &format!("required-{}", row.id), [n, *required], Some(format!("en1991.1-4.dyn-cscd.{}", row.id)), loc(&format!("{}: n₁ = {n:.2} Hz, required c_s c_d = {required:.3}", row.id), &format!("{}: n₁ = {n:.2} Hz, erforderliches c_s c_d = {required:.3}", row.id)), Some(bound(format!("dynamicElements[id={}].firstFrequency", row.id), "Hz", 1.0, 0.05))));
        markers.push(marker(report, &format!("assumed-{}", row.id), [n, row.assumed_cscd], Some(format!("en1991.1-4.dyn-cscd.{}", row.id)), loc(&format!("{}: assumed c_s c_d = {:.3}", row.id, row.assumed_cscd), &format!("{}: angenommenes c_s c_d = {:.3}", row.id, row.assumed_cscd)), Some(bound(format!("dynamicElements[id={}].assumedCscd", row.id), "1", 1.0, 0.01))));
    }
    Some(Diagram2d { id: "dynamic:cscd-frequency".into(),
        title: loc("Structural factor c_s c_d over the frequency", "Strukturbeiwert c_s c_d über der Eigenfrequenz"), description: loc("c_s c_d of the first slender element at its height over the first natural frequency n₁ (Annex B); the markers are the required and the assumed value of every element at its own n₁ (46/h unless given); below the registry height the procedure is skipped and c_s c_d = 1.", "c_s c_d des ersten schlanken Bauteils in seiner Höhe über der ersten Eigenfrequenz n₁ (Anhang B); die Marker sind der erforderliche und der angenommene Wert jedes Bauteils bei seinem eigenen n₁ (46/h, sofern nicht angegeben); unter der Registerhöhe entfällt das Verfahren, c_s c_d = 1."),
        axes: vec![axis("Natural frequency n₁", "Eigenfrequenz n₁", QuantityKind::Dimensionless, "Hz", AxisScale::Linear, Some((0.05, n_max))), axis("Structural factor c_s c_d", "Strukturbeiwert c_s c_d", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)], series: vec![line("cscd-frequency", "c_s c_d(n₁) of the first element", "c_s c_d(n₁) des ersten Bauteils", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &first.id, format!("dynamicElements[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Natural frequency n₁", "Eigenfrequenz n₁", "Hz", curve.iter().map(|q| q[0]).collect()), column("Structural factor c_s c_d", "Strukturbeiwert c_s c_d", "1", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🌪️ Critical velocity against the natural frequency (EN 1991-1-4, Annex E): v_crit = b n₁/St of the first element against the onset velocity 1.25 v_m of every element.
fn vortex_velocity(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.dynamic_elements.iter().filter_map(|r| wave3::dynamic_values(d, r).map(|v| (r, v))).collect();
    let (first, _) = *rows.first()?;
    let n_of = |r: &DynamicElement| if r.first_frequency > 0.0 { r.first_frequency } else { 46.0 / r.height };
    let n_max = rows.iter().map(|(r, _)| n_of(r)).fold(1.0, f64::max) * 2.0;
    let critical = curves::sample(|n| part_1_4::vortex_critical_velocity(first.width, n, first.strouhal), 0.0, n_max, SAMPLES);
    let mut onset: Vec<Xy> = Vec::new();
    let mut markers: Vec<Marker> = Vec::new();
    for (row, (_, v_crit, v_m)) in &rows {
        let n = n_of(row);
        onset.extend([[0.0, part_1_4::vortex_onset_velocity(*v_m)], [n_max, part_1_4::vortex_onset_velocity(*v_m)]]);
        markers.push(marker(report, &format!("critical-{}", row.id), [n, *v_crit], Some(format!("en1991.1-4.vortex-shedding.{}", row.id)), loc(&format!("{}: n₁ = {n:.2} Hz, v_crit = {v_crit:.1} m/s", row.id), &format!("{}: n₁ = {n:.2} Hz, v_crit = {v_crit:.1} m/s", row.id)), Some(bound(format!("dynamicElements[id={}].firstFrequency", row.id), "Hz", 1.0, 0.05))));
    }
    Some(Diagram2d { id: "vortex:velocity".into(),
        title: loc("Vortex shedding onset", "Beginn der Wirbelerregung"), description: loc("Critical velocity v_crit = b n₁/St over the first natural frequency against the onset velocity 1.25 v_m: vortex shedding is relevant where the line lies below the onset (EN 1991-1-4, E.1.3.1).", "Kritische Geschwindigkeit v_crit = b n₁/St über der ersten Eigenfrequenz gegen die Einsatzgeschwindigkeit 1,25 v_m: Wirbelerregung ist relevant, wo die Linie unter der Einsatzgeschwindigkeit liegt (EN 1991-1-4, E.1.3.1)."),
        axes: vec![axis("Natural frequency n₁", "Eigenfrequenz n₁", QuantityKind::Dimensionless, "Hz", AxisScale::Linear, Some((0.0, n_max))), axis("Velocity", "Geschwindigkeit", QuantityKind::Velocity, "m/s", AxisScale::Linear, None)], series: vec![line("critical", "v_crit(n₁) of the first element", "v_crit(n₁) des ersten Bauteils", critical.clone(), Tone::Info, 2.0, Dash::Solid), line("onset", "1.25 v_m", "1,25 v_m", onset, Tone::Warning, 1.5, Dash::Dashed)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &first.id, format!("dynamicElements[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Natural frequency n₁", "Eigenfrequenz n₁", "Hz", critical.iter().map(|q| q[0]).collect()), column("Critical velocity v_crit", "Kritische Geschwindigkeit v_crit", "m/s", critical.iter().map(|q| q[1]).collect())] },
    })
}

/// 💥 Impact force F = v √(k m) against the speed (EN 1991-1-7, Annex C) for the first vehicle or ship, with the required and the assumed force of every case.
fn impact_force(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.impact_actions.iter().filter(|r| r.kind <= 1 && r.mass > 0.0 && r.speed > 0.0 && (r.kind == 0 || r.stiffness > 0.0)).collect();
    let first = *rows.first()?;
    let v_max = rows.iter().map(|r| r.speed).fold(10.0, f64::max) * 1.5;
    let stiffness = part_1_7::impact_stiffness_n_m(first.stiffness);
    let curve = curves::sample(|v| part_1_7::dynamic_impact_force_n(first.mass, v, stiffness) / 1000.0, 0.0, v_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        let required = part_1_7::dynamic_impact_force_n(row.mass, row.speed, part_1_7::impact_stiffness_n_m(row.stiffness)) / 1000.0;
        let check = format!("en1991.1-7.impact-force.{}", row.id);
        markers.push(marker(report, &format!("required-{}", row.id), [row.speed, required], Some(check), loc(&format!("{}: F_d,x = {required:.0} kN", row.id), &format!("{}: F_d,x = {required:.0} kN", row.id)), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.speed, row.assumed_force / 1000.0], Some(check), loc(&format!("{}: assumed {:.0} kN", row.id, row.assumed_force / 1000.0), &format!("{}: angenommen {:.0} kN", row.id, row.assumed_force / 1000.0)), Some(bound(format!("impactActions[id={}].assumedForce", row.id), "N", 1000.0, 10.0))));
    }
    Some(Diagram2d { id: "impact:force".into(),
        title: loc("Impact force against speed", "Anprallkraft über der Geschwindigkeit"), description: loc("Equivalent static impact force F_d,x = v √(k m) of the first vehicle or ship over the impact speed, with the required and the assumed force of every case (EN 1991-1-7, Annex C).", "Statische Ersatzkraft F_d,x = v √(k m) des ersten Fahrzeugs oder Schiffs über der Anprallgeschwindigkeit, mit der erforderlichen und der angenommenen Kraft jedes Falls (EN 1991-1-7, Anhang C)."),
        axes: vec![axis("Impact speed v", "Anprallgeschwindigkeit v", QuantityKind::Velocity, "m/s", AxisScale::Linear, Some((0.0, v_max))), axis("Impact force F_d,x", "Anprallkraft F_d,x", QuantityKind::Force, "kN", AxisScale::Linear, None)], series: vec![line("force", "F_d,x(v) of the first case", "F_d,x(v) des ersten Falls", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &first.id, format!("impactActions[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Impact speed", "Anprallgeschwindigkeit", "m/s", curve.iter().map(|q| q[0]).collect()), column("Impact force F_d,x", "Anprallkraft F_d,x", "kN", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🏠 Internal explosion design pressure against the vent ratio A_v/V (EN 1991-1-7, Annex D): the D.1 and D.2 curves of the first room and the required and assumed pressure of every room.
fn gas_explosion(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.internal_explosions.iter().filter(|r| r.volume > 0.0 && r.vent_area > 0.0).collect();
    let first = *rows.first()?;
    let ratio_of = |r: &InternalExplosion| r.vent_area / r.volume;
    let x_max = rows.iter().map(|r| ratio_of(r)).fold(0.05, f64::max) * 1.5;
    let x_min = rows.iter().map(|r| ratio_of(r)).fold(x_max, f64::min) / 2.0;
    let p_v = first.vent_pressure / 1000.0;
    let d1 = vec![[x_min, part_1_7::gas_explosion_pd_static_kn_m2(p_v)], [x_max, part_1_7::gas_explosion_pd_static_kn_m2(p_v)]];
    let d2 = curves::sample(|r| part_1_7::gas_explosion_pd_kn_m2(p_v, r), x_min, x_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        let required = part_1_7::gas_explosion_governing_kn_m2(row.vent_pressure / 1000.0, ratio_of(row));
        markers.push(marker(report, &format!("required-{}", row.id), [ratio_of(row), required], Some(format!("en1991.1-7.gas-explosion.{}", row.id)), loc(&format!("{}: p_d = {required:.1} kN/m²", row.id), &format!("{}: p_d = {required:.1} kN/m²", row.id)), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [ratio_of(row), row.assumed_pressure / 1000.0], Some(format!("en1991.1-7.gas-explosion.{}", row.id)), loc(&format!("{}: assumed {:.1} kN/m²", row.id, row.assumed_pressure / 1000.0), &format!("{}: angenommen {:.1} kN/m²", row.id, row.assumed_pressure / 1000.0)), Some(bound(format!("internalExplosions[id={}].assumedPressure", row.id), "Pa", 1000.0, 100.0))));
    }
    Some(Diagram2d { id: "gas-explosion:pressure".into(),
        title: loc("Internal explosion design pressure", "Bemessungsdruck der inneren Explosion"), description: loc("Design pressure p_d of Annex D.1 (constant) and D.2 (vent ratio) of the first room over A_v/V, with the required and the assumed pressure of every room; the larger formula governs (selection rule unverified).", "Bemessungsdruck p_d nach Anhang D.1 (konstant) und D.2 (Entlastungsverhältnis) des ersten Raums über A_v/V, mit dem erforderlichen und dem angenommenen Druck jedes Raums; die größere Formel ist maßgebend (Auswahlregel ungeprüft)."),
        axes: vec![axis("Vent ratio A_v/V", "Entlastungsverhältnis A_v/V", QuantityKind::Dimensionless, "1/m", AxisScale::Linear, Some((x_min, x_max))), axis("Design pressure p_d", "Bemessungsdruck p_d", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], series: vec![line("d1", "D.1: 3 + p_v", "D.1: 3 + p_v", d1, Tone::Warning, 1.5, Dash::Dashed), line("d2", "D.2: 3 + p_v/2 + 0.04/(A_v/V)²", "D.2: 3 + p_v/2 + 0,04/(A_v/V)²", d2.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &first.id, format!("internalExplosions[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Vent ratio A_v/V", "Entlastungsverhältnis A_v/V", "1/m", d2.iter().map(|q| q[0]).collect()), column("Design pressure D.2", "Bemessungsdruck D.2", "kN/m²", d2.iter().map(|q| q[1]).collect())] },
    })
}

/// 🌡️ Linear temperature difference over the section depth (EN 1991-1-5, 6.1.4): the heating and cooling profiles of the first bridge section with its bound range check.
fn section_profile(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.thermal_sections.iter().filter(|r| r.depth > 0.0).filter_map(|r| part_1_5::bridge_delta_t_m(r.deck_type, r.surfacing).map(|v| (r, v))).collect();
    let (first, (heat, cool)) = *rows.first()?;
    let heating = vec![[-heat / 2.0, 0.0], [heat / 2.0, first.depth]];
    let cooling = vec![[cool / 2.0, 0.0], [-cool / 2.0, first.depth]];
    let markers: Vec<Marker> = rows.iter().map(|(r, (h, _))| marker(report, &format!("section-{}", r.id), [h / 2.0, r.depth], Some(format!("en1991.1-5.section-range.{}", r.id)), loc(&format!("{}: ΔT_M,heat = {h:.1} K, h = {:.2} m", r.id, r.depth), &format!("{}: ΔT_M,heat = {h:.1} K, h = {:.2} m", r.id, r.depth)), Some(bound(format!("thermalSections[id={}].depth", r.id), "m", 1.0, 0.05)))).collect();
    Some(Diagram2d { id: "section:temperature-profile".into(),
        title: loc("Linear temperature difference over the section", "Linearer Temperaturunterschied über den Querschnitt"), description: loc("Heating and cooling profile ΔT_M of the first bridge section over its depth (EN 1991-1-5, Table 6.1 base values times k_sur, unverified).", "Erwärmungs- und Abkühlungsprofil ΔT_M des ersten Brückenquerschnitts über seine Höhe (EN 1991-1-5, Grundwerte der Tabelle 6.1 mal k_sur, ungeprüft)."),
        axes: vec![axis("Temperature difference", "Temperaturunterschied", QuantityKind::Temperature, "K", AxisScale::Linear, None), axis("Height in the section", "Höhe im Querschnitt", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, first.depth)))], series: vec![line("heating", "Heating ΔT_M,heat", "Erwärmung ΔT_M,heat", heating.clone(), Tone::Error, 2.0, Dash::Solid), line("cooling", "Cooling ΔT_M,cool", "Abkühlung ΔT_M,cool", cooling, Tone::Info, 2.0, Dash::Solid)],
        markers, annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &first.id, format!("thermalSections[id={}]", first.id))), legend: true, table_twin: DataTable { columns: vec![column("Height in the section", "Höhe im Querschnitt", "m", heating.iter().map(|q| q[1]).collect()), column("Heating ΔT_M", "Erwärmung ΔT_M", "K", heating.iter().map(|q| q[0]).collect())] },
    })
}

/// 🪢 Horizontal tie force against the span (EN 1991-1-7, Annex A): the peripheral T_p = 0.4 (g + ψ q) s L and the internal T_i = 0.8 (g + ψ q) s L, each capped at 75 kN, with the required and the provided force of every tie run.
fn tie_forces(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.tie_runs.iter().filter(|r| r.span > 0.0 && r.spacing > 0.0).collect();
    let first = *rows.first()?;
    let l_max = rows.iter().map(|r| r.span).fold(1.0, f64::max) * 1.5;
    let force = |factor: f64| move |l: f64| part_1_7::tie_force_kn(factor, first.dead_load / 1000.0, first.imposed_load / 1000.0, first.psi, first.spacing, l).1;
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        let factor = if row.kind == 0 { 0.4 } else { 0.8 };
        let (_, required) = part_1_7::tie_force_kn(factor, row.dead_load / 1000.0, row.imposed_load / 1000.0, row.psi, row.spacing, row.span);
        markers.push(marker(report, &format!("required-{}", row.id), [row.span, required], Some(format!("en1991.1-7.tie-force.{}", row.id)), loc(&format!("{}: required {required:.1} kN", row.id), &format!("{}: erforderlich {required:.1} kN", row.id)), None));
        markers.push(marker(report, &format!("provided-{}", row.id), [row.span, row.provided_force / 1000.0], Some(format!("en1991.1-7.tie-force.{}", row.id)), loc(&format!("{}: provided {:.1} kN", row.id, row.provided_force / 1000.0), &format!("{}: vorhanden {:.1} kN", row.id, row.provided_force / 1000.0)), Some(bound(format!("tieRuns[id={}].providedForce", row.id), "N", 1000.0, 500.0))));
    }
    let peripheral = curves::sample(force(0.4), 0.0, l_max, SAMPLES);
    let internal = curves::sample(force(0.8), 0.0, l_max, SAMPLES);
    Some(Diagram2d {
        id: "tie:forces".into(),
        title: loc("Horizontal tie force against span", "Horizontale Zugkraft über der Spannweite"),
        description: loc("Peripheral and internal tie force T = factor (g + \u{3c8} q) s L of the first tie run over the span L, limited to 75 kN, with the required and the provided force of every tie run (EN 1991-1-7, Annex A).", "Rand- und Innenzugkraft T = Faktor (g + \u{3c8} q) s L des ersten Zugankers \u{fc}ber der Spannweite L, begrenzt auf 75 kN, mit erforderlicher und vorhandener Kraft jedes Zugankers (EN 1991-1-7, Anhang A)."),
        axes: vec![axis("Span L", "Spannweite L", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, l_max))), axis("Tie force T", "Zugkraft T", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![line("peripheral", "Peripheral tie T_p", "Randzugkraft T_p", peripheral.clone(), Tone::Info, 2.0, Dash::Solid), line("internal", "Internal tie T_i", "Innenzugkraft T_i", internal.clone(), Tone::Warning, 2.0, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &first.id, format!("tieRuns[id={}]", first.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Span L", "Spannweite L", "m", peripheral.iter().map(|q| q[0]).collect()), column("Peripheral tie T_p", "Randzugkraft T_p", "kN", peripheral.iter().map(|q| q[1]).collect()), column("Internal tie T_i", "Innenzugkraft T_i", "kN", internal.iter().map(|q| q[1]).collect())] },
    })
}

/// 🚁 Helicopter hard-landing force F_d = C \u{221a}m against the helicopter mass (EN 1991-1-7, Annex E.5, C = 3000 N/\u{221a}kg as in the check), with the assumed force of every landing pad.
fn helicopter_landing(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.helicopter_landings.iter().filter(|r| r.mass > 0.0).collect();
    let first = *rows.first()?;
    let m_max = rows.iter().map(|r| r.mass).fold(500.0, f64::max) * 1.5;
    let curve = curves::sample(|m| 3.0 * m.sqrt(), 0.0, m_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        markers.push(marker(report, &format!("required-{}", row.id), [row.mass, 3.0 * row.mass.sqrt()], Some(format!("en1991.1-7.helicopter-force.{}", row.id)), loc(&format!("{}: F_d = {:.1} kN", row.id, 3.0 * row.mass.sqrt()), &format!("{}: F_d = {:.1} kN", row.id, 3.0 * row.mass.sqrt())), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.mass, row.assumed_force / 1000.0], Some(format!("en1991.1-7.helicopter-force.{}", row.id)), loc(&format!("{}: assumed {:.1} kN", row.id, row.assumed_force / 1000.0), &format!("{}: angenommen {:.1} kN", row.id, row.assumed_force / 1000.0)), Some(bound(format!("helicopterLandings[id={}].assumedForce", row.id), "N", 1000.0, 500.0))));
    }
    Some(Diagram2d {
        id: "helicopter:landing-force".into(),
        title: loc("Helicopter hard landing force", "Hubschrauber-Hartlandungskraft"),
        description: loc("Equivalent static hard-landing force F_d = C \u{221a}m over the helicopter mass, with the required and the assumed force of every landing pad (EN 1991-1-7, Annex E).", "\u{c4}quivalente statische Hartlandungskraft F_d = C \u{221a}m \u{fc}ber der Hubschraubermasse, mit erforderlicher und angenommener Kraft jedes Landeplatzes (EN 1991-1-7, Anhang E)."),
        axes: vec![axis("Helicopter mass m", "Hubschraubermasse m", QuantityKind::Mass, "kg", AxisScale::Linear, Some((0.0, m_max))), axis("Landing force F_d", "Landekraft F_d", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![line("force", "F_d = C \u{221a}m", "F_d = C \u{221a}m", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &first.id, format!("helicopterLandings[id={}]", first.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Helicopter mass m", "Hubschraubermasse m", "kg", curve.iter().map(|q| q[0]).collect()), column("Landing force F_d", "Landekraft F_d", "kN", curve.iter().map(|q| q[1]).collect())] },
    })
}

fn dec(text: String) -> String {
    text.replace('.', ",")
}

struct Pair {
    id: String,
    name: String,
    required: f64,
    assumed: f64,
    check: String,
    path: String,
    unit: &'static str,
    k: f64,
    step: f64,
}

/// 📊 A required-against-assumed comparison over the numbered rows of a collection: the required value of every row, the value the document assumes (bound to its field, so the marker drags it) and the check that judges it.
fn compare(report: &CheckReport, id: &str, title: LocalizedCopy, description: LocalizedCopy, axes: [Axis; 2], pairs: &[Pair], focus: EntityRef) -> Option<Diagram2d> {
    if pairs.is_empty() {
        return None;
    }
    let at = |i: usize| (i + 1) as f64;
    let y_unit = axes[1].unit.clone();
    let required: Vec<Xy> = pairs.iter().enumerate().map(|(i, r)| [at(i), r.required]).collect();
    let assumed: Vec<Xy> = pairs.iter().enumerate().map(|(i, r)| [at(i), r.assumed]).collect();
    let mut markers: Vec<Marker> = Vec::new();
    for (i, r) in pairs.iter().enumerate() {
        markers.push(marker(report, &format!("required-{}", r.id), [at(i), r.required], Some(r.check.clone()), loc(&format!("{}: required {:.2} {y_unit}", r.name, r.required), &dec(format!("{}: erforderlich {:.2} {y_unit}", r.name, r.required))), None));
        markers.push(marker(report, &format!("assumed-{}", r.id), [at(i), r.assumed], Some(r.check.clone()), loc(&format!("{}: assumed {:.2} {y_unit}", r.name, r.assumed), &dec(format!("{}: angenommen {:.2} {y_unit}", r.name, r.assumed))), Some(bound(r.path.clone(), r.unit, r.k, r.step))));
    }
    let columns = vec![column("Row number", "Zeilennummer", "1", (0..pairs.len()).map(at).collect()), column("Required", "Erforderlich", &y_unit, required.iter().map(|q| q[1]).collect()), column("Assumed", "Angenommen", &y_unit, assumed.iter().map(|q| q[1]).collect())];
    Some(Diagram2d {
        id: id.into(),
        title,
        description,
        axes: axes.into(),
        series: vec![line("required", "Required", "Erforderlich", required, Tone::Info, 2.0, Dash::Solid), line("assumed", "Assumed", "Angenommen", assumed, Tone::Warning, 1.5, Dash::Dashed)],
        markers,
        annotations: pairs.iter().enumerate().map(|(i, r)| DiagramNote { id: format!("row-{}", r.id), at: Vec2::new(at(i), 0.0), text: loc(&r.name, &r.name), tone: Tone::Neutral }).collect(),
        focus: Some(focus),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn row_axis(en: &str, de: &str, rows: usize) -> Axis {
    axis(en, de, QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.5, rows as f64 + 0.5)))
}

/// 🌬️ Wind pressure plan: the net pressure q_p (c_pe(A) − c_pi) c_s c_d of every wind face and zone at its reference height against the assumed pressure (EN 1991-1-4, 5.2 and 7.2).
fn wind_face_pressure(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .wind_faces
        .iter()
        .filter_map(|f| {
            let ze = if f.zone.trim().eq_ignore_ascii_case("D") { part_1_4::reference_height_ze(f.z, d.height, d.width) } else { d.height };
            let qp = qp_at(d, ze)?;
            let required = part_1_4::governing_wind_pressure_pa(qp, part_1_4::area_cpe(f.c_pe1, f.c_pe10, f.loaded_area), f.c_pi, f.c_s, f.c_d) / 1000.0;
            Some(Pair { id: f.id.clone(), name: format!("{} ({})", f.id, f.zone), required, assumed: f.assumed_wp / 1000.0, check: format!("en1991.1-4.wind.{}", f.id), path: format!("windFaces[id={}].assumedWp", f.id), unit: "Pa", k: 1000.0, step: 10.0 })
        })
        .collect();
    let first = d.wind_faces.first()?;
    compare(report, "wind:face-pressure", loc("Wind pressure per face and zone", "Winddruck je Fläche und Zone"), loc("Net wind pressure of every face from q_p at the reference height, c_pe(A), c_pi, c_s and c_d against the pressure the document assumes; the marker drags the assumed pressure (EN 1991-1-4, 5.2 and 7.2).", "Netto-Winddruck jeder Fläche aus q_p in der Bezugshöhe, c_pe(A), c_pi, c_s und c_d gegen den im Dokument angenommenen Druck; der Marker verschiebt den angenommenen Druck (EN 1991-1-4, 5.2 und 7.2)."), [row_axis("Wind face number", "Windflächennummer", pairs.len()), axis("Wind pressure w", "Winddruck w", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Surface, &first.id, format!("windFaces[id={}]", first.id)))
}

/// ❄️ Roof snow plan: the snow load μ C_e C_t s_k of every roof with its shape coefficient (monopitch, duopitch, parapet, drift, multi-span) against the assumed snow load (EN 1991-1-3, 5.2).
fn roof_snow_loads(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let sk = part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk)?;
    let pairs: Vec<Pair> = d
        .roofs
        .iter()
        .map(|r| {
            let mu = part_1_3::shape_mu(&r.roof_type, r.pitch_deg, r.has_parapet, r.parapet_height, r.drift_obstruction_height, r.multi_span, sk, d.annex);
            Pair { id: r.id.clone(), name: format!("{} (μ = {mu:.2})", r.id), required: part_1_3::roof_snow_pa(sk, mu, r.c_e, r.c_t) / 1000.0, assumed: r.assumed_sk / 1000.0, check: format!("en1991.1-3.snow.{}", r.id), path: format!("roofs[id={}].assumedSk", r.id), unit: "Pa", k: 1000.0, step: 10.0 }
        })
        .collect();
    let first = d.roofs.first()?;
    compare(report, "roof:snow-loads", loc("Snow load per roof", "Schneelast je Dach"), loc("Snow load on every roof from its governing shape coefficient (pitch, parapet, drift obstruction, multi-span valley), C_e, C_t and the ground snow load against the load the document assumes (EN 1991-1-3, 5.2).", "Schneelast auf jedem Dach aus dem maßgebenden Formbeiwert (Neigung, Attika, Verwehung, Kehle mehrfeldriger Dächer), C_e, C_t und der Bodenschneelast gegen die im Dokument angenommene Last (EN 1991-1-3, 5.2)."), [row_axis("Roof number", "Dachnummer", pairs.len()), axis("Snow load s", "Schneelast s", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Surface, &first.id, format!("roofs[id={}]", first.id)))
}

/// 🧱 Self-weight breakdown: the characteristic self-weight γ t of every element from its material and thickness against the assumed g_k (EN 1991-1-1, Annex A).
fn self_weight_breakdown(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .self_weight_elements
        .iter()
        .filter_map(|e| {
            let required = checks::part_1_1::self_weight_pa(&e.material, e.thickness, d.annex)? / 1000.0;
            Some(Pair { id: e.id.clone(), name: format!("{}, t = {:.2} m", e.id, e.thickness), required, assumed: e.assumed_gk / 1000.0, check: format!("en1991.1-1.self-weight.{}", e.id), path: format!("selfWeightElements[id={}].assumedGk", e.id), unit: "Pa", k: 1000.0, step: 10.0 })
        })
        .collect();
    let first = d.self_weight_elements.first()?;
    compare(report, "self-weight:breakdown", loc("Self-weight per element", "Eigengewicht je Bauteil"), loc("Characteristic self-weight g_k = γ t of every element from the unit weight of its material and its thickness against the assumed value (EN 1991-1-1, Annex A).", "Charakteristisches Eigengewicht g_k = γ t jedes Bauteils aus der Wichte seines Baustoffs und seiner Dicke gegen den angenommenen Wert (EN 1991-1-1, Anhang A)."), [row_axis("Element number", "Bauteilnummer", pairs.len()), axis("Self-weight g_k", "Eigengewicht g_k", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Surface, &first.id, format!("selfWeightElements[id={}]", first.id)))
}

/// 🏗️ Construction stage loads: the combined load ψ₀ Σ Q_c,i of the construction loads of every stage against the assumed stage load (EN 1991-1-6, Annex A.2 and 4.11).
fn stage_loads(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .construction_stages
        .iter()
        .filter(|s| d.construction_loads.iter().any(|l| l.stage == s.id))
        .map(|s| Pair {
            id: s.id.clone(),
            name: s.id.clone(),
            required: part_1_6::combined_construction_load_pa(d.construction_loads.iter().filter(|l| l.stage == s.id).map(|l| l.magnitude)) / 1000.0,
            assumed: s.assumed_qk / 1000.0,
            check: format!("en1991.1-6.stage-combination.{}", s.id),
            path: format!("constructionStages[id={}].assumedQk", s.id),
            unit: "Pa",
            k: 1000.0,
            step: 10.0,
        })
        .collect();
    let first = pairs.first()?;
    let focus = EntityRef::new(EntityKind::Load, &first.id, format!("constructionStages[id={}]", first.id));
    compare(report, "construction:stage-loads", loc("Construction load per stage", "Bauzustandslast je Bauzustand"), loc("Combined construction load Q_c = ψ₀ Σ Q_c,i of the loads of every stage against the stage load the document assumes (EN 1991-1-6, Annex A.2).", "Kombinierte Bauzustandslast Q_c = ψ₀ Σ Q_c,i der Lasten jedes Bauzustands gegen die im Dokument angenommene Bauzustandslast (EN 1991-1-6, Anhang A.2)."), [row_axis("Stage number", "Bauzustandsnummer", pairs.len()), axis("Construction load Q_c", "Bauzustandslast Q_c", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], &pairs, focus)
}

/// 💥 Accidental impact rows: the vehicle impact force F = v √(k m) of every nested impact row of every accidental case against its assumed force (EN 1991-1-7, Annex C).
fn accidental_impacts(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .accidental_cases
        .iter()
        .flat_map(|c| {
            c.impact.iter().enumerate().map(move |(j, i)| Pair {
                id: format!("{}-{j}", c.id),
                name: format!("{}[{j}]", c.id),
                required: part_1_7::vehicle_impact_force_n(i.vehicle_mass, i.vehicle_speed) / 1000.0,
                assumed: i.assumed_force / 1000.0,
                check: format!("en1991.1-7.impact.{}.{j}", c.id),
                path: format!("accidentalCases[id={}].impact[{j}].assumedForce", c.id),
                unit: "N",
                k: 1000.0,
                step: 10.0,
            })
        })
        .collect();
    let first = d.accidental_cases.iter().find(|c| !c.impact.is_empty())?;
    compare(report, "accidental:impacts", loc("Accidental vehicle impact per case", "Außergewöhnlicher Fahrzeuganprall je Fall"), loc("Equivalent static vehicle impact force v √(k m) of every impact row of the accidental cases against the assumed force (EN 1991-1-7, Annex C).", "Statische Ersatzkraft des Fahrzeuganpralls v √(k m) jeder Anprallzeile der außergewöhnlichen Fälle gegen die angenommene Kraft (EN 1991-1-7, Anhang C)."), [row_axis("Impact row number", "Anprallzeilennummer", pairs.len()), axis("Impact force F", "Anprallkraft F", QuantityKind::Force, "kN", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Load, &first.id, format!("accidentalCases[id={}]", first.id)))
}

/// 💣 Accidental explosion rows: the blast overpressure at the stand-off of every nested explosion row of every accidental case against its assumed pressure (EN 1991-1-7, Annex D).
fn accidental_explosions(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .accidental_cases
        .iter()
        .flat_map(|c| {
            c.explosion.iter().enumerate().map(move |(j, e)| Pair {
                id: format!("{}-{j}", c.id),
                name: format!("{}[{j}]", c.id),
                required: part_1_7::explosion_pressure_pa(e.explosion_mass, e.standoff) / 1000.0,
                assumed: e.assumed_pressure / 1000.0,
                check: format!("en1991.1-7.explosion.{}.{j}", c.id),
                path: format!("accidentalCases[id={}].explosion[{j}].assumedPressure", c.id),
                unit: "Pa",
                k: 1000.0,
                step: 100.0,
            })
        })
        .collect();
    let first = d.accidental_cases.iter().find(|c| !c.explosion.is_empty())?;
    compare(report, "accidental:explosions", loc("Accidental explosion per case", "Außergewöhnliche Explosion je Fall"), loc("Incident blast overpressure of the charge at its stand-off for every explosion row of the accidental cases against the assumed pressure (EN 1991-1-7, Annex D).", "Einfallender Explosionsüberdruck der Ladung im Abstand für jede Explosionszeile der außergewöhnlichen Fälle gegen den angenommenen Druck (EN 1991-1-7, Anhang D)."), [row_axis("Explosion row number", "Explosionszeilennummer", pairs.len()), axis("Overpressure p", "Überdruck p", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Load, &first.id, format!("accidentalCases[id={}]", first.id)))
}

/// ⏱️ Impact force-time pulse of the first vehicle impact: the idealised triangular pulse with the peak F_d,x = v √(k m) whose impulse equals the momentum m v (duration 2 m v / F), after EN 1991-1-7 Annex C.
fn impact_pulse(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let row = d.impact_actions.iter().find(|r| r.kind == 0 && r.mass > 0.0 && r.speed > 0.0)?;
    let peak = part_1_7::dynamic_impact_force_n(row.mass, row.speed, part_1_7::impact_stiffness_n_m(row.stiffness)) / 1000.0;
    let duration = 2.0 * row.mass * row.speed / (peak * 1000.0);
    let pulse = vec![[0.0, 0.0], [duration / 2.0, peak], [duration, 0.0]];
    let markers = vec![marker(report, &format!("peak-{}", row.id), [duration / 2.0, peak], Some(format!("en1991.1-7.impact-force.{}", row.id)), loc(&format!("{}: F = {peak:.0} kN, t = {:.0} ms", row.id, duration * 1000.0), &format!("{}: F = {peak:.0} kN, t = {:.0} ms", row.id, duration * 1000.0)), None)];
    Some(Diagram2d {
        id: "impact:pulse".into(),
        title: loc("Impact force against time", "Anprallkraft über der Zeit"),
        description: loc("Idealised triangular force pulse of the first vehicle impact: peak v √(k m), duration 2 m v / F so that the impulse equals the momentum m v (EN 1991-1-7, Annex C).", "Idealisierter dreieckiger Kraftstoß des ersten Fahrzeuganpralls: Spitze v √(k m), Dauer 2 m v / F, sodass der Impuls dem Bewegungsimpuls m v entspricht (EN 1991-1-7, Anhang C)."),
        axes: vec![axis("Time t", "Zeit t", QuantityKind::Time, "s", AxisScale::Linear, Some((0.0, duration))), axis("Impact force F", "Anprallkraft F", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![line("pulse", "F(t)", "F(t)", pulse.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &row.id, format!("impactActions[id={}]", row.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Time t", "Zeit t", "s", pulse.iter().map(|q| q[0]).collect()), column("Impact force F", "Anprallkraft F", "kN", pulse.iter().map(|q| q[1]).collect())] },
    })
}

/// 🪨 Falling load: the mean arrest force m g (h + s) / s of the first falling load against the stopping distance s, with the required and the assumed force of every falling load (EN 1991-1-6, 4.12).
fn falling_force(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.falling_loads.iter().filter(|r| r.mass > 0.0 && r.stopping_distance > 0.0 && r.drop_height >= 0.0).collect();
    let first = *rows.first()?;
    let s_max = rows.iter().map(|r| r.stopping_distance).fold(0.0, f64::max) * 2.0;
    let s_min = rows.iter().map(|r| r.stopping_distance).fold(s_max, f64::min) / 4.0;
    let curve = curves::sample(|s| part_1_6::falling_arrest_force_n(first.mass, first.drop_height, s) / 1000.0, s_min, s_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        let required = part_1_6::falling_arrest_force_n(row.mass, row.drop_height, row.stopping_distance) / 1000.0;
        markers.push(marker(report, &format!("required-{}", row.id), [row.stopping_distance, required], Some(format!("en1991.1-6.falling-load.{}", row.id)), loc(&format!("{}: required {required:.1} kN", row.id), &dec(format!("{}: erforderlich {required:.1} kN", row.id))), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.stopping_distance, row.assumed_force / 1000.0], Some(format!("en1991.1-6.falling-load.{}", row.id)), loc(&format!("{}: assumed {:.1} kN", row.id, row.assumed_force / 1000.0), &dec(format!("{}: angenommen {:.1} kN", row.id, row.assumed_force / 1000.0))), Some(bound(format!("fallingLoads[id={}].assumedForce", row.id), "N", 1000.0, 1000.0))));
    }
    Some(Diagram2d {
        id: "falling:load-force".into(),
        title: loc("Falling load arrest force", "Bremskraft der herabfallenden Last"),
        description: loc("Mean arrest force F = m g (h + s) / s of the first falling load over the stopping distance s, with the required and the assumed force of every falling load (EN 1991-1-6, 4.12; energy balance only).", "Mittlere Bremskraft F = m g (h + s) / s der ersten herabfallenden Last über dem Bremsweg s, mit erforderlicher und angenommener Kraft jeder herabfallenden Last (EN 1991-1-6, 4.12; nur Energiebilanz)."),
        axes: vec![axis("Stopping distance s", "Bremsweg s", QuantityKind::Length, "m", AxisScale::Linear, Some((s_min, s_max))), axis("Arrest force F", "Bremskraft F", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![line("force", "F(s) of the first case", "F(s) des ersten Falls", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &first.id, format!("fallingLoads[id={}]", first.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Stopping distance s", "Bremsweg s", "m", curve.iter().map(|q| q[0]).collect()), column("Arrest force F", "Bremskraft F", "kN", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 📐 Execution imperfection: the equivalent horizontal force N Δ / h of the first member against the tolerance Δ, with the required and the assumed force of every member (EN 1991-1-6, 4.9).
fn imperfection_force(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.execution_imperfections.iter().filter(|r| r.height > 0.0 && r.tolerance >= 0.0 && r.vertical_load >= 0.0).collect();
    let first = *rows.first()?;
    let t_max = rows.iter().map(|r| r.tolerance).fold(0.01, f64::max) * 2.0;
    let curve = curves::sample(|t| part_1_6::imperfection_force_n(first.vertical_load, t, first.height) / 1000.0, 0.0, t_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        let required = part_1_6::imperfection_force_n(row.vertical_load, row.tolerance, row.height) / 1000.0;
        markers.push(marker(report, &format!("required-{}", row.id), [row.tolerance, required], Some(format!("en1991.1-6.execution-imperfection.{}", row.id)), loc(&format!("{}: required {required:.2} kN", row.id), &dec(format!("{}: erforderlich {required:.2} kN", row.id))), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.tolerance, row.assumed_force / 1000.0], Some(format!("en1991.1-6.execution-imperfection.{}", row.id)), loc(&format!("{}: assumed {:.2} kN", row.id, row.assumed_force / 1000.0), &dec(format!("{}: angenommen {:.2} kN", row.id, row.assumed_force / 1000.0))), Some(bound(format!("executionImperfections[id={}].assumedForce", row.id), "N", 1000.0, 100.0))));
    }
    Some(Diagram2d {
        id: "execution:imperfection-force".into(),
        title: loc("Execution imperfection force", "Kraft aus Ausführungsimperfektion"),
        description: loc("Equivalent horizontal force H = N Δ / h of the first member over the execution tolerance Δ, with the required and the assumed force of every member (EN 1991-1-6, 4.9; statics only).", "Horizontale Ersatzkraft H = N Δ / h des ersten Bauteils über der Ausführungstoleranz Δ, mit erforderlicher und angenommener Kraft jedes Bauteils (EN 1991-1-6, 4.9; nur Statik)."),
        axes: vec![axis("Tolerance Δ", "Toleranz Δ", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, t_max))), axis("Horizontal force H", "Horizontalkraft H", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![line("force", "H(Δ) of the first member", "H(Δ) des ersten Bauteils", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &first.id, format!("executionImperfections[id={}]", first.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Tolerance Δ", "Toleranz Δ", "m", curve.iter().map(|q| q[0]).collect()), column("Horizontal force H", "Horizontalkraft H", "kN", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🚆 Derailment forces against the distance from the track axis: the entered design forces F_dx and F_dy of every case with its assumed resistance (EN 1991-1-7, 4.4; the table values are entered, not reproduced).
fn derailment_forces(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let mut rows: Vec<_> = d.derailment_cases.iter().collect();
    rows.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    let first = *rows.first()?;
    let trace = |pick: fn(&DerailmentCase) -> f64| -> Vec<Xy> { rows.iter().map(|r| [r.distance, pick(r) / 1000.0]).collect() };
    let (fdx, fdy, assumed) = (trace(|r| r.design_force_x), trace(|r| r.design_force_y), trace(|r| r.assumed_force));
    let x_max = rows.iter().map(|r| r.distance).fold(1.0, f64::max) * 1.2;
    let markers: Vec<Marker> = rows
        .iter()
        .map(|r| marker(report, &format!("assumed-{}", r.id), [r.distance, r.assumed_force / 1000.0], Some(format!("en1991.1-7.derailment-force.{}", r.id)), loc(&format!("{}: d = {:.1} m, assumed {:.0} kN, F_dx = {:.0} kN", r.id, r.distance, r.assumed_force / 1000.0, r.design_force_x / 1000.0), &dec(format!("{}: d = {:.1} m, angenommen {:.0} kN, F_dx = {:.0} kN", r.id, r.distance, r.assumed_force / 1000.0, r.design_force_x / 1000.0))), Some(bound(format!("derailmentCases[id={}].assumedForce", r.id), "N", 1000.0, 10_000.0))))
        .collect();
    Some(Diagram2d {
        id: "derailment:forces".into(),
        title: loc("Derailment forces against distance", "Entgleisungskräfte über dem Abstand"),
        description: loc("Entered design forces F_dx (along the track) and F_dy (across) of the derailment cases over their distance from the track axis against the assumed resistance (EN 1991-1-7, 4.4).", "Eingegebene Bemessungskräfte F_dx (in Gleisrichtung) und F_dy (quer) der Entgleisungsfälle über ihrem Abstand von der Gleisachse gegen den angenommenen Widerstand (EN 1991-1-7, 4.4)."),
        axes: vec![axis("Distance d from the track axis", "Abstand d von der Gleisachse", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, x_max))), axis("Force F", "Kraft F", QuantityKind::Force, "kN", AxisScale::Linear, None)],
        series: vec![line("fdx", "Design force F_dx", "Bemessungskraft F_dx", fdx.clone(), Tone::Info, 2.0, Dash::Solid), line("fdy", "Design force F_dy", "Bemessungskraft F_dy", fdy.clone(), Tone::Neutral, 1.5, Dash::Solid), line("assumed", "Assumed resistance", "Angenommener Widerstand", assumed.clone(), Tone::Warning, 1.5, Dash::Dashed)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Load, &first.id, format!("derailmentCases[id={}]", first.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Distance d", "Abstand d", "m", fdx.iter().map(|q| q[0]).collect()), column("Design force F_dx", "Bemessungskraft F_dx", "kN", fdx.iter().map(|q| q[1]).collect()), column("Design force F_dy", "Bemessungskraft F_dy", "kN", fdy.iter().map(|q| q[1]).collect()), column("Assumed resistance", "Angenommener Widerstand", "kN", assumed.iter().map(|q| q[1]).collect())] },
    })
}

/// 🔁 Road fatigue models: the axle load of FLM1 (0.7 Q_ik of the lane) or FLM3 (120 kN) of every model against the axle load the document assumes (EN 1991-2, 4.6).
fn fatigue_axle_loads(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .fatigue_models
        .iter()
        .filter_map(|r| {
            let (_, axle) = wave3_rows::fatigue_axles(r, d.annex)?;
            Some(Pair { id: r.id.clone(), name: format!("{} ({} {})", r.id, r.model.to_uppercase(), r.lane), required: axle / 1000.0, assumed: r.assumed_axle_load / 1000.0, check: format!("en1991.2.fatigue-axle.{}", r.id), path: format!("fatigueModels[id={}].assumedAxleLoad", r.id), unit: "N", k: 10_000.0, step: 1000.0 })
        })
        .collect();
    let first = d.fatigue_models.first()?;
    compare(report, "fatigue:axle-loads", loc("Fatigue axle loads", "Ermüdungsachslasten"), loc("Axle load of every road fatigue model (FLM1: 0.7 Q_ik of its lane, FLM3: the axle of the single vehicle) against the axle load the document assumes; the marker drags the assumed load (EN 1991-2, 4.6).", "Achslast jedes Straßenverkehrs-Ermüdungslastmodells (FLM1: 0,7 Q_ik des Fahrstreifens, FLM3: Achse des Einzelfahrzeugs) gegen die im Dokument angenommene Achslast; der Marker verschiebt die angenommene Last (EN 1991-2, 4.6)."), [row_axis("Fatigue model number", "Ermüdungsmodellnummer", pairs.len()), axis("Axle load", "Achslast", QuantityKind::Force, "kN", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Surface, &first.id, format!("fatigueModels[id={}]", first.id)))
}

/// 🚛 Support collisions: the force in driving direction F_dx of every collision (support at deck level or soffit) against the force the document assumes (EN 1991-2, 4.7).
fn collision_forces(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .support_collisions
        .iter()
        .filter(|r| checks::part_2::COLLISION_LOCATIONS.contains(&r.location.as_str()))
        .map(|r| Pair { id: r.id.clone(), name: format!("{} ({})", r.id, r.location), required: checks::part_2::collision_force_x_n(r.location == "soffit") / 1000.0, assumed: r.assumed_force_x / 1000.0, check: format!("en1991.2.collision-x.{}", r.id), path: format!("supportCollisions[id={}].assumedForceX", r.id), unit: "N", k: 100_000.0, step: 10_000.0 })
        .collect();
    let first = d.support_collisions.first()?;
    compare(report, "collision:forces", loc("Collision forces on supports", "Anprallkräfte auf Stützen"), loc("Collision force F_dx in driving direction on every struck support or soffit against the force the document assumes; the marker drags the assumed force (EN 1991-2, 4.7; registry values unverified).", "Anprallkraft F_dx in Fahrtrichtung auf jede getroffene Stütze oder Unterseite gegen die im Dokument angenommene Kraft; der Marker verschiebt die angenommene Kraft (EN 1991-2, 4.7; Registerwerte ungeprüft)."), [row_axis("Collision number", "Anprallnummer", pairs.len()), axis("Force F_dx", "Kraft F_dx", QuantityKind::Force, "kN", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Load, &first.id, format!("supportCollisions[id={}]", first.id)))
}

/// 🛡️ Snow guards: the line force F_s = s b sin α of every guard against the line force the document assumes (EN 1991-1-3, 6.3).
fn snow_guard_forces(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let sk = part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk)?;
    let pairs: Vec<Pair> = d
        .snow_guards
        .iter()
        .filter_map(|g| {
            let r = d.roofs.iter().find(|r| r.id == g.roof_id)?;
            let mu = part_1_3::shape_mu(&r.roof_type, r.pitch_deg, r.has_parapet, r.parapet_height, r.drift_obstruction_height, r.multi_span, sk, d.annex);
            Some(Pair { id: g.id.clone(), name: format!("{} ({})", g.id, r.id), required: part_1_3::snow_guard_force_n_m(part_1_3::roof_snow_pa(sk, mu, r.c_e, r.c_t), g.slope_length, r.pitch_deg) / 1000.0, assumed: g.assumed_force / 1000.0, check: format!("en1991.1-3.snow-guard.{}", g.id), path: format!("snowGuards[id={}].assumedForce", g.id), unit: "N/m", k: 1000.0, step: 100.0 })
        })
        .collect();
    let first = d.snow_guards.first()?;
    compare(report, "snow-guard:forces", loc("Snow guard line forces", "Linienkräfte der Schneefanggitter"), loc("Line force F_s = s b sin α of every snow guard from the snow load of its roof against the line force the document assumes; the marker drags the assumed force (EN 1991-1-3, 6.3).", "Linienkraft F_s = s b sin α jedes Schneefanggitters aus der Schneelast seines Daches gegen die im Dokument angenommene Linienkraft; der Marker verschiebt die angenommene Kraft (EN 1991-1-3, 6.3)."), [row_axis("Snow guard number", "Schneefangnummer", pairs.len()), axis("Line force F_s", "Linienkraft F_s", QuantityKind::ForcePerLength, "kN/m", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Load, &first.id, format!("snowGuards[id={}]", first.id)))
}

/// 🔥 Localised fire: the Hasemi heat flux ĥ of the first fire over the horizontal distance r along the ceiling, with the flux and the assumed flux of every fire whose flame reaches the ceiling (EN 1991-1-2, Annex C).
fn local_fire_flux(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let flux_at = |r: &super::model::LocalFire, distance: f64| {
        let z_prime = part_1_2::local_fire_virtual_source_m(r.heat_release_rate, r.fire_diameter);
        let l_h = part_1_2::local_fire_horizontal_flame_m(r.heat_release_rate, r.ceiling_height);
        part_1_2::hasemi_flux_w_m2(part_1_2::hasemi_y(distance, r.ceiling_height, z_prime, l_h)) / 1000.0
    };
    let rows: Vec<_> = d.local_fires.iter().filter(|r| r.fire_diameter > 0.0 && r.heat_release_rate > 0.0 && r.ceiling_height > 0.0 && part_1_2::local_fire(r.fire_diameter, r.heat_release_rate, 1.0).0 >= r.ceiling_height).collect();
    let first = *rows.first()?;
    let r_max = rows.iter().map(|r| r.horizontal_distance).fold(5.0, f64::max) * 1.5;
    let curve = curves::sample(|distance| flux_at(first, distance), 0.0, r_max, SAMPLES);
    let mut markers: Vec<Marker> = Vec::new();
    for row in &rows {
        let required = flux_at(row, row.horizontal_distance);
        markers.push(marker(report, &format!("required-{}", row.id), [row.horizontal_distance, required], Some(format!("en1991.1-2.local-flux.{}", row.id)), loc(&format!("{}: required {required:.1} kW/m²", row.id), &dec(format!("{}: erforderlich {required:.1} kW/m²", row.id))), None));
        markers.push(marker(report, &format!("assumed-{}", row.id), [row.horizontal_distance, row.assumed_flux / 1000.0], Some(format!("en1991.1-2.local-flux.{}", row.id)), loc(&format!("{}: assumed {:.1} kW/m²", row.id, row.assumed_flux / 1000.0), &dec(format!("{}: angenommen {:.1} kW/m²", row.id, row.assumed_flux / 1000.0))), Some(bound(format!("localFires[id={}].assumedFlux", row.id), "W/m2", 10_000.0, 1000.0))));
    }
    Some(Diagram2d {
        id: "local-fire:flux".into(),
        title: loc("Localised fire heat flux", "Wärmestrom des lokalen Brandes"),
        description: loc("Hasemi heat flux ĥ of the first localised fire along the ceiling over the horizontal distance r, with the required and the assumed flux of every fire whose flame reaches the ceiling (EN 1991-1-2, Annex C; registry values unverified).", "Hasemi-Wärmestrom ĥ des ersten lokalen Brandes entlang der Decke über dem horizontalen Abstand r, mit erforderlichem und angenommenem Wärmestrom jedes Brandes, dessen Flamme die Decke erreicht (EN 1991-1-2, Anhang C; Registerwerte ungeprüft)."),
        axes: vec![axis("Horizontal distance r", "Horizontaler Abstand r", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, r_max))), axis("Heat flux ĥ", "Wärmestrom ĥ", QuantityKind::Power, "kW/m²", AxisScale::Linear, None)],
        series: vec![line("flux", "ĥ(r) of the first fire", "ĥ(r) des ersten Brandes", curve.clone(), Tone::Info, 2.0, Dash::Solid)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &first.id, format!("localFires[id={}]", first.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Horizontal distance r", "Horizontaler Abstand r", "m", curve.iter().map(|q| q[0]).collect()), column("Heat flux ĥ", "Wärmestrom ĥ", "kW/m²", curve.iter().map(|q| q[1]).collect())] },
    })
}

/// 🛣️ Lane plan: the LM1 uniformly distributed load of every notional lane across the carriageway at its offset and width; the lane with the heaviest tandem, which governs, is drawn in the warning tone (EN 1991-2, 4.3.2, Table 4.2).
fn lane_plan(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    if d.structure_kind != StructureKind::Bridge || d.bridge_lanes.is_empty() {
        return None;
    }
    let governing = d.bridge_lanes.iter().max_by(|a, b| part_2::lm1_tandem_n(d.annex, a.lm1_lane).total_cmp(&part_2::lm1_tandem_n(d.annex, b.lm1_lane)))?;
    let udl = |lane: u8| part_2::lm1_udl_pa(d.annex, lane) / 1000.0;
    let series: Vec<Series> = d
        .bridge_lanes
        .iter()
        .map(|l| {
            let (a, b, q) = (l.offset - l.width / 2.0, l.offset + l.width / 2.0, udl(l.lm1_lane));
            let tone = if l.id == governing.id { Tone::Warning } else { Tone::Info };
            line(&format!("lane-{}", l.id), &format!("{} (lane {}, q_ik = {q:.1} kN/m²)", l.id, l.lm1_lane), &dec(format!("{} (Fahrstreifen {}, q_ik = {q:.1} kN/m²)", l.id, l.lm1_lane)), vec![[a, 0.0], [a, q], [b, q], [b, 0.0]], tone, 2.0, Dash::Solid)
        })
        .collect();
    let markers: Vec<Marker> = d.bridge_lanes.iter().map(|l| marker(report, &format!("lane-{}", l.id), [l.offset, udl(l.lm1_lane)], Some(format!("en1991.2.lm1-lane-udl.{}", l.id)), loc(&format!("{}: offset {:.2} m, width {:.2} m{}", l.id, l.offset, l.width, if l.id == governing.id { ", governing tandem" } else { "" }), &dec(format!("{}: Versatz {:.2} m, Breite {:.2} m{}", l.id, l.offset, l.width, if l.id == governing.id { ", maßgebendes Tandem" } else { "" }))), Some(bound(format!("bridgeLanes[id={}].offset", l.id), "m", 1.0, 0.1)))).collect();
    let half = d.bridge_lanes.iter().map(|l| l.offset.abs() + l.width / 2.0).fold(d.bridge_lane_width / 2.0, f64::max) * 1.1;
    let columns = vec![column("Lane offset", "Fahrstreifenversatz", "m", d.bridge_lanes.iter().map(|l| l.offset).collect()), column("Lane width", "Fahrstreifenbreite", "m", d.bridge_lanes.iter().map(|l| l.width).collect()), column("UDL q_ik", "Gleichlast q_ik", "kN/m²", d.bridge_lanes.iter().map(|l| udl(l.lm1_lane)).collect()), column("Tandem Q_ik", "Tandem Q_ik", "kN", d.bridge_lanes.iter().map(|l| part_2::lm1_tandem_n(d.annex, l.lm1_lane) / 1000.0).collect())];
    Some(Diagram2d {
        id: "bridge:lane-plan".into(),
        title: loc("Lane plan of the load model", "Fahrstreifenplan des Lastmodells"),
        description: loc("Notional lanes across the carriageway at their offset and width, each with its LM1 uniformly distributed load; the lane with the heaviest tandem governs and is drawn in the warning tone (EN 1991-2, 4.3.2, Table 4.2).", "Rechnerische Fahrstreifen quer zur Fahrbahn mit Versatz und Breite, jeweils mit ihrer LM1-Gleichlast; der Fahrstreifen mit dem schwersten Tandem ist maßgebend und im Warnton gezeichnet (EN 1991-2, 4.3.2, Tabelle 4.2)."),
        axes: vec![axis("Position across the carriageway", "Lage quer zur Fahrbahn", QuantityKind::Length, "m", AxisScale::Linear, Some((-half, half))), axis("UDL q_ik", "Gleichlast q_ik", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Solid, &governing.id, format!("bridgeLanes[id={}]", governing.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

#[derive(Clone, Copy, PartialEq)]
enum Group {
    Wind,
    Snow,
    Fire,
    Bridge,
    Silo,
    Crane,
    Gravity,
    Stages,
    BridgeWind,
    Dynamic,
    Impact,
    Explosion,
    Section,
    Tie,
    Helicopter,
    Accidental,
    Falling,
    Imperfection,
    Derailment,
    Fatigue,
    Collision,
    LocalFire,
    SnowGuard,
    Footbridge,
    RailSpan,
    Hoist,
    Machine,
    Bulk,
    Dispersal,
    Placement,
}

/// 📈️ Every diagram of the document; with `focus` set to an entity id only the diagrams of the group that entity belongs to (building and wind faces: wind; roofs: snow; and so on).
pub fn diagrams(document: &En1991Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1991, document.annex))
}

fn diagrams_unbased(document: &En1991Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let groups: Vec<Group> = focus.map_or_else(Vec::new, |id| match owner_of(document, id) {
        Some(Owner::Building | Owner::Face(_)) => vec![Group::Wind],
        Some(Owner::Roof(..)) => vec![Group::Snow],
        Some(Owner::Fire) => vec![Group::Fire],
        Some(Owner::Bridge | Owner::Lane(_) | Owner::Vehicle(_)) => vec![Group::Bridge],
        Some(Owner::BridgeWind(_)) => vec![Group::BridgeWind],
        Some(Owner::Dynamic(_)) => vec![Group::Dynamic],
        Some(Owner::Impact(_)) => vec![Group::Impact],
        Some(Owner::Explosion(_)) => vec![Group::Explosion],
        Some(Owner::Section(_)) => vec![Group::Section],
        Some(Owner::Tie(_)) => vec![Group::Tie],
        Some(Owner::Helicopter(_)) => vec![Group::Helicopter],
        Some(Owner::Stage(_) | Owner::ConstructionLoad(_)) => vec![Group::Stages],
        Some(Owner::Accidental(_)) => vec![Group::Accidental],
        Some(Owner::Falling(_)) => vec![Group::Falling],
        Some(Owner::Imperfection(_)) => vec![Group::Imperfection],
        Some(Owner::Derailment(_)) => vec![Group::Derailment],
        Some(Owner::FatigueModel(_)) => vec![Group::Fatigue],
        Some(Owner::Collision(_)) => vec![Group::Collision],
        Some(Owner::LocalFire(_)) => vec![Group::LocalFire],
        Some(Owner::SnowGuard(_)) => vec![Group::SnowGuard],
        Some(Owner::Footbridge(_)) => vec![Group::Footbridge],
        Some(Owner::RailSpan(_)) => vec![Group::RailSpan],
        Some(Owner::CraneHoist(_)) => vec![Group::Hoist],
        Some(Owner::Machine(_)) => vec![Group::Machine],
        Some(Owner::BulkSolid(_)) => vec![Group::Bulk],
        Some(Owner::Dispersal(_)) => vec![Group::Dispersal],
        Some(Owner::Surface(_)) => vec![Group::Wind, Group::Snow],
        Some(Owner::Silo) => vec![Group::Silo],
        Some(Owner::Crane) => vec![Group::Crane],
        Some(Owner::Floor(_) | Owner::SelfWeight(_)) => vec![Group::Gravity],
        _ if document.placements.iter().any(|p| p.id == id) => vec![Group::Placement],
        _ => Vec::new(),
    });
    let want = |g: Group| focus.is_none() || groups.contains(&g);
    let mut out = Vec::new();
    if want(Group::Wind) {
        out.extend(qp_profile(document, report));
        out.extend(cpe_area(document, report));
        out.extend(wind_face_pressure(document, report));
    }
    if want(Group::Snow) {
        out.extend(snow_mu(document, report).into_iter().flatten());
        out.extend(roof_snow_loads(document, report));
        out.extend(row_curves::roof_surface_mu(document, report));
    }
    if want(Group::Fire) {
        out.extend(fire_curves(document, report));
    }
    if want(Group::Bridge) {
        out.extend(influence(document, report));
        out.extend(super::fields::travel::diagram(document));
        out.extend(super::fields::travel::envelope_diagrams(document));
        out.extend(rail_influence(document, report));
        out.extend(lane_loads(document, report));
        out.extend(lane_plan(document, report));
        out.extend(row_curves::special_vehicle_axles(document, report));
    }
    if want(Group::Stages) {
        out.extend(stage_timeline(document, report));
        out.extend(stage_loads(document, report));
    }
    if want(Group::Silo) {
        out.extend(silo_pressure(document, report));
    }
    if want(Group::Crane) {
        out.extend(super::fields::travel::crane_envelope_diagrams(document));
    }
    if want(Group::Gravity) {
        out.extend(psi_chart(document, report));
        out.extend(self_weight_breakdown(document, report));
    }
    if want(Group::BridgeWind) {
        out.extend(bridge_wind_force(document, report));
    }
    if want(Group::Dynamic) {
        out.extend(dynamic_cscd(document, report));
        out.extend(dynamic_cscd_frequency(document, report));
        out.extend(vortex_velocity(document, report));
    }
    if want(Group::Impact) {
        out.extend(impact_force(document, report));
        out.extend(impact_pulse(document, report));
    }
    if want(Group::Explosion) {
        out.extend(gas_explosion(document, report));
    }
    if want(Group::Section) {
        out.extend(section_profile(document, report));
    }
    if want(Group::Tie) {
        out.extend(tie_forces(document, report));
    }
    if want(Group::Accidental) {
        out.extend(accidental_impacts(document, report));
        out.extend(accidental_explosions(document, report));
    }
    if want(Group::Falling) {
        out.extend(falling_force(document, report));
    }
    if want(Group::Imperfection) {
        out.extend(imperfection_force(document, report));
    }
    if want(Group::Derailment) {
        out.extend(derailment_forces(document, report));
    }
    if want(Group::Fatigue) {
        out.extend(fatigue_axle_loads(document, report));
    }
    if want(Group::Collision) {
        out.extend(collision_forces(document, report));
    }
    if want(Group::LocalFire) {
        out.extend(local_fire_flux(document, report));
    }
    if want(Group::SnowGuard) {
        out.extend(snow_guard_forces(document, report));
    }
    if want(Group::Footbridge) {
        out.extend(row_curves::footbridge_load(document, report));
    }
    if want(Group::RailSpan) {
        out.extend(row_curves::rail_phi(document, report));
    }
    if want(Group::Hoist) {
        out.extend(row_curves::hoist_release(document, report));
    }
    if want(Group::Machine) {
        out.extend(row_curves::machine_force(document, report));
    }
    if want(Group::Bulk) {
        out.extend(row_curves::bulk_ratio(document, report));
    }
    if want(Group::Dispersal) {
        out.extend(row_curves::dispersal_footprint(document, report));
    }
    if want(Group::Placement) {
        out.extend(row_curves::placement_plan(document, report));
    }
    if want(Group::Helicopter) {
        out.extend(helicopter_landing(document, report));
    }
    for case in document.cases.iter().filter(|c| focus.is_none_or(|f| f == c.id)) {
        out.extend(super::scene::projection_kit::design_case::case_diagram(&case.id, &loc(&case.label_en, &case.label_de), report, None));
    }
    out.extend(outcomes::family_outcomes(document, report, focus));
    out
}
