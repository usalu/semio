import re, sys, os

ROOT = "/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"

TEMPLATE = '''//! 🎨️ Checked {kind_label} region painting with retained progress and cancellation.

use super::{{{command}, {editor}}};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity}};
use semio_framework_plugin::{{ActionArgDef, ActionDefinition, ActionKind, ArgSchema, EditorApp, Emit, Fault, FaultCode, FaultOrigin}};
use semio_framework_ui_locale::LocalizedLabel;
use {snapshot_path} as Snapshot;
use {mutation_path} as Mutation;

pub const ACTION_ID: &str = "paint-region";
pub const TOOL_IDS: &[&str] = &[ACTION_ID];
pub const PAYLOAD_SCHEMA: &str = "{payload_schema}";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const MAXIMUM_INTERACTIVE_ROWS: u32 = 65_536;
const ROWS_PER_TURN: usize = 64;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintRegionCommand {{
{fields}
}}

fn fault(code: &'static str, message: impl Into<String>) -> Fault {{
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}}

fn argument<'a>(args: Option<&'a semio_framework_value::DslValue>, name: &str) -> Option<&'a semio_framework_value::DslValue> {{
    let Some(semio_framework_value::DslValue::Object(fields)) = args else {{ return None }};
    fields.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}}

fn integer(args: Option<&semio_framework_value::DslValue>, name: &'static str, maximum: u64) -> Result<u64, Fault> {{
    let value = argument(args, name).ok_or_else(|| fault("{code_prefix}.missing-argument", format!("Missing {{name}}")))?;
    let semio_framework_value::DslValue::Number(number) = value else {{ return Err(fault("{code_prefix}.invalid-argument", format!("{{name}} must be an integer"))); }};
    let value = number.as_u64().ok_or_else(|| fault("{code_prefix}.invalid-argument", format!("{{name}} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("{code_prefix}.invalid-argument", format!("{{name}} exceeds {{maximum}}")))
}}

impl PaintRegionCommand {{
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {{
        let width = integer(args, "width", u32::MAX.into())? as u32;
        let height = integer(args, "height", u32::MAX.into())? as u32;
        if width == 0 || height == 0 {{ return Err(fault("{code_prefix}.empty", "Paint region width and height must be positive")); }}
        if height > MAXIMUM_INTERACTIVE_ROWS {{ return Err(fault("{code_prefix}.too-many-rows", format!("Paint region height exceeds {{MAXIMUM_INTERACTIVE_ROWS}}"))); }}
        Ok(Self {{
{from_action_fields}
        }})
    }}
}}

fn integer_arg(id: &'static str, en: &'static str, de: &'static str, minimum: f64, maximum: f64, default: u32) -> ActionArgDef {{
    let mut argument = ActionArgDef::number(id, LocalizedLabel::native(en, de)).required().default_value(&default);
    if let ArgSchema::Number {{ min, max, step, integer, .. }} = &mut argument.schema {{
        *min = Some(minimum);
        *max = Some(maximum);
        *step = Some(1.0);
        *integer = true;
    }}
    argument
}}

pub fn action() -> ActionDefinition {{
    ActionDefinition::bounded_catalog(ACTION_ID, LocalizedLabel::native("{label_en}", "{label_de}"), ActionKind::Mutation).with_args(vec![
{action_args}
    ])
}}

{validate}

pub struct PaintRegionWork {{
    revision: Option<String>,
    samples: Option<Vec<u8>>,
    cursor: usize,
    rows: usize,
    complete: bool,
    closing: bool,
}}

impl PaintRegionWork {{
    pub fn new() -> Self {{
        Self {{ revision: None, samples: None, cursor: 0, rows: 0, complete: false, closing: false }}
    }}
}}

impl ArtifactCommandWork<EditorApp<{editor}>> for PaintRegionWork {{
    fn tool_id(&self) -> &'static str {{ ACTION_ID }}

    fn extent(&self, command: &{command}, snapshot: &Snapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<{editor}>>>) -> Option<usize> {{
        let {command}::PaintRegion(command) = command else {{ return None }};
        if command.width == 0 || command.height == 0 || {extent_empty} {{ return None; }}
        CAPACITY.rows_for_items(1)
    }}

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<{editor}>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {{
        Ok(semio_framework_value::RetirementDemand {{ copy_bytes: {source_len}.max(size_of::<Mutation>()), depth: 1, ..Default::default() }})
    }}

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<{editor}>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<{editor}>>, semio_framework_job::OriginalTurnFailure> {{
        let original: Result<_, Fault> = (|| {{
        if self.closing || self.complete {{ return Err(fault("{code_prefix}.work-closed", "Paint region work is already closed")); }}
        let {command}::PaintRegion(command) = input.command else {{ return Err(fault("{code_prefix}.route-mismatch", "Paint work received another command")); }};
        if self.revision.is_none() {{
            self.revision = Some(validate(*command, input.snapshot)?);
            self.samples = Some({samples_clone});
            self.rows = command.height as usize;
            return Ok(ArtifactCommandWorkStep::Progress {{ stage: "{stage}-prepare", preview: br#"{{"en":"Preparing region paint","de":"Bereichsmalen wird vorbereitet"}}"# }});
        }}
        if self.cursor < self.rows {{
            let samples = self.samples.as_mut().ok_or_else(|| fault("{code_prefix}.work-closed", "Paint region work lost its sample buffer"))?;
            let end = (self.cursor + ROWS_PER_TURN).min(self.rows);
            while self.cursor < end {{
{paint_row}
                self.cursor += 1;
            }}
            return Ok(ArtifactCommandWorkStep::Progress {{ stage: "{stage}-row", preview: br#"{{"en":"Painting rows","de":"Zeilen werden gemalt"}}"# }});
        }}
        self.revision.take();
        let samples = self.samples.take().ok_or_else(|| fault("{code_prefix}.work-closed", "Paint region work lost its sample buffer"))?;
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![{mutation_build}])))
        }})();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }}

    fn begin_close(&mut self) {{ self.closing = true; }}

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{{
        semio_framework_job::close_original_granted_step(cx,|grant|{{
            if !self.closing {{ return semio_framework_value::RetainedWorkOutcome {{ result: Ok(InteractiveJobCloseStep::Blocked), work: 0 }}; }}
            self.rows = 0;
            self.cursor = 0;
            self.samples = None;
            semio_framework_value::RetainedWorkOutcome {{ result: Ok(semio_s_artifact_stdio_contract::editing::close_revision_turn(&mut self.revision, grant)), work: 1 }}
        }})
    }}

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {{ Ok(0) }}
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {{ Ok(0) }}
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {{ Ok(semio_s_artifact_stdio_contract::editing::revision_close_demand(&self.revision).release_bytes) }}
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {{ Ok(semio_s_artifact_stdio_contract::editing::revision_close_demand(&self.revision).depth) }}

    fn terminal_is_empty(&self) -> bool {{
        self.closing && self.revision.is_none() && self.samples.is_none() && self.rows == 0 && self.cursor == 0
    }}
}}
'''

