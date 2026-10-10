use super::*;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{ExprPoint, ExprPoint3, Family, FamilyParameter, FamilySolid, ParametricProfile, SolidShape};

/// 🧰️ Builders of authored families for the tests of this module and its children.
pub mod kit {
    use super::*;

    pub fn model() -> ModelSnapshot {
        let mut snapshot = ModelSnapshot::default();
        snapshot.materials.insert("m-steel".into(), crate::Material { name: "Steel".into(), category: crate::MaterialCategory::Metal, color: crate::Rgb { r: 0.5, g: 0.5, b: 0.55 }, density: 7850.0, conductivity: 50.0, specific_heat: 450.0 });
        snapshot
    }

    pub fn family(snapshot: &mut ModelSnapshot, id: &str, category: FamilyCategory) {
        snapshot.families.insert(id.into(), Family { name: id.into(), category });
    }

    pub fn parameter(snapshot: &mut ModelSnapshot, family: &str, name: &str, kind: ParameterKind, value: &str) {
        snapshot.family_parameters.insert(formula::parameter_id(family, name), FamilyParameter { family: family.into(), name: name.into(), kind, value: value.into() });
    }

    pub fn point3(x: &str, y: &str, z: &str) -> ExprPoint3 {
        ExprPoint3 { x: x.into(), y: y.into(), z: z.into() }
    }

    pub fn point(x: &str, y: &str) -> ExprPoint {
        ExprPoint { x: x.into(), y: y.into() }
    }

    pub fn rectangle(width: &str, depth: &str) -> ParametricProfile {
        ParametricProfile::Rectangle { width: width.into(), depth: depth.into() }
    }

    pub fn solid(snapshot: &mut ModelSnapshot, id: &str, family: &str, shape: SolidShape) {
        snapshot.family_solids.insert(id.into(), FamilySolid { family: family.into(), name: id.into(), shape, material: "\"m-steel\"".into(), visible: "true".into(), offset: point3("0 m", "0 m", "0 m") });
    }

    pub fn extrusion(profile: ParametricProfile, base: &str, height: &str) -> SolidShape {
        SolidShape::Extrusion { profile, base: base.into(), height: height.into() }
    }

    pub fn close(got: f64, want: f64) {
        assert!((got - want).abs() <= 1e-9 * want.abs().max(1.0), "{got} vs {want}");
    }

    pub fn length(value: &FamilyValue, name: &str) -> f64 {
        match value.value(name) {
            Some(ParameterValue::Length { value }) => *value,
            other => panic!("{name} is not a length: {other:?}"),
        }
    }

    /// 🏗️ The family of a wide-flange beam profile (a HEA 200): four lengths and one extruded I-shape three metres long.
    pub fn hea(snapshot: &mut ModelSnapshot) {
        family(snapshot, "fam-hea", FamilyCategory::Profile);
        for (name, text) in [("h", "190 mm"), ("b", "200 mm"), ("tw", "6.5 mm"), ("tf", "10 mm")] {
            parameter(snapshot, "fam-hea", name, ParameterKind::Length, text);
        }
        solid(snapshot, "s-hea", "fam-hea", extrusion(ParametricProfile::IShape { width: "b".into(), depth: "h".into(), web: "tw".into(), flange: "tf".into() }, "0 m", "3 m"));
    }
}

#[test]
fn a_missing_family_is_the_default_value() {
    assert_eq!(family_of(&kit::model(), "fam-ghost", &BTreeMap::new()), FamilyValue::default());
}

#[test]
fn a_profile_family_infers_its_outline_and_volume_from_the_formulas() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    let value = family_of(&snapshot, "fam-hea", &BTreeMap::new());
    assert!(value.issues.is_empty(), "{:?}", value.issues);
    kit::close(kit::length(&value, "h"), 0.19);
    kit::close(kit::length(&value, "tw"), 0.0065);
    assert_eq!(value.outline.len(), 12);
    let area = 2.0 * 0.2 * 0.01 + (0.19 - 2.0 * 0.01) * 0.0065;
    kit::close(value.solids["s-hea"].volume, area * 3.0);
    kit::close(value.volume(), area * 3.0);
    assert_eq!(value.category, Some(FamilyCategory::Profile));
    assert_eq!(value.solids["s-hea"].material, "m-steel");
}

#[test]
fn changing_a_parameter_changes_the_outline_but_not_the_other_families() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    kit::family(&mut snapshot, "fam-table", FamilyCategory::Furniture);
    kit::parameter(&mut snapshot, "fam-table", "top", ParameterKind::Length, "1.2 m");
    let before = family_of(&snapshot, "fam-hea", &BTreeMap::new());
    let table = family_of(&snapshot, "fam-table", &BTreeMap::new());
    snapshot.family_parameters.get_mut("fam-hea.h").expect("h").value = "240 mm".into();
    let after = family_of(&snapshot, "fam-hea", &BTreeMap::new());
    assert_ne!(before.outline, after.outline);
    kit::close(kit::length(&after, "h"), 0.24);
    assert_eq!(table, family_of(&snapshot, "fam-table", &BTreeMap::new()));
}

#[test]
fn a_profile_family_without_a_visible_extrusion_has_no_outline_and_says_so() {
    let mut snapshot = kit::model();
    kit::family(&mut snapshot, "fam-flat", FamilyCategory::Profile);
    let value = family_of(&snapshot, "fam-flat", &BTreeMap::new());
    assert!(value.outline.is_empty());
    assert_eq!(value.issues.len(), 1);
    assert_eq!(value.issues[0].code, FamilyIssueCode::Outline);
    assert_eq!(value.issues[0].owner, IssueOwner::Family);
    let mut hidden = kit::model();
    kit::hea(&mut hidden);
    hidden.family_solids.get_mut("s-hea").expect("solid").visible = "false".into();
    let value = family_of(&hidden, "fam-hea", &BTreeMap::new());
    assert!(value.outline.is_empty(), "a hidden extrusion is not the outline");
    assert!(!value.solids["s-hea"].visible);
    kit::close(value.volume(), 0.0);
}

