use super::*;
use semio_s_plugin_norm_compliance::design_case::{CaseValues, InputChoice};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult};

struct Toy;

static VOCABULARY: &[InputSpec] = &[
    InputSpec::number("b", "Width b", "Breite b", UnitId::Millimetre).range(1.0, 5000.0),
    InputSpec::number("angle", "Angle", "Winkel", UnitId::Degree),
    InputSpec::number("ratio", "Ratio", "Verhältnis", UnitId::One),
    InputSpec::choice("class", "Class", "Klasse", &[InputChoice::new("C24", "C24 softwood", "C24 Nadelholz"), InputChoice::new("GL24h", "GL24h glulam", "GL24h Brettschichtholz")]),
    InputSpec::text("note", "Note", "Notiz"),
    InputSpec::vector("profile", "Profile", "Profil", UnitId::Millimetre),
];

static ENTRIES: [CaseEntry; 2] = [
    CaseEntry { id: "T-01", part: "EN TOY-1", clause: "6.1.2", en: "Tension", de: "Zug", inputs: &["class", "b", "angle", "profile"], geometry: None },
    CaseEntry { id: "T-02", part: "EN TOY-1", clause: "6.2.1", en: "Compression", de: "Druck", inputs: &["b"], geometry: None },
];

impl CaseCatalogue for Toy {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.to_vec()
    }

    fn vocabulary() -> &'static [InputSpec] {
        VOCABULARY
    }

    fn evaluate(_: &CaseEntry, _: AnnexChoice, _: &DesignCase) -> Vec<CheckResult> {
        Vec::new()
    }
}

fn case() -> DesignCase {
    DesignCase { id: "c1".into(), label_en: "Tie".into(), label_de: "Zugstab".into(), entry_id: "T-01".into(), values: CaseValues::default().with_number("b", 200.0).with_text("note", "free") }
}

#[test]
fn number_input_is_a_unit_aware_quantity_with_its_bounds() {
    let field = input_field_spec(&VOCABULARY[0], 0);
    assert_eq!(field.unit(), Some("mm"));
    assert_eq!(field.label(Locale::De), "Breite b");
    assert!(field.accepts(200.0) && !field.accepts(0.0));
    assert!((field.parse("0,2 m", UnitId::Millimetre).unwrap() - 200.0).abs() < 1e-9);
    assert!((field.parse("350", UnitId::Millimetre).unwrap() - 350.0).abs() < 1e-9);
    assert!(field.units().contains(&UnitId::Metre));
    assert_eq!(field.affects, Affects::Entities(&["cases"]));
}

#[test]
fn dimensionless_input_offers_only_the_plain_unit_and_angles_convert() {
    assert_eq!(input_field_spec(&VOCABULARY[2], 2).units(), vec![UnitId::One]);
    let angle = input_field_spec(&VOCABULARY[1], 1);
    assert!((angle.to_display(90.0, UnitId::Radian).unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
}

#[test]
fn choice_and_text_inputs_keep_both_languages() {
    let field = input_field_spec(&VOCABULARY[3], 3);
    let choices = field.choices().expect("choices");
    assert_eq!(choices.len(), 2);
    assert_eq!((choices[1].label_en, choices[1].label_de), ("GL24h glulam", "GL24h Brettschichtholz"));
    assert!(std::ptr::eq(choices, input_field_spec(&VOCABULARY[3], 3).choices().unwrap()));
    assert_eq!(input_field_spec(&VOCABULARY[4], 4).kind, FieldKind::Text);
}

#[test]
fn case_input_paths_name_their_key_and_kind() {
    assert_eq!(case_input_key("cases[id=c1].numbers[id=h].value"), Some(("h", true)));
    assert_eq!(case_input_key("cases[id=c1].texts[id=class].value"), Some(("class", false)));
    assert_eq!(case_input_key("cases[id=c1].numbers[id=h].id"), None);
    assert_eq!(case_input_key("members[id=m].b"), None);
    assert_eq!(case_input_field_meta(VOCABULARY, "cases[id=c1].numbers[id=b].value").map(|f| f.unit()), Some(Some("mm")));
    assert!(case_input_field_meta(VOCABULARY, "cases[id=c1].numbers[id=zzz].value").is_none());
}

#[test]
fn rows_list_declared_inputs_first_even_when_absent_then_stored_extras() {
    let rows = case_input_rows::<Toy>(&case());
    let keys: Vec<&str> = rows.iter().map(|r| r.spec.key).collect();
    assert_eq!(keys, ["class", "b", "angle", "profile", "note"]);
    assert_eq!(rows.iter().map(|r| r.declared).collect::<Vec<_>>(), [true, true, true, true, false]);
    assert_eq!(rows[1].number, Some(200.0));
    assert_eq!(rows[0].number, None);
    assert_eq!(rows[1].path, "cases[id=c1].numbers[id=b].value");
    assert_eq!(rows[0].path, "cases[id=c1].texts[id=class].value");
    assert_eq!(rows[4].text.as_deref(), Some("free"));
    assert_eq!(rows[3].path, "cases[id=c1].numbers[id=profile.0].value");
    assert!(case_input_rows::<Toy>(&DesignCase { entry_id: "T-99".into(), ..case() }).is_empty());
}

#[test]
fn browser_table_filters_by_clause_part_and_both_titles() {
    let all = Toy::entries();
    let ids = |query: &str, locale| case_catalogue_table("EN TOY", &all, query, locale).rows.iter().map(|r| r.id.clone()).collect::<Vec<_>>();
    assert_eq!(ids("", Locale::En), ["T-01", "T-02"]);
    assert_eq!(ids("6.2", Locale::En), ["T-02"]);
    assert_eq!(ids("zug", Locale::En), ["T-01"]);
    assert_eq!(ids("compression", Locale::De), ["T-02"]);
    let table = case_catalogue_table("EN TOY", &all, "t-01", Locale::De);
    assert_eq!(table.rows[0].cells[3], CatalogueCell::text("Zug"));
    assert_eq!(table.rows[0].cells[4], CatalogueCell::number(3.0, 0));
    assert_eq!(table.columns.len(), 5);
}

#[test]
fn vector_inputs_list_their_elements_plus_the_next_empty_one() {
    let mut with_profile = case();
    with_profile.values = with_profile.values.with_vector("profile", &[1.0, 2.0]);
    let rows = case_input_rows::<Toy>(&with_profile);
    let paths: Vec<&str> = rows.iter().filter(|r| r.spec.key == "profile").map(|r| r.path.as_str()).collect();
    assert_eq!(paths, ["cases[id=c1].numbers[id=profile.0].value", "cases[id=c1].numbers[id=profile.1].value", "cases[id=c1].numbers[id=profile.2].value"]);
    assert_eq!(case_input_field_meta(VOCABULARY, "cases[id=c1].numbers[id=profile.1].value").map(|f| f.unit()), Some(Some("mm")));
}
