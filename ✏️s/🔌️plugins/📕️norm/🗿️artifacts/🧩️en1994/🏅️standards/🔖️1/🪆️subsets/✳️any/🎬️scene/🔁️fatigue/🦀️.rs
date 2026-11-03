//! 🔁️ Fatigue (EN 1994-2 6.8) of a bridge girder: the S–N diagram of the three detail categories (steel detail, headed stud, reinforcement) with the design points, damage and utilisation, the stress range and damage of every stud row along the girder, and the 3D highlight of the governing stud row. The curve constants stay unverified (`ndp.en1994.2.rebar-fatigue-range`).

use super::beams::local_frame;
use super::checks::catalogue::fatigue::{self, governing_row, stud_rows, Curve, Row};
use super::checks::part_2;
use super::diagrams as dg;
use super::facts::BeamFacts;
use super::kit::*;
use super::layout::Layout;
use super::model::En1994Snapshot;
use super::norm::CheckReport;
use super::scene::diagram_kit::Xy;
use super::scene::projection_kit::diagram_parts::{self, column, stroke};
use super::scene::*;
use super::SceneInput;

const STUD_PREFIXES: [&str; 1] = ["en1994.2.6.8.3.stud."];
const CURVE_POINTS: usize = 40;
const N_MIN: f64 = 1.0e4;
const N_MAX: f64 = 1.0e8;
const MPA: f64 = 1.0e6;
const ROW_SECTION_H: f64 = 0.02;

/// 🔩 The stud rows of a girder with the stress range and the damage of each, and the governing one.
pub(crate) struct StudRows {
    pub curve: Curve,
    pub peak_pa: f64,
    pub rows: Vec<Row>,
    pub governing: usize,
}

/// 🔩 The rows from the stud fatigue check: the range of the check scaled over the stations by the elastic shear flow `|V(x)|/|V|max` (the stud check evaluates the range at the governing section).
pub(crate) fn stud_rows_of(f: &BeamFacts, report: &CheckReport) -> Option<StudRows> {
    let check = check_with(report, &format!("en1994.2.6.8.3.stud.{}", f.beam.id))?;
    let curve = part_2::stud_curve(f.annex);
    let peak_pa = check.utilization * curve.resistance_pa(f.beam.n_cycles);
    if !(peak_pa.is_finite() && peak_pa > 0.0) {
        return None;
    }
    let shape = dg::shear_shape(f);
    let shares: Vec<(f64, f64)> = f.station_xs().into_iter().map(|x| (x, shape.as_ref().map_or_else(|| f.flow_share(x), |v| v(x)))).collect();
    let rows = stud_rows(&curve, peak_pa, f.beam.n_cycles, &shares);
    let governing = governing_row(&rows)?;
    Some(StudRows { curve, peak_pa, rows, governing })
}

fn log_cycles(n_max: f64) -> Vec<f64> {
    (0..CURVE_POINTS).map(|k| 10f64.powf(N_MIN.log10() + (n_max.log10() - N_MIN.log10()) * k as f64 / (CURVE_POINTS - 1) as f64)).collect()
}

struct Category<'a> {
    id: &'static str,
    en: &'static str,
    de: &'static str,
    curve: Curve,
    check: Option<&'a super::norm::CheckResult>,
    tone: Tone,
}

