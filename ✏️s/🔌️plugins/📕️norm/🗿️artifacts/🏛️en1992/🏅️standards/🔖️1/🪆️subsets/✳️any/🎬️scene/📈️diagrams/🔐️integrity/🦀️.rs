//! 🔐️ Model integrity (`en1992.integrity.*`): the references of every member to its concrete grade, reinforcement grade and prestressing steel, resolved against the grade tables, as one matrix diagram (a marker per reference, error where it dangles or is orphaned, bound to the integrity check when the report has one) and as an error label on the member in 3D. The structural integrity ties of EN 1992-1-1 9.10 are the DT-14 cases drawn by [`super::ties`]. See [`super`].

use super::axis;
use super::kit::*;
use super::model::{En1992Snapshot, RcMember};
use super::scene::projection_kit::diagram_parts::column;
use super::scene::*;
use super::SceneInput;

/// 🔗️ How one reference of a member resolves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Resolved,
    Missing,
    NotDeclared,
}

/// 🔗️ One reference of a member: what it points to, the table it points into and whether it resolves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    pub table: &'static str,
    pub target: String,
    pub state: State,
}

/// 🧮️ The concrete grade, reinforcement grade and prestressing steel references of a member, in that order: a prestress block needs a declared steel, a steel id without a block is orphaned.
pub fn links(document: &En1992Snapshot, m: &RcMember) -> [Link; 3] {
    let known = |found: bool| if found { State::Resolved } else { State::Missing };
    let prestress = match (&m.prestress, m.prestress_steel_id.is_empty()) {
        (None, true) => State::NotDeclared,
        (Some(_), _) => known(document.prestress_steels.iter().any(|s| s.id == m.prestress_steel_id)),
        (None, false) => State::Missing,
    };
    [
        Link { table: "concreteGrades", target: m.concrete_grade_id.clone(), state: known(document.concrete(&m.concrete_grade_id).is_some()) },
        Link { table: "reinforcementGrades", target: m.reinforcement_grade_id.clone(), state: known(document.reinforcement(&m.reinforcement_grade_id).is_some()) },
        Link { table: "prestressSteels", target: m.prestress_steel_id.clone(), state: prestress },
    ]
}

fn kind_names(table: &str) -> (&'static str, &'static str) {
    match table {
        "concreteGrades" => ("concrete grade", "Betonklasse"),
        "reinforcementGrades" => ("reinforcement grade", "Bewehrungsklasse"),
        _ => ("prestressing steel", "Spannstahl"),
    }
}

/// 📈️ Diagram `integrity`: a matrix of the references of all members (columns concrete grade, reinforcement grade, prestressing steel; rows the members) with an info marker where the reference resolves and an error marker where it dangles; `None` without members.
pub fn diagram(input: &SceneInput) -> Option<Diagram2d> {
    let document = input.document;
    if document.members.is_empty() {
        return None;
    }
    let mut markers = Vec::new();
    let mut columns: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (row, m) in document.members.iter().enumerate() {
        for (col, link) in links(document, m).iter().enumerate() {
            columns[col].push(f64::from(u8::from(link.state != State::Missing)));
            if link.state == State::NotDeclared {
                continue;
            }
            let (en, de) = kind_names(link.table);
            let ok = link.state == State::Resolved;
            let check = [format!("en1992.integrity.prestressSteel.{}", m.id), format!("en1992.integrity.prestressSteel.orphan.{}", m.id)].into_iter().find(|id| col == 2 && input.report.checks.iter().any(|c| &c.id == id));
            markers.push(Marker {
                id: format!("ref-{}-{col}", m.id),
                at: Vec2::new(col as f64 + 1.0, row as f64 + 1.0),
                shape: MarkerShape::Square,
                tone: if ok { Tone::Info } else { Tone::Error },
                label: loc(&format!("{} to {en} '{}': {}", m.id, link.target, if ok { "resolved" } else { "missing" }), &format!("{} auf {de} '{}': {}", m.id, link.target, if ok { "aufgelöst" } else { "fehlt" })),
                utilisation: None,
                bound_check: check,
                binding: None,
            });
        }
    }
    let [concrete, reinforcement, prestress] = columns;
    let rows = (1..=document.members.len()).map(|i| i as f64).collect();
    Some(Diagram2d {
        id: "integrity:document".into(),
        title: loc("Model integrity", "Modellintegrität"),
        description: loc("The references of every member to its concrete grade, reinforcement grade and prestressing steel; an error marker is a reference that dangles or is orphaned (check family en1992.integrity).", "Die Verweise jedes Bauteils auf Betonklasse, Bewehrungsklasse und Spannstahl; ein Fehlermarker ist ein hängender oder verwaister Verweis (Nachweisfamilie en1992.integrity)."),
        axes: vec![axis("Reference (1 concrete, 2 reinforcement, 3 prestressing steel)", "Verweis (1 Beton, 2 Bewehrung, 3 Spannstahl)", QuantityKind::Dimensionless, "1"), axis("Member", "Bauteil", QuantityKind::Dimensionless, "1")],
        series: Vec::new(),
        markers,
        annotations: Vec::new(),
        focus: None,
        legend: false,
        table_twin: DataTable { columns: vec![
            column("Member", "Bauteil", "1", rows),
            column("Concrete grade resolves (1 = yes)", "Betonklasse aufgelöst (1 = ja)", "1", concrete),
            column("Reinforcement grade resolves (1 = yes)", "Bewehrungsklasse aufgelöst (1 = ja)", "1", reinforcement),
            column("Prestressing steel resolves or not declared (1 = yes)", "Spannstahl aufgelöst oder nicht angegeben (1 = ja)", "1", prestress),
        ] },
    })
}

/// 🏷️ The error label of a member whose references dangle, at the member start, toned by the integrity checks of the report.
pub fn annotate(input: &SceneInput, m: &RcMember, anchor: &Vec3, out: &mut Out) {
    let broken: Vec<Link> = links(input.document, m).into_iter().filter(|l| l.state == State::Missing).collect();
    if broken.is_empty() {
        return;
    }
    let list = |german: bool| broken.iter().map(|l| format!("{} '{}'", if german { kind_names(l.table).1 } else { kind_names(l.table).0 }, l.target)).collect::<Vec<_>>().join(", ");
    let seen = binding(input.report, &m.id, &["en1992.integrity."]);
    let tone = if seen.is_some() { tone_of(&seen) } else { Tone::Error };
    out.annotate(Annotation::Label { id: format!("{}:integrity-label", m.id), anchor: anchor.clone(), text: loc(&format!("Missing references: {}", list(false)), &format!("Fehlende Verweise: {}", list(true))), tone }, anchor);
}