COLOR_FIELDS = ["x", "y", "width", "height", "red", "green", "blue", "alpha"]

def jpg_variant(name, editor, command, snapshot_path, controller_suffix):
    fields = "\n".join(f"    pub {f}: u{'8' if f in ('red','green','blue','alpha') else '32'}," for f in COLOR_FIELDS)
    from_action = "\n".join(f"            {f}: integer(args, \"{f}\", {'u8::MAX' if f in ('red','green','blue','alpha') else 'u32::MAX'}.into())? as u{'8' if f in ('red','green','blue','alpha') else '32'}," if f not in ("width","height") else f"            {f}," for f in COLOR_FIELDS)
    args = []
    for f, en, de, mn, mx, d in [("x","Left (x)","Links (x)","0.0","u32::MAX as f64","0"),("y","Top (y)","Oben (y)","0.0","u32::MAX as f64","0"),("width","Width","Breite","1.0","u32::MAX as f64","1"),("height","Height","Höhe","1.0","f64::from(MAXIMUM_INTERACTIVE_ROWS)","1"),("red","Red","Rot","0.0","255.0","0"),("green","Green","Grün","0.0","255.0","0"),("blue","Blue","Blau","0.0","255.0","0"),("alpha","Alpha","Alpha","0.0","255.0","255")]:
        args.append(f'        integer_arg("{f}", "{en}", "{de}", {mn}, {mx}, {d}),')
    validate = '''fn validate(command: PaintRegionCommand, snapshot: &Snapshot) -> Result<String, Fault> {
    let image = &snapshot.image;
    let right = command.x.checked_add(command.width).ok_or_else(|| fault("stdio.jpg.paint-region.out-of-bounds", "Paint region overflows the image"))?;
    let bottom = command.y.checked_add(command.height).ok_or_else(|| fault("stdio.jpg.paint-region.out-of-bounds", "Paint region overflows the image"))?;
    if right > image.width || bottom > image.height { return Err(fault("stdio.jpg.paint-region.out-of-bounds", format!("Paint region {}x{} at {},{} exceeds the {}x{} image", command.width, command.height, command.x, command.y, image.width, image.height))); }
    if image.pixels.len() != image.width as usize * image.height as usize * 4 { return Err(fault("stdio.jpg.paint-region.invalid-snapshot", "The decoded raster does not match the image extent")); }
    Ok(format!("{}x{}", image.width, image.height))
}'''
    paint_row = '''            let command = *command;
            let stride = input.snapshot.image.width as usize * 4;
            let row = command.y as usize + self.cursor;
            let start = row * stride + command.x as usize * 4;
            for column in 0..command.width as usize { samples[start + column * 4..start + column * 4 + 4].copy_from_slice(&[command.red, command.green, command.blue, command.alpha]); }'''
    return dict(name=name, editor=editor, command=command, kind_label="JPEG", snapshot_path=snapshot_path, mutation_path="crate::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation", payload_schema=f"s.stdio.jpg.{controller_suffix}.command.paint-region.v1", fields=fields, from_action_fields=from_action, label_en="Paint Region", label_de="Bereich malen", action_args="\n".join(args), validate=validate, extent_empty="snapshot.image.pixels.is_empty()", source_len="input.snapshot.image.pixels.len()", samples_clone="input.snapshot.image.pixels.clone()", paint_row=paint_row, mutation_build="Mutation::ReplacePixels(crate::standards::v_jfif_1_01::subsets::document::schema::mutations::ReplacePixelsMutation { pixels: samples })", code_prefix="stdio.jpg.paint-region", stage="jpg-paint-region")

