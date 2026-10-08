use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_deleted_container_takes_its_selected_contents_with_its_cascade() {
    let snapshot = demo();
    let (mutations, unsupported) = plan(&snapshot, &["st-ground".to_string(), "w-south".to_string(), "w-east".to_string()]);
    assert!(unsupported.is_empty());
    assert!(matches!(mutations.as_slice(), [ModelMutation::DeleteStorey(_)]), "the storey's own cascade removes its walls");
    let after = crate::mutations::apply_model_mutation(&snapshot, &mutations[0]).expect("the storey leaves");
    assert!(!after.storeys.contains_key("st-ground") && after.walls.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn elements_go_before_the_library_entries_they_use() {
    let (mutations, _) = plan(&demo(), &["m-brick".to_string(), "wt-300".to_string(), "w-south".to_string()]);
    assert!(matches!(mutations.as_slice(), [ModelMutation::DeleteWall(_), ModelMutation::DeleteWallType(_), ModelMutation::DeleteMaterial(_)]), "{mutations:?}");
}

#[semio_framework_async_macros::async_test]
async fn deleting_the_selection_removes_the_entities_and_clears_the_selection() {
    let snapshot = demo();
    let mut ctx = ctx(&["w-south", "w-north"]);
    let emit = run(&snapshot, |doc, cfg| handle(&DeleteSelection { ids: Vec::new() }, doc, cfg, &mut ctx)).expect("deletes");
    let after = applied(&snapshot, &emit);
    assert_eq!(after.walls.keys().cloned().collect::<Vec<_>>(), vec!["w-east".to_string(), "w-west".to_string()]);
    assert_eq!(emit.effects.len(), 2, "both domains' selections are cleared");
}

#[semio_framework_async_macros::async_test]
async fn explicit_ids_win_over_the_selection() {
    let snapshot = demo();
    let mut ctx = ctx(&["w-south"]);
    let emit = run(&snapshot, |doc, cfg| handle(&DeleteSelection { ids: vec!["w-east".into()] }, doc, cfg, &mut ctx)).expect("deletes");
    assert!(!applied(&snapshot, &emit).walls.contains_key("w-east") && applied(&snapshot, &emit).walls.contains_key("w-south"));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_selection_deletes_nothing() {
    let snapshot = demo();
    let mut ctx = ctx(&[]);
    let emit = run(&snapshot, |doc, cfg| handle(&DeleteSelection { ids: Vec::new() }, doc, cfg, &mut ctx)).expect("a no-op");
    assert!(emit.artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn an_unreferenced_library_entry_is_deleted_like_any_element() {
    let snapshot = demo();
    let (mutations, unsupported) = plan(&snapshot, &["w-south".to_string(), "m-brick".to_string()]);
    assert_eq!((mutations.len(), unsupported.len()), (2, 0));
}
