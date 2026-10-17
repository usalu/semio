//! ✏️ Bitmap editor — two windows over one document: the authored sample on the left, the inferred
//! collapse on the right. Every command maps 1:1 onto a real `BitmapMutation` builder from the
//! schema tree; the exceptions are named and each has a reason.
//!
//! **`SetActiveColor` writes the WINDOW CONFIG, not the document.** Which palette entry a stroke
//! paints is a property of the pane, not of the artifact — two input panes over one document hold
//! two brushes.
//!
//! **`PaintStroke` is the brush TOOL, not a document verb.** A pointer stroke across a bitmap is hundreds of
//! samples; it runs through the input window's brush tool machine (`🪛️utilities/🖌️brush`), whose open
//! `ToolTransaction` lives in the window TRANSIENT between dispatches and holds ONE parametric
//! `paint-input-stroke` leaf — the sampled cells and the colour. The commit publishes that leaf as ONE edit
//! stamped with the stroke's `TransactionRef`; a cancel leaves zero trace; tool state is never config or history.
//!
//! **`Solve` starts the fill tool run.** The collapse is an inference; the snapshot has no field to
//! hold it. The fill tool publishes partial pixels on each tick; `commit-fill-solve` writes
//! `SetSolve` only when the run commits. Abort leaves the transient untouched.
//!
//! **`SetActiveExample` loads a document without a phantom edit (a `LoadDocument` effect, host-only).** It is the verb the shell's
//! navbar picker AND its automatic boot announcement dispatch, so an app that does not declare it has
//! a dead picker and a boot-time `undeclared-action` refusal in every session.
//!
//! **Every verb rides ONE app-owned retained tool factory.** `qualified_tool_proof` refuses any typed
//! command that has no exact app-owned decoder/reducer (`interactive-job.missing-owned-reducer`), and
//! `dispatch_action` resolves EVERY host action through `command_from_action`, so without the bridge
//! and the factory below the whole action roster is dispatch-dead in the live shell no matter how it
//! is classified.

