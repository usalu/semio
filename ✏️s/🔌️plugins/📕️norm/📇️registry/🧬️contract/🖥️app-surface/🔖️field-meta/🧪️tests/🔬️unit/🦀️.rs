use super::*;

const MM_LENGTH: FieldSpec = FieldSpec {
    kind: FieldKind::Quantity { stored: UnitId::Metre, display: UnitId::Millimetre, units: &[UnitId::Millimetre, UnitId::Metre] },
    range: Some(FieldRange::new(0.0, 100.0)),
    precision: 1,
    ..FieldSpec::new("Span", "Spannweite")
};

const TABLE: &[(&str, FieldSpec)] = &[
    ("beams", FieldSpec::new("Beams", "Träger")),
    ("beams[].span", MM_LENGTH),
    ("beams[].angle", FieldSpec { kind: FieldKind::Angle, ..FieldSpec::new("Angle", "Winkel") }),
    ("beams[].grade", FieldSpec::choice("Grade", "Güte", &[NormFieldChoice { value: "S355", label_en: "S355", label_de: "S355" }])),
];

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-12
}

#[test]
fn lookup_keeps_exact_wildcard_and_longest_prefix_semantics() {
    assert_eq!(lookup_norm_field_meta(TABLE, "beams[id=B1].span").map(|s| s.label.en), Some("Span"));
    assert_eq!(lookup_norm_field_meta(TABLE, "beams[2].span").map(|s| s.label.de), Some("Spannweite"));
    assert_eq!(lookup_norm_field_meta(TABLE, "beams[2].unknown").map(|s| s.label.en), Some("Beams"));
    assert_eq!(lookup_norm_field_meta(TABLE, "columns[0].span"), None);
    let by_hook: NormFieldMetaFn = |path| lookup_norm_field_meta(TABLE, path);
    assert_eq!(resolve_field_meta("beams[7].span.x", Some(by_hook)).map(|s| s.label.en), Some("Span"));
    assert_eq!(resolve_field_meta("beams", Some(empty_field_meta)), None);
}

#[test]
fn labels_resolve_for_both_locales_and_lift_to_the_framework_label() {
    assert_eq!((MM_LENGTH.label(Locale::En), MM_LENGTH.label(Locale::De)), ("Span", "Spannweite"));
    let lifted = MM_LENGTH.label.localized();
    assert_eq!(lifted.resolve(Terminology::Native, Locale::De), "Spannweite");
    assert_eq!(field_label("beams[0].zzz", None, Locale::En), "zzz");
}

#[test]
fn quantity_fields_convert_only_at_the_display_boundary() {
    assert!(close(MM_LENGTH.to_stored(2500.0, UnitId::Millimetre).unwrap(), 2.5));
    assert!(close(MM_LENGTH.to_display(2.5, UnitId::Millimetre).unwrap(), 2500.0));
    assert!(close(MM_LENGTH.parse("2500", UnitId::Millimetre).unwrap(), 2.5));
    assert!(close(MM_LENGTH.parse("2,5 m", UnitId::Millimetre).unwrap(), 2.5));
    assert_eq!(MM_LENGTH.format(2.5, UnitId::Millimetre, Locale::De).unwrap(), "2500,0\u{a0}mm");
    assert!(MM_LENGTH.to_stored(1.0, UnitId::Newton).is_err());
}

#[test]
fn ranges_units_and_presets() {
    assert!(MM_LENGTH.accepts(99.0) && !MM_LENGTH.accepts(101.0) && FieldSpec::new("x", "x").accepts(1e30));
    assert_eq!(MM_LENGTH.units(), vec![UnitId::Millimetre, UnitId::Metre]);
    assert!(FieldSpec::quantity("L", "L", UnitId::Metre).units().contains(&UnitId::Kilometre));
    assert_eq!(MM_LENGTH.display_unit(UnitSystem::Si), Some(UnitId::Millimetre));
    assert_eq!(FieldSpec::quantity("L", "L", UnitId::Metre).display_unit(UnitSystem::Engineering), Some(UnitId::Millimetre));
    assert_eq!(MM_LENGTH.unit(), Some("m"));
}

#[test]
fn angle_fields_store_radians_and_show_degrees() {
    let angle = TABLE[2].1;
    assert!(close(angle.to_display(std::f64::consts::PI, UnitId::Degree).unwrap(), 180.0));
    assert!(close(angle.parse("90 °", UnitId::Degree).unwrap(), std::f64::consts::FRAC_PI_2));
    assert_eq!(angle.display_unit(UnitSystem::Si), Some(UnitId::Degree));
}

#[test]
fn choice_and_plain_fields_expose_their_kind() {
    assert_eq!(TABLE[3].1.choices().map(|c| c.len()), Some(1));
    assert_eq!(TABLE[0].1.kind, FieldKind::Inferred);
    assert_eq!(FieldSpec::EMPTY.affects, Affects::AllChecks);
    assert_eq!(FieldSpec::EMPTY.tier, Tier::Basic);
}