def gif_variant(name, editor, command, std, items, mutation_variant, mutation_mod, mutation_struct, controller_suffix):
    fields = "    pub item: u32,\n" + "\n".join(f"    pub {f}: u32," for f in ("x","y","width","height")) + "\n    pub index: u8,"
    from_action = "            item: integer(args, \"item\", u32::MAX.into())? as u32,\n            x: integer(args, \"x\", u32::MAX.into())? as u32,\n            y: integer(args, \"y\", u32::MAX.into())? as u32,\n            width,\n            height,\n            index: integer(args, \"index\", u8::MAX.into())? as u8,"
    args = ['        integer_arg("item", "Frame", "Einzelbild", 0.0, u32::MAX as f64, 0),', '        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),', '        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),', '        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),', '        integer_arg("height", "Height", "Höhe", 1.0, f64::from(MAXIMUM_INTERACTIVE_ROWS), 1),', '        integer_arg("index", "Palette index", "Palettenindex", 0.0, 255.0, 0),']
    validate = f'''fn validate(command: PaintRegionCommand, snapshot: &Snapshot) -> Result<String, Fault> {{
    let item = snapshot.{items}.get(command.item as usize).ok_or_else(|| fault("stdio.gif.paint-region.no-item", format!("The document has no {{}} number {{}}", "{items}", command.item)))?;
    let right = command.x.checked_add(command.width).ok_or_else(|| fault("stdio.gif.paint-region.out-of-bounds", "Paint region overflows the frame"))?;
    let bottom = command.y.checked_add(command.height).ok_or_else(|| fault("stdio.gif.paint-region.out-of-bounds", "Paint region overflows the frame"))?;
    if right > item.width || bottom > item.height {{ return Err(fault("stdio.gif.paint-region.out-of-bounds", format!("Paint region {{}}x{{}} at {{}},{{}} exceeds the {{}}x{{}} frame", command.width, command.height, command.x, command.y, item.width, item.height))); }}
    if item.indices.len() != item.width as usize * item.height as usize {{ return Err(fault("stdio.gif.paint-region.invalid-snapshot", "The frame indices do not match the frame extent")); }}
    Ok(format!("{{}}:{{}}x{{}}", command.item, item.width, item.height))
}}'''
    paint_row = f'''            let command = *command;
            let width = input.snapshot.{items}[command.item as usize].width as usize;
            let start = (command.y as usize + self.cursor) * width + command.x as usize;
            samples[start..start + command.width as usize].fill(command.index);'''
    return dict(name=name, editor=editor, command=command, kind_label="GIF", snapshot_path=f"crate::standards::{std}::subsets::any::schema::snapshot::GifSnapshot", mutation_path=f"crate::standards::{std}::subsets::any::schema::mutations::GifMutation", payload_schema=f"s.stdio.gif.{controller_suffix}.command.paint-region.v1", fields=fields, from_action_fields=from_action, label_en="Paint Frame Region", label_de="Einzelbildbereich malen", action_args="\n".join(args), validate=validate, extent_empty=f"snapshot.{items}.get(command.item as usize).is_none()", source_len=f"input.snapshot.{items}.get(match input.command {{ {command}::PaintRegion(command) => command.item as usize, _ => 0 }}).map_or(0, |item| item.indices.len())", samples_clone=f"input.snapshot.{items}[command.item as usize].indices.clone()", paint_row=paint_row, mutation_build=f"Mutation::{mutation_variant}(crate::standards::{std}::subsets::any::schema::mutations::{mutation_mod}::{mutation_struct} {{ index: command.item as usize, indices: samples }})", code_prefix="stdio.gif.paint-region", stage="gif-paint-region")

