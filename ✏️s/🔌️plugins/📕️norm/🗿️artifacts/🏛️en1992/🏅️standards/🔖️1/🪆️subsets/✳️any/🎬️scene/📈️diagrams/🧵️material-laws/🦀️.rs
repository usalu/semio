//! 🧵️ Material laws of the grade rows: the Table 3.1 properties of a concrete grade (M-02, EN 1992-1-1 3.1.3), the reinforcing steel diagram of a reinforcement grade (M-08, 3.2.7) and the ductility plane of Annex C (A-07, Table C.1: `k = (f_t/f_y)k` against `ε_uk`). Every number is read from the catalogue entry through `evaluate_outcomes`, the Annex C minima from the class table of the model, so picture and report cannot disagree; the grade point drags `fCk` and `k` (live re-evaluation through the normal projection). The same numbers label the member in 3D, toned by the grade checks. See [`super`].

use super::checks::{self, Case};
use super::{axis, stroke, xy};
use super::kit::*;
use super::model::{ConcreteGrade, DuctilityClass, En1992Snapshot, RcMember, ReinforcementGrade};
use super::norm::{CheckReport, CheckResult};
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

const CLASSES: [f64; 14] = [12.0, 16.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 70.0, 80.0, 90.0];
const DUCTILITY: [DuctilityClass; 3] = [DuctilityClass::A, DuctilityClass::B, DuctilityClass::C];

fn value(outcomes: &[checks::Outcome], key: &str) -> Option<f64> {
    outcomes.iter().find(|o| o.key == key).map(|o| o.capacity)
}

fn utilisation_of(check: Option<&CheckResult>) -> Option<f64> {
    check.map(|c| c.utilization).filter(|u| u.is_finite() && *u > 0.0)
}

fn tone_and_id(check: Option<&CheckResult>) -> (Tone, Option<String>) {
    check.map_or((Tone::Info, None), |c| (tone_of(&Some(projection_kit::binding_of_check(c))), Some(c.id.clone())))
}

fn check_of<'a>(report: &'a CheckReport, id: &str) -> Option<&'a CheckResult> {
    report.checks.iter().find(|c| c.id == id)
}

/// 🧵️ The worst `en1992.annexC.ductility.<member>` check of the members that use the grade.
pub fn ductility_check<'a>(document: &En1992Snapshot, report: &'a CheckReport, grade_id: &str) -> Option<&'a CheckResult> {
    document
        .members
        .iter()
        .filter(|m| m.reinforcement_grade_id == grade_id)
        .filter_map(|m| check_of(report, &format!("en1992.annexC.ductility.{}", m.id)))
        .max_by(|a, b| a.utilization.total_cmp(&b.utilization))
}

/// 📚️ The Table 3.1 row of a concrete class from M-02: `[f_ck, f_cm, f_ctm, f_ctk,0.05, f_ctk,0.95]` in MPa and `E_cm` in GPa; `None` outside the scope of the table.
pub fn concrete_row(document: &En1992Snapshot, f_ck_mpa: f64) -> Option<[f64; 6]> {
    let o = checks::evaluate_outcomes("M-02", &Case::new("m-02-sweep", document.annex).with_n("fck", f_ck_mpa)).ok()?;
    let mpa = |key: &str| value(&o, key).map(|v| v * 1.0e-6);
    Some([f_ck_mpa, mpa("fcm")?, mpa("fctm")?, mpa("fctk05")?, mpa("fctk95")?, value(&o, "ecm")? * 1.0e-9])
}

fn grade_diagram(id: &str, key: &str, title: LocalizedCopy, description: LocalizedCopy, axes: Vec<Axis>, collection: &str) -> Diagram2d {
    Diagram2d {
        id: format!("{key}:{id}"),
        title,
        description,
        axes,
        series: Vec::new(),
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(entity_ref(EntityKind::Group, id, &format!("{collection}[id={id}]"))),
        legend: true,
        table_twin: DataTable { columns: Vec::new() },
    }
}