fn sn(f: &BeamFacts, document: &En1994Snapshot, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let n = f.beam.n_cycles;
    let categories = [
        Category { id: "stud", en: "Headed stud Δτ_c = 90 MPa, m = 8", de: "Kopfbolzen Δτ_c = 90 MPa, m = 8", curve: part_2::stud_curve(f.annex), check: check_with(report, &format!("en1994.2.6.8.3.stud.{id}")), tone: Tone::Info },
        Category { id: "steel", en: "Steel detail category", de: "Kerbfallklasse Stahl", curve: part_2::steel_curve(document.fatigue_detail, f.annex), check: check_with(report, &format!("en1994.2.6.8.2.delta-sigma.{id}")), tone: Tone::Warning },
        Category { id: "rebar", en: "Reinforcement Δσ_Rsk = 162.5 MPa, k₂ = 9", de: "Bewehrung Δσ_Rsk = 162,5 MPa, k₂ = 9", curve: fatigue::rebar_curve(), check: check_with(report, &format!("en1994.2.6.8.2.rebar.{id}")), tone: Tone::Error },
    ];
    let cycles = log_cycles(N_MAX.max(n * 10.0));
    let lines: Vec<Vec<Xy>> = categories.iter().map(|c| cycles.iter().map(|&k| [k, c.curve.resistance_pa(k) / MPA]).collect()).collect();
    let markers: Vec<Marker> = categories
        .iter()
        .filter_map(|c| {
            let check = c.check?;
            let delta = check.utilization * c.curve.resistance_pa(n);
            Some(Marker {
                id: format!("{}-point", c.id),
                at: Vec2::new(n, delta / MPA),
                shape: MarkerShape::Circle,
                tone: dg::tone_of_ratio(check.utilization),
                label: loc(&format!("{}: Δ = {:.1} MPa at N = {n:.2e}, D = {:.3}, utilisation {:.2}", c.en, delta / MPA, c.curve.damage(delta, n), check.utilization), &format!("{}: Δ = {:.1} MPa bei N = {n:.2e}, D = {:.3}, Ausnutzung {:.2}", c.de, delta / MPA, c.curve.damage(delta, n), check.utilization)),
                utilisation: Some(check.utilization),
                bound_check: Some(check.id.clone()),
                binding: None,
            })
        })
        .collect();
    Diagram2d {
        id: format!("fatigue-sn:{id}"),
        title: loc(&format!("Fatigue stress range against cycles – {id}"), &format!("Ermüdungsschwingbreite über der Lastspielzahl – {id}")),
        description: loc(
            &format!("Design S–N lines Δ_R(N) = Δ_c·(2·10⁶/N)^(1/m)/γ_Mf of the stud, the steel detail category '{}' and the reinforcement with the design points of the FLM3 checks (EN 1994-2, 6.8.2, 6.8.3); the damage of one block is D = n/N_R = (Δ/Δ_R)^m (unverified: ndp.en1994.2.rebar-fatigue-range).", document.fatigue_detail),
            &format!("Bemessungs-Wöhlerlinien Δ_R(N) = Δ_c·(2·10⁶/N)^(1/m)/γ_Mf des Bolzens, der Kerbfallklasse '{}' des Stahls und der Bewehrung mit den Bemessungspunkten der FLM3-Nachweise (EN 1994-2, 6.8.2, 6.8.3); die Schädigung eines Blocks ist D = n/N_R = (Δ/Δ_R)^m (ungeprüft: ndp.en1994.2.rebar-fatigue-range).", document.fatigue_detail),
        ),
        axes: vec![diagram_parts::axis("Number of cycles N", "Lastspielzahl N", QuantityKind::Dimensionless, "1", AxisScale::Log10, Some((N_MIN, N_MAX.max(n * 10.0)))), diagram_parts::axis("Stress range Δ", "Schwingbreite Δ", QuantityKind::Stress, "MPa", AxisScale::Log10, Some((1.0, 1000.0)))],
        series: categories.iter().zip(&lines).map(|(c, line)| dg::polyline(c.id, c.en, c.de, line, stroke(c.tone, 2.0, Dash::Solid))).collect(),
        markers,
        annotations: Vec::new(),
        focus: dg::beam_entity(id),
        legend: true,
        table_twin: DataTable {
            columns: std::iter::once(column("Number of cycles N", "Lastspielzahl N", "1", cycles.clone()))
                .chain(categories.iter().zip(&lines).map(|(c, line)| column(c.en, c.de, "MPa", line.iter().map(|q| q[1]).collect())))
                .collect(),
        },
    }
}

