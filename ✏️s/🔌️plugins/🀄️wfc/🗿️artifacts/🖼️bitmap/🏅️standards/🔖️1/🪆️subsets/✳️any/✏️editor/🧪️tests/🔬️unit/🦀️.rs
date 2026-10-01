//! 🧪️ Editor surface — the manifest it declares, and that every typed command reaches exactly the
//! mutation it claims to.

use super::*;
use crate::{BitmapMutation, BitmapSnapshot};
use crate::schema::snapshot::encode_base64;

fn commands() -> Vec<(BitmapEditorCommand, &'static str)> {
    vec![
        (BitmapEditorCommand::ChangeSeed { seed: 99 }, "change-seed"),
        (BitmapEditorCommand::ResizeInput { width: 6, height: 4 }, "resize-input"),
        (BitmapEditorCommand::SetInputPixels { x: 0, y: 0, width: 1, height: 1, pixels: encode_base64(&[1]) }, "set-input-pixels"),
        (BitmapEditorCommand::AddPaletteColor { index: 2, r: 1, g: 2, b: 3, a: 255 }, "add-palette-color"),
        (BitmapEditorCommand::ChangePaletteColor { index: 0, r: 9, g: 9, b: 9, a: 255 }, "change-palette-color"),
        (BitmapEditorCommand::RemovePaletteColor { index: 1 }, "remove-palette-color"),
        (BitmapEditorCommand::ResizeOutput { width: 8, height: 8, periodic: true }, "resize-output"),
        (BitmapEditorCommand::ChangeModel { pattern_size: 3, symmetry: 4, periodic_input: false, ground: None }, "change-model"),
        (BitmapEditorCommand::PinPixel { x: 0, y: 0, color: 1 }, "pin-pixel"),
        (BitmapEditorCommand::UnpinPixel { x: 0, y: 0 }, "unpin-pixel"),
    ]
}

#[test]
fn the_manifest_binds_both_windows_and_a_row_layout() {
    let definition = create_bitmap_editor();
    assert_eq!(definition.id, "s.wfc.bitmap@1/*#editor");
    assert!(definition.window_kinds.iter().any(|window| window.id == input::WFC_BITMAP_WINDOW_INPUT));
    assert!(definition.window_kinds.iter().any(|window| window.id == output::WFC_BITMAP_WINDOW_OUTPUT));
    assert_eq!(definition.window_kinds.len(), 2);
}

#[test]
fn the_editor_boots_the_committed_example_rather_than_an_empty_default() {
    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    assert_eq!(snapshot, crate::examples::rooms_16::snapshot());
    assert!(snapshot.input.indices().is_some());
}

#[test]
fn every_command_round_trips_its_binary_op() {
    for (command, _) in commands() {
        let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
        assert_eq!(<BitmapEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("command decodes"), command);
    }
    for command in [BitmapEditorCommand::Solve, BitmapEditorCommand::CommitFillSolve { pixels: String::new(), contradiction: false, width: 1, height: 1 }, stroke(&[(1, 2), (3, 4)], Some(1), Some("stream")), stroke(&[], None, Some("abort")), BitmapEditorCommand::SetActiveColor { index: 1 }] {
        let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
        assert_eq!(<BitmapEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("command decodes"), command);
    }
}

/// 🖌️ A `paint-stroke` command — the brush tool's one verb — over `points`.
fn stroke(points: &[(u32, u32)], color: Option<u32>, phase: Option<&str>) -> BitmapEditorCommand {
    BitmapEditorCommand::PaintStroke { xs: points.iter().map(|point| point.0).collect(), ys: points.iter().map(|point| point.1).collect(), color, phase: phase.map(str::to_string), reason: None }
}

/// 🧭️ Every mutation kind is reachable from a window action: the verb of the same name, or — for the one leaf a
/// tool yields — the tool's verb (`paint-input-stroke` through the brush's `paint-stroke`).
#[test]
fn every_mutation_kind_is_reachable_from_a_window() {
    let declared: Vec<String> = create_bitmap_editor().window_kinds.iter().flat_map(|window| window.actions.iter().map(|action| action.id.clone())).collect();
    for kind in crate::mutations::KINDS {
        let verb = if *kind == "paint-input-stroke" { "paint-stroke" } else { kind };
        assert!(declared.iter().any(|action| action == verb), "mutation '{kind}' is not reachable from any window action");
    }
    for verb in ["set-active-color", "solve", "paint-stroke"] {
        assert!(declared.iter().any(|action| action == verb), "non-document verb '{verb}' is not declared by any window");
    }
}

