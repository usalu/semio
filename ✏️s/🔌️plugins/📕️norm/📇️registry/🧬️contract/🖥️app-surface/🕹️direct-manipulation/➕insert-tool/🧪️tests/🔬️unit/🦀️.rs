use super::*;

fn spec() -> InsertSpec {
    InsertSpec { id: "member".into(), collection: "members".into(), kind: EntityKind::Member, label_en: "Member".into(), label_de: "Bauteil".into() }
}

fn document(members: Vec<DslValue>) -> DslValue {
    DslValue::object(vec![("members".to_string(), DslValue::Array(members))])
}

fn member(id: &str) -> DslValue {
    DslValue::object(vec![("id".to_string(), DslValue::String(id.into())), ("x".to_string(), DslValue::float(1.0)), ("length".to_string(), DslValue::float(4.0))])
}

#[test]
fn the_cursor_snaps_to_the_grid_unless_freed() {
    assert_eq!(snap_point([1.234, -0.012, 2.49], false), [1.25, -0.0, 2.5]);
    assert_eq!(snap_point([1.234, 0.0, 0.0], true), [1.234, 0.0, 0.0]);
}

#[test]
fn a_placement_copies_the_last_item_with_a_fresh_id_and_the_snapped_position() {
    let edit = insert_edit(&document(vec![member("member-1"), member("member-2")]), &spec(), snap_point([3.02, 0.0, 0.0], false)).expect("edit");
    let NormEdit::InsertItem { path, index, value: Some(value) } = edit else { panic!("an insert edit") };
    assert_eq!((path.as_str(), index), ("members", 2));
    assert_eq!(value.get("id").and_then(DslValue::as_str), Some("member-3"));
    assert_eq!(value.get("x").and_then(DslValue::as_f64), Some(3.0));
    assert_eq!(value.get("length").and_then(DslValue::as_f64), Some(4.0));
    assert!(value.get("y").is_none(), "only fields the template has are positioned");
}

#[test]
fn an_id_never_collides_with_an_existing_one() {
    let edit = insert_edit(&document(vec![member("member-2")]), &spec(), [0.0; 3]).expect("edit");
    let NormEdit::InsertItem { value: Some(value), .. } = edit else { panic!("an insert edit") };
    assert_eq!(value.get("id").and_then(DslValue::as_str), Some("member-3"));
}

#[test]
fn an_empty_collection_inserts_the_default_item_and_an_unknown_one_is_refused() {
    assert_eq!(insert_edit(&document(vec![]), &spec(), [0.0; 3]), Ok(NormEdit::InsertItem { path: "members".into(), index: 0, value: None }));
    assert!(insert_edit(&DslValue::object(vec![]), &spec(), [0.0; 3]).is_err());
}

#[test]
fn a_dotted_collection_path_reaches_nested_arrays_and_id_keyed_maps() {
    let nested = |collection: &str| InsertSpec { collection: collection.into(), ..spec() };
    let array = DslValue::object(vec![("catalogue".to_string(), DslValue::object(vec![("products".to_string(), DslValue::Array(vec![member("member-1")]))]))]);
    let NormEdit::InsertItem { path, index, value: Some(value) } = insert_edit(&array, &nested("catalogue.products"), [2.0, 0.0, 0.0]).expect("nested array") else { panic!("an insert edit") };
    assert_eq!((path.as_str(), index), ("catalogue.products", 1));
    assert_eq!(value.get("id").and_then(DslValue::as_str), Some("member-2"));
    assert_eq!(value.get("x").and_then(DslValue::as_f64), Some(2.0));
    let map = DslValue::object(vec![("geometry".to_string(), DslValue::object(vec![("objects".to_string(), DslValue::object(vec![("a".to_string(), member("member-1")), ("b".to_string(), member("member-2"))]))]))]);
    let NormEdit::InsertItem { path, index, value: Some(value) } = insert_edit(&map, &nested("geometry.objects"), [0.5, 0.0, 0.0]).expect("keyed map") else { panic!("an insert edit") };
    assert_eq!((path.as_str(), index), ("geometry.objects", 2));
    assert_eq!(value.get("id").and_then(DslValue::as_str), Some("member-3"));
    assert!(insert_edit(&map, &nested("geometry.missing"), [0.0; 3]).is_err());
}
