//! 🧪️ Editor laws — the manifest shape, the action roster, the typed command channel, and the
//! per-pane dispatch every command owes its own window.

use crate::editor::grid2d::modes::edit;
use crate::editor::grid2d::modes::edit::tools::fill as fill_tool;
use crate::{editor::grid2d::modes::edit::windows::grid, editor::grid2d::modes::edit::windows::preview};
use crate::{editor::grid2d::window, editor::grid2d::window::Grid2dWindowConfig};
use crate::{Grid2dMutation, Grid2dSnapshot, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Editor;
use semio_plugin::InteractiveJobClassification;
use semio_plugin::NoConfig;
use semio_plugin::ViewModel;
use crate::editor::grid2d::*;

#[test]
fn the_editor_builds_a_definition_for_the_editor_role() {
    let definition = create_grid2d_editor();
    assert_eq!(definition.role, semio_plugin::AppRole::Editor);
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
    let layout = semio_framework_pack_json::to_json_string(&edit::layout());
    assert!(layout.contains(grid::WINDOW_KIND_ID) && layout.contains(preview::WINDOW_KIND_ID), "{layout}");
    assert!(layout.contains("50"), "the two panes share the row evenly: {layout}");
}

#[test]
fn both_panes_are_localized_en_and_de() {
    let definition = create_grid2d_editor();
    let grid_window = definition.window_kinds.iter().find(|window| window.id == grid::WINDOW_KIND_ID).expect("grid pane");
    assert_eq!(grid_window.label, semio_ui_locale::LocalizedLabel::native("Grid", "Raster"));
    let preview_window = definition.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("preview pane");
    assert_eq!(preview_window.label, semio_ui_locale::LocalizedLabel::native("Preview", "Vorschau"));
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
    let armed: Vec<&str> = window.utilities.iter().map(semio_plugin::UtilityRef::as_str).collect();
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
        <Grid2dEditor as ArtifactEditor>::command_from_action(id, &mut None, &mut None).unwrap_or_else(|error| panic!("{id}: {error:?}"));
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
        let command = <Grid2dEditor as ArtifactEditor>::command_from_action(id, &mut None, &mut None).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(grid2d_command_id(&command), *id, "the command a verb decodes to must answer that verb's own id");
    }
    for kind in crate::mutations::KINDS {
        assert!(GRID2D_TOOL_IDS.contains(kind), "{kind} has no retained tool proof and would be dispatch-dead");
    }
}

#[test]
fn an_unknown_action_is_refused_rather_than_silently_defaulted() {
    assert!(<Grid2dEditor as ArtifactEditor>::command_from_action("not-a-verb", &mut None, &mut None).is_err());
}

#[test]
fn the_command_channel_round_trips_through_its_binary_op_form() {
    for command in [
        Grid2dEditorCommand::ChangeSeed { seed: 99 },
        Grid2dEditorCommand::PickCell { x: 2, y: 1 },
        Grid2dEditorCommand::SetCamera { x: 1.0, y: 2.0, zoom: 3.0 },
        Grid2dEditorCommand::Solve,
        Grid2dEditorCommand::CommitFill { solve_json: String::new(), source_revision: Vec::new() },
    ] {
        let bytes = <Grid2dEditorCommand as protocol::OpBinary>::encode_op(&command).expect("encode");
        assert_eq!(<Grid2dEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("decode"), command);
    }
}

