use crate::*;
use semio_framework_value::DslValue;

fn text(value: &str) -> DslValue {
    DslValue::String(value.to_owned())
}

fn row(id: &str, fields: &[(&str, &str)]) -> DslValue {
    DslValue::Object(std::iter::once(("id".to_owned(), text(id))).chain(fields.iter().map(|(key, value)| ((*key).to_owned(), text(value)))).collect())
}

fn document() -> DslValue {
    DslValue::Object(vec![
        ("projectId".to_owned(), text("p")),
        ("members".to_owned(), DslValue::Array(vec![row("m1", &[("sectionId", "s1")]), row("m2", &[])])),
        ("sections".to_owned(), DslValue::Array(vec![row("s1", &[]), row("s2", &[])])),
    ])
}

fn check(id: &str, subject: &str) -> CheckResult {
    CheckResult::assess(id, "p", ClauseId::new("EN 1992", "1-1", "6.2"), SubjectRef::new(subject, "path", LocalizedCopy::new("Subject", "Gegenstand")), LocalizedCopy::new("Shear", "Querkraft"))
        .utilization(Quantity::force_kn(50.0), Quantity::force_kn(100.0))
        .build()
}

fn filed(owners: Owners, rows: &[(&str, &str)]) -> Vec<String> {
    let mut report = CheckReport::default();
    rows.iter().for_each(|(id, subject)| report.push(check(id, subject)));
    owners.file(&document(), &mut report);
    report.checks.iter().map(|c| c.subject.entity_id.clone()).collect()
}

const OWNING: Owners = Owners::new(&["members"], &["g."]);

#[test]
fn a_subject_that_is_an_owning_row_keeps_its_owner() {
    assert_eq!(filed(OWNING, &[("en.shear.m2", "m1")]), vec!["m1"]);
}

#[test]
fn a_check_id_that_embeds_an_owning_row_id_names_its_owner() {
    assert_eq!(filed(OWNING, &[("en.6.2.shear.m2", ""), ("en.6.2.shear.m2", "p"), ("en.m1.m2", "")]), vec!["m2", "m2", "m1"]);
}

#[test]
fn a_subject_row_of_a_non_owning_collection_is_owned_by_the_first_row_that_names_it() {
    assert_eq!(filed(OWNING, &[("en.section.s1", "s1"), ("en.section.s2", "s2")]), vec!["m1", "s2"]);
}

#[test]
fn a_documented_document_level_check_without_owner_is_filed_under_none_and_others_stay_as_emitted() {
    assert_eq!(filed(OWNING, &[("g.annex", "p"), ("en.other", "p")]), vec!["", "p"]);
    assert!(OWNING.is_document_level("g.annex") && !OWNING.is_document_level("en.other"));
}

#[test]
fn as_emitted_keeps_every_subject_and_rows_make_every_collection_an_owner() {
    assert_eq!(filed(Owners::AS_EMITTED, &[("en.shear.m1", ""), ("g.annex", "p")]), vec!["", "p"]);
    assert_eq!(filed(Owners::ROWS, &[("en.shear.s2", "")]), vec!["s2"]);
    assert_eq!(Owners::new(&["members"], &[]).owner_ids(&document()).into_iter().collect::<Vec<_>>(), vec!["m1", "m2"]);
    assert_eq!(Owners::ROWS.owner_ids(&document()).len(), 4);
}
