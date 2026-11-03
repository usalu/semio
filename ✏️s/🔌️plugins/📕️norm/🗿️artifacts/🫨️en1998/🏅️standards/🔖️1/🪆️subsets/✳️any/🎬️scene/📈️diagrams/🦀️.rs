//! 📈️ Diagrams: horizontal elastic and design response spectra with the period markers and T1, the vertical spectrum, the storey drift profile and the NA-DE seismic zone and ground chart.

use super::model::{En1998Building, En1998Snapshot};
use super::response::{respond, site, Site};
use super::scene::diagram_kit::{curves, spectra, Xy};
use super::scene::*;
use super::scene::projection_kit::diagram_parts::{column, stroke};
use super::scene::projection_kit::loc;
use super::scene::projection_kit::diagram_parts;
use super::schema::{cracked_stiffness, part_1};
use super::norm::CheckReport;
use super::kit::drawn_diagram;
use std::collections::BTreeSet;

const T_MAX: f64 = 4.0;
const SAMPLES: usize = 160;
const VERTICAL_RATIO: f64 = 0.9;
const VERTICAL_CORNERS: [f64; 3] = [0.05, 0.15, 1.0];

fn p(point: Xy) -> Vec2 {
    Vec2::new(point[0], point[1])
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: Option<(f64, f64)>) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, domain)
}

fn utilisation_tone(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

fn focus_building<'a>(document: &'a En1998Snapshot, focus: Option<&str>) -> Option<&'a En1998Building> {
    focus.and_then(|f| document.buildings.iter().find(|b| b.id == f || f.starts_with(&format!("{}:", b.id)))).or_else(|| document.buildings.first())
}

fn spectrum_parameters(site: &Site, q: f64) -> spectra::SpectrumParameters {
    spectra::SpectrumParameters { ag: site.gamma_i * site.a_gr, s: site.s, tb: site.tb, tc: site.tc, td: site.td, eta: 1.0, q: q.max(1.0), beta: 0.2 }
}

/// 📈️ The diagrams of one projection for the focused entity id (default: the first building).
pub fn diagrams(d: &En1998Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(d, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1998, d.annex))
}

fn diagrams_unbased(d: &En1998Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let site = site(d);
    let mut out = Vec::new();
    if site.a_gr > 0.0 {
        out.push(horizontal_spectra(report, &site, focus_building(d, focus)));
        out.push(vertical_spectrum(&site));
    }
    if let Some(b) = focus_building(d, focus) {
        if let Some(modes) = mode_profile(d, b) {
            out.push(modes);
        }
        out.extend(modal_participation(d, b));
        out.extend(joint_ratios(d, report, b));
        out.extend(super::capacity::force_profile(d, report, b));
        out.extend(super::storey_response::diagrams(d, report, b));
        out.extend(super::capacity::pushover(d, report, b));
        out.extend(super::capacity::adrs(d, b));
        out.extend(super::capacity::q_chart(b, report));
        out.extend(super::horizontal_elements::diagrams(d, report, b, &site));
    }
    out.extend(super::capacity::chord_rotation_ladders(d, report));
    out.extend(super::capacity::moment_curvature_diagrams(d, report));
    out.extend(super::assessment::diagrams(d, report));
    out.extend(super::geotechnical::diagrams(d, report));
    out.extend(super::structure_views::diagrams(d, report, &site));
    let drawn: BTreeSet<String> = out.iter().map(|x| x.id.clone()).collect();
    out.extend(entity_check_diagrams(d, report, &drawn));
    out.push(plateau_chart(d));
    out
}

