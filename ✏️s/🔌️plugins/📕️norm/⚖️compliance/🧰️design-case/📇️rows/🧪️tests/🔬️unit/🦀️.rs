use super::*;

struct Toy;

static ENTRY_IDS: [&str; 1] = ["T-01"];
static ENTRIES: [CaseEntry; 1] = [CaseEntry { id: "T-01", part: "EN TOY-1", clause: "6.1.2", en: "Tension", de: "Zug", inputs: &["b"], geometry: None }];

impl CaseCatalogue for Toy {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.to_vec()
    }

    fn vocabulary() -> &'static [InputSpec] {
        &[]
    }

    fn evaluate(_: &CaseEntry, _: AnnexChoice, _: &DesignCase) -> Vec<CheckResult> {
        Vec::new()
    }
}

fn row(id: &str) -> CaseRow {
    CaseRow { id: id.into(), label_en: "Tie".into(), label_de: "Zugstab".into(), entry_id: ENTRY_IDS[0].into(), numbers: vec![CaseNumber { id: "b".into(), value: 200.0 }], texts: vec![CaseText { id: "class".into(), value: "C24".into() }] }
}

#[test]
fn add_inserts_clamped_and_refuses_duplicates_and_unknown_entries() {
    let rows = vec![row("a")];
    let delta = add_case::<Toy>(&rows, 99, &row("b")).expect("add");
    assert_eq!((delta.inserted.len(), delta.inserted[0].index, delta.inserted[0].row.id.as_str()), (1, 1, "b"));
    assert_eq!(add_case::<Toy>(&rows, 0, &row("a")), Err(CaseRefusal::DuplicateCase("a".into())));
    let mut unknown = row("c");
    unknown.entry_id = "T-99".into();
    assert_eq!(add_case::<Toy>(&rows, 0, &unknown), Err(CaseRefusal::UnknownEntry("T-99".into())));
}

#[test]
fn remove_addresses_the_row_by_id() {
    let rows = vec![row("a"), row("b")];
    let delta = remove_case(&rows, "b").expect("remove");
    assert_eq!((delta.removed.len(), delta.removed[0].id.as_str(), delta.removed[0].index), (1, "b", 1));
    assert_eq!(remove_case(&rows, "c"), Err(CaseRefusal::UnknownCase("c".into())));
}

#[test]
fn set_number_modifies_inserts_removes_or_is_empty() {
    let rows = vec![row("a")];
    let changed = change_case_number(&rows, "a", "b", Some(250.0)).expect("modify");
    assert_eq!(changed.modified[0].patch.numbers.modified[0].patch.value, Some(250.0));
    let added = change_case_number(&rows, "a", "h", Some(400.0)).expect("insert");
    assert_eq!((added.modified[0].patch.numbers.inserted[0].index, added.modified[0].patch.numbers.inserted[0].row.id.as_str()), (1, "h"));
    let removed = change_case_number(&rows, "a", "b", None).expect("remove");
    assert_eq!(removed.modified[0].patch.numbers.removed[0].id, "b");
    assert_eq!(change_case_number(&rows, "a", "zz", None), Ok(CaseRowDelta::default()));
    assert_eq!(change_case_number(&rows, "x", "b", Some(1.0)), Err(CaseRefusal::UnknownCase("x".into())));
}

#[test]
fn set_text_mirrors_set_number() {
    let rows = vec![row("a")];
    assert_eq!(change_case_text(&rows, "a", "class", Some("GL24h")).unwrap().modified[0].patch.texts.modified[0].patch.value.as_deref(), Some("GL24h"));
    assert_eq!(change_case_text(&rows, "a", "kind", Some("nail")).unwrap().modified[0].patch.texts.inserted[0].row.value, "nail");
    assert_eq!(change_case_text(&rows, "a", "class", None).unwrap().modified[0].patch.texts.removed[0].id, "class");
}

#[test]
fn previous_values_drive_the_inverse() {
    let rows = vec![row("a")];
    assert_eq!(previous_number(&rows, "a", "b"), Some(200.0));
    assert_eq!(previous_number(&rows, "a", "h"), None);
    assert_eq!(previous_text(&rows, "a", "class").as_deref(), Some("C24"));
}

#[test]
fn refusals_carry_code_message_and_targets() {
    assert_eq!(CaseRefusal::DuplicateCase("a".into()).code(), "mutation.duplicate-id");
    assert_eq!(CaseRefusal::UnknownCase("a".into()).code(), "mutation.invariant");
    assert_eq!(CaseRefusal::UnknownCase("a".into()).targets(), ["a"]);
    assert!(CaseRefusal::UnknownCase("zz".into()).message().contains("zz"));
}

#[test]
fn rows_and_neutral_cases_convert_without_loss() {
    let original = row("a");
    let case = DesignCase::from(&original);
    assert_eq!(case.values.numbers.get("b"), Some(&200.0));
    assert_eq!(CaseRow::from(&case), original);
}
