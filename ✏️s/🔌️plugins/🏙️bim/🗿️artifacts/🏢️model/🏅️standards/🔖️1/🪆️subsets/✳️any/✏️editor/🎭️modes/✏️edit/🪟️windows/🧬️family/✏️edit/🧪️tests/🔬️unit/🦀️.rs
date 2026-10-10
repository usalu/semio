use super::*;
use crate::mutations::apply_model_mutation;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const TABLE: &str = include_str!("../../../../../../../../🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json");

fn model() -> ModelSnapshot {
    from_json_str(TABLE, JsonMemberPolicy::Reject).expect("the table decodes")
}

fn edit(part: &str, op: &str, key: &str, value: &str) -> Edit {
    Edit { part: part.into(), op: op.into(), key: key.into(), value: value.into() }
}

fn applied(snapshot: &ModelSnapshot, change: &Edit) -> ModelSnapshot {
    let mutations = apply(snapshot, "fam-table", change, "fs-fresh").expect("the edit applies");
    mutations.iter().fold(snapshot.clone(), |state, mutation| apply_model_mutation(&state, mutation).unwrap_or_else(|error| panic!("{mutation:?}: {error:?}")))
}

#[test]
fn a_formula_is_canonicalised_and_becomes_one_sparse_set_family_parameter() {
    let snapshot = model();
    let mutations = apply(&snapshot, "fam-table", &edit("parameter", "formula", "width", "1.8m"), "x").expect("applies");
    assert_eq!(mutations, vec![ModelMutation::SetFamilyParameter(SetFamilyParameter { family: "fam-table".into(), name: "width".into(), kind: None, value: Some("1.8 m".into()) })]);
    let next = applied(&snapshot, &edit("parameter", "formula", "width", "1.8m"));
    assert_eq!(next.family_parameters["fam-table.width"].value, "1.8 m");
    assert_eq!(apply(&snapshot, "fam-table", &edit("parameter", "formula", "width", "1.8 *"), "x"), Err("bim.family.formula-invalid"));
}

#[test]
fn a_kind_change_and_a_removal_are_one_mutation_each() {
    let snapshot = model();
    let mutations = apply(&snapshot, "fam-table", &edit("parameter", "kind", "ratio", "length"), "x").expect("applies");
    assert_eq!(mutations, vec![ModelMutation::SetFamilyParameter(SetFamilyParameter { family: "fam-table".into(), name: "ratio".into(), kind: Some(ParameterKind::Length), value: None })]);
    assert_eq!(apply(&snapshot, "fam-table", &edit("parameter", "kind", "ratio", "colour"), "x"), Err("bim.family.kind-unknown"));
    let removed = applied(&snapshot, &edit("parameter", "remove", "label", ""));
    assert!(!removed.family_parameters.contains_key("fam-table.label"));
}

#[test]
fn a_new_parameter_takes_a_free_name_and_the_default_formula_of_its_kind() {
    let mut state = model();
    for (kind, formula, name) in [("Length", "1 m", "new_length"), ("Angle", "0 deg", "new_angle"), ("Real", "1", "new_real"), ("Integer", "1", "new_integer"), ("Boolean", "true", "new_boolean"), ("Text", "\"text\"", "new_text"), ("Material", "\"m-oak\"", "new_material")] {
        state = applied(&state, &edit("parameter", "add", kind, ""));
        let row = &state.family_parameters[&format!("fam-table.{name}")];
        assert_eq!((row.value.as_str(), format!("{:?}", row.kind)), (formula, kind.to_string()));
    }
    state = applied(&state, &edit("parameter", "add", "length", ""));
    assert!(state.family_parameters.contains_key("fam-table.new_length_2"), "the second length gets the next free name");
    let mut bare = model();
    bare.materials.clear();
    assert_eq!(apply(&bare, "fam-table", &edit("parameter", "add", "Material", ""), "x"), Err("bim.family.material-missing"));
    assert_eq!(apply(&model(), "fam-table", &edit("parameter", "add", "Colour", ""), "x"), Err("bim.family.kind-unknown"));
}

#[test]
fn the_family_is_renamed_and_recategorised_by_set_family() {
    let snapshot = model();
    assert_eq!(apply(&snapshot, "fam-table", &edit("family", "rename", "", "Dining table"), "x").expect("applies"), vec![ModelMutation::SetFamily(SetFamily { id: "fam-table".into(), name: Some("Dining table".into()), category: None })]);
    let next = applied(&snapshot, &edit("family", "category", "", "casework"));
    assert_eq!(next.families["fam-table"].category, FamilyCategory::Casework);
    assert_eq!(apply(&snapshot, "fam-table", &edit("family", "category", "", "chair"), "x"), Err("bim.family.category-unknown"));
    assert_eq!(apply(&snapshot, "fam-ghost", &edit("family", "rename", "", "x"), "x"), Err("bim.family.missing"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("family", "paint", "", "x"), "x"), Err("bim.family.operation-unknown"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("wall", "paint", "", "x"), "x"), Err("bim.family.part-unknown"));
}

