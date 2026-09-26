//! ✏️ Bitmap editor — two windows over one document: the authored sample on the left, the inferred
//! collapse on the right. Every command maps 1:1 onto a real `BitmapMutation` builder from the
//! schema tree; the exceptions are named and each has a reason.
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
//! **`Solve` starts the fill tool run.** The collapse is an inference; the snapshot has no field to
//! hold it. The fill tool publishes partial pixels on each tick; `commit-fill-solve` writes
//! `SetSolve` only when the run commits. Abort leaves the transient untouched.
//!
//! **`SetActiveExample` replaces the document without a phantom edit.** It is the verb the shell's
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
use crate::editor::bitmap::modes::edit::windows::{input, output};
use crate::editor::bitmap::transient::{BitmapTransient, BitmapTransientMutation, SetSolve};
use crate::mutations::{add_palette_color, change_model, change_palette_color, change_seed, pin_pixel, remove_palette_color, resize_input, resize_output, set_input_pixels, unpin_pixel};
use crate::schema::snapshot::BitmapColor;
use crate::{BitmapMutation, BitmapSnapshot, WFC_BITMAP_DIALECT, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, EphemeralEmit, Fault, InteractiveJobClassification, Label, LocalizedLabel, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, PresenceView,
    ToolRef, TransientView, WindowConfigMutation,
};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

/// 🖌️ The one coalesce key every mid-drag config write carries, so the whole gesture is a single
/// edit in the pane's own ledger.
pub const BITMAP_STROKE_COALESCE_KEY: &str = "wfc-bitmap-stroke";

/// 🪪️ This editor's controller id — the address every retained tool key is qualified under.
pub const BITMAP_EDITOR_CONTROLLER_ID: &str = "s.wfc.bitmap@1/*#editor";

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `BitmapMutation` kind an editor UI
/// can trigger, plus the non-document verbs this file's own doc comment accounts for. Row order IS
/// the binary variant ordinal: appending is safe, reordering is a wire-format break.
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
    #[dsl(key = "commit-fill-solve")]
    CommitFillSolve { pixels: String, contradiction: bool, width: u32, height: u32 },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "pin-solution")]
    PinSolution { pixels: String, contradiction: bool },
}

impl protocol::OpBinary for BitmapEditorCommand {
    /// 🎯️ The typed tool ids this command channel generates. WITHOUT this the trait default
    /// (`["typed-command"]`) makes `validate_tool_job_rows` compute an EMPTY expected set and every
    /// bounded-first-step proof below is rejected with `interactive-job.catalog-authority`.
    const TOOL_JOB_IDS: &'static [&'static str] = BITMAP_TOOL_IDS;
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

/// 🏷️ Every manifest action id this editor dispatches, in `BitmapEditorCommand` row order. The
/// `#[dsl(key)]` spelling IS the manifest action id for every document/gesture verb; only
/// `setActiveExample` is a framework-owned camelCase id (the shell's navbar picker dispatches that
/// exact string).
pub const BITMAP_TOOL_IDS: &[&str] = &[
    "change-seed",
    "resize-input",
    "set-input-pixels",
    "add-palette-color",
    "change-palette-color",
    "remove-palette-color",
    "resize-output",
    "change-model",
    "pin-pixel",
    "unpin-pixel",
    "set-active-color",
    "stroke-begin",
    "stroke-extend",
    "stroke-commit",
    "solve",
    "commit-fill-solve",
    "setActiveExample",
    "pin-solution",
];

