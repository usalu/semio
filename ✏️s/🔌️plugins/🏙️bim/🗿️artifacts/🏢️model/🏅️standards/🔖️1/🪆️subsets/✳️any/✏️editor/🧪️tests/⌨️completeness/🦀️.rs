//! 🧪️ The completeness law of the editor UI: every mutation kind has a way in from the UI (a command, a field row or a tool), every visible text comes from the one `app_labels!` block
//! in both languages, every window and panel has an accessible name, and the gesture in progress shows in the 3D window too.

use super::*;
use semio_framework_plugin::PluginApp as _;
use crate::editor::bim::entities::kind_of;
use crate::editor::bim::unit_tests::context::{bim_app, dispatch, history_verb, render_text, view, BimApp};
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use semio_framework_ui_locale::{LabelText, Locale};

fn code(result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>) -> Option<String> {
    result.err().map(|fault| fault.code.0)
}

fn set(snapshot: &ModelSnapshot, ids: &[&str], field: &str, value: &str) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(&[]);
    run(snapshot, |doc, cfg| set_field::handle(&set_field::SetField { ids: ids.iter().map(|id| id.to_string()).collect(), field: field.into(), value: value.into() }, doc, cfg, &mut ctx))
}

fn with(snapshot: ModelSnapshot, kind: &str, id: &str, parent: &str) -> ModelSnapshot {
    let create = kind_of(kind).and_then(|row| row.create).expect("a creatable kind");
    let mutation = create(&snapshot, id, parent, "Sample").expect("creates");
    crate::mutations::apply_model_mutation(&snapshot, &mutation).expect("applies")
}

fn furnished() -> ModelSnapshot {
    let mut snapshot = demo();
    for (kind, id, parent) in [("slab-type", "sl-1", ""), ("slab", "slab-1", "st-ground"), ("roof-type", "rf-1", ""), ("roof", "roof-1", "st-ground"), ("stair", "stair-1", "st-ground")] {
        snapshot = with(snapshot, kind, id, parent);
    }
    snapshot
}

