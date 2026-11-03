//! 🛞 Diagrams of the isolation systems and the pier bases: the force–displacement loop of the equivalent linear analysis (EN 1998-2 chapter 7, EN 1998-1 chapter 10) and the displacement spectrum with the period lengthening of a pier on its foundation springs (EN 1998-5 and EN 1998-2). Pure projections of the document and its settled checks with the kernels the checks call. See [`super`].

use super::*;
use schema::isolation::Bilinear;

const PERIODS: usize = 80;
const T_MAX: f64 = 4.0;

fn check_value(report: &CheckReport, id: &str) -> Option<(f64, f64)> {
    report.checks.iter().find(|c| c.id == id).filter(|c| c.computed.value.is_finite()).map(|c| (c.computed.value, if c.utilization.is_finite() { c.utilization } else { 0.0 }))
}

fn point(loop_point: &(f64, f64)) -> Vec2 {
    Vec2::new(loop_point.0, loop_point.1)
}

/// 🛞 Diagram `isolation-loop:<system>`: the backbone, the closed hysteresis loop at the design displacement d_cd of the settled check, the secant of the effective stiffness K_eff and the displacement capacity ±d_max; the capacity marker is draggable onto `capacityM` and the design marker is bound to the displacement check. `None` while the system has no valid loop or no settled design displacement.
pub fn isolation_loops(d: &En1998Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    d.isolation_systems.iter().filter_map(|system| isolation_loop(report, system)).collect()
}

