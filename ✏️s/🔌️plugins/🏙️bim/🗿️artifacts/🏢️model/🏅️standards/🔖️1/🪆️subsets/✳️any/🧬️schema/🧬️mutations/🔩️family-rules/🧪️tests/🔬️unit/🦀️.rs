use super::*;
use crate::{BeamType, ColumnType, ExprPoint, ExprPoint3, FamilyCategory, FamilyParameter, ParameterKind};

fn base() -> ModelSnapshot {
    let mut base = ModelSnapshot::default();
    base.families.insert("fam".into(), Family { name: "Family".into(), category: FamilyCategory::Profile });
    base
}

fn parameter(base: &mut ModelSnapshot, name: &str, value: &str) {
    base.family_parameters.insert(formula::parameter_id("fam", name), FamilyParameter { family: "fam".into(), name: name.into(), kind: ParameterKind::Length, value: value.into() });
}

fn solid(shape: SolidShape) -> FamilySolid {
    let zero = || "0 m".to_string();
    FamilySolid { family: "fam".into(), name: "body".into(), shape, material: "\"m\"".into(), visible: "true".into(), offset: ExprPoint3 { x: zero(), y: zero(), z: zero() } }
}

fn extrusion(height: &str) -> SolidShape {
    SolidShape::Extrusion { profile: ParametricProfile::Rectangle { width: "w".into(), depth: "0.2 m".into() }, base: "0 m".into(), height: height.into() }
}

#[test]
fn a_family_needs_a_name() {
    assert!(family_record_fault(&Family { name: "Table".into(), category: FamilyCategory::Furniture }).is_none());
    let fault = family_record_fault(&Family { name: "  ".into(), category: FamilyCategory::Furniture }).expect("blank");
    assert_eq!((fault.code, fault.path(Some("family"))), (OutcomeCode::Invariant, vec!["family".to_string(), "name".to_string()]));
    assert_eq!(fault.path(None), vec!["name".to_string()]);
}

#[test]
fn formulas_must_parse_and_names_must_be_names() {
    assert!(formula_fault("2 * w + 40 mm", "value").is_none());
    let fault = formula_fault("2 *", "value").expect("broken");
    assert_eq!((fault.code, fault.field.as_str()), (OutcomeCode::Invariant, "value"));
    assert!(fault.message.contains("does not parse"));
    assert!(name_fault("width").is_none());
    for bad in ["", "2w", "a b", "a.b", "if"] {
        assert!(name_fault(bad).is_some(), "{bad:?}");
    }
}

#[test]
fn a_solid_needs_its_family_a_name_parsing_formulas_known_names_and_enough_points() {
    let mut base = base();
    parameter(&mut base, "w", "200 mm");
    assert!(solid_fault(&base, &solid(extrusion("1 m"))).is_none());
    let ghost = FamilySolid { family: "ghost".into(), ..solid(extrusion("1 m")) };
    let fault = solid_fault(&base, &ghost).expect("missing family");
    assert_eq!((fault.code, fault.field.as_str()), (OutcomeCode::TargetMissing, "family"));
    let unnamed = FamilySolid { name: " ".into(), ..solid(extrusion("1 m")) };
    assert_eq!(solid_fault(&base, &unnamed).map(|fault| fault.field), Some("name".to_string()));
    let fault = solid_fault(&base, &solid(extrusion("1 m +"))).expect("broken height");
    assert_eq!((fault.code, fault.field.as_str()), (OutcomeCode::Invariant, "height"));
    let mut late = solid(extrusion("1 m"));
    late.visible = "(".into();
    assert_eq!(solid_fault(&base, &late).map(|fault| fault.field), Some("visible".to_string()));
    let few = SolidShape::Extrusion { profile: ParametricProfile::Polygon { points: vec![ExprPoint { x: "0 m".into(), y: "0 m".into() }, ExprPoint { x: "1 m".into(), y: "0 m".into() }] }, base: "0 m".into(), height: "1 m".into() };
    assert_eq!(solid_fault(&base, &solid(few)).map(|fault| fault.field), Some("shape".to_string()));
    let short = SolidShape::Sweep { profile: ParametricProfile::Circle { diameter: "0.1 m".into() }, path: vec![ExprPoint { x: "0 m".into(), y: "0 m".into() }] };
    assert_eq!(solid_fault(&base, &solid(short)).map(|fault| fault.field), Some("shape".to_string()));
    let ghost_param = solid(extrusion("ghost * 2"));
    let fault = solid_fault(&base, &ghost_param).expect("unknown name");
    assert_eq!((fault.code, fault.field.as_str()), (OutcomeCode::TargetMissing, "height"));
    assert!(fault.message.contains("ghost"));
    assert!(shape_fault(&SolidShape::Cuboid { x: "0 m".into(), y: "0 m".into(), z: "0 m".into(), width: "1 m".into(), depth: "1 m".into(), height: "1 m".into() }).is_none());
}