//#region 🔖️Labels
#[semio_framework_async_macros::async_test]
async fn the_manifest_takes_every_visible_text_from_the_one_label_block_in_both_languages() {
    let json = serde_json::to_string(&create_bim_app()).expect("app definition json");
    let picks: [fn(&BimLabels) -> LabelText; 12] = [
        |labels| labels.cmd_create_entity,
        |labels| labels.cmd_create_entity_describe,
        |labels| labels.cmd_flip_walls,
        |labels| labels.cmd_engagement_submit,
        |labels| labels.utility_slab_walls,
        |labels| labels.utility_split_wall,
        |labels| labels.utility_select,
        |labels| labels.domain_elements,
        |labels| labels.domain_library,
        |labels| labels.arg_fraction,
        |labels| labels.arg_typed_line,
        |labels| labels.arg_container,
    ];
    for pick in picks {
        for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
            assert!(json.contains(pick(labels).as_str()), "the manifest lacks '{}'", pick(labels).as_str());
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_refusal_code_the_new_paths_raise_has_a_notice() {
    let codes: Vec<&str> = bim_fault_notices().iter().map(|(code, _)| *code).collect();
    for code in [
        "bim.tool.input-invalid",
        "bim.split.target-missing",
        "bim.split.fraction-invalid",
        "bim.flip.wall-missing",
        "bim.property.target-missing",
        "bim.property.name-invalid",
        "bim.property.value-invalid",
        "bim.property.missing",
        "bim.place.target-missing",
        "bim.place.point-invalid",
        "bim.place.unsupported",
        "bim.diagnostic.target-missing",
        "bim.classification.target-missing",
        "bim.classification.invalid",
        "bim.classification.missing",
    ] {
        assert!(codes.contains(&code), "no notice for {code}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_default_name_comes_from_the_label_block_in_the_viewers_language_and_never_from_a_literal() {
    use crate::editor::bim::gestures::session::{Surface, ToolContext};
    let snapshot = demo();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &snapshot, Clone::clone);
    let mut context = ToolContext::new(&snapshot, &inference, Surface::Plan { storey: "st-ground".into() }, "seed");
    assert_eq!(context.name_of(|labels| labels.kind_column, 0), "Column 1", "without a viewer the first language's labels name it");
    context.labels = Some(&BimLabels::NATIVE_DE);
    assert_eq!((context.name_of(|labels| labels.kind_column, 1), context.name_of(|labels| labels.kind_stair, 0)), ("Stütze 2".to_string(), "Treppe 1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn a_storey_is_renamed_through_rename_element_only() {
    let snapshot = demo();
    let mut context = ctx(&[]);
    let by_command = run(&snapshot, |doc, cfg| rename_entity::handle(&rename_entity::RenameEntity { id: "st-first".into(), name: "Upper".into() }, doc, cfg, &mut context)).expect("renames");
    let by_field = set(&snapshot, &["st-first"], "name", "Upper").expect("renames");
    for emit in [by_command, by_field] {
        assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::RenameElement(rename)] if rename.id == "st-first" && rename.name == "Upper"), "{:?}", emit.artifact_mutations);
        assert_eq!(applied(&snapshot, &emit).storeys["st-first"].name, "Upper");
    }
}
//#endregion 🔖️Labels

//#region 🔖️Fields
#[semio_framework_async_macros::async_test]
async fn a_slab_outline_and_its_holes_are_edited_as_text_through_set_slab_boundary() {
    let snapshot = furnished();
    let emit = set(&snapshot, &["slab-1"], "boundary", "0, 0; 6, 0; 6, 4 ⌒ 0.5; 0, 4").expect("sets the outline");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetSlabBoundary(_)]));
    let slab = &applied(&snapshot, &emit).slabs["slab-1"];
    assert_eq!((slab.boundary.len(), slab.boundary[2].bulge), (4, 0.5));
    let with_hole = applied(&snapshot, &set(&snapshot, &["slab-1"], "holes", "1, 1; 2, 1; 2, 2").expect("sets a hole"));
    assert_eq!(with_hole.slabs["slab-1"].holes.len(), 1);
    assert_eq!(with_hole.slabs["slab-1"].boundary, snapshot.slabs["slab-1"].boundary, "the hole edit keeps the outline");
    let cleared = applied(&with_hole, &set(&with_hole, &["slab-1"], "holes", "").expect("clears the holes"));
    assert!(cleared.slabs["slab-1"].holes.is_empty());
    assert_eq!(code(set(&snapshot, &["slab-1"], "boundary", "1, 1; 2, 2")), Some("bim.set.value-invalid".to_string()), "two corners are no outline");
}

