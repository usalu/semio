//! 🧪️ Entity editing tests: key or gesture, to events, to the resulting document value. The oracle is the framework's own value-path reducer (`insert_value_at_path`), independent of the edit builders.

use super::*;
use crate::accessibility::focus_order::{lookup, shortcuts, Chord, Region, Scope, ShortcutAction};
use crate::direct_manipulation::{DragEffect, DragEvent, DragState, HandleDragTool, Modifiers};
use semio_framework_value::DslValue;
use semio_s_plugin_norm_structural_scene::{Affine, EntityKind, EntityRef, FieldBinding, GripStyle, Handle, HandleKind, Interval, LocalizedCopy, SnapRule, Vec3};

fn row(id: &str, x: f64, link: Option<&str>) -> DslValue {
    let mut entries = vec![("id".to_string(), DslValue::String(id.into())), ("xM".to_string(), DslValue::float(x))];
    if let Some(link) = link {
        entries.push(("to".to_string(), DslValue::String(link.into())));
    }
    DslValue::Object(entries)
}

fn doc() -> DslValue {
    DslValue::Object(vec![("beams".to_string(), DslValue::Array(vec![row("b1", 0.0, None), row("b2", 1.0, Some("b1")), row("b3", 2.0, Some("b2"))]))])
}

fn ids(tree: &DslValue) -> Vec<String> {
    get_value_at_path(tree, "beams").unwrap().as_array().unwrap().iter().map(|r| object_id_field(r).unwrap().to_string()).collect()
}

fn chord(key: &'static str, ctrl: bool) -> Chord {
    Chord { key, ctrl, alt: false, shift: false }
}

#[test]
fn ctrl_d_duplicates_the_selection_after_the_last_original_with_fresh_ids_and_following_references() {
    assert_eq!(lookup(Scope::Viewport, chord("d", true)), Some(ShortcutAction::Duplicate));
    let mut tree = doc();
    let pasted = duplicate_edits(&tree, "beams", &["b2".into(), "b3".into()], &[PasteOffset { field: "xM".into(), delta: 0.5 }]).unwrap();
    assert_eq!(pasted.ids, ["b2-copy", "b3-copy"]);
    apply_to_tree(&mut tree, &pasted.edits).unwrap();
    assert_eq!(ids(&tree), ["b1", "b2", "b3", "b2-copy", "b3-copy"]);
    let rows = get_value_at_path(&tree, "beams").unwrap().as_array().unwrap();
    assert_eq!(rows[4].get("to"), Some(&DslValue::String("b2-copy".into())));
    assert_eq!(rows[3].get("to"), Some(&DslValue::String("b1".into())));
    assert_eq!(rows[3].get("xM").and_then(DslValue::as_f64), Some(1.5));
}

#[test]
fn copy_then_paste_twice_never_reuses_an_id_and_a_copy_of_a_copy_keeps_its_stem() {
    assert_eq!(lookup(Scope::Handle, chord("c", true)), Some(ShortcutAction::Copy));
    assert_eq!(lookup(Scope::Handle, chord("v", true)), Some(ShortcutAction::Paste));
    let mut tree = doc();
    let clip = copy_rows(&tree, "beams", &["b1".into()]).unwrap();
    for _ in 0..2 {
        let pasted = paste_edits(&tree, &clip, Some(0), &[]).unwrap();
        apply_to_tree(&mut tree, &pasted.edits).unwrap();
    }
    assert_eq!(ids(&tree), ["b1-copy-2", "b1-copy", "b1", "b2", "b3"]);
    let again = copy_rows(&tree, "beams", &["b1-copy".into()]).unwrap();
    assert_eq!(paste_edits(&tree, &again, None, &[]).unwrap().ids, ["b1-copy-3"]);
    assert!(copy_rows(&tree, "beams", &["nope".into()]).is_err());
}

#[test]
fn a_paste_is_one_emit_so_it_is_one_undo_step() {
    let tree = doc();
    let pasted = duplicate_edits(&tree, "beams", &["b1".into(), "b2".into()], &[]).unwrap();
    let emit = handle_edits(&tree, &pasted.edits, |_, edit| Ok(vec![format!("{edit:?}")])).unwrap();
    assert_eq!(emit.artifact_mutations.len(), 2);
    assert!(resolve_edits(&tree, &pasted.edits, |_, _| Err::<Vec<String>, _>("no rule".into())).is_err());
}

#[test]
fn a_bulk_edit_writes_every_selected_row_and_the_properties_view_shows_mixed_until_they_agree() {
    let mut tree = doc();
    let ids_ = ["b1".to_string(), "b3".to_string()];
    assert_eq!(shared_value(&tree, "beams", &ids_, "xM"), SharedValue::Mixed);
    let writes = bulk_writes(&tree, "beams", &ids_, "xM", &DslValue::float(4.0)).unwrap();
    assert_eq!(writes.iter().map(|w| w.path.as_str()).collect::<Vec<_>>(), ["beams[id=b1].xM", "beams[id=b3].xM"]);
    for write in &writes {
        set_value_at_path(&mut tree, &write.path, write.value.clone()).unwrap();
    }
    assert_eq!(shared_value(&tree, "beams", &ids_, "xM"), SharedValue::Same(DslValue::float(4.0)));
    assert_eq!(shared_value(&tree, "beams", &ids_, "yM"), SharedValue::Absent);
    let moved = bulk_delta_writes(&tree, "beams", &["b1".into(), "b2".into()], "xM", 0.5).unwrap();
    assert_eq!(moved.iter().map(|w| w.value.as_f64().unwrap()).collect::<Vec<_>>(), [4.5, 1.5]);
    assert!(bulk_writes(&tree, "beams", &["ghost".into()], "xM", &DslValue::Null).is_err());
    assert_eq!((mixed_label(false), mixed_label(true)), ("Mixed", "Gemischt"));
}

#[test]
fn select_all_and_history_chords_are_documented_in_both_languages() {
    assert_eq!(lookup(Scope::Viewport, chord("a", true)), Some(ShortcutAction::SelectAll));
    assert_eq!(history_step(&lookup(Scope::Handle, chord("z", true)).unwrap()), Some(HistoryStep::Undo));
    assert_eq!(history_step(&ShortcutAction::Redo), Some(HistoryStep::Redo));
    assert_eq!(history_step(&ShortcutAction::Copy), None);
    for action in [ShortcutAction::Copy, ShortcutAction::Paste, ShortcutAction::Duplicate, ShortcutAction::SelectAll] {
        let shortcut = shortcuts().into_iter().find(|s| s.action == action).expect("documented");
        assert!(!shortcut.label.en.is_empty() && !shortcut.label.de.is_empty() && shortcut.label.en != shortcut.label.de, "{action:?}");
    }
}

