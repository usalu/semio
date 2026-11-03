use super::*;

const ITEMS: &[SpecTrace] = &[
    SpecTrace::new("A-01", "3.2", "materials", Realisation::Check(&["en0000.3.2.grade"]), SpecText::new("x", "y")),
    SpecTrace::new("A-02", "3.3", "", Realisation::Data("tables::ALLOYS"), SpecText::new("x", "y")),
    SpecTrace::new("A-03", "4", "", Realisation::NotModelled(SpecText::new("the snapshot carries no such input", "die Momentaufnahme enthält keine solche Eingabe")), SpecText::new("description", "Hinweis")),
];

const TABLE: SpecTraceTable = SpecTraceTable { code: "en0000", standard: "EN 0000", spec: "spec.md", spec_ids: &["A-01", "A-02", "A-03"], items: ITEMS };

#[test]
fn a_complete_trace_has_no_problems_and_counts_its_kinds() {
    let emitted = Emitted::default().with_checks(["en0000.3.2.grade.beam-1"]);
    assert!(problems(&TABLE, &emitted).is_empty());
    assert_eq!(TABLE.counts(), TraceCounts { check: 1, data: 1, design_case: 0, not_modelled: 1 });
    assert_eq!(TABLE.item("A-02").map(|item| item.realisation.kind()), Some("data"));
    assert_eq!(ITEMS[0].realisation.describe(&Locale::De), "en0000.3.2.grade");
    assert_traced(&TABLE, &emitted);
}

#[test]
fn a_fragment_no_example_emits_is_reported() {
    let found = problems(&TABLE, &Emitted::default().with_checks(["en0000.other"]));
    assert_eq!(found.len(), 1);
    assert!(found[0].starts_with("A-01") && found[0].contains("en0000.3.2.grade"));
}

#[test]
fn untraced_unknown_duplicate_and_unexplained_ids_are_reported() {
    const BAD: &[SpecTrace] = &[
        SpecTrace::new("A-01", "3.2", "", Realisation::DesignCase("A-01"), SpecText::new("x", "y")),
        SpecTrace::new("A-01", "3.2", "", Realisation::Data(" "), SpecText::new("x", "y")),
        SpecTrace::new("Z-99", "1", "", Realisation::NotModelled(SpecText::new("short", "kurz")), SpecText::new("", "")),
    ];
    let table = SpecTraceTable { items: BAD, ..TABLE };
    let found = problems(&table, &Emitted::default());
    for expected in ["A-01: traced twice", "A-01: design-case entry", "A-01: empty data path", "Z-99: not an id of the spec", "Z-99: reason too short", "A-02: untraced", "A-03: untraced"] {
        assert!(found.iter().any(|line| line.contains(expected)), "{expected} missing in {found:?}");
    }
}

#[test]
fn a_showcase_reaches_every_fragment_but_the_documented_ones() {
    let none = Emitted::default();
    assert_eq!(unreached_fragments(&TABLE, &none), vec!["en0000.3.2.grade"]);
    assert_reaches_all_but(&TABLE, &none, &[("en0000.3.2.grade", "needs a second document of another kind")]);
    assert_reaches_all_but(&TABLE, &Emitted::default().with_checks(["en0000.3.2.grade.beam-1"]), &[]);
    assert!(std::panic::catch_unwind(|| assert_reaches_all_but(&TABLE, &Emitted::default(), &[])).is_err(), "an unexplained fragment fails");
    assert!(std::panic::catch_unwind(|| assert_reaches_all_but(&TABLE, &Emitted::default().with_checks(["en0000.3.2.grade.beam-1"]), &[("en0000.3.2.grade", "this allowance is stale")])).is_err(), "a stale allowance fails");
}

#[test]
fn descriptions_are_bilingual_and_a_missing_language_is_reported() {
    assert_eq!(ITEMS[2].description.resolve(&Locale::En), "description");
    assert_eq!(ITEMS[2].description.resolve(&Locale::De), "Hinweis");
    assert_eq!(ITEMS[2].realisation.describe(&Locale::De), "die Momentaufnahme enthält keine solche Eingabe");
    const HALF: &[SpecTrace] = &[SpecTrace::new("A-01", "3.2", "", Realisation::Data("x"), SpecText::new("only english", ""))];
    let found = problems(&SpecTraceTable { items: HALF, ..TABLE }, &Emitted::default());
    assert!(found.iter().any(|line| line.contains("A-01: description missing in a language")), "{found:?}");
}