#[test]
fn every_typed_command_dispatches_to_the_mutation_it_names() {
    for (command, kind) in commands() {
        let (mutation, description) = BitmapEditor::command_mutation(&command).unwrap_or_else(|| panic!("command '{kind}' maps to a mutation"));
        assert_eq!(protocol::SemanticMutation::semantics(&mutation).kind, kind);
        assert!(!description.is_empty(), "command '{kind}' describes its own edit");
    }
    for command in [BitmapEditorCommand::Solve, BitmapEditorCommand::CommitFillSolve { pixels: String::new(), contradiction: false, width: 1, height: 1 }, stroke(&[(0, 0)], None, None), BitmapEditorCommand::SetActiveColor { index: 0 }] {
        assert!(BitmapEditor::command_mutation(&command).is_none(), "a non-document verb emits no artifact mutation");
    }
}

/// 🩺 `remove-palette-color` and `unpin-pixel` are legitimately refused against the boot
/// example — every colour is painted and nothing is pinned — but a refusal must SAY so.
#[test]
fn every_dispatched_mutation_either_moves_the_boot_example_or_says_why_not() {
    let base = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    for (command, kind) in commands() {
        let Some((mutation, _)) = BitmapEditor::command_mutation(&command) else { continue };
        let outcome = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation, &base);
        let mut snapshot = base.clone();
        crate::mutations::apply_bitmap_mutation(&mut snapshot, &mutation).unwrap_or_else(|error| panic!("'{kind}' applies to the boot example: {error}"));
        if snapshot == base {
            assert!(!outcome.messages().is_empty(), "'{kind}' changed nothing and raised no diagnostic");
        }
    }
}

#[test]
fn the_dialect_is_shared_with_the_document_schema() {
    assert_eq!(<BitmapEditor as ArtifactEditor>::DIALECT, WFC_BITMAP_DIALECT);
    assert_eq!(<BitmapEditor as ArtifactEditor>::DOCUMENT_SCHEMA, WFC_BITMAP_DOCUMENT_SCHEMA);
}

//#region 🖌️Gesture
/// 🖌️ The brush verb the input window advertises is answered by a typed command, `Migrated` (an unclassified verb is
/// dispatch-dead in the shell), and the window declares the brush utility it belongs to. The window config holds
/// no stroke scratch any more: a stroke in flight is window-transient tool state.
#[test]
fn the_brush_verb_and_utility_are_declared_and_answered() {
    let definition = create_bitmap_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == input::WFC_BITMAP_WINDOW_INPUT).expect("the input window is declared");
    let action = window.actions.iter().find(|action| action.id == "paint-stroke").expect("'paint-stroke' is catalogued");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "'paint-stroke' would be dispatch-dead");
    assert!(action.args.iter().any(|arg| arg.id == "points" && arg.required && matches!(arg.schema, semio_framework_plugin::ArgSchema::Array { .. })));
    assert!(window.utilities.iter().any(|utility| utility.as_str() == input::utilities::brush::UTILITY_ID));
    for retired in ["stroke-begin", "stroke-extend", "stroke-commit"] {
        assert!(!BITMAP_TOOL_IDS.contains(&retired), "'{retired}' was replaced by the brush tool");
    }
    let config = dsl::json::to_json_string(&input::config::BitmapInputWindowConfig::default());
    assert!(!config.contains("stroke"), "the window config carries no stroke scratch: {config}");
}

/// 🌉️ The bridge reads a stroke's points as `{x, y}` objects or `[x, y]` pairs and refuses anything else.
#[test]
fn the_bridge_reads_stroke_points_in_both_spellings_and_refuses_garbage() {
    let objects = dsl::DslValue::from(&serde_json::json!({ "points": [{ "x": 1, "y": 2 }, { "x": 3, "y": 4 }], "color": 1 }));
    assert_eq!(<BitmapEditor as ArtifactEditor>::command_from_action("paint-stroke", Some(&objects)).expect("objects bridge"), stroke(&[(1, 2), (3, 4)], Some(1), None));
    let pairs = dsl::DslValue::from(&serde_json::json!({ "points": [[1, 2], [3, 4]], "phase": "stream" }));
    assert_eq!(<BitmapEditor as ArtifactEditor>::command_from_action("paint-stroke", Some(&pairs)).expect("pairs bridge"), stroke(&[(1, 2), (3, 4)], None, Some("stream")));
    for garbage in [serde_json::json!({ "points": [[1.5, 2]] }), serde_json::json!({ "points": [[-1, 2]] }), serde_json::json!({ "points": [[1]] }), serde_json::json!({ "points": "1,2" })] {
        assert!(<BitmapEditor as ArtifactEditor>::command_from_action("paint-stroke", Some(&dsl::DslValue::from(&garbage))).is_err(), "{garbage} must be refused");
    }
}
//#endregion 🖌️Gesture