/// 🏷️ The manifest action id one command was declared under — command-log labelling and the
/// registry's kind-discipline lookup both read it.
pub fn bitmap_command_id(command: &BitmapEditorCommand) -> &'static str {
    match command {
        BitmapEditorCommand::ChangeSeed { .. } => "change-seed",
        BitmapEditorCommand::ResizeInput { .. } => "resize-input",
        BitmapEditorCommand::SetInputPixels { .. } => "set-input-pixels",
        BitmapEditorCommand::AddPaletteColor { .. } => "add-palette-color",
        BitmapEditorCommand::ChangePaletteColor { .. } => "change-palette-color",
        BitmapEditorCommand::RemovePaletteColor { .. } => "remove-palette-color",
        BitmapEditorCommand::ResizeOutput { .. } => "resize-output",
        BitmapEditorCommand::ChangeModel { .. } => "change-model",
        BitmapEditorCommand::PinPixel { .. } => "pin-pixel",
        BitmapEditorCommand::UnpinPixel { .. } => "unpin-pixel",
        BitmapEditorCommand::SetActiveColor { .. } => "set-active-color",
        BitmapEditorCommand::StrokeBegin { .. } => "stroke-begin",
        BitmapEditorCommand::StrokeExtend { .. } => "stroke-extend",
        BitmapEditorCommand::StrokeCommit => "stroke-commit",
        BitmapEditorCommand::Solve => "solve",
        BitmapEditorCommand::CommitFillSolve { .. } => "commit-fill-solve",
        BitmapEditorCommand::SetActiveExample { .. } => "setActiveExample",
        BitmapEditorCommand::PinSolution { .. } => "pin-solution",
    }
}
//#endregion 🔖️Command

//#region 🌉️ActionBridge
/// 🌉️ Host args → typed command. `VcsArtifactApp::dispatch_action` resolves EVERY host action
/// through this function, and the trait default answers a hard `app.command.unsupported` fault — so
/// an editor without this bridge has a complete, correctly classified, completely dead action roster.
mod args_bridge {
    use super::{BitmapEditorCommand, Fault};
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn field<'a>(args: Option<&'a dsl::DslValue>, key: &str) -> Option<&'a dsl::DslValue> {
        args?.get(key)
    }

    /// 🔤️ A string arg — also accepts a number, so a select whose option ids are numeric reads the
    /// same whether the host sends `"3"` or `3`.
    fn text(args: Option<&dsl::DslValue>, key: &str) -> Option<String> {
        let value = field(args, key)?;
        value.as_str().map(str::to_owned).or_else(|| value.as_u64().map(|number| number.to_string())).or_else(|| value.as_i64().map(|number| number.to_string())).or_else(|| value.as_f64().map(|number| number.to_string()))
    }

    /// 🔢️ A numeric arg — also accepts a numeric string, which is how select-sourced numbers arrive.
    fn number(args: Option<&dsl::DslValue>, key: &str) -> Option<f64> {
        let value = field(args, key)?;
        value.as_f64().or_else(|| value.as_str()?.parse().ok())
    }

    fn flag(args: Option<&dsl::DslValue>, key: &str) -> Option<bool> {
        let value = field(args, key)?;
        value.as_bool().or_else(|| value.as_str()?.parse().ok())
    }

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("wfc bitmap has no command for action '{action}'"))
    }

    pub fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<BitmapEditorCommand, Fault> {
        let u32_or = |key: &str, fallback: u32| number(args, key).map_or(fallback, |value| value.max(0.0) as u32);
        let u64_or = |key: &str, fallback: u64| number(args, key).map_or(fallback, |value| value.max(0.0) as u64);
        let usize_or = |key: &str, fallback: usize| number(args, key).map_or(fallback, |value| value.max(0.0) as usize);
        let bool_or = |key: &str, fallback: bool| flag(args, key).unwrap_or(fallback);
        Ok(match action {
            "change-seed" => BitmapEditorCommand::ChangeSeed { seed: u64_or("seed", 1) },
            "resize-input" => BitmapEditorCommand::ResizeInput { width: u32_or("width", 16), height: u32_or("height", 16) },
            "set-input-pixels" => BitmapEditorCommand::SetInputPixels {
                x: u32_or("x", 0),
                y: u32_or("y", 0),
                width: u32_or("width", 1),
                height: u32_or("height", 1),
                pixels: text(args, "pixels").unwrap_or_default(),
            },
            "add-palette-color" => BitmapEditorCommand::AddPaletteColor { index: usize_or("index", 0), r: u32_or("r", 0), g: u32_or("g", 0), b: u32_or("b", 0), a: u32_or("a", 255) },
            "change-palette-color" => BitmapEditorCommand::ChangePaletteColor { index: usize_or("index", 0), r: u32_or("r", 0), g: u32_or("g", 0), b: u32_or("b", 0), a: u32_or("a", 255) },
            "remove-palette-color" => BitmapEditorCommand::RemovePaletteColor { index: usize_or("index", 0) },
            "resize-output" => BitmapEditorCommand::ResizeOutput { width: u32_or("width", 24), height: u32_or("height", 24), periodic: bool_or("periodic", true) },
            "change-model" => BitmapEditorCommand::ChangeModel {
                pattern_size: u32_or("patternSize", 3),
                symmetry: u32_or("symmetry", 8),
                periodic_input: bool_or("periodicInput", true),
                ground: number(args, "ground").map(|value| value.max(0.0) as u32),
            },
            "pin-pixel" => BitmapEditorCommand::PinPixel { x: u32_or("x", 0), y: u32_or("y", 0), color: u32_or("color", 0) },
            "unpin-pixel" => BitmapEditorCommand::UnpinPixel { x: u32_or("x", 0), y: u32_or("y", 0) },
            "set-active-color" => BitmapEditorCommand::SetActiveColor { index: u32_or("index", 0) },
            "stroke-begin" => BitmapEditorCommand::StrokeBegin { x: u32_or("x", 0), y: u32_or("y", 0) },
            "stroke-extend" => BitmapEditorCommand::StrokeExtend { x: u32_or("x", 0), y: u32_or("y", 0) },
            "stroke-commit" => BitmapEditorCommand::StrokeCommit,
            "solve" => BitmapEditorCommand::Solve,
            "setActiveExample" => BitmapEditorCommand::SetActiveExample { example_id: text(args, "exampleId").unwrap_or_else(|| super::set_active_example::BITMAP_EXAMPLE_BOOT_ID.to_string()) },
            "commit-fill-solve" => BitmapEditorCommand::CommitFillSolve {
                pixels: text(args, "pixels").unwrap_or_default(),
                contradiction: bool_or("contradiction", false),
                width: u32_or("width", 0),
                height: u32_or("height", 0),
            },
            "pin-solution" => BitmapEditorCommand::PinSolution { pixels: text(args, "pixels").unwrap_or_default(), contradiction: bool_or("contradiction", false) },
            _ => return Err(unknown(action)),
        })
    }
}
//#endregion 🌉️ActionBridge