/// 📈️ Diagram `<grade>/concrete-properties`: `f_ck`, `f_cm`, `f_ctm`, `f_ctk,0.05` and `f_ctk,0.95` over the classes of Table 3.1 with the grade point; the grip on the `f_ck` point drags `fCk`; `None` outside the table.
pub fn concrete_diagram(document: &En1992Snapshot, report: &CheckReport, grade: &ConcreteGrade) -> Option<Diagram2d> {
    let rows: Vec<[f64; 6]> = CLASSES.iter().filter_map(|c| concrete_row(document, *c)).collect();
    let own = concrete_row(document, grade.f_ck * 1.0e-6)?;
    if rows.len() < 3 {
        return None;
    }
    let line = |i: usize| xy(&rows.iter().map(|r| [r[0], r[i]]).collect::<Vec<_>>());
    let mut g = grade_diagram(
        &grade.id,
        "concrete-properties",
        loc(&format!("{} - Table 3.1 properties", grade.name), &format!("{} - Kennwerte nach Tabelle 3.1", grade.name)),
        loc("Mean and characteristic strengths of the concrete classes of EN 1992-1-1 Table 3.1 (catalogue entry M-02) with the grade of the document; the circle on the f_ck line drags the characteristic strength.", "Mittel- und charakteristische Festigkeiten der Betonklassen nach EN 1992-1-1 Tabelle 3.1 (Katalogeintrag M-02) mit der Klasse des Dokuments; der Kreis auf der f_ck-Linie verschiebt die charakteristische Festigkeit."),
        vec![axis("Characteristic cylinder strength f_ck", "Charakteristische Zylinderdruckfestigkeit f_ck", QuantityKind::Stress, "MPa"), axis("Strength", "Festigkeit", QuantityKind::Stress, "MPa")],
        "concreteGrades",
    );
    for (i, key, en, de, dash) in [(0, "fck", "f_ck", "f_ck", Dash::Dotted), (1, "fcm", "f_cm", "f_cm", Dash::Solid), (2, "fctm", "f_ctm", "f_ctm", Dash::Solid), (3, "fctk05", "f_ctk,0.05", "f_ctk,0,05", Dash::Dashed), (4, "fctk95", "f_ctk,0.95", "f_ctk,0,95", Dash::Dashed)] {
        g.series.push(Series::Polyline { id: key.into(), label: loc(en, de), points: line(i), style: stroke(if i == 1 || i == 2 { 2.0 } else { 1.5 }, dash, Tone::Info) });
    }
    let (fcm, fctm) = (check_of(report, &format!("en1992.m-02.fcm.{}", grade.id)), check_of(report, &format!("en1992.m-02.fctm.{}", grade.id)));
    let point = |id: &str, y: f64, shape: MarkerShape, check: Option<&CheckResult>, en: String, de: String, binding: Option<FieldBinding>| {
        let (tone, bound_check) = tone_and_id(check);
        Marker { id: id.into(), at: Vec2::new(own[0], y), shape, tone, label: loc(&en, &de), utilisation: utilisation_of(check), bound_check, binding }
    };
    let grip = FieldBinding { path: format!("concreteGrades[id={}].fCk", grade.id), map: Affine { base: 0.0, k: 1.0e6 }, unit: "Pa".into(), quantise: Some(Quantise::Round { step: 1.0e5 }) };
    g.markers.push(point("fck-grip", own[0], MarkerShape::Circle, None, format!("{}: f_ck = {:.0} MPa", grade.name, own[0]), format!("{}: f_ck = {:.0} MPa", grade.name, own[0]), Some(grip)));
    g.markers.push(point("fcm", own[1], MarkerShape::Diamond, fcm, format!("f_cm = f_ck + 8 = {:.1} MPa", own[1]), format!("f_cm = f_ck + 8 = {:.1} MPa", own[1]), None));
    g.markers.push(point("fctm", own[2], MarkerShape::Triangle, fctm, format!("f_ctm = {:.2} MPa, E_cm = {:.1} GPa", own[2], own[5]), format!("f_ctm = {:.2} MPa, E_cm = {:.1} GPa", own[2], own[5]), None));
    let col = |en: &str, de: &str, unit: &str, i: usize| column(en, de, unit, rows.iter().map(|r| r[i]).collect());
    g.table_twin = DataTable { columns: vec![col("f_ck", "f_ck", "MPa", 0), col("f_cm", "f_cm", "MPa", 1), col("f_ctm", "f_ctm", "MPa", 2), col("f_ctk,0.05", "f_ctk,0,05", "MPa", 3), col("f_ctk,0.95", "f_ctk,0,95", "MPa", 4), col("E_cm", "E_cm", "GPa", 5)] };
    Some(g)
}

