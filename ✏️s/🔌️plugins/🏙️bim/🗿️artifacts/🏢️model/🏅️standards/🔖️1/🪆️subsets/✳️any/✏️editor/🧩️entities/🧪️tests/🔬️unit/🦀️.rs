use super::*;

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn every_kind_is_declared_once() {
    let mut kinds: Vec<&str> = ENTITIES.iter().map(|row| row.kind).collect();
    kinds.sort();
    kinds.dedup();
    assert_eq!(kinds.len(), ENTITIES.len());
    assert!(ENTITIES.iter().any(|row| row.kind == "wall-sweep" && !row.library), "the wall sweep is an element of the model");
}

#[semio_framework_async_macros::async_test]
async fn the_table_reads_every_entity_of_the_demo_model() {
    let snapshot = demo();
    for row in ENTITIES {
        for id in (row.ids)(&snapshot) {
            assert!((row.name)(&snapshot, &id).is_some(), "{} {id} has no name", row.kind);
            for field in row.fields {
                assert!((field.read)(&snapshot, &id).is_some() || matches!(field.key, "width" | "height"), "{} {id} cannot read {}", row.kind, field.key);
            }
        }
    }
    assert!(kind_holding(&snapshot, "w-south").is_some_and(|row| row.kind == "wall"));
    assert!(kind_holding(&snapshot, "wt-300").is_some_and(|row| row.library));
}

#[semio_framework_async_macros::async_test]
async fn the_tree_parents_follow_the_hierarchy() {
    let snapshot = demo();
    let parent = |kind: &str, id: &str| (kind_of(kind).expect("kind").parent)(&snapshot, id);
    assert_eq!(parent("building", "bldg-1").as_deref(), Some("site-1"));
    assert_eq!(parent("storey", "st-ground").as_deref(), Some("bldg-1"));
    assert_eq!(parent("wall", "w-south").as_deref(), Some("st-ground"));
    assert_eq!(storey_of(&snapshot, "w-south").as_deref(), Some("st-ground"));
    assert_eq!(storey_of(&snapshot, "st-first").as_deref(), Some("st-first"));
    assert_eq!(storey_of(&snapshot, "bldg-1"), None);
    assert_eq!(storey_of(&snapshot, "wt-300"), None);
}

