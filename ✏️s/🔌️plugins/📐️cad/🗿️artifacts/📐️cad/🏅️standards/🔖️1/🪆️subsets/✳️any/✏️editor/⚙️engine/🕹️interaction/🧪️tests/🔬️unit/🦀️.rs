use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::Brep;

fn point_arg(x: f64, y: f64, z: f64) -> DslValue {
    DslValue::object([("point".to_string(), vec3_json([x, y, z]))])
}

fn value_arg(value: f64) -> DslValue {
    DslValue::object([("value".to_string(), DslValue::float(value))])
}

#[semio_framework_async_macros::async_test]
async fn catalog_lists_every_model_definition_asset() {
    assert!(interaction_by_id("primitive.box").is_some());
    assert!(interaction_by_id("solid.sphere").is_some());
    assert!(interaction_by_id("energy.energy.constructExternalWall").is_some());
    assert!(interaction_by_id("structure.structure.constructReinforcedConcreteColumn").is_some());
    assert!(interaction_by_id("building.building.placeWall").is_some());
    assert_eq!(list_interactions_for_model_definition("spatial.shape").len(), 37);
    assert_eq!(list_interactions_for_model_definition("aec.building").len(), 11);
    assert_eq!(catalog().len(), RAW_INTERACTION_ASSETS.len(), "every embedded asset parses into a catalog entry");
}

/// 📌️ `RAW_INTERACTION_ASSETS` is a hand-maintained `include_str!` list — pin it against the
/// on-disk `🕹️interactions` folders so a new or renamed asset cannot silently vanish from the app.
#[semio_framework_async_macros::async_test]
async fn every_interaction_asset_on_disk_is_embedded() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions");
    let mut on_disk = Vec::new();
    for model_definition in std::fs::read_dir(&root).expect("model definitions root") {
        let interactions = model_definition.expect("entry").path().join("🕹️interactions");
        let Ok(files) = std::fs::read_dir(&interactions) else {
            continue;
        };
        for file in files {
            let path = file.expect("file").path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).expect("asset");
            let spec = parse_interaction_spec(&raw).unwrap_or_else(|| panic!("{} parses as an interaction spec", path.display()));
            on_disk.push(spec.id);
        }
    }
    on_disk.sort_unstable();
    let mut embedded: Vec<String> = catalog().iter().map(|entry| entry.id.clone()).collect();
    embedded.sort_unstable();
    assert!(!on_disk.is_empty(), "no interaction assets under {}", root.display());
    assert_eq!(embedded, on_disk, "embedded catalog ids match the on-disk 🕹️interactions assets");
}

#[semio_framework_async_macros::async_test]
async fn box_interaction_commits_after_height() {
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "mode.diagonal", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([2.0, 3.0, 0.0]))));
    assert!(apply_event(&mut session, "set.height", Some(&DslValue::float(2.5))));
    assert!(apply_event(&mut session, "confirm", None));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    assert!(object.is_some());
    assert_eq!(object.unwrap().typology, "spatial.shape.primitive.box");
}

#[semio_framework_async_macros::async_test]
async fn box_interaction_default_mode_is_point_and_requires_length_prompt() {
    // 📦️box.json's default `boxMode` (set by the `start` transition) is "point", not "diagonal" —
    // a plain pointer.down after start does NOT reach diagonal_rubber.
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert_eq!(session.state, "first_corner_other_or_length");
}

#[semio_framework_async_macros::async_test]
async fn sphere_interaction_commits_via_command_finish() {
    let mut session = start_session("solid.sphere", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(2.0, 0.0, 0.0))));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    let object = object.expect("sphere commits");
    assert_eq!(object.typology, "spatial.shape.solid.sphere");
    assert_eq!(object.origin, [0.0, 0.0, 0.0]);
    assert_eq!(object.extent, Some([4.0, 4.0, 4.0]));
}

#[semio_framework_async_macros::async_test]
async fn external_wall_interaction_commits_via_generic_from_2_points_and_height() {
    let mut session = start_session("energy.energy.constructExternalWall", CadPaneId::Energy).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(4.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "set.height", Some(&value_arg(3.0))));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    let object = object.expect("wall commits");
    assert_eq!(object.typology, "energy.energy.externalwall");
}

