//! 📂 Uses the existing shell's bounded local picker for PNG authoring.

use super::{check_image_placement, ImagePlacement, MAX_DATA_URL, MAX_FILE_BYTES, PNG_PREFIX, revision_bytes};
use super::{PdfMutation, PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactApp, Emit, Fault};
use semio_plugin::__value::{DslValue, ToValue};
use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_plugin::retained_command::__action_bus::{ToolFactoryKey, ToolJobFactory};

pub const IMPORT: &str = "import-local-image";
pub const INSERT: &str = "insert-image";
pub const REPLACE: &str = "replace-image";
const BINARY_PREFIX: &str = "data:application/octet-stream;base64,";
pub const INSERT_OPERATOR: u32 = u32::MAX;
pub const WIRE_BYTES: usize = 48 * 1024;
pub const CONTRACT: semio_plugin::ToolExecutionContract =
    semio_plugin::ToolExecutionContract::resumable(WIRE_BYTES, 4096, 16384, 16 * 1024 * 1024, 7500, 1, 1);

fn fault(message: &str) -> Fault {
    Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("stdio.pdf.image-import"), message)
}

fn number(args: Option<&DslValue>, field: &str) -> Result<f64, Fault> {
    semio_s_artifact_stdio_contract::window_kit_text_argument(args, &[field], "")
        .parse::<f64>().ok().filter(|value| value.is_finite()).ok_or_else(|| fault("Invalid image placement coordinates"))
}

fn index(args: Option<&DslValue>, field: &str) -> Result<u32, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, field)
}

fn text(args: Option<&DslValue>, field: &str) -> Result<String, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, field)
}

pub fn admits_png_action(action: &str) -> bool {
    matches!(action, INSERT | REPLACE | IMPORT)
}