#[semio_framework_async_macros::async_test]
async fn storeys_are_ordered_by_level() {
    assert_eq!(ordered_storeys(&demo(), "bldg-1"), vec!["st-ground".to_string(), "st-first".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn authored_fields_with_a_mutation_write_it_and_the_rest_stay_read_only() {
    let snapshot = demo();
    let storey = kind_of("storey").expect("storey");
    let height = storey.fields.iter().find(|field| field.key == "height").expect("height row");
    let write = height.write.expect("storey height is editable");
    assert!(matches!(write(&snapshot, "st-ground", "3.5"), Some(ModelMutation::SetStoreyHeight(_))));
    assert!(write(&snapshot, "st-ground", "tall").is_none());
    let wall = kind_of("wall").expect("wall");
    let field = |key: &str| wall.fields.iter().find(|field| field.key == key).expect("wall field");
    assert!(matches!(field("axis").write.expect("axis is editable")(&snapshot, "w-south", "0, 0 → 4, 0"), Some(ModelMutation::SetWallAxis(_))));
    assert!(matches!(field("top").write.expect("top is editable")(&snapshot, "w-south", "storey top 0.5"), Some(ModelMutation::SetWallTop(_))));
    assert!(field("phase").write.is_none() && field("storey").write.is_none());
}

#[semio_framework_async_macros::async_test]
async fn every_kind_has_a_create_delete_and_rename_mutation() {
    for row in ENTITIES {
        let named_by_content = matches!(row.kind, "tag" | "text-note" | "leader" | "curtain-panel-override");
        assert!(row.create.is_some() && row.delete.is_some() && (row.rename.is_some() || named_by_content), "{} is fully wired", row.kind);
    }
}

#[semio_framework_async_macros::async_test]
async fn every_create_and_delete_mutation_of_the_model_has_its_row() {
    let row_of = |suffix: &str| ENTITIES.iter().find(|row| row.kind == suffix || (suffix == "grid-line" && row.kind == "grid"));
    for kind in crate::mutations::KINDS {
        if let Some(suffix) = kind.strip_prefix("create-").or_else(|| kind.strip_prefix("delete-")).filter(|suffix| *suffix != "elements") {
            let row = row_of(suffix).unwrap_or_else(|| panic!("the entity table has no row for '{kind}'"));
            assert!(if kind.starts_with("create-") { row.create.is_some() } else { row.delete.is_some() }, "{kind} is wired");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_field_write_decodes_the_value_its_own_read_shows() {
    let snapshot = demo();
    for row in ENTITIES {
        for id in (row.ids)(&snapshot) {
            for field in row.fields {
                let (Some(write), Some(value)) = (field.write, (field.read)(&snapshot, &id)) else { continue };
                assert!(write(&snapshot, &id, &value).is_some(), "{} {id} writes '{value}' back into {}", row.kind, field.key);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_kind_creates_a_mutation_that_applies_and_the_library_comes_first() {
    let mut snapshot = demo();
    let mut order: Vec<&EntityKind> = ENTITIES.iter().filter(|row| row.library).collect();
    order.extend(ENTITIES.iter().filter(|row| !row.library));
    for row in order {
        let id = format!("x-new-{}", row.kind);
        let parent = match row.kind {
            "building" => "site-1",
            "storey" | "grid" => "bldg-1",
            "opening" | "wall-sweep" => "w-south",
            "curtain-panel-override" => "x-new-curtain-wall",
            "family-solid" => "x-new-family",
            "site" | "family" | "material" | "wall-type" | "slab-type" | "roof-type" | "column-type" | "beam-type" | "window-type" | "door-type" | "curtain-wall-type" => "",
            _ => "st-ground",
        };
        let mutation = (row.create.expect("create"))(&snapshot, &id, parent, "New").unwrap_or_else(|code| panic!("{} cannot be created: {code}", row.kind));
        snapshot = crate::mutations::apply_model_mutation(&snapshot, &mutation).unwrap_or_else(|refusal| panic!("{} is refused: {refusal:?}", row.kind));
        assert!((row.name)(&snapshot, &id).is_some_and(|name| row.rename.is_none() || name == "New"), "{} exists after its create", row.kind);
        let removed = (row.delete.expect("delete"))(&id);
        let without = crate::mutations::apply_model_mutation(&snapshot, &removed).unwrap_or_else(|refusal| panic!("{} cannot be deleted again: {refusal:?}", row.kind));
        assert!((row.name)(&without, &id).is_none());
    }
}

#[semio_framework_async_macros::async_test]
async fn creating_an_entity_builds_the_create_mutation_or_names_the_missing_reference() {
    let snapshot = demo();
    let create = kind_of("wall").and_then(|row| row.create).expect("wall create");
    assert!(matches!(create(&snapshot, "w-new", "st-ground", "New"), Ok(ModelMutation::CreateWall(_))));
    assert_eq!(create(&snapshot, "w-new", "missing", "New").err(), Some("bim.create.storey-missing"));
    let storey = kind_of("storey").and_then(|row| row.create).expect("storey create");
    let Ok(ModelMutation::CreateStorey(payload)) = storey(&snapshot, "st-second", "bldg-1", "Second") else { panic!("create storey") };
    assert_eq!(payload.storey.level, 2);
}

#[semio_framework_async_macros::async_test]
async fn inferred_rows_read_the_levels_and_the_wall_layout() {
    let snapshot = demo();
    let inference = ModelInference::default();
    let storey = kind_of("storey").expect("storey");
    assert!(storey.inferred.iter().all(|row| (row.read)(&snapshot, &inference, "st-ground").is_none()));
    let computed = <ModelInference as protocol::Inference<ModelSnapshot>>::infer(&snapshot).expect("infer");
    let elevation = storey.inferred.iter().find(|row| row.key == "top_elevation").expect("top elevation row");
    assert_eq!((elevation.read)(&snapshot, &computed, "st-ground").as_deref(), Some("3"));
}

#[semio_framework_async_macros::async_test]
async fn a_top_text_names_a_free_height_a_storey_or_the_roof_slab_or_ceiling_it_follows() {
    for (text, expected) in [
        ("2.5", TopConstraint::Unconnected { height: 2.5 }),
        ("storey top 0.5", TopConstraint::StoreyTop { offset: 0.5 }),
        ("st-first 0.2", TopConstraint::Storey { storey: "st-first".into(), offset: 0.2 }),
        ("roof r-main 0.1", TopConstraint::Roof { roof: "r-main".into(), offset: 0.1 }),
        ("slab sl 0", TopConstraint::Slab { slab: "sl".into(), offset: 0.0 }),
        ("ceiling ce -0.05", TopConstraint::Ceiling { ceiling: "ce".into(), offset: -0.05 }),
    ] {
        assert_eq!(parse_top(text), Some(expected.clone()), "{text}");
        assert_eq!(parse_top(&top_text(&expected)), Some(expected), "{text} reads back");
    }
    assert!(parse_top("roof r-main").is_none(), "an offset is needed");
    assert!(parse_top("roof r-main high").is_none());
}

#[semio_framework_async_macros::async_test]
async fn a_profile_text_names_a_rectangle_circle_i_shape_custom_outline_or_family_and_reads_back() {
    let outline = vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 0.03, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 0.0, y: 0.05 }, bulge: 0.5 }];
    for profile in [Profile::Rectangle { width: 0.02, depth: 0.1 }, Profile::Circle { diameter: 0.04 }, Profile::IShape { width: 0.2, depth: 0.4, web: 0.01, flange: 0.02 }, Profile::Custom { outline }, Profile::Family { family: "fam-1".into() }] {
        assert_eq!(parse_profile(&profile_text(&profile)), Some(profile.clone()), "{profile:?}");
    }
    assert_eq!(parse_profile("rect 0.02 x 0.1"), Some(Profile::Rectangle { width: 0.02, depth: 0.1 }));
    assert_eq!(parse_profile("rectangle 0.05 × 0.1"), Some(Profile::Rectangle { width: 0.05, depth: 0.1 }));
    assert!(parse_profile("custom 0, 0; 1, 0").is_none() && parse_profile("blob 1").is_none() && parse_profile("rect 1").is_none());
}