#[semio_framework_async_macros::async_test]
async fn reinforced_concrete_column_interaction_commits_as_cylinder() {
    let mut session = start_session("structure.structure.constructReinforcedConcreteColumn", CadPaneId::StructureClassic).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(1.0, 1.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(1.5, 1.0, 0.0))));
    assert!(apply_event(&mut session, "set.height", Some(&value_arg(3.0))));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    let object = object.expect("column commits");
    assert_eq!(object.typology, "structure.structure.reinforcedconcretecolumn");
    assert_eq!(object.origin, [1.0, 1.0, 0.0]);
}

#[semio_framework_async_macros::async_test]
async fn slab_interaction_commits() {
    let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(4.0, 5.0, 0.0))));
    assert!(apply_event(&mut session, "set.height", Some(&value_arg(0.3))));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    assert!(object.is_some());
}

#[semio_framework_async_macros::async_test]
async fn slab_preview_shows_footprint_point() {
    let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    let items = preview_display_items(&session);
    assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
}

#[semio_framework_async_macros::async_test]
async fn building_column_preview_follows_the_asset_display_block() {
    let mut session = start_session("building.building.placeColumn", CadPaneId::Building).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.move", Some(&point_arg(0.5, 0.5, 0.0))));
    let items = preview_display_items(&session);
    assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")), "first_point paints the cursor: {items:?}");
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(1.0, 2.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.move", Some(&point_arg(3.0, 4.0, 0.0))));
    let items = preview_display_items(&session);
    let kinds: Vec<&str> = items.iter().filter_map(|item| item.get("kind").and_then(|value| value.as_str())).collect();
    assert!(kinds.contains(&"point") && kinds.contains(&"segment") && kinds.contains(&"box-preview"), "second_point rubber-bands the footprint: {kinds:?}");
    let anchor = items.iter().find(|item| item.get("id").and_then(|value| value.as_str()) == Some("anchor-a")).expect("anchor");
    assert_eq!(anchor.get("position").and_then(parse_vec3), Some([1.0, 2.0, 0.0]));
    assert_eq!(state_prompt(&session), "Place Column: click opposite footprint corner");
}

#[semio_framework_async_macros::async_test]
async fn building_wall_asset_commits_a_wall_from_two_picks() {
    let mut session = start_session("building.building.placeWall", CadPaneId::Building).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(!can_commit(&session));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(4.0, 0.0, 0.0))));
    assert!(can_commit(&session), "`committed` is the asset's final state");
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1")).expect("wall");
    assert_eq!(object.typology, "building.building.wall");
    assert_eq!(object.origin, [0.0, 0.0, 0.0]);
    assert_eq!(object.extent, Some([4.0, 0.2, 2.7]), "span × wall thickness × the asset's 2.7 m height");
    let items = preview_display_items(&session);
    assert!(items.iter().any(|item| item.get("id").and_then(|value| value.as_str()) == Some("preview-final")));
}

#[semio_framework_async_macros::async_test]
async fn building_beam_asset_commits_a_bar_between_two_picks() {
    let mut session = start_session("building.building.placeBeam", CadPaneId::Building).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 3.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 5.0, 3.0))));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1")).expect("beam");
    assert_eq!(object.typology, "building.building.beam");
    assert_eq!(object.extent, Some([5.0, 0.3, 0.3]), "bar length along its own x, then the beam's 0.3 m section");
}

#[semio_framework_async_macros::async_test]
async fn parse_repl_line_accepts_raw_forms() {
    assert_eq!(parse_repl_line("set.height 2.5", None), Some(("set.height".into(), Some(DslValue::float(2.5)))));
    assert_eq!(parse_repl_line("dist 12", None), Some(("set.distance".into(), Some(DslValue::float(12.0)))));
}

#[semio_framework_async_macros::async_test]
async fn parse_repl_line_accepts_shell_normalized_forms() {
    // The React shell PascalCases every draft (framework/renderer/react `normalizeEngagementCommandText`),
    // so `set.height 3.5` arrives as `SetHeight3.5` with no separators.
    assert_eq!(parse_repl_line("SetHeight3.5", None), Some(("set.height".into(), Some(DslValue::float(3.5)))));
    assert_eq!(parse_repl_line("setheight0.25", None), Some(("set.height".into(), Some(DslValue::float(0.25)))));
    assert_eq!(parse_repl_line("Dist12.75", None), Some(("set.distance".into(), Some(DslValue::float(12.75)))));
}