#[semio_framework_async_macros::async_test]
async fn a_roof_footprint_and_shape_are_edited_as_text() {
    let snapshot = furnished();
    let footprint = applied(&snapshot, &set(&snapshot, &["roof-1"], "footprint", "0, 0; 8, 0; 8, 6; 0, 6").expect("sets the footprint"));
    assert_eq!(footprint.roofs["roof-1"].footprint.len(), 4);
    for (text, expected) in [("flat", crate::RoofShape::Flat), ("shed 0.2 1.57", crate::RoofShape::Shed { pitch: 0.2, direction: 1.57 }), ("gable 0.52 0", crate::RoofShape::Gable { pitch: 0.52, ridge_direction: 0.0 }), ("hip 0.4", crate::RoofShape::Hip { pitch: 0.4 }), ("MANSARD 1.2 0.5 2", crate::RoofShape::Mansard { lower_pitch: 1.2, upper_pitch: 0.5, break_height: 2.0 })] {
        let emit = set(&snapshot, &["roof-1"], "shape", text).unwrap_or_else(|fault| panic!("{text}: {}", fault.message));
        assert_eq!(applied(&snapshot, &emit).roofs["roof-1"].shape, expected, "{text}");
    }
    assert_eq!(code(set(&snapshot, &["roof-1"], "shape", "gable 0.5")), Some("bim.set.value-invalid".to_string()));
    assert_eq!(code(set(&snapshot, &["roof-1"], "shape", "dome 1")), Some("bim.set.value-invalid".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn a_stair_flight_is_edited_as_text() {
    let snapshot = furnished();
    for (text, expected) in [
        ("l-turn 1.5 left", crate::StairFlight::LTurn { split: 1.5, turn: crate::Turn::Left }),
        ("u-turn 0.2", crate::StairFlight::UTurn { gap: 0.2 }),
        ("spiral 1.5 6.28", crate::StairFlight::Spiral { radius: 1.5, sweep: 6.28 }),
    ] {
        let emit = set(&snapshot, &["stair-1"], "flight", text).unwrap_or_else(|fault| panic!("{text}: {}", fault.message));
        assert_eq!(applied(&snapshot, &emit).stairs["stair-1"].flight, expected, "{text}");
    }
    let bent = applied(&snapshot, &set(&snapshot, &["stair-1"], "flight", "u-turn 0.2").expect("bends"));
    let straightened = applied(&bent, &set(&bent, &["stair-1"], "flight", "straight").expect("straightens"));
    assert_eq!(straightened.stairs["stair-1"].flight, crate::StairFlight::Straight);
    assert_eq!(code(set(&snapshot, &["stair-1"], "flight", "l-turn 1.5 up")), Some("bim.set.value-invalid".to_string()));
    assert_eq!(crate::editor::bim::entities::flight_text(&crate::StairFlight::LTurn { split: 1.5, turn: crate::Turn::Right }), "l-turn 1.5 right", "the text reads back as it was written");
}

#[semio_framework_async_macros::async_test]
async fn the_project_record_is_edited_through_the_project_id() {
    let snapshot = demo();
    let renamed = applied(&snapshot, &set(&snapshot, &["project"], "name", "  Villa Rosa ").expect("renames the project"));
    assert_eq!(renamed.project.name, "Villa Rosa");
    let described = applied(&snapshot, &set(&snapshot, &["project"], "phase_names", "Design, Build ,, Handover").expect("sets the phases"));
    assert_eq!(described.project.phase_names, vec!["Design", "Build", "Handover"]);
    let reads: [(&str, fn(&crate::Project) -> String); 3] = [("description", |p| p.description.clone()), ("author", |p| p.author.clone()), ("organization", |p| p.organization.clone())];
    for (field, read) in reads {
        assert_eq!(read(&applied(&snapshot, &set(&snapshot, &["project"], field, "Text").expect(field)).project), "Text", "{field}");
    }
    assert_eq!(code(set(&snapshot, &["project"], "colour", "red")), Some("bim.set.field-unknown".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_classification_is_no_field_row_any_more_it_has_its_commands_and_its_panel_rows() {
    let snapshot = demo();
    assert_eq!(code(set(&snapshot, &["w-south"], "classification", "Uniclass Ss_25")), Some("bim.set.field-unknown".to_string()), "a classification is one code per system: the setClassification and removeClassification commands write it");
}

#[semio_framework_async_macros::async_test]
async fn the_wall_sweep_the_attach_and_the_reveal_are_reachable_from_the_ui_through_set_field_and_the_commands() {
    let snapshot = with(with(furnished(), "wall-sweep", "sw-1", "w-south"), "opening", "op-1", "w-south");
    assert_eq!(snapshot.wall_sweeps["sw-1"].host, "w-south", "createEntity names the wall as the container of a sweep");
    for (field, value) in [("side", "Right"), ("profile", "circle 0.04"), ("height", "0.2"), ("inset", "0.005"), ("name", "Skirting"), ("host", "w-east"), ("material", "m-wool")] {
        let emit = set(&snapshot, &["sw-1"], field, value).unwrap_or_else(|fault| panic!("{field}: {}", fault.message));
        assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetWallSweep(_) | ModelMutation::RenameElement(_)]), "{field}");
        applied(&snapshot, &emit);
    }
    assert_eq!(code(set(&snapshot, &["sw-1"], "side", "Up")), Some("bim.set.value-invalid".to_string()));
    let attached = applied(&snapshot, &set(&snapshot, &["w-south"], "top_attach", "roof-1").expect("attaches the top"));
    assert!(matches!(attached.walls["w-south"].top, crate::TopConstraint::Roof { .. }));
    let based = applied(&attached, &set(&attached, &["w-south"], "base_slab", "slab-1").expect("follows the slab"));
    assert_eq!(based.walls["w-south"].base_slab.as_deref(), Some("slab-1"));
    let freed = applied(&based, &set(&based, &["w-south"], "top_attach", "").expect("frees the top"));
    assert!(matches!(freed.walls["w-south"].top, crate::TopConstraint::StoreyTop { .. }));
    let revealed = applied(&snapshot, &set(&snapshot, &["op-1"], "reveal_depth", "0.1").expect("sets the reveal"));
    assert_eq!(revealed.openings["op-1"].reveal_depth, Some(0.1));
    let json = serde_json::to_string(&create_bim_app()).expect("app definition json");
    let picks: [fn(&BimLabels) -> LabelText; 4] = [|labels| labels.utility_sweep, |labels| labels.cmd_arm_sweep, |labels| labels.cmd_attach_walls, |labels| labels.cmd_attach_walls_describe];
    for pick in picks {
        for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
            assert!(json.contains(pick(labels).as_str()), "the manifest lacks '{}'", pick(labels).as_str());
        }
    }
    let notices: Vec<&str> = bim_fault_notices().iter().map(|(code, _)| *code).collect();
    for code in ["bim.attach.wall-missing", "bim.attach.target-missing", "bim.create.wall-missing", "bim.create.material-missing"] {
        assert!(notices.contains(&code), "no notice for {code}");
    }
}
//#endregion 🔖️Fields

//#region 🔖️Panel
fn panel(snapshot: &ModelSnapshot, elements: &[&str], library: &[&str], locale: Locale) -> String {
    let own = |ids: &[&str]| ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, snapshot, Clone::clone);
    let view = ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native);
    let node = properties_panel::render(snapshot, &inference, &own(elements), &own(library), bim_labels(&view)).expect("the properties panel renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn a_wall_offers_flip_split_place_classification_and_property_rows_in_both_languages() {
    let mut snapshot = demo();
    snapshot.properties.insert("w-south".into(), std::collections::BTreeMap::from([("Pset".to_string(), std::collections::BTreeMap::from([("Load".to_string(), crate::PropertyValue::Real { value: 2.5 })]))]));
    snapshot.classification_systems.insert("cs-uni".into(), crate::ClassificationSystem { name: "Uniclass 2015".into(), edition: String::new(), source: None, entries: vec![crate::ClassificationItem { code: "Ss_25".into(), title: "Walls".into(), parent: None }] });
    let english = panel(&snapshot, &["w-south"], &[], Locale::En);
    for expected in ["Flip wall", "Split wall at", "Place at x, y", "bim-properties.action.split.input", "bim-properties.action.place.input", "Classification", "bim-properties.classification.cs-uni.input", "Property sets", "Pset · Load (real)", "bim-properties.property.Pset.Load.input", "Remove property Pset.Load", "Add property: Set.Property = value", "bim-properties.property.add.input"] {
        assert!(english.contains(expected), "the English wall panel shows '{expected}': {english}");
    }
    let german = panel(&snapshot, &["w-south"], &[], Locale::De);
    for expected in ["Wand wenden", "Wand teilen bei", "Platzieren bei x, y", "Klassifizierung", "Eigenschaftssätze", "Eigenschaft Pset.Load entfernen", "Eigenschaft hinzufügen"] {
        assert!(german.contains(expected), "the German wall panel shows '{expected}': {german}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_rows_follow_the_kind_and_the_selection() {
    let snapshot = demo();
    let storey = panel(&snapshot, &["st-first"], &[], Locale::En);
    assert!(!storey.contains("Flip wall") && !storey.contains("Place at"), "a storey is neither a wall nor placed: {storey}");
    assert!(storey.contains("Property sets"), "but it can be described");
    let two = panel(&snapshot, &["w-south", "w-east"], &[], Locale::En);
    assert!(two.contains("Flip wall") && !two.contains("Property sets"), "several elements flip together, but their property sets are edited one by one: {two}");
    let library = panel(&snapshot, &[], &["wt-300"], Locale::En);
    assert!(library.contains("Property sets") && !library.contains("Flip wall"), "a type carries properties, which its instances inherit, but is no wall: {library}");
    let material = panel(&snapshot, &[], &["m-brick"], Locale::En);
    assert!(!material.contains("Property sets"), "a material carries no properties: {material}");
}

#[semio_framework_async_macros::async_test]
async fn with_nothing_selected_the_panel_edits_the_project() {
    let english = panel(&demo(), &[], &[], Locale::En);
    for expected in ["Project", "Demo House", "bim-properties.name.input", "bim-properties.description.input", "bim-properties.author.input", "bim-properties.organization.input", "bim-properties.phase_names.input", "Phase names"] {
        assert!(english.contains(expected), "the project section shows '{expected}': {english}");
    }
    assert!(panel(&demo(), &[], &[], Locale::De).contains("Phasennamen"));
}
//#endregion 🔖️Panel

//#region 🔖️Accessibility
async fn accessible(app: &mut BimApp, body: &str, view: &ViewModel) -> (Option<String>, Option<String>) {
    use semio_framework_plugin::PluginApp;
    let tree = app.render(body, None, view).await.expect("render");
    let name = tree.root.accessibility.label.as_ref().map(|label| label.0.as_str().to_string());
    let description = tree.root.accessibility.description.as_ref().map(|label| label.0.as_str().to_string());
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire");
    (name, description)
}

#[semio_framework_async_macros::async_test]
async fn every_window_and_panel_body_has_an_accessible_name_in_the_viewers_language() {
    let mut app = bim_app().await;
    for (locale, names) in [(Locale::En, ["Plan", "3D", "Section", "Schedule", "Outliner", "Properties", "Library", "Diagnostics"]), (Locale::De, ["Grundriss", "3D", "Schnitt", "Mengen", "Struktur", "Eigenschaften", "Bibliothek", "Diagnose"])] {
        let view = view(locale, &[], None);
        for (body, name) in [plan::BODY_KEY, world::BODY_KEY, section::BODY_KEY, schedule::BODY_KEY, outliner_panel::BODY_KEY, properties_panel::BODY_KEY, library_panel::BODY_KEY, diagnostics_panel::BODY_KEY].into_iter().zip(names) {
            assert_eq!(accessible(&mut app, body, &view).await.0.as_deref(), Some(name), "{body} in {locale:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn the_drawing_surfaces_say_how_to_drive_them_without_a_pointer() {
    let mut app = bim_app().await;
    let view = view(Locale::En, &[], None);
    for body in [plan::BODY_KEY, world::BODY_KEY, section::BODY_KEY, schedule::BODY_KEY] {
        let description = accessible(&mut app, body, &view).await.1;
        assert!(description.is_some_and(|text| !text.is_empty()), "{body} has a description");
    }
    assert!(accessible(&mut app, plan::BODY_KEY, &view).await.1.is_some_and(|text| text.contains("Enter")), "the plan names the entry field's Enter");
    assert!(accessible(&mut app, outliner_panel::BODY_KEY, &view).await.1.is_none(), "a panel's tree labels itself");
}
//#endregion 🔖️Accessibility

//#region 🔖️Keyboard
#[semio_framework_async_macros::async_test]
async fn no_key_is_bound_twice_across_the_commands_the_utilities_and_the_gesture_keys() {
    let bindings = all_keybindings();
    let mut seen: std::collections::BTreeMap<&str, &str> = std::collections::BTreeMap::new();
    for (keys, action) in &bindings {
        if let Some(other) = seen.insert(keys, action) {
            panic!("the key {keys} is bound to both {other} and {action}");
        }
    }
    assert!(bindings.len() > crate::editor::bim::utilities::UTILITIES.len(), "the commands and the gesture keys are listed too");
}

#[semio_framework_async_macros::async_test]
async fn every_modify_utility_is_armed_by_its_own_key_in_the_plan_and_the_3d_window() {
    use crate::editor::bim::utilities::{UTILITIES, for_window};
    let bindings = all_keybindings();
    let json = serde_json::to_string(&create_bim_app()).expect("app definition json");
    for id in ["copy", "mirror", "array", "array-radial", "offset", "trim", "extend", "align", "split"] {
        let row = UTILITIES.iter().find(|row| row.id == id).unwrap_or_else(|| panic!("no utility {id}"));
        let (keys, arm) = row.keys.zip(row.arm).unwrap_or_else(|| panic!("{id} has no hotkey"));
        assert!(bindings.contains(&(keys, arm)), "{id}: {keys} is registered");
        assert!(json.contains(arm), "{id}: the manifest offers {arm}");
        assert_eq!(row.group, Some("modify"), "{id} sits in the modify ribbon group");
        for window in [plan::WINDOW_KIND_ID, world::WINDOW_KIND_ID] {
            assert!(for_window(window).iter().any(|utility| utility.as_str() == id), "{id} is armed in {window}");
        }
        let command = BimModelApp::command_from_action(arm, None).unwrap_or_else(|error| panic!("{arm}: {}", error.message));
        assert!(BIM_TOOL_IDS.contains(&command.command_id()), "{arm} is a tool command");
        for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
            assert!((row.label)(labels).as_str().len() > 1, "{id} has a label");
        }
    }
    assert_eq!(bindings.iter().find(|(_, action)| *action == "flipWalls"), Some(&("shift+f", "flipWalls")), "the wall flip does not take the key of the offset tool");
}
//#endregion 🔖️Keyboard

//#region 🔖️Mounted
#[semio_framework_async_macros::async_test]
async fn splitting_and_flipping_walls_are_single_undoable_rows_of_the_mounted_app() {
    let mut app = bim_app().await;
    assert!(dispatch(&mut app, BimCommand::SplitWallAt(split_wall::SplitWallAt { ids: vec!["w-south".into()], at: "25%".into() })).await.edited_document());
    assert_eq!(app.snapshot().expect("snapshot").walls.len(), 5);
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("snapshot").walls.len(), 4);
    let before = app.snapshot().expect("snapshot").walls["w-south"].axis.clone();
    assert!(dispatch(&mut app, BimCommand::FlipWalls(flip_walls::FlipWalls { ids: vec!["w-south".into()] })).await.edited_document());
    assert_ne!(app.snapshot().expect("snapshot").walls["w-south"].axis, before, "the wall runs the other way");
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("snapshot").walls["w-south"].axis, before);
}

#[semio_framework_async_macros::async_test]
async fn moving_a_wall_between_storeys_by_menu_key_or_drop_is_one_undoable_row_and_its_openings_follow() {
    let mut app = bim_app().await;
    let host = |app: &BimApp, wall: &str| app.snapshot().expect("snapshot").openings.values().filter(|opening| opening.host == wall).count();
    let hosted = host(&app, "w-south");
    assert!(dispatch(&mut app, BimCommand::StoreyUp(move_storey::StoreyUp { ids: vec!["w-south".into()] })).await.edited_document());
    assert_eq!(app.snapshot().expect("snapshot").walls["w-south"].storey, "st-first");
    assert_eq!(host(&app, "w-south"), hosted, "the openings follow their host");
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("snapshot").walls["w-south"].storey, "st-ground");
    let dropped = BimModelApp::command_from_action("setField", Some(&semio_framework_plugin::DslValue::object([("field", "storey"), ("value", "st-first"), (outliner_panel::DRAG_ELEMENT_MIME, "w-east")].map(|(key, value)| (key.to_string(), semio_framework_plugin::DslValue::String(value.into())))))).expect("the drop decodes");
    assert!(dispatch(&mut app, dropped).await.edited_document());
    assert_eq!(app.snapshot().expect("snapshot").walls["w-east"].storey, "st-first", "the row dropped on a storey stands on it");
    history_verb(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("snapshot").walls["w-east"].storey, "st-ground");
}

#[semio_framework_async_macros::async_test]
async fn properties_classifications_the_project_and_typed_placements_are_undoable_through_the_mounted_app() {
    let mut app = bim_app().await;
    assert!(dispatch(&mut app, BimCommand::SetProperty(set_property::SetProperty { ids: vec!["w-south".into()], pset: String::new(), property: String::new(), value: "Pset.Fire = 90".into() })).await.edited_document());
    assert!(app.snapshot().expect("snapshot").properties.contains_key("w-south"));
    assert!(dispatch(&mut app, BimCommand::RemoveProperty(remove_property::RemoveProperty { ids: vec!["w-south".into()], pset: "Pset".into(), property: "Fire".into() })).await.edited_document());
    assert!(!app.snapshot().expect("snapshot").properties.contains_key("w-south"), "the last property drops the entry");
    history_verb(&mut app, "undo").await;
    assert!(app.snapshot().expect("snapshot").properties.contains_key("w-south"), "the remove's concrete inverse restores it");
    assert!(dispatch(&mut app, BimCommand::SetField(set_field::SetField { ids: vec!["project".into()], field: "name".into(), value: "Villa Rosa".into() })).await.edited_document());
    assert_eq!(app.snapshot().expect("snapshot").project.name, "Villa Rosa");
    assert!(dispatch(&mut app, BimCommand::CreateEntity(create_entity::CreateEntity { kind: "classification-system".into(), parent: String::new(), name: "Uniclass".into() })).await.edited_document());
    let system = app.snapshot().expect("snapshot").classification_systems.keys().next().cloned().expect("the new system");
    assert!(dispatch(&mut app, BimCommand::SetClassification(set_classification::SetClassification { ids: vec!["w-south".into()], system: system.clone(), code: "Ss_25".into() })).await.edited_document());
    assert!(app.snapshot().expect("snapshot").classifications.contains_key("w-south"));
    assert!(dispatch(&mut app, BimCommand::PlaceAt(place_elements::PlaceAt { ids: vec!["w-south".into()], at: "10, 5".into() })).await.edited_document());
    let ends = crate::editor::bim::gestures::plane::axis_ends(&app.snapshot().expect("snapshot").walls["w-south"].axis);
    assert_eq!(ends, ([10.0, 5.0], [18.0, 5.0]));
    history_verb(&mut app, "undo").await;
    assert_eq!(crate::editor::bim::gestures::plane::axis_ends(&app.snapshot().expect("snapshot").walls["w-south"].axis), ([0.0, 0.0], [8.0, 0.0]), "place-elements restores the base placement exactly");
}
//#endregion 🔖️Mounted

//#region 🔖️WorldPreview
#[semio_framework_async_macros::async_test]
async fn the_marks_of_a_gesture_become_segments_and_points_at_the_floor_of_the_storey_in_the_3d_window() {
    use crate::editor::bim::gestures::tests::fixture::{room, Rig};
    let mut rig = Rig::plan("wall", room());
    rig.down(0.0, -2.0);
    rig.mv(3.0, -2.0);
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &rig.snapshot, Clone::clone);
    let config = world::config::BimWorldWindowConfig::default();
    assert!(world::scene(&rig.snapshot, &inference, &config, &[], &[], 1).engagement_preview_json.is_none(), "a scene without a gesture has no preview");
    let scene = world::scene_over(&rig.snapshot, &inference, &config, &[], &[], 1, &rig.preview);
    let items: Vec<serde_json::Value> = serde_json::from_str(&scene.engagement_preview_json.expect("the gesture shows")).expect("a JSON array");
    let segment = items.iter().find(|item| item["kind"] == "segment").expect("the rubber band is a segment");
    let numbers = |value: &serde_json::Value| value.as_array().map(|items| items.iter().filter_map(serde_json::Value::as_f64).collect::<Vec<_>>()).unwrap_or_default();
    let (from, to) = (numbers(&segment["from"]), numbers(&segment["to"]));
    assert!(from.iter().zip([0.0, -2.0, 0.02]).chain(to.iter().zip([3.0, -2.0, 0.02])).all(|(found, expected)| (found - expected).abs() < 1e-9), "{from:?} -> {to:?}");
    assert!(items.iter().any(|item| item["kind"] == "point"), "the snap marker is a point cross");
    let isolated = world::config::BimWorldWindowConfig { isolated_storey: "st-first".into(), ..config };
    let lifted = world::scene_over(&rig.snapshot, &inference, &isolated, &[], &[], 1, &rig.preview);
    let items: Vec<serde_json::Value> = serde_json::from_str(&lifted.engagement_preview_json.expect("the gesture shows")).expect("a JSON array");
    assert!(items.iter().find(|item| item["kind"] == "segment").and_then(|item| item["from"][2].as_f64()).is_some_and(|z| (z - 3.02).abs() < 1e-9), "the marks float over the isolated storey");
}

#[semio_framework_async_macros::async_test]
async fn a_closed_path_gets_its_closing_segment_and_a_label_has_no_3d_form() {
    use crate::editor::bim::gestures::session::{Mark, Preview, Style};
    let snapshot = demo();
    let inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(None, &snapshot, Clone::clone);
    let preview = Preview::of(vec![Mark::path(&[[0.0, 0.0], [2.0, 0.0], [2.0, 2.0]], true, Style::Ghost), Mark::label([1.0, 1.0], "2.00 m")]);
    let items = world::preview_items(&snapshot, &inference, &world::config::BimWorldWindowConfig::default(), &preview);
    assert_eq!(items.len(), 3, "two segments and the closing one; the label stays in the plan");
}
//#endregion 🔖️WorldPreview

//#region 🔖️View
#[semio_framework_async_macros::async_test]
async fn the_projection_of_the_3d_window_must_be_one_of_its_five_kinds_and_the_chrome_offers_exactly_those() {
    let view = view(Locale::En, &[("bim-world", world::WINDOW_KIND_ID)], Some("bim-world"));
    for (value, accepted) in [("orthographic", true), ("threePoint", true), ("twoPoint", true), ("cabinet", false), ("", false)] {
        let mut context = BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None);
        let result = run(&demo(), |doc, cfg| set_view::handle(&set_view::SetView { field: "projection".into(), value: value.into(), pressed: None }, doc, cfg, &mut context));
        assert_eq!(result.is_ok(), accepted, "{value}");
        if !accepted {
            assert_eq!(code(result), Some("bim.view.value-invalid".to_string()));
        }
    }
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let measures = crate::editor::bim::chrome::measures(&demo(), &cfg, &view).remove("bim-world").expect("the world measures");
    let Some(WindowMeasure::Select { items, .. }) = measures.iter().find(|measure| matches!(measure, WindowMeasure::Select { id, .. } if id == "bim.measure.world.projection")) else { panic!("the projection select") };
    assert_eq!(items.iter().map(|item| item.value.as_str()).collect::<Vec<_>>(), world::PROJECTION_KINDS);
}
//#endregion 🔖️View

