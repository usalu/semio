use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::PropertyValue;

fn set(snapshot: &ModelSnapshot, ids: &[&str], pset: &str, property: &str, value: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&SetProperty { ids: ids.iter().map(|id| id.to_string()).collect(), pset: pset.into(), property: property.into(), value: value.into() }, doc, cfg, &mut ctx))
}

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

fn stored(snapshot: &ModelSnapshot, id: &str, pset: &str, property: &str) -> Option<PropertyValue> {
    snapshot.properties.get(id)?.get(pset)?.get(property).cloned()
}

#[semio_framework_async_macros::async_test]
async fn an_entry_on_the_selection_creates_a_property_whose_kind_the_value_shows() {
    let snapshot = demo();
    let emit = set(&snapshot, &[], "", "", "Pset_WallCommon.FireRating = 90", &["w-south"]).expect("sets");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetElementProperty(_)]));
    assert_eq!(stored(&applied(&snapshot, &emit), "w-south", "Pset_WallCommon", "FireRating"), Some(PropertyValue::Integer { value: 90 }));
    for (entry, expected) in [("Pset.Load = 2.5", PropertyValue::Real { value: 2.5 }), ("Pset.Load = true", PropertyValue::Boolean { value: true }), ("Pset.Load = oak", PropertyValue::Text { value: "oak".into() })] {
        let emit = set(&snapshot, &["w-south"], "", "", entry, &[]).expect("sets");
        assert_eq!(stored(&applied(&snapshot, &emit), "w-south", "Pset", "Load"), Some(expected), "{entry}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_named_kind_wins_and_an_existing_property_keeps_its_kind() {
    let snapshot = demo();
    let emit = set(&snapshot, &["w-south"], "", "", "Dim.Width:length = 3", &[]).expect("sets");
    let with_length = applied(&snapshot, &emit);
    assert_eq!(stored(&with_length, "w-south", "Dim", "Width"), Some(PropertyValue::Length { value: 3.0 }));
    let edit = set(&with_length, &["w-south"], "Dim", "Width", "3.5", &[]).expect("edits the length");
    assert_eq!(stored(&applied(&with_length, &edit), "w-south", "Dim", "Width"), Some(PropertyValue::Length { value: 3.5 }), "a length stays a length");
    assert!(set(&with_length, &["w-south"], "Dim", "Width", "3", &[]).expect("same value").artifact_mutations.is_empty(), "the same value writes nothing");
}

#[semio_framework_async_macros::async_test]
async fn several_elements_get_the_same_property_in_one_emit() {
    let snapshot = demo();
    let emit = set(&snapshot, &["w-south", "w-east"], "P", "n", "1", &[]).expect("sets both");
    assert_eq!(emit.artifact_mutations.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn a_malformed_entry_a_mismatching_value_or_no_element_is_refused_with_its_own_code() {
    let snapshot = demo();
    assert_eq!(code(set(&snapshot, &["w-south"], "", "", "no equals sign", &[])), Some("bim.property.name-invalid".to_string()));
    assert_eq!(code(set(&snapshot, &["w-south"], "", "", "Pset.Width:length = tall", &[])), Some("bim.property.value-invalid".to_string()));
    assert_eq!(code(set(&snapshot, &["w-south"], "", "", "Pset.Width:metres = 3", &[])), Some("bim.property.value-invalid".to_string()));
    assert_eq!(code(set(&snapshot, &[], "P", "n", "1", &[])), Some("bim.property.target-missing".to_string()));
    assert_eq!(code(set(&snapshot, &["m-brick"], "P", "n", "1", &[])), Some("bim.property.target-missing".to_string()), "library entries carry no properties");
}