#[semio_framework_async_macros::async_test]
async fn parse_repl_line_commits_bare_number_only_in_numeric_entry_state() {
    // Bare numeric entry (premigration `tryCommitNumericEntry`) only applies while a
    // numeric-entry state (e.g. box's first_corner_height) is active.
    assert_eq!(parse_repl_line("3.5", Some("first_corner_height")), Some(("set.height".into(), Some(DslValue::float(3.5)))));
    assert_eq!(parse_repl_line("2", Some("column_height")), Some(("set.height".into(), Some(DslValue::float(2.0)))));
    // Outside a numeric-entry state, a bare number is treated as an (unresolvable) interaction key.
    assert_eq!(parse_repl_line("3.5", None), Some(("3.5".into(), None)));
    assert_eq!(parse_repl_line("3.5", Some("idle")), Some(("3.5".into(), None)));
}

#[semio_framework_async_macros::async_test]
async fn box_interaction_commits_via_shell_normalized_repl_line() {
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "mode.diagonal", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([2.0, 3.0, 0.0]))));
    let (event_kind, payload) = parse_repl_line("SetHeight2.5", Some(&session.state)).expect("parsed line");
    assert!(apply_event(&mut session, &event_kind, payload.as_ref()));
    assert!(apply_event(&mut session, "confirm", None));
    assert!(can_commit(&session));
}

//#region 🔖️EveryInteraction
/// 🤖️ Drives one JSON-driven session to its commit state the way the shell would: `start`, then per
/// state the first applicable handler — a ground pick (`pointer.down`, walking a diagonal so no two
/// picks coincide), a scalar entry (`set.*`), the injected selection, or a keyed `confirm`/`close`.
fn drive_to_commit(session: &mut CadEngagementScratch, selection: &[String]) -> bool {
    // 🏁️ Most machines open with an explicit `start`; the selection-first ones (measures, anchors)
    // begin in their selecting state and simply ignore it.
    let _ = apply_event(session, "start", None);
    // 📦️ The box opens in its length-prompt `point` mode; the diagonal mode is the pick-driven one.
    let _ = apply_event(session, "mode.diagonal", None);
    let mut picks = 0usize;
    let mut selected_in: Option<String> = None;
    let mut scalar_in: Option<String> = None;
    for _ in 0..24 {
        if can_commit(session) {
            return true;
        }
        let spec = spec_by_id(&session.interaction_id).expect("spec");
        let state = spec.state(&session.state).expect("state");
        let events: Vec<&str> = state.on.iter().map(|handler| handler.event.as_str()).collect();
        let next_point = {
            let step = (picks + 1) as f64;
            vec3_json([step * 1.5, step, 0.0])
        };
        let applied = if events.contains(&"selection.changed") && selected_in.as_deref() != Some(session.state.as_str()) {
            selected_in = Some(session.state.clone());
            inject_selection(session, selection)
        } else if events.contains(&"selection.changed") && events.contains(&"confirm") {
            apply_event(session, "confirm", None)
        } else if events.contains(&"selection.changed") && events.contains(&"contextmenu") {
            apply_event(session, "contextmenu", None)
        } else if let Some(event) = events.iter().find(|event| event.starts_with("set.")).filter(|_| (picks >= 2 || !events.contains(&"pointer.down")) && scalar_in.as_deref() != Some(session.state.as_str())) {
            scalar_in = Some(session.state.clone());
            apply_event(session, event, Some(&DslValue::float(2.0)))
        } else if scalar_in.as_deref() == Some(session.state.as_str()) && events.contains(&"confirm") {
            apply_event(session, "confirm", None)
        } else if picks >= 3 && events.iter().any(|event| ["confirm", "close", "contextmenu"].contains(event)) {
            // 📈️ Open-ended point lists (polylines, curves) close after three picks.
            let event = ["close", "confirm", "contextmenu"].into_iter().find(|event| events.contains(event)).unwrap();
            apply_event(session, event, None)
        } else if events.contains(&"pointer.down") {
            picks += 1;
            apply_event(session, "pointer.down", Some(&next_point))
        } else if events.contains(&"confirm") {
            apply_event(session, "confirm", None)
        } else if events.contains(&"close") {
            apply_event(session, "close", None)
        } else if events.contains(&"contextmenu") {
            apply_event(session, "contextmenu", None)
        } else if let Some(keyed) = keyed_transitions(session).into_iter().next() {
            apply_event(session, &keyed.event_kind, None)
        } else {
            false
        };
        if !applied {
            return false;
        }
    }
    can_commit(session)
}

