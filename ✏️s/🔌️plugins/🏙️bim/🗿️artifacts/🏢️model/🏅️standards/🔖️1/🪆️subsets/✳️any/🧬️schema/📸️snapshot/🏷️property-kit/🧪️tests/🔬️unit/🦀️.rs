use super::*;

fn real(value: f64) -> PropertyValue {
    PropertyValue::Real { value }
}

fn text(value: &str) -> PropertyValue {
    PropertyValue::Text { value: value.into() }
}

fn definition(kind: PropertyKind) -> PropertyDef {
    PropertyDef { name: "P".into(), kind, unit: None, description: None, required: false, default_value: None, allowed: Vec::new(), minimum: None, maximum: None }
}

fn item(code: &str, parent: Option<&str>) -> ClassificationItem {
    ClassificationItem { code: code.into(), title: format!("Title of {code}"), parent: parent.map(str::to_string) }
}

#[test]
fn a_value_knows_its_kind_its_number_and_its_text() {
    assert_eq!(real(1.5).kind(), PropertyKind::Real);
    assert_eq!(PropertyValue::Integer { value: 3 }.number(), Some(3.0));
    assert_eq!(text("a").number(), None);
    assert!(!real(f64::NAN).is_finite() && text("a").is_finite());
    assert_eq!((real(0.3).display(), PropertyValue::Boolean { value: true }.display(), text("EI30").display()), ("0.3".to_string(), "true".to_string(), "EI30".to_string()));
    assert!(real(0.1 + 0.2).same_as(&real(0.3)) && !real(0.3).same_as(&PropertyValue::Length { value: 0.3 }));
}

#[test]
fn a_value_is_checked_for_kind_range_and_enumeration_in_that_order() {
    let mut bounded = definition(PropertyKind::Real);
    bounded.minimum = Some(0.0);
    bounded.maximum = Some(5.0);
    assert_eq!(bounded.violation(&real(2.0)), None);
    assert_eq!(bounded.violation(&real(-0.1)), Some(Violation::BelowMinimum));
    assert_eq!(bounded.violation(&real(5.1)), Some(Violation::AboveMaximum));
    assert_eq!(bounded.violation(&text("2")), Some(Violation::KindMismatch));
    let mut listed = definition(PropertyKind::Text);
    listed.allowed = vec![text("EI30"), text("EI60")];
    assert_eq!(listed.violation(&text("EI60")), None);
    assert_eq!(listed.violation(&text("EI90")), Some(Violation::NotAllowed));
}

#[test]
fn a_definition_must_be_sound() {
    let mut bad = definition(PropertyKind::Text);
    bad.name = " ".into();
    assert_eq!(bad.problem().map(|row| row.0), Some("name"));
    let mut ranged_text = definition(PropertyKind::Text);
    ranged_text.minimum = Some(1.0);
    assert_eq!(ranged_text.problem().map(|row| row.0), Some("minimum"));
    let mut inverted = definition(PropertyKind::Real);
    inverted.minimum = Some(3.0);
    inverted.maximum = Some(1.0);
    assert_eq!(inverted.problem().map(|row| row.0), Some("minimum"));
    let mut twice = definition(PropertyKind::Text);
    twice.allowed = vec![text("a"), text("a")];
    assert_eq!(twice.problem().map(|row| row.0), Some("allowed"));
    let mut default_outside = definition(PropertyKind::Real);
    default_outside.maximum = Some(1.0);
    default_outside.default_value = Some(real(2.0));
    assert_eq!(default_outside.problem().map(|row| row.0), Some("default_value"));
    let mut default_kind = definition(PropertyKind::Real);
    default_kind.default_value = Some(text("x"));
    assert_eq!(default_kind.problem().map(|row| row.0), Some("default_value"));
    assert_eq!(definition(PropertyKind::Length).problem(), None);
}

#[test]
fn a_template_needs_a_name_each_kind_once_and_unique_property_names() {
    let (a, b) = (definition(PropertyKind::Text), definition(PropertyKind::Real));
    assert_eq!(template_problem("Pset", &[TemplateTarget::Wall], std::slice::from_ref(&a)), None);
    assert_eq!(template_problem(" ", &[], &[]).map(|row| row.0), Some("name"));
    assert_eq!(template_problem("Pset", &[TemplateTarget::Wall, TemplateTarget::Wall], &[]).map(|row| row.0), Some("applies_to"));
    assert_eq!(template_problem("Pset", &[], &[a, b]).map(|row| row.0), Some("properties"));
}

#[test]
fn the_target_names_are_unique_and_a_type_serves_its_instances() {
    let names: std::collections::BTreeSet<&str> = TemplateTarget::ALL.iter().map(|target| target.name()).collect();
    assert_eq!(names.len(), TemplateTarget::ALL.len());
    assert_eq!(TemplateTarget::WallType.served(), Some(TemplateTarget::Wall));
    assert!(TemplateTarget::DoorType.is_type() && !TemplateTarget::Door.is_type());
    assert_eq!(TemplateTarget::Wall.served(), None);
}

#[test]
fn an_entry_table_has_unique_codes_existing_parents_and_no_cycle() {
    assert_eq!(entries_problem(&[item("A", None), item("B", Some("A"))]), None);
    assert!(entries_problem(&[item("", None)]).is_some());
    assert!(entries_problem(&[item("A", None), item("A", None)]).is_some());
    assert!(entries_problem(&[item("A", Some("Z"))]).is_some());
    assert!(entries_problem(&[item("A", Some("A"))]).is_some());
    assert!(entries_problem(&[item("A", Some("B")), item("B", Some("A"))]).is_some());
    assert!(entries_problem(&[item("R", None), item("A", Some("B")), item("B", Some("C")), item("C", Some("B"))]).is_some());
}

#[test]
fn a_table_is_navigated_as_a_tree() {
    let system = ClassificationSystem { name: "S".into(), edition: String::new(), source: None, entries: vec![item("A", None), item("A1", Some("A")), item("A11", Some("A1")), item("B", None), item("A2", Some("A"))] };
    assert_eq!(system.tree().iter().map(|(depth, entry)| (*depth, entry.code.as_str())).collect::<Vec<_>>(), [(0, "A"), (1, "A1"), (2, "A11"), (1, "A2"), (0, "B")]);
    assert_eq!(system.lineage("A11").iter().map(|entry| entry.code.as_str()).collect::<Vec<_>>(), ["A", "A1", "A11"]);
    assert_eq!((system.depth("A11"), system.depth("B"), system.depth("Z")), (Some(2), Some(0), None));
    assert_eq!(system.children(Some("A")).len(), 2);
    assert_eq!(system.children(None).len(), 2);
    assert!(system.lineage("Z").is_empty());
}