fn handle(kind_axis: Vec3, writes_unit: &str, k: f64) -> Handle {
    Handle {
        id: "b1/x".into(),
        entity: EntityRef::new(EntityKind::Member, "b1", "beams[id=b1]"),
        kind: HandleKind::Translate { axis: kind_axis },
        anchor: Vec3::new(1.0, 0.0, 0.0),
        grip: GripStyle::Arrow,
        writes: vec![FieldBinding::new("beams[id=b1].xM", Affine { base: 0.0, k }, writes_unit)],
        snap: SnapRule { grid: None, increment: None, targets: vec![], catalogue: vec![], tolerance: 0.1 },
        limits: Some(Interval::new(-5.0, 5.0)),
        accessible_name: LocalizedCopy::new("Position", "Lage"),
    }
}

#[test]
fn a_drag_near_a_member_end_or_midpoint_lands_exactly_on_it() {
    let points = [Vec3::new(3.0, 2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(40.0, 0.0, 0.0), Vec3::new(3.0, 5.0, 5.0)];
    let mut h = handle(Vec3::new(1.0, 0.0, 0.0), "m", 1.0);
    assert_eq!(targets_from_points(&points, &h), [2.0, 3.0]);
    h.snap.targets = targets_from_points(&points, &h);
    let tool = HandleDragTool { handles: std::slice::from_ref(&h), seq: 1 };
    let (state, _) = tool.step(DragState::Idle, DragEvent::Press { handle: h.id.clone(), s: 0.0 });
    let (_, effect) = tool.step(state, DragEvent::Move { s: 2.06, modifiers: Modifiers::NONE });
    let DragEffect::Preview(preview) = effect else { panic!("preview") };
    assert_eq!(preview.writes[0].value.as_f64(), Some(2.0));
    let (state, _) = tool.step(DragState::Idle, DragEvent::Press { handle: h.id.clone(), s: 0.0 });
    let (_, free) = tool.step(state, DragEvent::Move { s: 2.06, modifiers: Modifiers::CTRL });
    let DragEffect::Preview(free) = free else { panic!("preview") };
    assert_eq!(free.writes[0].value.as_f64(), Some(2.06));
    assert!(targets_from_points(&points, &handle_radial()).is_empty());
}

fn handle_radial() -> Handle {
    Handle { kind: HandleKind::Radial { centre: Vec3::ZERO }, ..handle(Vec3::X, "m", 1.0) }
}

#[test]
fn typed_values_carry_their_unit_and_land_in_the_unit_of_the_field() {
    assert!((parse_quantity_entry("35 cm", "m").unwrap() - 0.35).abs() < 1e-12);
    assert_eq!(parse_quantity_entry("1,5m", "mm"), Some(1500.0));
    assert_eq!(parse_quantity_entry("12", "kN"), Some(12.0));
    assert_eq!(parse_quantity_entry("2 kN", "N"), Some(2000.0));
    assert_eq!(parse_quantity_entry("2 kN", "m"), None);
    assert_eq!(parse_quantity_entry("2 furlong", "m"), None);
    assert_eq!(parse_quantity_entry("abc", "m"), None);
    let h = handle(Vec3::X, "m", 2.0);
    assert_eq!(entry_to_s(&h, "100 cm"), Some(0.5));
    assert_eq!(entry_to_s(&h, "100 m"), Some(5.0));
}

#[test]
fn the_host_routes_history_view_entity_and_selection_chords() {
    let id = |action: ShortcutAction| crate::accessibility::focus_order::shortcut_action_id(&action);
    assert_eq!(shortcut_route(&id(ShortcutAction::Undo)), HostRoute::History(HistoryStep::Undo));
    assert_eq!(shortcut_route(&id(ShortcutAction::Redo)), HostRoute::History(HistoryStep::Redo));
    assert_eq!(shortcut_route(&id(ShortcutAction::Copy)), HostRoute::Entity("copyEntities"));
    assert_eq!(shortcut_route(&id(ShortcutAction::Paste)), HostRoute::Entity("pasteEntities"));
    assert_eq!(shortcut_route(&id(ShortcutAction::Duplicate)), HostRoute::Entity("duplicateEntities"));
    assert_eq!(shortcut_route(&id(ShortcutAction::Delete)), HostRoute::Entity("deleteEntities"));
    assert_eq!(shortcut_route(&id(ShortcutAction::SelectAll)), HostRoute::SelectAll);
    for region in [Region::Outliner, Region::Viewport, Region::Properties, Region::Checks, Region::Diagrams] {
        assert_eq!(shortcut_route(&id(ShortcutAction::Focus(region))), HostRoute::FocusWindow(region), "{region:?} is the framework focusWindow verb, not an app action");
        assert!(host_entity_call(&id(ShortcutAction::Focus(region)), &HostContext::default()).is_none());
    }
    assert_eq!(shortcut_route("norm.shortcut.next-check"), HostRoute::ViewAct(crate::VIEW_ACTION_STEP_CHECK));
    assert_eq!(shortcut_route("norm.shortcut.unknown"), HostRoute::Local);
    for tool in ["copyEntities", "duplicateEntities", "pasteEntities", "deleteEntities"] {
        assert!(crate::NORM_EDITOR_TOOL_IDS.contains(&tool), "{tool} is an editor tool");
    }
}

#[test]
fn a_clip_survives_the_clipboard_text_and_pastes_like_the_original_clip() {
    let tree = doc();
    let text = copy_clip_json(&tree, "beams", &["b2".into()]).unwrap();
    let clip = clip_from_json(&text).unwrap();
    assert_eq!(clip, copy_rows(&tree, "beams", &["b2".into()]).unwrap());
    assert!(clip_from_json("{\"collection\":\"beams\",\"rows\":[]}").is_err());
    assert!(clip_from_json("not json").is_err());
}

#[test]
fn the_touch_long_press_menu_enables_entries_by_selection_and_clip_and_routes_like_the_chords() {
    let empty = touch_menu(0, false, None);
    assert_eq!(empty.iter().filter(|entry| entry.enabled).map(|entry| entry.label.en.as_str()).collect::<Vec<_>>(), ["Select all"]);
    let full = touch_menu(2, true, Some(0));
    assert!(full.iter().all(|entry| entry.enabled && !entry.label.de.is_empty() && entry.label.de != entry.label.en));
    assert_eq!(full.iter().map(|entry| shortcut_route(&entry.action_id)).collect::<Vec<_>>(), [HostRoute::Local, HostRoute::Entity("copyEntities"), HostRoute::Entity("pasteEntities"), HostRoute::Entity("duplicateEntities"), HostRoute::Entity("deleteEntities"), HostRoute::SelectAll]);
    assert!(touch_menu(1, false, None)[2].enabled == false);
}

#[test]
fn the_properties_lines_of_a_multi_selection_show_mixed_in_the_language_of_the_user() {
    let tree = doc();
    let both = ["b1".to_string(), "b2".to_string()];
    let rows = bulk_rows(&tree, "beams", &both, false);
    assert_eq!(rows.iter().find(|row| row.field == "xM").map(|row| (row.text.as_str(), row.mixed)), Some(("Mixed", true)));
    assert_eq!(bulk_rows(&tree, "beams", &both, true).iter().find(|row| row.field == "xM").map(|row| row.text.clone()), Some("Gemischt".to_string()));
    let one = bulk_rows(&tree, "beams", &["b2".to_string()], false);
    assert_eq!(one.iter().find(|row| row.field == "to").map(|row| (row.text.as_str(), row.mixed)), Some(("b1", false)));
    assert!(one.iter().all(|row| row.field != "id"));
    assert!(bulk_rows(&tree, "nope", &both, false).is_empty());
}

fn press(hooks: &crate::NormShellHooks, scope: &str, key: &str, ctrl: bool, shift: bool) -> semio_framework::ShellKeyOutcome {
    use semio_framework::PluginShellHooks;
    hooks.handle_key(&semio_framework::ShellKeyEvent { scope: scope.to_owned(), key: key.to_owned(), ctrl, alt: false, shift, meta: false })
}

fn action_of(outcome: semio_framework::ShellKeyOutcome) -> String {
    match outcome {
        semio_framework::ShellKeyOutcome::Action { action } => action,
        other => panic!("an app action was expected, got {other:?}"),
    }
}

fn tool_call(action: &str, context: &HostContext) -> (LiveCommand, DslValue) {
    let (tool, args) = host_entity_call(action, context).expect("an entity route").expect("something to act on");
    (LiveCommand::parse(tool, Some(&args)).unwrap_or_else(|error| panic!("{tool}: {error}")), args)
}

#[test]
fn the_key_handler_turns_copy_paste_and_duplicate_chords_into_entity_commands_over_the_selection() {
    let hooks = crate::NormShellHooks;
    let mut tree = doc();
    let (collection, picked) = selected_rows(&tree, &["b2".to_string(), "beams[id=b3]".to_string()]).expect("both rows are in beams");
    assert_eq!((collection.as_str(), picked.as_slice()), ("beams", ["b2".to_string(), "b3".to_string()].as_slice()));
    let mut context = HostContext { collection, ids: picked, clip_json: None };
    let (copy, _) = tool_call(&action_of(press(&hooks, "norm-viewport", "c", true, false)), &context);
    let LiveCommand::CopyEntities { collection, ids: copied } = copy else { panic!("a copy") };
    context.clip_json = Some(copy_clip_json(&tree, &collection, &copied).unwrap());
    let (paste, _) = tool_call(&action_of(press(&hooks, "norm-viewport", "v", true, false)), &context);
    let LiveCommand::PasteEntities { clip_json, at } = paste else { panic!("a paste") };
    let pasted = paste_edits(&tree, &clip_from_json(&clip_json).unwrap(), at, &[]).unwrap();
    apply_to_tree(&mut tree, &pasted.edits).unwrap();
    assert_eq!(ids(&tree), ["b1", "b2", "b3", "b2-copy", "b3-copy"], "paste appends fresh rows");
    let (duplicate, _) = tool_call(&action_of(press(&hooks, "norm-viewport", "d", true, false)), &context);
    let LiveCommand::DuplicateEntities { collection, ids: picked } = duplicate else { panic!("a duplicate") };
    let again = duplicate_edits(&tree, &collection, &picked, &[]).unwrap();
    assert_eq!(again.ids, ["b2-copy-2", "b3-copy-2"], "the copies of a copy keep the stem and stay unique");
}

#[test]
fn an_entity_chord_with_nothing_selected_or_copied_is_refused_with_a_reason() {
    let hooks = crate::NormShellHooks;
    let empty = HostContext::default();
    let copy = host_entity_call(&action_of(press(&hooks, "norm-viewport", "c", true, false)), &empty).expect("an entity route");
    assert_eq!(copy, Err("nothing is selected".to_string()));
    let paste = host_entity_call(&action_of(press(&hooks, "norm-viewport", "v", true, false)), &empty).expect("an entity route");
    assert_eq!(paste, Err("nothing was copied".to_string()));
    assert!(host_entity_call("norm.shortcut.next-check", &empty).is_none(), "a view step is no entity route");
}

#[test]
fn undo_and_redo_chords_are_left_to_the_store_history_and_select_all_is_not_answered() {
    let hooks = crate::NormShellHooks;
    for (key, shift) in [("z", false), ("z", true), ("y", false)] {
        assert_eq!(press(&hooks, "norm-global", key, true, shift), semio_framework::ShellKeyOutcome::Ignored, "{key} {shift}");
    }
    assert_eq!(shortcut_route(&crate::accessibility::focus_order::shortcut_action_id(&ShortcutAction::Undo)), HostRoute::History(HistoryStep::Undo));
    assert_eq!(press(&hooks, "norm-viewport", "a", true, false), semio_framework::ShellKeyOutcome::Ignored);
}

#[test]
fn a_bulk_set_writes_the_field_of_every_selected_row_as_one_bundle_and_matches_the_pure_bulk_writes() {
    let mut tree = doc();
    let selected = ["b1".to_string(), "b3".to_string()];
    let args = DslValue::object(vec![("collection".to_string(), DslValue::String("beams".into())), ("ids".to_string(), DslValue::String(selected.join("\n"))), ("field".to_string(), DslValue::String("xM".into())), ("value".to_string(), DslValue::float(7.0))]);
    let Ok(LiveCommand::SetFields { writes }) = LiveCommand::parse("setFields", Some(&args)) else { panic!("a bulk set") };
    assert_eq!(writes, bulk_writes(&tree, "beams", &selected, "xM", &DslValue::float(7.0)).unwrap(), "the command bridge and the pure bulk edit agree");
    for write in &writes {
        set_value_at_path(&mut tree, &write.path, write.value.clone()).unwrap();
    }
    let value_of = |id: &str| get_value_at_path(&tree, &format!("beams[id={id}].xM")).unwrap().as_f64();
    assert_eq!((value_of("b1"), value_of("b2"), value_of("b3")), (Some(7.0), Some(1.0), Some(7.0)));
    assert_eq!(shared_value(&tree, "beams", &selected, "xM"), SharedValue::Same(DslValue::float(7.0)));
    let rows = bulk_rows(&tree, "beams", &["b1".to_string(), "b2".to_string()], true);
    assert_eq!(rows.iter().find(|row| row.field == "xM").map(|row| (row.text.as_str(), row.mixed)), Some(("Gemischt", true)));
}

#[test]
fn moving_a_row_by_step_position_or_drop_is_one_move_item_edit_and_the_tree_follows() {
    let tree = doc();
    let target = |args: Vec<(&str, DslValue)>| {
        let base = vec![("path".to_string(), DslValue::String("beams".into())), ("id".to_string(), DslValue::String("b3".into()))];
        let args = DslValue::object(base.into_iter().chain(args.into_iter().map(|(key, value)| (key.to_string(), value))).collect::<Vec<_>>());
        let Ok(LiveCommand::MoveEntity { path, id, target }) = LiveCommand::parse("moveEntity", Some(&args)) else { panic!("a move") };
        let edit = crate::move_edit(&tree, &path, &id, &target).unwrap();
        let mut moved = tree.clone();
        apply_to_tree(&mut moved, std::slice::from_ref(&edit)).unwrap();
        (edit, ids(&moved))
    };
    let (edit, order) = target(vec![("step", DslValue::int(-1))]);
    assert_eq!((edit, order), (NormEdit::MoveItem { path: "beams".into(), from: 2, to: 1 }, vec!["b1".to_string(), "b3".into(), "b2".into()]));
    let (edit, order) = target(vec![("over", DslValue::String("b1".into()))]);
    assert_eq!((edit, order), (NormEdit::MoveItem { path: "beams".into(), from: 2, to: 0 }, vec!["b3".to_string(), "b1".into(), "b2".into()]));
    let (_, order) = target(vec![("to", DslValue::uint(0))]);
    assert_eq!(order, ["b3", "b1", "b2"]);
    assert!(crate::move_item_step_edit(&tree, "beams", "b3", 1).is_err(), "the last row cannot move down");
    assert!(crate::move_item_step_edit(&tree, "beams", "b1", -1).is_err(), "the first row cannot move up");
    assert!(crate::move_item_over_edit(&tree, "beams", "b2", "b2").is_err(), "dropping a row on itself changes nothing");
    assert!(crate::move_item_over_edit(&tree, "beams", "nope", "b2").is_err());
}

#[test]
fn the_touch_menu_renders_actions_for_enabled_entries_and_names_the_disabled_ones_in_both_languages() {
    use crate::viewport3d::config::Lang;
    let entries = touch_menu(0, false, None);
    for (locale, copy, none) in [(Locale::En, "Copy", "Not available"), (Locale::De, "Kopieren", "Nicht verfügbar")] {
        let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_touch_menu(&entries, "norm.mock", locale).expect("menu"))).expect("projection");
        assert!(json.contains(copy) && json.contains(none), "{json}");
        let projected: serde_json::Value = serde_json::from_str(&json).expect("projected menu");
        let rows = projected["children"].as_array().expect("menu entries");
        let entry = |id: &str| rows.iter().find(|row| row["key"].as_str().and_then(|key| key.strip_prefix("norm-touch-menu.")) == Some(id)).expect("menu entry");
        assert_eq!(entry("norm.shortcut.select-all")["component"]["target"]["activation"], "selectAll");
        assert!(entry("norm.shortcut.copy")["component"].get("target").is_none(), "a disabled entry raises nothing: {json}");
        assert!(entry("norm.shortcut.copy")["bindings"].as_array().expect("bindings").is_empty());
    }
    let nodes = touch_menu_nodes(&touch_menu(2, true, None), Lang::De);
    assert_eq!((nodes[0].node.role, nodes[0].node.name.as_str()), ("menu", "Bearbeiten"));
    assert_eq!(nodes.iter().skip(1).map(|node| (node.node.role, node.node.position)).collect::<Vec<_>>(), (1..=6).map(|at| ("menuitem", Some((at, 6)))).collect::<Vec<_>>());
    let disabled = touch_menu_nodes(&entries, Lang::En);
    assert_eq!(disabled[1].node.description.as_deref(), Some("Not available"));
    assert!(disabled.iter().all(|node| node.node.attributes().iter().all(|(key, value)| (*key == "role" || key.starts_with("aria-")) && !value.is_empty())));
    assert_eq!(crate::accessibility::published_nodes::window_scope("norm-en1993-touch-menu"), Some(("menu", "norm.en1993".to_string())));
}

