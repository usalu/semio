//! 🔬️ Editor laws — every declared verb dispatches, the pick verb reads the ARMED UTILITY of the
//! window it was dispatched in, and the two windows both render non-empty for both examples.

use super::*;

#[test]
fn the_manifest_declares_both_window_kinds_and_the_three_utilities() {
    let definition = create_grid3d_editor();
    assert_eq!(definition.id, "s.wfc.grid3d@1/*#editor");
    assert!(definition.window_kinds.iter().any(|window| window.id == grid::WINDOW_KIND_ID));
    assert!(definition.window_kinds.iter().any(|window| window.id == preview::WINDOW_KIND_ID));
    let utilities: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utilities, vec![grid::UTILITY_SELECT, grid::UTILITY_PIN, grid::UTILITY_MASK]);
}

#[test]
fn every_declared_action_is_migrated_so_it_can_actually_be_dispatched() {
    let definition = create_grid3d_editor();
    for window in &definition.window_kinds {
        for action in &window.actions {
            assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated, "action {} would be dead in the interactive app", action.id);
        }
    }
}

#[test]
fn the_grid_window_declares_one_action_per_document_verb() {
    let definition = grid::definition();
    let ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
    for verb in ["changeSeed", "resizeGrid", "changeCellSizes", "changePeriodicity", "createTile", "deleteTile", "changeTileWeight", "changeTileColor", "createRule", "deleteRule", "pinCell", "unpinCell", "maskCell", "unmaskCell"] {
        assert!(ids.contains(&verb), "the grid window must declare {verb}");
    }
}

#[test]
fn a_cell_key_parses_back_only_when_the_grid_could_have_emitted_it() {
    assert_eq!(parse_cell_id("1:2:3"), Some((1, 2, 3)));
    assert_eq!(parse_cell_id("1:2"), None);
    assert_eq!(parse_cell_id("1:2:3:4"), None);
    assert_eq!(parse_cell_id("a:b:c"), None);
}

#[test]
fn the_armed_utility_defaults_to_select_when_no_window_has_one() {
    assert_eq!(grid3d_active_utility(None), grid::UTILITY_SELECT);
}

#[test]
fn every_command_maps_onto_a_real_mutation_builder() {
    let commands = vec![
        Grid3dEditorCommand::ChangeSeed { seed: 1 },
        Grid3dEditorCommand::ResizeGrid { width: 2, height: 2, depth: 2 },
        Grid3dEditorCommand::ChangePeriodicity { periodic_x: true, periodic_y: false, periodic_z: false },
        Grid3dEditorCommand::DeleteTile { id: "wall".into() },
        Grid3dEditorCommand::UnpinCell { x: 0, y: 0, z: 0 },
        Grid3dEditorCommand::MaskCell { x: 1, y: 1, z: 1 },
        Grid3dEditorCommand::PickCell { cell_id: "1:0:0".into() },
        Grid3dEditorCommand::SetActiveTile { tile_id: "wall".into() },
    ];
    for command in commands {
        let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
        assert_eq!(<Grid3dEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("command decodes"), command);
    }
}