/// 🧮️ The steel stress in MPa at the strain `eps` (permille) of M-08 for the grade, on the inclined (`1`) or the horizontal (`0`) top branch.
pub fn steel_stress(document: &En1992Snapshot, grade: &ReinforcementGrade, eps: f64, inclined: f64) -> Option<f64> {
    let case = Case::new("m-08-law", document.annex).with_n("fyk", grade.f_yk * 1.0e-6).with_n("es", grade.e_s * 1.0e-6).with_n("k", grade.k).with_n("eps_uk", grade.eps_uk * 1.0e3).with_n("inclined", inclined).with_n("eps", eps);
    checks::evaluate_outcomes("M-08", &case).ok().and_then(|o| value(&o, "sigma_s")).map(|v| v * 1.0e-6)
}

/// 🧮️ The design yield strain `ε_yd` and the design limit strain `ε_ud = 0.9 ε_uk` of M-08 in permille.
pub fn steel_strains(document: &En1992Snapshot, grade: &ReinforcementGrade) -> Option<(f64, f64)> {
    let case = Case::new("m-08-law", document.annex).with_n("fyk", grade.f_yk * 1.0e-6).with_n("es", grade.e_s * 1.0e-6).with_n("k", grade.k).with_n("eps_uk", grade.eps_uk * 1.0e3).with_n("eps", 0.0);
    let o = checks::evaluate_outcomes("M-08", &case).ok()?;
    Some((value(&o, "eps_yd")?, value(&o, "eps_ud")?))
}

/// 📈️ Diagram `<grade>/steel-law`: the design stress-strain diagram of EN 1992-1-1 3.2.7 with the inclined and the horizontal top branch, the yield point, the design limit strain `ε_ud = 0.9 ε_uk` and the Table C.1 limits of the declared class (`k_min f_yd`, `ε_uk,min`) as limit lines.
pub fn steel_diagram(document: &En1992Snapshot, report: &CheckReport, grade: &ReinforcementGrade) -> Option<Diagram2d> {
    if grade.f_yk <= 0.0 || grade.e_s <= 0.0 || grade.eps_uk <= 0.0 {
        return None;
    }
    let (eps_yd, eps_ud) = steel_strains(document, grade)?;
    let (f_yd, top, flat) = (steel_stress(document, grade, eps_yd, 1.0)?, steel_stress(document, grade, eps_ud, 1.0)?, steel_stress(document, grade, eps_ud, 0.0)?);
    let (k_min, eps_min) = grade.ductility.table_c1_minima();
    let mut g = grade_diagram(
        &grade.id,
        "steel-law",
        loc(&format!("{} - reinforcing steel diagram", grade.name), &format!("{} - Spannungs-Dehnungs-Linie Betonstahl", grade.name)),
        loc("Design stress-strain diagram of the reinforcing steel (EN 1992-1-1 3.2.7, catalogue entry M-08) with the inclined top branch up to k f_yd at ε_ud = 0.9 ε_uk, the horizontal alternative, and the Annex C Table C.1 minima of the declared ductility class as limit lines.", "Bemessungs-Spannungs-Dehnungs-Linie des Betonstahls (EN 1992-1-1 3.2.7, Katalogeintrag M-08) mit geneigtem oberem Ast bis k f_yd bei ε_ud = 0,9 ε_uk, der horizontalen Alternative und den Mindestwerten nach Anhang C Tabelle C.1 der angegebenen Duktilitätsklasse als Grenzlinien."),
        vec![axis("Strain ε", "Dehnung ε", QuantityKind::Dimensionless, "‰"), axis("Stress σ_s", "Spannung σ_s", QuantityKind::Stress, "MPa")],
        "reinforcementGrades",
    );
    g.series.push(Series::Polyline { id: "inclined".into(), label: loc("Inclined top branch", "Geneigter oberer Ast"), points: xy(&[[0.0, 0.0], [eps_yd, f_yd], [eps_ud, top]]), style: stroke(2.0, Dash::Solid, Tone::Info) });
    g.series.push(Series::Polyline { id: "horizontal".into(), label: loc("Horizontal top branch", "Horizontaler oberer Ast"), points: xy(&[[0.0, 0.0], [eps_yd, f_yd], [eps_ud, flat]]), style: stroke(1.5, Dash::Dashed, Tone::Neutral) });
    g.series.push(Series::Limit { id: "k-min".into(), label: loc(&format!("k_min f_yd = {:.2} f_yd (Table C.1)", k_min), &format!("k_min f_yd = {:.2} f_yd (Tabelle C.1)", k_min)), axis: AxisRef::Y, value: k_min * f_yd, style: stroke(1.0, Dash::Dotted, Tone::Warning) });
    g.series.push(Series::Limit { id: "eps-uk-min".into(), label: loc(&format!("ε_uk,min = {:.1} % (Table C.1)", eps_min * 100.0), &format!("ε_uk,min = {:.1} % (Tabelle C.1)", eps_min * 100.0)), axis: AxisRef::X, value: eps_min * 1.0e3, style: stroke(1.0, Dash::Dotted, Tone::Warning) });
    let yield_check = check_of(report, &format!("en1992.m-08.sigma_s.{}", grade.id));
    let ductility = ductility_check(document, report, &grade.id);
    let (yield_tone, yield_id) = tone_and_id(yield_check);
    let (tone, bound_check) = tone_and_id(ductility);
    g.markers.push(Marker { id: "yield".into(), at: Vec2::new(eps_yd, f_yd), shape: MarkerShape::Triangle, tone: yield_tone, label: loc(&format!("ε_yd = {eps_yd:.2} ‰, f_yd = {f_yd:.0} MPa"), &format!("ε_yd = {eps_yd:.2} ‰, f_yd = {f_yd:.0} MPa")), utilisation: utilisation_of(yield_check), bound_check: yield_id, binding: None });
    g.markers.push(Marker { id: "ultimate".into(), at: Vec2::new(eps_ud, top), shape: MarkerShape::Diamond, tone, label: loc(&format!("ε_ud = 0.9 ε_uk = {eps_ud:.1} ‰, k f_yd = {top:.0} MPa"), &format!("ε_ud = 0,9 ε_uk = {eps_ud:.1} ‰, k f_yd = {top:.0} MPa")), utilisation: utilisation_of(ductility), bound_check, binding: None });
    g.table_twin = DataTable { columns: vec![
        column("Strain", "Dehnung", "‰", vec![0.0, eps_yd, eps_ud]),
        column("Stress, inclined branch", "Spannung, geneigter Ast", "MPa", vec![0.0, f_yd, top]),
        column("Stress, horizontal branch", "Spannung, horizontaler Ast", "MPa", vec![0.0, f_yd, flat]),
    ] };
    Some(g)
}