use crate::editor::bitmap::commands::{pin_solution, set_active_example};
use crate::editor::bitmap::modes::edit;
use crate::editor::bitmap::modes::edit::tools::fill as fill_tool;
use crate::editor::bitmap::modes::edit::windows::input::utilities::brush::{bitmap_brush_pointer_stroke, BitmapBrush, BitmapBrushPointer, BrushToolRequest, BITMAP_PAINT_STROKE_VERB};
use semio_framework_tool_machine::{GesturePhase, ToolAbortReason};
use crate::editor::bitmap::modes::edit::windows::{input, output};
use crate::editor::bitmap::transient::{BitmapTransient, BitmapTransientMutation, SetSolve};
use crate::mutations::{add_palette_color, change_model, change_palette_color, change_seed, pin_pixel, remove_palette_color, resize_input, resize_output, set_input_pixels, unpin_pixel, BitmapStrokePoint};
use crate::schema::snapshot::BitmapColor;
use crate::{BitmapMutation, BitmapSnapshot, WFC_BITMAP_DIALECT, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::PresenceView;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::TransientView;
use semio_framework_plugin::WindowConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_2d::compute::EngineHandles;

#[path = "🎛️command-and-action-bridge/🦀️.rs"]
mod command_and_action_bridge;
pub use command_and_action_bridge::*;

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
pub use store_preparation::*;

#[path = "📝️artifact-editor/🦀️.rs"]
mod artifact_editor;
pub use artifact_editor::*;

//#region 🔖️Manifest
/// 🎬️ App-level, not window-owned: `try_build_definition` copies an unowned action onto EVERY
/// window kind, which is exactly what the shell's app-wide undeclared-action gate asks for —
/// the navbar picker and the boot announcement dispatch it from whichever pane is focused.
pub fn create_bitmap_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC_BITMAP_DIALECT)
        .document(["semio", "wfc", "bitmap"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("image")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_BITMAP_MODE_EDIT)
        .utility(input::utilities::brush::definition())
        .window_kind_def(input::definition())
        .window_kind_def(output::definition())
        .default_layout(edit::layout())
        .tool(fill_tool::definition())
        .mode_tools(edit::WFC_BITMAP_MODE_EDIT, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
        .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Load Example", "Beispiel laden"), ActionKind::Mutation, "panel-left"))
        .action_destructive("setActiveExample")
        .action_destructive("remove-palette-color")
        .action_args(
            "setActiveExample",
            vec![ActionArgDef::select(
                "exampleId",
                LocalizedLabel::native("Example", "Beispiel"),
                vec![
                    ActionArgOption::new(crate::examples::rooms_16::ID, crate::examples::rooms_16::label()),
                    ActionArgOption::new(crate::examples::flowers_24::ID, crate::examples::flowers_24::label()),
                ],
            )
            .required()
            .default_value(&set_active_example::BITMAP_EXAMPLE_BOOT_ID)],
        )
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition::new(pin_solution::PIN_SOLUTION_ACTION_ID, LocalizedLabel::native("Pin Solution", "Lösung anheften"), ActionKind::Mutation, "check"))
        .action_destructive(pin_solution::PIN_SOLUTION_ACTION_ID)
        .action_args(
            pin_solution::PIN_SOLUTION_ACTION_ID,
            vec![
                ActionArgDef::text("pixels", LocalizedLabel::native("Solved Pixels", "Gelöste Pixel")).required(),
                ActionArgDef::toggle("contradiction", LocalizedLabel::native("Contradiction", "Widerspruch")).default_value(&false),
            ],
        )
        .action_interactive_job(pin_solution::PIN_SOLUTION_ACTION_ID, InteractiveJobClassification::Migrated)
        .action_describe("set-input-pixels", LocalizedLabel::native("Overwrites a rectangle of the input sample at x, y with the given palette indices (base64, one byte per pixel, row by row); the solve learns its patterns from this sample.", "Überschreibt ein Rechteck des Eingabemusters an x, y mit den angegebenen Palettenindizes (Base64, ein Byte pro Pixel, zeilenweise); der Löser lernt seine Muster aus diesem Beispiel."))
        .action_describe("resize-input", LocalizedLabel::native("Resizes the input sample to the given width and height; growing pads the new right and bottom margin with palette colour 0, shrinking drops the pixels outside.", "Ändert die Größe des Eingabemusters; beim Vergrößern wird der neue rechte und untere Rand mit Palettenfarbe 0 gefüllt, beim Verkleinern fallen die Pixel außerhalb weg."))
        .action_describe("add-palette-color", LocalizedLabel::native("Inserts an RGBA colour into the palette at the given index and renumbers every pixel and pin that used that index or a higher one.", "Fügt der Palette an der angegebenen Position eine RGBA-Farbe hinzu und nummeriert alle Pixel und Anheftungen um, die diesen oder einen höheren Index verwenden."))
        .action_describe("change-palette-color", LocalizedLabel::native("Recolours one palette entry in place to the given RGBA value; every pixel and pin using that index changes colour with it.", "Färbt einen Paletteneintrag direkt auf den angegebenen RGBA-Wert um; alle Pixel und Anheftungen mit diesem Index ändern ihre Farbe mit."))
        .action_describe("remove-palette-color", LocalizedLabel::native("Removes one palette colour and renumbers the entries above it; refused while any pixel or pin still uses the colour.", "Entfernt eine Palettenfarbe und nummeriert die darüberliegenden Einträge um; wird abgelehnt, solange noch ein Pixel oder eine Anheftung die Farbe verwendet."))
        .action_describe("set-active-color", LocalizedLabel::native("Arms the palette colour the input window's brush paints with; only this window's brush state changes, not the document.", "Wählt die Palettenfarbe, mit der der Pinsel des Eingabefensters malt; nur der Pinselzustand dieses Fensters ändert sich, nicht das Dokument."))
        .action_describe("change-model", LocalizedLabel::native("Sets the overlapping model the solve learns from the sample: pattern size N, how many of the sample's 8 rotations and reflections to use, whether the sample wraps around, and an optional ground colour.", "Legt das überlappende Modell fest, das der Löser aus dem Muster lernt: Mustergröße N, wie viele der 8 Drehungen und Spiegelungen genutzt werden, ob das Muster umläuft, und eine optionale Bodenfarbe."))
        .action_describe("change-seed", LocalizedLabel::native("Sets the random seed the solve starts from; the same seed and model always produce the same output bitmap.", "Legt den Zufallsstartwert des Lösers fest; derselbe Startwert und dasselbe Modell erzeugen immer dieselbe Ausgabebitmap."))
        .action_describe("solve", LocalizedLabel::native("Starts the fill tool run that generates the output bitmap from the input sample; the result stays a preview until it is pinned, so the document does not change.", "Startet den Füll-Werkzeuglauf, der aus dem Eingabemuster die Ausgabebitmap erzeugt; das Ergebnis bleibt eine Vorschau, bis es angeheftet wird, das Dokument ändert sich nicht."))
        .action_describe("commit-fill-solve", LocalizedLabel::native("Hands a finished fill run's pixels and contradiction flag to the output window's preview; the document is not changed, Pin Solution keeps a result.", "Übergibt die Pixel und das Widerspruchskennzeichen eines fertigen Füll-Laufs an die Vorschau des Ausgabefensters; das Dokument ändert sich nicht, Lösung anheften behält ein Ergebnis."))
        .action_describe("resize-output", LocalizedLabel::native("Sets the width, height and wrap-around of the bitmap the solve generates; pins outside the new size are removed.", "Legt Breite, Höhe und Umlauf der Bitmap fest, die der Löser erzeugt; Anheftungen außerhalb der neuen Größe werden entfernt."))
        .action_describe("pin-pixel", LocalizedLabel::native("Fixes one output pixel at x, y to a palette colour before solving; the solve must keep it.", "Legt ein Ausgabepixel an x, y vor dem Lösen auf eine Palettenfarbe fest; der Löser muss sie beibehalten."))
        .action_describe("unpin-pixel", LocalizedLabel::native("Releases the pin on one output pixel at x, y so the solve chooses its colour again; refused when the pixel carries no pin.", "Löst die Anheftung eines Ausgabepixels an x, y, sodass der Löser dessen Farbe wieder selbst wählt; wird abgelehnt, wenn das Pixel nicht angeheftet ist."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document (sample, palette, model, output size, seed and pins) with one of the plugin's bundled bitmap examples, by example id.", "Ersetzt das gesamte Dokument (Muster, Palette, Modell, Ausgabegröße, Startwert und Anheftungen) durch eines der mitgelieferten Bitmap-Beispiele, anhand der Beispiel-Id."))
        .action_describe("pin-solution", LocalizedLabel::native("Makes a finished solve durable by pinning every output pixel to the colour the solve chose, replacing any pin that disagrees; a contradiction is refused.", "Macht eine fertige Lösung dauerhaft, indem jedes Ausgabepixel auf die vom Löser gewählte Farbe angeheftet wird; abweichende Anheftungen werden ersetzt, ein Widerspruch wird abgelehnt."))
        .action_describe("paint-stroke", LocalizedLabel::native("Paints one brush stroke on the input sample: the cells at the given points, joined by straight lines of cells, take the given palette colour (else the input window's armed colour); cells outside the sample are skipped. The whole stroke is one undoable edit whose points and colour can be edited in history.", "Malt einen Pinselstrich auf das Eingabemuster: Die Zellen an den angegebenen Punkten, verbunden durch gerade Zellenlinien, erhalten die angegebene Palettenfarbe (sonst die gewählte Farbe des Eingabefensters); Zellen außerhalb des Musters werden übersprungen. Der ganze Strich ist ein rückgängig machbarer Schritt, dessen Punkte und Farbe im Verlauf bearbeitet werden können."))
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests

//#endregion 🧪️Tests