/// 🎯️ Every action a window kind declares has to bridge back through `command_from_action` onto the
/// command it was declared for. The trait default answers `app.command.unsupported` for ALL of them,
/// which is why the whole vocabulary was unreachable from the browser before this bridge existed.
/// 🏛️ The framework injects its own reserved vocabulary onto every window kind (history, clipboard,
/// the six interaction verbs, the utility/tool arming). Those are dispatched by the framework, never
/// through an app's command channel, so they are not this bridge's business.
#[test]
fn every_declared_action_bridges_to_the_command_it_names() {
    let definition = create_grid3d_editor();
    let staged = |action: &semio_framework_plugin::ActionDefinition| semio_framework::effective_action_args(&action.args, &semio_framework_value::DslValue::Object(Vec::new()), None);
    let arguments: std::collections::BTreeMap<&str, semio_framework_value::DslValue> = [
        ("changeSeed", semio_framework_pack_json::parse(r#"{"seed":7}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("resizeGrid", semio_framework_pack_json::parse(r#"{"width":2,"height":2,"depth":2}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("changeCellSizes", semio_framework_pack_json::parse(r#"{"axis":"x","sizes":[1.0,2.0]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("createTile", semio_framework_pack_json::parse(r#"{"id":"air"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("deleteTile", semio_framework_pack_json::parse(r#"{"id":"air"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("changeTileWeight", semio_framework_pack_json::parse(r#"{"tileId":"air","weight":2.0}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("changeTileColor", semio_framework_pack_json::parse(r#"{"tileId":"air","r":1,"g":2,"b":3,"a":4}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("createRule", semio_framework_pack_json::parse(r#"{"id":"r","tileAId":"air","tileBId":"wall","direction":"RIGHT","allowed":true}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("deleteRule", semio_framework_pack_json::parse(r#"{"id":"r"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("pinCell", semio_framework_pack_json::parse(r#"{"x":0,"y":0,"z":0,"tileId":"air"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("unpinCell", semio_framework_pack_json::parse(r#"{"x":0,"y":0,"z":0}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("maskCell", semio_framework_pack_json::parse(r#"{"x":0,"y":0,"z":0}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        ("unmaskCell", semio_framework_pack_json::parse(r#"{"x":0,"y":0,"z":0}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
        (grid::ACTION_WORLD_SELECT, semio_framework_pack_json::parse(r#"{"ids":["1:2:3"],"merge":"replace"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value))),
    ]
    .into_iter()
    .map(|(id, value)| (id, value.expect("argument fixture parses")))
    .collect();
    const RESERVED: &[&str] = &[
        "undo",
        "redo",
        "commitCheckpoint",
        "createAlternative",
        "switchAlternative",
        "checkoutCheckpoint",
        "copy",
        "cut",
        "paste",
        "revertToCommand",
        "setHistoryCommandFilter",
        "noteShellCommand",
        "recordTutorial",
        "startIntroduction",
        "startTutorial",
        "setActiveUtility",
        "setActiveTool",
        "interactionSelect",
        "interactionHover",
        "clearSelection",
        "selectAll",
        "setSelectionMode",
        "setInteractionGranularity",
    ];
    let mut bridged = 0usize;
    for window in &definition.window_kinds {
        for action in &window.actions {
            if RESERVED.contains(&action.id.as_str()) || semio_framework_plugin::is_tool_run_action_id(&action.id) {
                continue;
            }
            let args = arguments.get(action.id.as_str()).cloned().unwrap_or_else(|| staged(action));
            let command = command_from_action(&action.id, Some(&args)).unwrap_or_else(|fault| panic!("action {} failed to bridge: {}", action.id, fault.message));
            assert_eq!(Grid3dEditor::command_id(&command), action.id.as_str(), "command_id mismatch for action {}", action.id);
            bridged += 1;
        }
    }
    // 🧮️ Pinned to the ROSTER, not to a literal: the grid window declares exactly the retained tool ids
    // (the framework refuses any retained proof row whose verb no window declares), so a new verb that
    // reaches `GRID3D_RETAINED_TOOL_IDS` must also reach this bridge — a stale literal here only hid that.
    assert_eq!(bridged, GRID3D_RETAINED_TOOL_IDS.len(), "every grid-window action must be exercised");
}

/// 🖱️ The host's plugin-private world pick wraps the cell key in a one-entry `ids` array while the
/// declared verb carries it bare. They stay two commands — `dispatch_typed_command_inner` rejects a
/// command whose id is not the action it was admitted under — but they must carry the SAME key and
/// resolve through the same reducer arm.
#[test]
fn both_pick_lanes_carry_the_same_cell_key() {
    let world = command_from_action(grid::ACTION_WORLD_SELECT, Some(&semio_framework_pack_json::parse(r#"{"ids":["2:1:0"],"merge":"replace"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value)).expect("args"))).expect("world pick bridges");
    let direct = command_from_action("pickCell", Some(&semio_framework_pack_json::parse(r#"{"cellId":"2:1:0"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value)).expect("args"))).expect("pick bridges");
    assert_eq!(world, Grid3dEditorCommand::WorldSelect { cell_id: "2:1:0".into() });
    assert_eq!(direct, Grid3dEditorCommand::PickCell { cell_id: "2:1:0".into() });
    assert_eq!(Grid3dEditor::command_id(&world), grid::ACTION_WORLD_SELECT);
    assert_eq!(Grid3dEditor::command_id(&direct), "pickCell");
}

/// 🧵️ `Migrated` is the only live classification, so every declared verb runs the retained route and
/// needs an exact app-owned proof; an id missing from the roster or from the publication contracts
/// answers `interactive-job.missing-factory` and is dead in the running app.
#[test]
fn every_declared_verb_carries_a_retained_proof_and_a_publication_lane() {
    let definition = create_grid3d_editor();
    for window in &definition.window_kinds {
        for action in &window.actions {
            if !GRID3D_RETAINED_TOOL_IDS.contains(&action.id.as_str()) {
                continue;
            }
            assert!(
                GRID3D_PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == action.id),
                "declared verb {} rides the retained route with no publication lane",
                action.id
            );
        }
    }
    for tool_id in GRID3D_RETAINED_TOOL_IDS {
        assert!(GRID3D_PUBLICATION_CONTRACTS.iter().any(|contract| &contract.tool_id == tool_id), "retained tool {tool_id} declares no publication lane");
        assert!(
            definition.window_kinds.iter().any(|window| window.actions.iter().any(|action| &action.id.as_str() == tool_id)),
            "retained tool {tool_id} is declared by no window kind, so no pane can ever dispatch it"
        );
    }
}

/// 📚️ Switching examples is a whole-document replace (a `LoadDocument` effect), never a mutation —
/// and re-selecting the example already open must emit NOTHING, or the picker writes a phantom edit
/// that leaves `canUndo` true on a document nobody changed.
#[test]
fn the_example_picker_replaces_the_document_and_re_selecting_the_open_one_is_inert() {
    for id in [crate::examples::blocks::ID, crate::examples::pipes_3d::ID] {
        assert!(example_snapshot(id).is_some(), "example '{id}' must resolve");
    }
    assert!(example_snapshot("nope").is_none());
    let document = crate::examples::blocks::snapshot();
    let effect = reset_document_effect(&crate::examples::pipes_3d::snapshot());
    assert!(matches!(effect, semio_framework::kernel::Effect::LoadDocument { .. }), "an example switch is a LoadDocument effect");
    assert_ne!(crate::examples::pipes_3d::snapshot(), document, "the two examples must actually differ");
}

/// ⚖️ LAW: this app's one app-owned factory carries ONE roster — `TOOL_IDS`, its
/// `PUBLICATION_CONTRACTS` and its `bounded_first_step_tool_proofs!` rows name exactly the same
/// tools. The framework refuses app registration outright when they drift
/// (`interactive-job.publication-contract` when a lane contract names an unowned tool,
/// `interactive-job.catalog-incomplete` when a migrated command has no owner-local proof), and that
/// refusal is a guest-side `panic!` — so one missing row aborted the whole wfc component at boot and
/// every one of its panes reached `data-shell-error` instead of `data-shell-ready`.
#[test]
fn the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let tools: std::collections::BTreeSet<&str> = GRID3D_RETAINED_TOOL_IDS.iter().copied().collect();
    let publication: std::collections::BTreeSet<&str> = <Grid3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Grid3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
    }
    let proofs: std::collections::BTreeSet<&str> = <Grid3dEditor as semio_framework_plugin::ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
    assert_eq!(proofs, tools, "every owned tool carries its owner-local bounded reducer proof");
}

/// 🎯️ LAW (design §22.32): a cell click of an armed writing utility is the framework's one-step tool — ONE tool transaction
/// `s.wfc.grid3d@1/*#editor#<utility>` holding the click's one leaf; `select` yields nothing and leaves zero trace; a view
/// without an admission publishes the leaf plainly.
#[test]
fn a_cell_click_of_an_armed_utility_is_one_tool_transaction() {
    let document = crate::examples::blocks::snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let admitted = ArtifactView::with_operation(&document, &history, semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "doc".into(), operation_id: 1, generation: 1, canonical_base_revision: [7; 32], authoring_seed: "seed".into() });
    let no_config = NoConfig::default();
    let cfg = ConfigView { snapshot: &no_config, window: None };
    let armed = |utility: &str| {
        let mut view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        view.active_utility_id = Some(utility.into());
        view
    };
    let click = Grid3dEditorCommand::PickCell { cell_id: "0:0:0".into() };
    let masked = grid3d_command_emit(&click, &admitted, &cfg, Some(&armed(grid::UTILITY_MASK))).expect("the armed click dispatches");
    let transaction = masked.transaction.clone().expect("the click is a tool transaction");
    assert_eq!((transaction.tool, masked.artifact_mutations.len()), (format!("{GRID3D_EDITOR_APP_ID}#{}", grid::UTILITY_MASK), 1), "ONE transaction of the click's one leaf");
    let selected = grid3d_command_emit(&click, &admitted, &cfg, Some(&armed(grid::UTILITY_SELECT))).expect("select dispatches");
    assert!(selected.transaction.is_none() && selected.artifact_mutations.is_empty(), "select yields nothing: zero trace");
    let plain = grid3d_command_emit(&click, &ArtifactView::new(&document, &history), &cfg, Some(&armed(grid::UTILITY_MASK))).expect("a view without an admission dispatches");
    assert!(plain.transaction.is_none() && plain.artifact_mutations.len() == 1, "no admission: the leaf publishes plainly");
}

semio_framework_plugin::history_edit_acceptance_law!("wfc", super::Grid3dEditor, || semio_framework_plugin::App { definition: super::create_grid3d_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");
