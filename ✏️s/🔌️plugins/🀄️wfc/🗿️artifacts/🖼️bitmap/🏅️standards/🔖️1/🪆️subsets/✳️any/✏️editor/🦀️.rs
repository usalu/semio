//! ✏️ Bitmap editor — two windows over one document: the authored sample on the left, the inferred
//! collapse on the right. Every command maps 1:1 onto a real `BitmapMutation` builder from the
//! schema tree; the three exceptions are named and each has a reason.
//!
//! **`SetActiveColor` writes the WINDOW CONFIG, not the document.** Which palette entry a stroke
//! paints is a property of the pane, not of the artifact — two input panes over one document hold
//! two brushes.
//!
//! **`StrokeBegin`/`StrokeExtend` emit NO DOCUMENT OPERATION.** A pointer drag across a bitmap is
//! hundreds of samples; one document mutation per sample is the coalesced-amend O(n²) trap the
//! framework's own per-frame-state law names. Mid-drag ticks only grow a bounding box in the pane's
//! own window config, every write carrying one coalesce key so the whole drag folds into a single
//! config edit — and `StrokeCommit` turns the settled box into exactly ONE `set-input-pixels`.
//!
//! **`Solve` writes the app TRANSIENT.** The collapse is an inference; the snapshot has no field to
//! hold it, and an edit that produced one would be a lie about what this document persists.

use crate::editor::bitmap::modes::edit;
use crate::editor::bitmap::modes::edit::windows::{input, output};
use crate::editor::bitmap::transient::{BitmapTransient, BitmapTransientMutation, SetSolve};
use crate::mutations::{add_palette_color, change_model, change_palette_color, change_seed, pin_pixel, remove_palette_color, resize_input, resize_output, set_input_pixels, unpin_pixel};
use crate::schema::snapshot::BitmapColor;
use crate::{BitmapMutation, BitmapSnapshot, WFC_BITMAP_DIALECT, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, Label, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, PresenceView, TransientView, WindowConfigMutation,
};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

/// 🖌️ The one coalesce key every mid-drag config write carries, so the whole gesture is a single
/// edit in the pane's own ledger.
pub const BITMAP_STROKE_COALESCE_KEY: &str = "wfc-bitmap-stroke";

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `BitmapMutation` kind an editor UI
/// can trigger, plus the four non-document verbs this file's own doc comment accounts for.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum BitmapEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "resize-input")]
    ResizeInput { width: u32, height: u32 },
    #[dsl(key = "set-input-pixels")]
    SetInputPixels { x: u32, y: u32, width: u32, height: u32, pixels: String },
    #[dsl(key = "add-palette-color")]
    AddPaletteColor { index: usize, r: u32, g: u32, b: u32, a: u32 },
    #[dsl(key = "change-palette-color")]
    ChangePaletteColor { index: usize, r: u32, g: u32, b: u32, a: u32 },
    #[dsl(key = "remove-palette-color")]
    RemovePaletteColor { index: usize },
    #[dsl(key = "resize-output")]
    ResizeOutput { width: u32, height: u32, periodic: bool },
    #[dsl(key = "change-model")]
    ChangeModel { pattern_size: u32, symmetry: u32, periodic_input: bool, ground: Option<u32> },
    #[dsl(key = "pin-pixel")]
    PinPixel { x: u32, y: u32, color: u32 },
    #[dsl(key = "unpin-pixel")]
    UnpinPixel { x: u32, y: u32 },
    #[dsl(key = "set-active-color")]
    SetActiveColor { index: u32 },
    #[dsl(key = "stroke-begin")]
    StrokeBegin { x: u32, y: u32 },
    #[dsl(key = "stroke-extend")]
    StrokeExtend { x: u32, y: u32 },
    #[dsl(key = "stroke-commit")]
    StrokeCommit,
    #[dsl(key = "solve")]
    Solve,
}

impl protocol::OpBinary for BitmapEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct BitmapEditor;

impl ArtifactEditor for BitmapEditor {
    type Snapshot = BitmapSnapshot;
    type Mutation = BitmapMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = BitmapTransient;
    type TransientMutation = BitmapTransientMutation;
    type Command = BitmapEditorCommand;

    const DIALECT: Dialect = WFC_BITMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_BITMAP_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> BitmapSnapshot {
        crate::examples::rooms_16::snapshot()
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<input::config::BitmapInputWindowConfigOwner>()?;
        registry.register::<output::config::BitmapOutputWindowConfigOwner>()
    }

