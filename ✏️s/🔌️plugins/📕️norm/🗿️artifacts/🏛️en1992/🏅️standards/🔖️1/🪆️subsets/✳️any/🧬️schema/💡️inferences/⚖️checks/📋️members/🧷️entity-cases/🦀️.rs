//! 🧷️ Entity cases: catalogue entries evaluated on the entities of the document, so their ids are reachable without a design case: bond strength, basic anchorage and bar spacing per longitudinal layer (B-01, B-02, B-09), the stressing limits of the prestressing steel (PR-01, PR-02, A-16), the control perimeters of a punching column (P-01), the cracking moment of a member (M-14) and the material laws of the grade rows (M-02, M-03, M-08, M-09). Every other entry is reached through [`reach`]. See [`super`].

use super::*;

#[path = "📍️reach/🦀️.rs"]
pub mod reach;

fn push_case(out: &mut Vec<CheckResult>, subject: &SubjectRef, entry_id: &str, case: &Case) {
    for mut result in evaluate_case(entry_id, case) {
        result.subject = subject.clone();
        result.remedies.clear();
        out.push(result);
    }
}

fn layers(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64, f_yk: f64) {
    for layer in member.longitudinal.iter().filter(|l| l.diameter > 0.0) {
        let subject = member_ref(doc, member, &layer_path(&member.id, &layer.id, "diameter"));
        let base = Case::new(&format!("{}-{}", member.id, layer.id), annex)
            .with_n("fck", f_ck / 1.0e6)
            .with_n("phi", layer.diameter * 1.0e3)
            .with_s("bond", if layer.bond_condition == semio_s_artifact_norm_en1992_model::BondCondition::Poor { "poor" } else { "good" });
        push_case(out, &subject, "B-01", &base);
        push_case(out, &subject, "B-02", &base.clone().with_n("fyk", f_yk / 1.0e6));
        let count = layer.count as f64;
        let clear = (member.width - 2.0 * member.cover - layer.diameter) / (count - 1.0).max(1.0) - layer.diameter;
        if layer.count > 1 && clear > 0.0 {
            let spacing = base.with_n("clear", clear * 1.0e3);
            push_case(out, &subject, "B-09", &if layer.aggregate_size > 0.0 { spacing.with_n("dg", layer.aggregate_size * 1.0e3) } else { spacing });
        }
    }
}

fn prestress(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice) {
    let Some(ps) = member.prestress.as_ref().filter(|p| p.area > 0.0) else { return };
    let Some(steel) = doc.prestress_steels.iter().find(|s| s.id == member.prestress_steel_id) else { return };
    let subject = member_ref(doc, member, &member_path(&member.id, "prestress"));
    let case = Case::new(&member.id, annex).with_n("ap", ps.area * 1.0e6).with_n("fpk", steel.f_pk / 1.0e6).with_n("fp01k", steel.f_p0_1k / 1.0e6).with_n("pm0", ps.force);
    for entry in ["PR-01", "PR-02", "A-16"] {
        push_case(out, &subject, entry, &case);
    }
}

fn punching(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice) {
    let Some(spec) = member.punching.as_ref().filter(|p| p.column_width > 0.0 && member.effective_depth > 0.0) else { return };
    let subject = member_ref(doc, member, &member_path(&member.id, "punching"));
    let case = Case::new(&member.id, annex)
        .with_n("c1", spec.column_width * 1.0e3)
        .with_n("c2", if spec.column_depth > 0.0 { spec.column_depth * 1.0e3 } else { spec.column_width * 1.0e3 })
        .with_n("d", member.effective_depth * 1.0e3)
        .with_s("position", match spec.column_position { semio_s_artifact_norm_en1992_model::ColumnPosition::Interior => "interior", semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => "edge", semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => "corner", semio_s_artifact_norm_en1992_model::ColumnPosition::WallEnd => "wall_end" });
    push_case(out, &subject, "P-01", &case);
}

fn cracking(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64) {
    let subject = member_ref(doc, member, &member_path(&member.id, "height"));
    let base = Case::new(&member.id, annex).with_n("fck", f_ck / 1.0e6).with_n("b", member.width * 1.0e3).with_n("h", member.height * 1.0e3);
    let case = match member.prestress.as_ref().filter(|p| p.force > 0.0) {
        Some(ps) => base.with_n("p", ps.force).with_n("e_p", ps.eccentricity * 1.0e3),
        None => base,
    };
    push_case(out, &subject, "M-14", &case);
}

fn grade_subject(id: &str, table: &str, leaf: &str, name: &str) -> SubjectRef {
    SubjectRef::new(id, format!("{table}[id={id}].{leaf}"), semio_s_plugin_norm_compliance::LocalizedCopy::new(name, name))
}

/// 🧷️ Evaluates the entity-bound catalogue entries of one member.
pub(super) fn check_entity_cases(doc: &En1992Snapshot, member: &RcMember, out: &mut Vec<CheckResult>, annex: AnnexChoice, f_ck: f64, f_yk: f64) {
    layers(doc, member, out, annex, f_ck, f_yk);
    prestress(doc, member, out, annex);
    punching(doc, member, out, annex);
    cracking(doc, member, out, annex, f_ck);
}

/// 🧷️ The material laws of the grade rows: M-02 and M-03 per concrete grade, M-08 per reinforcement grade (at the design yield strain) and M-09 per prestressing steel (at the design strain of f_p0.1k/γ_s).
pub fn evaluate_material_cases(doc: &En1992Snapshot) -> Vec<CheckResult> {
    let annex = doc.annex;
    let mut out = Vec::new();
    for grade in doc.concrete_grades.iter().filter(|g| g.f_ck > 0.0) {
        let subject = grade_subject(&grade.id, "concreteGrades", "fCk", &grade.name);
        let case = Case::new(&grade.id, annex).with_n("fck", grade.f_ck / 1.0e6);
        for entry in ["M-02", "M-03"] {
            push_case(&mut out, &subject, entry, &case);
        }
    }
    for grade in doc.reinforcement_grades.iter().filter(|g| g.f_yk > 0.0 && g.e_s > 0.0) {
        let subject = grade_subject(&grade.id, "reinforcementGrades", "fYk", &grade.name);
        let (fyk, es) = (grade.f_yk / 1.0e6, grade.e_s / 1.0e6);
        let case = Case::new(&grade.id, annex).with_n("fyk", fyk).with_n("es", es).with_n("k", grade.k).with_n("eps_uk", grade.eps_uk * 1.0e3).with_n("eps", f_yd(annex, fyk, false) / es * 1.0e3);
        push_case(&mut out, &subject, "M-08", &case);
    }
    for steel in doc.prestress_steels.iter().filter(|s| s.f_pk > 0.0 && s.f_p0_1k > 0.0) {
        let subject = grade_subject(&steel.id, "prestressSteels", "fPk", &steel.name);
        let (fpk, fp01k) = (steel.f_pk / 1.0e6, steel.f_p0_1k / 1.0e6);
        let case = Case::new(&steel.id, annex).with_n("fpk", fpk).with_n("fp01k", fp01k).with_n("eps", fp01k / ndp(annex, "ndp.en1992.gamma-s") / 195000.0 * 1.0e3);
        push_case(&mut out, &subject, "M-09", &case);
    }
    out
}
