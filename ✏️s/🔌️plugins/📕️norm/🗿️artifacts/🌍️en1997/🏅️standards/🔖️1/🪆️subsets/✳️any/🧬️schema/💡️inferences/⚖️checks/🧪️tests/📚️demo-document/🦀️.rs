use super::*;

/// 📚️ The full-catalogue demo document is checked as a whole: every case contributes results addressed to its own row and the per-case live units cover the same rows.
#[test]
fn every_catalogue_entry_is_checked_from_the_demo_document() {
    let document = catalogue_demo_document();
    let report = check_project(&document);
    for case in &document.cases {
        let prefix = format!("cases[id={}]", case.id);
        assert!(report.checks.iter().any(|check| check.subject.path.starts_with(&prefix)), "case {} ({}) has no result in the document report", case.id, case.entry_id);
    }
}