/// 📈️ One outcome diagram per bridge, silo, tank, tower, retaining wall, pier, bearing, joint and tie beam for the checks no dedicated physical diagram draws ([`drawn_diagram`]): the checks of the entity in report order against the limit 1, each marker bound to its check and listed in the table twin.
fn entity_check_diagrams(d: &En1998Snapshot, report: &CheckReport, drawn: &BTreeSet<String>) -> Vec<Diagram2d> {
    let owners: Vec<(&str, &str)> = d.bridges.iter().map(|e| (e.id.as_str(), "bridges"))
        .chain(d.silos.iter().map(|e| (e.id.as_str(), "silos")))
        .chain(d.tanks.iter().map(|e| (e.id.as_str(), "tanks")))
        .chain(d.towers.iter().map(|e| (e.id.as_str(), "towers")))
        .chain(d.retaining_walls.iter().map(|e| (e.id.as_str(), "retainingWalls")))
        .chain(d.piers.iter().map(|e| (e.id.as_str(), "piers")))
        .chain(d.bearings.iter().map(|e| (e.id.as_str(), "bearings")))
        .chain(d.joints.iter().map(|e| (e.id.as_str(), "joints")))
        .chain(d.tie_beams.iter().map(|e| (e.id.as_str(), "tieBeams")))
        .collect();
    owners
        .into_iter()
        .filter_map(|(id, collection)| {
            let checks: Vec<_> = report.checks.iter().filter(|c| c.subject.entity_id == id && c.utilization.is_finite() && !drawn_diagram(&c.id).is_some_and(|diagram| drawn.contains(&diagram))).collect();
            if checks.is_empty() {
                return None;
            }
            let points: Vec<Vec2> = checks.iter().enumerate().map(|(i, c)| Vec2::new(i as f64 + 1.0, c.utilization.min(3.0))).collect();
            let markers = checks
                .iter()
                .zip(&points)
                .map(|(c, at)| Marker { id: format!("check-{}", c.id), at: at.clone(), shape: MarkerShape::Square, tone: utilisation_tone(c.utilization), label: loc(&format!("{}: {:.2}", c.title.en, c.utilization), &format!("{}: {:.2}", c.title.de, c.utilization)), utilisation: Some(c.utilization), bound_check: Some(c.id.clone()), binding: None })
                .collect();
            let columns = vec![column("Check (report order)", "Nachweis (Berichtsreihenfolge)", "1", points.iter().map(|p| p.x).collect()), column("Utilisation", "Ausnutzung", "1", points.iter().map(|p| p.y).collect())];
            Some(Diagram2d {
                id: format!("{collection}-checks:{id}"),
                title: loc(&format!("Utilisation of the checks of {id}"), &format!("Ausnutzung der Nachweise von {id}")),
                description: loc("Utilisation of every check of the entity in report order; values above the limit 1 do not comply.", "Ausnutzung aller Nachweise des Objekts in Berichtsreihenfolge; Werte über der Grenze 1 sind nicht eingehalten."),
                axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", Some((0.5, points.len() as f64 + 0.5))), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", None)],
                series: vec![
                    Series::Polyline { id: "utilisation".into(), label: loc("Utilisation", "Ausnutzung"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) },
                    Series::Limit { id: "limit".into(), label: loc("Limit 1", "Grenze 1"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
                ],
                markers,
                annotations: Vec::new(),
                focus: Some(EntityRef::new(EntityKind::Group, id, format!("{collection}[id={id}]"))),
                legend: true,
                table_twin: DataTable { columns },
            })
        })
        .collect()
}

