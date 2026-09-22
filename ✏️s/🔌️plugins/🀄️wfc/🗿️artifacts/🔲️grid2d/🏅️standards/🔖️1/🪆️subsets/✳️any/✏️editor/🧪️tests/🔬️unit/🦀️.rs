//! 🧪️ Editor laws — the manifest shape, the action roster, the typed command channel, and the
//! per-pane dispatch every command owes its own window.

use super::*;

#[test]
fn the_editor_builds_a_definition_for_the_editor_role() {
    let definition = create_grid2d_editor();
    assert_eq!(definition.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(definition.dialect, WFC_GRID2D_DIALECT.into());
}

#[test]
fn the_editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Grid2dEditor as ArtifactEditor>::DIALECT, WFC_GRID2D_DIALECT);
    assert_eq!(<Grid2dEditor as ArtifactEditor>::DOCUMENT_SCHEMA, WFC_GRID2D_DOCUMENT_SCHEMA);
}

#[test]
fn the_editor_declares_both_panes_and_a_fifty_fifty_row_layout() {
    let definition = create_grid2d_editor();
    assert!(definition.window_kinds.iter().any(|window| window.id == grid::WINDOW_KIND_ID));
    assert!(definition.window_kinds.iter().any(|window| window.id == preview::WINDOW_KIND_ID));
    let layout = dsl::json::to_json_string(&edit::layout());
    assert!(layout.contains(grid::WINDOW_KIND_ID) && layout.contains(preview::WINDOW_KIND_ID), "{layout}");
    assert!(layout.contains("50"), "the two panes share the row evenly: {layout}");
}

#[test]
fn both_panes_are_localized_en_and_de() {
    let definition = create_grid2d_editor();
    let grid_window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    assert_eq!(grid_window.label, semio_framework_plugin::LocalizedLabel::native("Grid", "Raster"));
    let preview_window = definition.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("preview pane");
    assert_eq!(preview_window.label, semio_framework_plugin::LocalizedLabel::native("Preview", "Vorschau"));
}

#[test]
fn the_grid_pane_declares_every_mutation_action_as_a_migrated_verb() {
    let definition = create_grid2d_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    for id in crate::mutations::KINDS {
        assert!(window.actions.iter().any(|action| &action.id == id), "{id} is not clickable in the grid pane");
    }
    for action in &window.actions {
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{} would be hard-dead in the interactive app", action.id);
    }
}

#[test]
fn the_preview_pane_owns_the_solve_verb() {
    let definition = create_grid2d_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("preview pane");
    assert!(window.actions.iter().any(|action| action.id == "solve"));
}

#[test]
fn the_grid_pane_arms_exactly_the_three_declared_utilities() {
    let definition = create_grid2d_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    let armed: Vec<&str> = window.utilities.iter().map(semio_framework_plugin::UtilityRef::as_str).collect();
    for utility in [grid::UTILITY_SELECT, grid::UTILITY_PIN, grid::UTILITY_MASK] {
        assert!(armed.contains(&utility), "{utility} is not armed: {armed:?}");
    }
    assert_eq!(edit::utilities().len(), 3);
}

/// 🎮️ Every verb THIS editor authors resolves to a typed command. Framework-reserved rows the
/// builder injects (history/clipboard/revert/filter) are deliberately excluded: they are dispatched
/// by the shell, never through an app's own command channel.
#[test]
fn every_authored_action_id_maps_back_to_a_typed_command() {
    const AUTHORED: &[&str] = &[
        "pick-cell",
        "set-active-tile",
        "set-camera",
        "set-grid-visible",
        "set-grid-snap-enabled",
        "set-grid-factor",
        "solve",
        "commit-fill",
        "setActiveExample",
        "canvasPointerDown",
        "canvasPointerMove",
        "canvasPointerUp",
        "canvasDoubleClick",
        "setCamera",
    ];
    let definition = create_grid2d_editor();
    let declared: Vec<String> = definition.window_kinds.iter().flat_map(|window| window.actions.iter().map(|action| action.id.clone())).collect();
    for id in crate::mutations::KINDS.iter().copied().chain(AUTHORED.iter().copied()) {
        assert!(declared.iter().any(|declared| declared == id), "{id} is declared by no pane");
        <Grid2dEditor as ArtifactEditor>::command_from_action(id, None).unwrap_or_else(|error| panic!("{id}: {error:?}"));
    }
}

