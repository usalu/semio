//! 🌋️ Capacity diagrams: the pushover capacity curve of the equivalent SDOF system (EN 1998-1, 4.3.3.4.2 and Annex B), the behaviour-factor chart (5.2.2.2) and the storey force profile (4.3.3.2.3), all derived from the same `response` quantities as the full seismic check.

use super::model::{En1998Building, En1998Snapshot, En1998System};
use super::response::{annex_params, respond, site, Mode, Response, Site, SystemResponse};
use super::scene::projection_kit::diagram_parts::{self, column, stroke};
use super::scene::projection_kit::loc;
use super::scene::*;
use super::schema::{part_1, part_3};
use super::norm::CheckReport;

const ALPHA_MAX: f64 = 1.6;
const ALPHA_STEPS: usize = 12;
const DISPLAY_FACTOR: f64 = 2.0;
const ADRS_SAMPLES: usize = 120;

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: Option<(f64, f64)>) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, domain)
}

fn path_of(building: &str) -> String {
    format!("buildings[id={building}]")
}

fn focus(building: &En1998Building) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Group, &building.id, path_of(&building.id)))
}

fn tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn base_shear_check(report: &CheckReport, building: &str, system: &str) -> (String, Option<f64>) {
    let id = format!("en1998.1.{building}.{system}.baseShear");
    let utilisation = report.checks.iter().find(|c| c.id == id).map(|c| c.utilization).filter(|u| u.is_finite());
    (id, utilisation)
}

/// 🌋️ The idealised capacity curve of one system: elastic stiffness `K = Vb,el / d_et`, plateau `Vu = (αu/α1)·Fb`, yield displacement `Vu / K` and the target displacement of Annex B.5 (equal-displacement rule for `T ≥ TC`, the short-period amplification otherwise).
#[derive(Clone, Debug, PartialEq)]
pub struct Capacity {
    pub stiffness: f64,
    pub elastic_shear: f64,
    pub plateau: f64,
    pub yield_displacement: f64,
    pub target_displacement: f64,
    pub design_displacement: f64,
    pub overstrength: f64,
    pub q_u: f64,
}

pub fn capacity(site: &Site, system: &En1998System, response: &SystemResponse, mode: &Mode) -> Option<Capacity> {
    let elastic_shear = response.f_b * response.q;
    let d_et = mode.amplitude_m;
    if elastic_shear <= 0.0 || d_et <= 0.0 || system.alpha_u_over_alpha_1 <= 0.0 {
        return None;
    }
    let overstrength = system.alpha_u_over_alpha_1;
    let plateau = overstrength * response.f_b;
    let stiffness = elastic_shear / d_et;
    let q_u = elastic_shear / plateau;
    let target_displacement = if mode.period >= site.tc || q_u <= 1.0 { d_et } else { (d_et / q_u * (1.0 + (q_u - 1.0) * site.tc / mode.period)).max(d_et) };
    Some(Capacity { stiffness, elastic_shear, plateau, yield_displacement: plateau / stiffness, target_displacement, design_displacement: response.f_b / stiffness, overstrength, q_u })
}

fn first_mode<'a>(response: &'a Response, along_y: bool) -> Option<&'a Mode> {
    response.modes.iter().find(|m| m.along_y == along_y && m.index == 1)
}

/// 🌋️ Where the plateau of the pushover curve comes from: the storey shear resistances of the snapshot (the weakest storey governs) or, while those are missing, the overstrength factor times the design base shear.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlateauSource {
    StoreyResistance { storey: usize },
    Overstrength,
}

/// 🌋️ One performance point of the N2 method: the target displacement of a limit state read at the capacity curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PerformancePoint {
    pub state: &'static str,
    pub ag_factor: f64,
    pub target: part_1::N2Target,
    pub roof_displacement: f64,
    pub base_shear: f64,
}

/// 🌋️ N2 solution of one system: the SDOF transformation, the elastic-plastic capacity curve and the performance points of the limit states DL, SD (design earthquake) and NC.
#[derive(Clone, Debug, PartialEq)]
pub struct N2Result {
    pub gamma: f64,
    pub m_star: f64,
    pub yield_displacement: f64,
    pub yield_shear: f64,
    pub source: PlateauSource,
    pub points: Vec<PerformancePoint>,
}

/// 📅 The N2 diagrams show the three limit states at the EN 1998-3 recommended return periods, independent of the annex of any single assessment.
const EN_PERIODS: semio_s_plugin_norm_compliance::AnnexChoice = semio_s_plugin_norm_compliance::AnnexChoice::En;

const LIMIT_STATES: [&str; 3] = ["dl", "sd", "nc"];