/// 📈️ Diagram `<grade>/ductility-class`: the plane of `k = (f_t/f_y)k` against `ε_uk` with the nested regions of the classes A, B and C of Table C.1 and the grade point, toned by the Annex C check of the members that use the grade; the point drags `k`.
pub fn ductility_diagram(document: &En1992Snapshot, report: &CheckReport, grade: &ReinforcementGrade) -> Option<Diagram2d> {
    if grade.k <= 0.0 || grade.eps_uk <= 0.0 {
        return None;
    }
    let (x_top, y_top) = ((grade.k * 1.05).max(1.3), (grade.eps_uk * 120.0).max(10.0));
    let mut g = grade_diagram(
        &grade.id,
        "ductility-class",
        loc(&format!("{} - ductility class", grade.name), &format!("{} - Duktilitätsklasse", grade.name)),
        loc("Annex C: a reinforcement grade meets a ductility class when k = (f_t/f_y)k and ε_uk lie in its region of Table C.1; the point is the grade of the document and drags k.", "Anhang C: Eine Betonstahlsorte erfüllt eine Duktilitätsklasse, wenn k = (f_t/f_y)k und ε_uk in deren Bereich nach Tabelle C.1 liegen; der Punkt ist die Sorte des Dokuments und verschiebt k."),
        vec![axis("k = (f_t/f_y)k", "k = (f_t/f_y)k", QuantityKind::Dimensionless, "1"), axis("Strain at maximum force ε_uk", "Dehnung bei Höchstkraft ε_uk", QuantityKind::Dimensionless, "%")],
        "reinforcementGrades",
    );
    for class in DUCTILITY {
        let (k_min, eps_min) = class.table_c1_minima();
        let ring = vec![Vec2::new(k_min, eps_min * 100.0), Vec2::new(x_top, eps_min * 100.0), Vec2::new(x_top, y_top), Vec2::new(k_min, y_top)];
        g.series.push(Series::Polygon { id: format!("class-{class:?}"), label: loc(&format!("Class {class:?}: k >= {k_min:.2}, ε_uk >= {:.1} %", eps_min * 100.0), &format!("Klasse {class:?}: k >= {k_min:.2}, ε_uk >= {:.1} %", eps_min * 100.0)), ring, fill: Fill { tone: Tone::Info, opacity: 0.12 }, hatch: None });
    }
    let ductility = ductility_check(document, report, &grade.id);
    let (tone, bound_check) = tone_and_id(ductility);
    let grip = FieldBinding { path: format!("reinforcementGrades[id={}].k", grade.id), map: Affine { base: 0.0, k: 1.0 }, unit: "1".into(), quantise: Some(Quantise::Round { step: 0.01 }) };
    g.markers.push(Marker { id: "grade".into(), at: Vec2::new(grade.k, grade.eps_uk * 100.0), shape: MarkerShape::Diamond, tone, label: loc(&format!("{}: k = {:.2}, ε_uk = {:.1} %", grade.name, grade.k, grade.eps_uk * 100.0), &format!("{}: k = {:.2}, ε_uk = {:.1} %", grade.name, grade.k, grade.eps_uk * 100.0)), utilisation: utilisation_of(ductility), bound_check, binding: Some(grip) });
    let rows: Vec<(f64, f64)> = DUCTILITY.iter().map(|c| c.table_c1_minima()).collect();
    let met = |class: &DuctilityClass| {
        let (k_min, eps_min) = class.table_c1_minima();
        f64::from(u8::from(grade.k >= k_min && grade.eps_uk >= eps_min))
    };
    g.table_twin = DataTable { columns: vec![
        column("Class (1 = A, 2 = B, 3 = C)", "Klasse (1 = A, 2 = B, 3 = C)", "1", vec![1.0, 2.0, 3.0]),
        column("k minimum", "k Mindestwert", "1", rows.iter().map(|r| r.0).collect()),
        column("ε_uk minimum", "ε_uk Mindestwert", "%", rows.iter().map(|r| r.1 * 100.0).collect()),
        column("Grade k", "Sorte k", "1", vec![grade.k; 3]),
        column("Grade ε_uk", "Sorte ε_uk", "%", vec![grade.eps_uk * 100.0; 3]),
        column("Class met (1 = yes)", "Klasse erfüllt (1 = ja)", "1", DUCTILITY.iter().map(met).collect()),
    ] };
    Some(g)
}

