//! 🎨️ Grades: the strength grades of every member as a colour field on its side face, concrete `f_ck` and reinforcement `f_yk` in MPa, with one scalar legend per field. The layer is hidden by default; showing it is the view option, and the field colour follows the grade table of the document.

use super::kit::*;
use super::layout::Layout;
use super::model::{En1992Snapshot, MemberKind};
use super::scene::*;
use super::tracks::{side_grid, spec};
use super::SceneInput;

pub const CONCRETE_FIELD: &str = "grade-fck";
pub const REINFORCEMENT_FIELD: &str = "grade-fyk";
const OFFSET: f64 = 0.012;

fn range(values: impl Iterator<Item = f64>) -> (f64, f64) {
    let (lo, hi) = values.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    if lo.is_finite() && hi > lo { (lo, hi) } else { (lo.min(hi).max(0.0), lo.max(hi).max(1.0) + 1.0) }
}

/// 📐️ The legend domains in MPa: the concrete and the reinforcement grade table of the document, so one colour always names the same grade.
pub fn domains(document: &En1992Snapshot) -> [(f64, f64); 2] {
    [range(document.concrete_grades.iter().map(|g| g.f_ck / 1.0e6)), range(document.reinforcement_grades.iter().map(|g| g.f_yk / 1.0e6))]
}

/// 🎨️ Adds the grade field entity of every member whose concrete and reinforcement grade resolve, and the two scalar legends once.
pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    let titles = [(CONCRETE_FIELD, "Concrete grade f_ck (MPa)", "Betongüte f_ck (MPa)"), (REINFORCEMENT_FIELD, "Reinforcement grade f_yk (MPa)", "Bewehrungsgüte f_yk (MPa)")];
    for m in &d.members {
        if !out.wants(&m.id) {
            continue;
        }
        let (Some(pose), Some(concrete), Some(steel)) = (layout.member(&m.id), d.concrete(&m.concrete_grade_id), d.reinforcement(&m.reinforcement_grade_id)) else { continue };
        if m.span <= 1e-6 || m.height <= 1e-6 || m.width <= 1e-6 {
            continue;
        }
        let width = if m.kind == MemberKind::FlatSlab { m.span } else { m.width };
        let (points, grid) = side_grid(&pose.frame, m.span, width, m.height, OFFSET);
        let uniform = |value: f64| FieldSamples::Vertex { values: vec![value; points.len()] };
        let scalars = vec![
            ScalarField { id: CONCRETE_FIELD.into(), unit: QuantityKind::Stress, samples: uniform(concrete.f_ck / 1.0e6) },
            ScalarField { id: REINFORCEMENT_FIELD.into(), unit: QuantityKind::Stress, samples: uniform(steel.f_yk / 1.0e6) },
        ];
        let detail = (format!("concrete {} and reinforcement {}", concrete.id, steel.id), format!("Beton {} und Bewehrung {}", concrete.id, steel.id));
        out.push(spec(m, "grade-field", "concreteGradeId", grid, LAYER_GRADES, [("strength grades", "Festigkeitsklassen"), (&detail.0, &detail.1)], scalars), &points);
        super::material_laws::annotate(input, m, &at(&pose.frame, m.span / 2.0, 0.0, m.height / 2.0 + OFFSET), out);
        for ((id, en, de), (min, max)) in titles.iter().zip(domains(d)) {
            if !out.legends.iter().any(|l| l.id == *id) {
                out.legends.push(Legend { id: (*id).into(), title: loc(en, de), scale: LegendScale::Scalar { field_id: (*id).into(), map: ColourMapRef { id: "viridis".into(), min, max }, unit: QuantityKind::Stress } });
            }
        }
    }
}