#[test]
fn the_overrides_of_an_instance_replace_formulas_without_touching_the_family() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    let overrides = BTreeMap::from([("h".to_string(), "300 mm".to_string())]);
    let placed = family_of(&snapshot, "fam-hea", &overrides);
    kit::close(kit::length(&placed, "h"), 0.3);
    assert_eq!(placed.parameters["h"].formula, "300 mm");
    kit::close(kit::length(&family_of(&snapshot, "fam-hea", &BTreeMap::new()), "h"), 0.19);
    let unknown = family_of(&snapshot, "fam-hea", &BTreeMap::from([("ghost".to_string(), "1 m".to_string())]));
    assert!(unknown.issues.iter().any(|issue| issue.code == FamilyIssueCode::Unknown && issue.subject == "ghost"));
}

#[test]
fn the_dependency_is_exactly_what_the_family_reads() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    kit::family(&mut snapshot, "fam-table", FamilyCategory::Furniture);
    let base = dependency(&snapshot, "fam-hea");
    let mut other = snapshot.clone();
    kit::parameter(&mut other, "fam-table", "top", ParameterKind::Length, "1 m");
    assert_eq!(dependency(&other, "fam-hea"), base, "another family's parameter is not read");
    let mut edited = snapshot.clone();
    edited.family_parameters.get_mut("fam-hea.b").expect("b").value = "210 mm".into();
    assert_ne!(dependency(&edited, "fam-hea"), base, "an own parameter is read");
    let mut solid = snapshot.clone();
    solid.family_solids.get_mut("s-hea").expect("solid").visible = "false".into();
    assert_ne!(dependency(&solid, "fam-hea"), base, "an own solid is read");
    let mut materials = snapshot.clone();
    materials.materials.remove("m-steel");
    assert_ne!(dependency(&materials, "fam-hea"), base, "the material ids are read");
    assert_eq!(dependency(&snapshot.clone(), "fam-hea"), base, "deterministic");
}

#[test]
fn family_profiles_resolve_a_family_reference_to_its_outline() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    let value = family_of(&snapshot, "fam-hea", &BTreeMap::new());
    let profiles = FamilyProfiles::new().with("fam-hea", &value.outline);
    let family = Profile::Family { family: "fam-hea".into() };
    assert_eq!(profiles.resolve(&family).as_ref(), &Profile::Custom { outline: value.outline.clone() });
    let rectangle = Profile::Rectangle { width: 1.0, depth: 2.0 };
    assert!(matches!(profiles.resolve(&rectangle), Cow::Borrowed(_)), "a plain profile is not copied");
    assert_eq!(FamilyProfiles::new().resolve(&family).as_ref(), &Profile::Custom { outline: Vec::new() });
    assert_eq!(family_of_profile(&family), Some("fam-hea"));
    assert_eq!(family_of_profile(&rectangle), None);
}

#[test]
fn the_value_survives_the_wire_form() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    let value = family_of(&snapshot, "fam-hea", &BTreeMap::new());
    let wire = semio_framework_value::ToValue::to_value(&value);
    let back: FamilyValue = semio_framework_value::FromValue::from_value(wire).expect("round trip");
    assert_eq!(back, value);
}

fn railing(profile: Profile, post: Profile, baluster: Option<Profile>) -> crate::Railing {
    crate::Railing {
        storey: "st-ground".into(),
        path: Vec::new(),
        height: 1.0,
        post_spacing: 1.0,
        profile,
        post_profile: post,
        baluster: baluster.map(|profile| crate::Baluster { profile, spacing: 0.12 }),
        infill: crate::Infill::None,
        material: "m-steel".into(),
        base_offset: 0.0,
        host: None,
        phase: crate::Phase::New,
        name: "Rail".into(),
    }
}

#[test]
fn a_railing_resolves_every_family_section_and_a_plain_railing_is_not_copied() {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    let value = family_of(&snapshot, "fam-hea", &BTreeMap::new());
    let profiles = FamilyProfiles::new().with("fam-hea", &value.outline);
    let plain = railing(Profile::Rectangle { width: 0.05, depth: 0.04 }, Profile::Circle { diameter: 0.05 }, None);
    assert!(matches!(profiles.resolve_railing(&plain), Cow::Borrowed(_)));
    let named = Profile::Family { family: "fam-hea".into() };
    let outline = Profile::Custom { outline: value.outline.clone() };
    for (rail, post, baluster) in [(named.clone(), plain.post_profile.clone(), None), (plain.profile.clone(), named.clone(), None), (plain.profile.clone(), plain.post_profile.clone(), Some(named.clone()))] {
        let source = railing(rail, post, baluster);
        let resolved = profiles.resolve_railing(&source);
        let held = [&resolved.profile, &resolved.post_profile].into_iter().chain(resolved.baluster.as_ref().map(|row| &row.profile)).filter(|profile| **profile == outline).count();
        assert_eq!(held, 1, "exactly the named section becomes the outline");
        assert!(!matches!(resolved.as_ref().profile, Profile::Family { .. }) && !matches!(resolved.as_ref().post_profile, Profile::Family { .. }));
        assert_eq!(resolved.height, source.height);
    }
    let unknown = railing(named, plain.post_profile.clone(), None);
    assert_eq!(FamilyProfiles::new().resolve_railing(&unknown).profile, Profile::Custom { outline: Vec::new() });
}