VARIANTS = [
    dict(path="📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/", **{"cfg": jpg_variant("jpg-any", "JpgAnyEditor", "JpgAnyEditCommand", "crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot", "any")}),
    dict(path="📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/", **{"cfg": jpg_variant("jpg-baseline", "JpgBaselineEditor", "JpgBaselineEditCommand", "crate::standards::v_jfif_1_01::subsets::baseline::schema::snapshot::JpgSnapshot", "baseline")}),
    dict(path="🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/", **{"cfg": gif_variant("gif-87a", "Gif87aEditor", "Gif87aEditCommand", "v87a", "images", "SetImagePixels", "set_image_pixels", "SetImagePixels", "87a")}),
    dict(path="🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/", **{"cfg": gif_variant("gif-89a", "Gif89aEditor", "Gif89aEditCommand", "v89a", "frames", "SetFramePixels", "set_frame_pixels", "SetFramePixels", "89a")}),
]

def write_command_file(v):
    d = ROOT + v["path"] + "🎭️modes/✏️edit/🎮️commands/🎨️paint-region/"
    os.makedirs(d, exist_ok=True)
    open(d + "🦀️.rs", "w").write(TEMPLATE.format(**v["cfg"]))

if __name__ == "__main__":
    for v in VARIANTS:
        write_command_file(v)
        print("wrote", v["cfg"]["name"])