/// 🧾️ Every model-definition interaction asset drives to its commit state through the generic
/// interpreter, and its commit family resolves to a document outcome — objects, a transform on the
/// injected selection, or the declared `Unsupported` families whose persisted shape does not exist yet.
#[semio_framework_async_macros::async_test]
async fn every_model_definition_interaction_drives_to_a_commit_outcome() {
    let mut reached = 0;
    let mut objects = Vec::new();
    let mut transforms = Vec::new();
    let mut unsupported = Vec::new();
    let mut stuck = Vec::new();
    let selection = vec!["object-a".to_string(), "object-b".to_string()];
    for entry in catalog() {
        let pane = crate::cad_pane_from_model_definition_id(&entry.model_definition_id).unwrap_or(CadPaneId::Shape);
        let mut session = start_session(&entry.id, pane).expect("session");
        if !drive_to_commit(&mut session, &selection) {
            stuck.push(format!("{}@{}", entry.id, session.state));
            continue;
        }
        reached += 1;
        let mut kernel = Brep::new();
        match commit_session(&mut kernel, &session, 0, |prefix| format!("{prefix}-test")) {
            Some(CommitOutcome::Objects(built)) => {
                assert!(!built.is_empty(), "{}", entry.id);
                assert!(built.iter().all(|object| object.extent.is_some() && object.solid_handle.is_some()), "{} objects persist an extent and seed a kernel solid", entry.id);
                objects.push(entry.id.clone());
            }
            Some(CommitOutcome::Move { targets, .. }) | Some(CommitOutcome::Copy { targets, .. }) | Some(CommitOutcome::Rotate { targets, .. }) | Some(CommitOutcome::Scale { targets, .. }) => {
                assert_eq!(targets, selection, "{} acts on the injected selection", entry.id);
                transforms.push(entry.id.clone());
            }
            Some(CommitOutcome::Unsupported(action)) => unsupported.push(format!("{} ({action})", entry.id)),
            None => stuck.push(format!("{}@commit", entry.id)),
        }
    }
    eprintln!("[DEBUG] interactions reached={reached} objects={} transforms={} unsupported={} stuck={stuck:?}\n[DEBUG] unsupported={unsupported:?}", objects.len(), transforms.len(), unsupported.len());
    // 🧩️ Three mode-chooser assets delegate to a nested `interaction.call` (composition the
    // interpreter does not run yet); `measure.area` gates on a `kernel.query` effect and
    // `surface.extrudeCrv` on `command.assignExtrusionDistance` over an existing curve — exact, so a
    // newly runnable one is celebrated here.
    let mut stuck_ids: Vec<&str> = stuck.iter().map(|row| row.split('@').next().unwrap()).collect();
    stuck_ids.sort_unstable();
    assert_eq!(stuck_ids, vec!["curve.construct", "energy.energy.constructBasePlate", "measure.area", "surface.construct", "surface.extrudeCrv"], "every other interaction reaches its commit state: {stuck:?}");
    assert_eq!(reached, 57, "{objects:?} {transforms:?}");
    assert_eq!(objects.len(), 32, "object-producing interactions: {objects:?}");
    assert_eq!(transforms.len(), 6, "selection transforms: {transforms:?}");
    // 🚧️ The families whose results the persisted `CadObjectSpec` cannot carry (topology edits,
    // booleans, lofts/sweeps/extrusions on existing curves, measures, anchors).
    let expected_unsupported = [
        "edit.split", "edit.trim", "solid.booleanIntersection", "solid.booleanUnion", "solid.booleanDifference", "surface.sweep1", "surface.sweep2", "edit.fillet", "surface.loft", "edit.explode", "measure.distance",
        "edit.join", "surface.networkSrf", "feature.offsetSurface", "feature.extrudeWire", "edit.chamfer", "entity.createAnchor",
    ];
    let mut unsupported_ids: Vec<&str> = unsupported.iter().map(|row| row.split(' ').next().unwrap()).collect();
    unsupported_ids.sort_unstable();
    let mut expected: Vec<&str> = expected_unsupported.to_vec();
    expected.sort_unstable();
    assert_eq!(unsupported_ids, expected);
}