#[test]
fn a_profile_family_is_used_by_types_curtain_walls_and_railings_that_name_it() {
    let mut base = base();
    assert_eq!(profile_user(&base, "fam"), None);
    let used = Profile::Family { family: "fam".into() };
    let other = Profile::Family { family: "other".into() };
    base.column_types.insert("ct".into(), ColumnType { name: "C".into(), profile: other.clone(), material: "m".into() });
    base.beam_types.insert("bt".into(), BeamType { name: "B".into(), profile: Profile::Rectangle { width: 1.0, depth: 1.0 }, material: "m".into() });
    assert_eq!(profile_user(&base, "fam"), None, "another family or a plain profile is not a use");
    base.beam_types.get_mut("bt").expect("bt").profile = used.clone();
    assert_eq!(profile_user(&base, "fam"), Some("beam types"));
    base.beam_types.get_mut("bt").expect("bt").profile = other;
    base.column_types.get_mut("ct").expect("ct").profile = used;
    assert_eq!(profile_user(&base, "fam"), Some("column types"));
}

#[test]
fn a_parameter_is_used_by_another_formula_or_a_solid_slot_and_never_by_itself() {
    let mut base = base();
    parameter(&mut base, "w", "200 mm");
    parameter(&mut base, "double", "2 * w");
    parameter(&mut base, "alone", "1 m");
    assert_eq!(parameter_user(&base, "fam", "w"), Some("the formula of parameter \"double\"".to_string()));
    assert_eq!(parameter_user(&base, "fam", "alone"), None);
    assert_eq!(parameter_user(&base, "fam", "double"), None);
    base.family_parameters.remove("fam.double");
    assert_eq!(parameter_user(&base, "fam", "w"), None);
    base.family_solids.insert("body".into(), solid(extrusion("1 m")));
    assert_eq!(parameter_user(&base, "fam", "w"), Some("the profile.width of solid \"body\"".to_string()));
    assert_eq!(parameter_user(&base, "other", "w"), None, "another family's parameter is not this one");
    parameter(&mut base, "self_ref", "self_ref + 1 m");
    assert_eq!(parameter_user(&base, "fam", "self_ref"), None);
}

#[test]
fn a_formula_may_only_use_parameters_that_exist_and_never_close_a_circle() {
    let mut base = base();
    parameter(&mut base, "a", "1 m");
    parameter(&mut base, "b", "a + 1 m");
    assert!(reference_fault(&base, "fam", "a + b", "value").is_none());
    assert!(reference_fault(&base, "fam", "2 m", "value").is_none());
    let fault = reference_fault(&base, "fam", "a + c + d", "value").expect("unknown");
    assert_eq!((fault.code, fault.field.as_str()), (OutcomeCode::TargetMissing, "value"));
    assert!(fault.message.contains("c\", \"d"), "{}", fault.message);
    assert!(reference_fault(&base, "other", "a", "value").is_some(), "another family has no parameter a");
    assert!(cycle_fault(&base, "fam", "c", "a + b").is_none(), "a new parameter that depends on others closes nothing");
    assert!(cycle_fault(&base, "fam", "b", "a * 2").is_none(), "a rewrite that stays acyclic");
    let fault = cycle_fault(&base, "fam", "a", "b").expect("a depends on b which depends on a");
    assert_eq!((fault.code, fault.field.as_str()), (OutcomeCode::Invariant, "value"));
    assert!(cycle_fault(&base, "fam", "a", "a + 1 m").is_some(), "self reference");
    assert!(cycle_fault(&base, "fam", "z", "z").is_some(), "self reference of a new parameter");
}

#[test]
fn parameters_come_in_the_order_their_formulas_need_them() {
    let mut base = base();
    parameter(&mut base, "a_last", "b_mid + 1 m");
    parameter(&mut base, "b_mid", "z_first * 2");
    parameter(&mut base, "z_first", "1 m");
    parameter(&mut base, "c_free", "3 m");
    let names: Vec<&str> = parameters_in_order(&base, "fam").iter().map(|row| row.name.as_str()).collect();
    let at = |name: &str| names.iter().position(|row| *row == name).expect(name);
    assert_eq!(names.len(), 4);
    assert!(at("z_first") < at("b_mid") && at("b_mid") < at("a_last"));
    assert!(parameters_in_order(&base, "other").is_empty());
    parameter(&mut base, "loop_a", "loop_b");
    parameter(&mut base, "loop_b", "loop_a");
    let names: Vec<&str> = parameters_in_order(&base, "fam").iter().map(|row| row.name.as_str()).collect();
    assert_eq!(&names[names.len() - 2..], ["loop_a", "loop_b"], "a circle comes last, by name");
}
