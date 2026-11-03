//! 🌉️ Diagrams of the bridge seismic action (EN 1998-2): the horizontal and vertical spectra with the importance factor, and the behaviour factor of the RC piers over the shear span. Pure projections of the document with the kernels the checks call. See [`super`].

use super::*;

const PERIODS: usize = 80;
const T_MAX: f64 = 4.0;

fn period_grid() -> Vec<f64> {
    (0..=PERIODS).map(|i| 0.02 + (T_MAX - 0.02) * i as f64 / PERIODS as f64).collect()
}

fn curve(id: &str, en: &str, de: &str, times: &[f64], values: &[f64], tone: Tone, dash: Dash) -> Series {
    Series::Polyline { id: id.into(), label: loc(en, de), points: times.iter().zip(values).map(|(t, v)| Vec2::new(*t, *v)).collect(), style: stroke(tone, 2.0, dash) }
}

/// 🌉️ Diagram `bridge-spectrum:<bridge>`: elastic `S_e` and design `S_d` spectrum with `γ_I` of the bridge class, the vertical spectrum `S_ve` with `a_vg = 0.7 a_g`, and the vertical period marker draggable onto `verticalPeriodS` (EN 1998-2 2.1 and 3.2).
pub fn bridge_spectrum(b: &En1998Bridge, site: &Site) -> Diagram2d {
    let gamma_i = part_2::bridge_gamma_i(b.importance_class);
    let q = part_2::design_behaviour_factor(b.period_ratio);
    let ratio = norm::ndp_common("ndp.en1998.en2.spectrum.vertical-ratio");
    let times = period_grid();
    let elastic: Vec<f64> = times.iter().map(|t| gamma_i * part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, *t)).collect();
    let design: Vec<f64> = times.iter().map(|t| part_1::design_spectrum_at(site.a_gr, gamma_i, site.s, site.tb, site.tc, site.td, q, *t)).collect();
    let vertical: Vec<f64> = times.iter().map(|t| part_2::bridge_vertical_spectrum(gamma_i * site.a_gr, ratio, *t)).collect();
    let mut markers = Vec::new();
    if b.vertical_period_s > 0.0 {
        let s_ve = part_2::bridge_vertical_spectrum(gamma_i * site.a_gr, ratio, b.vertical_period_s);
        let mut binding = FieldBinding::new(format!("bridges[id={}].verticalPeriodS", b.id), Affine::IDENTITY, "s");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        markers.push(Marker { id: "reference:vertical-period".into(), at: Vec2::new(b.vertical_period_s, s_ve), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc(&format!("T_v = {:.2} s: S_ve = {:.2} m/s\u{b2}", b.vertical_period_s, s_ve), &format!("T_v = {:.2} s: S_ve = {:.2} m/s\u{b2}", b.vertical_period_s, s_ve)), utilisation: None, bound_check: None, binding: Some(binding) });
    }
    Diagram2d {
        id: format!("bridge-spectrum:{}", b.id),
        title: loc(&format!("Bridge spectra \u{2013} {}", b.id), &format!("Br\u{fc}ckenspektren \u{2013} {}", b.id)),
        description: loc(&format!("Elastic and design spectrum with the importance factor \u{3b3}_I = {gamma_i} of class {:.0} and the vertical spectrum with a_vg/a_g = {ratio}; drag the vertical period marker to change T_v (EN 1998-2, 2.1 and 3.2).", b.importance_class.round().clamp(1.0, 3.0)), &format!("Elastisches und Bemessungsspektrum mit dem Bedeutungsbeiwert \u{3b3}_I = {gamma_i} der Klasse {:.0} und das Vertikalspektrum mit a_vg/a_g = {ratio}; die Marke der Vertikalperiode ziehen \u{e4}ndert T_v (EN 1998-2, 2.1 und 3.2).", b.importance_class.round().clamp(1.0, 3.0))),
        axes: vec![axis("Period T", "Periode T", QuantityKind::Time, "s"), axis("Spectral acceleration", "Spektrale Beschleunigung", QuantityKind::Acceleration, "m/s²")],
        series: vec![curve("elastic", "Elastic S_e (\u{3b3}_I a_gR)", "Elastisch S_e (\u{3b3}_I a_gR)", &times, &elastic, Tone::Info, Dash::Solid), curve("design", "Design S_d", "Bemessung S_d", &times, &design, Tone::Warning, Dash::Solid), curve("vertical", "Vertical S_ve", "Vertikal S_ve", &times, &vertical, Tone::Error, Dash::Dashed)],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &b.id, format!("bridges[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Period T", "Periode T", "s", times.clone()), column("S_e(T)", "S_e(T)", "m/s²", elastic), column("S_d(T)", "S_d(T)", "m/s²", design), column("S_ve(T)", "S_ve(T)", "m/s²", vertical)] },
    }
}