//#region 🚚️LiveDispatch
/// ✉️ The transient's own envelope must be publishable. `print_dsl` is what the framework's transient
/// publication calls, and a NON-DOTTED envelope id made its own `expect` PANIC the guest the first
/// time a solve was published — trapping every later dispatch in the live shell. Found on the
/// playground, not by any test that existed before this one.
#[test]
fn the_solve_transient_prints_and_packs_its_own_envelope() {
    use crate::editor::bitmap::transient::BitmapTransient;
    let transient = BitmapTransient { output_pixels: Some(encode_base64(&[0, 1, 2, 1])), contradiction: false, output_width: 2, output_height: 2 };
    let text = <BitmapTransient as store::ArtifactDsl>::print_dsl(&transient);
    assert!(text.starts_with("semio wfc.bitmaptransient.dsl v1"), "unexpected transient preamble: {:?}", text.lines().next());
    assert_eq!(<BitmapTransient as store::ArtifactDsl>::parse_dsl(&text).expect("the transient parses back"), transient);
    let packed = <BitmapTransient as store::ArtifactPack>::encode_pack(&transient);
    assert_eq!(<BitmapTransient as store::ArtifactPack>::decode_pack(&packed).expect("the transient unpacks"), transient);
}

/// 🫧️ The `Solve` verb's own transient write really carries a collapse — not a uniform square, not a
/// contradiction — for the boot example. This is the payload the output window renders.
#[test]
fn the_solve_command_publishes_a_real_collapse_on_the_transient_lane() {
    use crate::editor::bitmap::transient::BitmapTransientMutation;
    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    let BitmapTransientMutation::SetSolve(solve) = BitmapEditor::solve_transient(&snapshot).expect("the boot example's solve runs");
    assert!(!solve.contradiction, "the boot example must collapse");
    assert_eq!((solve.output_width, solve.output_height), (snapshot.output.width, snapshot.output.height));
    let pixels = crate::schema::snapshot::decode_base64(solve.output_pixels.as_deref().expect("a solved output")).expect("the commit decodes");
    assert_eq!(pixels.len(), (snapshot.output.width as usize) * (snapshot.output.height as usize));
    assert!(pixels.iter().any(|index| *index != pixels[0]), "a uniform square is not a collapse");
}

/// 🌉️ Every action the manifest declares must bridge through `command_from_action` to the command
/// whose id it is. `VcsArtifactApp::dispatch_action` resolves EVERY host action that way, so a
/// missing arm is a dispatch-dead verb no classification audit can see.
#[test]
fn every_declared_action_bridges_to_the_command_it_names() {
    for action in BITMAP_TOOL_IDS {
        let command = <BitmapEditor as ArtifactEditor>::command_from_action(action, None).unwrap_or_else(|error| panic!("action '{action}' does not bridge: {}", error.message));
        assert_eq!(bitmap_command_id(&command), *action, "action '{action}' bridged to the wrong command");
    }
    assert!(<BitmapEditor as ArtifactEditor>::command_from_action("no-such-action", None).is_err());
}

/// 🧾️ The retained roster is ONE list spelled in four places — the command channel's `TOOL_JOB_IDS`,
/// the owned factory's `TOOL_IDS`, its publication contracts and the bounded-first-step proofs. The
/// framework joins all four at registration and fails closed on any disagreement.
#[test]
fn the_retained_tool_roster_agrees_with_itself_and_with_the_manifest() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    assert_eq!(<BitmapEditorCommand as protocol::OpBinary>::TOOL_JOB_IDS, BITMAP_TOOL_IDS);
    assert_eq!(<BitmapCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, BITMAP_TOOL_IDS);
    let contracts: Vec<&str> = <BitmapCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(contracts, BITMAP_TOOL_IDS.to_vec());
    assert!(<BitmapCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().all(|contract| !contract.lanes.is_empty()));
    let proofs: Vec<&str> = <BitmapEditor as ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(semio_framework_plugin::ArtifactBoundedFirstStepProof::tool_id).collect();
    assert_eq!(proofs, BITMAP_TOOL_IDS.to_vec());

    let definition = create_bitmap_editor();
    let declared: std::collections::BTreeSet<String> = definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(&definition, window))
        .map(|action| action.id.clone())
        .collect();
    for action in BITMAP_TOOL_IDS {
        assert!(declared.contains(*action), "retained tool '{action}' is not declared by any window kind");
    }
}

/// 🎬️ The example picker's verb is declared, classified and offers both bundled examples — the exact
/// three facts the shell's boot announcement and navbar combobox need.
#[test]
fn the_example_picker_verb_is_declared_with_both_examples() {
    let definition = create_bitmap_editor();
    for window in &definition.window_kinds {
        let action = semio_framework::window_kind_actions(&definition, window)
            .into_iter()
            .find(|action| action.id == "setActiveExample")
            .unwrap_or_else(|| panic!("window '{}' does not declare setActiveExample", window.id));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
        let arg = action.args.first().unwrap_or_else(|| panic!("window '{}' offers setActiveExample no example argument", window.id));
        assert_eq!(arg.id, "exampleId");
        assert!(arg.required);
        assert_eq!(arg.default.as_ref().and_then(dsl::DslValue::as_str), Some(set_active_example::BITMAP_EXAMPLE_BOOT_ID));
    }
}
//#endregion 🚚️LiveDispatch