fn horizontal_spectra(report: &CheckReport, site: &Site, building: Option<&En1998Building>) -> Diagram2d {
    let response = building.map(|b| respond(site, b));
    let system = response.as_ref().and_then(|r| r.systems.first());
    let params = spectrum_parameters(site, system.map_or(1.0, |s| s.q));
    let elastic = spectra::spectrum_curve(&params, spectra::SpectrumKind::Elastic, T_MAX, SAMPLES);
    let design = spectra::spectrum_curve(&params, spectra::SpectrumKind::Design, T_MAX, SAMPLES);
    let mut markers: Vec<Marker> = spectra::period_markers(&params, spectra::SpectrumKind::Design)
        .into_iter()
        .map(|(name, point)| Marker { id: format!("reference:corner-{name}"), at: p(point), shape: MarkerShape::Square, tone: Tone::Neutral, label: loc(&format!("{name} = {:.2} s", point[0]), &format!("{name} = {:.2} s", point[0])), utilisation: None, bound_check: None, binding: None })
        .collect();
    let mut series = vec![
        Series::Polyline { id: "elastic".into(), label: loc("Elastic spectrum Se(T)", "Elastisches Spektrum Se(T)"), points: elastic.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Polyline { id: "design".into(), label: loc("Design spectrum Sd(T)", "Bemessungsspektrum Sd(T)"), points: design.iter().copied().map(p).collect(), style: stroke(Tone::Warning, 2.0, Dash::Dashed) },
    ];
    let focus_ref = building.map(|b| EntityRef::new(EntityKind::Group, &b.id, path_of_building(&b.id)));
    if let (Some(b), Some(r), Some(sys)) = (building, response.as_ref(), system) {
        let t1 = r.t1.clamp(0.0, T_MAX);
        let sd = spectra::design(&params, t1);
        let check = format!("en1998.1.{}.{}.baseShear", b.id, sys.id);
        let utilisation = report.checks.iter().find(|c| c.id == check).map(|c| c.utilization).filter(|u| u.is_finite());
        let t1_check = utilisation.map(|_| check.clone());
        let binding = (b.t1_method == semio_s_artifact_norm_en1998_model::T1Method::Given).then(|| {
            let mut binding = FieldBinding::new(format!("{}.t1GivenS", path_of_building(&b.id)), Affine::IDENTITY, "s");
            binding.quantise = Some(Quantise::Round { step: 0.01 });
            binding
        });
        markers.push(Marker { id: "t1".into(), at: Vec2::new(t1, sd), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, utilisation_tone), label: { let (basis_en, basis_de) = cracked_stiffness::building_basis_text(b); loc(&format!("T1 = {t1:.2} s ({basis_en}), Sd = {sd:.2} m/s\u{b2}, Fb = {:.0} kN", sys.f_b / 1000.0), &format!("T1 = {t1:.2} s ({basis_de}), Sd = {sd:.2} m/s\u{b2}, Fb = {:.0} kN", sys.f_b / 1000.0)) }, utilisation, bound_check: utilisation.map(|_| check), binding });
        for (id, factor, en, de) in [("2tc", 2.0, "2 TC", "2 TC"), ("4tc", 4.0, "4 TC", "4 TC")] {
            let t = factor * site.tc;
            if t <= T_MAX {
                markers.push(Marker { id: format!("reference:{id}"), at: Vec2::new(t, spectra::design(&params, t)), shape: MarkerShape::Triangle, tone: Tone::Neutral, label: loc(&format!("{en} = {t:.2} s"), &format!("{de} = {t:.2} s")), utilisation: None, bound_check: None, binding: None });
            }
        }
        for mode in r.modes.iter().filter(|m| m.period > 0.0 && m.period <= T_MAX) {
            let direction = if mode.along_y { "y" } else { "x" };
            let sd_mode = spectra::design(&params, mode.period);
            markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id(&format!("mode-{direction}{}", mode.index), &t1_check), at: Vec2::new(mode.period, sd_mode), shape: MarkerShape::Triangle, tone: Tone::Info, label: loc(&format!("Mode {direction}{}: T = {:.3} s, Sd = {sd_mode:.2} m/s\u{b2}, m_eff = {:.0} %", mode.index, mode.period, 100.0 * mode.m_eff_ratio), &format!("Eigenform {direction}{}: T = {:.3} s, Sd = {sd_mode:.2} m/s\u{b2}, m_eff = {:.0} %", mode.index, mode.period, 100.0 * mode.m_eff_ratio)), utilisation: None, bound_check: t1_check.clone(), binding: None });
        }
        series.push(Series::Limit { id: "t1-line".into(), label: loc("Fundamental period T1", "Grundperiode T1"), axis: AxisRef::X, value: t1, style: stroke(Tone::Info, 1.0, Dash::Dotted) });
        let se = spectra::elastic(&params, t1);
        markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id("t1-se", &t1_check), at: Vec2::new(t1, se), shape: MarkerShape::Circle, tone: Tone::Info, label: loc(&format!("Se(T1) = {se:.2} m/s\u{b2}"), &format!("Se(T1) = {se:.2} m/s\u{b2}")), utilisation: None, bound_check: t1_check.clone(), binding: None });
        let gross = r.t1_gross.clamp(0.0, T_MAX);
        if (gross - t1).abs() > 1e-9 {
            let (sd_gross, se_gross) = (spectra::design(&params, gross), spectra::elastic(&params, gross));
            markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id("t1-gross", &t1_check), at: Vec2::new(gross, sd_gross), shape: MarkerShape::Diamond, tone: Tone::Warning, label: loc(&format!("T1 gross = {gross:.2} s (uncracked), Sd = {sd_gross:.2} m/s\u{b2}"), &format!("T1 ungerissen = {gross:.2} s, Sd = {sd_gross:.2} m/s\u{b2}")), utilisation: None, bound_check: t1_check.clone(), binding: None });
            markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id("t1-gross-se", &t1_check), at: Vec2::new(gross, se_gross), shape: MarkerShape::Circle, tone: Tone::Warning, label: loc(&format!("Se(T1 gross) = {se_gross:.2} m/s\u{b2}"), &format!("Se(T1 ungerissen) = {se_gross:.2} m/s\u{b2}")), utilisation: None, bound_check: t1_check.clone(), binding: None });
            series.push(Series::Limit { id: "t1-gross-line".into(), label: loc("Fundamental period T1, gross stiffness", "Grundperiode T1, ungerissene Steifigkeit"), axis: AxisRef::X, value: gross, style: stroke(Tone::Warning, 1.0, Dash::Dotted) });
        }
    }
    let times: Vec<f64> = elastic.iter().map(|q| q[0]).collect();
    Diagram2d {
        id: "spectra:document".into(),
        title: loc("Response spectra", "Antwortspektren"),
        description: loc("Elastic spectrum Se and design spectrum Sd of EN 1998-1, 3.2.2 over the period T for the design ground acceleration a_g = \u{3b3}_I\u{b7}a_gR, with the corner periods TB, TC, TD and the fundamental period T1.", "Elastisches Spektrum Se und Bemessungsspektrum Sd nach EN 1998-1, 3.2.2 \u{fc}ber der Periode T f\u{fc}r die Bemessungs-Bodenbeschleunigung a_g = \u{3b3}_I\u{b7}a_gR, mit den Eckperioden TB, TC, TD und der Grundperiode T1."),
        axes: vec![axis("Period T", "Periode T", QuantityKind::Time, "s", Some((0.0, T_MAX))), axis("Spectral acceleration", "Spektrale Beschleunigung", QuantityKind::Acceleration, "m/s²", None)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus_ref,
        legend: true,
        table_twin: DataTable { columns: vec![column("Period T", "Periode T", "s", times), column("Se(T)", "Se(T)", "m/s²", elastic.iter().map(|q| q[1]).collect()), column("Sd(T)", "Sd(T)", "m/s²", design.iter().map(|q| q[1]).collect())] },
    }
}