/// 🌋️ The N2 method of EN 1998-1 Annex B on the shear building: the pushover under the pattern m·φ (roof-normalised first mode), the equal-energy idealisation of the SDOF curve, T*, q_u and the target displacement of every limit state (the spectrum scales with the return period factor of EN 1998-3 2.1); `None` while the mode or the data are missing.
pub fn n2(site: &Site, building: &En1998Building, system: &En1998System, result: &SystemResponse, mode: &Mode, response: &Response) -> Option<N2Result> {
    let roof = *mode.shape.last()?;
    if roof.abs() < 1e-12 {
        return None;
    }
    let shape: Vec<f64> = mode.shape.iter().map(|v| v / roof).collect();
    let (gamma, m_star) = part_1::n2_participation(&response.masses, &shape)?;
    let stiffness: Vec<f64> = building.storeys.iter().map(|s| if result.along_y { super::schema::cracked_stiffness::storey_y(building, s) } else { super::schema::cracked_stiffness::storey_x(building, s) }).collect();
    let resistance: Vec<f64> = building.storeys.iter().map(|s| s.shear_resistance_n).collect();
    let (yield_displacement, yield_shear, source) = match part_1::n2_pushover_yield(&response.masses, &stiffness, &resistance, &shape) {
        Some((d, v, storey)) => (d, v, PlateauSource::StoreyResistance { storey }),
        None => {
            let c = capacity(site, system, result, mode)?;
            (c.yield_displacement, c.plateau, PlateauSource::Overstrength)
        }
    };
    let curve = [(0.0, 0.0), (yield_displacement, yield_shear), (1.0e3 * yield_displacement, yield_shear)];
    let sdof: Vec<(f64, f64)> = curve.iter().map(|(d, v)| (d / gamma, v / gamma)).collect();
    let mut points = Vec::new();
    for state in LIMIT_STATES {
        let factor = part_3::limit_state_ag_factor(state, EN_PERIODS);
        let a_g = site.gamma_i * site.a_gr * factor;
        let spectrum = |t: f64| part_1::elastic_response_spectrum(a_g, site.s, site.tb, site.tc, site.td, t);
        let (target, _) = part_1::n2_solve(&sdof, &spectrum, site.tc, m_star)?;
        let roof_displacement = gamma * target.d_t_star;
        points.push(PerformancePoint { state, ag_factor: factor, target, roof_displacement, base_shear: part_1::curve_force_at(&curve, roof_displacement) });
    }
    Some(N2Result { gamma, m_star, yield_displacement, yield_shear, source, points })
}

fn state_label(state: &str) -> (&'static str, &'static str) {
    match state {
        "dl" => ("DL", "DL"),
        "nc" => ("NC", "NC"),
        _ => ("SD", "SD"),
    }
}

fn state_tone(state: &str) -> Tone {
    match state {
        "dl" => Tone::Info,
        "nc" => Tone::Error,
        _ => Tone::Warning,
    }
}

