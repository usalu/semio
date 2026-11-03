//! 📊️ Check ledger: one diagram per document entity that owns checks, the utilisation of every check addressed to it against the limit of 100 %, one marker per check bound to its id. It binds every check family that has no picture of its own (serviceability stresses 7.2, exposure and scope 4.2 and 3.1, material and national-annex rows, the entity cases of the grade rows) to a visual of its entity, and it is the fallback for new families. See [`super`].

use super::kit::*;
use super::model::En1992Snapshot;
use super::scene::*;
use super::SceneInput;

fn ledger(input: &SceneInput, id: &str, title: LocalizedCopy, description: LocalizedCopy, focus: EntityRef) -> Option<Diagram2d> {
    let mut d = projection_kit::design_case::case_diagram(id, &title, input.report, None)?;
    d.id = format!("check-ledger:{id}");
    d.description = description;
    d.focus = Some(focus);
    Some(d)
}

/// 📊️ Diagram `check-ledger:<member>` of a member; `None` without checks.
pub fn member(input: &SceneInput, m: &super::model::RcMember, kind: EntityKind, path: &str) -> Option<Diagram2d> {
    ledger(
        input,
        &m.id,
        loc(&format!("{} - check ledger", m.label_en), &format!("{} - Nachweisübersicht", m.label_de)),
        loc("Utilisation of every check addressed to the member against the limit of 100 %; each marker is bound to its check.", "Ausnutzung jedes Nachweises des Bauteils gegen die Grenze von 100 %; jeder Marker ist an seinen Nachweis gebunden."),
        entity_ref(kind, &m.id, path),
    )
}

/// 📊️ The ledgers of the grade rows (concrete, reinforcement, prestressing steel) and of the document (national annex).
pub fn rows(input: &SceneInput) -> Vec<Diagram2d> {
    let d: &En1992Snapshot = input.document;
    let grades = d.concrete_grades.iter().map(|g| (g.id.as_str(), g.name.as_str(), "concreteGrades")).chain(d.reinforcement_grades.iter().map(|g| (g.id.as_str(), g.name.as_str(), "reinforcementGrades"))).chain(d.prestress_steels.iter().map(|g| (g.id.as_str(), g.name.as_str(), "prestressSteels")));
    let mut out: Vec<Diagram2d> = grades
        .filter_map(|(id, name, collection)| ledger(input, id, loc(&format!("{name} - check ledger"), &format!("{name} - Nachweisübersicht")), loc("Utilisation of every material check of the grade row against the limit of 100 %; each marker is bound to its check.", "Ausnutzung jedes Werkstoffnachweises der Klassenzeile gegen die Grenze von 100 %; jeder Marker ist an seinen Nachweis gebunden."), entity_ref(EntityKind::Group, id, &format!("{collection}[id={id}]"))))
        .collect();
    out.extend(ledger(input, "annex", loc("National annex - check ledger", "Nationaler Anhang - Nachweisübersicht"), loc("Utilisation of the document-level checks (national annex edition) against the limit of 100 %.", "Ausnutzung der Nachweise auf Dokumentebene (Ausgabe des Nationalen Anhangs) gegen die Grenze von 100 %."), entity_ref(EntityKind::Group, "annex", "annex")));
    out
}
