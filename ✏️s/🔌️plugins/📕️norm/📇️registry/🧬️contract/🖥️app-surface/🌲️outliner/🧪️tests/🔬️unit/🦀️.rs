use crate::*;
use semio_framework_value::DslValue;

fn entity(id: &str, name: Option<&str>) -> DslValue {
    let mut entries = vec![("id".to_string(), DslValue::String(id.into()))];
    if let Some(name) = name {
        entries.push(("name".into(), DslValue::String(name.into())));
    }
    DslValue::Object(entries)
}

fn project(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(node)).expect("projection")
}

#[test]
fn titles_prefer_id_and_name_then_position() {
    assert_eq!(entity_title(&entity("b1", Some("Main beam")), 0), "b1 — Main beam");
    assert_eq!(entity_title(&entity("b1", Some("b1")), 0), "b1");
    assert_eq!(entity_title(&entity("b1", None), 0), "b1");
    assert_eq!(entity_title(&DslValue::object([("name".into(), DslValue::String("Slab".into()))]), 4), "Slab");
    assert_eq!(entity_title(&DslValue::object([]), 4), "#5");
}

#[test]
fn the_filter_matches_every_word_case_insensitively_over_title_and_path() {
    assert!(filter_matches("", "anything"));
    assert!(filter_matches("MAIN beam", "b1 — Main beam members[id=b1]"));
    assert!(!filter_matches("column beam", "b1 — Main beam"));
    let items = vec![entity("b1", Some("Main beam")), entity("c2", Some("Column")), entity("b3", None)];
    let rows = collection_rows("members", &items, "beam");
    assert_eq!(rows.iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), ["b1"]);
    assert_eq!(rows[0].path, "members[id=b1]");
    assert_eq!(collection_rows("members", &items, "").len(), 3);
    assert_eq!(collection_rows("members", &items, "members[id=c").len(), 1);
}

#[test]
fn only_object_lists_are_entity_collections() {
    assert!(is_entity_collection(&[]));
    assert!(is_entity_collection(&[entity("a", None)]));
    assert!(!is_entity_collection(&[DslValue::float(1.0)]));
}