/// 🌉️ Diagram `pier-q:<bridge>`: the behaviour factor `q = 3.5 λ(α_s)` of vertical RC piers over the shear span ratio with the applied `q` as a limit; every pier is a marker at its `α_s = L_s/h` draggable onto `heightM` (EN 1998-2 4.1.6).
pub fn pier_behaviour(d: &En1998Snapshot, report: &CheckReport, b: &En1998Bridge) -> Option<Diagram2d> {
    let (q0, full, min) = (norm::ndp_common("ndp.en1998.en2.q.rc-pier"), norm::ndp_common("ndp.en1998.en2.q.shear-span-full"), norm::ndp_common("ndp.en1998.en2.q.shear-span-min"));
    let piers: Vec<_> = d.piers.iter().filter(|p| p.bridge_id == b.id && p.height_m > 0.0).collect();
    if piers.is_empty() {
        return None;
    }
    let alpha_max = piers.iter().map(|p| part_2::shear_span_ratio(p.height_m, p.section_b_m.max(p.section_h_m))).filter(|a| a.is_finite()).fold(full * 1.5, f64::max);
    let points: Vec<Vec2> = (0..=60).map(|i| min + (alpha_max - min) * i as f64 / 60.0).filter_map(|a| part_2::rc_pier_behaviour_factor(q0, a, full, min).map(|q| Vec2::new(a, q))).collect();
    let q_applied = part_2::design_behaviour_factor(b.period_ratio);
    let (mut alphas, mut limits) = (Vec::new(), Vec::new());
    let mut markers = Vec::new();
    for pier in piers {
        let depth = pier.section_b_m.max(pier.section_h_m);
        let alpha = part_2::shear_span_ratio(pier.height_m, depth);
        let Some(q_max) = part_2::rc_pier_behaviour_factor(q0, alpha, full, min) else { continue };
        let check = format!("en1998.2.{}.pier.{}.behaviour", b.id, pier.id);
        let utilisation = report.checks.iter().find(|c| c.id == check).map(|c| c.utilization).filter(|u| u.is_finite());
        let mut binding = FieldBinding::new(format!("piers[id={}].heightM", pier.id), Affine { base: 0.0, k: depth }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.05 });
        markers.push(Marker { id: format!("pier-{}", pier.id), at: Vec2::new(alpha, q_max), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, scene::utilisation_tone), label: loc(&format!("{}: \u{3b1}_s = {alpha:.2}, q_max = {q_max:.2}", pier.id), &format!("{}: \u{3b1}_s = {alpha:.2}, q_max = {q_max:.2}", pier.id)), utilisation, bound_check: utilisation.map(|_| check), binding: Some(binding) });
        alphas.push(alpha);
        limits.push(q_max);
    }
    Some(Diagram2d {
        id: format!("pier-q:{}", b.id),
        title: loc(&format!("Pier behaviour factor \u{2013} {}", b.id), &format!("Pfeiler-Verhaltensbeiwert \u{2013} {}", b.id)),
        description: loc("Behaviour factor q = 3.5\u{b7}\u{3bb}(\u{3b1}_s) of vertical RC piers over the shear span ratio \u{3b1}_s = L_s/h against the q of the analysis; drag a pier marker to change its height (EN 1998-2, 4.1.6).", "Verhaltensbeiwert q = 3,5\u{b7}\u{3bb}(\u{3b1}_s) vertikaler Stahlbetonpfeiler \u{fc}ber der Schubschlankheit \u{3b1}_s = L_s/h gegen das q der Berechnung; eine Pfeilermarke ziehen \u{e4}ndert die Pfeilerh\u{f6}he (EN 1998-2, 4.1.6)."),
        axes: vec![axis("Shear span ratio \u{3b1}_s", "Schubschlankheit \u{3b1}_s", QuantityKind::Dimensionless, "1"), axis("Behaviour factor q", "Verhaltensbeiwert q", QuantityKind::Dimensionless, "1")],
        series: vec![Series::Polyline { id: "q-max".into(), label: loc("q = 3.5\u{b7}\u{3bb}(\u{3b1}_s)", "q = 3,5\u{b7}\u{3bb}(\u{3b1}_s)"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) }, Series::Limit { id: "q-applied".into(), label: loc("Applied q", "Angesetztes q"), axis: AxisRef::Y, value: q_applied, style: stroke(Tone::Warning, 1.0, Dash::Dashed) }],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &b.id, format!("bridges[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Shear span ratio", "Schubschlankheit", "1", alphas), column("q_max", "q_max", "1", limits)] },
    })
}

