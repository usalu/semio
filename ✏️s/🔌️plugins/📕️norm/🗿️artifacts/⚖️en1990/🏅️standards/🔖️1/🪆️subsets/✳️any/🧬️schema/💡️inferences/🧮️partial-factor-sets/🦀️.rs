//! 🧮️ Partial factor sets and input validity of the EN 1990 compliance report: K_FI, the RC/CC link and the closed vocabularies of structure kind, classes, action kinds and ψ categories. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
fn invalid_input(report: &mut CheckReport, document: &En1990Snapshot, id: String, path: String, label: (&str, &str), declared: &str, options: Vec<String>) {
    let subject = SubjectRef::new(document.project_id.clone(), &path, copy(label.0, label.1));
    let remedy = Remedy::one_of(subject.clone(), options.clone(), copy(format!("Set {} to one of: {}.", label.0, options.join(", ")), format!("{} auf einen der Werte setzen: {}.", label.1, options.join(", "))));
    report.push(
        CheckResult::assess(id, "EN 1990 input", ClauseId::new("EN 1990", "§1", "vocabulary"), subject, copy(format!("Valid {}", label.0), format!("Gültige(r) {}", label.1)))
            .status(CheckStatus::Fail)
            .annex(document.annex)
            .explanation(copy(format!("{} '{declared}' is not defined; allowed: {}.", label.0, options.join(", ")), format!("{} '{declared}' ist nicht definiert; erlaubt: {}.", label.1, options.join(", "))))
            .remedy(remedy)
            .build(),
    );
}

/// 🚧️ Emits one failing check per undefined enumerated input; returns whether the combination checks may run.
pub(super) fn push_input_validation(report: &mut CheckReport, document: &En1990Snapshot) -> bool {
    let before = report.summary.fail;
    if !matches!(document.consequence_class, 1..=3) {
        invalid_input(report, document, "en1990.input.consequenceClass".into(), "consequenceClass".into(), ("Consequence class", "Schadensfolgeklasse"), &document.consequence_class.to_string(), vec!["1".into(), "2".into(), "3".into()]);
    }
    for permanent in &document.permanents {
        if !PERMANENT_KINDS.contains(&permanent.kind.as_str()) {
            invalid_input(report, document, format!("en1990.input.permanents.{}.kind", permanent.id), format!("permanents[id={}].kind", permanent.id), ("Permanent action kind", "Art der ständigen Einwirkung"), permanent.kind.as_str(), PERMANENT_KINDS.iter().map(ToString::to_string).collect());
        }
    }
    for variable in &document.variables {
        if psi_entry_of(&variable.category).is_none() {
            let options = PSI_TABLE.iter().map(|entry| entry.key.to_string()).collect();
            invalid_input(report, document, format!("en1990.input.variables.{}.category", variable.id), format!("variables[id={}].category", variable.id), ("ψ category", "ψ-Kategorie"), &variable.category, options);
        }
    }
    report.summary.fail == before
}

pub(super) fn push_rc_vs_cc(report: &mut CheckReport, document: &En1990Snapshot) {
    if !matches!(document.consequence_class, 1..=3) {
        return;
    }
    let equal = document.consequence_class == document.reliability_class.number();
    let subject = SubjectRef::new(document.project_id.clone(), "reliabilityClass", copy("Reliability class", "Zuverlässigkeitsklasse"));
    let mut check = CheckResult::assess("en1990.annex-b.rc-vs-cc", "EN 1990 Annex B", ClauseId::new("EN 1990", "Annex B", "B3"), subject.clone(), copy("Reliability class derived from consequence class", "Zuverlässigkeitsklasse aus Schadensfolgeklasse"))
        .status(if equal { CheckStatus::Pass } else { CheckStatus::Warning })
        .annex(document.annex)
        .explanation(copy(
            format!("CC{} gives RC{} by default; RC{} is an explicit Annex B.3 override.", document.consequence_class, document.consequence_class, document.reliability_class.number()),
            format!("CC{} ergibt standardmäßig RC{}; RC{} ist eine ausdrückliche Abweichung nach Anhang B.3.", document.consequence_class, document.consequence_class, document.reliability_class.number()),
        ));
    if !equal {
        check = check.remedy(Remedy::one_of(
            subject,
            vec![format!("RC{}", document.consequence_class)],
            copy(format!("Set the reliability class to RC{}, the default of CC{} (Annex B.3), or keep the override with a justification.", document.consequence_class, document.consequence_class), format!("Zuverlässigkeitsklasse auf RC{} setzen, den Standard von CC{} (Anhang B.3), oder die Abweichung begründet beibehalten.", document.consequence_class, document.consequence_class)),
        ));
    }
    report.push(check.build());
}
//#endregion 🔖️ComplianceReport
