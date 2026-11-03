use super::*;

struct Toy;

static VOCABULARY: &[InputSpec] = &[
    InputSpec::number("b", "Width b", "Breite b", UnitId::Millimetre).example(200.0).lever(),
    InputSpec::number("h", "Depth h", "Höhe h", UnitId::Millimetre).example(400.0).lever(),
    InputSpec::number("n", "Force N", "Kraft N", UnitId::Newton),
    InputSpec::number("n", "Force N", "Kraft N", UnitId::Kilonewton).scope("T-02"),
    InputSpec::number("angle", "Angle", "Winkel", UnitId::Degree).range(0.0, 90.0),
    InputSpec::choice("class", "Class", "Klasse", &[InputChoice::new("C24", "C24", "C24"), InputChoice::new("GL24h", "GL24h", "GL24h")]).example_text("C24"),
    InputSpec::text("note", "Note", "Notiz"),
];

static ENTRIES: [CaseEntry; 2] = [
    CaseEntry { id: "T-01", part: "EN TOY-1", clause: "6.1.2", en: "Tension parallel to grain", de: "Zug parallel zur Faser", inputs: &["class", "n", "b", "h"], geometry: Some(CaseGeometry { form: CaseForm::Member, width: "b", length: "", height: "h", load: "n", aux: &[] }) },
    CaseEntry { id: "T-02", part: "EN TOY-1", clause: "6.2.1", en: "Compression", de: "Druck", inputs: &["n"], geometry: None },
];

impl CaseCatalogue for Toy {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.to_vec()
    }

    fn vocabulary() -> &'static [InputSpec] {
        VOCABULARY
    }

    fn evaluate(entry: &CaseEntry, annex: AnnexChoice, case: &DesignCase) -> Vec<CheckResult> {
        let get = |key: &str| case.values.numbers.get(key).copied().unwrap_or(0.0);
        let capacity = get("b") * get("h") * 14.0;
        vec![CheckResult::assess(format!("toy.{}.{}", entry.id, case.id), entry.part, ClauseId::new(entry.part, "§6", entry.clause), SubjectRef::whole(loc("Toy", "Spielzeug")), loc(entry.en, entry.de))
            .annex(annex)
            .utilization(Quantity::new(QuantityKind::Force, get("n")), Quantity::new(QuantityKind::Force, capacity))
            .build()]
    }
}

fn case(entry: &str, b: f64, n: f64) -> DesignCase {
    DesignCase { id: "c1".into(), label_en: "Tie".into(), label_de: "Zugstab".into(), entry_id: entry.into(), values: CaseValues::default().with_number("b", b).with_number("h", 400.0).with_number("n", n).with_text("class", "C24") }
}

#[test]
fn search_matches_id_clause_part_and_both_titles() {
    let all = Toy::entries();
    let ids = |query: &str| search(&all, query).iter().map(|e| e.id).collect::<Vec<_>>();
    assert_eq!(ids("t01"), ["T-01"]);
    assert_eq!(ids("T-02"), ["T-02"]);
    assert_eq!(ids("6.1"), ["T-01"]);
    assert_eq!(ids("toy"), ["T-01", "T-02"]);
    assert_eq!(ids("faser zug"), ["T-01"]);
    assert_eq!(ids("compression"), ["T-02"]);
    assert_eq!(ids("druck"), ["T-02"]);
    assert!(ids("nothing here").is_empty());
    assert_eq!(ids(""), ["T-01", "T-02"]);
}

#[test]
fn exact_id_and_clause_prefix_rank_first() {
    let all = Toy::entries();
    assert_eq!(search(&all, "6.2 toy").first().map(|e| e.id), Some("T-02"));
    assert_eq!(search(&all, "t-02 toy").first().map(|e| e.id), Some("T-02"));
}

#[test]
fn scoped_vocabulary_row_wins_over_the_unscoped_row() {
    assert_eq!(spec::<Toy>("T-01", "n").map(|r| r.unit), Some(UnitId::Newton));
    assert_eq!(spec::<Toy>("T-02", "n").map(|r| r.unit), Some(UnitId::Kilonewton));
    assert!(spec::<Toy>("T-01", "missing").is_none());
}

#[test]
fn seed_applies_the_examples_of_the_declared_inputs_only() {
    let seeded = seed::<Toy>(&ENTRIES[0], "c9");
    assert_eq!(seeded.values.numbers.get("b"), Some(&200.0));
    assert_eq!(seeded.values.numbers.get("h"), Some(&400.0));
    assert!(!seeded.values.numbers.contains_key("n"));
    assert_eq!(seeded.values.texts.get("class").map(String::as_str), Some("C24"));
    assert_eq!((seeded.label_en.as_str(), seeded.label_de.as_str()), ("Tension parallel to grain", "Zug parallel zur Faser"));
    assert_eq!(declared::<Toy>(&ENTRIES[0]).iter().map(|r| r.key).collect::<Vec<_>>(), ["class", "n", "b", "h"]);
}