#[test]
fn every_shape_can_be_added_as_a_valid_solid_with_literal_formulas() {
    let mut state = model();
    for (index, token) in SHAPES.iter().enumerate() {
        let id = format!("fs-new-{index}");
        let mutations = apply(&state, "fam-table", &edit("solid", "add", token, ""), &id).expect("applies");
        state = apply_model_mutation(&state, &mutations[0]).unwrap_or_else(|error| panic!("{token}: {error:?}"));
        let row = &state.family_solids[&id];
        assert_eq!((shape_token(&row.shape), row.name.as_str()), (*token, format!("{token} 1").as_str()));
        assert_eq!((row.material.as_str(), row.visible.as_str()), ("\"m-oak\"", "true"));
    }
    assert_eq!(apply(&state, "fam-table", &edit("solid", "add", "Sphere", ""), "x"), Err("bim.family.shape-unknown"));
    let again = applied(&state, &edit("solid", "add", "Cuboid", ""));
    assert_eq!(again.family_solids["fs-fresh"].name, "Cuboid 2", "the next cuboid of the family gets the next number");
}

#[test]
fn a_slot_edit_changes_exactly_that_formula_and_leaves_the_rest_alone() {
    let snapshot = model();
    let mutations = apply(&snapshot, "fam-table", &edit("solid", "slot", "s-top#height", "40mm"), "x").expect("applies");
    let ModelMutation::SetFamilySolid(patch) = &mutations[0] else { panic!("{mutations:?}") };
    assert!(patch.name.is_none() && patch.material.is_none() && patch.visible.is_none() && patch.offset.is_none());
    assert!(matches!(&patch.shape, Some(SolidShape::Cuboid { height, width, .. }) if height == "40 mm" && width == "width"));
    let visibility = applied(&snapshot, &edit("solid", "slot", "s-top#visible", "width > 1 m"));
    assert_eq!(visibility.family_solids["s-top"].visible, "width > 1 m");
    assert_eq!(visibility.family_solids["s-top"].shape, snapshot.family_solids["s-top"].shape);
    let offset = applied(&snapshot, &edit("solid", "slot", "s-foot#offset.z", "10 mm"));
    assert_eq!(offset.family_solids["s-foot"].offset.z, "10 mm");
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "slot", "s-top#nonsense", "1 m"), "x"), Err("bim.family.slot-unknown"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "slot", "s-top#height", "40 mm +"), "x"), Err("bim.family.formula-invalid"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "slot", "s-ghost#height", "1 m"), "x"), Err("bim.family.solid-missing"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "slot", "s-hea#height", "1 m"), "x"), Err("bim.family.solid-missing"), "a solid of another family is not editable here");
}

#[test]
fn a_solid_is_renamed_turned_and_removed() {
    let snapshot = model();
    let renamed = applied(&snapshot, &edit("solid", "name", "s-top", "Table top"));
    assert_eq!(renamed.family_solids["s-top"].name, "Table top");
    let turned = applied(&snapshot, &edit("solid", "axis", "s-foot", "x"));
    assert!(matches!(turned.family_solids["s-foot"].shape, SolidShape::Revolution { axis: SolidAxis::X, .. }));
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "axis", "s-top", "x"), "x"), Err("bim.family.axis-unavailable"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "axis", "s-foot", "w"), "x"), Err("bim.family.axis-unknown"));
    let removed = applied(&snapshot, &edit("solid", "remove", "s-top", ""));
    assert!(!removed.family_solids.contains_key("s-top"));
    assert_eq!(apply(&snapshot, "fam-table", &edit("solid", "polish", "s-top", ""), "x"), Err("bim.family.operation-unknown"));
}

#[test]
fn removing_a_parameter_a_formula_uses_is_applied_as_a_mutation_the_leaf_refuses() {
    let snapshot = model();
    let mutations = apply(&snapshot, "fam-table", &edit("parameter", "remove", "width", ""), "x").expect("the edit itself applies");
    assert!(apply_model_mutation(&snapshot, &mutations[0]).is_err(), "the leaf refuses while another formula refers to the parameter");
}