//#region 🧵️RetainedCommands
/// 🧬️ The payload schema id every bitmap retained command job is admitted under.
const BITMAP_RETAINED_COMMAND_SCHEMA: &str = "s.wfc.bitmap/v1.tool-command.v1";
/// 🎒️ Wire ceiling for one bitmap tool dispatch. The largest payload is a `set-input-pixels` region
/// covering a whole 512-edge sample's row band; a full 128 × 128 repaint is 22 KiB of base64.
const BITMAP_RETAINED_RAW_BYTES: usize = 65_536;
/// 📬️ Admission envelope for one published document edit — `setActiveExample` publishes the widest
/// one, a whole re-laid-out sample buffer.
const BITMAP_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 262_144;
/// 🧮️ Durable items one dispatch may fold. Every ordinary verb folds ONE; `setActiveExample` folds a
/// whole declared-state diff (pins released, palette grown, buffer rewritten, palette shrunk, output,
/// model, seed, pins restored), so the ceiling is the example roster's widest replay, not one row.
const BITMAP_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🚦️ Per-tool publication lanes, read straight off each command's own emit.
const BITMAP_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    artifact_route("change-seed"),
    artifact_route("resize-input"),
    artifact_route("set-input-pixels"),
    artifact_route("add-palette-color"),
    artifact_route("change-palette-color"),
    artifact_route("remove-palette-color"),
    artifact_route("resize-output"),
    artifact_route("change-model"),
    artifact_route("pin-pixel"),
    artifact_route("unpin-pixel"),
    window_config_route("set-active-color"),
    window_config_route("stroke-begin"),
    window_config_route("stroke-extend"),
    ArtifactToolPublicationContract { tool_id: "stroke-commit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "solve", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "commit-fill-solve", lanes: &[ArtifactToolPublicationLane::Transient] },
    artifact_route("setActiveExample"),
    artifact_route("pin-solution"),
];

const fn artifact_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact] }
}

const fn window_config_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::WindowConfig] }
}

fn bitmap_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BITMAP_RETAINED_RAW_BYTES, BITMAP_RETAINED_WORK_ITEMS, 1, BITMAP_ARTIFACT_MUTATION_MAXIMUM_BYTES, 7_500)
}