#[test]
fn issues_name_unknown_wrong_kind_range_and_choice() {
    let values = CaseValues::default().with_number("zzz", 1.0).with_number("class", 1.0).with_number("angle", 120.0).with_text("class", "XX").with_text("b", "wide");
    let found = issues::<Toy>(&ENTRIES[0], &values);
    assert!(found.contains(&CaseIssue::UnknownInput { key: "zzz".into() }));
    assert!(found.contains(&CaseIssue::WrongKind { key: "class".into() }));
    assert!(found.contains(&CaseIssue::WrongKind { key: "b".into() }));
    assert!(found.contains(&CaseIssue::OutOfRange { key: "angle".into(), value: 120.0, min: 0.0, max: 90.0 }));
    assert!(found.contains(&CaseIssue::NotAChoice { key: "class".into(), value: "XX".into() }));
    assert!(issues::<Toy>(&ENTRIES[0], &seed::<Toy>(&ENTRIES[0], "c").values).is_empty());
}

#[test]
fn passing_case_is_addressed_to_its_row_with_clause_and_utilisation() {
    let results = evaluate_case::<Toy>(&case("T-01", 200.0, 560_000.0), AnnexChoice::De);
    assert_eq!(results.len(), 1);
    let r = &results[0];
    assert_eq!(r.status, CheckStatus::Pass);
    assert!((r.utilization - 0.5).abs() < 1e-12);
    assert_eq!((r.subject.entity_id.as_str(), r.subject.path.as_str()), ("c1", "cases[id=c1]"));
    assert_eq!((r.subject.label.en.as_str(), r.subject.label.de.as_str()), ("Tie", "Zugstab"));
    assert_eq!(r.clause.to_string(), ClauseId::new("EN TOY-1", "§6", "6.1.2").to_string());
    assert!(r.remedies.is_empty());
    assert_eq!(r.annex, AnnexChoice::De);
}

#[test]
fn failing_case_gets_a_linear_sizing_remedy_on_the_first_lever_input() {
    let results = evaluate_case::<Toy>(&case("T-01", 200.0, 1_300_000.0), AnnexChoice::En);
    let r = &results[0];
    assert_eq!(r.status, CheckStatus::Fail);
    let utilisation = 1_300_000.0 / (200.0 * 400.0 * 14.0);
    assert!((r.utilization - utilisation).abs() < 1e-12);
    let remedy = r.remedies.first().expect("sizing remedy");
    assert_eq!(remedy.bound, RemedyBound::AtLeast);
    assert_eq!(remedy.target.path, "cases[id=c1].numbers[id=b].value");
    assert!((remedy.current.value - 200.0).abs() < 1e-12);
    assert!((remedy.required.value - 200.0 * utilisation).abs() < 1e-9);
    assert_eq!(remedy.required.kind, QuantityKind::Dimensionless);
}

#[test]
fn invalid_inputs_fail_with_remedies_before_the_catalogue_results() {
    let mut bad = case("T-01", 200.0, 560_000.0);
    bad.values.texts.insert("class".into(), "XX".into());
    bad.values.numbers.insert("angle".into(), 120.0);
    let results = evaluate_case::<Toy>(&bad, AnnexChoice::En);
    assert_eq!(results.len(), 3);
    let choice = results.iter().find(|r| r.id.ends_with(".class")).expect("choice issue");
    assert_eq!(choice.status, CheckStatus::Fail);
    assert_eq!(choice.remedies[0].bound, RemedyBound::OneOf);
    assert_eq!(choice.remedies[0].options, ["C24", "GL24h"]);
    let range = results.iter().find(|r| r.id.ends_with(".angle")).expect("range issue");
    assert_eq!(range.remedies[0].bound, RemedyBound::AtMost);
    assert!((range.remedies[0].required.value - 90.0).abs() < 1e-12);
    assert!(results.last().is_some_and(|r| r.id.starts_with("toy.")));
}

#[test]
fn unknown_entry_is_not_applicable() {
    let results = evaluate_case::<Toy>(&case("T-99", 200.0, 1.0), AnnexChoice::En);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, CheckStatus::NotApplicable);
}

#[test]
fn input_paths_address_the_number_and_text_rows() {
    assert_eq!(input_path("c1", "b", true), "cases[id=c1].numbers[id=b].value");
    assert_eq!(input_path("c1", "class", false), "cases[id=c1].texts[id=class].value");
}

fn leak(text: &str) -> &'static str {
    Box::leak(text.to_string().into_boxed_str())
}

fn fixture() -> serde_json::Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧰️design-case/🧪️tests/🧫️fixtures/🔣️.json");
    serde_json::from_slice(&std::fs::read(path).expect("fixture")).expect("fixture json")
}