#[semio_framework_async_macros::async_test]
async fn polyline_commits_one_bar_per_segment_and_closes() {
    let mut session = start_session("curve.polyline", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    for point in [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [4.0, 3.0, 0.0]] {
        assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json(point))));
    }
    assert!(apply_event(&mut session, "close", None));
    let mut kernel = Brep::new();
    let Some(CommitOutcome::Objects(objects)) = commit_session(&mut kernel, &session, 0, |prefix| format!("{prefix}-{}", next_test_id())) else { panic!("polyline commits objects") };
    assert_eq!(objects.len(), 3, "two authored segments plus the closing one");
    assert!((objects[0].extent.unwrap()[0] - 4.0).abs() < 1e-9);
    assert!((objects[2].extent.unwrap()[0] - 5.0).abs() < 1e-9, "closing segment spans the hypotenuse");
    assert_eq!(objects[1].origin, [4.0, 0.0, 0.0]);
    let q = objects[1].orientation.unwrap();
    assert!((q[2] - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-9 && (q[3] - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-9, "a +Y segment is a 90° yaw: {q:?}");
}

fn next_test_id() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[semio_framework_async_macros::async_test]
async fn move_interaction_commits_the_selection_delta() {
    let mut session = start_session("transform.move", CadPaneId::Building).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(!apply_event(&mut session, "confirm", None), "confirm is guarded on a non-empty selection");
    assert!(inject_selection(&mut session, &["bim-1".to_string()]));
    assert!(apply_event(&mut session, "confirm", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([1.0, 1.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([3.0, 2.0, 0.0]))));
    let mut kernel = Brep::new();
    assert_eq!(commit_session(&mut kernel, &session, 0, |prefix| prefix.to_string()), Some(CommitOutcome::Move { targets: vec!["bim-1".into()], delta: [2.0, 1.0, 0.0] }));
}

#[semio_framework_async_macros::async_test]
async fn numeric_entry_follows_the_state_declared_scalar_event() {
    let mut session = start_session("curve.circle", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert_eq!(numeric_entry_event(&session).as_deref(), Some("set.radius"));
    assert_eq!(parse_repl_line_for("2.5", Some(&session)), Some(("set.radius".to_string(), Some(DslValue::float(2.5)))));
    assert_eq!(parse_repl_line("SetRadius2.5", None), Some(("set.radius".to_string(), Some(DslValue::float(2.5)))));
    assert_eq!(state_prompt(&session), "Radius", "the display prompt label, not the raw state name");
    assert!(apply_event(&mut session, "set.radius", Some(&DslValue::float(2.5))));
    let mut kernel = Brep::new();
    let Some(CommitOutcome::Objects(objects)) = commit_session(&mut kernel, &session, 0, |prefix| format!("{prefix}-{}", next_test_id())) else { panic!("circle commits") };
    assert_eq!(objects.len(), 32, "a circle is its 32-segment polygon");
}
//#endregion 🔖️EveryInteraction

#[semio_framework_async_macros::async_test]
async fn final_state_satisfies_a_dangling_commit_from_state() {
    let mut session = start_session("surface.plane", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([2.0, 3.0, 0.0]))));
    let spec = spec_by_id("surface.plane").unwrap();
    let state = spec.state(&session.state).unwrap();
    assert!(state.r#final, "the wire `final` flag decodes onto the raw-identifier field");
    assert!(state.on.is_empty());
    assert!(spec.state("ready").is_none(), "the asset's `fromStates` names a state its machine never defines");
    assert!(can_commit(&session), "a final state satisfies a dangling from-state");
}


/// 🔤️ The persisted session JSON is canonical — `snapshot_of` compares it byte-for-byte against the
/// checkpointed twin, so two encodes of one multi-field context must agree regardless of `HashMap`
/// iteration order.
#[semio_framework_async_macros::async_test]
async fn engagement_session_json_is_canonical_across_encodes() {
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.move", Some(&vec3_json([1.0, 2.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([1.0, 2.0, 0.0]))));
    assert!(session.context.len() >= 2, "the box session carries several context fields: {:?}", session.context.keys().collect::<Vec<_>>());
    let first = protocol::json::to_json_string(&session);
    for _ in 0..8 {
        let decoded: CadEngagementScratch = protocol::json::from_json_str(&first).expect("scratch");
        let again = protocol::json::to_json_string(&decoded);
        assert_eq!(again, first, "re-encoding a decoded session must reproduce the same bytes");
    }
}
