use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, german, run};

fn create(kind: &str, parent: &str, name: &str, selected: &[&str], german_view: bool) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(selected);
    ctx.view = german_view.then(german);
    run(&snapshot, |doc, cfg| handle(&CreateEntity { kind: kind.into(), parent: parent.into(), name: name.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn a_storey_is_created_above_the_last_one_with_the_given_name() {
    let emit = create("storey", "bldg-1", "Attic", &[], false).expect("creates");
    let after = applied(&demo(), &emit);
    let attic = after.storeys.values().find(|storey| storey.name == "Attic").expect("the new storey");
    assert_eq!((attic.level, attic.building.as_str()), (2, "bldg-1"));
    assert_eq!(emit.effects.len(), 1, "the new entity is selected");
}

#[semio_framework_async_macros::async_test]
async fn the_container_is_the_explicit_parent_then_the_selection_then_the_first_one() {
    let explicit = applied(&demo(), &create("wall", "st-first", "Lone", &["st-ground"], false).expect("creates"));
    assert_eq!(explicit.walls.values().find(|wall| wall.name == "Lone").map(|wall| wall.storey.as_str()), Some("st-first"));
    let selected = applied(&demo(), &create("wall", "", "Picked", &["st-first"], false).expect("creates"));
    assert_eq!(selected.walls.values().find(|wall| wall.name == "Picked").map(|wall| wall.storey.as_str()), Some("st-first"));
    let first = applied(&demo(), &create("wall", "", "Fallback", &[], false).expect("creates"));
    assert_eq!(first.walls.values().find(|wall| wall.name == "Fallback").map(|wall| wall.storey.as_str()), Some("st-ground"));
}

#[semio_framework_async_macros::async_test]
async fn the_default_name_speaks_the_addressed_locale_or_the_kind_id_without_one() {
    let german_name = applied(&demo(), &create("storey", "", "", &[], true).expect("creates"));
    assert!(german_name.storeys.values().any(|storey| storey.name == "Geschoss 3"));
    let neutral = applied(&demo(), &create("storey", "", "", &[], false).expect("creates"));
    assert!(neutral.storeys.values().any(|storey| storey.name == "storey 3"));
}

#[semio_framework_async_macros::async_test]
async fn every_new_entity_gets_an_id_no_collection_holds() {
    let snapshot = demo();
    let emit = create("wall", "", "A", &[], false).expect("creates");
    let ModelMutation::CreateWall(payload) = &emit.artifact_mutations[0] else { panic!("a wall is created") };
    assert!(!crate::editor::bim::entities::id_taken(&snapshot, &payload.id));
}

#[semio_framework_async_macros::async_test]
async fn a_missing_container_or_an_unknown_kind_is_refused_with_a_stable_code() {
    assert_eq!(create("teapot", "", "", &[], false).err().map(|fault| fault.code.0), Some("bim.create.kind-unknown".to_string()));
    let empty = ModelSnapshot::default();
    let mut ctx = ctx(&[]);
    let refused = run(&empty, |doc, cfg| handle(&CreateEntity { kind: "wall".into(), parent: String::new(), name: String::new() }, doc, cfg, &mut ctx));
    assert_eq!(refused.err().map(|fault| fault.code.0), Some("bim.create.storey-missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_panels_offer_an_add_row_for_exactly_the_kinds_with_a_create_mutation() {
    assert!(creatable().iter().all(|row| row.create.is_some()));
    for kind in ["site", "building", "storey", "wall"] {
        assert!(creatable().iter().any(|row| row.kind == kind), "{kind} can be created");
    }
}