/// 🎯️ The proof catalog, the wire's tool-job ids and the command roster are ONE declaration: a
/// verb missing from `TOOL_JOB_IDS` is refused `interactive-job.missing-factory` at every UI
/// dispatch, and a verb missing from the window manifest is dropped before that.
#[test]
fn every_tool_job_id_is_a_declared_migrated_verb_with_a_command() {
    assert_eq!(GRID2D_TOOL_IDS, <Grid2dEditorCommand as protocol::OpBinary>::TOOL_JOB_IDS);
    let definition = create_grid2d_editor();
    for id in GRID2D_TOOL_IDS {
        let action = definition
            .window_kinds
            .iter()
            .flat_map(|window| window.actions.iter())
            .find(|action| &action.id == id)
            .unwrap_or_else(|| panic!("{id} is a tool-job id no window declares"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{id}");
        let command = <Grid2dEditor as ArtifactEditor>::command_from_action(id, None).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(grid2d_command_id(&command), *id, "the command a verb decodes to must answer that verb's own id");
    }
    for kind in crate::mutations::KINDS {
        assert!(GRID2D_TOOL_IDS.contains(kind), "{kind} has no retained tool proof and would be dispatch-dead");
    }
}

#[test]
fn an_unknown_action_is_refused_rather_than_silently_defaulted() {
    assert!(<Grid2dEditor as ArtifactEditor>::command_from_action("not-a-verb", None).is_err());
}

#[test]
fn the_command_channel_round_trips_through_its_binary_op_form() {
    for command in [
        Grid2dEditorCommand::ChangeSeed { seed: 99 },
        Grid2dEditorCommand::PickCell { x: 2, y: 1 },
        Grid2dEditorCommand::SetCamera { x: 1.0, y: 2.0, zoom: 3.0 },
        Grid2dEditorCommand::Solve,
        Grid2dEditorCommand::CommitFill { solve_json: String::new() },
    ] {
        let bytes = <Grid2dEditorCommand as protocol::OpBinary>::encode_op(&command).expect("encode");
        assert_eq!(<Grid2dEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode"), command);
    }
}

#[test]
fn the_boot_document_is_the_first_bundled_example() {
    assert_eq!(<Grid2dEditor as ArtifactEditor>::initial_snapshot(), crate::examples::grid2d::pipes::document());
}

/// 🗃️ The navbar switcher's verb resolves every bundled id and refuses anything else, so a picker
/// row can never open a blank document.
#[test]
fn the_example_switcher_resolves_exactly_the_bundled_ids() {
    for source in crate::examples::grid2d::sources() {
        assert!(example_document(source.id()).is_some(), "{} is offered by the picker but resolves to no document", source.id());
    }
    assert_eq!(example_document("pipes"), Some(crate::examples::grid2d::pipes::document()));
    assert!(example_document("not-an-example").is_none());
}

/// 🌱️ Switching example replaces the whole document through `Effect::LoadDocument`: no artifact
/// mutation exists for it, so no history patch is journalled and `canUndo` stays exactly where it
/// was — re-picking the boot example is not an edit.
#[test]
fn switching_example_loads_a_document_instead_of_journalling_an_edit() {
    let command = <Grid2dEditor as ArtifactEditor>::command_from_action("setActiveExample", Some(&dsl::DslValue::Object(vec![("exampleId".into(), dsl::DslValue::String("terrain".into()))]))).expect("decode");
    assert_eq!(command, Grid2dEditorCommand::SetActiveExample { example_id: "terrain".into() });
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action("setActiveExample", Some(&dsl::DslValue::Object(vec![("id".into(), dsl::DslValue::String("pipes".into()))]))).expect("decode"),
        Grid2dEditorCommand::SetActiveExample { example_id: "pipes".into() },
        "the navbar's alternative argument spelling decodes to the same command"
    );
    let effect = reset_document_effect(&crate::examples::grid2d::pipes::document());
    assert!(matches!(effect, semio_framework::kernel::Effect::LoadDocument { .. }), "a document swap rides LoadDocument, never a mutation");
}

/// 🖱️ A press on the grid surface runs the armed utility on exactly the cell under the pointer; the
/// `select` utility only picks, and a press outside the authored grid resolves to no cell at all.
#[test]
fn a_canvas_press_on_the_grid_pins_the_cell_under_the_pointer() {
    let document = crate::examples::grid2d::pipes::document();
    let config = Grid2dWindowConfig::default();
    let centre = grid::cell_at(&document, &config, 400.0, 300.0, 800.0, 600.0).expect("the pane centre is a cell");
    assert_eq!(centre, (document.width / 2, document.height / 2));
    let pinned = Grid2dEditor::armed_pick(&document, &config, grid::UTILITY_PIN, centre.0, centre.1).expect("pick").expect("the pin utility edits");
    assert_eq!(pinned.1, format!("Pin cell ({}, {})", centre.0, centre.1));
    let masked = Grid2dEditor::armed_pick(&document, &config, grid::UTILITY_MASK, 5, 5).expect("pick").expect("the mask utility edits");
    assert_eq!(masked.1, "Unmask cell (5, 5)", "an already masked cell toggles back");
    assert!(Grid2dEditor::armed_pick(&document, &config, grid::UTILITY_SELECT, 1, 1).expect("pick").is_none(), "the select utility only picks");
    assert_eq!(grid::cell_at(&document, &config, 1.0, 1.0, 800.0, 600.0), None, "a press beside the grid picks nothing");
    assert!(!grid::owns_surface(preview::SURFACE_ID), "a press on the read-only preview surface never edits");
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action(
            "canvasPointerDown",
            Some(&dsl::DslValue::Object(vec![
                ("surfaceId".into(), dsl::DslValue::String(grid::SURFACE_ID.into())),
                ("x".into(), dsl::DslValue::Number(dsl::Number::Float(400.0))),
                ("y".into(), dsl::DslValue::Number(dsl::Number::Float(300.0))),
                ("width".into(), dsl::DslValue::Number(dsl::Number::Float(800.0))),
                ("height".into(), dsl::DslValue::Number(dsl::Number::Float(600.0))),
            ]))
        )
        .expect("decode"),
        Grid2dEditorCommand::CanvasPointerDown { surface_id: grid::SURFACE_ID.into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 }
    );
}

/// 🎥️ The canvas host syncs its camera as one nested object; the palette form states the scalars
/// flat. They are two commands on purpose — a dispatch is refused when a command's own id is not
/// the action it was admitted under — but they write the same camera.
#[test]
fn both_camera_spellings_decode_to_their_own_command() {
    let nested = dsl::DslValue::Object(vec![(
        "camera".into(),
        dsl::DslValue::Object(vec![("x".into(), dsl::DslValue::Number(dsl::Number::Float(4.0))), ("y".into(), dsl::DslValue::Number(dsl::Number::Float(5.0))), ("zoom".into(), dsl::DslValue::Number(dsl::Number::Float(2.0)))]),
    )]);
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action("setCamera", Some(&nested)).expect("decode"),
        Grid2dEditorCommand::SyncCamera { x: 4.0, y: 5.0, zoom: 2.0 }
    );
    let flat = dsl::DslValue::Object(vec![("x".into(), dsl::DslValue::Number(dsl::Number::Float(4.0))), ("y".into(), dsl::DslValue::Number(dsl::Number::Float(5.0))), ("zoom".into(), dsl::DslValue::Number(dsl::Number::Float(2.0)))]);
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action("set-camera", Some(&flat)).expect("decode"),
        Grid2dEditorCommand::SetCamera { x: 4.0, y: 5.0, zoom: 2.0 }
    );
}