    /// ✏️ Dispatches straight onto the real schema-tree mutation builders — one `BitmapMutation` per
    /// document command, no whole-document rewrite anywhere.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation>, Fault> {
        let Some((mutation, description)) = Self::command_mutation(command) else {
            return match command {
                BitmapEditorCommand::SetActiveColor { index } => Self::set_active_color(doc, cfg, view_state, *index),
                BitmapEditorCommand::StrokeBegin { x, y } => Self::accumulate_stroke(cfg, view_state, Some(*x), Some(*y), true),
                BitmapEditorCommand::StrokeExtend { x, y } => Self::accumulate_stroke(cfg, view_state, Some(*x), Some(*y), false),
                BitmapEditorCommand::StrokeCommit => Self::commit_stroke(doc, cfg, view_state),
                BitmapEditorCommand::Solve => Ok(Emit { description: Some("Solve".to_string()), ..Default::default() }),
                _ => Err(Fault::from("wfc-bitmap-command-unmapped")),
            };
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }

    /// 🫧️ `Solve` is the one command that writes the ephemeral lane: it runs the same headless
    /// adapter the `InferredField` bodies run and caches the answer for the output window. Every
    /// other command leaves the cache alone.
    fn ephemeral(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &PresenceView<'_, Self::Presence>,
        _transient: &TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        if !matches!(command, BitmapEditorCommand::Solve) {
            return (Vec::new(), Vec::new());
        }
        let solve = crate::inferences::solve_with_job(doc.snapshot);
        let (pixels, contradiction) = match solve {
            Ok(commit) if !commit.contradiction && !commit.pixels.is_empty() => (Some(commit.pixels), false),
            Ok(_) => (None, true),
            Err(_) => (None, true),
        };
        (Vec::new(), vec![BitmapTransientMutation::SetSolve(SetSolve { output_pixels: pixels, contradiction, output_width: doc.snapshot.output.width, output_height: doc.snapshot.output.height })])
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_bodies(body_key, doc.snapshot, cfg, &BitmapTransient::default())
    }

    /// 🧮️ The output window's real render path: the same bodies, but reading the solve cache the
    /// `Solve` command wrote instead of an empty one.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_bodies(body_key, doc.snapshot, cfg, transient.snapshot)
    }
}

impl BitmapEditor {
    /// 🗺️ The command→mutation map, lifted out of `handle` so it is a pure, testable function: a
    /// command that answers `None` is one of the three non-document verbs this file's own doc
    /// comment accounts for, and nothing else.
    pub fn command_mutation(command: &BitmapEditorCommand) -> Option<(BitmapMutation, String)> {
        Some(match command {
            BitmapEditorCommand::ChangeSeed { seed } => (change_seed(*seed), format!("Change seed to {seed}")),
            BitmapEditorCommand::ResizeInput { width, height } => (resize_input(*width, *height), format!("Resize input to {width}×{height}")),
            BitmapEditorCommand::SetInputPixels { x, y, width, height, pixels } => (set_input_pixels(*x, *y, *width, *height, pixels.clone()), format!("Paint {width}×{height} at ({x}, {y})")),
            BitmapEditorCommand::AddPaletteColor { index, r, g, b, a } => (add_palette_color(*index, BitmapColor { r: *r, g: *g, b: *b, a: *a }), format!("Add palette colour at {index}")),
            BitmapEditorCommand::ChangePaletteColor { index, r, g, b, a } => (change_palette_color(*index, BitmapColor { r: *r, g: *g, b: *b, a: *a }), format!("Change palette colour {index}")),
            BitmapEditorCommand::RemovePaletteColor { index } => (remove_palette_color(*index), format!("Remove palette colour {index}")),
            BitmapEditorCommand::ResizeOutput { width, height, periodic } => (resize_output(*width, *height, *periodic), format!("Resize output to {width}×{height}")),
            BitmapEditorCommand::ChangeModel { pattern_size, symmetry, periodic_input, ground } => (change_model(*pattern_size, *symmetry, *periodic_input, *ground), "Change model".to_string()),
            BitmapEditorCommand::PinPixel { x, y, color } => (pin_pixel(*x, *y, *color), format!("Pin ({x}, {y})")),
            BitmapEditorCommand::UnpinPixel { x, y } => (unpin_pixel(*x, *y), format!("Unpin ({x}, {y})")),
            BitmapEditorCommand::SetActiveColor { .. } | BitmapEditorCommand::StrokeBegin { .. } | BitmapEditorCommand::StrokeExtend { .. } | BitmapEditorCommand::StrokeCommit | BitmapEditorCommand::Solve => return None,
        })
    }