/// 🌋️ Base shear `Vb` over the control-node displacement `d_n` of every system: the elastic-plastic pushover curve of the shear building (plateau from the weakest storey resistance, overstrength fallback), the elastic branch, the design base shear and the N2 performance points of the limit states DL, SD and NC (EN 1998-1 Annex B); the table twin lists T*, q_u and the target displacement of every state.
pub fn pushover(document: &En1998Snapshot, report: &CheckReport, building: &En1998Building) -> Option<Diagram2d> {
    let site = site(document);
    let response = respond(&site, building);
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut rows: Vec<[f64; 8]> = Vec::new();
    for (index, (system, result)) in building.systems.iter().zip(&response.systems).enumerate() {
        let Some(mode) = first_mode(&response, result.along_y) else { continue };
        let Some(n2) = n2(&site, building, system, result, mode, &response) else { continue };
        let stiffness = n2.yield_shear / n2.yield_displacement;
        let largest = n2.points.iter().map(|p| p.roof_displacement).fold(n2.yield_displacement, f64::max);
        let reach = DISPLAY_FACTOR * largest;
        let capacity_curve = vec![Vec2::new(0.0, 0.0), Vec2::new(n2.yield_displacement, n2.yield_shear), Vec2::new(reach, n2.yield_shear)];
        let elastic_to = n2.points.iter().map(|p| n2.gamma * p.target.d_et_star).fold(n2.yield_displacement, f64::max);
        let elastic = vec![Vec2::new(0.0, 0.0), Vec2::new(elastic_to, stiffness * elastic_to)];
        let (check, utilisation) = base_shear_check(report, &building.id, &system.id);
        series.push(Series::Polyline { id: format!("capacity-{}", system.id), label: loc(&format!("Capacity curve, system {}", system.id), &format!("Kapazit\u{e4}tskurve, System {}", system.id)), points: capacity_curve, style: stroke(Tone::Info, 2.0, Dash::Solid) });
        series.push(Series::Polyline { id: format!("elastic-{}", system.id), label: loc(&format!("Elastic response, system {}", system.id), &format!("Elastische Antwort, System {}", system.id)), points: elastic, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) });
        markers.push(Marker { id: format!("reference:yield-{}", system.id), at: Vec2::new(n2.yield_displacement, n2.yield_shear), shape: MarkerShape::Square, tone: Tone::Neutral, label: loc(&format!("Yield d_y = {:.1} mm, V_y = {:.0} kN", n2.yield_displacement * 1000.0, n2.yield_shear / 1000.0), &format!("Flie\u{df}en d_y = {:.1} mm, V_y = {:.0} kN", n2.yield_displacement * 1000.0, n2.yield_shear / 1000.0)), utilisation: None, bound_check: None, binding: None });
        markers.push(Marker { id: format!("design-{}", system.id), at: Vec2::new(result.f_b / stiffness, result.f_b), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Design Fb = {:.0} kN at d = {:.1} mm", result.f_b / 1000.0, result.f_b / stiffness * 1000.0), &format!("Bemessung Fb = {:.0} kN bei d = {:.1} mm", result.f_b / 1000.0, result.f_b / stiffness * 1000.0)), utilisation, bound_check: utilisation.map(|_| check.clone()), binding: None });
        for (number, point) in n2.points.iter().enumerate() {
            let (en, de) = state_label(point.state);
            markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id(&format!("target-{}-{}", system.id, point.state), &utilisation.map(|_| check.clone())), at: Vec2::new(point.roof_displacement, point.base_shear), shape: MarkerShape::Diamond, tone: state_tone(point.state), label: loc(&format!("{en}: target d_t = {:.1} mm, T* = {:.2} s, q_u = {:.2}", point.roof_displacement * 1000.0, point.target.t_star, point.target.q_u), &format!("{de}: Zielverschiebung d_t = {:.1} mm, T* = {:.2} s, q_u = {:.2}", point.roof_displacement * 1000.0, point.target.t_star, point.target.q_u)), utilisation: None, bound_check: utilisation.map(|_| check.clone()), binding: None });
            rows.push([(index + 1) as f64, (number + 1) as f64, point.ag_factor, point.target.t_star, point.target.q_u, point.target.d_et_star * n2.gamma * 1000.0, point.roof_displacement * 1000.0, point.base_shear / 1000.0]);
        }
    }
    if series.is_empty() {
        return None;
    }
    let names = [("System", "System", "1"), ("State (1 DL, 2 SD, 3 NC)", "Zustand (1 DL, 2 SD, 3 NC)", "1"), ("a_g factor", "a_g-Faktor", "1"), ("T*", "T*", "s"), ("q_u", "q_u", "1"), ("d_et (roof)", "d_et (Dach)", "mm"), ("d_t (roof)", "d_t (Dach)", "mm"), ("V_b at d_t", "V_b bei d_t", "kN")];
    let columns = names.iter().enumerate().map(|(i, (en, de, unit))| column(en, de, unit, rows.iter().map(|r| r[i]).collect())).collect();
    Some(Diagram2d {
        id: format!("pushover:{}", building.id),
        title: loc(&format!("Pushover curve with N2 performance points \u{2013} {}", building.name), &format!("Pushover-Kurve mit N2-Leistungspunkten \u{2013} {}", building.name)),
        description: loc("Pushover curve of the shear building under the load pattern m\u{b7}\u{3c6} (EN 1998-1, 4.3.3.4.2): the weakest storey resistance sets the plateau, the overstrength factor replaces it while storey resistances are missing. The equal-energy idealisation, T*, q_u and the target displacement of Annex B give the performance points of the limit states DL, SD and NC (spectrum scaled by the return period factor of EN 1998-3, 2.1). Simplified: elastic-plastic storeys without redistribution, no post-yield hardening or degradation.", "Pushover-Kurve des Schubbalkens unter dem Lastbild m\u{b7}\u{3c6} (EN 1998-1, 4.3.3.4.2): die schw\u{e4}chste Geschosstragf\u{e4}higkeit bestimmt das Plateau, fehlen sie, ersetzt der \u{dc}berfestigkeitsfaktor es. Die energiegleiche Idealisierung, T*, q_u und die Zielverschiebung nach Anhang B ergeben die Leistungspunkte der Grenzzust\u{e4}nde DL, SD und NC (Spektrum mit dem Wiederkehrperiodenfaktor nach EN 1998-3, 2.1 skaliert). Vereinfacht: elastisch-plastische Geschosse ohne Umlagerung, keine Verfestigung oder Degradation im Nachbruchbereich."),
        axes: vec![axis("Control-node displacement d_n", "Verschiebung des Kontrollknotens d_n", QuantityKind::Length, "m", None), axis("Base shear Vb", "Fu\u{df}punktquerkraft Vb", QuantityKind::Force, "N", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(building),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 📉 The N2 method in the acceleration-displacement format (EN 1998-1 Annex B.5): the elastic response spectra S_a over S_d of the limit states DL, SD and NC, the capacity spectrum of the SDOF system (plateau F_y*/m*, yield d_y*), the radial line of T* through the yield point and the performance points (d_t*, F_y*/m*); the table twin lists T*, q_u, S_ae, d_et* and d_t* per system and state.
pub fn adrs(document: &En1998Snapshot, building: &En1998Building) -> Option<Diagram2d> {
    let site = site(document);
    let response = respond(&site, building);
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut rows: Vec<[f64; 7]> = Vec::new();
    let mut reach = 0.0_f64;
    let mut peak = 0.0_f64;
    for (index, (system, result)) in building.systems.iter().zip(&response.systems).enumerate() {
        let Some(mode) = first_mode(&response, result.along_y) else { continue };
        let Some(n2) = n2(&site, building, system, result, mode, &response) else { continue };
        let sa_y = part_1::n2_adrs_capacity_sa(n2.yield_shear / n2.gamma, n2.m_star);
        let d_y = n2.yield_displacement / n2.gamma;
        let largest = n2.points.iter().map(|p| p.target.d_t_star).fold(d_y, f64::max);
        reach = reach.max(DISPLAY_FACTOR * largest);
        series.push(Series::Polyline { id: format!("capacity-{}", system.id), label: loc(&format!("Capacity spectrum, system {}", system.id), &format!("Kapazit\u{e4}tsspektrum, System {}", system.id)), points: vec![Vec2::new(0.0, 0.0), Vec2::new(d_y, sa_y), Vec2::new(DISPLAY_FACTOR * largest, sa_y)], style: stroke(Tone::Info, 2.0, Dash::Solid) });
        markers.push(Marker { id: format!("reference:yield-{}", system.id), at: Vec2::new(d_y, sa_y), shape: MarkerShape::Square, tone: Tone::Neutral, label: loc(&format!("Yield d_y* = {:.1} mm, S_a = {:.2} m/s\u{b2}", d_y * 1000.0, sa_y), &format!("Flie\u{df}en d_y* = {:.1} mm, S_a = {:.2} m/s\u{b2}", d_y * 1000.0, sa_y)), utilisation: None, bound_check: None, binding: None });
        for (number, point) in n2.points.iter().enumerate() {
            let (en, de) = state_label(point.state);
            let sa_e = point.target.q_u * sa_y;
            let (sd_e, _) = part_1::n2_adrs_point(sa_e, point.target.t_star);
            peak = peak.max(sa_e);
            if point.state == "sd" {
                series.push(Series::Polyline { id: format!("period-{}", system.id), label: loc(&format!("Period line T* = {:.2} s, system {}", point.target.t_star, system.id), &format!("Periodengerade T* = {:.2} s, System {}", point.target.t_star, system.id)), points: vec![Vec2::new(0.0, 0.0), Vec2::new(sd_e, part_1::n2_adrs_radial_sa(point.target.t_star, sd_e))], style: stroke(Tone::Neutral, 1.0, Dash::Dashed) });
            }
            markers.push(Marker { id: format!("reference:elastic-{}-{}", system.id, point.state), at: Vec2::new(sd_e, sa_e), shape: MarkerShape::Circle, tone: Tone::Neutral, label: loc(&format!("{en}: elastic demand d_et* = {:.1} mm, S_ae = {:.2} m/s\u{b2}", sd_e * 1000.0, sa_e), &format!("{de}: elastische Beanspruchung d_et* = {:.1} mm, S_ae = {:.2} m/s\u{b2}", sd_e * 1000.0, sa_e)), utilisation: None, bound_check: None, binding: None });
            markers.push(Marker { id: format!("reference:target-{}-{}", system.id, point.state), at: Vec2::new(point.target.d_t_star, sa_y), shape: MarkerShape::Diamond, tone: state_tone(point.state), label: loc(&format!("{en}: performance point d_t* = {:.1} mm, T* = {:.2} s, q_u = {:.2}", point.target.d_t_star * 1000.0, point.target.t_star, point.target.q_u), &format!("{de}: Leistungspunkt d_t* = {:.1} mm, T* = {:.2} s, q_u = {:.2}", point.target.d_t_star * 1000.0, point.target.t_star, point.target.q_u)), utilisation: None, bound_check: None, binding: None });
            rows.push([(index + 1) as f64, (number + 1) as f64, point.target.t_star, point.target.q_u, sa_e, sd_e * 1000.0, point.target.d_t_star * 1000.0]);
        }
    }
    if series.is_empty() {
        return None;
    }
    let t_max = site.td.max(2.0) * 2.0;
    for state in LIMIT_STATES {
        let (en, de) = state_label(state);
        let a_g = site.gamma_i * site.a_gr * part_3::limit_state_ag_factor(state, EN_PERIODS);
        let points: Vec<Vec2> = (0..=ADRS_SAMPLES)
            .map(|i| {
                let t = t_max * i as f64 / ADRS_SAMPLES as f64;
                let (sd, sa) = part_1::n2_adrs_point(part_1::elastic_response_spectrum(a_g, site.s, site.tb, site.tc, site.td, t), t);
                Vec2::new(sd, sa)
            })
            .filter(|p| p.x <= reach.max(1e-9) * 1.5)
            .collect();
        series.push(Series::Polyline { id: format!("demand-{state}"), label: loc(&format!("Elastic demand spectrum {en}"), &format!("Elastisches Bedarfsspektrum {de}")), points, style: stroke(state_tone(state), 1.0, Dash::Solid) });
    }
    let names = [("System", "System", "1"), ("State (1 DL, 2 SD, 3 NC)", "Zustand (1 DL, 2 SD, 3 NC)", "1"), ("T*", "T*", "s"), ("q_u", "q_u", "1"), ("S_ae", "S_ae", "m/s²"), ("d_et*", "d_et*", "mm"), ("d_t*", "d_t*", "mm")];
    let columns = names.iter().enumerate().map(|(i, (en, de, unit))| column(en, de, unit, rows.iter().map(|r| r[i]).collect())).collect();
    Some(Diagram2d {
        id: format!("adrs:{}", building.id),
        title: loc(&format!("N2 method in ADRS format \u{2013} {}", building.name), &format!("N2-Verfahren im ADRS-Format \u{2013} {}", building.name)),
        description: loc("Capacity spectrum of the equivalent SDOF system against the elastic response spectra of the limit states DL, SD and NC in the acceleration-displacement format (EN 1998-1, Annex B.5): the radial period line T* through the yield point meets the elastic spectrum at S_ae, the performance point is read at d_t* (equal displacement for T* \u{2265} T_C, otherwise the short-period amplification). Same data as the pushover curve, scaled by the transformation factor.", "Kapazit\u{e4}tsspektrum des \u{e4}quivalenten Einmassenschwingers gegen die elastischen Antwortspektren der Grenzzust\u{e4}nde DL, SD und NC im Beschleunigungs-Verschiebungs-Format (EN 1998-1, Anhang B.5): die Periodengerade T* durch den Fließpunkt schneidet das elastische Spektrum bei S_ae, der Leistungspunkt liegt bei d_t* (gleiche Verschiebung f\u{fc}r T* \u{2265} T_C, sonst die Kurzperiodenerh\u{f6}hung). Dieselben Daten wie die Pushover-Kurve, mit dem Transformationsfaktor skaliert."),
        axes: vec![axis("Spectral displacement S_d", "Spektralverschiebung S_d", QuantityKind::Length, "m", None), axis("Spectral acceleration S_a", "Spektralbeschleunigung S_a", QuantityKind::Acceleration, "m/s²", Some((0.0, 1.2 * peak.max(1e-9))))],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(building),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 📐 q over the overstrength ratio of the first system.
fn q_curve(system: &En1998System) -> (Vec<Vec2>, f64) {
    let q_max = part_1::q_max_for_system(system.system_type, system.material, system.ductility_class);
    let points = (0..=ALPHA_STEPS).map(|i| 1.0 + (ALPHA_MAX - 1.0) * i as f64 / ALPHA_STEPS as f64).map(|alpha| Vec2::new(alpha, part_1::behaviour_factor(system.q0, alpha, system.k_w).min(q_max))).collect();
    (points, q_max)
}

/// 🧮️ The behaviour factor `q = q0·(αu/α1)·kw` (5.2.2.2) over the overstrength ratio with the cap of the system type and the current value as a draggable marker.
pub fn q_chart(building: &En1998Building, report: &CheckReport) -> Option<Diagram2d> {
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut columns = Vec::new();
    for (index, system) in building.systems.iter().enumerate() {
        let (points, q_max) = q_curve(system);
        if index == 0 {
            columns.push(column("\u{3b1}u/\u{3b1}1", "\u{3b1}u/\u{3b1}1", "1", points.iter().map(|p| p.x).collect()));
        }
        columns.push(column(&format!("q, system {}", system.id), &format!("q, System {}", system.id), "1", points.iter().map(|p| p.y).collect()));
        series.push(Series::Polyline { id: format!("q-{}", system.id), label: loc(&format!("q(\u{3b1}u/\u{3b1}1), system {}", system.id), &format!("q(\u{3b1}u/\u{3b1}1), System {}", system.id)), points, style: stroke(Tone::Info, 2.0, Dash::Solid) });
        series.push(Series::Limit { id: format!("q-max-{}", system.id), label: loc(&format!("q cap {q_max:.2}, system {}", system.id), &format!("q-Grenze {q_max:.2}, System {}", system.id)), axis: AxisRef::Y, value: q_max, style: stroke(Tone::Error, 1.0, Dash::Dashed) });
        let q = part_1::behaviour_factor(system.q0, system.alpha_u_over_alpha_1, system.k_w).min(q_max);
        let (check, utilisation) = base_shear_check(report, &building.id, &system.id);
        let mut binding = FieldBinding::new(format!("{}.systems[id={}].alphaUOverAlpha1", path_of(&building.id), system.id), Affine::IDENTITY, "1");
        binding.quantise = Some(Quantise::Round { step: 0.05 });
        markers.push(Marker { id: format!("q-now-{}", system.id), at: Vec2::new(system.alpha_u_over_alpha_1.clamp(1.0, ALPHA_MAX), q), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("System {}: q = {q:.2}", system.id), &format!("System {}: q = {q:.2}", system.id)), utilisation, bound_check: utilisation.map(|_| check), binding: Some(binding) });
    }
    if series.is_empty() {
        return None;
    }
    let rows = columns.iter().map(|c| c.values.len()).min().unwrap_or(0);
    columns.iter_mut().for_each(|c| c.values.truncate(rows));
    Some(Diagram2d {
        id: format!("q-factor:{}", building.id),
        title: loc(&format!("Behaviour factor q \u{2013} {}", building.name), &format!("Verhaltensbeiwert q \u{2013} {}", building.name)),
        description: loc("Behaviour factor q = q0\u{b7}(\u{3b1}u/\u{3b1}1)\u{b7}kw (EN 1998-1, 5.2.2.2) over the overstrength ratio, capped at the maximum of the structural type, material and ductility class; the marker is the current value and can be dragged.", "Verhaltensbeiwert q = q0\u{b7}(\u{3b1}u/\u{3b1}1)\u{b7}kw (EN 1998-1, 5.2.2.2) \u{fc}ber dem \u{dc}berfestigkeitsverh\u{e4}ltnis, begrenzt auf das Maximum von Tragwerksart, Baustoff und Duktilit\u{e4}tsklasse; der Marker ist der aktuelle Wert und l\u{e4}sst sich ziehen."),
        axes: vec![axis("Overstrength ratio \u{3b1}u/\u{3b1}1", "\u{dc}berfestigkeitsverh\u{e4}ltnis \u{3b1}u/\u{3b1}1", QuantityKind::Dimensionless, "1", Some((1.0, ALPHA_MAX))), axis("Behaviour factor q", "Verhaltensbeiwert q", QuantityKind::Dimensionless, "1", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(building),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 📶️ The lateral force `Fi` of every storey (4.3.3.2.3) over its elevation, one polyline per system; the top marker carries the base-shear check.
pub fn force_profile(document: &En1998Snapshot, report: &CheckReport, building: &En1998Building) -> Option<Diagram2d> {
    let response = respond(&site(document), building);
    let mid: Vec<f64> = response.elevations.iter().zip(&response.heights).map(|(z, h)| z - h / 2.0).collect();
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut columns = vec![column("Elevation z", "H\u{f6}he z", "m", mid.clone())];
    for system in &response.systems {
        if system.forces.len() != mid.len() || mid.is_empty() {
            continue;
        }
        let points: Vec<Vec2> = system.forces.iter().zip(&mid).map(|(f, z)| Vec2::new(f / 1000.0, *z)).collect();
        let (check, utilisation) = base_shear_check(report, &building.id, &system.id);
        let top = points.last().copied().unwrap_or(Vec2::new(0.0, 0.0));
        markers.push(Marker { id: format!("force-top-{}", system.id), at: top, shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("System {}: Fb = {:.0} kN, top storey {:.0} kN", system.id, system.f_b / 1000.0, top.x), &format!("System {}: Fb = {:.0} kN, oberstes Geschoss {:.0} kN", system.id, system.f_b / 1000.0, top.x)), utilisation, bound_check: utilisation.map(|_| check), binding: None });
        columns.push(column(&format!("Fi, system {}", system.id), &format!("Fi, System {}", system.id), "kN", points.iter().map(|p| p.x).collect()));
        series.push(Series::Polyline { id: format!("force-{}", system.id), label: loc(&format!("Storey forces, system {}", system.id), &format!("Geschosskr\u{e4}fte, System {}", system.id)), points, style: stroke(Tone::Info, 2.0, Dash::Solid) });
    }
    if series.is_empty() {
        return None;
    }
    Some(Diagram2d {
        id: format!("force-profile:{}", building.id),
        title: loc(&format!("Storey force profile \u{2013} {}", building.name), &format!("Geschosskraftverlauf \u{2013} {}", building.name)),
        description: loc("Lateral force Fi = Fb\u{b7}mi\u{b7}zi / \u{3a3}(mj\u{b7}zj) per storey over the elevation (EN 1998-1, 4.3.3.2.3) including the torsion amplification of the system.", "Horizontalkraft Fi = Fb\u{b7}mi\u{b7}zi / \u{3a3}(mj\u{b7}zj) je Geschoss \u{fc}ber der H\u{f6}he (EN 1998-1, 4.3.3.2.3) einschlie\u{df}lich der Torsionserh\u{f6}hung des Systems."),
        axes: vec![axis("Storey force Fi", "Geschosskraft Fi", QuantityKind::Force, "kN", None), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus(building),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🏛️ The design chord rotation capacity of every limit state of one assessment (DL θ_y, SD the registry fraction of θ_um, NC θ_um) with the demand as a limit line: direct inputs over CF and γ_el, or the Annex A expressions of the member section; the marker of the assessed limit state is bound to the chord rotation check.
pub fn chord_rotation_ladders(document: &En1998Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    let annex = annex_params(document).choice();
    let coefficients = part_3::AnnexA::from_registry(annex);
    let sd_factor = super::norm::ndp_number("ndp.en1998.en3.chord-rotation.sd-factor", annex);
    document.assessments.iter().filter_map(|a| {
        let capacity = |state: &str| {
            let cf = if part_3::limit_state_applies_cf(state) { part_3::confidence_factor(a.knowledge_level) } else { 1.0 };
            let part_3::DesignCapacities { yield_rad, ultimate_rad, .. } = part_3::design_capacities(a, &coefficients, cf);
            part_3::chord_rotation_capacity_rad(state, yield_rad, ultimate_rad, sd_factor).unwrap_or(0.0)
        };
        let values: Vec<f64> = LIMIT_STATES.iter().map(|state| capacity(*state)).collect();
        if a.chord_rotation_demand_rad <= 0.0 || values.iter().all(|v| *v <= 0.0) {
            return None;
        }
        let state = a.limit_state.token();
        let current = LIMIT_STATES.iter().position(|s| state.starts_with(*s)).or_else(|| (state.contains("damage")).then_some(0)).or_else(|| (state.contains("significant")).then_some(1)).or_else(|| (state.contains("collapse")).then_some(2));
        let check_id = format!("en1998.3.{}.chordRotation", a.id);
        let utilisation = report.checks.iter().find(|c| c.id == check_id).map(|c| c.utilization).filter(|u| u.is_finite());
        let points: Vec<Vec2> = values.iter().enumerate().map(|(i, v)| Vec2::new((i + 1) as f64, *v)).collect();
        let mut markers: Vec<Marker> = current.map(|i| Marker { id: format!("assessed-{}", a.id), at: points[i].clone(), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("{}: capacity {:.4} rad against demand {:.4} rad", a.limit_state.token().to_uppercase(), values[i], a.chord_rotation_demand_rad), &format!("{}: Kapazit\u{e4}t {:.4} rad gegen Einwirkung {:.4} rad", a.limit_state.token().to_uppercase(), values[i], a.chord_rotation_demand_rad)), utilisation, bound_check: utilisation.map(|_| check_id), binding: None }).into_iter().collect();
        let mut demand_binding = FieldBinding::new(format!("assessments[id={}].chordRotationDemandRad", a.id), Affine::IDENTITY, "rad");
        demand_binding.quantise = Some(Quantise::Round { step: 0.0001 });
        markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id(&format!("demand-{}", a.id), &utilisation.map(|_| check_id.clone())), at: Vec2::new(current.map_or(1.0, |i| (i + 1) as f64), a.chord_rotation_demand_rad), shape: MarkerShape::Square, tone: utilisation.map_or(Tone::Neutral, tone), label: loc(&format!("Demand \u{3b8}Ed = {:.4} rad", a.chord_rotation_demand_rad), &format!("Einwirkung \u{3b8}Ed = {:.4} rad", a.chord_rotation_demand_rad)), utilisation: None, bound_check: utilisation.map(|_| check_id.clone()), binding: Some(demand_binding) });
        let columns = vec![column("Limit state (1 DL, 2 SD, 3 NC)", "Grenzzustand (1 DL, 2 SD, 3 NC)", "1", points.iter().map(|p| p.x).collect()), column("Design capacity", "Bemessungskapazit\u{e4}t", "rad", values.clone()), column("Demand", "Einwirkung", "rad", vec![a.chord_rotation_demand_rad; values.len()])];
        Some(Diagram2d {
            id: format!("chord-rotation:{}", a.id),
            title: loc(&format!("Chord rotation capacity \u{2013} {}", a.id), &format!("Sehnendrehwinkel-Kapazit\u{e4}t \u{2013} {}", a.id)),
            description: loc("Design chord rotation capacity of the member per limit state (EN 1998-3, 2.3 and Annex A): DL \u{3b8}y, SD the registry fraction of \u{3b8}um, NC \u{3b8}um, derived from the section, the axial load, the reinforcement, the confinement and the shear span while no direct value is given; the dashed line is the demand.", "Bemessungskapazit\u{e4}t des Sehnendrehwinkels je Grenzzustand (EN 1998-3, 2.3 und Anhang A): DL \u{3b8}y, SD der Registerbruchteil von \u{3b8}um, NC \u{3b8}um, aus Querschnitt, Normalkraft, Bewehrung, Umschn\u{fc}rung und Schubspannweite abgeleitet, solange kein direkter Wert angegeben ist; die gestrichelte Linie ist die Einwirkung."),
            axes: vec![axis("Limit state", "Grenzzustand", QuantityKind::Dimensionless, "1", Some((0.5, 3.5))), axis("Chord rotation", "Sehnendrehwinkel", QuantityKind::Angle, "rad", None)],
            series: vec![
                Series::Polyline { id: "capacity".into(), label: loc("Design capacity", "Bemessungskapazit\u{e4}t"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) },
                Series::Limit { id: "demand".into(), label: loc("Demand \u{3b8}Ed", "Einwirkung \u{3b8}Ed"), axis: AxisRef::Y, value: a.chord_rotation_demand_rad, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
            ],
            markers,
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Group, &a.id, format!("assessments[id={}]", a.id))),
            legend: true,
            table_twin: DataTable { columns },
        })
    }).collect()
}

const CURVE_POINTS: usize = 48;

/// 🧵️ The moment of the curve at the curvature `kappa` by linear interpolation, the last moment beyond the curve.
fn moment_on(curve: &[(f64, f64)], kappa: f64) -> f64 {
    curve.windows(2).find(|w| kappa <= w[1].0).map_or_else(|| curve.last().map_or(0.0, |p| p.1), |w| w[0].1 + (w[1].1 - w[0].1) * (kappa - w[0].0) / (w[1].0 - w[0].0).max(f64::MIN_POSITIVE))
}

/// 🧵️ Diagram `moment-curvature:<assessment>`: the moment-curvature curve of the assessed section (fibre section with parabola-rectangle concrete and elastic perfectly plastic steel at the axial force N, strengths over CF) with the yield point (φ_y, M_y) feeding θ_y of EN 1998-3 A.10, the ultimate point (φ_u, M_u) and the live demand marker at the equivalent curvature φ_y θ_Ed/θ_y, tied to the chord rotation check; empty while the section is not given or outside the registry strength range.
pub fn moment_curvature_diagrams(document: &En1998Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    let coefficients = part_3::AnnexA::from_registry(annex_params(document).choice());
    document.assessments.iter().filter_map(|a| {
        let cf = if part_3::limit_state_applies_cf(&a.limit_state.token()) { part_3::confidence_factor(a.knowledge_level) } else { 1.0 };
        let member = part_3::RcMember::of(a)?.over_cf(cf);
        let response = member.section_response(&coefficients)?;
        let curve: Vec<(f64, f64)> = member.moment_curvature(&coefficients, CURVE_POINTS).into_iter().map(|(kappa, m)| (kappa, m / 1.0e3)).collect();
        if curve.len() < 2 {
            return None;
        }
        let theta_y = part_3::chord_rotation_yield_rad(&member, &coefficients);
        let check_id = format!("en1998.3.{}.chordRotation", a.id);
        let utilisation = report.checks.iter().find(|c| c.id == check_id).map(|c| c.utilization).filter(|u| u.is_finite());
        let (m_y, m_u) = (response.m_y / 1.0e3, response.m_u / 1.0e3);
        let marker = |id: &str, at: Vec2, shape: MarkerShape, tone: Tone, label: LocalizedCopy| Marker { id: format!("reference:{id}-{}", a.id), at, shape, tone, label, utilisation: None, bound_check: None, binding: None };
        let mut markers = vec![
            marker("yield", Vec2::new(response.kappa_y, m_y), MarkerShape::Diamond, Tone::Info, loc(&format!("Yield: \u{3c6}y = {:.5} 1/m, My = {:.1} kNm", response.kappa_y, m_y), &format!("Flie\u{df}en: \u{3c6}y = {:.5} 1/m, My = {:.1} kNm", response.kappa_y, m_y))),
            marker("ultimate", Vec2::new(response.kappa_u, m_u), MarkerShape::Diamond, Tone::Warning, loc(&format!("Ultimate: \u{3c6}u = {:.5} 1/m, Mu = {:.1} kNm", response.kappa_u, m_u), &format!("Bruch: \u{3c6}u = {:.5} 1/m, Mu = {:.1} kNm", response.kappa_u, m_u))),
        ];
        let equivalent = (a.chord_rotation_demand_rad > 0.0 && theta_y > 0.0).then(|| response.kappa_y * a.chord_rotation_demand_rad / theta_y);
        if let Some(kappa) = equivalent {
            let at = Vec2::new(kappa.min(response.kappa_u), moment_on(&curve, kappa));
            markers.push(Marker { utilisation, bound_check: Some(check_id), ..marker("demand", at, MarkerShape::Square, utilisation.map_or(Tone::Neutral, tone), loc(&format!("Demand \u{3b8}Ed = {:.4} rad, equivalent curvature {:.5} 1/m (\u{3c6}y \u{3b8}Ed/\u{3b8}y)", a.chord_rotation_demand_rad, kappa), &format!("Einwirkung \u{3b8}Ed = {:.4} rad, \u{e4}quivalente Kr\u{fc}mmung {:.5} 1/m (\u{3c6}y \u{3b8}Ed/\u{3b8}y)", a.chord_rotation_demand_rad, kappa))) });
        }
        let columns = vec![column("Curvature", "Kr\u{fc}mmung", "1/m", curve.iter().map(|p| p.0).collect()), column("Moment", "Moment", "kN·m", curve.iter().map(|p| p.1).collect())];
        Some(Diagram2d {
            id: format!("moment-curvature:{}", a.id),
            title: loc(&format!("Moment-curvature \u{2013} {}", a.id), &format!("Moment-Kr\u{fc}mmung \u{2013} {}", a.id)),
            description: loc("Moment-curvature relation of the assessed section at its axial force (fibre section: parabola-rectangle concrete without tension, elastic perfectly plastic steel, strengths over CF): the yield curvature feeds the yield chord rotation of EN 1998-3 A.10; the demand marker sits at the equivalent curvature of the chord rotation demand.", "Moment-Kr\u{fc}mmungs-Beziehung des bewerteten Querschnitts bei seiner Normalkraft (Faserquerschnitt: Parabel-Rechteck-Beton ohne Zug, elastisch-plastischer Stahl, Festigkeiten \u{fc}ber CF): die Fliesskr\u{fc}mmung geht in den Fliess-Sehnendrehwinkel nach EN 1998-3 A.10 ein; die Einwirkungsmarke liegt bei der \u{e4}quivalenten Kr\u{fc}mmung des Sehnendrehwinkels."),
            axes: vec![axis("Curvature", "Kr\u{fc}mmung", QuantityKind::Dimensionless, "1/m", None), axis("Moment", "Moment", QuantityKind::Moment, "kN·m", None)],
            series: vec![Series::Polyline { id: "curve".into(), label: loc("Moment-curvature", "Moment-Kr\u{fc}mmung"), points: curve.iter().map(|p| Vec2::new(p.0, p.1)).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }],
            markers,
            annotations: Vec::new(),
            focus: Some(EntityRef::new(EntityKind::Group, &a.id, format!("assessments[id={}]", a.id))),
            legend: true,
            table_twin: DataTable { columns },
        })
    }).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