#[test]
fn fixture_searches_and_levers_equal_the_oracle_expectations() {
    let fixture = fixture();
    let entries: Vec<CaseEntry> = fixture["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| CaseEntry { id: leak(e["id"].as_str().unwrap()), part: leak(e["part"].as_str().unwrap()), clause: leak(e["clause"].as_str().unwrap()), en: leak(e["en"].as_str().unwrap()), de: leak(e["de"].as_str().unwrap()), inputs: &[], geometry: None })
        .collect();
    for case in fixture["searches"].as_array().unwrap() {
        let expected: Vec<&str> = case["ids"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
        let got: Vec<&str> = search(&entries, case["query"].as_str().unwrap()).iter().map(|e| e.id).collect();
        assert_eq!(got, expected, "query {:?}", case["query"]);
    }
    for lever in fixture["levers"].as_array().unwrap() {
        let (current, utilisation, required) = (lever["current"].as_f64().unwrap(), lever["utilization"].as_f64().unwrap(), lever["required"].as_f64().unwrap());
        assert!((current * utilisation - required).abs() < 1e-9);
    }
}

#[test]
fn independent_python_oracle_accepts_the_fixture() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧰️design-case/🔮️oracles/🐍️.py");
    let output = crate::python_oracle::run_ok(&script, &[], None);
    let answer: serde_json::Value = serde_json::from_slice(&output.stdout).expect("oracle json");
    assert_eq!(answer["searches"].as_u64(), Some(12));
}

#[test]
fn geometry_converts_the_stored_unit_to_metres_and_needs_both_sizes() {
    let tie = seed::<Toy>(&ENTRIES[0], "c");
    let (w, h) = geometry_m::<Toy>(&ENTRIES[0], &tie.values).expect("geometry");
    assert!((w - 0.2).abs() < 1e-12 && (h - 0.4).abs() < 1e-12);
    assert!(geometry_m::<Toy>(&ENTRIES[1], &tie.values).is_none());
    let mut missing = tie.values.clone();
    missing.numbers.remove("h");
    assert!(geometry_m::<Toy>(&ENTRIES[0], &missing).is_none());
    missing.numbers.insert("h".into(), 0.0);
    assert!(geometry_m::<Toy>(&ENTRIES[0], &missing).is_none());
}

#[test]
fn vector_inputs_are_numbered_rows_with_the_vector_unit() {
    static VECTOR: &[InputSpec] = &[InputSpec::vector("profile", "Profile", "Profil", UnitId::Millimetre).range(0.0, 100.0)];
    struct Vec1;
    impl CaseCatalogue for Vec1 {
        fn entries() -> Vec<CaseEntry> {
            vec![CaseEntry { id: "V-01", part: "P", clause: "1", en: "v", de: "v", inputs: &["profile"], geometry: None }]
        }
        fn vocabulary() -> &'static [InputSpec] {
            VECTOR
        }
        fn evaluate(_: &CaseEntry, _: AnnexChoice, _: &DesignCase) -> Vec<CheckResult> {
            Vec::new()
        }
    }
    let values = CaseValues::default().with_vector("profile", &[1.0, 2.0, 3.0]);
    assert_eq!(values.vector("profile"), Some(vec![1.0, 2.0, 3.0]));
    assert_eq!(CaseValues::default().vector("profile"), None);
    assert_eq!((element_key("profile", 2), series_key("h1", 0), series_key("h1", 2), series_key("rm3", 1), series_key("bar_a_m2", 1)), ("profile.2".to_string(), "h1".to_string(), "h3".to_string(), "rm4".to_string(), "bar_a_m3".to_string()));
    assert_eq!((element_index("profile", "profile.12"), element_index("profile", "profile"), series_index("h1", "h3"), series_index("h1", "h0"), series_index("h1", "hx"), series_index("h1", "gh2"), series_index("h1", "h")), (Some(12), None, Some(2), None, None, None, None));
    assert_eq!(CaseValues::default().with_series("h1", &[3.0, 4.0]).series("h1"), Some(vec![3.0, 4.0]));
    assert_eq!(InputSpec::series("h1", "H", "H", UnitId::Metre).element_key(1), "h2");
    assert_eq!(InputSpec::vector("bar_a_m2", "A", "A", UnitId::SquareMetre).element_key(1), "bar_a_m2.1");
    let entry = entry::<Vec1>("V-01").unwrap();
    assert!(issues::<Vec1>(&entry, &values).is_empty());
    assert!(issues::<Vec1>(&entry, &CaseValues::default().with_number("profile.0", 500.0)).contains(&CaseIssue::OutOfRange { key: "profile.0".into(), value: 500.0, min: 0.0, max: 100.0 }));
    assert_eq!(issues::<Vec1>(&entry, &CaseValues::default().with_number("profile", 1.0)), vec![CaseIssue::UnknownInput { key: "profile".into() }]);
    assert_eq!(seed::<Vec1>(&entry, "c").values.numbers.len(), 0);
}