mod select_all {
    use super::*;
    use semio_framework::{PluginShellHooks, ShellKeyEvent};

    fn press(scope: &str) -> semio_framework::ShellKeyOutcome {
        crate::NormShellHooks.handle_key(&ShellKeyEvent { scope: scope.to_owned(), key: "a".to_owned(), ctrl: true, ..ShellKeyEvent::default() })
    }

    #[test]
    fn the_touch_menu_select_all_raises_the_framework_verb_with_the_norm_scope_for_any_scene_size() {
        use crate::viewport3d::config::Lang;
        let entries = touch_menu(0, false, None);
        let tree = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render_touch_menu(&entries, "norm.mock", Locale::En).expect("menu"))).expect("projection");
        assert!(tree.contains("selectAll") && tree.contains(&select_all_granularities()) && !tree.contains("interactionSelect"), "{tree}");
        assert!(tree.len() < 4_096, "the menu never carries targets, so it does not grow with the scene");
        assert_eq!(touch_menu_nodes(&entries, Lang::En).len(), entries.len() + 1);
    }

    #[test]
    fn the_select_all_scope_names_every_entity_kind_and_neither_checks_nor_load_cases() {
        let scope = select_all_granularities();
        let kinds: Vec<&str> = scope.split(',').collect();
        assert_eq!(kinds.len(), 9);
        assert!(kinds.iter().all(|kind| *kind != crate::viewport3d::picking::GRANULARITY_CHECK && *kind != crate::viewport3d::picking::GRANULARITY_LOAD_CASE));
        for kind in [EntityKind::Member, EntityKind::Plate, EntityKind::Shell, EntityKind::Solid, EntityKind::Fastener, EntityKind::Support, EntityKind::Load, EntityKind::Surface, EntityKind::Group] {
            assert!(kinds.contains(&crate::viewport3d::picking::granularity(kind)), "{kind:?}");
        }
    }

    #[test]
    fn ctrl_a_raises_the_framework_select_all_verb_with_the_norm_scope() {
        let outcome = press("norm-en9101-viewport");
        assert_eq!(outcome, semio_framework::ShellKeyOutcome::verb("selectAll", select_all_args()), "{outcome:?}");
        assert!(matches!(&outcome, semio_framework::ShellKeyOutcome::Verb { action, args } if action == semio_framework::SELECT_ALL_ACTION_ID && args.get("granularities").is_some_and(|scope| scope.starts_with("member,"))));
    }

    #[test]
    fn the_outliner_filter_is_published_for_the_topology_and_clearing_it_widens_again() {
        let tree = DslValue::Object(vec![("beams".to_string(), DslValue::Array(vec![row("b1", 0.0, None), row("b2", 1.0, None)]))]);
        publish_visible_rows("norm.en9102", &tree, "b2");
        assert_eq!(visible_rows("norm.en9102"), Some(vec!["b2".to_owned()]));
        publish_visible_rows("norm.en9102", &tree, "");
        assert_eq!(visible_rows("norm.en9102"), None);
    }

    #[test]
    fn a_window_without_domain_answers_no_select_all() {
        assert_eq!(press("norm-viewport"), semio_framework::ShellKeyOutcome::Ignored);
    }
}