#[test]
fn a_collection_renders_rows_with_status_selection_add_remove_and_an_optional_filter() {
    let items = vec![entity("b1", Some("Main beam")), entity("c2", None)];
    let mut report = CheckReport::default();
    report.push(
        semio_s_plugin_norm_compliance::CheckResult::assess(
            "b1.u",
            "p",
            ClauseId::new("EN", "1", "1"),
            semio_s_plugin_norm_compliance::SubjectRef::new("b1", "members[id=b1].length", semio_s_plugin_norm_compliance::LocalizedCopy::new("b1", "b1")),
            semio_s_plugin_norm_compliance::LocalizedCopy::new("t", "t"),
        )
        .utilization(Quantity::new(QuantityKind::Dimensionless, 1.2), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .explanation(semio_s_plugin_norm_compliance::LocalizedCopy::new("x", "x"))
        .build(),
    );
    let state = OutlinerState { selected: vec!["c2".into()], filter: String::new(), filter_action: Some("setOutlinerFilter"), ..Default::default() };
    let windows = TreeWindows::unhosted();
    let context = OutlinerContext { report: &report, locale: Locale::En, controller_id: "norm.mock", windows: &windows, state: &state };
    let node = render_collection(&context, "members", "Members", &items, true, |row| tree_item_desc(format!("body.{}", row.id), norm_ui_label("body").expect("label"), None)).expect("collection");
    let json = project(node);
    assert!(json.contains("Members (2)") && json.contains("b1 — Main beam · ✗ 120 %"), "{json}");
    assert!(json.contains("norm-outliner.members[id=c2]") && json.contains("body.c2"), "{json}");
    assert!(json.contains("insertItem") && json.contains("removeItem") && json.contains("setOutlinerFilter"), "{json}");
    let plain = OutlinerState::default();
    let context = OutlinerContext { state: &plain, ..context };
    let json = project(render_collection(&context, "members", "Members", &items, true, |row| tree_item_desc(format!("body.{}", row.id), norm_ui_label("body").expect("label"), None)).expect("collection"));
    assert!(!json.contains("setOutlinerFilter") && !json.contains(".filter"), "{json}");
}

#[test]
fn a_filtered_collection_reports_shown_over_total() {
    let items = vec![entity("b1", Some("Main beam")), entity("c2", None)];
    let state = OutlinerState { filter: "beam".into(), ..Default::default() };
    let windows = TreeWindows::unhosted();
    let report = CheckReport::default();
    let context = OutlinerContext { report: &report, locale: Locale::De, controller_id: "norm.mock", windows: &windows, state: &state };
    let json = project(render_collection(&context, "members", "Bauteile", &items, true, |row| tree_item_desc(format!("body.{}", row.id), norm_ui_label("x").expect("label"), None)).expect("collection"));
    assert!(json.contains("Bauteile (1/2)") && json.contains("body.b1") && !json.contains("body.c2"), "{json}");
    assert!(json.contains("Entfernen") && json.contains("Eintrag hinzufügen"), "{json}");
}

#[test]
fn the_document_outliner_lists_entity_kinds_first_then_supporting_tables_and_marks_the_selection() {
    let tree = DslValue::object([
        ("sections".into(), DslValue::Array(vec![entity("s1", Some("HEB 240"))])),
        ("members".into(), DslValue::Array(vec![entity("b1", Some("Main beam")), entity("c2", None)])),
        ("annex".into(), DslValue::String("en".into())),
        ("empty".into(), DslValue::Array(Vec::new())),
    ]);
    let entities = [OutlinedCollection { path: "members", label_en: "Members", label_de: "Stäbe" }];
    let state = OutlinerState { selected: vec!["c2".into()], ..Default::default() };
    let windows = TreeWindows::unhosted();
    let report = CheckReport::default();
    let context = OutlinerContext { report: &report, locale: Locale::De, controller_id: "norm.mock", windows: &windows, state: &state };
    let json = project(render_outliner_document(&tree, &entities, None, &context).expect("outliner"));
    let members = json.find("Stäbe (2)").expect("entity kind first");
    let sections = json.find("sections (1)").expect("supporting table second");
    assert!(members < sections && !json.contains("empty (") && !json.contains("annex ("), "{json}");
    assert!(json.contains("norm-outliner.members[id=c2]") && json.contains("b1 — Main beam"), "{json}");
}

#[test]
fn rows_of_an_interaction_domain_are_scoped_pick_targets_the_selection_mirror_resolves() {
    let tree = DslValue::object([("members".into(), DslValue::Array(vec![entity("b1", Some("Main beam")), entity("c2", None)])), ("loads".into(), DslValue::Array(vec![entity("b1", None)]))]);
    let entities = [OutlinedCollection { path: "members", label_en: "Members", label_de: "Stäbe" }, OutlinedCollection { path: "loads", label_en: "Loads", label_de: "Lasten" }];
    let state = OutlinerState { interaction_domain: Some("norm.mock"), ..Default::default() };
    let windows = TreeWindows::unhosted();
    let report = CheckReport::default();
    let context = OutlinerContext { report: &report, locale: Locale::En, controller_id: "norm.mock", windows: &windows, state: &state };
    let json = project(render_outliner_document(&tree, &entities, None, &context).expect("outliner"));
    let member = crate::protocol::wire::encode_scoped_identity(&["members", "b1"]).expect("address");
    let load = crate::protocol::wire::encode_scoped_identity(&["loads", "b1"]).expect("address");
    assert_ne!(member, load, "equal ids of different collections stay distinct");
    assert!(json.contains(&member) && json.contains(&load) && json.contains("\"entity\""), "{json}");
    let row = EntityRow { path: "members[id=b1]".into(), id: "b1".into(), title: "b1".into(), index: 0, value: &entity("b1", None) };
    assert_eq!(row_address(&row), Some(member.clone()));
    let picked = NormSelection::resolve(&report, None, &[member]);
    assert_eq!(picked.entities, ["b1"], "the mirror resolves the scoped row address to the entity");
    assert_eq!(picked.outliner_state_in("norm.mock").interaction_domain, Some("norm.mock"));
}

#[test]
fn a_reorderable_collection_offers_drag_drop_and_up_down_steps_that_raise_move_entity() {
    let items = vec![entity("b1", None), entity("b2", None), entity("b3", None)];
    let report = CheckReport::default();
    let windows = TreeWindows::unhosted();
    let render = |reorderable: Vec<&'static str>, locale| {
        let state = OutlinerState { reorderable, ..Default::default() };
        let context = OutlinerContext { report: &report, locale, controller_id: "norm.mock", windows: &windows, state: &state };
        project(render_collection(&context, "members", "Members", &items, true, |row| tree_item_desc(format!("body.{}", row.id), norm_ui_label("body").expect("label"), None)).expect("collection"))
    };
    let plain = render(Vec::new(), Locale::En);
    assert!(!plain.contains("moveEntity") && !plain.contains(ROW_DRAG_MIME), "a collection without a reorder rule offers nothing: {plain}");
    let json = render(vec!["members"], Locale::En);
    assert!(json.contains("moveEntity") && json.contains(ROW_DRAG_MIME), "{json}");
    assert!(json.contains("norm-outliner.members[id=b2].move-up") && json.contains("norm-outliner.members[id=b2].move-down"), "a middle row moves both ways: {json}");
    assert!(!json.contains("norm-outliner.members[id=b1].move-up") && json.contains("norm-outliner.members[id=b1].move-down"), "the first row only moves down: {json}");
    assert!(json.contains("norm-outliner.members[id=b3].move-up") && !json.contains("norm-outliner.members[id=b3].move-down"), "the last row only moves up: {json}");
    assert!(json.contains("Move up") && json.contains("Move down"), "{json}");
    let german = render(vec!["members"], Locale::De);
    assert!(german.contains("Nach oben") && german.contains("Nach unten") && !german.contains("Move up"), "{german}");
}

#[test]
fn the_reorderable_collections_are_the_top_level_lists_of_the_move_rules() {
    const RULES: NormEditRules = NormEditRules {
        move_item: &[MoveItemRule { path: "members", kind: "change-member-order", selectors: &[], from: "fromIndex", to: "toIndex" }, MoveItemRule { path: "groups[].items", kind: "change-group-item-order", selectors: &[SelectorKey::Id("groupId")], from: "fromIndex", to: "toIndex" }],
        set_field: &[],
        insert_item: &[],
        remove_item: &[],
    };
    assert_eq!(OutlinerState::default().with_reorderable(&RULES).reorderable, ["members"], "a nested list is not an outliner collection");
    assert!(OutlinerState::default().reorderable.is_empty());
}