#[test]
fn the_boot_document_is_the_first_bundled_example() {
    assert_eq!(<Grid2dEditor as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births"), crate::examples::grid2d::pipes::document());
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
    let command = <Grid2dEditor as ArtifactEditor>::command_from_action("setActiveExample", &mut Some((semio_framework_value::DslValue::Object(vec![("exampleId".into(), semio_framework_value::DslValue::String("terrain".into()))])).clone()), &mut None).expect("decode");
    assert_eq!(command, Grid2dEditorCommand::SetActiveExample { example_id: "terrain".into() });
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action("setActiveExample", &mut Some((semio_framework_value::DslValue::Object(vec![("id".into(), semio_framework_value::DslValue::String("pipes".into()))])).clone()), &mut None).expect("decode"),
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
    let english = |mutation: &Grid2dMutation| protocol::SemanticMutation::<Grid2dSnapshot>::label(mutation).resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::En).to_string();
    assert!(english(&pinned).starts_with(&format!("Pin cell ({}, {}) to ", centre.0, centre.1)), "{}", english(&pinned));
    let masked = Grid2dEditor::armed_pick(&document, &config, grid::UTILITY_MASK, 5, 5).expect("pick").expect("the mask utility edits");
    assert_eq!(english(&masked), "Unmask cell (5, 5)", "an already masked cell toggles back");
    assert!(Grid2dEditor::armed_pick(&document, &config, grid::UTILITY_SELECT, 1, 1).expect("pick").is_none(), "the select utility only picks");
    assert_eq!(grid::cell_at(&document, &config, 1.0, 1.0, 800.0, 600.0), None, "a press beside the grid picks nothing");
    assert!(!grid::owns_surface(preview::SURFACE_ID), "a press on the read-only preview surface never edits");
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action(
            "canvasPointerDown", &mut Some((semio_framework_value::DslValue::Object(vec![
                ("surfaceId".into(), semio_framework_value::DslValue::String(grid::SURFACE_ID.into())),
                ("x".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(400.0))),
                ("y".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(300.0))),
                ("width".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(800.0))),
                ("height".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(600.0))),
            ])).clone()), &mut None)
        .expect("decode"),
        Grid2dEditorCommand::CanvasPointerDown { surface_id: grid::SURFACE_ID.into(), x: 400.0, y: 300.0, width: 800.0, height: 600.0 }
    );
}

/// 🎥️ The canvas host syncs its camera as one nested object; the palette form states the scalars
/// flat. They are two commands on purpose — a dispatch is refused when a command's own id is not
/// the action it was admitted under — but they write the same camera.
#[test]
fn both_camera_spellings_decode_to_their_own_command() {
    let nested = semio_framework_value::DslValue::Object(vec![(
        "camera".into(),
        semio_framework_value::DslValue::Object(vec![("x".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(4.0))), ("y".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(5.0))), ("zoom".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.0)))]),
    )]);
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action("setCamera", &mut Some((nested).clone()), &mut None).expect("decode"),
        Grid2dEditorCommand::SyncCamera { x: 4.0, y: 5.0, zoom: 2.0 }
    );
    let flat = semio_framework_value::DslValue::Object(vec![("x".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(4.0))), ("y".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(5.0))), ("zoom".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(2.0)))]);
    assert_eq!(
        <Grid2dEditor as ArtifactEditor>::command_from_action("set-camera", &mut Some((flat).clone()), &mut None).expect("decode"),
        Grid2dEditorCommand::SetCamera { x: 4.0, y: 5.0, zoom: 2.0 }
    );
}

#[test]
fn the_armed_utility_falls_back_through_window_then_focus_then_flat_then_select() {
    let mut view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_SELECT);
    view.active_utility_id = Some(grid::UTILITY_MASK.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_MASK);
    view.focused_window_id = Some("focused".into());
    view.active_utility_by_window_id.cold_insert("focused".into(), grid::UTILITY_PIN.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_PIN);
    view.window_id = Some("rendered".into());
    view.active_utility_by_window_id.cold_insert("rendered".into(), grid::UTILITY_SELECT.into());
    assert_eq!(grid2d_active_utility(&view), grid::UTILITY_SELECT);
}