fn pose(id: &str, x: f64, y: f64) -> DslValue {
    DslValue::Object(vec![("id".to_string(), DslValue::String(id.into())), ("x".to_string(), DslValue::float(x)), ("y".to_string(), DslValue::float(y)), ("z".to_string(), DslValue::float(0.0)), ("yaw".to_string(), DslValue::float(0.3))])
}

fn with_poses(mut tree: DslValue, poses: Vec<DslValue>) -> DslValue {
    if let DslValue::Object(entries) = &mut tree {
        entries.push(("placements".to_string(), DslValue::Array(poses)));
    }
    tree
}

fn pose_of(tree: &DslValue, id: &str) -> Option<(f64, f64, f64)> {
    let row = get_value_at_path(tree, "placements").ok()?.as_array()?.iter().find(|row| object_id_field(row) == Some(id))?.clone();
    Some((row.get("x")?.as_f64()?, row.get("y")?.as_f64()?, row.get("yaw")?.as_f64()?))
}

#[test]
fn a_duplicate_offsets_the_copy_and_its_placement_row_and_one_paste_into_another_document_keeps_the_pose() {
    let mut tree = with_poses(doc(), vec![pose("b2", 1.0, 2.0)]);
    let pasted = duplicate_edits(&tree, "beams", &["b2".into()], &paste_offsets()).unwrap();
    assert_eq!(pasted.edits.len(), 2, "the row and its pose are the only edits of the one batch");
    apply_to_tree(&mut tree, &pasted.edits).unwrap();
    assert_eq!(pose_of(&tree, "b2"), Some((1.0, 2.0, 0.3)), "the original stays");
    assert_eq!(pose_of(&tree, "b2-copy"), Some((1.0 + PASTE_STEP, 2.0 + PASTE_STEP, 0.3)));
    let clip = clip_from_json(&copy_clip_json(&tree, "beams", &["b2".into()]).unwrap()).unwrap();
    assert_eq!(clip.poses.len(), 1);
    let mut other = with_poses(DslValue::Object(vec![("beams".to_string(), DslValue::Array(vec![row("b2", 9.0, None)]))]), vec![pose("b2", 7.0, 7.0)]);
    let there = paste_edits(&other, &clip, None, &paste_offsets()).unwrap();
    apply_to_tree(&mut other, &there.edits).unwrap();
    assert_eq!(there.ids, ["b2-copy"]);
    assert_eq!(pose_of(&other, "b2-copy"), Some((1.0 + PASTE_STEP, 2.0 + PASTE_STEP, 0.3)));
    assert_eq!(pose_of(&other, "b2"), Some((7.0, 7.0, 0.3)), "the other document's own pose is untouched");
    let bare = doc();
    assert_eq!(paste_edits(&bare, &clip, None, &[]).unwrap().edits.len(), 1, "a document without placement rows takes no pose");
}

