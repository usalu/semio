//! 🧷️ Structural integrity ties (EN 1992-1-1 9.10, catalogue entry DT-14): the peripheral, internal and column or wall ties of a tying-system case drawn on its slab in 3D, the tie of the case in full tone and the other two ghosted, and one diagram of the three tie forces over the span `l_i` with the capacity of the provided steel, whose marker drags `l_i`. Forces, required areas and the capacity are the DT-14 outcomes on the case row with the tie kind swapped, so the picture is the report. See [`super`].

use super::checks::{self, Case};
use super::{axis, stroke, xy};
use super::kit::*;
use super::model::CaseRow;
use super::norm::{design_case::DesignCase, AnnexChoice};
use super::scene::projection_kit::design_case::forms::{Extras, Note, Size, Sub, SubShape};
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

pub const ENTRY: &str = "DT-14";
const KINDS: [&str; 3] = ["peripheral", "internal", "column"];
const BAR: f64 = 0.05;
const SAMPLES: usize = 20;

fn number(row: &CaseRow, key: &str) -> Option<f64> {
    row.numbers.iter().find(|n| n.id == key).map(|n| n.value)
}

fn active(row: &CaseRow) -> &str {
    row.texts.iter().find(|t| t.id == "tie").map_or("peripheral", |t| t.value.as_str())
}

fn case_of(annex: AnnexChoice, row: &CaseRow, kind: &str, li: Option<f64>) -> Case {
    let mut case = checks::case_input(annex, &DesignCase::from(row));
    case.text.insert("tie".into(), kind.into());
    if let Some(li) = li {
        case.num.insert("li".into(), li);
    }
    case
}

/// 🧮️ `(tie force in N, required steel in mm²)` of one tie kind of the case from DT-14.
pub fn tie(annex: AnnexChoice, row: &CaseRow, kind: &str, li: Option<f64>) -> Option<(f64, f64)> {
    let o = checks::evaluate_outcomes(ENTRY, &case_of(annex, row, kind, li)).ok()?;
    let get = |key: &str| o.iter().find(|x| x.key == key).map(|x| x.capacity);
    Some((get("f_tie")?, get("as_req")? * 1.0e6))
}

fn sub(row: &CaseRow, kind: &str, tag: &str, at: [f64; 3], size: [f64; 3], names: ((&str, &str), (&str, &str))) -> Sub {
    let opacity = if active(row) == kind { 0.95 } else { 0.3 };
    Sub { tag: tag.into(), name: (names.0 .0.into(), names.0 .1.into()), detail: (names.1 .0.into(), names.1 .1.into()), role: MaterialRole::Rebar, rgb: None, opacity, at, axes: None, shape: SubShape::Box { size } }
}

/// 🧷️ The extras of a DT-14 case: the peripheral ties along the four slab edges, three internal ties across the slab, four column or wall ties as stubs at the edge centres and a note with the three tie forces; every other entry gets none.
pub fn extras(annex: AnnexChoice, row: &CaseRow, size: Size) -> Extras {
    if row.entry_id != ENTRY {
        return Extras::default();
    }
    let (hx, hy, z) = (size.dx * 0.45, size.dy * 0.45, size.h);
    let per = (("Peripheral tie", "Randzuganker"), ("Peripheral tie of EN 1992-1-1 9.10.2.2", "Randzuganker nach EN 1992-1-1 9.10.2.2"));
    let int = (("Internal tie", "Innenzuganker"), ("Internal tie of EN 1992-1-1 9.10.2.3", "Innenzuganker nach EN 1992-1-1 9.10.2.3"));
    let col = (("Column or wall tie", "Stützen- oder Wandzuganker"), ("Horizontal tie to a column or wall of EN 1992-1-1 9.10.2.4", "Horizontalzuganker zu Stütze oder Wand nach EN 1992-1-1 9.10.2.4"));
    let mut attachments = vec![
        sub(row, "peripheral", "tie-peripheral-n", [0.0, hy, z], [2.0 * hx, BAR, BAR], per),
        sub(row, "peripheral", "tie-peripheral-s", [0.0, -hy, z], [2.0 * hx, BAR, BAR], per),
        sub(row, "peripheral", "tie-peripheral-e", [hx, 0.0, z], [BAR, 2.0 * hy, BAR], per),
        sub(row, "peripheral", "tie-peripheral-w", [-hx, 0.0, z], [BAR, 2.0 * hy, BAR], per),
    ];
    attachments.extend([-0.5, 0.0, 0.5].iter().enumerate().map(|(i, f)| sub(row, "internal", &format!("tie-internal-{i}"), [0.0, f * hy, z], [2.0 * hx, 0.6 * BAR, 0.6 * BAR], int)));
    let stub = 0.15 * size.dx.max(size.dy);
    attachments.extend([(hx + stub / 2.0, 0.0), (-hx - stub / 2.0, 0.0)].iter().enumerate().map(|(i, p)| sub(row, "column", &format!("tie-column-x{i}"), [p.0, p.1, z], [stub, BAR, BAR], col)));
    attachments.extend([(0.0, hy + stub / 2.0), (0.0, -hy - stub / 2.0)].iter().enumerate().map(|(i, p)| sub(row, "column", &format!("tie-column-y{i}"), [p.0, p.1, z], [BAR, stub, BAR], col)));
    let forces: Vec<String> = KINDS.iter().map(|k| tie(annex, row, k, None).map_or_else(|| "-".into(), |(f, _)| format!("{:.0}", f * 1.0e-3))).collect();
    let text = (format!("Ties: peripheral {} kN, internal {} kN, column or wall {} kN", forces[0], forces[1], forces[2]), format!("Zuganker: Rand {} kN, innen {} kN, Stütze oder Wand {} kN", forces[0], forces[1], forces[2]));
    Extras { attachments, notes: vec![Note { tag: "tie-forces".into(), at: [0.0, 0.0, z + 0.3], text }], ..Extras::default() }
}