/// 🧵️ The one retained work every bitmap verb rides. `Solve` is the only arm that answers
/// `CompleteWithEphemeral`: it runs the artifact's own headless inference adapter and hands the
/// collapse to the transient lane the output window renders from.
struct BitmapCommandWork {
    tool_id: &'static str,
    consumed: bool,
}

impl ArtifactCommandWork<EditorApp<BitmapEditor>> for BitmapCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &BitmapEditorCommand,
        _snapshot: &BitmapSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BitmapEditor>>>,
    ) -> Option<usize> {
        BITMAP_TOOL_IDS.contains(&bitmap_command_id(command)).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BitmapEditor>>) -> Result<ArtifactCommandWorkStep<EditorApp<BitmapEditor>>, Fault> {
        if self.consumed {
            return Err(Fault::from("wfc-bitmap-retained-work-repeated"));
        }
        self.consumed = true;
        if bitmap_command_id(input.command) != self.tool_id {
            return Err(Fault::from("wfc-bitmap-retained-route-mismatch"));
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let view_state = input.context.and_then(|context| context.view_state.as_ref());
        let emit = BitmapEditor::dispatch(input.command, &doc, &cfg, view_state)?;
        match input.command {
            BitmapEditorCommand::Solve => Ok(ArtifactCommandWorkStep::Complete(Emit {
                effects: vec![fill_tool::start_fill_effect()],
                description: Some("Solve".to_string()),
                ui_scope: semio_framework::kernel::UiDirtyScope::Full,
                ..Default::default()
            })),
            BitmapEditorCommand::CommitFillSolve { pixels, contradiction, width, height } => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: EphemeralEmit {
                    transient: vec![BitmapTransientMutation::SetSolve(SetSolve {
                        output_pixels: if *contradiction || pixels.is_empty() { None } else { Some(pixels.clone()) },
                        contradiction: *contradiction,
                        output_width: *width,
                        output_height: *height,
                    })],
                    ..Default::default()
                },
            }),
            _ => Ok(ArtifactCommandWorkStep::Complete(emit)),
        }
    }
}

/// 🏭️ The app-owned bounded tool-job factory. `qualified_tool_proof` refuses every typed command
/// that resolves to anything else, so this registration is what makes the whole roster reachable.
pub struct BitmapCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BitmapCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BITMAP_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }

    fn register(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<BitmapEditor>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Self::new(&controller))
    }
}

impl semio_framework::ToolJobFactory for BitmapCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BitmapEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BitmapEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BITMAP_RETAINED_COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        bitmap_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > BITMAP_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded wfc bitmap command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BitmapCommandJobFactory {
    type Owner = EditorApp<BitmapEditor>;
    const TOOL_IDS: &'static [&'static str] = BITMAP_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_BITMAP_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BITMAP_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
/// 🏷️ Id prefix every staged bitmap document edit is minted under.
const BITMAP_STORE_PREFIX: &str = "wfc-bitmap-artifact-retained";
/// 🧺️ Inverse rows `resize-output` may declare: its own prior spec plus ONE `pin-pixel` per pin the
/// shrink cascaded away. `preflight` never sees the base, so this is the structural upper bound the
/// lane proves from the mutation alone — a document that pins more than this cannot shrink its output
/// in one gesture, which is honest refusal rather than a half-applied cascade.
const BITMAP_PIN_CASCADE_ROWS: usize = 1_024;

/// 🧺️ Staged inverse rows one bitmap mutation may fold. Two of this artifact's ten verbs are NOT
/// point-invertible — `resize-input` answers `resize back` PLUS a full-buffer restore, and
/// `resize-output` answers its prior spec PLUS the pins it cascaded — so the SDK's generic
/// `for_one_invertible_item` (a flat two rows) under-declares them and `ArtifactStore::fold_batch_item`
/// refuses the candidate with `batched item candidate failed its exact fixed fold contract`.
fn bitmap_inverse_rows(mutation: &BitmapMutation) -> usize {
    match mutation {
        BitmapMutation::ResizeInput(_) => 2,
        BitmapMutation::ResizeOutput(_) => 1 + BITMAP_PIN_CASCADE_ROWS,
        _ => 1,
    }
}

