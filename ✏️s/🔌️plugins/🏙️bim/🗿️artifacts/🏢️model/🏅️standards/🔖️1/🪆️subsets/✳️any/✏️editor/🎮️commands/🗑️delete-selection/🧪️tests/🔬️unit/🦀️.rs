use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_deleted_container_takes_its_selected_contents_with_its_cascade() {
    let snapshot = demo();
    let Plan { mutations, unsupported, remaining } = plan(&snapshot, &["st-ground".to_string(), "w-south".to_string(), "w-east".to_string()]);
    assert!(unsupported.is_empty() && remaining.is_empty());
    assert!(matches!(mutations.as_slice(), [ModelMutation::DeleteElements(DeleteElements { ids })] if ids == &["st-ground".to_string()]), "the storey's own cascade removes its walls");
    let after = crate::mutations::apply_model_mutation(&snapshot, &mutations[0]).expect("the storey leaves");
    assert!(!after.storeys.contains_key("st-ground") && after.walls.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn elements_go_before_the_library_entries_they_use() {
    let Plan { mutations, .. } = plan(&demo(), &["m-brick".to_string(), "wt-300".to_string(), "w-south".to_string()]);
    assert!(matches!(mutations.as_slice(), [ModelMutation::DeleteElements(_), ModelMutation::DeleteWallType(_), ModelMutation::DeleteMaterial(_)]), "{mutations:?}");
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
    let Plan { mutations, unsupported, .. } = plan(&snapshot, &["w-south".to_string(), "m-brick".to_string()]);
    assert_eq!((mutations.len(), unsupported.len()), (2, 0));
}

fn crowded(count: usize) -> (ModelSnapshot, Vec<String>) {
    let mut snapshot = demo();
    for (kind, id, parent) in [("column-type", "ct-1", ""), ("column", "c-0", "st-ground")] {
        let create = crate::editor::bim::entities::kind_of(kind).and_then(|row| row.create).expect("a creatable kind");
        snapshot = crate::mutations::apply_model_mutation(&snapshot, &create(&snapshot, id, parent, "Sample").expect("creates")).expect("applies");
    }
    let column = snapshot.columns.remove("c-0").expect("the column");
    let ids: Vec<String> = (0..count).map(|index| format!("c-{index:06}")).collect();
    for id in &ids {
        snapshot.columns.insert(id.clone(), column.clone());
    }
    (snapshot, ids)
}

#[semio_framework_async_macros::async_test]
async fn any_number_of_elements_leave_as_parts_of_a_bounded_inverse() {
    let (snapshot, ids) = crowded(20_000);
    let Plan { mutations, remaining, .. } = plan(&snapshot, &ids);
    assert!(remaining.is_empty() && mutations.len() > 2, "{} parts", mutations.len());
    let mut after = snapshot.clone();
    for mutation in &mutations {
        assert!(protocol::Mutation::<ModelSnapshot>::inverse_rows(mutation) <= INVERSE_ROWS, "every part declares a bounded inverse");
        let inverse = protocol::Mutation::<ModelSnapshot>::inverse(mutation, &after).expect("the inverse");
        assert!(inverse.len() <= INVERSE_ROWS, "{} rows", inverse.len());
        after = crate::mutations::apply_model_mutation(&after, mutation).expect("the part leaves");
    }
    assert!(ids.iter().all(|id| !after.columns.contains_key(id)));
    assert!(store::ArtifactStoreOneItemFootprint::for_gesture::<ModelSnapshot, ModelMutation>(&mutations).is_ok(), "the gesture fits the store");
}

#[semio_framework_async_macros::async_test]
async fn a_selection_beyond_one_gesture_streams_and_the_rest_stays_selected() {
    let (snapshot, ids) = crowded(70_000);
    let first = plan(&snapshot, &ids);
    assert!(!first.mutations.is_empty() && !first.remaining.is_empty(), "{} parts, {} left", first.mutations.len(), first.remaining.len());
    assert!(store::ArtifactStoreOneItemFootprint::for_gesture::<ModelSnapshot, ModelMutation>(&first.mutations).is_ok());
    let mut after = snapshot.clone();
    for mutation in &first.mutations {
        after = crate::mutations::apply_model_mutation(&after, mutation).expect("the part leaves");
    }
    let second = plan(&after, &first.remaining);
    assert!(second.remaining.is_empty(), "the second press finishes the selection");
    for mutation in &second.mutations {
        after = crate::mutations::apply_model_mutation(&after, mutation).expect("the part leaves");
    }
    assert!(ids.iter().all(|id| !after.columns.contains_key(id)));
}

#[semio_framework_async_macros::async_test]
async fn the_press_keeps_a_storey_selected_until_its_contents_are_gone() {
    let (snapshot, _) = crowded(20_000);
    let Plan { mutations, remaining, .. } = plan(&snapshot, &["st-ground".to_string()]);
    assert_eq!(remaining, vec!["st-ground".to_string()], "the storey outgrows one removal, its contents go first");
    let mut after = snapshot.clone();
    for mutation in &mutations {
        after = crate::mutations::apply_model_mutation(&after, mutation).expect("the part leaves");
    }
    assert!(after.columns.is_empty() && after.storeys.contains_key("st-ground"));
    let last = plan(&after, &remaining);
    assert!(last.remaining.is_empty() && matches!(last.mutations.as_slice(), [ModelMutation::DeleteElements(_)]));
}

#[semio_framework_async_macros::async_test]
async fn one_removal_above_the_bound_is_refused_and_has_no_inverse() {
    let (snapshot, ids) = crowded(INVERSE_ROWS + 1);
    let whole = ModelMutation::DeleteElements(DeleteElements { ids });
    assert!(crate::mutations::apply_model_mutation(&snapshot, &whole).is_err(), "the bound is a promise the diff keeps");
    assert!(protocol::Mutation::<ModelSnapshot>::inverse(&whole, &snapshot).expect("an inverse").is_empty());
}