fn isolation_loop(report: &CheckReport, system: &model::En1998IsolationSystem) -> Option<Diagram2d> {
    let law = Bilinear { strength_n: system.strength_n, post_yield_n_per_m: system.stiffness_n_per_m, yield_displacement_m: system.yield_displacement_m };
    let check_id = format!("en1998.isolation.{}.displacement", system.id);
    let (d_cd, utilisation) = check_value(report, &check_id).filter(|(v, _)| law.is_valid() && *v > 0.0)?;
    let reach = 1.15 * d_cd.max(system.capacity_m);
    let backbone: Vec<(f64, f64)> = [-reach, -system.yield_displacement_m, 0.0, system.yield_displacement_m, reach].iter().map(|x| (*x, x.signum() * law.force_n(*x))).collect();
    let loop_points = law.loop_points(d_cd);
    let force = law.force_n(d_cd);
    let k_eff = force / d_cd;
    let path = format!("isolationSystems[id={}]", system.id);
    let mut binding = FieldBinding::new(format!("{path}.capacityM"), Affine::IDENTITY, "m");
    binding.quantise = Some(Quantise::Round { step: 0.005 });
    let tone = scene::utilisation_tone(utilisation);
    let columns = vec![
        column("Backbone displacement", "Verschiebung Rückgratkurve", "m", backbone.iter().map(|p| p.0).collect()),
        column("Backbone force", "Kraft Rückgratkurve", "N", backbone.iter().map(|p| p.1).collect()),
        column("Loop displacement", "Verschiebung Schleife", "m", loop_points.iter().map(|p| p.0).collect()),
        column("Loop force", "Kraft Schleife", "N", loop_points.iter().map(|p| p.1).collect()),
    ];
    Some(Diagram2d {
        id: format!("isolation-loop:{}", system.id),
        title: loc(&format!("Isolation hysteresis \u{2013} {}", system.id), &format!("Isolationshysterese \u{2013} {}", system.id)),
        description: loc(
            &format!("Bilinear backbone (Q_d = {:.0} N, K_d = {:.3e} N/m, d_y = {:.4} m), the hysteresis loop at the design displacement d_cd = {d_cd:.4} m with the secant stiffness K_eff = {k_eff:.3e} N/m, and the displacement capacity d_max = {:.4} m; the area of the loop is the energy E_D that gives the equivalent damping (EN 1998-2 chapter 7, EN 1998-1 chapter 10). Drag the capacity marker to change d_max.", system.strength_n, system.stiffness_n_per_m, system.yield_displacement_m, system.capacity_m),
            &format!("Bilineare R\u{fc}ckgratkurve (Q_d = {:.0} N, K_d = {:.3e} N/m, d_y = {:.4} m), die Hystereseschleife bei der Bemessungsverschiebung d_cd = {d_cd:.4} m mit der Sekantensteifigkeit K_eff = {k_eff:.3e} N/m und die Verschiebungskapazit\u{e4}t d_max = {:.4} m; die Fl\u{e4}che der Schleife ist die Energie E_D der \u{e4}quivalenten D\u{e4}mpfung (EN 1998-2 Kapitel 7, EN 1998-1 Kapitel 10). Den Kapazit\u{e4}tsmarker ziehen, um d_max zu \u{e4}ndern.", system.strength_n, system.stiffness_n_per_m, system.yield_displacement_m, system.capacity_m),
        ),
        axes: vec![axis("Displacement d", "Verschiebung d", QuantityKind::Length, "m"), axis("Force F", "Kraft F", QuantityKind::Force, "N")],
        series: vec![
            Series::Polyline { id: "backbone".into(), label: loc("Backbone F(d)", "R\u{fc}ckgratkurve F(d)"), points: backbone.iter().map(point).collect(), style: stroke(Tone::Neutral, 1.5, Dash::Dotted) },
            Series::Polyline { id: "loop".into(), label: loc("Hysteresis loop at d_cd", "Hystereseschleife bei d_cd"), points: loop_points.iter().map(point).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "secant".into(), label: loc("Secant K_eff", "Sekante K_eff"), points: vec![Vec2::new(0.0, 0.0), Vec2::new(d_cd, force)], style: stroke(Tone::Warning, 1.5, Dash::Dashed) },
            Series::Limit { id: "capacity-positive".into(), label: loc("Capacity +d_max", "Kapazit\u{e4}t +d_max"), axis: AxisRef::X, value: system.capacity_m, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
            Series::Limit { id: "capacity-negative".into(), label: loc("Capacity \u{2212}d_max", "Kapazit\u{e4}t \u{2212}d_max"), axis: AxisRef::X, value: -system.capacity_m, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![
            Marker { id: "design".into(), at: Vec2::new(d_cd, force), shape: MarkerShape::Diamond, tone, label: loc(&format!("d_cd = {d_cd:.4} m, F = {force:.0} N"), &format!("d_cd = {d_cd:.4} m, F = {force:.0} N")), utilisation: Some(utilisation), bound_check: Some(check_id), binding: None },
            Marker { id: "reference:capacity".into(), at: Vec2::new(system.capacity_m, law.force_n(system.capacity_m)), shape: MarkerShape::Square, tone: Tone::Error, label: loc(&format!("d_max = {:.4} m", system.capacity_m), &format!("d_max = {:.4} m", system.capacity_m)), utilisation: None, bound_check: None, binding: Some(binding) },
        ],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Support, &system.id, path)),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🪨 Diagram `pier-base:<base>`: the elastic displacement spectrum γ_I S_e(T)(T/2π)² of the bridge class with the fixed-base period and the lengthened period of the pier on its foundation springs, the bearing displacement capacity as the limit; the lengthened marker is bound to the displacement check. `None` while the check has not settled a lengthened period.
pub fn pier_base_spectra(d: &En1998Snapshot, report: &CheckReport, site: &Site) -> Vec<Diagram2d> {
    d.pier_bases.iter().filter_map(|base| pier_base_spectrum(d, report, site, base)).collect()
}

fn pier_base_spectrum(d: &En1998Snapshot, report: &CheckReport, site: &Site, base: &model::En1998PierBase) -> Option<Diagram2d> {
    let bridge_row = d.bridges.iter().find(|b| b.id == base.bridge_id)?;
    let (t_ssi, _) = check_value(report, &format!("en1998.2.{}.base.{}.period", bridge_row.id, base.id)).filter(|(t, _)| *t > 0.0)?;
    let gamma_i = part_2::bridge_gamma_i(bridge_row.importance_class);
    let t_fixed = bridge_view(d, bridge_row, site).and_then(|v| v.longitudinal).and_then(|r| r.periods_s.first().copied()).unwrap_or_else(|| bridge_row.fundamental_period_s.max(0.05));
    let displacement = |t: f64| part_2::bearing_displacement_m(gamma_i * part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, t), t);
    let times: Vec<f64> = (0..=PERIODS).map(|i| 0.02 + (T_MAX - 0.02) * i as f64 / PERIODS as f64).collect();
    let values: Vec<f64> = times.iter().map(|t| displacement(*t)).collect();
    let check_id = format!("en1998.2.{}.base.{}.displacement", bridge_row.id, base.id);
    let utilisation = check_value(report, &check_id).map(|(_, u)| u);
    let path = format!("pierBases[id={}]", base.id);
    let marker = |id: &str, t: f64, tone: Tone, en: &str, de: &str, bound: Option<String>| Marker { id: id.into(), at: Vec2::new(t, displacement(t)), shape: MarkerShape::Diamond, tone, label: loc(&format!("{en}: T = {t:.3} s, d = {:.4} m", displacement(t)), &format!("{de}: T = {t:.3} s, d = {:.4} m", displacement(t))), utilisation: bound.as_ref().and(utilisation), bound_check: bound, binding: None };
    Some(Diagram2d {
        id: format!("pier-base:{}", base.id),
        title: loc(&format!("Pier base period lengthening \u{2013} {}", base.id), &format!("Periodenverl\u{e4}ngerung des Pfeilerfu\u{df}es \u{2013} {}", base.id)),
        description: loc(
            &format!("Elastic displacement spectrum \u{3b3}_I S_e(T)(T/2\u{3c0})\u{b2} with the importance factor {gamma_i} of the bridge: the pier on foundation springs lengthens the period from {t_fixed:.3} s to {t_ssi:.3} s; the line is the bearing displacement capacity {:.4} m (EN 1998-5 and EN 1998-2, 7.5).", bridge_row.bearing_d_rd_m),
            &format!("Elastisches Verschiebungsspektrum \u{3b3}_I S_e(T)(T/2\u{3c0})\u{b2} mit dem Bedeutungsbeiwert {gamma_i} der Br\u{fc}cke: der Pfeiler auf Fundamentfedern verl\u{e4}ngert die Periode von {t_fixed:.3} s auf {t_ssi:.3} s; die Linie ist die Verschiebungskapazit\u{e4}t der Lager {:.4} m (EN 1998-5 und EN 1998-2, 7.5).", bridge_row.bearing_d_rd_m),
        ),
        axes: vec![axis("Period T", "Periode T", QuantityKind::Time, "s"), axis("Displacement d", "Verschiebung d", QuantityKind::Length, "m")],
        series: vec![
            Series::Polyline { id: "displacement".into(), label: loc("Elastic displacement spectrum", "Elastisches Verschiebungsspektrum"), points: times.iter().zip(&values).map(|(t, v)| Vec2::new(*t, *v)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "capacity".into(), label: loc("Bearing capacity d_Rd", "Lagerkapazit\u{e4}t d_Rd"), axis: AxisRef::Y, value: bridge_row.bearing_d_rd_m, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers: vec![marker("fixed", t_fixed, Tone::Neutral, "Fixed base", "Feste Einspannung", None), marker("ssi", t_ssi, utilisation.map_or(Tone::Neutral, scene::utilisation_tone), "With foundation springs", "Mit Fundamentfedern", Some(check_id))],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Support, &base.id, path)),
        legend: true,
        table_twin: DataTable { columns: vec![column("Period T", "Periode T", "s", times), column("Displacement d", "Verschiebung d", "m", values)] },
    })
}