/// 📌 Validated target fields used by both picker and completion commands.
pub struct PngTarget {
    pub page: u32,
    pub operator: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl PngTarget {
    pub fn placement(&self) -> ImagePlacement {
        if self.operator == INSERT_OPERATOR {
            ImagePlacement::Insert {
                page: self.page as usize,
                matrix: [self.width, 0.0, 0.0, self.height, self.x, self.y],
            }
        } else {
            ImagePlacement::Replace { page: self.page as usize, operator: self.operator as usize }
        }
    }
}

pub enum PngInvocation {
    Pick { target: PngTarget, guard: String },
    Import { target: PngTarget, revision: String, payload: String },
}

pub fn png_invocation(action: &str, args: Option<&DslValue>) -> Result<PngInvocation, Fault> {
    let page = index(args, "page")?;
    if action == REPLACE {
        let operator = index(args, "operator")?;
        if operator == INSERT_OPERATOR { return Err(fault("Invalid PDF image operator")); }
        return Ok(PngInvocation::Pick { target: PngTarget { page, operator, x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, guard: text(args, "text")? });
    }
    let geometry = || Ok::<_, Fault>((number(args, "x")?, number(args, "y")?, number(args, "width")?, number(args, "height")?));
    let (x, y, width, height) = geometry()?;
    if action == INSERT {
        if width <= 0.0 || height <= 0.0 { return Err(fault("Image width and height must be positive")); }
        return Ok(PngInvocation::Pick { target: PngTarget { page, operator: INSERT_OPERATOR, x, y, width, height }, guard: String::new() });
    }
    let operator = index(args, "operator")?;
    let revision = text(args, "revision")?;
    let wire = text(args, "payload")?;
    let encoded = wire.strip_prefix(BINARY_PREFIX).or_else(|| wire.strip_prefix(PNG_PREFIX))
        .ok_or_else(|| fault("Choose a PNG within the local image import limits"))?;
    if index(args, "chunk")? != 0 || index(args, "chunkCount")? != 1 ||
       revision.len() != 64 || encoded.is_empty() || encoded.len() > MAX_DATA_URL - PNG_PREFIX.len() {
        return Err(fault("Choose a PNG within the local image import limits"));
    }
    let payload = format!("{PNG_PREFIX}{encoded}");
    Ok(PngInvocation::Import { target: PngTarget { page, operator, x, y, width, height }, revision, payload })
}


/// 📂 Opens the selected file under the exact committed target revision.
pub fn picker(snapshot: &PdfSnapshot, operation: &semio_plugin::AppOperationContext, target: PngTarget, guard: &str) -> Result<Emit<PdfMutation>, Fault> {
    if target.operator != INSERT_OPERATOR && guard.as_bytes() != &revision_bytes(operation)[..16] {
        return Err(fault("Selected PDF image changed before the picker opened"));
    }
    check_image_placement(snapshot, target.placement())?;
    let PngTarget { page, operator, x, y, width, height } = target;
    let args = DslValue::object([
        ("page".into(), page.to_value()), ("operator".into(), operator.to_value()),
        ("revision".into(), String::from_utf8(revision_bytes(operation).to_vec()).expect("hexadecimal revision").to_value()), ("x".into(), x.to_value()), ("y".into(), y.to_value()),
        ("width".into(), width.to_value()), ("height".into(), height.to_value()),
    ]);
    Ok(Emit::effect(semio_plugin::Effect::RequestFileOpen {
        req: semio_plugin::RequestId(0x5044494d), accept: "image/png,.png".into(), read_as: Some("dataUrl".into()),
        import_action: IMPORT.into(), multiple: false, args: Some(args), max_file_bytes: Some(MAX_FILE_BYTES),
    }))
}


pub fn png_actions() -> [semio_plugin::ActionDefinition; 3] {
    use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind, InteractiveJobClassification};
    use semio_plugin::LocalizedLabel;
    let label = LocalizedLabel::native;
    let page = || ActionArgDef::number("page", label("Page", "Seite")).required();
    let operator = || ActionArgDef::number("operator", label("Image placement", "Bildposition")).required();
    let geometry = || [
        ActionArgDef::number("x", label("X", "X")).required(),
        ActionArgDef::number("y", label("Y", "Y")).required(),
        ActionArgDef::number("width", label("Width", "Breite")).required(),
        ActionArgDef::number("height", label("Height", "Höhe")).required(),
    ];
    let action = |id, en, de, args| {
        let mut definition = ActionDefinition::bounded_catalog(id, label(en, de), ActionKind::Mutation).with_args(args);
        definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        definition
    };
    [
        action(INSERT, "Insert PNG Image", "PNG-Bild einfügen", [vec![page()], geometry().to_vec()].concat()),
        action(REPLACE, "Replace Image with PNG", "Bild durch PNG ersetzen", vec![page(), operator(), ActionArgDef::text("text", label("Revision", "Revision")).min_length(16).required()]).in_palette(false),
        action(IMPORT, "Import PNG Samples", "PNG-Bildpunkte importieren", [
            vec![page(), operator(),
                ActionArgDef::text("revision", label("Revision", "Revision")).min_length(64).required(),
                ActionArgDef::text("payload", label("PNG data", "PNG-Daten")).min_length(1).required(),
                ActionArgDef::text("name", label("File name", "Dateiname")).min_length(1),
                ActionArgDef::number("chunk", label("Chunk", "Abschnitt")).required(),
                ActionArgDef::number("chunkCount", label("Chunk count", "Abschnittsanzahl")).required(),
            ],
            geometry().to_vec(),
        ].concat()).in_palette(false),
    ]
}

pub struct PngFactory<A: ArtifactApp> {
    keys: [ToolFactoryKey; 1],
    marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp> PngFactory<A> {
    pub fn new(controller: &str) -> Self { Self { keys: [ToolFactoryKey::new(controller, IMPORT)], marker: std::marker::PhantomData } }
}

impl<A: ArtifactApp> ToolJobFactory for PngFactory<A> {
    type Payload = ArtifactRetainedCommandPayload<A>;
    type Job = ArtifactRetainedCommandJob<A>;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { "stdio.pdf.local-png-import.v1" }
    fn classification(&self) -> semio_plugin::InteractiveJobClassification { semio_plugin::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> semio_plugin::ToolExecutionContract { CONTRACT }

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: WIRE_BYTES, checkpoint_resume: false);
}

impl<A: ArtifactApp> semio_plugin::ArtifactOwnedToolJobFactory for PngFactory<A> {
    type Owner = A;
    const TOOL_IDS: &'static [&'static str] = &[IMPORT];
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF17_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = &[
        semio_plugin::ArtifactToolPublicationContract { tool_id: IMPORT, lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
    ];
}