#[test]
fn the_solve_command_starts_the_fill_run() {
    let document = crate::examples::grid2d::pipes::document();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let emit = Grid2dEditor::dispatch(&Grid2dEditorCommand::Solve, &doc, &cfg, None).expect("solve starts");
    assert!(emit.window_config_mutations.is_empty(), "solve must not write solve_json itself");
    assert!(
        emit.effects.iter().any(|effect| match effect {
            semio_framework::kernel::Effect::DispatchAction { action, args: Some(args), .. } => {
                action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID
                    && matches!(args, semio_framework_value::DslValue::Object(entries) if entries.iter().any(|(key, value)| key == semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID && matches!(value, semio_framework_value::DslValue::String(text) if text == fill_tool::TOOL_ID)))
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
    let commit = crate::host::inferences::solve_with_clock(&document, semio_framework_job::logical_now_us).expect("pipes solves");
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::with_operation(&document, &history, AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: [7; 32], retained: semio_plugin::artifact_app_laws::fixture_retained_turn().grant, authoring_seed: "seed".into() });
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let mut view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    view.window_id = Some("preview-1".into());
    view.window_instances = vec![semio_framework::ViewWindowInstance { id: "preview-1".into(), window_kind_id: preview::WINDOW_KIND_ID.into() }];
    let solve_json = semio_framework_pack_json::to_json_string(&fill_tool::Grid2dCachedSolve { source_revision: vec![7; 32], commit });
    let emit = Grid2dEditor::dispatch(&Grid2dEditorCommand::CommitFill { solve_json: solve_json.clone(), source_revision: vec![7; 32] }, &doc, &cfg, Some(&view)).expect("commit-fill writes");
    assert_eq!(emit.window_config_mutations.len(), 1);
}

/// 🎯️ LAW (design §22.32): a cell click of an armed writing utility is the framework's one-step tool — ONE tool transaction
/// `s.wfc.grid2d@1/*#editor#<utility>` holding the click's one leaf, from the pick verb; `select` yields nothing and leaves
/// zero trace; a view without an admission publishes the leaf plainly.
#[test]
fn a_cell_click_of_an_armed_utility_is_one_tool_transaction() {
    let document = crate::examples::grid2d::pipes::document();
    let history = semio_plugin::HistoryView::empty();
    let admitted = ArtifactView::with_operation(&document, &history, semio_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: [7; 32], retained: semio_plugin::artifact_app_laws::fixture_retained_turn().grant, authoring_seed: "seed".into() });
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let armed = |utility: &str| {
        let mut view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
        view.active_utility_id = Some(utility.into());
        view
    };
    let click = Grid2dEditorCommand::PickCell { x: 0, y: 0 };
    for utility in [grid::UTILITY_PIN, grid::UTILITY_MASK] {
        let emit = Grid2dEditor::dispatch(&click, &admitted, &cfg, Some(&armed(utility))).expect("the armed click dispatches");
        let transaction = emit.transaction.clone().expect("the click is a tool transaction");
        assert_eq!((transaction.tool, emit.artifact_mutations.len()), (format!("{}#{utility}", Grid2dEditor::TOOL_APP_ID), 1), "ONE transaction of the click's one leaf");
    }
    let selected = Grid2dEditor::dispatch(&click, &admitted, &cfg, Some(&armed(grid::UTILITY_SELECT))).expect("select dispatches");
    assert!(selected.transaction.is_none() && selected.artifact_mutations.is_empty(), "select yields nothing: zero trace");
    let plain = Grid2dEditor::dispatch(&click, &ArtifactView::new(&document, &history), &cfg, Some(&armed(grid::UTILITY_MASK))).expect("a view without an admission dispatches");
    assert!(plain.transaction.is_none() && plain.artifact_mutations.len() == 1, "no admission: the leaf publishes plainly");
}

/// ⚖️ LAW: `TOOL_IDS`, the per-tool publication-lane contracts and the `bounded_first_step_tool_proofs!`
/// rows are ONE roster. The framework joins all three at app registration and refuses the whole app with
/// `interactive-job.publication-contract` the moment they drift — and that refusal is a guest-side
/// `panic!`, so one missing lane row aborts the entire wfc component and EVERY wfc pane reaches
/// `data-shell-error` instead of `data-shell-ready`. That is exactly how `wfc2d`, `wfc3d` and `grid3d`
/// went red on 2026-09-22 when a new tool reached `TOOL_IDS` and the proofs but not the lane contracts.
#[test]
fn the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster() {
    use semio_plugin::ArtifactOwnedToolJobFactory;
    let tools: std::collections::BTreeSet<&str> = GRID2D_TOOL_IDS.iter().copied().collect();
    assert_eq!(<Grid2dCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, GRID2D_TOOL_IDS);
    let publication: std::collections::BTreeSet<&str> = <Grid2dCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Grid2dCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
        assert!(
            !contract.lanes.contains(&semio_plugin::ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1,
            "tool {} mixes the HostOnly lane with a publishing lane",
            contract.tool_id
        );
    }
    let proofs: std::collections::BTreeSet<&str> = <Grid2dEditor as ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
    assert_eq!(proofs, tools, "every owned tool carries its owner-local bounded reducer proof");
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a WFC 2D grid can be created and opened as a hub document.
#[test]
fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_grid2d_editor().artifact_kinds, vec![crate::artifact_kind()]);
}

semio_plugin::history_edit_acceptance_law!("wfc", crate::editor::grid2d::Grid2dEditor, || semio_plugin::App { definition: crate::editor::grid2d::create_grid2d_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");
/// 🎬️ The shared picker contract loads the boot document when its selection is cleared.
#[test]
fn the_example_picker_obeys_the_language_neutral_selection_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️example-selection/🔣️.json")).expect("selection contract");
    let artifact = fixture["artifacts"].as_array().unwrap().iter().find(|row| row["id"] == "grid2d").unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let input = case["input"].as_str().unwrap_or_else(|| artifact["examples"][case["exampleIndex"].as_u64().unwrap() as usize].as_str().unwrap());
        let actual = example_document(input);
        let expected = match case["select"].as_str().unwrap() {
            "default" => Some(crate::examples::grid2d::pipes::document()),
            "named" => Some(crate::examples::grid2d::terrain::document()),
            _ => None,
        };
        assert_eq!(actual, expected, "{}", case["id"]);
    }
    let examples = [
        (crate::examples::grid2d::pipes::ID, crate::examples::grid2d::pipes::document()),
        (crate::examples::grid2d::terrain::ID, crate::examples::grid2d::terrain::document()),
    ];
    let expected_ids: Vec<_> = examples.iter().map(|(id, _)| *id).collect();
    let fixture_ids: Vec<_> = artifact["examples"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
    assert_eq!(fixture_ids, expected_ids);
    for (id, expected) in examples {
        assert_eq!(example_document(id), Some(expected), "{id}");
    }
}

/// 📝️ Every author-facing action stages the arguments its typed command consumes.
#[test]
fn the_action_forms_obey_the_language_neutral_argument_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️grid2d-action-arguments/🔣️.json")).expect("argument contract");
    let source = include_str!("../../../../../../../../../🧪️tests/🧫️grid2d-action-arguments/🔣️.json");
    let native = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native contract");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_string(&native)).expect("native output"), fixture);
    let definition = create_grid2d_editor();
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["action"].as_str().unwrap();
        let action = definition.window_kinds.iter().flat_map(|window| &window.actions).find(|action| action.id == id).unwrap();
        let args = semio_framework_pack_json::parse(&case["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native arguments");
        let args = semio_framework_pack_json::to_dsl_value(&args);
        let effective = semio_framework::effective_action_args(&action.args, &args, None);
        let command = <Grid2dEditor as ArtifactEditor>::command_from_action(id, &mut Some(effective), &mut None).expect("form bridges");
        let actual = serde_json::Value::from(semio_framework_value::ToValue::to_value(&command));
        assert_eq!(actual, case["command"], "{id}");
    }
    for id in fixture["internalActions"].as_array().unwrap() {
        let id = id.as_str().unwrap();
        let action = definition.window_kinds.iter().flat_map(|window| &window.actions).find(|action| action.id == id).unwrap();
        assert!(!action.in_palette, "{id}");
    }
    for (id, expected) in fixture["actions"].as_object().unwrap() {
        let actions: Vec<_> = definition.window_kinds.iter().flat_map(|window| &window.actions).filter(|action| &action.id == id).collect();
        assert!(!actions.is_empty(), "{id}");
        for action in actions {
            let actual: Vec<&str> = action.args.iter().map(|arg| arg.id.as_str()).collect();
            let expected: Vec<&str> = expected.as_array().unwrap().iter().map(|arg| arg.as_str().unwrap()).collect();
            assert_eq!(actual, expected, "{id}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mounted_preview_uses_render_authority_for_live_source_binding() {
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunState};
    let snapshot = crate::examples::grid2d::pipes::document();
    let history = semio_plugin::HistoryView::empty();
    let context = semio_plugin::AppRenderOperationContext {
        app_instance_id: 1,
        base_revision: semio_framework_job::RevisionId(1),
        generation: semio_framework_job::Generation(1),
        canonical_base_revision: [0; 32],
        mounted_policy: semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,
    };
    let payload = fill_tool::Grid2dFillPayload { assignments: vec![fill_tool::Grid2dFillCell { x: 0, y: 0, tile_id: Some(snapshot.tiles[0].id.clone()) }], ..Default::default() };
    let make_run = || {
        let mut run = semio_plugin::ToolRunView::new("fill", ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]), ToolRunState::Running);
        run.payload = Some(payload.encode().into());
        run
    };
    let no_config = NoConfig::default();
    let config = ConfigView { snapshot: &no_config, window: None };
    let view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    let doc = ArtifactView::with_render_context(&snapshot, &history, semio_plugin::ChildContentView::EMPTY, context, "grid2d", None).await;
    assert!(doc.operation_optional().is_none());
    let empty = <Grid2dEditor as ArtifactEditor>::render(preview::BODY_KEY, &doc, &config, &view).expect("empty");
    let fresh = <Grid2dEditor as ArtifactEditor>::render(preview::BODY_KEY, &doc.with_tool_run(Some(make_run())), &config, &view).expect("fresh");
    let empty = serde_json::to_value(&empty.root).expect("empty tree");
    assert_ne!(serde_json::to_value(&fresh.root).expect("fresh tree"), empty);
    let changed = ArtifactView::with_render_context(&snapshot, &history, semio_plugin::ChildContentView::EMPTY, semio_plugin::AppRenderOperationContext { canonical_base_revision: [1; 32], ..context }, "grid2d", None).await.with_tool_run(Some(make_run()));
    let stale = <Grid2dEditor as ArtifactEditor>::render(preview::BODY_KEY, &changed, &config, &view).expect("stale");
    assert_eq!(serde_json::to_value(&stale.root).expect("stale tree"), empty);
}

#[test]
fn a_supported_control_grid_fits_the_completion_dispatch_envelope() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️dispatch-envelope/🔣️.json")).expect("portable envelope fixture");
    let width = fixture["width"].as_u64().unwrap() as u32;
    let height = fixture["height"].as_u64().unwrap() as u32;
    let tile_id = fixture["tileId"].as_str().unwrap();
    let commit = crate::schema::inferences::Grid2dInferenceCommit {
        assignments: (0..height).flat_map(|y| (0..width).map(move |x| (x, y, tile_id.to_owned()))).collect(),
        contradiction: false,
        entropy: Vec::new(),
    };
    let cache = fill_tool::Grid2dCachedSolve { source_revision: vec![0; 32], commit };
    let encoded = semio_framework_pack_json::to_json_string(&cache);
    assert_eq!(encoded.len() as u64, fixture["cacheBytes"].as_u64().unwrap());
    let args = semio_framework::dsl_value!({ "solveJson": encoded, "sourceRevision": vec![0u8; 32] });
    let native = semio_framework_pack_json::to_json_string(&args);
    let independent = serde_json::to_vec(&serde_json::json!({ "solveJson": semio_framework_pack_json::to_json_string(&cache), "sourceRevision": vec![0u8; 32] })).expect("third-party argument encoding");
    assert_eq!(native.as_bytes(), independent.as_slice());
    assert_eq!(native.len() as u64, fixture["argsBytes"].as_u64().unwrap());
    assert!(native.len() > 65_536, "the former envelope refused this supported completion");
    assert!(native.len() <= super::GRID2D_RETAINED_RAW_BYTES);
    assert!(native.len() <= super::GRID2D_ARTIFACT_MUTATION_MAXIMUM_BYTES);
}