const CURVE_POINTS: usize = 48;

/// 🌉️ Diagrams `pier-moment-curvature:<pier>` of the piers on the computed stiffness basis: the moment-curvature curve of the pier section (fibre section at the pier axial load, longitudinal bending) with the yield point (φ_y, M_y), the ultimate point and the secant line of slope EI_eff = M_y/φ_y that feeds the modal analysis (EN 1998-2 2.3.6.1, Annex C).
pub fn pier_moment_curvature(d: &En1998Snapshot, b: &En1998Bridge, site: &Site) -> Vec<Diagram2d> {
    d.piers.iter().filter(|p| p.bridge_id == b.id && p.stiffness_basis == PierStiffnessBasis::Computed).filter_map(|p| {
        let section = bridge::pier_section(p, bridge::Direction::Longitudinal);
        let response = section.section_response(&site.section)?;
        let ei = response.m_y / response.kappa_y;
        let curve: Vec<(f64, f64)> = section.moment_curvature(&site.section, CURVE_POINTS).into_iter().map(|(kappa, m)| (kappa, m / 1.0e3)).collect();
        if curve.len() < 2 || !(ei > 0.0) {
            return None;
        }
        let (m_y, m_u) = (response.m_y / 1.0e3, response.m_u / 1.0e3);
        let marker = |id: &str, at: Vec2, tone: Tone, label: LocalizedCopy| Marker { id: format!("reference:{id}-{}", p.id), at, shape: MarkerShape::Diamond, tone, label, utilisation: None, bound_check: None, binding: None };
        let markers = vec![
            marker("yield", Vec2::new(response.kappa_y, m_y), Tone::Info, loc(&format!("Yield: \u{3c6}y = {:.5} 1/m, My = {:.1} kNm, EI_eff = {:.3e} Nm\u{b2}", response.kappa_y, m_y, ei), &format!("Flie\u{df}en: \u{3c6}y = {:.5} 1/m, My = {:.1} kNm, EI_eff = {:.3e} Nm\u{b2}", response.kappa_y, m_y, ei))),
            marker("ultimate", Vec2::new(response.kappa_u, m_u), Tone::Warning, loc(&format!("Ultimate: \u{3c6}u = {:.5} 1/m, Mu = {:.1} kNm", response.kappa_u, m_u), &format!("Bruch: \u{3c6}u = {:.5} 1/m, Mu = {:.1} kNm", response.kappa_u, m_u))),
        ];
        let secant = vec![Vec2::new(0.0, 0.0), Vec2::new(response.kappa_y, m_y)];
        let columns = vec![column("Curvature", "Kr\u{fc}mmung", "1/m", curve.iter().map(|c| c.0).collect()), column("Moment", "Moment", "kN·m", curve.iter().map(|c| c.1).collect())];
        Some(Diagram2d {
            id: format!("pier-moment-curvature:{}", p.id),
            title: loc(&format!("Pier moment-curvature \u{2013} {}", p.id), &format!("Pfeiler Moment-Kr\u{fc}mmung \u{2013} {}", p.id)),
            description: loc("Moment-curvature relation of the pier section at its axial load (fibre section: parabola-rectangle concrete without tension, elastic perfectly plastic steel); the secant to the yield point has the slope EI_eff = M_y/\u{3c6}_y that stiffens the piers of the bridge analysis (EN 1998-2 2.3.6.1, Annex C).", "Moment-Kr\u{fc}mmungs-Beziehung des Pfeilerquerschnitts bei seiner Normalkraft (Fasermodell: Parabel-Rechteck-Beton ohne Zugfestigkeit, elastisch-ideal plastischer Stahl); die Sekante zum Flie\u{df}punkt hat die Steigung EI_eff = M_y/\u{3c6}_y, mit der die Pfeiler der Br\u{fc}ckenberechnung steifer gerechnet werden (EN 1998-2 2.3.6.1, Anhang C)."),
            axes: vec![axis("Curvature", "Kr\u{fc}mmung", QuantityKind::Dimensionless, "1/m"), axis("Moment", "Moment", QuantityKind::Moment, "kN·m")],
            series: vec![
                Series::Polyline { id: "curve".into(), label: loc("Moment-curvature", "Moment-Kr\u{fc}mmung"), points: curve.iter().map(|c| Vec2::new(c.0, c.1)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
                Series::Polyline { id: "secant".into(), label: loc("Secant EI_eff = M_y/\u{3c6}_y", "Sekante EI_eff = M_y/\u{3c6}_y"), points: secant, style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
            ],
            markers,
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Member, &p.id, format!("piers[id={}]", p.id))),
            legend: true,
            table_twin: DataTable { columns },
        })
    }).collect()
}

/// 🌉️ Diagram `bearing-displacement:<bridge>`: the design displacement `d(T) = γ_I S_e(T) (T/2π)²` of the isolation bearings over the period, the displacement capacity of the bridge and of every bearing row as limits, and the design point at the period of the bridge bound to the bearing check; the marker drags onto `fundamentalPeriodS` when that period is an input (EN 1998-2 7.5).
pub fn bearing_displacement(d: &En1998Snapshot, report: &CheckReport, b: &En1998Bridge, site: &Site, derived_period_s: Option<f64>) -> Option<Diagram2d> {
    if site.a_gr <= 0.0 {
        return None;
    }
    let gamma_i = part_2::bridge_gamma_i(b.importance_class);
    let displacement_mm = |t: f64| 1000.0 * part_2::bearing_displacement_m(gamma_i * part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, t), t);
    let times = period_grid();
    let values: Vec<f64> = times.iter().map(|t| displacement_mm(*t)).collect();
    let t1 = derived_period_s.unwrap_or_else(|| b.fundamental_period_s.max(0.05)).clamp(0.02, T_MAX);
    let bearings: Vec<_> = d.bearings.iter().filter(|k| k.bridge_id == b.id && k.d_rd_m > 0.0).collect();
    if b.bearing_d_rd_m <= 0.0 && bearings.is_empty() {
        return None;
    }
    let mut series = vec![curve("displacement", "Design displacement d(T)", "Bemessungsverschiebung d(T)", &times, &values, Tone::Info, Dash::Solid)];
    if b.bearing_d_rd_m > 0.0 {
        series.push(Series::Limit { id: "capacity".into(), label: loc("Capacity d_Rd of the bridge", "Kapazit\u{e4}t d_Rd der Br\u{fc}cke"), axis: AxisRef::Y, value: 1000.0 * b.bearing_d_rd_m, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    }
    for k in &bearings {
        series.push(Series::Limit { id: format!("capacity-{}", k.id), label: loc(&format!("Capacity d_Rd of {}", k.id), &format!("Kapazit\u{e4}t d_Rd von {}", k.id)), axis: AxisRef::Y, value: 1000.0 * k.d_rd_m, style: stroke(Tone::Warning, 1.0, Dash::Dotted) });
    }
    let (check_id, check) = (format!("en1998.2.{}.bearing", b.id), report.checks.iter().find(|c| c.id == format!("en1998.2.{}.bearing", b.id)));
    let utilisation = check.map(|c| c.utilization).filter(|u| u.is_finite());
    let at = Vec2::new(t1, displacement_mm(t1));
    let binding = derived_period_s.is_none().then(|| {
        let mut binding = FieldBinding::new(format!("bridges[id={}].fundamentalPeriodS", b.id), Affine::IDENTITY, "s");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        binding
    });
    let marker = Marker { id: "design".into(), at: at.clone(), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, scene::utilisation_tone), label: loc(&format!("T = {t1:.2} s: d_Ed = {:.1} mm", at.y), &format!("T = {t1:.2} s: d_Ed = {:.1} mm", at.y)), utilisation, bound_check: utilisation.map(|_| check_id), binding };
    Some(Diagram2d {
        id: format!("bearing-displacement:{}", b.id),
        title: loc(&format!("Bearing displacement \u{2013} {}", b.id), &format!("Lagerverschiebung \u{2013} {}", b.id)),
        description: loc("Design displacement of the bearings d = \u{3b3}_I S_e(T) (T/2\u{3c0})\u{b2} over the period against the displacement capacity of the bridge and of its bearings (EN 1998-2, 7.5).", "Bemessungsverschiebung der Lager d = \u{3b3}_I S_e(T) (T/2\u{3c0})\u{b2} \u{fc}ber der Periode gegen die Verschiebungskapazit\u{e4}t der Br\u{fc}cke und ihrer Lager (EN 1998-2, 7.5)."),
        axes: vec![axis("Period T", "Periode T", QuantityKind::Time, "s"), axis("Displacement d", "Verschiebung d", QuantityKind::Length, "mm")],
        series,
        markers: vec![marker],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &b.id, format!("bridges[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Period T", "Periode T", "s", times.clone()), column("Displacement d(T)", "Verschiebung d(T)", "mm", values)] },
    })
}
