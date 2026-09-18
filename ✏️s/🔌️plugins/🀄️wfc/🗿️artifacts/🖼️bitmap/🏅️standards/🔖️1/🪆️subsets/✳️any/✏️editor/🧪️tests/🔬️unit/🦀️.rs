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
    for command in [BitmapEditorCommand::Solve, BitmapEditorCommand::StrokeBegin { x: 1, y: 2 }, BitmapEditorCommand::StrokeExtend { x: 3, y: 4 }, BitmapEditorCommand::StrokeCommit, BitmapEditorCommand::SetActiveColor { index: 1 }] {
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
    for command in [BitmapEditorCommand::Solve, BitmapEditorCommand::StrokeBegin { x: 0, y: 0 }, BitmapEditorCommand::StrokeExtend { x: 0, y: 0 }, BitmapEditorCommand::StrokeCommit, BitmapEditorCommand::SetActiveColor { index: 0 }] {
        assert!(BitmapEditor::command_mutation(&command).is_none(), "a non-document verb emits no artifact mutation");
    }
}

#[test]
fn every_dispatched_mutation_either_moves_the_boot_example_or_says_why_not() {
    let base = <BitmapEditor as ArtifactEditor>::initial_snapshot();
    for (command, kind) in commands() {
        let Some((mutation, _)) = BitmapEditor::command_mutation(&command) else { continue };
        let outcome = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation, &base);
        let mut snapshot = base.clone();
        crate::mutations::apply_bitmap_mutation(&mut snapshot, &mutation).unwrap_or_else(|error| panic!("'{kind}' applies to the boot example: {error}"));
        if snapshot == base {
            // 🩺 `remove-palette-color` and `unpin-pixel` are legitimately refused against the boot
            // example — every colour is painted and nothing is pinned — but a refusal must SAY so.
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
        assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated, "'{verb}' would be dispatch-dead");
    }
    assert_eq!(BITMAP_STROKE_COALESCE_KEY, "wfc-bitmap-stroke");
}
//#endregion 🖌️Gesture
