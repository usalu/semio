use super::*;

#[test]
fn parameter_names_follow_the_expression_grammar() {
    for good in ["width", "w2", "_gap", "Höhe", "frame_width"] {
        assert!(is_name(good), "{good}");
    }
    for bad in ["", "2w", "a b", "a.b", "a-b", "if", "and", "true", "x:y"] {
        assert!(!is_name(bad), "{bad:?}");
    }
}

#[test]
fn a_parameter_key_splits_at_its_last_dot() {
    assert_eq!(parameter_id("fam-hea", "width"), "fam-hea.width");
    assert_eq!(split_parameter_id("fam-hea.width"), Some(("fam-hea", "width")));
    assert_eq!(split_parameter_id("v1.2.depth"), Some(("v1.2", "depth")));
    assert_eq!(split_parameter_id("nodot"), None);
    assert_eq!(split_parameter_id(".width"), None);
    assert_eq!(split_parameter_id("fam.2bad"), None);
}

#[test]
fn canonical_text_is_stable_and_equal_formulas_have_equal_text() {
    assert_eq!(canonical("2*width+40mm").expect("parses"), canonical("2 * width + 40 mm").expect("parses"));
    let once = canonical("if a > 1 m then b else c").expect("parses");
    assert_eq!(canonical(&once).expect("parses"), once);
    assert!(is_canonical(&once));
    assert!(!is_canonical("2*width"));
    assert!(!is_canonical("2 *"));
    assert!(canonical("2 *").is_err());
}

#[test]
fn references_name_every_parameter_of_the_text_and_none_for_broken_text() {
    assert_eq!(references("a + b * 2 m"), BTreeSet::from(["a".to_string(), "b".to_string()]));
    assert_eq!(references("if flag then x else y"), BTreeSet::from(["flag".to_string(), "x".to_string(), "y".to_string()]));
    assert_eq!(references("`frame width` / 2"), BTreeSet::from(["frame width".to_string()]));
    assert!(references("2 *").is_empty());
    assert!(references("1 m").is_empty());
}

fn solid(shape: SolidShape) -> FamilySolid {
    let zero = || "0 m".to_string();
    FamilySolid { family: "f".into(), name: "s".into(), shape, material: "\"m\"".into(), visible: "true".into(), offset: crate::ExprPoint3 { x: zero(), y: zero(), z: zero() } }
}

#[test]
fn every_formula_slot_of_a_solid_is_listed_once_with_its_field_name() {
    let fields = |solid: &FamilySolid| solid_slots(solid).into_iter().map(|(field, _)| field).collect::<Vec<_>>();
    let rectangle = ParametricProfile::Rectangle { width: "w".into(), depth: "d".into() };
    let extrusion = solid(SolidShape::Extrusion { profile: rectangle.clone(), base: "0 m".into(), height: "h".into() });
    assert_eq!(fields(&extrusion), ["profile.width", "profile.depth", "base", "height", "material", "visible", "offset.x", "offset.y", "offset.z"]);
    let points = vec![ExprPoint { x: "0 m".into(), y: "0 m".into() }, ExprPoint { x: "1 m".into(), y: "2 m".into() }];
    let sweep = solid(SolidShape::Sweep { profile: ParametricProfile::Circle { diameter: "d".into() }, path: points.clone() });
    assert_eq!(fields(&sweep), ["profile.diameter", "path[0].x", "path[0].y", "path[1].x", "path[1].y", "material", "visible", "offset.x", "offset.y", "offset.z"]);
    let turned = solid(SolidShape::Revolution { profile: ParametricProfile::Polygon { points }, axis: crate::SolidAxis::Z, angle: "90 deg".into() });
    assert_eq!(fields(&turned)[..5], ["profile.points[0].x", "profile.points[0].y", "profile.points[1].x", "profile.points[1].y", "angle"]);
    let cuboid = solid(SolidShape::Cuboid { x: "1 m".into(), y: "2 m".into(), z: "3 m".into(), width: "w".into(), depth: "d".into(), height: "h".into() });
    assert_eq!(fields(&cuboid)[..6], ["x", "y", "z", "width", "depth", "height"]);
    let i_shape = solid(SolidShape::Extrusion { profile: ParametricProfile::IShape { width: "a".into(), depth: "b".into(), web: "c".into(), flange: "d".into() }, base: "0 m".into(), height: "1 m".into() });
    assert_eq!(solid_slots(&i_shape)[2], ("profile.web".to_string(), "c"));
    let texts: Vec<&str> = solid_slots(&extrusion).into_iter().map(|(_, text)| text).collect();
    assert!(texts.contains(&"h") && texts.contains(&"\"m\""));
}

#[test]
fn every_slot_of_a_solid_can_be_replaced_by_its_name_and_only_that_slot() {
    let points = vec![ExprPoint { x: "0 m".into(), y: "0 m".into() }, ExprPoint { x: "1 m".into(), y: "2 m".into() }, ExprPoint { x: "3 m".into(), y: "1 m".into() }];
    let shapes = [
        SolidShape::Extrusion { profile: ParametricProfile::IShape { width: "a".into(), depth: "b".into(), web: "c".into(), flange: "d".into() }, base: "0 m".into(), height: "1 m".into() },
        SolidShape::Revolution { profile: ParametricProfile::Polygon { points: points.clone() }, axis: crate::SolidAxis::X, angle: "90 deg".into() },
        SolidShape::Sweep { profile: ParametricProfile::Circle { diameter: "d".into() }, path: points[..2].to_vec() },
        SolidShape::Cuboid { x: "1 m".into(), y: "2 m".into(), z: "3 m".into(), width: "w".into(), depth: "d".into(), height: "h".into() },
        SolidShape::Extrusion { profile: ParametricProfile::Rectangle { width: "w".into(), depth: "d".into() }, base: "0 m".into(), height: "h".into() },
    ];
    for shape in shapes {
        let original = solid(shape);
        for (field, _) in solid_slots(&original) {
            let mut edited = original.clone();
            assert!(set_slot(&mut edited, &field, "MARK"), "{field}");
            let changed: Vec<String> = solid_slots(&edited).into_iter().filter(|(_, text)| *text == "MARK").map(|(name, _)| name).collect();
            assert_eq!(changed, vec![field.clone()], "exactly the slot {field} holds the mark");
        }
        let mut untouched = original.clone();
        assert!(!set_slot(&mut untouched, "no.such.slot", "MARK"));
        assert_eq!(untouched, original);
    }
}
