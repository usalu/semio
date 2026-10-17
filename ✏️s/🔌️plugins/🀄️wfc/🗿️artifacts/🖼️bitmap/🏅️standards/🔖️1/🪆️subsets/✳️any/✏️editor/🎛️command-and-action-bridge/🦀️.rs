//! 🎛️ Command surface of the WFC bitmap editor: the command vocabulary with its wire ids and tool ids and the argument bridge that turns declared action arguments into commands. See [`super`].

use super::*;

/// 🪪️ This editor's controller id — the address every retained tool key is qualified under.
pub const BITMAP_EDITOR_CONTROLLER_ID: &str = "s.wfc.bitmap@1/*#editor";

//#region 🔖️Command
/// ✏️ The editor's typed command channel — one variant per real `BitmapMutation` kind an editor UI
/// can trigger, plus the non-document verbs this file's own doc comment accounts for. Row order IS
/// the binary variant ordinal: appending is safe, reordering is a wire-format break.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum BitmapEditorCommand {
    #[dsl(key = "change-seed")]
    ChangeSeed { seed: u64 },
    #[dsl(key = "resize-input")]
    ResizeInput { width: u32, height: u32 },
    #[dsl(key = "set-input-pixels")]
    SetInputPixels { x: u32, y: u32, width: u32, height: u32, #[value(with = "semio_framework_value::bytes")] pixels: Vec<u8> },
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
    #[dsl(key = "paint-stroke")]
    PaintStroke { xs: Vec<u32>, ys: Vec<u32>, color: Option<u32>, phase: Option<String>, reason: Option<String> },
    #[dsl(key = "solve")]
    Solve,
    #[dsl(key = "commit-fill-solve")]
    CommitFillSolve { pixels: String, contradiction: bool, width: u32, height: u32 },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "pin-solution")]
    PinSolution { pixels: String, contradiction: bool },
    /// 🖱️ A canvas press in world coordinates: on the input surface, a primary press opens a brush stroke.
    #[dsl(key = "canvasPointerDown")]
    CanvasPointerDown { surface_id: String, world_x: Option<f64>, world_y: Option<f64>, button: u32 },
    /// 🖱️ A batch of canvas samples in world coordinates: streams the open brush stroke, a hover otherwise.
    #[dsl(key = "canvasPointerMove")]
    CanvasPointerMove { surface_id: String, world_xs: Vec<f64>, world_ys: Vec<f64> },
    /// 🖱️ A canvas release in world coordinates: commits the open brush stroke, or aborts it when `cancelled`.
    #[dsl(key = "canvasPointerUp")]
    CanvasPointerUp { surface_id: String, world_x: Option<f64>, world_y: Option<f64>, cancelled: bool },
    /// 🖱️ A canvas double click: declared so it is never refused, inert on every bitmap pane.
    #[dsl(key = "canvasDoubleClick")]
    CanvasDoubleClick,
    /// 🎥️ The canvas host's debounced camera echo: declared and inert, every pointer verb carries world coordinates.
    #[dsl(key = "setCamera")]
    SyncCamera,
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
    "paint-stroke",
    "solve",
    "commit-fill-solve",
    "setActiveExample",
    "pin-solution",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "canvasDoubleClick",
    "setCamera",
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
        BitmapEditorCommand::PaintStroke { .. } => "paint-stroke",
        BitmapEditorCommand::Solve => "solve",
        BitmapEditorCommand::CommitFillSolve { .. } => "commit-fill-solve",
        BitmapEditorCommand::SetActiveExample { .. } => "setActiveExample",
        BitmapEditorCommand::PinSolution { .. } => "pin-solution",
        BitmapEditorCommand::CanvasPointerDown { .. } => "canvasPointerDown",
        BitmapEditorCommand::CanvasPointerMove { .. } => "canvasPointerMove",
        BitmapEditorCommand::CanvasPointerUp { .. } => "canvasPointerUp",
        BitmapEditorCommand::CanvasDoubleClick => "canvasDoubleClick",
        BitmapEditorCommand::SyncCamera => "setCamera",
    }
}

impl BitmapEditorCommand {
    /// 🖱️ The brush pointer event a canvas verb on the INPUT surface carries; `None` for every other command and for a
    /// pointer verb on another pane.
    pub fn brush_pointer(&self) -> Option<BitmapBrushPointer> {
        let world = |x: &Option<f64>, y: &Option<f64>| x.zip(*y).map(|(x, y)| [x, y]);
        match self {
            Self::CanvasPointerDown { surface_id, world_x, world_y, button } if input::owns_surface(surface_id) => Some(BitmapBrushPointer::Down { world: world(world_x, world_y), button: *button }),
            Self::CanvasPointerMove { surface_id, world_xs, world_ys } if input::owns_surface(surface_id) => Some(BitmapBrushPointer::Move { samples: world_xs.iter().zip(world_ys).map(|(x, y)| [*x, *y]).collect() }),
            Self::CanvasPointerUp { surface_id, world_x, world_y, cancelled } if input::owns_surface(surface_id) => Some(BitmapBrushPointer::Up { world: world(world_x, world_y), cancelled: *cancelled }),
            _ => None,
        }
    }
}
//#endregion 🔖️Command