#[test]
fn a_selection_that_ties_between_an_entity_collection_and_its_placement_rows_acts_on_the_entities() {
    let tree = with_poses(doc(), vec![pose("b2", 1.0, 2.0)]);
    assert_eq!(selected_rows(&tree, &["b2".to_string()]), Some(("beams".to_string(), vec!["b2".to_string()])));
}

mod context_menu {
    use super::*;
    use semio_framework_plugin::ContextMenuItemSpec;

    fn hit(domain: &str, id: &str) -> MenuProbe {
        MenuProbe { hits: vec![(domain.to_string(), id.to_string())], ..MenuProbe::default() }
    }

    fn items(probe: &MenuProbe, context: &HostContext, german: bool) -> Vec<ContextMenuItemSpec> {
        context_menu_items(&doc(), probe, context, &|id| (id == "b2").then_some(4), german)
    }

    fn named<'a>(items: &'a [ContextMenuItemSpec], id: &str) -> &'a ContextMenuItemSpec {
        items.iter().find(|item| item.id == format!("norm.context.{id}")).unwrap_or_else(|| panic!("{id} in {:?}", items.iter().map(|item| &item.id).collect::<Vec<_>>()))
    }

    fn tool_of(item: &ContextMenuItemSpec) -> LiveCommand {
        LiveCommand::parse(item.action.as_deref().expect("an action"), item.args.as_ref()).unwrap_or_else(|error| panic!("{}: {error}", item.id))
    }

    #[test]
    fn a_secondary_click_on_an_outliner_row_acts_on_that_entity_with_the_commands_of_the_chords() {
        let clip = copy_clip_json(&doc(), "beams", &["b1".into()]).unwrap();
        let context = HostContext { clip_json: Some(clip), ..HostContext::default() };
        let menu = items(&hit(TREE_HIT_DOMAIN, "norm-outliner.beams[id=b2].select"), &context, false);
        assert_eq!(tool_of(named(&menu, "duplicate")), LiveCommand::DuplicateEntities { collection: "beams".into(), ids: vec!["b2".into()] });
        assert_eq!(tool_of(named(&menu, "copy")), LiveCommand::CopyEntities { collection: "beams".into(), ids: vec!["b2".into()] });
        assert!(matches!(tool_of(named(&menu, "paste")), LiveCommand::PasteEntities { .. }));
        let delete = named(&menu, "delete");
        assert_eq!(delete.destructive, Some(true));
        assert_eq!(tool_of(delete), LiveCommand::DeleteEntities { collection: "beams".into(), ids: vec!["b2".into()] });
        assert_eq!(named(&menu, "focus-check").args.as_ref().and_then(|args| args.get("index")).and_then(DslValue::as_u64), Some(4));
        let select_all = named(&menu, "select-all");
        assert_eq!(select_all.action.as_deref(), Some(semio_framework::SELECT_ALL_ACTION_ID));
        assert!(select_all.args.as_ref().and_then(|args| args.get("granularities")).and_then(DslValue::as_str).is_some_and(|granularities| granularities.contains("member")));
    }

    #[test]
    fn a_3d_pick_by_plain_or_scoped_id_resolves_to_the_same_rows() {
        let scoped = crate::viewport3d::picking::entity_address(EntityKind::Member, "b3").expect("an address");
        for id in ["b3", scoped.as_str()] {
            let menu = items(&hit("member", id), &HostContext::default(), false);
            assert_eq!(tool_of(named(&menu, "copy")), LiveCommand::CopyEntities { collection: "beams".into(), ids: vec!["b3".into()] }, "{id}");
        }
    }

    #[test]
    fn a_secondary_click_inside_the_selection_keeps_the_selection_and_outside_it_acts_on_the_hit_row_alone() {
        let context = HostContext { collection: "beams".into(), ids: vec!["b1".into(), "b2".into()], clip_json: None };
        let inside_probe = MenuProbe { window: "norm-en1993-outliner".to_owned(), ..hit(TREE_HIT_DOMAIN, "norm-outliner.beams[id=b2].select") };
        let inside = items(&inside_probe, &context, false);
        assert_eq!(tool_of(named(&inside, "duplicate")), LiveCommand::DuplicateEntities { collection: "beams".into(), ids: vec!["b1".into(), "b2".into()] });
        assert_eq!(tool_of(named(&inside, "delete")), LiveCommand::DeleteEntities { collection: "beams".into(), ids: vec!["b1".into(), "b2".into()] }, "delete takes the whole selection");
        let bulk = named(&inside, "bulk-edit");
        assert!(bulk.label.as_deref().is_some_and(|label| label.contains('2')) && bulk.disabled.is_none());
        assert_eq!(bulk.action.as_deref(), Some(semio_framework::FOCUS_WINDOW_ACTION_ID), "the row raises the framework focusWindow verb");
        assert_eq!(bulk.args.as_ref().and_then(|args| args.get(semio_framework::FOCUS_WINDOW_ARG_KIND)).and_then(DslValue::as_str), Some("norm-en1993-properties"), "for the Properties window kind of the open artifact");
        let outside = items(&hit("member", "b3"), &context, false);
        assert_eq!(tool_of(named(&outside, "duplicate")), LiveCommand::DuplicateEntities { collection: "beams".into(), ids: vec!["b3".into()] });
        assert!(outside.iter().all(|item| item.id != "norm.context.bulk-edit"));
    }

    #[test]
    fn a_checks_row_offers_to_focus_its_check_and_its_detail_rows_name_the_same_check() {
        for id in ["norm-checks-check-3", "norm-checks-check-3.status"] {
            let menu = items(&hit(TREE_HIT_DOMAIN, id), &HostContext::default(), false);
            let focus = named(&menu, "focus-check");
            assert_eq!((focus.action.as_deref(), focus.args.as_ref().and_then(|args| args.get("index")).and_then(DslValue::as_u64)), (Some("setSelectedCheckIndex"), Some(3)), "{id}");
        }
        assert_eq!(check_row_index("norm-outliner.beams[id=b2].select"), None);
    }

    #[test]
    fn nothing_hit_and_nothing_selected_keeps_every_row_in_place_with_its_reason_and_select_all_runs() {
        assert!(items(&MenuProbe::default(), &HostContext::default(), false).is_empty(), "outside the viewport the shell's window actions apply");
        let menu = items(&MenuProbe { window: "norm-en1992-viewport".into(), ..MenuProbe::default() }, &HostContext::default(), false);
        for id in ["duplicate", "copy", "paste", "delete"] {
            let row = named(&menu, id);
            assert!(row.disabled == Some(true) && row.action.is_none() && row.reason.as_deref().is_some_and(|reason| !reason.is_empty()), "{id}");
        }
        assert_eq!(named(&menu, "paste").reason.as_deref(), Some("Nothing was copied"));
        assert!(menu.iter().all(|item| item.id != "norm.context.focus-check"));
        assert!(named(&menu, "select-all").disabled.is_none() && named(&menu, "select-all").action.is_some());
    }

    #[test]
    fn every_row_is_labelled_in_both_languages_and_an_entity_without_a_check_says_why() {
        let probe = hit("member", "b3");
        let (english, german) = (items(&probe, &HostContext::default(), false), items(&probe, &HostContext::default(), true));
        assert_eq!(english.iter().map(|item| &item.id).collect::<Vec<_>>(), german.iter().map(|item| &item.id).collect::<Vec<_>>());
        for (en, de) in english.iter().zip(&german).filter(|(item, _)| item.separator.is_none()) {
            assert!(en.label.as_deref().is_some_and(|label| !label.is_empty()) && de.label.as_deref().is_some_and(|label| !label.is_empty()) && en.label != de.label, "{}", en.id);
        }
        assert_eq!(named(&english, "focus-check").reason.as_deref(), Some("No check covers this entity"));
        assert_eq!(named(&german, "focus-check").reason.as_deref(), Some("Keine Prüfung betrifft diese Entität"));
    }

    #[test]
    fn the_touch_menu_and_the_context_menu_dispatch_the_same_entity_tools() {
        let context = HostContext { collection: "beams".into(), ids: vec!["b1".into()], clip_json: Some(copy_clip_json(&doc(), "beams", &["b1".into()]).unwrap()) };
        let menu = items(&MenuProbe { window: "norm-en1992-viewport".into(), ..MenuProbe::default() }, &context, false);
        for (entry, id) in touch_menu(1, true, None).iter().filter(|entry| matches!(shortcut_route(&entry.action_id), HostRoute::Entity(_))).zip(["copy", "paste", "duplicate", "delete"]) {
            let (tool, _) = host_entity_call(&entry.action_id, &context).expect("an entity route").expect("something to act on");
            assert_eq!(named(&menu, id).action.as_deref(), Some(tool), "{id}");
        }
    }
}

