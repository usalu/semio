//! 🧰️ Report helpers of the EN 1990 compliance report: the copy, quantity and subject helpers and the action set per member. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
pub(super) fn copy(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(super) fn q_force(n: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, n)
}

pub(super) fn q_len(m: f64) -> Quantity {
    Quantity::new(QuantityKind::Length, m)
}

pub(super) fn q_dim(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}

pub(super) fn member_label(member: &Member) -> LocalizedCopy {
    copy(member.label_en.clone(), member.label_de.clone())
}

pub(super) fn project_subject(document: &En1990Snapshot) -> SubjectRef {
    SubjectRef::new(
        document.project_id.clone(),
        "projectId",
        copy(
            format!("Project {}", document.project_id),
            format!("Projekt {}", document.project_id),
        ),
    )
}

pub(super) fn str_title(section: &str) -> LocalizedCopy {
    copy(
        format!("STR {section} — ultimate limit state"),
        format!("STR Gl. {section} — Grenzzustand der Tragfähigkeit"),
    )
}

pub(super) fn geo_title(section: &str) -> LocalizedCopy {
    copy(
        format!("GEO {section} — geotechnical ultimate limit state"),
        format!("GEO Gl. {section} — geotechnischer Grenzzustand der Tragfähigkeit"),
    )
}

fn member_index(document: &En1990Snapshot, member_id: &str) -> usize {
    document.members.iter().position(|m| m.id == member_id).unwrap_or(0)
}

pub(super) fn member_path(_document: &En1990Snapshot, member_id: &str, field: &str) -> String {
    format!("members[id={member_id}].{field}")
}

fn influence_for(effects: &[MemberEffect], member_id: &str, action_id: &str) -> f64 {
    effects.iter().find(|e| e.member_id == member_id && e.action_id == action_id).map(|e| e.influence).unwrap_or(0.0)
}

pub(crate) fn action_set_for_member(document: &En1990Snapshot, member_id: &str) -> ActionSet {
    let g_k: f64 = document
        .permanents
        .iter()
        .filter(|p| p.kind == "g_sup")
        .map(|p| influence_for(&document.effects, member_id, &p.id) * p.gk)
        .sum();
    let g_inf: f64 = document
        .permanents
        .iter()
        .filter(|p| p.kind == "g_inf")
        .map(|p| influence_for(&document.effects, member_id, &p.id) * p.gk)
        .sum();
    let p_k: f64 = document
        .permanents
        .iter()
        .filter(|p| p.kind == "prestress")
        .map(|p| influence_for(&document.effects, member_id, &p.id) * p.gk)
        .sum();
    let q_k: Vec<(String, f64)> = document
        .variables
        .iter()
        .map(|v| {
            let cat = resolve_psi_category(document.annex, &v.category, document.altitude_m);
            (cat, influence_for(&document.effects, member_id, &v.id) * v.qk)
        })
        .collect();
    let a_d: f64 = document.accidentals.iter().map(|a| influence_for(&document.effects, member_id, &a.id) * a.ad).sum();
    let a_ed: f64 = document.seismics.iter().map(|s| influence_for(&document.effects, member_id, &s.id) * s.a_ed()).sum();
    ActionSet { g_k, g_k_inf: g_inf, p_k, q_k, a_d, a_ed }
}
//#endregion 🔖️ComplianceReport