fn along(f: &BeamFacts, rows: &StudRows, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let n = f.beam.n_cycles;
    let resistance = rows.curve.resistance_pa(n) / MPA;
    let line: Vec<Xy> = rows.rows.iter().map(|r| [r.x_m, r.delta_pa / MPA]).collect();
    let worst_row = &rows.rows[rows.governing];
    let check = check_with(report, &format!("en1994.2.6.8.3.stud.{id}"));
    let total: f64 = rows.rows.iter().map(|r| r.damage).sum();
    Diagram2d {
        id: format!("fatigue-rows:{id}"),
        title: loc(&format!("Stud stress range and damage along the girder – {id}"), &format!("Bolzenschwingbreite und Schädigung entlang des Trägers – {id}")),
        description: loc(
            &format!("Stress range Δτ(x) of the stud rows, the range of the fatigue check scaled by the elastic shear flow |V(x)|/|V|max, against Δτ_R = {resistance:.1} MPa at N = {n:.2e}; the governing row at x = {:.2} m has Δτ = {:.1} MPa and D = {:.3} (sum over all rows {total:.3}); it is highlighted in the 3D view.", worst_row.x_m, worst_row.delta_pa / MPA, worst_row.damage),
            &format!("Schwingbreite Δτ(x) der Bolzenreihen, die mit dem elastischen Schubfluss |V(x)|/|V|max skalierte Schwingbreite des Ermüdungsnachweises, gegen Δτ_R = {resistance:.1} MPa bei N = {n:.2e}; die maßgebende Reihe bei x = {:.2} m hat Δτ = {:.1} MPa und D = {:.3} (Summe über alle Reihen {total:.3}); sie ist in der 3D-Ansicht hervorgehoben.", worst_row.x_m, worst_row.delta_pa / MPA, worst_row.damage),
        ),
        axes: vec![dg::axis("Position x", "Lage x", QuantityKind::Length, "m", Some((0.0, f.l_total))), dg::axis("Stress range Δτ", "Schwingbreite Δτ", QuantityKind::Stress, "MPa", Some((0.0, (rows.peak_pa / MPA).max(resistance) * 1.15)))],
        series: vec![
            dg::polyline("delta-tau", "Stress range Δτ(x)", "Schwingbreite Δτ(x)", &line, stroke(Tone::Info, 2.0, Dash::Solid)),
            dg::limit("delta-tau-r", "Resistance Δτ_R(N)", "Widerstand Δτ_R(N)", AxisRef::Y, resistance, stroke(Tone::Warning, 1.5, Dash::Dashed)),
        ],
        markers: vec![Marker {
            id: "governing-row".into(),
            at: Vec2::new(worst_row.x_m, worst_row.delta_pa / MPA),
            shape: MarkerShape::Triangle,
            tone: check.map_or(Tone::Neutral, |c| dg::tone_of_ratio(c.utilization)),
            label: loc(&format!("Governing row x = {:.2} m: Δτ = {:.1} MPa, D = {:.3}", worst_row.x_m, worst_row.delta_pa / MPA, worst_row.damage), &format!("Maßgebende Reihe x = {:.2} m: Δτ = {:.1} MPa, D = {:.3}", worst_row.x_m, worst_row.delta_pa / MPA, worst_row.damage)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: dg::beam_entity(id),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Position x", "Lage x", "m", rows.rows.iter().map(|r| r.x_m).collect()),
                column("Shear-flow share", "Schubflussanteil", "1", rows.rows.iter().map(|r| r.share).collect()),
                column("Stress range Δτ", "Schwingbreite Δτ", "MPa", line.iter().map(|q| q[1]).collect()),
                column("Damage D", "Schädigung D", "1", rows.rows.iter().map(|r| r.damage).collect()),
            ],
        },
    }
}

/// 🔁️ The S–N diagram and the stud-row diagram of a bridge girder (none without the stud fatigue check).
pub(crate) fn diagrams(f: &BeamFacts, document: &En1994Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    match stud_rows_of(f, report) {
        Some(rows) => vec![sn(f, document, report), along(f, &rows, report)],
        None => Vec::new(),
    }
}

/// 🔩 Adds the translucent box that marks the governing stud row of every bridge girder, bound to the stud fatigue check.
pub(crate) fn highlight(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for beam in &input.document.beams {
        if !out.wants(&beam.id) {
            continue;
        }
        let Some(pose) = layout.pose(&beam.id) else { continue };
        let f = BeamFacts::of(input.document, beam);
        if f.l_total <= 1e-6 || f.steel.height_m <= 1e-6 {
            continue;
        }
        let Some(rows) = stud_rows_of(&f, input.report) else { continue };
        let row = rows.rows[rows.governing];
        let pitch = f.station_pitch();
        let h_sc = f.stud_top().clamp(0.03, (f.depth - 0.02).max(0.03));
        let frame = local_frame(pose, (row.x_m - pitch / 2.0, 0.0, h_sc / 2.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let end = add(&frame.origin, &mul(&frame.x, pitch));
        let geometry = Geometry::Member(MemberGeometry {
            axis: polyline(&[frame.origin.clone(), end.clone()]),
            frame: frame.clone(),
            section: SectionShape::parametric(ProfileKind::Rect, &[("h", h_sc.max(ROW_SECTION_H)), ("b", f.steel.width_m.max(ROW_SECTION_H))]),
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Rigid,
            stations: Vec::new(),
        });
        let name = loc(&format!("Governing stud row of {}", beam.id), &format!("Maßgebende Bolzenreihe von {}", beam.id));
        let detail = loc(
            &format!("Stud row at x = {:.2} m with the largest stress range Δτ = {:.1} MPa against Δτ_R = {:.1} MPa at N = {:.2e}; damage D = {:.3} (EN 1994-2, 6.8.3).", row.x_m, row.delta_pa / MPA, rows.curve.resistance_pa(beam.n_cycles) / MPA, beam.n_cycles, row.damage),
            &format!("Bolzenreihe bei x = {:.2} m mit der größten Schwingbreite Δτ = {:.1} MPa gegen Δτ_R = {:.1} MPa bei N = {:.2e}; Schädigung D = {:.3} (EN 1994-2, 6.8.3).", row.x_m, row.delta_pa / MPA, rows.curve.resistance_pa(beam.n_cycles) / MPA, beam.n_cycles, row.damage),
        );
        let spec = Spec::new(EntityKind::Member, format!("{}:fatigue-row", beam.id), "beams", geometry, MaterialRole::Steel, name, LAYER_CONNECTORS).bound(binding(input.report, &beam.id, &STUD_PREFIXES)).detailed(detail).translucent(0.45);
        out.push(spec, &[frame.origin, end]);
    }
}