#[test]
fn completed_cache_callbacks_publish_only_for_current_command_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).expect("revision cases");
    let document = crate::examples::grid2d::pipes::document();
    let history = semio_plugin::HistoryView::empty();
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let mut view = ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    view.window_id = Some("preview-1".into());
    view.window_instances = vec![semio_framework::ViewWindowInstance { id: "preview-1".into(), window_kind_id: preview::WINDOW_KIND_ID.into() }];
    for case in fixture["cases"].as_array().expect("cases") {
        let current: Option<[u8; 32]> = serde_json::from_value(case["currentRevision"].clone()).expect("current");
        let doc = match current {
            Some(revision) => ArtifactView::with_operation(&document, &history, AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: revision, retained: semio_plugin::artifact_app_laws::fixture_retained_turn().grant, authoring_seed: "seed".into() }),
            None => ArtifactView::new(&document, &history),
        };
        let source_revision: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).expect("source");
        let solve_json = semio_framework_pack_json::to_json_string(&fill_tool::Grid2dCachedSolve { source_revision: source_revision.clone(), commit: crate::schema::inferences::Grid2dInferenceCommit { assignments: vec![(0, 0, document.tiles[0].id.clone())], ..Default::default() } });
        let command = Grid2dEditorCommand::CommitFill { solve_json, source_revision };
        let emitted = Grid2dEditor::dispatch(&command, &doc, &cfg, Some(&view)).expect("completion");
        assert_eq!(emitted.window_config_mutations.len(), usize::from(case["visible"].as_bool().unwrap()), "{}", case["id"]);
        assert!(emitted.artifact_mutations.is_empty());
    }
}