#[semio_framework_async_macros::async_test]
async fn the_diagnostics_panel_of_the_mounted_app_names_a_clean_model_in_both_languages() {
    let mut app = bim_app().await;
    assert!(render_text(&mut app, diagnostics_panel::BODY_KEY, &view(Locale::En, &[], None)).await.contains("No problems found"));
    assert!(render_text(&mut app, diagnostics_panel::BODY_KEY, &view(Locale::De, &[], None)).await.contains("Keine Probleme gefunden"));
}

#[semio_framework_async_macros::async_test]
async fn activating_a_finding_selects_its_elements_in_the_shared_elements_domain_without_touching_the_document() {
    let mut app = bim_app().await;
    let before = app.document_pack().await.expect("pack");
    let selected = dispatch(&mut app, BimCommand::SelectFindings(select_findings::SelectFindings { ids: vec!["w-south".into(), "w-east".into()] })).await;
    assert!(!selected.edited_document());
    let selection = app.interaction_state().await.selection.get(crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default();
    assert_eq!(selection, vec!["w-south".to_string(), "w-east".to_string()], "the plan, the 3D view and the outliner share this selection");
    let after = app.document_pack().await.expect("pack");
    assert!(before.pack == after.pack && before.spr == after.spr, "selecting a finding writes no document byte");
}

#[semio_framework_async_macros::async_test]
async fn classifying_and_unclassifying_through_the_typed_commands_are_undoable_rows_of_the_mounted_app() {
    let mut app = bim_app().await;
    assert!(dispatch(&mut app, BimCommand::CreateEntity(create_entity::CreateEntity { kind: "classification-system".into(), parent: String::new(), name: "Uniclass".into() })).await.edited_document());
    let system = app.snapshot().expect("snapshot").classification_systems.keys().next().cloned().expect("the new system");
    let classify = BimCommand::SetClassification(set_classification::SetClassification { ids: vec!["w-south".into(), "w-east".into()], system: system.clone(), code: "Ss_25".into() });
    assert!(dispatch(&mut app, classify).await.edited_document());
    assert_eq!(app.snapshot().expect("snapshot").classifications.len(), 2);
    assert!(dispatch(&mut app, BimCommand::RemoveClassification(remove_classification::RemoveClassification { ids: vec!["w-south".into()], system })).await.edited_document());
    assert!(!app.snapshot().expect("snapshot").classifications.contains_key("w-south"));
    history_verb(&mut app, "undo").await;
    assert!(app.snapshot().expect("snapshot").classifications.contains_key("w-south"), "the remove has a concrete inverse");
    history_verb(&mut app, "undo").await;
    assert!(app.snapshot().expect("snapshot").classifications.is_empty(), "the classification of both walls is one row");
    history_verb(&mut app, "undo").await;
    assert!(app.snapshot().expect("snapshot").classification_systems.is_empty(), "the new system is one row");
}