fn path_of_building(id: &str) -> String {
    format!("buildings[id={id}]")
}

fn vertical_spectrum(site: &Site) -> Diagram2d {
    let a_g = site.gamma_i * site.a_gr;
    let points = curves::sample_with_breaks(|t| part_1::vertical_elastic_spectrum(a_g, VERTICAL_RATIO, t), 0.0, T_MAX, SAMPLES, &VERTICAL_CORNERS);
    let markers = VERTICAL_CORNERS.iter().zip(["TB", "TC", "TD"]).map(|(t, name)| Marker { id: format!("reference:vertical-{name}"), at: Vec2::new(*t, part_1::vertical_elastic_spectrum(a_g, VERTICAL_RATIO, *t)), shape: MarkerShape::Square, tone: Tone::Neutral, label: loc(&format!("{name} = {t:.2} s"), &format!("{name} = {t:.2} s")), utilisation: None, bound_check: None, binding: None }).collect();
    Diagram2d {
        id: "spectrum-vertical:document".into(),
        title: loc("Vertical elastic spectrum", "Vertikales elastisches Spektrum"),
        description: loc("Vertical elastic spectrum Sve(T) of EN 1998-1, 3.2.2.3 with a_vg/a_g = 0.9 and the recommended corner periods TB = 0.05 s, TC = 0.15 s, TD = 1.0 s.", "Vertikales elastisches Spektrum Sve(T) nach EN 1998-1, 3.2.2.3 mit a_vg/a_g = 0,9 und den empfohlenen Eckperioden TB = 0,05 s, TC = 0,15 s, TD = 1,0 s."),
        axes: vec![axis("Period T", "Periode T", QuantityKind::Time, "s", Some((0.0, T_MAX))), axis("Vertical acceleration", "Vertikale Beschleunigung", QuantityKind::Acceleration, "m/s²", None)],
        series: vec![Series::Polyline { id: "vertical".into(), label: loc("Vertical spectrum Sve(T)", "Vertikalspektrum Sve(T)"), points: points.iter().copied().map(p).collect(), style: stroke(Tone::Info, 2.0, Dash::Solid) }],
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Period T", "Periode T", "s", points.iter().map(|q| q[0]).collect()), column("Sve(T)", "Sve(T)", "m/s²", points.iter().map(|q| q[1]).collect())] },
    }
}