/// 📈️ Diagram `tying-forces:<case>` of a DT-14 case: the peripheral tie force `min(l_i q1, q2)` over the span `l_i`, the internal and the column or wall tie force as levels, the capacity of the provided tie steel as limit line and the design marker bound to the tie check; with the peripheral tie the marker drags `l_i`.
pub fn diagram(input: &SceneInput, row: &CaseRow) -> Option<Diagram2d> {
    if row.entry_id != ENTRY {
        return None;
    }
    let annex = input.document.annex;
    let li = number(row, "li").filter(|v| *v > 0.0)?;
    let reach = 2.0 * li;
    let sweep: Vec<[f64; 2]> = (0..=SAMPLES).filter_map(|i| {
        let x = reach * i as f64 / SAMPLES as f64;
        tie(annex, row, "peripheral", Some(x)).map(|(f, _)| [x, f * 1.0e-3])
    }).collect();
    let (internal, column_tie) = (tie(annex, row, "internal", None)?, tie(annex, row, "column", None)?);
    let (current, required) = tie(annex, row, active(row), None)?;
    if sweep.len() < 3 {
        return None;
    }
    let check = input.report.checks.iter().find(|c| c.id == format!("en1992.dt-14.util.{}", row.id));
    let tone = check.map_or(Tone::Info, |c| tone_of(&Some(projection_kit::binding_of_check(c))));
    let title = loc(&format!("{} - tying forces", row.id), &format!("{} - Zugankerkräfte", row.id));
    let level = |kn: f64| xy(&[[0.0, kn], [reach, kn]]);
    let mut g = Diagram2d {
        id: format!("tying-forces:{}", row.id),
        title,
        description: loc("Tie forces of EN 1992-1-1 9.10.2 (catalogue entry DT-14): peripheral tie min(l_i q1, q2) over the span l_i, internal and column or wall ties as levels, and the capacity of the provided tie steel; with the peripheral tie the marker drags l_i.", "Zugankerkräfte nach EN 1992-1-1 9.10.2 (Katalogeintrag DT-14): Randzuganker min(l_i q1, q2) über der Spannweite l_i, Innen- sowie Stützen- und Wandzuganker als Niveaus und die Tragfähigkeit des vorhandenen Zugankerstahls; beim Randzuganker verschiebt der Marker l_i."),
        axes: vec![axis("Span l_i", "Spannweite l_i", QuantityKind::Length, "m"), axis("Tie force", "Zugankerkraft", QuantityKind::Force, "kN")],
        series: vec![
            Series::Polyline { id: "peripheral".into(), label: loc("Peripheral tie", "Randzuganker"), points: xy(&sweep), style: stroke(2.0, Dash::Solid, Tone::Info) },
            Series::Polyline { id: "internal".into(), label: loc("Internal tie", "Innenzuganker"), points: level(internal.0 * 1.0e-3), style: stroke(1.5, Dash::Dashed, Tone::Info) },
            Series::Polyline { id: "column".into(), label: loc("Column or wall tie", "Stützen- oder Wandzuganker"), points: level(column_tie.0 * 1.0e-3), style: stroke(1.5, Dash::Dotted, Tone::Info) },
        ],
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(entity_ref(EntityKind::Group, &row.id, &format!("cases[id={}]", row.id))),
        legend: true,
        table_twin: DataTable { columns: Vec::new() },
    };
    if let Some(provided) = number(row, "as_provided").filter(|a| *a > 0.0 && required > 0.0) {
        let capacity = provided * current / required * 1.0e-3;
        g.series.push(Series::Limit { id: "capacity".into(), label: loc(&format!("Capacity of the provided steel, {capacity:.0} kN"), &format!("Tragfähigkeit des vorhandenen Stahls, {capacity:.0} kN")), axis: AxisRef::Y, value: capacity, style: stroke(1.0, Dash::Dashed, Tone::Warning) });
    }
    let binding = (active(row) == "peripheral").then(|| FieldBinding { path: super::norm::design_case::input_path(&row.id, "li", true), map: Affine { base: 0.0, k: 1.0 }, unit: "m".into(), quantise: Some(Quantise::Round { step: 0.1 }) });
    g.markers.push(Marker { id: "design".into(), at: Vec2::new(li, current * 1.0e-3), shape: MarkerShape::Diamond, tone, label: loc(&format!("{} tie: {:.0} kN, A_s,req = {required:.0} mm²", active(row), current * 1.0e-3), &format!("Zuganker {}: {:.0} kN, A_s,erf = {required:.0} mm²", active(row), current * 1.0e-3)), utilisation: check.map(|c| c.utilization).filter(|u| u.is_finite() && *u > 0.0), bound_check: check.map(|c| c.id.clone()), binding });
    g.table_twin = DataTable { columns: vec![
        column("Span l_i", "Spannweite l_i", "m", sweep.iter().map(|p| p[0]).collect()),
        column("Peripheral tie", "Randzuganker", "kN", sweep.iter().map(|p| p[1]).collect()),
        column("Internal tie", "Innenzuganker", "kN", vec![internal.0 * 1.0e-3; sweep.len()]),
        column("Column or wall tie", "Stützen- oder Wandzuganker", "kN", vec![column_tie.0 * 1.0e-3; sweep.len()]),
    ] };
    Some(g)
}

/// 📈️ The tying diagrams of every DT-14 case of the document.
pub fn diagrams(input: &SceneInput) -> Vec<Diagram2d> {
    input.document.cases.iter().filter_map(|row| diagram(input, row)).collect()
}
