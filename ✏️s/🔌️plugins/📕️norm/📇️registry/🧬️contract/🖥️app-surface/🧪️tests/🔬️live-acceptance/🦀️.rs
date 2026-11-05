//! 🧪️ Live direct-manipulation acceptance of one editor, stamped into its unit tests: a drag previews in the transient lane and commits as one `setFields` edit, Escape leaves no trace, a keyboard nudge commits what the equivalent drag commits, and the preview document is what every window paints.

/// 🧪️ Declares the `live_acceptance` test module of an editor; `entities` is `fn(&Snapshot) -> Vec<EntityRef>` over the default document.
#[macro_export]
macro_rules! norm_live_acceptance {
    ($app:ty, entities = $entities:path) => {
        #[cfg(test)]
        mod live_acceptance {
            use super::*;
            use semio_framework_value::ToValue;
            type Doc = <$app as semio_plugin::ArtifactEditor>::Snapshot;
            type Family = <$app as $crate::NormRetainedEditor>::Family;

            fn probe() -> (Doc, semio_s_plugin_norm_structural_scene::EntityRef, semio_s_plugin_norm_structural_scene::Handle, f64, f64) {
                let doc = Doc::default();
                for entity in $entities(&doc) {
                    for handle in <$app as $crate::NormLiveEditor>::live_handles(&doc, std::slice::from_ref(&entity)) {
                        let lo = handle.limits.as_ref().map_or(0.0, |limits| limits.clamp(-1.0e9));
                        let hi = handle.limits.as_ref().map_or(1.0, |limits| limits.clamp(1.0e9));
                        if $crate::direct_manipulation::writes_for(&handle, lo) != $crate::direct_manipulation::writes_for(&handle, hi) {
                            return (doc, entity, handle, lo, hi);
                        }
                    }
                }
                panic!("the default document offers a handle with a free range");
            }

            fn json(entity: &semio_s_plugin_norm_structural_scene::EntityRef, handle: &semio_s_plugin_norm_structural_scene::Handle, s: f64, origin: f64, step: &str) -> String {
                format!(r#"{{"entity":{{"kind":"{}","id":"{}"}},"handle":"{}","s":{s},"origin":{origin},{step}}}"#, $crate::viewport3d::picking::granularity(entity.kind), entity.id, handle.id)
            }

            fn transient_after(doc: &Doc, _entity: &semio_s_plugin_norm_structural_scene::EntityRef, from: &$crate::NormTransient, gesture: &str) -> $crate::NormTransient {
                let invocation = $crate::LiveInvocation::set_fields("[]", Some(gesture), None).expect("gesture json");
                $crate::lane_events::<$app>(&invocation, doc, from).into_iter().fold(from.clone(), $crate::NormTransient::apply)
            }

            #[test]
            fn a_drag_previews_in_the_transient_lane_and_commits_as_one_set_fields_event() {
                let (doc, entity, handle, lo, hi) = probe();
                let stream = json(&entity, &handle, hi, lo, r#""phase":"stream""#);
                assert!($crate::live_set_fields::<$app>(&doc, "[]", Some(&stream), None).expect("stream").artifact_mutations.is_empty(), "a streamed tick never edits the document");
                let transient = transient_after(&doc, &entity, &$crate::NormTransient::default(), &stream);
                assert!(transient.drag.is_some(), "the tick previews in the transient lane");
                assert_ne!($crate::overlay_document(&doc.to_value(), &transient).expect("overlay"), doc.to_value(), "the overlay document carries the preview");
                let shown = $crate::LiveView::<Family>::with_lanes(doc.clone(), None, &[], &transient);
                assert_ne!(shown.host.document(), &doc, "every window paints the preview document");
                assert_eq!($crate::LiveView::<Family>::with_lanes(doc.clone(), None, &[], &$crate::NormTransient::default()).host.document(), &doc, "no preview paints the committed document");
                let commit = json(&entity, &handle, hi, lo, r#""phase":"commit""#);
                let committed = $crate::live_set_fields::<$app>(&doc, "[]", Some(&commit), None).expect("commit");
                assert!(!committed.artifact_mutations.is_empty(), "the release commits the bundle as one emission");
                assert!(transient_after(&doc, &entity, &transient, &commit).drag.is_none(), "the release clears the preview");
            }

            fn placement_probe() -> (Doc, semio_s_plugin_norm_structural_scene::EntityRef, semio_s_plugin_norm_structural_scene::Handle, f64) {
                let doc = Doc::default();
                for entity in $entities(&doc) {
                    for handle in <$app as $crate::NormLiveEditor>::live_handles(&doc, std::slice::from_ref(&entity)) {
                        let moves = handle.writes.iter().any(|binding| semio_s_plugin_norm_structural_scene::projection_kit::placement_target(&binding.path).is_some());
                        let reach = [1.0, -1.0, 0.5].into_iter().map(|s| handle.limits.as_ref().map_or(s, |limits| limits.clamp(s))).find(|s| $crate::direct_manipulation::writes_for(&handle, *s) != $crate::direct_manipulation::writes_for(&handle, 0.0));
                        if let (true, Some(s)) = (moves, reach) {
                            return (doc, entity, handle, s);
                        }
                    }
                }
                panic!("the default document offers a placement grip");
            }

            #[test]
            fn a_synthetic_placement_drag_previews_the_complete_row_commits_one_event_and_reloads_to_the_same_pose() {
                use semio_s_plugin_norm_structural_scene::projection_kit::placements_of;
                let (doc, entity, handle, s) = placement_probe();
                let stream = json(&entity, &handle, s, 0.0, r#""phase":"stream""#);
                let transient = transient_after(&doc, &entity, &$crate::NormTransient::default(), &stream);
                let preview = $crate::overlay_document(&doc.to_value(), &transient).expect("the preview overlays even for an entity without a row");
                let commit = json(&entity, &handle, s, 0.0, r#""phase":"commit""#);
                let gesture = $crate::Gesture::parse(&commit).expect("gesture json");
                let handles = <$app as $crate::NormLiveEditor>::live_handles(&doc, std::slice::from_ref(&entity));
                let writes = $crate::commit_writes(&handles, &gesture).expect("the release commits");
                let rows = placements_of(&doc);
                let edits = $crate::direct_manipulation::placement_edits(&rows, &$crate::standing_of::<$app>(&doc, &handles), &writes);
                assert_eq!(edits.len(), 1, "one typed event per release: {edits:?}");
                let has_row = writes.iter().any(|write| semio_s_plugin_norm_structural_scene::projection_kit::placement_target(&write.path).is_some_and(|(id, _)| rows.iter().any(|row| row.entity_id == id)));
                assert!(has_row || matches!(edits[0], $crate::NormEdit::InsertItem { .. }), "an entity without a row is ONE insert: {edits:?}");
                let mut reloaded = doc.to_value();
                for edit in &edits {
                    match edit {
                        $crate::NormEdit::InsertItem { path, index, value } => $crate::insert_value_at_path(&mut reloaded, path, *index, value.clone()).expect("insert"),
                        $crate::NormEdit::SetField { path, value } => $crate::set_value_at_path(&mut reloaded, path, value.clone()).expect("set"),
                        $crate::NormEdit::RemoveItem { path, index } => $crate::remove_value_at_path(&mut reloaded, path, *index).expect("remove"),
                        $crate::NormEdit::MoveItem { .. } => unreachable!("a drag never reorders"),
                    }
                }
                assert_eq!(placements_of(&reloaded), placements_of(&preview), "the reloaded pose is the previewed pose");
                for edit in &edits {
                    assert_eq!(<$app as $crate::NormLiveEditor>::live_resolve(&doc, edit).expect("the typed kind resolves").len(), 1, "{edit:?} is one mutation");
                }
                let emit = $crate::live_set_fields::<$app>(&doc, "[]", Some(&commit), None).expect("commit");
                assert_eq!(emit.artifact_mutations.len(), 1, "the release raises exactly one mutation");
            }

            #[test]
            fn the_first_drag_of_a_row_less_entity_keeps_its_derived_yaw_inserts_through_dispatch_and_selects_by_id() {
                use semio_s_plugin_norm_structural_scene::projection_kit::{placement_target, placements_of};
                let (seed, entity, handle, s) = placement_probe();
                let mut tree = seed.to_value();
                $crate::set_value_at_path(&mut tree, "placements", semio_framework_value::DslValue::Array(Vec::new())).expect("the placements are dropped");
                let bare = <Doc as semio_framework_value::FromValue>::from_value(tree.clone()).expect("a document without placement rows");
                let handles = <$app as $crate::NormLiveEditor>::live_handles(&bare, std::slice::from_ref(&entity));
                let binds_yaw = handles.iter().flat_map(|handle| handle.writes.iter()).any(|binding| placement_target(&binding.path).is_some_and(|(id, leaf)| id == entity.id && leaf == "yaw"));
                let derived = $crate::direct_manipulation::derived_yaw(&<$app as $crate::NormLiveEditor>::live_scene(&bare, semio_s_plugin_norm_structural_scene::projection::ViewOptions::none()), &entity.id).unwrap_or(0.0);
                let standing = $crate::standing_of::<$app>(&bare, &handles);
                let pose = standing(&entity.id).expect("the grips give the standing pose");
                assert!(binds_yaw || (pose.yaw - derived).abs() < 1e-12, "the standing yaw is the derived yaw {derived}, not {}", pose.yaw);
                let writes = $crate::direct_manipulation::writes_for(&handle, s).into_iter().filter(|write| !$crate::is_view_write(write)).collect::<Vec<_>>();
                let edits = $crate::direct_manipulation::placement_edits(&placements_of(&bare), &standing, &writes);
                assert!(matches!(edits.as_slice(), [$crate::NormEdit::InsertItem { .. }]), "one insert for the row-less entity: {edits:?}");
                let outcome = $crate::dispatch_live_command::<Family, _>(&bare, &$crate::LiveCommand::SetFields { writes: writes.clone() }, &standing, |d, e| <$app as $crate::NormLiveEditor>::live_resolve(d, e)).unwrap_or_else(|fault| panic!("a placement leaf of a row-less entity dispatches through the insert path: {fault:?}"));
                assert!(matches!(outcome, $crate::LiveOutcome::Mutations(ref emit) if emit.artifact_mutations.len() == 1), "dispatch raises exactly the one insert");
                if let $crate::NormEdit::InsertItem { path, index, value } = &edits[0] {
                    $crate::insert_value_at_path(&mut tree, path, *index, value.clone()).expect("insert");
                }
                let row = placements_of(&tree).into_iter().find(|row| row.entity_id == entity.id).expect("the inserted row");
                assert!(binds_yaw || (row.yaw - derived).abs() < 1e-12, "the first drag keeps the derived yaw {derived}, the row says {}", row.yaw);
                let placed = <Doc as semio_framework_value::FromValue>::from_value(tree).expect("the document with its row");
                for leaf in ["x", "y", "z", "yaw"] {
                    let edit = $crate::NormEdit::SetField { path: format!("placements[id={}].{leaf}", entity.id), value: semio_framework_value::DslValue::float(1.5) };
                    let mutations = <$app as $crate::NormLiveEditor>::live_resolve(&placed, &edit).unwrap_or_else(|error| panic!("{leaf}: {error}"));
                    let text = format!("{mutations:?}");
                    assert!(mutations.len() == 1 && text.contains(&format!("id: \"{}\"", entity.id)) && !text.to_lowercase().contains("placement_id") && !text.contains("placementId"), "every placement rule selects by `id`: {text}");
                }
            }

            #[test]
            fn every_handle_binding_of_the_default_document_resolves_to_a_typed_kind() {
                let doc = Doc::default();
                let mut checked = 0;
                for entity in $entities(&doc) {
                    for handle in <$app as $crate::NormLiveEditor>::live_handles(&doc, std::slice::from_ref(&entity)) {
                        let s = handle.limits.as_ref().map_or(1.0, |limits| limits.clamp(1.0));
                        for write in $crate::direct_manipulation::writes_for(&handle, s).into_iter().filter(|write| !$crate::is_view_write(write)) {
                            let resolved = $crate::direct_manipulation::resolve_placed_fields(&doc, std::slice::from_ref(&write), &$crate::standing_of::<$app>(&doc, std::slice::from_ref(&handle)), |d, e| <$app as $crate::NormLiveEditor>::live_resolve(d, e));
                            assert!(resolved.as_ref().is_ok_and(|mutations| !mutations.is_empty()), "handle '{}' writes '{}' which resolves to no typed kind: {:?}", handle.id, write.path, resolved.err());
                            checked += 1;
                        }
                    }
                }
                assert!(checked > 0, "the default document offers handles");
            }

            #[test]
            fn escape_and_a_lost_capture_leave_no_event_and_no_trace() {
                let (doc, entity, handle, lo, hi) = probe();
                let transient = transient_after(&doc, &entity, &$crate::NormTransient::default(), &json(&entity, &handle, hi, lo, r#""phase":"stream""#));
                assert!(transient.drag.is_some());
                for abort in [r#""key":"Escape""#, r#""phase":"abort","reason":"escape""#, r#""phase":"abort","reason":"captureLost""#] {
                    let gesture = json(&entity, &handle, hi, lo, abort);
                    assert!($crate::live_set_fields::<$app>(&doc, "[]", Some(&gesture), None).expect("abort").artifact_mutations.is_empty(), "{abort} raises no event");
                    assert!(transient_after(&doc, &entity, &transient, &gesture).drag.is_none(), "{abort} leaves no trace");
                }
            }

            #[test]
            fn a_keyboard_nudge_commits_what_the_equivalent_drag_commits() {
                let (doc, entity, handle, lo, _) = probe();
                let nudged = transient_after(&doc, &entity, &$crate::NormTransient::default(), &json(&entity, &handle, lo, lo, r#""key":"ArrowRight""#));
                let target = nudged.drag.as_ref().map_or(lo, |drag| drag.s);
                let by_key = $crate::live_set_fields::<$app>(&doc, "[]", Some(&json(&entity, &handle, target, lo, r#""key":"Enter""#)), None).expect("key commit");
                let by_pointer = $crate::live_set_fields::<$app>(&doc, "[]", Some(&json(&entity, &handle, target, lo, r#""phase":"commit""#)), None).expect("pointer commit");
                assert_eq!(by_key.artifact_mutations, by_pointer.artifact_mutations);
            }

            #[test]
            fn a_grip_pick_id_without_an_entity_finds_its_handle_and_commits_the_same_bundle() {
                let (doc, entity, handle, lo, hi) = probe();
                let with_entity = $crate::live_set_fields::<$app>(&doc, "[]", Some(&json(&entity, &handle, hi, lo, r#""phase":"commit""#)), None).expect("with entity");
                let bare = format!(r#"{{"handle":"{}{}","s":{hi},"origin":{lo},"phase":"commit"}}"#, $crate::HANDLE_PICK_PREFIX, handle.id);
                let by_pick = $crate::live_set_fields::<$app>(&doc, "[]", Some(&bare), None).expect("by pick id");
                assert_eq!(with_entity.artifact_mutations, by_pick.artifact_mutations);
            }

            #[test]
            fn the_insert_tool_arms_a_snapped_ghost_and_a_click_places_one_item_as_one_emission() {
                let doc = Doc::default();
                let palette = <$app as $crate::NormLiveEditor>::live_palette();
                assert!(!palette.is_empty(), "the editor offers insertable kinds");
                let spec = &palette[0];
                let ghost = $crate::LiveInvocation::preview_remedy("", 0, 0, Some(&format!(r#"{{"spec":"{}","at":[1.234,0,2.49]}}"#, spec.id))).expect("insert ghost");
                let events = $crate::lane_events::<$app>(&ghost, &doc, &$crate::NormTransient::default());
                let armed = events.into_iter().fold($crate::NormTransient::default(), $crate::NormTransient::apply);
                assert!(matches!(armed.ghost.as_ref().map(|g| &g.source), Some($crate::GhostSource::Insert { spec: id, kind, at }) if *id == spec.id && *kind == spec.kind && (at.x - 1.25).abs() < 1e-9 && (at.z - 2.5).abs() < 1e-9), "{armed:?}");
                let moved = $crate::LiveInvocation::preview_remedy("", 0, 0, Some(r#"{"at":[3,0,0]}"#)).expect("moved ghost");
                let followed = $crate::lane_events::<$app>(&moved, &doc, &armed).into_iter().fold(armed.clone(), $crate::NormTransient::apply);
                assert!(matches!(followed.ghost.as_ref().map(|g| &g.source), Some($crate::GhostSource::Insert { spec: id, at, .. }) if *id == spec.id && (at.x - 3.0).abs() < 1e-9));
                let placed = palette.iter().find_map(|spec| {
                    let json = format!(r#"{{"spec":"{}","at":[1.234,0,2.49]}}"#, spec.id);
                    $crate::live_set_fields::<$app>(&doc, "[]", None, Some(&json)).ok().filter(|emit| !emit.artifact_mutations.is_empty())
                });
                assert!(placed.is_some(), "a click on at least one tool of the palette places an item as one emission");
                let cleared = $crate::lane_events::<$app>(&$crate::LiveInvocation::clear_preview(), &doc, &armed).into_iter().fold(armed, $crate::NormTransient::apply);
                assert!(cleared.ghost.is_none() && cleared.drag.is_none(), "Escape leaves no trace");
            }

            #[test]
            fn a_secondary_click_on_an_entity_row_opens_a_menu_whose_rows_run_the_real_editor_commands_in_both_languages() {
                use semio_framework_value::{DslValue, FromValue};
                let seed = Doc::default();
                let placements = semio_s_plugin_norm_structural_scene::projection_kit::PLACEMENTS_FIELD;
                let request = |id: &str| semio_plugin::ContextMenuRequest {
                    menu: semio_plugin::UiMenuRef { id: "window".into(), args: None },
                    surface: Some(semio_plugin::ContextMenuSurfaceTarget { surface_id: "shell".into(), kind: "window".into(), hits: vec![semio_plugin::ContextMenuHit { domain: "tree".into(), id: format!("norm-outliner.{id}.select"), label: None }], selection: Vec::new(), text: None }),
                    window_instance_id: Some("norm-test-outliner".into()),
                    point: None,
                };
                let mut asked = 0;
                for spec in <$app as $crate::NormLiveEditor>::live_palette().iter().filter(|spec| spec.collection != placements) {
                    let mut tree = seed.to_value();
                    if tree.get(&spec.collection).and_then(DslValue::as_array).is_none_or(|rows| rows.is_empty()) {
                        let insert = $crate::direct_manipulation::insert_edit(&tree, spec, [0.0, 0.0, 0.0]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                        $crate::entity_editing::apply_to_tree(&mut tree, &[insert]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    }
                    let Some(id) = tree.get(&spec.collection).and_then(DslValue::as_array).and_then(|list| list.first()).and_then($crate::object_id_field).map(str::to_owned) else { continue };
                    let doc = <Doc as FromValue>::from_value(tree.clone()).unwrap_or_else(|error| panic!("kind '{}': the document with a first row does not load: {error:?}", spec.id));
                    let path = format!("{}[id={id}]", spec.collection);
                    let english = $crate::live_context_menu::<$app>(&request(&path), &doc, false);
                    let german = $crate::live_context_menu::<$app>(&request(&path), &doc, true);
                    assert_eq!(english.iter().map(|item| &item.id).collect::<Vec<_>>(), german.iter().map(|item| &item.id).collect::<Vec<_>>(), "kind '{}': both languages offer the same rows", spec.id);
                    let find = |items: &[semio_plugin::ContextMenuItemSpec], name: &str| items.iter().find(|item| item.id == format!("norm.context.{name}")).cloned().unwrap_or_else(|| panic!("kind '{}': row {name}", spec.id));
                    let duplicate = find(&english, "duplicate");
                    let command = $crate::LiveCommand::parse(duplicate.action.as_deref().expect("duplicate runs"), duplicate.args.as_ref()).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    let emitted = $crate::live_entity_edit::<$app>(&doc, &command).unwrap_or_else(|fault| panic!("kind '{}': {fault:?}", spec.id));
                    assert!(!emitted.artifact_mutations.is_empty(), "kind '{}': the menu's duplicate is the real duplicate", spec.id);
                    let copy = find(&english, "copy");
                    $crate::LiveCommand::parse(copy.action.as_deref().expect("copy runs"), copy.args.as_ref()).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    let (delete, delete_de) = (find(&english, "delete"), find(&german, "delete"));
                    assert!(delete.action.as_deref() == Some("deleteEntities") && delete.label != delete_de.label, "kind '{}': delete is the batch delete command and is localized", spec.id);
                    let command = $crate::LiveCommand::parse("deleteEntities", delete.args.as_ref()).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    let removed = $crate::live_entity_edit::<$app>(&doc, &command).unwrap_or_else(|fault| panic!("kind '{}': the menu's delete does not resolve: {fault:?}", spec.id));
                    assert!(!removed.artifact_mutations.is_empty(), "kind '{}': the menu's delete is the real delete", spec.id);
                    asked += 1;
                }
                assert!(asked > 0, "the default document has an entity row to ask the menu about");
            }

            #[test]
            fn every_registered_entity_kind_supports_duplicate_copy_paste_and_bulk_edit_as_one_undoable_batch() {
                use semio_framework_value::{DslValue, FromValue};
                let seed = Doc::default();
                let palette = <$app as $crate::NormLiveEditor>::live_palette();
                assert!(!palette.is_empty(), "the editor registers entity kinds");
                let placements = semio_s_plugin_norm_structural_scene::projection_kit::PLACEMENTS_FIELD;
                let mut covered = 0;
                for spec in palette.iter().filter(|spec| spec.collection != placements) {
                    let mut tree = seed.to_value();
                    let count = |tree: &DslValue| tree.get(&spec.collection).and_then(DslValue::as_array).map(|rows| rows.len());
                    let before = count(&tree).unwrap_or_else(|| panic!("kind '{}': the snapshot has no list '{}'", spec.id, spec.collection));
                    if before == 0 {
                        let insert = $crate::direct_manipulation::insert_edit(&tree, spec, [0.0, 0.0, 0.0]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                        $crate::entity_editing::apply_to_tree(&mut tree, &[insert]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    }
                    let rows = count(&tree).unwrap_or_default();
                    let doc = <Doc as FromValue>::from_value(tree.clone()).unwrap_or_else(|error| panic!("kind '{}': the document with a first row does not load: {error:?}", spec.id));
                    let first = tree.get(&spec.collection).and_then(DslValue::as_array).and_then(|list| list.first()).and_then($crate::object_id_field).unwrap_or_else(|| panic!("kind '{}': rows carry an id", spec.id)).to_string();
                    let ids = vec![first.clone()];

                    let pasted = $crate::entity_editing::duplicate_edits(&tree, &spec.collection, &ids, &$crate::entity_editing::paste_offsets()).unwrap_or_else(|error| panic!("kind '{}': duplicate: {error}", spec.id));
                    let mut after = tree.clone();
                    $crate::entity_editing::apply_to_tree(&mut after, &pasted.edits).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    assert_eq!(count(&after), Some(rows + 1), "kind '{}': a duplicate adds exactly one row", spec.id);
                    assert!(pasted.ids.iter().all(|id| *id != first), "kind '{}': a copy gets a fresh id", spec.id);
                    assert!(<Doc as FromValue>::from_value(after).is_ok(), "kind '{}': the duplicated document loads", spec.id);
                    let duplicated = $crate::live_entity_edit::<$app>(&doc, &$crate::LiveCommand::DuplicateEntities { collection: spec.collection.clone(), ids: ids.clone() }).unwrap_or_else(|fault| panic!("kind '{}': duplicate does not resolve: {fault:?}", spec.id));
                    assert!(!duplicated.artifact_mutations.is_empty(), "kind '{}': duplicate raises one emission", spec.id);

                    $crate::live_copy_entities::<$app>(&doc, &spec.collection, &format!(r#"["{first}"]"#)).unwrap_or_else(|fault| panic!("kind '{}': copy: {fault:?}", spec.id));
                    let clip = $crate::entity_editing::copy_clip_json(&tree, &spec.collection, &ids).unwrap_or_else(|error| panic!("kind '{}': copy: {error}", spec.id));
                    let pasted = $crate::live_entity_edit::<$app>(&seed, &$crate::LiveCommand::PasteEntities { clip_json: clip, at: None }).unwrap_or_else(|fault| panic!("kind '{}': a paste into another document does not resolve: {fault:?}", spec.id));
                    assert!(!pasted.artifact_mutations.is_empty(), "kind '{}': paste raises one emission", spec.id);

                    let leaves: Vec<(String, DslValue)> = match tree.get(&spec.collection).and_then(DslValue::as_array).and_then(|list| list.first()) {
                        Some(DslValue::Object(entries)) => entries.iter().filter(|(key, value)| key != "id" && matches!(value, DslValue::Number(_) | DslValue::Bool(_) | DslValue::String(_))).cloned().collect(),
                        _ => Vec::new(),
                    };
                    let editable = leaves.iter().any(|(key, value)| {
                        let writes = $crate::entity_editing::bulk_writes(&tree, &spec.collection, &ids, key, value).expect("bulk writes");
                        let mut resolve = |d: &Doc, e: &$crate::NormEdit| <$app as $crate::NormLiveEditor>::live_resolve(d, e);
                        $crate::resolve_set_fields(&doc, &writes, &mut resolve).is_ok_and(|mutations| !mutations.is_empty())
                    });
                    assert!(editable, "kind '{}': at least one field of '{}' is bulk editable through the edit rules", spec.id, spec.collection);
                    covered += 1;
                }
                assert!(covered > 0, "the editor registers a kind with rows");
            }

            #[test]
            fn every_registered_entity_kind_deletes_a_multi_selection_as_one_atomic_batch_that_undoes_and_redoes() {
                use semio_framework_value::{DslValue, FromValue};
                type Mut = <$app as semio_plugin::ArtifactEditor>::Mutation;
                fn step(document: &Doc, mutation: &Mut) -> Doc {
                    let raised = <Mut as protocol::ColdMutation<Doc>>::diff(mutation, document);
                    protocol::apply_diff_cold(raised.diff(), document).unwrap_or_else(|error| panic!("a mutation of the batch does not apply: {error:?}"))
                }
                let seed = Doc::default();
                let placements = semio_s_plugin_norm_structural_scene::projection_kit::PLACEMENTS_FIELD;
                let mut covered = 0;
                for spec in <$app as $crate::NormLiveEditor>::live_palette().iter().filter(|spec| spec.collection != placements) {
                    let mut tree = seed.to_value();
                    let rows = |tree: &DslValue| tree.get(&spec.collection).and_then(DslValue::as_array).map(|rows| rows.iter().filter_map(|row| $crate::object_id_field(row).map(str::to_owned)).collect::<Vec<String>>()).unwrap_or_default();
                    if rows(&tree).is_empty() {
                        let insert = $crate::direct_manipulation::insert_edit(&tree, spec, [0.0, 0.0, 0.0]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                        $crate::entity_editing::apply_to_tree(&mut tree, &[insert]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    }
                    let first = rows(&tree).into_iter().next().unwrap_or_else(|| panic!("kind '{}': rows carry an id", spec.id));
                    let copy = $crate::entity_editing::duplicate_edits(&tree, &spec.collection, std::slice::from_ref(&first), &[]).unwrap_or_else(|error| panic!("kind '{}': duplicate: {error}", spec.id));
                    $crate::entity_editing::apply_to_tree(&mut tree, &copy.edits).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    let before = rows(&tree);
                    let selection = vec![first.clone(), copy.ids[0].clone()];
                    let Ok(doc) = <Doc as FromValue>::from_value(tree.clone()) else { panic!("kind '{}': the document with two rows does not load", spec.id) };

                    let emitted = $crate::live_entity_edit::<$app>(&doc, &$crate::LiveCommand::DeleteEntities { collection: spec.collection.clone(), ids: selection.clone() }).unwrap_or_else(|fault| panic!("kind '{}': the multi-delete does not resolve: {fault:?}", spec.id));
                    assert_eq!(emitted.artifact_mutations.len(), 2, "kind '{}': one emission carries the removal of every selected row", spec.id);
                    let mut current = <Doc as FromValue>::from_value(tree.clone()).unwrap_or_else(|error| panic!("kind '{}': {error:?}", spec.id));
                    let mut inverses = Vec::new();
                    for mutation in &emitted.artifact_mutations {
                        inverses.push(<Mut as protocol::ColdMutation<Doc>>::inverse(mutation, &current).unwrap_or_else(|error| panic!("kind '{}': a removal has no inverse: {error:?}", spec.id)));
                        current = step(&current, mutation);
                    }
                    let deleted = current.to_value();
                    assert_eq!(rows(&deleted), before.iter().filter(|id| !selection.contains(id)).cloned().collect::<Vec<_>>(), "kind '{}': exactly the selected rows are gone", spec.id);

                    for rows_back in inverses.iter().rev() {
                        for mutation in rows_back {
                            current = step(&current, mutation);
                        }
                    }
                    assert_eq!(current.to_value(), tree, "kind '{}': undoing the one batch restores the document", spec.id);

                    for mutation in &emitted.artifact_mutations {
                        current = step(&current, mutation);
                    }
                    assert_eq!(current.to_value(), deleted, "kind '{}': redoing the batch deletes the same rows again", spec.id);

                    let refused = $crate::live_entity_edit::<$app>(&doc, &$crate::LiveCommand::DeleteEntities { collection: spec.collection.clone(), ids: vec![first.clone(), "no-such-row".to_owned()] });
                    assert!(refused.is_err(), "kind '{}': an unknown row refuses the whole batch", spec.id);
                    covered += 1;
                }
                assert!(covered > 0, "the editor registers a kind with rows");
            }

            #[test]
            fn every_grip_of_every_entity_kind_is_keyboard_operable_and_has_an_accessible_name_in_both_languages() {
                use $crate::accessibility::screen_reader::handle_node;
                use $crate::direct_manipulation::{cycle_focus, key_intent, nudge_step, writes_for, DragEffect, DragEvent, DragState, HandleDragTool, Key, KeyIntent, Modifiers};
                use $crate::viewport3d::config::Lang;
                use semio_framework_value::{DslValue, FromValue};
                use semio_s_plugin_norm_structural_scene::{projection::ViewOptions, projection_kit::base_id, EntityRef};
                let seed = Doc::default();
                let mut cases: Vec<(Doc, EntityRef, String)> = $entities(&seed).into_iter().map(|entity| (seed.clone(), entity, "default document".to_owned())).collect();
                for spec in <$app as $crate::NormLiveEditor>::live_palette() {
                    let mut tree = seed.to_value();
                    if tree.get(&spec.collection).and_then(DslValue::as_array).is_none_or(|rows| rows.is_empty()) {
                        let insert = $crate::direct_manipulation::insert_edit(&tree, &spec, [0.0, 0.0, 0.0]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                        $crate::entity_editing::apply_to_tree(&mut tree, &[insert]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    }
                    let Some(id) = tree.get(&spec.collection).and_then(DslValue::as_array).and_then(|list| list.first()).and_then($crate::object_id_field).map(str::to_owned) else { continue };
                    let doc = <Doc as FromValue>::from_value(tree).unwrap_or_else(|error| panic!("kind '{}': the document with a first row does not load: {error:?}", spec.id));
                    let path = format!("{}[id={id}]", spec.collection);
                    cases.push((doc, EntityRef::new(spec.kind, id, path), format!("kind '{}'", spec.id)));
                }
                let enter = || match key_intent(&Key::Enter, Modifiers::NONE) { KeyIntent::Drag(event) => event, other => panic!("{other:?}") };
                let (mut grips, mut moved) = (0, 0);
                for (doc, entity, what) in &cases {
                    let handles = <$app as $crate::NormLiveEditor>::live_handles(doc, std::slice::from_ref(entity));
                    let scene = <$app as $crate::NormLiveEditor>::live_scene(doc, ViewOptions::none());
                    let ids: Vec<String> = handles.iter().map(|handle| handle.id.clone()).collect();
                    let tool = HandleDragTool { handles: &handles, seq: 1 };
                    for handle in &handles {
                        grips += 1;
                        let who = format!("{what}: grip '{}' of {}", handle.id, entity.id);
                        assert!(!handle.accessible_name.en.trim().is_empty() && !handle.accessible_name.de.trim().is_empty(), "{who} has no accessible name in both languages");
                        assert_eq!(cycle_focus(&ids, Some(&handle.id), 0).as_deref(), Some(handle.id.as_str()), "{who} is in the Tab order");
                        let rest = $crate::accessibility::published_nodes::rest_parameter(handle);
                        let governing = scene.entities.iter().find(|drawn| base_id(&drawn.entity.id) == base_id(&entity.id)).and_then(|drawn| drawn.utilisation.as_ref()).map(|u| (u.ratio, u.status)).filter(|(ratio, status)| ratio.is_finite() && *status != semio_s_plugin_norm_structural_scene::CheckStatus::NotApplicable);
                        for lang in [Lang::En, Lang::De] {
                            let node = handle_node(handle, Some(rest), governing, lang);
                            let name = if lang == Lang::De { &handle.accessible_name.de } else { &handle.accessible_name.en };
                            assert!((node.role, node.name.as_str()) == ("slider", name.as_str()) && node.value_text.is_some() && node.range.is_some_and(|range| range.min.is_some() == handle.limits.is_some() && range.max.is_some() == handle.limits.is_some()) && node.missing_required().is_empty(), "{who}: slider node in {lang:?}: {node:?}");
                            assert!(governing.is_none() || node.value_text.as_deref().is_some_and(|text| text.contains('%')), "{who}: the slider speaks the live utilisation of its bound check in {lang:?}");
                        }
                        let hover = DragState::Hover { handle: handle.id.clone(), s: rest };
                        let reaches = [Key::ArrowRight, Key::ArrowLeft].iter().any(|arrow| {
                            let KeyIntent::Drag(nudge) = key_intent(arrow, Modifiers::NONE) else { panic!("{who}: an arrow nudges") };
                            matches!(tool.step(hover.clone(), nudge), (_, DragEffect::Preview(preview)) if preview.writes != writes_for(handle, rest))
                        });
                        let span = handle.limits.as_ref().map_or(f64::INFINITY, |limits| limits.max - limits.min);
                        assert!(reaches || span <= 0.0, "{who}: no arrow key changes the bound field from its rest value (step {})", nudge_step(handle, Modifiers::NONE));
                        moved += usize::from(reaches);
                        assert!(nudge_step(handle, Modifiers::ALT) < nudge_step(handle, Modifiers::NONE) && nudge_step(handle, Modifiers::NONE) < nudge_step(handle, Modifiers::SHIFT), "{who}: Alt is fine and Shift is large");
                        let is_token = handle.writes.first().is_some_and(|binding| matches!(binding.quantise, Some(semio_s_plugin_norm_structural_scene::Quantise::Token { .. })));
                        let (opened, effect) = tool.step(hover.clone(), enter());
                        assert!(matches!(effect, DragEffect::Preview(ref preview) if !preview.entry.is_empty()) != is_token, "{who}: Enter opens the value field exactly for numeric grips");
                        if !is_token {
                            assert_eq!(tool.step(opened.clone(), enter()).1, DragEffect::Cleared, "{who}: applying the untouched field commits nothing");
                            assert_eq!(tool.step(opened, DragEvent::Abort($crate::direct_manipulation::AbortReason::Escape)).1, DragEffect::Cleared, "{who}: Escape closes the field without a trace");
                        }
                    }
                }
                assert!(grips > 0 && moved > 0, "the editor offers grips that the keyboard moves ({grips} grips, {moved} moved)");
            }

            #[test]
            fn the_grip_walk_cycles_inside_an_entity_the_shell_keys_drive_it_and_the_paint_marks_the_focus_and_honours_the_platform_preferences() {
                use $crate::{walk_grips, GripRef, GripStep, GripWalk};
                use $crate::accessibility::A11yPrefs;
                use $crate::viewport3d::lane_overlays::{grips, GripView, LaneOverlays};
                use semio_framework::PluginShellHooks as _;
                use semio_plugin::World3dAnnotation;
                let doc = Doc::default();
                let handles = <$app as $crate::NormLiveEditor>::live_handles(&doc, &$entities(&doc));
                let refs: Vec<GripRef> = handles.iter().map(|handle| GripRef { id: handle.id.clone(), entity: handle.entity.id.clone() }).collect();
                assert!(!refs.is_empty(), "the default document offers grips");
                for start in &refs {
                    let ring: Vec<&String> = refs.iter().filter(|grip| grip.entity == start.entity).map(|grip| &grip.id).collect();
                    let mut at = start.id.clone();
                    let mut visited = vec![at.clone()];
                    for _ in 0..ring.len() {
                        let GripStep::To(next) = walk_grips(&refs, Some(&at), GripWalk::ForwardInEntity) else { panic!("grip '{}': Ctrl+Arrow finds a next grip", start.id) };
                        assert!(ring.contains(&&next), "grip '{}': Ctrl+Arrow left the entity '{}' for '{next}'", start.id, start.entity);
                        visited.push(next.clone());
                        at = next;
                    }
                    assert_eq!(at, start.id, "a full turn of Ctrl+Arrow returns to the grip it started at");
                    visited.truncate(ring.len());
                    visited.sort();
                    visited.dedup();
                    assert_eq!(visited.len(), ring.len(), "grip '{}': Ctrl+Arrow visits every grip of its entity once", start.id);
                    let GripStep::To(forward) = walk_grips(&refs, Some(&start.id), GripWalk::ForwardInEntity) else { panic!("forward") };
                    assert_eq!(walk_grips(&refs, Some(&forward), GripWalk::BackwardInEntity), GripStep::To(start.id.clone()), "Ctrl+Arrow backward undoes forward");
                }
                let variant = crate::VARIANT;
                let domain = $crate::interaction_domain_id(variant);
                let window = format!("norm-{variant}-viewport");
                $crate::note_grips(&domain, refs.clone());
                $crate::set_focused_grip(&domain, None);
                let hooks = $crate::NormShellHooks;
                let press = |key: &str, ctrl: bool| hooks.handle_key(&semio_framework::ShellKeyEvent { scope: window.clone(), key: key.to_owned(), ctrl, alt: false, shift: false, meta: false });
                assert_eq!(press("Enter", false), semio_framework::ShellKeyOutcome::Consumed, "Enter in the viewport starts the grip walk");
                assert_eq!($crate::focused_grip(&domain).as_deref(), Some(refs[0].id.as_str()));
                assert_eq!(press("ArrowRight", true), semio_framework::ShellKeyOutcome::Consumed);
                let first_ring: Vec<&GripRef> = refs.iter().filter(|grip| grip.entity == refs[0].entity).collect();
                assert_eq!($crate::focused_grip(&domain).as_deref(), Some(first_ring[1 % first_ring.len()].id.as_str()), "one Ctrl+Arrow moves to the next grip of the same entity");
                assert!(matches!(press("ArrowRight", false), semio_framework::ShellKeyOutcome::Action { .. }), "plain arrows stay nudges");
                let focused = $crate::focused_grip(&domain).expect("the walk holds a focus");
                let frame = $crate::LaneFrame::default();
                assert_eq!(LaneOverlays::of_frame(&frame, &domain).focused.as_deref(), Some(focused.as_str()), "the paint reads the focus the keys set");
                let marked = |prefs: A11yPrefs| grips(&handles, &GripView { focused: Some(&focused), prefs, ..GripView::default() });
                for prefs in [A11yPrefs::default(), A11yPrefs { forced_colors: true, ..A11yPrefs::default() }, A11yPrefs { prefers_reduced_motion: true, ..A11yPrefs::default() }] {
                    let marks = marked(prefs);
                    assert_eq!(marks.len(), handles.len() + 1, "one ring per grip and a cross at the focused one");
                    assert!(marks.iter().any(|mark| matches!(mark, World3dAnnotation::Marker(marker) if marker.id == format!("focus:{focused}"))), "the focused grip is marked by shape, not by colour alone");
                }
                hooks.platform_prefs_changed(&semio_framework::ShellPlatformPrefs { forced_colors: true, prefers_contrast_more: false, prefers_dark: false, prefers_reduced_motion: true });
                assert!($crate::platform_a11y_prefs().forced_colors && $crate::platform_a11y_prefs().prefers_reduced_motion && LaneOverlays::of_frame(&frame, &domain).prefs.forced_colors, "the preferences the shell reported reach the overlays");
                hooks.platform_prefs_changed(&semio_framework::ShellPlatformPrefs::default());
                $crate::set_focused_grip(&domain, None);
            }

            #[test]
            fn every_bound_diagram_marker_commits_a_typed_row_value_as_one_set_fields_and_ignores_the_shown_value() {
                use $crate::direct_manipulation::{entry_field_text, nudge_step, Modifiers};
                let doc = Doc::default();
                let diagrams = <$app as $crate::NormLiveEditor>::live_diagrams(&doc);
                for diagram in &diagrams {
                    for marker in &diagram.markers {
                        let Some(grip) = $crate::diagrams2d::marker_grip(diagram, marker) else { continue };
                        let rest = $crate::diagrams2d::marker_rest(diagram, marker).expect("a bound marker rests at its field value");
                        let (binding, who) = (grip.writes[0].clone(), format!("marker '{}' of diagram '{}'", marker.id, diagram.id));
                        let commit = |text: &str| {
                            let gesture = format!("{{\"handle\":\"{}\",\"key\":\"Enter\",\"s\":{rest},\"origin\":{rest},\"entry\":\"{text}\"}}", grip.id);
                            $crate::live_set_fields::<$app>(&doc, "null", Some(&gesture), None).unwrap_or_else(|fault| panic!("{who}: {fault:?}"))
                        };
                        let shown = entry_field_text(&grip, rest).map_or_else(String::new, |text| format!("{text} {}", binding.unit));
                        if !shown.is_empty() {
                            assert!(commit(&shown).artifact_mutations.is_empty(), "{who}: typing the shown value '{shown}' commits nothing");
                        }
                        let step = nudge_step(&grip, Modifiers::SHIFT);
                        let other = [rest + step * 3.0, rest - step * 3.0].into_iter().map(|s| grip.limits.as_ref().map_or(s, |limits| limits.clamp(s))).filter(|s| (s - rest).abs() > f64::EPSILON).map(|s| format!("{} {}", binding.value_at(s), binding.unit)).collect::<Vec<_>>();
                        assert!(other.is_empty() || other.iter().any(|text| !commit(text).artifact_mutations.is_empty()), "{who}: no typed value other than the shown one commits");
                    }
                }
            }

            #[test]
            fn the_open_drags_of_the_peers_are_mirrored_into_the_transient() {
                let peer = $crate::PeerDrag { user: "ana".into(), drag: $crate::DragPreview { handle: "h".into(), writes: Vec::new(), started_seq: 0, origin: 0.0, s: 1.0, entry: String::new() } };
                let seen = $crate::NormTransient::default().apply($crate::LaneEvent::PeersSeen(vec![peer.clone()]));
                assert_eq!(seen.peers, vec![peer]);
                assert!(seen.apply($crate::LaneEvent::PeersSeen(Vec::new())).is_clear());
            }

            #[test]
            fn the_dragged_handle_keeps_its_snap_targets_and_gains_those_of_the_scene() {
                let (doc, entity, handle, _lo, _hi) = probe();
                let gesture = $crate::Gesture::parse(&json(&entity, &handle, 0.0, 0.0, r#""key":"Tab""#)).expect("gesture json");
                let dragged = $crate::gesture_handles::<$app>(&doc, &gesture).into_iter().find(|candidate| candidate.id == handle.id).expect("the dragged handle");
                assert!(handle.snap.targets.iter().all(|target| dragged.snap.targets.contains(target)), "the handle's own targets stay");
                assert_eq!(dragged.writes.len(), handle.writes.len());
            }

            #[test]
            fn the_retained_handles_of_a_drag_equal_the_resolved_ones_and_follow_the_document() {
                let (doc, entity, handle, lo, hi) = probe();
                let stream = json(&entity, &handle, hi, lo, r#""phase":"stream""#);
                let gesture = $crate::Gesture::parse(&stream).expect("gesture json");
                let resolved = $crate::gesture_handles_uncached::<$app>(&doc, &gesture);
                assert_eq!($crate::gesture_handles::<$app>(&doc, &gesture), resolved, "the first tick resolves the handles");
                assert_eq!($crate::gesture_handles::<$app>(&doc, &gesture), resolved, "a further tick of the same drag is answered from the retained entry unchanged");
                let transient = transient_after(&doc, &entity, &$crate::NormTransient::default(), &stream);
                let moved = <Doc as semio_framework_value::FromValue>::from_value($crate::overlay_document(&doc.to_value(), &transient).expect("overlay")).expect("the preview document reads back");
                assert_eq!($crate::gesture_handles::<$app>(&moved, &gesture), $crate::gesture_handles_uncached::<$app>(&moved, &gesture), "a changed document is never answered from the old entry");
                assert_eq!($crate::gesture_handles::<$app>(&doc, &gesture), resolved, "the committed document resolves to the same handles again");
                let other = $crate::Gesture { handle: format!("{}-other", gesture.handle), ..gesture.clone() };
                assert_eq!($crate::gesture_handles::<$app>(&doc, &other), $crate::gesture_handles_uncached::<$app>(&doc, &other), "another handle id is never answered from the entry of this one");
            }

            #[test]
            fn a_host_and_the_lane_clock_share_the_live_report_by_handle_and_equal_the_family_report() {
                let host = semio_s_plugin_norm_compliance::NormHost::<Family>::live(Doc::default());
                let mut clock = $crate::LaneClock::default();
                clock.store_report("shared", host.shared_report());
                let cached = clock.cached_report("shared").expect("stored");
                assert!(std::sync::Arc::ptr_eq(&cached, &host.shared_report()), "a drag tick between two due reports reads the stored allocation");
                assert_eq!(*cached, *<Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(host.document()), "the shared report is the live report of the document");
            }

            #[test]
            fn the_live_commands_are_declared_with_their_transient_lanes() {
                $crate::retained_disposition_oracle::assert_live_fixture(VARIANT);
                for id in $crate::NORM_LIVE_TOOL_IDS {
                    assert!(<$app as $crate::NormRetainedEditor>::TOOL_IDS.contains(id), "{id} is a tool id of the editor");
                }
            }

            #[test]
            fn a_click_on_a_check_row_routes_to_the_selection_of_its_owner_entity() {
                use semio_s_plugin_norm_structural_scene::{projection::ViewOptions, projection_kit::base_id};
                let doc = Doc::default();
                let report = <Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&doc);
                let scene = <$app as $crate::NormLiveEditor>::live_scene(&doc, ViewOptions::none());
                let index = report.checks.iter().position(|check| !check.subject.entity_id.is_empty() && scene.entities.iter().any(|drawn| base_id(&drawn.entity.id) == base_id(&check.subject.entity_id))).expect("the default document has a check on a drawn entity");
                let mut args = Some(semio_framework_value::DslValue::object(vec![("index".to_string(), semio_framework_value::DslValue::float(index as f64))]));
                let command = <$app as semio_plugin::ArtifactEditor>::command_from_action("setSelectedCheckIndex", &mut args, &mut None).expect("the action of the row decodes");
                let effects = <$app as $crate::NormRetainedEditor>::check_focus_effects(&command, &doc);
                let [semio_plugin::Effect::ReplayShellCommand { action_id, args: Some(args) }] = effects.as_slice() else { panic!("one interactionSelect replay, got {effects:?}") };
                assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
                let text = |key: &str| args.get(key).and_then(semio_framework_value::DslValue::as_str).map(str::to_owned).unwrap_or_default();
                assert_eq!((text("domainId"), text("merge")), ($crate::interaction_domain_id(VARIANT), "replace".to_owned()));
                assert!(text("targets").contains(base_id(&report.checks[index].subject.entity_id)), "the selection names the owner: {}", text("targets"));
            }

            #[test]
            fn the_interaction_topology_names_every_entity_of_the_scene_and_every_check() {
                use semio_s_plugin_norm_structural_scene::projection::ViewOptions;
                let doc = Doc::default();
                let domain = $crate::interaction_domain_id(VARIANT);
                let topology = $crate::live_interaction_topology::<$app>(&doc, &domain).expect("the default document admits a topology");
                let known: std::collections::HashSet<&str> = topology.domains[&domain].ordered.iter().map(|node| node.id.as_str()).collect();
                let scene = <$app as $crate::NormLiveEditor>::live_scene(&doc, ViewOptions::none());
                assert!(!scene.entities.is_empty() && scene.entities.iter().all(|drawn| $crate::viewport3d::picking::entity_address(drawn.entity.kind, &drawn.entity.id).is_some_and(|address| known.contains(address.as_str()))), "every drawn entity is a node");
                let report = <Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&doc);
                assert!(report.checks.iter().all(|check| $crate::check_address(&check.id).is_some_and(|address| known.contains(address.as_str()))), "every check is a node");
            }

            #[test]
            fn every_editor_action_is_reachable_by_touch_a_list_a_long_press_menu_and_nudge_buttons_in_both_languages() {
                use $crate::device_profiles::{device_profile, PRIORITY};
                use $crate::direct_manipulation::{nudge, writes_for, Modifiers};
                use $crate::viewport3d::config::DeviceClass;
                use semio_plugin::{WindowLayoutChild, WindowLayoutRoot};
                use semio_ui_locale::Locale;
                use semio_framework_value::{DslValue, FromValue};
                use semio_s_plugin_norm_structural_scene::EntityRef;
                fn kinds(child: &WindowLayoutChild, out: &mut Vec<String>) {
                    match child {
                        WindowLayoutChild::Axis(axis) => axis.children.iter().for_each(|inner| kinds(inner, out)),
                        WindowLayoutChild::Stack(stack) => out.extend(stack.children.iter().map(|window| window.window_kind_id.clone())),
                    }
                }
                for device in [DeviceClass::Mobile, DeviceClass::Tablet] {
                    let mut windows = Vec::new();
                    match $crate::visual_layout(VARIANT, device).root {
                        WindowLayoutRoot::Axis(axis) => axis.children.iter().for_each(|child| kinds(child, &mut windows)),
                        WindowLayoutRoot::Stack(stack) => windows.extend(stack.children.iter().map(|window| window.window_kind_id.clone())),
                    }
                    for role in ["outliner", "viewport", "properties", "checks", "diagrams"] {
                        assert!(windows.contains(&$crate::visual_window_id(VARIANT, role)), "{device:?}: the {role} window is a tab of the touch layout, so the list selects and the Properties tab edits");
                    }
                    assert!(device_profile(device).nudge_buttons && $crate::touch_profile_of(device).long_press, "{device:?}: nudge buttons and the long press are on");
                }
                assert!(!device_profile(DeviceClass::Desktop).nudge_buttons && PRIORITY.iter().all(|class| $crate::touch_profile_of(*class).hit_px == device_profile(*class).min_target_px));
                assert!($crate::touch_profile_of(DeviceClass::Mobile).one_finger_orbit && !$crate::touch_profile_of(DeviceClass::Tablet).one_finger_orbit, "only tap-to-nudge devices orbit with one finger");
                let doc = Doc::default();
                let tree = doc.to_value();
                let entities = $entities(&doc);
                let first = entities.first().expect("the default document offers an entity");
                let (collection, ids) = $crate::selected_rows(&tree, std::slice::from_ref(&first.id)).expect("the first entity is a row of the document");
                let context = $crate::HostContext { collection: collection.clone(), ids: ids.clone(), clip_json: Some("{}".to_owned()) };
                let menu = $crate::touch_menu(ids.len(), true);
                assert!(menu.iter().all(|entry| entry.enabled), "with a selection and a clip every entry of the Edit menu is on");
                let select_all = $crate::accessibility::focus_order::shortcut_action_id(&$crate::accessibility::focus_order::ShortcutAction::SelectAll);
                for entry in menu.iter().filter(|entry| entry.action_id != select_all) {
                    let (tool, args) = $crate::host_entity_call(&entry.action_id, &context).unwrap_or_else(|| panic!("{} is an entity route", entry.action_id)).unwrap_or_else(|message| panic!("{}: {message}", entry.action_id));
                    <$app as semio_plugin::ArtifactEditor>::command_from_action(tool, &mut Some(args), &mut None).unwrap_or_else(|fault| panic!("the editor refuses {tool} from the Edit menu: {fault:?}"));
                    assert!(!entry.label.en.is_empty() && !entry.label.de.is_empty() && entry.label.en != entry.label.de, "{}: labelled in both languages", entry.action_id);
                }
                for locale in [Locale::En, Locale::De] {
                    let node = $crate::render_touch_menu(&menu, "norm.law", locale).expect("the Edit menu renders");
                    let projected = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(node)).expect("the Edit menu projects");
                    assert!(projected.contains("selectAll") && projected.contains("norm.shortcut.delete") && projected.contains("norm.shortcut.copy"), "{locale:?}: select all raises the framework verb and copy and delete are action items of the menu: {projected}");
                }
                let mut cases: Vec<(Doc, EntityRef, String)> = entities.into_iter().map(|entity| (doc.clone(), entity, "default document".to_owned())).collect();
                for spec in <$app as $crate::NormLiveEditor>::live_palette() {
                    let mut grown = doc.to_value();
                    if grown.get(&spec.collection).and_then(DslValue::as_array).is_none_or(|rows| rows.is_empty()) {
                        let insert = $crate::direct_manipulation::insert_edit(&grown, &spec, [0.0, 0.0, 0.0]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                        $crate::entity_editing::apply_to_tree(&mut grown, &[insert]).unwrap_or_else(|error| panic!("kind '{}': {error}", spec.id));
                    }
                    let Some(id) = grown.get(&spec.collection).and_then(DslValue::as_array).and_then(|list| list.first()).and_then($crate::object_id_field).map(str::to_owned) else { continue };
                    let loaded = <Doc as FromValue>::from_value(grown).unwrap_or_else(|error| panic!("kind '{}': the document with a first row does not load: {error:?}", spec.id));
                    let path = format!("{}[id={id}]", spec.collection);
                    cases.push((loaded, EntityRef::new(spec.kind, id, path), format!("kind '{}'", spec.id)));
                }
                let (mut grips, mut nudged) = (0, 0);
                for (case_doc, entity, what) in &cases {
                    let handles = <$app as $crate::NormLiveEditor>::live_handles(case_doc, std::slice::from_ref(entity));
                    if handles.is_empty() {
                        continue;
                    }
                    let view = $crate::viewport3d::toolbar::ViewportView { lanes: $crate::viewport3d::lane_overlays::LaneOverlays { handles: handles.clone(), ..Default::default() }, ..Default::default() };
                    for locale in [Locale::En, Locale::De] {
                        let controls = $crate::viewport3d::toolbar::render_touch_controls(&view, Some(DeviceClass::Mobile), "norm.law", locale, "norm.law").expect("the touch controls render").unwrap_or_else(|| panic!("{what}: a phone offers nudge buttons for the selected grips"));
                        let projected = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(controls)).expect("the touch controls project");
                        for handle in &handles {
                            assert!(projected.contains(&format!("norm-viewport.nudge.{}.minus", handle.id)) && projected.contains(&format!("norm-viewport.nudge.{}.plus", handle.id)), "{what}: grip '{}' has a minus and a plus button in {locale:?}", handle.id);
                        }
                    }
                    assert!($crate::viewport3d::toolbar::render_touch_controls(&view, Some(DeviceClass::Desktop), "norm.law", Locale::En, "norm.law").expect("renders").is_none(), "{what}: the desktop keeps the keyboard and the pointer");
                    for handle in &handles {
                        grips += 1;
                        let rest = $crate::accessibility::published_nodes::rest_parameter(handle);
                        let reached = [-1.0, 1.0].into_iter().any(|direction| {
                            let to = nudge(handle, rest, direction, Modifiers::NONE).s;
                            if (to - rest).abs() <= f64::EPSILON {
                                return false;
                            }
                            let gesture = $crate::Gesture::parse(&json(entity, handle, to, rest, r#""phase":"commit","ctrl":true"#)).expect("the nudge button gesture parses");
                            let committed = $crate::commit_writes(&handles, &gesture);
                            assert!(committed.is_some() || writes_for(handle, to) == writes_for(handle, rest), "{what}: grip '{}' nudged to {to} commits one setFields", handle.id);
                            committed.is_some_and(|writes| writes == writes_for(handle, to))
                        });
                        let span = handle.limits.as_ref().map_or(f64::INFINITY, |limits| limits.max - limits.min);
                        assert!(reached || span <= 0.0, "{what}: no nudge button changes grip '{}' from its rest value", handle.id);
                        nudged += usize::from(reached);
                    }
                }
                assert!(grips > 0 && nudged > 0, "the editor offers grips that the nudge buttons move ({grips} grips, {nudged} moved)");
            }
        }
    };
}