fn mode_profile(d: &En1998Snapshot, b: &En1998Building) -> Option<Diagram2d> {
    let r = respond(&site(d), b);
    let shown: Vec<_> = r.modes.iter().filter(|m| m.index <= 3).collect();
    if shown.is_empty() {
        return None;
    }
    let mid: Vec<f64> = r.elevations.iter().zip(&r.heights).map(|(z, h)| z - h / 2.0).collect();
    let tones = [Tone::Info, Tone::Warning, Tone::Error];
    let mut series = Vec::new();
    let mut columns = vec![column("Elevation z", "H\u{f6}he z", "m", mid.clone())];
    for m in &shown {
        let dir = if m.along_y { "y" } else { "x" };
        let label = format!("{dir}{}: T = {:.3} s, M_eff = {:.0} %", m.index, m.period, m.m_eff_ratio * 100.0);
        let points = std::iter::once(Vec2::new(0.0, 0.0)).chain(m.shape.iter().zip(&r.elevations).map(|(phi, z)| Vec2::new(*phi, *z))).collect();
        series.push(Series::Polyline { id: format!("mode-{dir}{}", m.index), label: loc(&label, &label), points, style: stroke(tones[(m.index as usize - 1) % 3], if m.along_y { 1.5 } else { 2.0 }, if m.along_y { Dash::Dashed } else { Dash::Solid }) });
        columns.push(column(&format!("\u{3c6} {dir}{}", m.index), &format!("\u{3c6} {dir}{}", m.index), "1", m.shape.clone()));
    }
    Some(Diagram2d {
        id: format!("modes:{}", b.id),
        title: loc(&format!("Eigenmodes \u{2013} {}", b.name), &format!("Eigenformen \u{2013} {}", b.name)),
        description: loc("First three shear-building eigenmodes per direction from the stiffness and mass matrices (EN 1998-1, 4.3.3.3), normalised to the peak, with period and effective modal mass share.", "Die ersten drei Schubbalken-Eigenformen je Richtung aus Steifigkeits- und Massenmatrix (EN 1998-1, 4.3.3.3), auf den Maximalwert normiert, mit Periode und effektivem Massenanteil."),
        axes: vec![axis("Mode shape \u{3c6}", "Eigenform \u{3c6}", QuantityKind::Dimensionless, "1", Some((-1.2, 1.2))), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m", None)],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &b.id, path_of_building(&b.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

/// 🎚️ Cumulative effective modal mass over the mode number per direction with the 90 % requirement of EN 1998-1 4.3.3.1(3) and the period of every mode in its label; the mode selector of the 3D view shows the same modes.
fn modal_participation(d: &En1998Snapshot, b: &En1998Building) -> Option<Diagram2d> {
    let r = respond(&site(d), b);
    if r.modes.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let mut columns = Vec::new();
    for along_y in [false, true] {
        let dir = if along_y { "y" } else { "x" };
        let modes: Vec<_> = r.modes.iter().filter(|m| m.along_y == along_y).collect();
        if modes.is_empty() {
            continue;
        }
        let mut cumulative = 0.0;
        let mut points = Vec::new();
        let mut totals = Vec::new();
        for m in &modes {
            cumulative += m.m_eff_ratio;
            totals.push(cumulative * 100.0);
            points.push(Vec2::new(m.index as f64, cumulative * 100.0));
            let label = format!("{dir}{}: T = {:.3} s, M_eff = {:.1} %, \u{3a3} = {:.1} %", m.index, m.period, m.m_eff_ratio * 100.0, cumulative * 100.0);
            markers.push(Marker { id: format!("reference:mode-{dir}{}", m.index), at: Vec2::new(m.index as f64, cumulative * 100.0), shape: MarkerShape::Diamond, tone: utilisation_tone(0.9 / cumulative.max(1e-9)), label: loc(&label, &label), utilisation: None, bound_check: None, binding: None });
        }
        series.push(Series::Polyline { id: format!("cumulative-{dir}"), label: loc(&format!("Cumulative effective mass {dir}"), &format!("Kumulierte effektive Masse {dir}")), points, style: stroke(if along_y { Tone::Warning } else { Tone::Info }, 2.0, if along_y { Dash::Dashed } else { Dash::Solid }) });
        columns.push(column(&format!("Mode {dir}"), &format!("Eigenform {dir}"), "1", modes.iter().map(|m| m.index as f64).collect()));
        columns.push(column(&format!("Period T {dir}"), &format!("Periode T {dir}"), "s", modes.iter().map(|m| m.period).collect()));
        columns.push(column(&format!("Effective mass {dir}"), &format!("Effektive Masse {dir}"), "%", modes.iter().map(|m| m.m_eff_ratio * 100.0).collect()));
        columns.push(column(&format!("Cumulative {dir}"), &format!("Kumuliert {dir}"), "%", totals));
    }
    series.push(Series::Limit { id: "required".into(), label: loc("Required 90 %", "Erforderlich 90 %"), axis: AxisRef::Y, value: 90.0, style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    Some(Diagram2d {
        id: format!("modal-participation:{}", b.id),
        title: loc(&format!("Modal mass participation \u{2013} {}", b.name), &format!("Modale Massenbeteiligung \u{2013} {}", b.name)),
        description: loc("Cumulative effective modal mass per direction against the mode number; the modes considered must reach 90 % of the total mass (EN 1998-1, 4.3.3.1(3)).", "Kumulierte effektive modale Masse je Richtung \u{fc}ber der Eigenformnummer; die ber\u{fc}cksichtigten Eigenformen m\u{fc}ssen 90 % der Gesamtmasse erreichen (EN 1998-1, 4.3.3.1(3))."),
        axes: vec![axis("Mode number", "Eigenformnummer", QuantityKind::Dimensionless, "1", None), axis("Cumulative effective mass", "Kumulierte effektive Masse", QuantityKind::Dimensionless, "%", Some((0.0, 105.0)))],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &b.id, path_of_building(&b.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn plateau_chart(d: &En1998Snapshot) -> Diagram2d {
    let (_, ground_s, _, _, _) = super::response::annex_params(d).ground_params();
    let grid: Vec<f64> = (0..=12).map(|i| f64::from(i) * 0.25).collect();
    let plateau = |s_ap_r: f64| ground_s * s_ap_r;
    let points: Vec<ScatterPoint> = grid.iter().map(|x| ScatterPoint { at: Vec2::new(*x, plateau(*x)), tone: Tone::Neutral, label: Some(loc(&format!("S_aP,R {x:.2} m/s\u{b2}: S\u{b7}S_aP,R = {:.2} m/s\u{b2}", plateau(*x)), &format!("S_aP,R {x:.2} m/s\u{b2}: S\u{b7}S_aP,R = {:.2} m/s\u{b2}", plateau(*x)))) }).collect();
    let current = d.site.s_ap_r;
    let mut binding = FieldBinding::new("site.sApR", Affine::IDENTITY, "m/s2");
    binding.quantise = Some(Quantise::Round { step: 0.05 });
    let marker = Marker { id: "reference:site-s-ap-r".into(), at: Vec2::new(current, plateau(current)), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc(&format!("Site: S_aP,R = {current:.2} m/s\u{b2}, plateau {:.2} m/s\u{b2}", plateau(current)), &format!("Standort: S_aP,R = {current:.2} m/s\u{b2}, Plateau {:.2} m/s\u{b2}", plateau(current))), utilisation: None, bound_check: None, binding: (!d.annex == semio_s_plugin_norm_compliance::AnnexChoice::En).then_some(binding) };
    Diagram2d {
        id: "plateau-de:document".into(),
        title: loc("NA-DE plateau acceleration", "NA-DE-Plateau-Beschleunigung"),
        description: loc("Elastic plateau ordinate S\u{b7}S_aP,R of the selected ground combination over the plateau spectral acceleration S_aP,R of subsoil A-R (DIN EN 1998-1/NA:2023-11); the site marker is draggable. The digital S_aP,R map of the national annex is not part of this chart.", "Elastische Plateau-Ordinate S\u{b7}S_aP,R der gew\u{e4}hlten Untergrundkombination \u{fc}ber der Plateau-Spektralbeschleunigung S_aP,R des Untergrunds A-R (DIN EN 1998-1/NA:2023-11); der Standortmarker ist ziehbar. Die digitale S_aP,R-Karte des Nationalen Anhangs ist nicht Teil dieses Diagramms."),
        axes: vec![axis("Plateau acceleration S_aP,R", "Plateau-Beschleunigung S_aP,R", QuantityKind::Acceleration, "m/s²", Some((0.0, 3.0))), axis("Plateau ordinate S\u{b7}S_aP,R", "Plateau-Ordinate S\u{b7}S_aP,R", QuantityKind::Acceleration, "m/s²", Some((0.0, 6.0)))],
        series: vec![Series::Scatter { id: "plateau".into(), label: loc("Plateau ordinate", "Plateau-Ordinate"), points }],
        markers: vec![marker],
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("S_aP,R", "S_aP,R", "m/s²", grid.clone()), column("S\u{b7}S_aP,R", "S\u{b7}S_aP,R", "m/s²", grid.iter().map(|x| plateau(*x)).collect())] },
    }
}

/// 🔩️ Diagram `joint-ratios:<building>`: the capacity-design ratio ΣM_Rc/ΣM_Rb of every joint row against the required γ_Rd = 1.3 (EN 1998-1 4.4.2.3(4)); each marker is bound to its joint check and stands at the ratio its two resistances give.
fn joint_ratios(d: &En1998Snapshot, report: &CheckReport, b: &En1998Building) -> Option<Diagram2d> {
    let joints: Vec<_> = d.joints.iter().filter(|j| j.building_id == b.id && j.beam_moment_rd_nm > 0.0).collect();
    if joints.is_empty() {
        return None;
    }
    let points: Vec<Vec2> = joints.iter().enumerate().map(|(i, j)| Vec2::new(i as f64 + 1.0, j.column_moment_rd_nm / j.beam_moment_rd_nm)).collect();
    let factor = part_1::STRONG_COLUMN_FACTOR;
    let markers = joints
        .iter()
        .zip(&points)
        .map(|(j, at)| {
            let check = report.checks.iter().find(|c| c.id == format!("en1998.1.{}.capacityDesign.joint.{}", b.id, j.id));
            let mut binding = FieldBinding::new(format!("joints[id={}].columnMomentRdNm", j.id), Affine { base: j.column_moment_rd_nm, k: j.beam_moment_rd_nm }, "N*m");
            binding.quantise = Some(Quantise::Round { step: 1000.0 });
            Marker { id: format!("joint-{}", j.id), at: at.clone(), shape: MarkerShape::Diamond, tone: if at.y >= factor { Tone::Info } else { Tone::Error }, label: loc(&format!("{}: M_Rc/M_Rb = {:.2}", j.id, at.y), &format!("{}: M_Rc/M_Rb = {:.2}", j.id, at.y)), utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite()), bound_check: check.map(|c| c.id.clone()), binding: Some(binding) }
        })
        .collect();
    let columns = vec![
        column("Joint (row order)", "Knoten (Zeilenfolge)", "1", points.iter().map(|p| p.x).collect()),
        column("Column resistance sum M_Rc", "Stützenwiderstandssumme M_Rc", "N·m", joints.iter().map(|j| j.column_moment_rd_nm).collect()),
        column("Beam resistance sum M_Rb", "Riegelwiderstandssumme M_Rb", "N·m", joints.iter().map(|j| j.beam_moment_rd_nm).collect()),
        column("Ratio M_Rc/M_Rb", "Verhältnis M_Rc/M_Rb", "1", points.iter().map(|p| p.y).collect()),
    ];
    Some(Diagram2d {
        id: format!("joint-ratios:{}", b.id),
        title: loc(&format!("Capacity design of the joints \u{2013} {}", b.id), &format!("Kapazit\u{e4}tsbemessung der Knoten \u{2013} {}", b.id)),
        description: loc("Ratio of the column to the beam resistance sums at every joint against the required 1.3 (EN 1998-1, 4.4.2.3(4)); drag a marker to change the column resistance.", "Verh\u{e4}ltnis der St\u{fc}tzen- zur Riegelwiderstandssumme je Knoten gegen die geforderten 1,3 (EN 1998-1, 4.4.2.3(4)); Marker ziehen \u{e4}ndert den St\u{fc}tzenwiderstand."),
        axes: vec![axis("Joint", "Knoten", QuantityKind::Dimensionless, "1", Some((0.5, points.len() as f64 + 0.5))), axis("M_Rc / M_Rb", "M_Rc / M_Rb", QuantityKind::Dimensionless, "1", None)],
        series: vec![
            Series::Polyline { id: "ratio".into(), label: loc("Ratio M_Rc/M_Rb", "Verh\u{e4}ltnis M_Rc/M_Rb"), points, style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Limit { id: "required".into(), label: loc("Required 1.3", "Gefordert 1,3"), axis: AxisRef::Y, value: factor, style: stroke(Tone::Error, 1.5, Dash::Dashed) },
        ],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &b.id, path_of_building(&b.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}