//#region 🌡Fill
#[test]
fn the_manifest_declares_the_fill_tool_on_edit_mode() {
    use crate::editor::bitmap::modes::edit;
    use crate::editor::bitmap::modes::edit::tools::fill as fill_tool;
    let definition = create_bitmap_editor();
    assert!(definition.tools.iter().any(|tool| tool.id == fill_tool::TOOL_ID));
    let mode = definition.modes.iter().find(|mode| mode.id == edit::WFC_BITMAP_MODE_EDIT).expect("edit mode");
    assert!(mode.tools.iter().any(|tool| tool.as_str() == fill_tool::TOOL_ID));
}

#[test]
fn solve_starts_the_fill_run_instead_of_writing_set_solve() {
    use semio_framework_plugin::{AppOperationContext, Effect, HistoryView};
    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    let history = HistoryView::empty();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "wfc-bitmap-fill".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32], authoring_seed: "authoring-seed-test".into() };
    let config = NoConfig {};
    let emit = BitmapEditor::dispatch(&BitmapEditorCommand::Solve, &ArtifactView::with_operation(&snapshot, &history, operation), &ConfigView { snapshot: &config, window: None }, None).expect("solve dispatches");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == semio_framework_tool_run::TOOL_RUN_START_ACTION_ID)));
}

#[test]
fn a_partial_fill_render_differs_from_empty_and_finished() {
    use crate::editor::bitmap::modes::edit::tools::fill::payload_from_assignment;
    use crate::editor::bitmap::modes::edit::windows::output::{self, config::BitmapOutputWindowConfig};
    use crate::editor::bitmap::transient::BitmapTransient;
    use semio_framework_plugin::ToolRunView;
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunState};

    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    let empty = output::render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None);
    let finished_pixels = crate::inferences::solve_with_clock(&snapshot, semio_framework_job::logical_now_us).expect("oracle");
    let finished = BitmapTransient {
        output_pixels: if finished_pixels.contradiction { None } else { Some(finished_pixels.pixels.clone()) },
        contradiction: finished_pixels.contradiction,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
    };
    let finished_layers = output::render_layers_fingerprint(&snapshot, &finished, &BitmapOutputWindowConfig::default(), None);
    assert_ne!(empty, finished_layers);

    let partial = payload_from_assignment(snapshot.output.width, snapshot.output.height, &[(0, 1)], false, false);
    let mut run = ToolRunView::new(fill_tool::TOOL_ID, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 3 }, [0; 32]), ToolRunState::Running);
    run.payload = Some(partial.encode_json().into_bytes().into());
    let partial_layers = output::render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), Some(&run));
    assert_ne!(partial_layers, empty);
    assert_ne!(partial_layers, finished_layers);
}

#[test]
fn a_contradiction_transient_paints_a_distinct_overlay() {
    use crate::editor::bitmap::modes::edit::windows::output::{self, config::BitmapOutputWindowConfig};
    use crate::editor::bitmap::transient::BitmapTransient;
    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    let empty = output::render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None);
    let contradiction = BitmapTransient { output_pixels: None, contradiction: true, output_width: snapshot.output.width, output_height: snapshot.output.height };
    let layers = output::render_layers_fingerprint(&snapshot, &contradiction, &BitmapOutputWindowConfig::default(), None);
    assert!(layers.contains("out-contradiction"));
    assert_ne!(layers, empty);
}

#[test]
fn the_python_oracle_vector_matches_the_rust_payload_shape() {
    use crate::editor::bitmap::modes::edit::tools::fill::{payload_from_assignment, BitmapFillPayload};
    let text = include_str!("../../🎭️modes/✏️edit/🛠️tools/🌡fill/🧫️fixtures/🐍️python-fill-oracle/🔣️.json");
    let payload = BitmapFillPayload::decode_json(text).expect("vector decodes");
    assert_eq!(payload.width, 2);
    assert_eq!(payload.height, 2);
    assert_eq!(payload.decided_count(), 2);
    assert!(!payload.done);
    let expected = payload_from_assignment(2, 2, &[(0, 0), (3, 1)], false, false);
    assert_eq!(payload.pixels, expected.pixels);
    assert_eq!(payload.decided, expected.decided);
    assert!(payload.trace.iter().any(|event| event.discarded), "the vector keeps a cell the search undid");
}
//#endregion 🌡Fill

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a WFC bitmap can be created and opened as a hub document.
#[test]
fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_bitmap_editor().artifact_kinds, vec![crate::artifact_kind()]);
}