#[test]
fn the_armed_utility_falls_back_through_window_then_focus_then_flat_then_select() {
    let mut view = ViewModel::default();
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_SELECT);
    view.active_utility_id = Some(grid::UTILITY_MASK.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_MASK);
    view.focused_window_id = Some("focused".into());
    view.active_utility_by_window_id.insert("focused".into(), grid::UTILITY_PIN.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_PIN);
    view.window_id = Some("rendered".into());
    view.active_utility_by_window_id.insert("rendered".into(), grid::UTILITY_SELECT.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_SELECT);
}

#[test]
fn the_solve_command_starts_the_fill_run() {
    let document = crate::examples::grid2d::pipes::document();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let emit = Grid2dEditor::dispatch(&Grid2dEditorCommand::Solve, &doc, &cfg, None).expect("solve starts");
    assert!(emit.window_config_mutations.is_empty(), "solve must not write solve_json itself");
    assert!(
        emit.effects.iter().any(|effect| match effect {
            semio_framework::kernel::Effect::DispatchAction { action, args: Some(args), .. } => {
                action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID
                    && matches!(args, dsl::DslValue::Object(entries) if entries.iter().any(|(key, value)| key == semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID && matches!(value, dsl::DslValue::String(text) if text == fill_tool::TOOL_ID)))
            }
            _ => false,
        }),
        "solve must start the fill tool run: {:?}",
        emit.effects
    );
}