//#region 🌉️ActionBridge
/// 🌉️ Host args → typed command. `VcsArtifactApp::dispatch_action` resolves EVERY host action
/// through this function, and the trait default answers a hard `app.command.unsupported` fault — so
/// an editor without this bridge has a complete, correctly classified, completely dead action roster.
pub(super) mod args_bridge {
    use super::{BitmapEditorCommand, Fault};
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn field<'a>(args: Option<&'a semio_framework_value::DslValue>, key: &str) -> Option<&'a semio_framework_value::DslValue> {
        args?.get(key)
    }

    /// 🔤️ A string arg — also accepts a number, so a select whose option ids are numeric reads the
    /// same whether the host sends `"3"` or `3`.
    fn text(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<String> {
        let value = field(args, key)?;
        value.as_str().map(str::to_owned).or_else(|| value.as_u64().map(|number| number.to_string())).or_else(|| value.as_i64().map(|number| number.to_string())).or_else(|| value.as_f64().map(|number| number.to_string()))
    }

    /// 🔢️ A numeric arg — also accepts a numeric string, which is how select-sourced numbers arrive.
    fn number(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<f64> {
        let value = field(args, key)?;
        value.as_f64().or_else(|| value.as_str()?.parse().ok())
    }

    fn flag(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<bool> {
        let value = field(args, key)?;
        value.as_bool().or_else(|| value.as_str()?.parse().ok())
    }

    /// 🌍️ A canvas move's world samples — `worldSamples` (`[x, y]` pairs, oldest first), else its one `worldX` /
    /// `worldY` — split into two coordinate columns; a malformed pair is skipped.
    fn world_samples(args: Option<&semio_framework_value::DslValue>) -> (Vec<f64>, Vec<f64>) {
        let pairs: Vec<(f64, f64)> = match field(args, "worldSamples").and_then(semio_framework_value::DslValue::as_array) {
            Some(rows) => rows.iter().filter_map(|row| match row.as_array() {
                Some([x, y]) => x.as_f64().zip(y.as_f64()),
                _ => None,
            }).collect(),
            None => number(args, "worldX").zip(number(args, "worldY")).into_iter().collect(),
        };
        pairs.into_iter().unzip()
    }

    /// 📍️ The stroke points of a `paint-stroke` dispatch — each `{x, y}` or `[x, y]`, non-negative whole cells —
    /// split into the command's two coordinate columns; absent points are an empty stroke.
    fn points(args: Option<&semio_framework_value::DslValue>) -> Result<(Vec<u32>, Vec<u32>), Fault> {
        let invalid = || Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid"), "paint-stroke points must be {x, y} or [x, y] pairs of non-negative whole cells");
        let cell = |value: Option<&semio_framework_value::DslValue>| value.and_then(semio_framework_value::DslValue::as_f64).filter(|value| value.is_finite() && *value >= 0.0 && value.fract() == 0.0 && *value <= f64::from(u32::MAX)).map(|value| value as u32);
        let Some(rows) = field(args, "points") else { return Ok((Vec::new(), Vec::new())) };
        let rows = rows.as_array().ok_or_else(invalid)?;
        let mut columns = (Vec::with_capacity(rows.len()), Vec::with_capacity(rows.len()));
        for row in rows {
            let (x, y) = match row.as_array() {
                Some([x, y]) => (cell(Some(x)), cell(Some(y))),
                Some(_) => (None, None),
                None => (cell(row.get("x")), cell(row.get("y"))),
            };
            columns.0.push(x.ok_or_else(invalid)?);
            columns.1.push(y.ok_or_else(invalid)?);
        }
        Ok(columns)
    }

    fn unknown(action: &str) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("wfc bitmap has no command for action '{action}'"))
    }

    pub fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<BitmapEditorCommand, Fault> {
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
                pixels: crate::standards::v1::subsets::any::io::text::snapshot::decode_pixel_text(&text(args, "pixels").unwrap_or_default()).map_err(|error| Fault::from(error.to_string()))?,
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
            "paint-stroke" => {
                let (xs, ys) = points(args)?;
                BitmapEditorCommand::PaintStroke { xs, ys, color: number(args, "color").map(|value| value.max(0.0) as u32), phase: text(args, "phase"), reason: text(args, "reason") }
            }
            "solve" => BitmapEditorCommand::Solve,
            "setActiveExample" => BitmapEditorCommand::SetActiveExample { example_id: text(args, "exampleId").unwrap_or_else(|| super::set_active_example::BITMAP_EXAMPLE_BOOT_ID.to_string()) },
            "commit-fill-solve" => BitmapEditorCommand::CommitFillSolve {
                pixels: text(args, "pixels").unwrap_or_default(),
                contradiction: bool_or("contradiction", false),
                width: u32_or("width", 0),
                height: u32_or("height", 0),
            },
            "pin-solution" => BitmapEditorCommand::PinSolution { pixels: text(args, "pixels").unwrap_or_default(), contradiction: bool_or("contradiction", false) },
            "canvasPointerDown" => BitmapEditorCommand::CanvasPointerDown { surface_id: text(args, "surfaceId").unwrap_or_default(), world_x: number(args, "worldX"), world_y: number(args, "worldY"), button: u32_or("button", 0) },
            "canvasPointerMove" => {
                let (world_xs, world_ys) = world_samples(args);
                BitmapEditorCommand::CanvasPointerMove { surface_id: text(args, "surfaceId").unwrap_or_default(), world_xs, world_ys }
            }
            "canvasPointerUp" => BitmapEditorCommand::CanvasPointerUp { surface_id: text(args, "surfaceId").unwrap_or_default(), world_x: number(args, "worldX"), world_y: number(args, "worldY"), cancelled: bool_or("cancelled", false) },
            "canvasDoubleClick" => BitmapEditorCommand::CanvasDoubleClick,
            "setCamera" => BitmapEditorCommand::SyncCamera,
            _ => return Err(unknown(action)),
        })
    }
}
//#endregion 🌉️ActionBridge