/// 🎨️ The grade legend diagram: the strength of every concrete (f_ck), reinforcement (f_yk) and prestressing grade of the document as a bar, hatched where a member uses it, with a diamond per bar that drags the strength leaf; the table twin lists id, strength and the number of members using it.
pub fn legend_diagram(document: &En1992Snapshot) -> Option<Diagram2d> {
    use super::scene::projection_kit::diagram_parts::column;
    let used = |select: &dyn Fn(&super::model::RcMember) -> &str, id: &str| document.members.iter().filter(|m| select(m) == id).count();
    let groups: [(&str, &str, &str, Tone, &str, Vec<(String, String, f64, usize)>); 3] = [
        ("concreteGrades", "fCk", "f_ck", Tone::Neutral, "Concrete", document.concrete_grades.iter().map(|g| (g.id.clone(), g.name.clone(), g.f_ck / 1.0e6, used(&|m| m.concrete_grade_id.as_str(), &g.id))).collect()),
        ("reinforcementGrades", "fYk", "f_yk", Tone::Info, "Reinforcement", document.reinforcement_grades.iter().map(|g| (g.id.clone(), g.name.clone(), g.f_yk / 1.0e6, used(&|m| m.reinforcement_grade_id.as_str(), &g.id))).collect()),
        ("prestressSteels", "fPk", "f_pk", Tone::Warning, "Prestressing steel", document.prestress_steels.iter().map(|g| (g.id.clone(), g.name.clone(), g.f_pk / 1.0e6, used(&|m| m.prestress_steel_id.as_str(), &g.id))).collect()),
    ];
    let total: usize = groups.iter().map(|g| g.5.len()).sum();
    if total == 0 {
        return None;
    }
    let top = groups.iter().flat_map(|g| g.5.iter().map(|r| r.2)).fold(1.0, f64::max) * 1.15;
    let (mut series, mut markers, mut notes) = (Vec::new(), Vec::new(), Vec::new());
    let mut table = [Vec::new(), Vec::new(), Vec::new()];
    let mut x = 0.0;
    for (collection, leaf, symbol, tone, _, rows) in &groups {
        for (id, name, strength, members) in rows {
            series.push(Series::Polygon { id: format!("grade-{collection}-{id}"), label: loc(&format!("{name}: {symbol} = {strength:.0} MPa, {members} members"), &format!("{name}: {symbol} = {strength:.0} MPa, {members} Bauteile")), ring: vec![Vec2::new(x + 0.1, 0.0), Vec2::new(x + 0.9, 0.0), Vec2::new(x + 0.9, *strength), Vec2::new(x + 0.1, *strength)], fill: Fill { tone: *tone, opacity: 0.6 }, hatch: (*members > 0).then_some(Hatch::Diagonal) });
            notes.push(DiagramNote { id: format!("name-{collection}-{id}"), at: Vec2::new(x + 0.5, -0.05 * top), text: loc(name, name), tone: Tone::Neutral });
            markers.push(Marker { id: format!("reference:tip-{collection}-{id}"), at: Vec2::new(x + 0.5, *strength), shape: MarkerShape::Diamond, tone: *tone, label: loc(&format!("{name} {symbol} = {strength:.0} MPa"), &format!("{name} {symbol} = {strength:.0} MPa")), utilisation: None, bound_check: None, binding: Some(FieldBinding { path: format!("{collection}[id={id}].{leaf}"), map: Affine { base: 0.0, k: 1.0e6 }, unit: "Pa".into(), quantise: Some(Quantise::Round { step: 1.0e5 }) }) });
            for (column, value) in table.iter_mut().zip([x, *strength, *members as f64]) {
                column.push(value);
            }
            x += 1.0;
        }
        x += 0.5;
    }
    let [index, strength, members] = table;
    Some(Diagram2d {
        id: "grades:document".into(),
        title: loc("Material grades", "Werkstoffklassen"),
        description: loc("The characteristic strength of every concrete grade (f_ck), reinforcement grade (f_yk) and prestressing steel (f_pk); hatched bars are used by a member, the diamonds drag the strength.", "Die charakteristische Festigkeit jeder Betonklasse (f_ck), Bewehrungsklasse (f_yk) und jedes Spannstahls (f_pk); schraffierte Balken werden von einem Bauteil verwendet, die Rauten verschieben die Festigkeit."),
        axes: vec![super::axis("Grade", "Klasse", QuantityKind::Dimensionless, "1"), super::axis("Strength", "Festigkeit", QuantityKind::Stress, "MPa")],
        series,
        markers,
        annotations: notes,
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Position", "Lage", "1", index), column("Strength", "Festigkeit", "MPa", strength), column("Members using it", "Verwendende Bauteile", "1", members)] },
    })
}