    fn render_bodies(body_key: &str, snapshot: &BitmapSnapshot, cfg: &ConfigView<'_, NoConfig>, transient: &BitmapTransient) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            input::BODY_KEY => input::render(snapshot, &input::config::current(cfg)).map(semio_framework_plugin::built_to_component_tree),
            output::BODY_KEY => output::render(snapshot, transient, &output::config::current(cfg)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🖌️ Grows the pane's in-flight stroke box by one sampled cell. ZERO document operations, and
    /// one coalesce key across the whole gesture so a long drag is one config edit, not hundreds.
    fn accumulate_stroke(cfg: &ConfigView<'_, NoConfig>, view_state: Option<&semio_framework_plugin::ViewModel>, x: Option<u32>, y: Option<u32>, restart: bool) -> Result<Emit<BitmapMutation>, Fault> {
        let view = view_state.ok_or_else(|| Fault::from("wfc-bitmap-view-state-required"))?;
        let mut config = input::config::current(cfg);
        let (x, y) = (x.ok_or_else(|| Fault::from("wfc-bitmap-stroke-sample-required"))?, y.ok_or_else(|| Fault::from("wfc-bitmap-stroke-sample-required"))?);
        config.stroke = Some(match config.stroke {
            Some(stroke) if !restart => stroke.extended(x, y),
            _ => input::config::BitmapStroke::at(x, y),
        });
        Ok(Emit { window_config_mutations: vec![input::config::addressed(view, config)?], coalesce_key: Some(BITMAP_STROKE_COALESCE_KEY.to_string()), description: Some("Paint stroke".to_string()), ..Default::default() })
    }

    /// 🖌️ Turns the settled box into exactly ONE `set-input-pixels` filled with the pane's armed
    /// colour, and clears the box in the same emit. A gesture that never began, or one whose box
    /// left the sample, is REFUSED — clipping it would produce an edit the user did not draw.
    fn commit_stroke(doc: &ArtifactView<'_, BitmapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<BitmapMutation>, Fault> {
        let view = view_state.ok_or_else(|| Fault::from("wfc-bitmap-view-state-required"))?;
        let mut config = input::config::current(cfg);
        let stroke = config.stroke.ok_or_else(|| Fault::from("wfc-bitmap-stroke-not-begun"))?;
        let mutation = Self::stroke_mutation(doc.snapshot, stroke, config.active_color)?;
        config.stroke = None;
        Ok(Emit {
            artifact_mutations: vec![mutation],
            window_config_mutations: vec![input::config::addressed(view, config)?],
            description: Some(format!("Paint {}×{} at ({}, {})", stroke.width(), stroke.height(), stroke.min_x, stroke.min_y)),
            ..Default::default()
        })
    }

    /// 🖌️ The pure half of a stroke commit: box plus armed colour in, ONE `set-input-pixels` out.
    /// Lifted out of `commit_stroke` so the gesture contract is testable without a mounted host.
    pub fn stroke_mutation(snapshot: &BitmapSnapshot, stroke: input::config::BitmapStroke, color: u32) -> Result<BitmapMutation, Fault> {
        if color as usize >= snapshot.input.palette.len() {
            return Err(Fault::from("wfc-bitmap-unknown-palette-color"));
        }
        if stroke.max_x >= snapshot.input.width || stroke.max_y >= snapshot.input.height {
            return Err(Fault::from("wfc-bitmap-stroke-outside-input"));
        }
        let index = u8::try_from(color).map_err(|_| Fault::from("wfc-bitmap-unknown-palette-color"))?;
        let cells = (stroke.width() as usize) * (stroke.height() as usize);
        Ok(set_input_pixels(stroke.min_x, stroke.min_y, stroke.width(), stroke.height(), crate::schema::snapshot::encode_base64(&vec![index; cells])))
    }

    /// 🎨️ The active brush colour is per-window-instance state: refused outright when the index is
    /// not a real palette entry, so a pane can never arm a colour the document does not have.
    fn set_active_color(doc: &ArtifactView<'_, BitmapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: Option<&semio_framework_plugin::ViewModel>, index: u32) -> Result<Emit<BitmapMutation>, Fault> {
        if index as usize >= doc.snapshot.input.palette.len() {
            return Err(Fault::from("wfc-bitmap-unknown-palette-color"));
        }
        let view = view_state.ok_or_else(|| Fault::from("wfc-bitmap-view-state-required"))?;
        let mut config = input::config::current(cfg);
        config.active_color = index;
        let mutation: WindowConfigMutation = input::config::addressed(view, config)?;
        Ok(Emit { window_config_mutations: vec![mutation], description: Some(format!("Set active colour {index}")), ..Default::default() })
    }
}
//#endregion 🔖️Editor

//#region 🔖️Manifest
pub fn create_bitmap_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC_BITMAP_DIALECT)
        .document(["semio", "wfc", "bitmap"])
        .icon_id("image")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_BITMAP_MODE_EDIT)
        .window_kind_def(input::definition())
        .window_kind_def(output::definition())
        .default_layout(edit::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