#[test]
fn the_commit_fill_command_writes_solve_json() {
    let document = crate::examples::grid2d::pipes::document();
    let commit = crate::schema::inferences::solve_with_job(&document).expect("pipes solves");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let mut view = ViewModel::default();
    view.window_id = Some("preview-1".into());
    view.window_instances = vec![semio_framework::ViewWindowInstance { id: "preview-1".into(), window_kind_id: preview::WINDOW_KIND_ID.into() }];
    let solve_json = protocol::json::to_json_string(&commit);
    let emit = Grid2dEditor::dispatch(&Grid2dEditorCommand::CommitFill { solve_json: solve_json.clone() }, &doc, &cfg, Some(&view)).expect("commit-fill writes");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.description.as_deref(), Some("Commit fill"));
}

/// ⚖️ LAW: `TOOL_IDS`, the per-tool publication-lane contracts and the `bounded_first_step_tool_proofs!`
/// rows are ONE roster. The framework joins all three at app registration and refuses the whole app with
/// `interactive-job.publication-contract` the moment they drift — and that refusal is a guest-side
/// `panic!`, so one missing lane row aborts the entire wfc component and EVERY wfc pane reaches
/// `data-shell-error` instead of `data-shell-ready`. That is exactly how `wfc2d`, `wfc3d` and `grid3d`
/// went red on 2026-09-22 when a new tool reached `TOOL_IDS` and the proofs but not the lane contracts.
#[test]
fn the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let tools: std::collections::BTreeSet<&str> = GRID2D_TOOL_IDS.iter().copied().collect();
    assert_eq!(<Grid2dCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, GRID2D_TOOL_IDS);
    let publication: std::collections::BTreeSet<&str> = <Grid2dCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Grid2dCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
        assert!(
            !contract.lanes.contains(&semio_framework_plugin::ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1,
            "tool {} mixes the HostOnly lane with a publishing lane",
            contract.tool_id
        );
    }
    let proofs: std::collections::BTreeSet<&str> = <Grid2dEditor as ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
    assert_eq!(proofs, tools, "every owned tool carries its owner-local bounded reducer proof");
}