mod batch_delete {
    use super::*;

    fn tree_with(ids: &[&str]) -> DslValue {
        DslValue::Object(vec![("beams".to_string(), DslValue::Array(ids.iter().map(|id| row(id, 0.0, None)).collect()))])
    }

    #[test]
    fn a_batch_delete_removes_every_selected_row_from_the_last_position_first_so_all_positions_stay_valid() {
        let mut tree = doc();
        let edits = delete_edits(&tree, "beams", &["b1".into(), "b3".into()]).unwrap();
        assert_eq!(edits, [NormEdit::RemoveItem { path: "beams".into(), index: 2 }, NormEdit::RemoveItem { path: "beams".into(), index: 0 }]);
        apply_to_tree(&mut tree, &edits).unwrap();
        assert_eq!(ids(&tree), ["b2"]);
        assert_eq!(delete_edits(&doc(), "beams", &["b2".into(), "b2".into()]).unwrap().len(), 1, "a row listed twice leaves once");
    }

    #[test]
    fn a_batch_delete_is_one_emit_so_it_is_one_undo_step_and_one_failing_row_refuses_all() {
        let tree = doc();
        let edits = delete_edits(&tree, "beams", &["b1".into(), "b2".into()]).unwrap();
        let emit = handle_edits(&tree, &edits, |_, edit| Ok(vec![format!("{edit:?}")])).unwrap();
        assert_eq!(emit.artifact_mutations.len(), 2, "one emission carries both removals");
        assert!(handle_edits(&tree, &edits, |_, edit| if matches!(edit, NormEdit::RemoveItem { index: 0, .. }) { Err("invariant".to_owned()) } else { Ok(vec![String::new()]) }).is_err(), "the batch is all or nothing");
    }