/// 📈️ The three diagrams of every grade row that carries them.
pub fn diagrams(input: &SceneInput) -> Vec<Diagram2d> {
    let (document, report) = (input.document, input.report);
    let concrete = document.concrete_grades.iter().filter_map(|g| concrete_diagram(document, report, g));
    let steel = document.reinforcement_grades.iter().flat_map(|g| [steel_diagram(document, report, g), ductility_diagram(document, report, g)]).flatten();
    concrete.chain(steel).collect()
}

/// 🏷️ The 3D labels of the grades of a member beside its grade field: the Table 3.1 values of the concrete (toned by M-02) and the Annex C verdict of the reinforcement (toned by the member's ductility check).
pub fn annotate(input: &SceneInput, m: &RcMember, anchor: &Vec3, out: &mut Out) {
    let document = input.document;
    if let Some(grade) = document.concrete(&m.concrete_grade_id) {
        if let Some(row) = concrete_row(document, grade.f_ck * 1.0e-6) {
            let tone = tone_of(&binding(input.report, &grade.id, &["en1992.m-02.", "en1992.m-03."]));
            let text = loc(&format!("{}: f_cm = {:.0} MPa, f_ctm = {:.1} MPa, E_cm = {:.0} GPa (Table 3.1)", grade.name, row[1], row[2], row[5]), &format!("{}: f_cm = {:.0} MPa, f_ctm = {:.1} MPa, E_cm = {:.0} GPa (Tabelle 3.1)", grade.name, row[1], row[2], row[5]));
            out.annotate(Annotation::Label { id: format!("{}:grade-concrete-label", m.id), anchor: anchor.clone(), text, tone }, anchor);
        }
    }
    if let Some(grade) = document.reinforcement(&m.reinforcement_grade_id) {
        let (k_min, eps_min) = grade.ductility.table_c1_minima();
        let tone = tone_of(&binding(input.report, &m.id, &["en1992.annexC."]).or_else(|| binding(input.report, &grade.id, &["en1992.m-08."])));
        let up = add(anchor, &v3(0.0, 0.0, 0.1));
        let text = loc(&format!("{}: class {:?}, k = {:.2} (min {k_min:.2}), ε_uk = {:.1} % (min {:.1} %)", grade.name, grade.ductility, grade.k, grade.eps_uk * 100.0, eps_min * 100.0), &format!("{}: Klasse {:?}, k = {:.2} (min {k_min:.2}), ε_uk = {:.1} % (min {:.1} %)", grade.name, grade.ductility, grade.k, grade.eps_uk * 100.0, eps_min * 100.0));
        out.annotate(Annotation::Label { id: format!("{}:grade-ductility-label", m.id), anchor: up.clone(), text, tone }, &up);
    }
}