fn bitmap_mutation_retained_bytes(mutation: &BitmapMutation) -> Result<usize, String> {
    protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| format!("{BITMAP_STORE_PREFIX}-mutation-encode-failed"))
}

fn bitmap_one_item_footprint(mutation: &BitmapMutation, maximum_bytes: usize) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = bitmap_mutation_retained_bytes(mutation)?;
    if retained_bytes > maximum_bytes {
        return Err(format!("{BITMAP_STORE_PREFIX}-mutation-envelope"));
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_one_item(bitmap_inverse_rows(mutation), retained_bytes))
}

/// 🏭️ The bitmap document lane's own one-item publication authority.
///
/// It is not the SDK's `bounded_config_store_one_item_preparation_factory` for two reasons, both
/// measured live on the playground: that factory declares every mutation point-invertible (above), and
/// it CONSUMES its mutation owner before the fallible steps, so the store's second admission retry
/// reports `…-mutation-owner-missing` and the real first failure is never printed anywhere.
pub struct BitmapOneItemPreparationFactory {
    maximum_bytes: usize,
}

struct BitmapOneItemPreparation {
    maximum_bytes: usize,
    base: Option<store::SnapshotRead<BitmapSnapshot>>,
    mutation: Option<BitmapMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<BitmapSnapshot, BitmapMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    failure: Option<String>,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<BitmapSnapshot, BitmapMutation> for BitmapOneItemPreparationFactory {
    fn preflight(&self, mutation: &BitmapMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err(format!("{BITMAP_STORE_PREFIX}-lane-or-description-envelope"));
        }
        bitmap_one_item_footprint(mutation, self.maximum_bytes)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<BitmapSnapshot, BitmapMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<BitmapSnapshot, BitmapMutation>>, store::ArtifactStoreOneItemPreparationRequest<BitmapSnapshot, BitmapMutation>> {
        let retained_bytes = bitmap_mutation_retained_bytes(&request.mutation).unwrap_or(self.maximum_bytes.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > self.maximum_bytes
        {
            return Err(request);
        }
        Ok(Box::new(BitmapOneItemPreparation {
            maximum_bytes: self.maximum_bytes,
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            failure: None,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<BitmapSnapshot, BitmapMutation> for BitmapOneItemPreparation {
    /// 📬️ Stages ONE document edit. Every fallible step runs while the mutation is still OWNED here,
    /// and the first failure is remembered, so the store's retry reports the real cause instead of an
    /// owner-missing artefact of the first attempt.
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.stage() {
            Ok(step) => Ok(step),
            Err(failure) => {
                self.failure = Some(failure.clone());
                Err(failure)
            }
        }
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<BitmapSnapshot, BitmapMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<BitmapSnapshot, BitmapMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() || self.failure.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(format!("{BITMAP_STORE_PREFIX}-base-retirement-rejected"));
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none() && self.failure.is_none()
    }
}

impl BitmapOneItemPreparation {
    fn stage(&mut self) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        let base = self.base.as_ref().ok_or_else(|| format!("{BITMAP_STORE_PREFIX}-base-owner-missing"))?;
        let mutation = self.mutation.as_ref().ok_or_else(|| format!("{BITMAP_STORE_PREFIX}-mutation-owner-missing"))?;
        bitmap_one_item_footprint(mutation, self.maximum_bytes)?;
        let inverse = protocol::Mutation::inverse(mutation, base.get());
        let diff = protocol::Mutation::diff(mutation, base.get()).into_parts().0;
        let post = protocol::MutationDiff::apply(&diff, base.get()).map_err(|error| format!("{BITMAP_STORE_PREFIX}-diff-apply-failed:{error}"))?;
        let authority = self.authority.as_ref().ok_or_else(|| format!("{BITMAP_STORE_PREFIX}-authority-missing"))?;
        let forward = self.mutation.take().expect("the staged mutation was read above");
        let edit = bitmap_next_edit(forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }
}

/// 🧬️ One staged `protocol::Edit` for the bitmap document lane.
fn bitmap_next_edit(forward: BitmapMutation, inverse: Vec<BitmapMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<BitmapMutation> {
    let id = format!("{BITMAP_STORE_PREFIX}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}
//#endregion 📬️StorePreparation

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

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    /// 🗃️ The bounded retirement catalog every store lane is released THROUGH: a store built without
    /// owners answers `artifact store has no owner-supplied bounded disposer` the moment the close
    /// ladder reaches it, so the owners and the disposers below are one declaration in two halves.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    /// ♻️ The instance close ladder walks one owned store lane per stage and faults the whole close
    /// with `interactive-job.close-owned-disposer-missing` the moment a lane answers `None`, so an
    /// editor must declare how EVERY store it types is released, not only the ones it edits.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn initial_snapshot() -> BitmapSnapshot {
        crate::examples::rooms_16::snapshot()
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<input::config::BitmapInputWindowConfigOwner>()?;
        registry.register::<output::config::BitmapOutputWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        BitmapCommandJobFactory::register(registry)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<BitmapEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.bitmap@1/*#editor",
        artifact_schema: "s.wfc.bitmap",
        factory: "BitmapCommandJobFactory",
        factory_type: BitmapCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "change-seed",
            "resize-input",
            "set-input-pixels",
            "add-palette-color",
            "change-palette-color",
            "remove-palette-color",
            "resize-output",
            "change-model",
            "pin-pixel",
            "unpin-pixel",
            "set-active-color",
            "stroke-begin",
            "stroke-extend",
            "stroke-commit",
            "solve",
            "commit-fill-solve",
            "setActiveExample",
            "pin-solution"
        ]
    }

    /// 📬️ Publication authority for the document lane. Without it every `Artifact`-lane verb is
    /// refused closed with `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(BitmapOneItemPreparationFactory { maximum_bytes: BITMAP_ARTIFACT_MUTATION_MAXIMUM_BYTES }))
    }

    /// 🫧️ Publication + retirement authority for the solve cache the output window renders.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BITMAP_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = bitmap_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::from("wfc-bitmap-command-tool-mismatch"));
        }
        let operation = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation,
                completion: request.completion,
            },
            bitmap_command_id,
            BITMAP_RETAINED_RAW_BYTES,
            BITMAP_RETAINED_WORK_ITEMS,
            Box::new(BitmapCommandWork { tool_id, consumed: false }),
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn command_id(command: &Self::Command) -> &'static str {
        bitmap_command_id(command)
    }

    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
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
        Self::dispatch(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_bodies(body_key, doc.snapshot, cfg, &BitmapTransient::default(), None)
    }

    /// 🧮️ The output window's real render path: the same bodies, but reading the solve cache the
    /// `Solve` command published on the transient lane instead of an empty one.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_bodies(body_key, doc.snapshot, cfg, transient.snapshot, doc.tool_run())
    }

    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        if request.tool_id != fill_tool::TOOL_ID || request.purpose != semio_framework_plugin::ToolRunJobPurpose::Run {
            return Ok(None);
        }
        Ok(Some(Box::new(fill_tool::BitmapFillRunJob::from_request(request)?)))
    }

    /// 🫧️ Retained through the tool factory instead: `ArtifactApp::ephemeral` is never called by the
    /// live dispatch path, so the solve is published from [`BitmapCommandWork::step`].
    fn ephemeral(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _presence: &PresenceView<'_, Self::Presence>,
        _transient: &TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        (Vec::new(), Vec::new())
    }
}

impl BitmapEditor {
    /// 🎯️ The pure dispatch every route shares — the ordinary `handle` and the retained work step
    /// both enter here, so a verb can never behave differently depending on which one ran it.
    /// 🖥️ `ui_scope` is the whole point of this arm: `Solve` publishes ONLY on the transient
    /// lane, and a transient publication carries no document revision for the shell to notice,
    /// so with the default scope the collapse landed in the store and the output window kept
    /// rendering the buffer it already had (measured: a black square after a successful solve).
    pub fn dispatch(command: &BitmapEditorCommand, doc: &ArtifactView<'_, BitmapSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: Option<&semio_framework_plugin::ViewModel>) -> Result<Emit<BitmapMutation>, Fault> {
        let Some((mutation, description)) = Self::command_mutation(command) else {
            return match command {
                BitmapEditorCommand::SetActiveColor { index } => Self::set_active_color(doc, cfg, view_state, *index),
                BitmapEditorCommand::StrokeBegin { x, y } => Self::accumulate_stroke(cfg, view_state, Some(*x), Some(*y), true),
                BitmapEditorCommand::StrokeExtend { x, y } => Self::accumulate_stroke(cfg, view_state, Some(*x), Some(*y), false),
                BitmapEditorCommand::StrokeCommit => Self::commit_stroke(doc, cfg, view_state),
                BitmapEditorCommand::SetActiveExample { example_id } => set_active_example::handle(&set_active_example::SetActiveExample { example_id: example_id.clone() }, doc),
                BitmapEditorCommand::PinSolution { pixels, contradiction } => pin_solution::handle(&pin_solution::PinSolution { pixels: pixels.clone(), contradiction: *contradiction }, doc),
                BitmapEditorCommand::Solve => Ok(Emit { effects: vec![fill_tool::start_fill_effect()], description: Some("Solve".to_string()), ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }),
                BitmapEditorCommand::CommitFillSolve { .. } => Ok(Emit { description: Some("Commit fill solve".to_string()), ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }),
                _ => Err(Fault::from("wfc-bitmap-command-unmapped")),
            };
        };
        Ok(Emit { artifact_mutations: vec![mutation], description: Some(description), ..Default::default() })
    }

    /// 🫧️ Runs the artifact's own headless inference adapter and packages the answer as the one
    /// transient write the output window reads.
    ///
    /// An UNSATISFIABLE problem answers a real transient with no pixels — that is the document's own
    /// answer and the pane says so. An inference that could not RUN is a FAULT, not a contradiction:
    /// folding the two together is what made a silently refused solve look like a black square with no
    /// diagnostic anywhere in the shell.
    pub fn solve_transient(snapshot: &BitmapSnapshot) -> Result<BitmapTransientMutation, Fault> {
        let (pixels, contradiction) = match crate::inferences::solve_with_job(snapshot) {
            Ok(commit) if !commit.contradiction && !commit.pixels.is_empty() => (Some(commit.pixels), false),
            Ok(_) => (None, true),
            Err(error) => return Err(Fault::from(format!("wfc-bitmap-solve-failed:{error}"))),
        };
        Ok(BitmapTransientMutation::SetSolve(SetSolve { output_pixels: pixels, contradiction, output_width: snapshot.output.width, output_height: snapshot.output.height }))
    }

    /// 🗺️ The command→mutation map, lifted out of `dispatch` so it is a pure, testable function: a
    /// command that answers `None` is one of the non-document verbs this file's own doc comment
    /// accounts for, and nothing else.
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
            BitmapEditorCommand::SetActiveColor { .. }
            | BitmapEditorCommand::StrokeBegin { .. }
            | BitmapEditorCommand::StrokeExtend { .. }
            | BitmapEditorCommand::StrokeCommit
            | BitmapEditorCommand::Solve
            | BitmapEditorCommand::CommitFillSolve { .. }
            | BitmapEditorCommand::SetActiveExample { .. }
            | BitmapEditorCommand::PinSolution { .. } => return None,
        })
    }

    fn render_bodies(body_key: &str, snapshot: &BitmapSnapshot, cfg: &ConfigView<'_, NoConfig>, transient: &BitmapTransient, tool_run: Option<&semio_framework_plugin::ToolRunView>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            input::BODY_KEY => input::render(snapshot, &input::config::current(cfg)).map(semio_framework_plugin::built_to_component_tree),
            output::BODY_KEY => output::render(snapshot, transient, &output::config::current(cfg), tool_run).map(semio_framework_plugin::built_to_component_tree),
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
        .window_kind_def(input::definition())
        .window_kind_def(output::definition())
        .default_layout(edit::layout())
        .tool(fill_tool::definition())
        .mode_tools(edit::WFC_BITMAP_MODE_EDIT, vec![semio_framework::io::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))])
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
        .action_audience("stroke-begin", semio_framework_plugin::CapabilityAudience::Input)
        .action_audience("stroke-extend", semio_framework_plugin::CapabilityAudience::Input)
        .action_audience("stroke-commit", semio_framework_plugin::CapabilityAudience::Input)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
