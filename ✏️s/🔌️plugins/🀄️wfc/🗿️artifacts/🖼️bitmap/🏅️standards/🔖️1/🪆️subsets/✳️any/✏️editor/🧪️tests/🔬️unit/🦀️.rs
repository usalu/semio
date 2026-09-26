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
    for command in [BitmapEditorCommand::Solve, BitmapEditorCommand::CommitFillSolve { pixels: String::new(), contradiction: false, width: 1, height: 1 }, BitmapEditorCommand::StrokeBegin { x: 1, y: 2 }, BitmapEditorCommand::StrokeExtend { x: 3, y: 4 }, BitmapEditorCommand::StrokeCommit, BitmapEditorCommand::SetActiveColor { index: 1 }] {
        let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
        assert_eq!(<BitmapEditorCommand as protocol::OpBinary>::decode_op(&bytes).expect("command decodes"), command);
    }
}

#[test]
fn every_mutation_kind_is_reachable_from_a_window() {
    let declared: Vec<String> = create_bitmap_editor().window_kinds.iter().flat_map(|window| window.actions.iter().map(|action| action.id.clone())).collect();
    for kind in crate::mutations::KINDS {
        assert!(declared.iter().any(|action| action == kind), "mutation '{kind}' is not reachable from any window action");
    }
    for verb in ["set-active-color", "solve", "stroke-begin", "stroke-extend", "stroke-commit"] {
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
    for command in [BitmapEditorCommand::Solve, BitmapEditorCommand::CommitFillSolve { pixels: String::new(), contradiction: false, width: 1, height: 1 }, BitmapEditorCommand::StrokeBegin { x: 0, y: 0 }, BitmapEditorCommand::StrokeExtend { x: 0, y: 0 }, BitmapEditorCommand::StrokeCommit, BitmapEditorCommand::SetActiveColor { index: 0 }] {
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
/// 🖌️ The gesture contract, driven end to end without a mounted host: pointer-down, two drag
/// samples and a release produce EXACTLY ONE `set-input-pixels`, over the UNION of every sampled
/// cell, filled with the pane's armed colour. Mid-drag ticks grow a box in the pane's own config and
/// emit no document operation at all.
#[test]
fn a_drag_of_four_samples_commits_exactly_one_mutation_over_the_union_region() {
    use crate::editor::bitmap::modes::edit::windows::input::config::BitmapStroke;

    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    let mut stroke: Option<BitmapStroke> = None;
    for (index, (x, y)) in [(2u32, 3u32), (5, 3), (4, 7), (3, 5)].into_iter().enumerate() {
        stroke = Some(match stroke {
            Some(previous) if index > 0 => previous.extended(x, y),
            _ => BitmapStroke::at(x, y),
        });
    }
    let stroke = stroke.expect("the gesture began");
    assert_eq!((stroke.min_x, stroke.min_y, stroke.max_x, stroke.max_y), (2, 3, 5, 7), "the box is the union of every sample");
    assert_eq!((stroke.width(), stroke.height()), (4, 5));

    let mutation = BitmapEditor::stroke_mutation(&snapshot, stroke, 1).expect("the settled gesture commits");
    match &mutation {
        crate::BitmapMutation::SetInputPixels(payload) => {
            assert_eq!((payload.x, payload.y, payload.width, payload.height), (2, 3, 4, 5), "one mutation, over the union region");
            let pixels = crate::schema::snapshot::decode_base64(&payload.pixels).expect("the stroke payload decodes");
            assert_eq!(pixels.len(), 20, "one index per covered cell");
            assert!(pixels.iter().all(|index| *index == 1), "the whole region takes the armed colour");
        }
        other => panic!("a settled stroke must be one set-input-pixels, not {other:?}"),
    }

    let mut applied = snapshot.clone();
    crate::mutations::apply_bitmap_mutation(&mut applied, &mutation).expect("the stroke applies");
    assert_ne!(applied, snapshot, "the settled gesture really moved the sample");
}

/// 🖌️ A restart really restarts: `stroke-begin` after a gesture is a fresh box, never a union with
/// the previous one.
#[test]
fn a_second_gesture_does_not_inherit_the_first_ones_box() {
    use crate::editor::bitmap::modes::edit::windows::input::config::BitmapStroke;
    let first = BitmapStroke::at(0, 0).extended(9, 9);
    let second = BitmapStroke::at(3, 3).extended(4, 4);
    assert_eq!((second.min_x, second.max_x), (3, 4));
    assert_ne!(first, second);
}

/// 🖌️ A box that leaves the sample, or an unarmed colour, is REFUSED — clipping either would commit
/// an edit the author did not draw.
#[test]
fn a_stroke_outside_the_sample_or_off_the_palette_is_refused() {
    use crate::editor::bitmap::modes::edit::windows::input::config::BitmapStroke;
    let snapshot = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    let outside = BitmapStroke::at(snapshot.input.width - 1, 0).extended(snapshot.input.width, 0);
    assert!(BitmapEditor::stroke_mutation(&snapshot, outside, 0).is_err());
    let inside = BitmapStroke::at(0, 0);
    assert!(BitmapEditor::stroke_mutation(&snapshot, inside, snapshot.input.palette.len() as u32).is_err());
    assert!(BitmapEditor::stroke_mutation(&snapshot, inside, 0).is_ok());
}

/// 🖌️ Every stroke verb the input window advertises is answered by a typed command variant, and all
/// three are `Migrated` — an unclassified verb is dispatch-dead in the shell.
#[test]
fn the_stroke_verbs_are_declared_migrated_and_answered() {
    let definition = create_bitmap_editor();
    let window = definition.window_kinds.iter().find(|window| window.id == input::WFC_BITMAP_WINDOW_INPUT).expect("the input window is declared");
    for verb in ["stroke-begin", "stroke-extend", "stroke-commit"] {
        let action = window.actions.iter().find(|action| action.id == verb).unwrap_or_else(|| panic!("'{verb}' is not catalogued"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "'{verb}' would be dispatch-dead");
    }
    assert_eq!(BITMAP_STROKE_COALESCE_KEY, "wfc-bitmap-stroke");
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
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "wfc-bitmap-fill".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32] };
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
    let finished_pixels = crate::inferences::solve_with_job(&snapshot).expect("oracle");
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