    #[test]
    fn nothing_selected_an_unknown_list_and_an_unknown_row_are_refused_with_a_localized_reason_and_a_notice() {
        let tree = tree_with(&["b1"]);
        let refusals = [
            (delete_edits(&tree, "beams", &[]), "norm.entity-delete-empty"),
            (delete_edits(&tree, "walls", &["b1".into()]), "norm.entity-delete-unknown"),
            (delete_edits(&tree, "beams", &["b1".into(), "gone".into()]), "norm.entity-delete-unknown"),
        ];
        for (result, code) in refusals {
            let refusal = result.unwrap_err();
            assert_eq!(refusal.code(), code);
            assert!(crate::norm_fault_notices().iter().any(|(known, _)| *known == code), "{code} has a notice");
            assert_ne!(refusal.message(false), refusal.message(true));
        }
    }

    #[test]
    fn the_language_agnostic_rows_replay_to_the_same_edits_refusals_notices_and_one_undo_step() {
        use semio_framework_ui_locale::{Locale, Terminology};
        let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🗑️delete-entities/🔣️.json")).expect("delete fixture");
        let dsl = |value: &serde_json::Value| semio_framework_pack_json::from_json_str::<DslValue>(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture value");
        let text = |value: &serde_json::Value| value.as_str().expect("text").to_owned();
        let tree = dsl(&law["document"]);
        for (code, expected) in law["notices"].as_object().expect("notices") {
            let (_, label) = crate::norm_fault_notices().iter().find(|(known, _)| known == code).unwrap_or_else(|| panic!("{code} has a notice"));
            assert_eq!(label.resolve(Terminology::Native, Locale::En).to_string(), text(&expected["en"]), "{code} en");
            assert_eq!(label.resolve(Terminology::Native, Locale::De).to_string(), text(&expected["de"]), "{code} de");
        }
        for row in law["arguments"].as_array().expect("arguments") {
            let parsed = LiveCommand::parse("deleteEntities", Some(&dsl(&row["args"])));
            match row["error"].as_str() {
                Some(error) => assert_eq!(parsed, Err(error.to_owned()), "{}", row["id"]),
                None => assert_eq!(parsed, Ok(LiveCommand::DeleteEntities { collection: text(&row["parsed"]["collection"]), ids: row["parsed"]["ids"].as_array().unwrap().iter().map(text).collect() }), "{}", row["id"]),
            }
        }
        for row in law["cases"].as_array().expect("cases") {
            let (collection, ids): (String, Vec<String>) = (text(&row["collection"]), row["ids"].as_array().unwrap().iter().map(text).collect());
            let remaining = |tree: &DslValue| -> Vec<String> { tree.get(&collection).and_then(DslValue::as_array).map(|rows| rows.iter().map(|row| object_id_field(row).unwrap().to_owned()).collect()).unwrap_or_default() };
            match (row["outcome"].as_str().unwrap(), delete_edits(&tree, &collection, &ids)) {
                ("emitted", Ok(edits)) => {
                    let expected: Vec<NormEdit> = row["edits"].as_array().unwrap().iter().map(|edit| NormEdit::RemoveItem { path: text(&edit["path"]), index: edit["index"].as_u64().unwrap() as usize }).collect();
                    assert_eq!(edits, expected, "{}", row["id"]);
                    let mut after = tree.clone();
                    apply_to_tree(&mut after, &edits).unwrap();
                    assert_eq!(remaining(&after), row["remaining"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{}", row["id"]);
                    let emit = handle_edits(&tree, &edits, |_, edit| Ok(vec![format!("{edit:?}")])).unwrap();
                    assert_eq!(emit.artifact_mutations.len() as u64, row["mutations"].as_u64().unwrap(), "{}: one emit carries every removal, so it is {} undo step", row["id"], row["undoSteps"]);
                    assert_eq!(row["undoSteps"].as_u64(), Some(1));
                }
                ("refused", Err(refusal)) => {
                    assert_eq!(refusal.code(), text(&row["code"]), "{}", row["id"]);
                    assert_eq!(refusal.message(false), text(&row["message"]["en"]), "{}", row["id"]);
                    assert_eq!(refusal.message(true), text(&row["message"]["de"]), "{}", row["id"]);
                    assert_eq!(remaining(&tree), row["remaining"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{}", row["id"]);
                }
                ("refused", Ok(edits)) => {
                    let refused_at = row["refuseRemovalAt"].as_u64().expect("a refused plan names the removal its kind refuses") as usize;
                    assert_eq!(text(&row["code"]), "norm.edit-unresolved");
                    assert!(handle_edits(&tree, &edits, |_, edit| if matches!(edit, NormEdit::RemoveItem { index, .. } if *index == refused_at) { Err(text(&row["resolverError"])) } else { Ok(vec![String::new()]) }).is_err(), "{}: the batch is all or nothing", row["id"]);
                }
                (outcome, other) => panic!("{}: {outcome} but {other:?}", row["id"]),
            }
        }
    }

    #[test]
    fn the_delete_chord_is_global_routed_to_the_batch_delete_and_declared_as_a_window_action() {
        assert_eq!(lookup(Scope::Viewport, chord("Delete", false)), Some(ShortcutAction::Delete));
        assert_eq!(lookup(Scope::Global, chord("Delete", false)), Some(ShortcutAction::Delete));
        let context = HostContext { collection: "beams".into(), ids: vec!["b1".into(), "b2".into()], clip_json: None };
        let (tool, args) = host_entity_call("norm.shortcut.delete", &context).unwrap().unwrap();
        assert_eq!(LiveCommand::parse(tool, Some(&args)), Ok(LiveCommand::DeleteEntities { collection: "beams".into(), ids: vec!["b1".into(), "b2".into()] }));
        assert!(host_entity_call("norm.shortcut.delete", &HostContext::default()).unwrap().is_err(), "nothing selected, nothing to delete");
        assert!(crate::shortcut_action_definitions().iter().any(|action| action.id == "norm.shortcut.delete"));
    }
}

mod marker_fixtures {
    use semio_s_plugin_norm_structural_scene::*;

    pub fn diagram() -> Diagram2d {
        let axis = |label: &str, unit: &str, domain: Option<Interval>| Axis { label: LocalizedCopy::new(label, label), quantity: QuantityKind::Length, unit: unit.into(), scale: AxisScale::Linear, domain, ticks: TickRule::Auto };
        let mut binding = FieldBinding::new("placements[id=p1].x", Affine { base: 0.0, k: 1.0 }, "m");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        Diagram2d {
            id: "plan".into(),
            title: LocalizedCopy::new("Plan", "Grundriss"),
            description: LocalizedCopy::new("Plan", "Grundriss"),
            axes: vec![axis("Position x", "m", Some(Interval::new(0.0, 10.0))), axis("Deflection", "mm", None)],
            series: Vec::new(),
            markers: vec![Marker { id: "p1".into(), at: Vec2::new(2.5, 14.0), shape: MarkerShape::Diamond, tone: Tone::Warning, label: LocalizedCopy::new("Placement P1", "Platzierung P1"), utilisation: Some(0.93), bound_check: Some("c1".into()), binding: Some(binding) }, Marker { id: "fixed".into(), at: Vec2::new(1.0, 1.0), shape: MarkerShape::Circle, tone: Tone::Neutral, label: LocalizedCopy::new("Fixed", "Fest"), utilisation: None, bound_check: None, binding: None }],
            annotations: Vec::new(),
            focus: None,
            legend: false,
            table_twin: DataTable { columns: Vec::new() },
        }
    }
}

#[test]
fn a_bound_marker_is_a_grip_that_rests_at_its_field_value_and_a_table_row_commits_like_a_drag() {
    let diagram = marker_fixtures::diagram();
    let marker = &diagram.markers[0];
    assert!(crate::diagrams2d::marker_grip(&diagram, &diagram.markers[1]).is_none(), "an unbound marker is not draggable");
    let grip = crate::diagrams2d::marker_grip(&diagram, marker).expect("grip");
    assert_eq!((grip.id.as_str(), grip.accessible_name.de.as_str()), ("marker:plan:p1", "Platzierung P1"));
    assert_eq!(grip.limits, Some(Interval::new(0.0, 10.0)), "limited to the domain of the axis in the unit of the field");
    assert_eq!(crate::diagrams2d::marker_rest(&diagram, marker), Some(2.5));
    assert_eq!(crate::diagrams2d::marker_utilisation(marker), Some((0.93, semio_s_plugin_norm_structural_scene::CheckStatus::Warning)));
    let handles = [grip];
    let tool = HandleDragTool { handles: &handles, seq: 0 };
    let (state, preview) = tool.step(DragState::Hover { handle: "marker:plan:p1".into(), s: 2.5 }, DragEvent::Nudge { direction: 1.0, modifiers: Modifiers::NONE });
    assert!((match &preview { DragEffect::Preview(p) => p.writes[0].value.as_f64().expect("number"), other => panic!("{other:?}") } - 2.51).abs() < 1e-12, "an arrow moves the marker by the schema step of its field");
    let (_, committed) = tool.step(state, DragEvent::Commit);
    let only = |writes: Option<Vec<FieldWrite>>| match writes.as_deref() {
        Some([write]) if write.path == "placements[id=p1].x" => write.value.as_f64(),
        other => panic!("one write of the bound field expected, got {other:?}"),
    };
    assert!((only(match committed { DragEffect::Commit(writes) => Some(writes), _ => None }).expect("number") - 2.51).abs() < 1e-12);
    assert!((only(marker_row_commit(&diagram, marker, "350 cm")).expect("number") - 3.5).abs() < 1e-12, "a typed quantity in another unit of the same dimension");
    assert!((only(marker_row_commit(&diagram, marker, "99")).expect("number") - 10.0).abs() < 1e-12, "clamped to the limits");
    assert_eq!(marker_row_commit(&diagram, marker, "2,5"), None, "the current value commits nothing");
    assert_eq!(marker_row_commit(&diagram, marker, "3 kN"), None, "a unit of another dimension is refused");
    assert_eq!(marker_row_commit(&diagram, &diagram.markers[1], "3"), None);
}
