//! 📤️ `exportModel` delivers IFC2x3, IFC4, GLB, SVG, CSV and BCF files through the shell.
//! [`ModelExportWork`] borrows the mounted view's exact inference and yields between IFC
//! families. BCF carries the authored issues. [`ExportJob`] computes inference in reference
//! steps, and [`handle`] writes the same file through the synchronous command path.
//!
//! @see ../../../🚪️io/📤️export — the dialect writers this command speaks.

use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use crate::editor::bim::{BimCommand, BimModelApp};
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::Schema;
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::{StagedExport, can_emit_cold_column};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::{Analysis, MountedAnalysisAccess, PROJECTION_UNAVAILABLE};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::original_storeys::OriginalStoreyLevels;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, OriginalTurnPrimary, StepContext};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🧾️ Model and issue export formats used by the editor chrome.
pub const FORMATS: &[&str] = &["ifc2x3", "ifc4", "glb", "svg", "csv", "bcf"];
/// ⏩️ Graph nodes one job step may finish.
pub const NODES_PER_STEP: usize = 256;
/// 🗣️ The progress text of the inference stage, English and German.
pub const INFER_PREVIEW: &[u8] = br#"{"en":"Analysing the model","de":"Modell wird analysiert"}"#;
/// 🗣️ The progress text of the writing stage, English and German.
pub const ENCODE_PREVIEW: &[u8] = br#"{"en":"Writing the file","de":"Datei wird geschrieben"}"#;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "export-model")]
pub struct ExportModel {
    /// 📰️ The file format selected in the editor.
    pub format: String,
    /// 🔘️ The state a toggle that triggers the export reports; the export ignores it.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
}

/// 📄️ One written file, ready for the shell.
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    pub filename: String,
    pub mime_type: &'static str,
    pub data: String,
    pub encoding: Option<&'static str>,
}

/// 🏷️ The stem of the file name: the project name with every run of characters other than letters and digits turned into one dash.
pub fn stem(name: &str) -> String {
    let mut stem = String::new();
    for ch in name.chars() {
        if ch.is_alphanumeric() {
            stem.push(ch);
        } else if !stem.ends_with('-') && !stem.is_empty() {
            stem.push('-');
        }
    }
    let stem = stem.trim_end_matches('-');
    if stem.is_empty() { "model".to_string() } else { stem.to_string() }
}

fn refusal(format: &str) -> Fault {
    fault("bim.export.format-unknown", format!("'{format}' is not an export format of the model (one of {})", FORMATS.join(", ")))
}

fn failed(format: &str, message: impl std::fmt::Display) -> Fault {
    fault("bim.export.failed", format!("the {format} file could not be written: {message}"))
}

fn binary(bytes: &[u8]) -> (String, Option<&'static str>) {
    match std::str::from_utf8(bytes) {
        Ok(text) => (text.to_string(), None),
        Err(_) => (semio_framework_io_base64::base64_standard_encode(bytes), Some(MEDIA_EXPORT_BASE64_ENCODING)),
    }
}

fn issues_output(model: &ModelSnapshot) -> Result<Output, Fault> {
    let bytes = semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::bcf::export_bcf(model).map_err(|error| failed("bcf", error))?;
    Ok(Output {
        filename: format!("{}.bcfzip", stem(&model.project.name)),
        mime_type: "application/zip",
        data: semio_framework_io_base64::base64_standard_encode(&bytes),
        encoding: Some(MEDIA_EXPORT_BASE64_ENCODING),
    })
}

/// ✍️ The file of `format` for `model` from its inference.
pub fn encode(format: &str, model: &ModelSnapshot, inferred: &ModelInference) -> Result<Output, Fault> {
    let stem = stem(&model.project.name);
    let (extension, mime_type, data, encoding) = match format {
        "bcf" => return issues_output(model),
        "ifc2x3" | "ifc4" => {
            let (document, _) = semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::inferred_to_part21(schema_of(format).unwrap_or(Schema::Ifc2x3), model, inferred);
            return document_output(format, &stem, document);
        }
        "glb" => {
            let (scene, _) = semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::gltf::scene::build(model, inferred);
            let bytes = scene.to_glb().map_err(|error| failed(format, error))?;
            ("glb", "model/gltf-binary", semio_framework_io_base64::base64_standard_encode(&bytes), Some(MEDIA_EXPORT_BASE64_ENCODING))
        }
        "svg" => ("svg", "image/svg+xml", semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::views_to_svg(model, &inferred.view_linework).map_err(|error| failed(format, error))?, None),
        "csv" => ("csv", "text/csv", semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::report_csv(model, inferred), None),
        other => return Err(refusal(other)),
    };
    Ok(Output { filename: format!("{stem}.{extension}"), mime_type, data, encoding })
}

/// 🔖️ The IFC schema a format writes, `None` for the other formats.
pub fn schema_of(format: &str) -> Option<Schema> {
    match format {
        "ifc2x3" => Some(Schema::Ifc2x3),
        "ifc4" => Some(Schema::Ifc4),
        _ => None,
    }
}

fn document_output(format: &str, stem: &str, document: semio_s_artifact_stdio_contract::part21::Part21Document) -> Result<Output, Fault> {
    let bytes = semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::encode(schema_of(format).unwrap_or(Schema::Ifc2x3), document).map_err(|error| failed(format, error))?;
    let (data, encoding) = binary(&bytes);
    Ok(Output { filename: format!("{stem}.ifc"), mime_type: "application/x-step", data, encoding })
}

fn download(output: Output) -> Emit<ModelMutation, NoConfigMutation> {
    Emit::effect(Effect::DownloadMediaExport { filename: output.filename, mime_type: output.mime_type.into(), data: output.data, encoding: output.encoding.map(str::to_string) })
}

//#region 🔖️Job
/// ⏳️ What one step of a job did.
#[derive(Debug, PartialEq)]
pub enum Advance {
    Progress(&'static str),
    Done(Output),
}

/// 🧵️ A reference model export that can reuse inference and advance one IFC family per step.
pub struct ExportJob {
    format: String,
    analysis: Analysis,
    writer: Option<StagedExport>,
}

impl ExportJob {
    /// 🏗️ A job writing `format` of the document that instance `instance` shows.
    pub fn new(format:String, instance: semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Instance<'_>) -> Self {
        Self { format,analysis:Analysis::new(instance),writer:None }
    }

    /// 📈️ How far the job is: the analysis is the first half, the families of an IFC file the second half (one once the file is written).
    pub fn fraction(&self) -> f32 {
        match (&self.writer, schema_of(&self.format)) {
            (Some(writer), _) => 0.5 + 0.5 * writer.fraction(),
            (None, Some(_)) => 0.5 * self.analysis.fraction(),
            (None, None) => self.analysis.fraction(),
        }
    }

    /// 🏷️ The family the next step writes, `None` outside the writing of an IFC file.
    pub fn writing(&self) -> Option<&'static str> {
        self.writer.as_ref().and_then(StagedExport::stage)
    }

    /// 📝️ Advances a reference export by bounded inference work or one IFC family.
    pub fn advance_cold(&mut self, snapshot: &ModelSnapshot, nodes: usize) -> Result<Advance, Fault> {
        if !FORMATS.contains(&self.format.as_str()) {
            return Err(refusal(&self.format));
        }
        let ended = |error: protocol::InferenceError| fault("bim.export.inference", format!("the model could not be analysed: {error}"));
        if !self.analysis.is_done() {
            let progress = self.analysis.advance_cold(snapshot, nodes).map_err(ended)?;
            return Ok(Advance::Progress(if progress.done { "bim-export-encode" } else { "bim-export-infer" }));
        }
        let Some(schema) = schema_of(&self.format) else {
            return self.analysis.read_cold(snapshot, |inferred| encode(&self.format, snapshot, inferred)).map_err(ended)?.map(Advance::Done);
        };
        let writer = self.writer.get_or_insert_with(|| StagedExport::new(schema, snapshot));
        self.analysis.read_cold(snapshot, |inferred| advance_ifc(&self.format, snapshot, inferred, writer)).map_err(ended)?
    }

    /// 🛑️ Cancels reference export work and retains the completed inference nodes.
    pub fn cancel_cold(&mut self, snapshot: &ModelSnapshot) {
        self.writer = None;
        self.analysis.cancel_cold(snapshot);
    }
}
//#endregion 🔖️Job

fn advance_ifc(format: &str, snapshot: &ModelSnapshot, inferred: &ModelInference, writer: &mut StagedExport) -> Result<Advance, Fault> {
    match writer.step(snapshot, inferred) {
        Some((document, _)) => document_output(format, &stem(&snapshot.project.name), document).map(Advance::Done),
        None => Ok(Advance::Progress("bim-export-encode")),
    }
}

//#region 🔖️Work
/// ♻️ Returns the custody and allocation budget of an interrupted document writer.
macro_rules! close_export_writer {
    (@next $work:ident, $cx:ident) => {
        Ok(InteractiveJobCloseStep::Complete { progress: Default::default() })
    };
    (@next $work:ident, $cx:ident, $close:ident) => {
        $work.$close($cx)
    };
    ($($close:ident)?) => {
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand(bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(0)?.depth)
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let bytes = cx.retained_grant().maximum_copy_bytes;
        let Some(writer) = self.writer.as_mut() else {
            return close_export_writer!(@next self, cx $(, $close)?);
        };
        let demand = writer.selected_demand(bytes)?;
        let step = semio_framework_job::close_original_effect_step(cx, demand, |effect| writer.step(bytes, effect))?;
        if writer.terminal_is_empty() {
            self.writer = None;
            if !self.terminal_is_empty() {
                return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
            }
        }
        Ok(step)
    }
    };
}
pub(crate) use close_export_writer;

/// 🧵️ The mounted model export. Each IFC family is one cancellable boundary. Other codecs write one file per turn.
pub struct ModelExportWork {
    tool_id: &'static str,
    owner: semio_plugin::ArtifactInstanceOperationOwnerHandle,
    access: MountedAnalysisAccess,
    original_storeys: OriginalStoreyLevels,
    next_cold_column: usize,
    cold_column_refusal: Option<&'static str>,
    writer: Option<ControlledRetirement<StagedExport>>,
    complete: bool,
}

impl ModelExportWork {
    /// 🏗️ The work of tool `tool_id`.
    pub fn new(tool_id: &'static str, owner: semio_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, access: Default::default(), original_storeys: Default::default(), next_cold_column: 0, cold_column_refusal: None, writer: None, complete: false }
    }

    fn close_demand(&self, bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.writer.as_ref().map_or_else(|| self.original_storeys.retirement_demands(bytes), |writer| writer.selected_demand(bytes))
    }

    fn step_original_ifc(
        &mut self,
        format: &str,
        schema: Schema,
        snapshot: &ModelSnapshot,
        cx: &mut StepContext<'_>,
    ) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, OriginalTurnFailure> {
        let fuel = cx.fuel_remaining();
        if !self.original_storeys.step(snapshot, cx).map_err(original_failure)? {
            let stage = "bim-export-storey-levels";
            cx.set_stage(stage);
            return Ok(if fuel == cx.fuel_remaining() {
                ArtifactCommandWorkStep::Pending { stage, preview: INFER_PREVIEW }
            } else {
                ArtifactCommandWorkStep::Progress { stage, preview: INFER_PREVIEW }
            });
        }
        let stage = self.writer.as_ref().and_then(|writer| writer.original()).and_then(StagedExport::stage).unwrap_or("library");
        cx.set_stage(stage);
        if !cx.charge_work_unit() {
            return Ok(ArtifactCommandWorkStep::Pending { stage, preview: ENCODE_PREVIEW });
        }
        let writer = self.writer.get_or_insert_with(|| ControlledRetirement::retain(StagedExport::new(schema, snapshot)));
        let writer = writer.original_mut().expect("active spatial IFC exporter retains its writer");
        if let Some((doc, _)) = writer.step_from_levels(snapshot, |id| self.original_storeys.get(snapshot, id)) {
            self.complete = true;
            return Ok(ArtifactCommandWorkStep::Complete(download(document_output(format, &stem(&snapshot.project.name), doc)?)));
        }
        Ok(ArtifactCommandWorkStep::Progress { stage, preview: ENCODE_PREVIEW })
    }

    fn close_storeys(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let done = self.original_storeys.close_step(cx).map_err(original_failure)?;
        let progress = Default::default();
        Ok(if done { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } })
    }
}

fn original_failure(failure: protocol::InferenceWorkFailure) -> OriginalTurnFailure {
    match failure {
        protocol::InferenceWorkFailure::Original(failure) => failure,
        other => fault("bim.export.inference", format!("{other:?}")).into(),
    }
}

fn cold_ifc_refusal(model: &ModelSnapshot) -> Option<&'static str> {
    macro_rules! requires_render {
        ($($collection:ident => $family:literal),+ $(,)?) => {
            $(if !model.$collection.is_empty() { return Some(concat!("IFC export needs a completed render for ", $family, ".")); })+
        };
    }
    requires_render! {
        walls => "walls", wall_sweeps => "wall sweeps", curtain_walls => "curtain walls", openings => "openings",
        beams => "beams", slabs => "slabs", roofs => "roofs", ceilings => "ceilings",
        stairs => "stairs", railings => "railings", ramps => "ramps", components => "components", mep_elements => "MEP elements",
        spaces => "spaces", zones => "zones", area_schemes => "area schemes",
        dimensions => "dimensions", tags => "tags", text_notes => "text notes", leaders => "leaders",
        space_conditions => "space conditions", supports => "structural supports", loads => "structural loads", load_cases => "structural load cases",
    }
    None
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for ModelExportWork {
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(std::mem::size_of::<Self>())
    }

    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn terminal_is_empty(&self) -> bool {
        self.writer.is_none() && self.access.terminal_is_empty() && self.original_storeys.terminal_is_empty()
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (!self.complete && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, OriginalTurnFailure> {
        let BimCommand::ExportModel(payload) = input.command else {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "original BIM model export command changed").into());
        };
        if self.complete {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "original BIM model export already completed").into());
        }
        if cx.is_cancelled() {
            return Err(Fault::from("bim.export.cancelled").into());
        }
        let format = payload.format.as_str();
        if !FORMATS.contains(&format) {
            return Err(refusal(format).into());
        }
        let snapshot = input.snapshot;
        if format == "bcf" {
            if !cx.charge_work_unit() {
                return Ok(ArtifactCommandWorkStep::Pending { stage: "bim-export-encode", preview: ENCODE_PREVIEW });
            }
            let output = issues_output(snapshot)?;
            self.complete = true;
            return Ok(ArtifactCommandWorkStep::Complete(download(output)));
        }
        let schema = schema_of(format);
        let mut cold_refusal = schema.and_then(|_| cold_ifc_refusal(snapshot)).or(self.cold_column_refusal);
        if let Some(schema) = schema.filter(|_| cold_refusal.is_none()) {
            if let Some((_, column)) = snapshot.columns.get_index(self.next_cold_column) {
                let stage = "bim-export-column-plan";
                cx.set_stage(stage);
                if !cx.charge_retained_item(std::mem::size_of::<usize>(), 0, 0)? {
                    return Ok(ArtifactCommandWorkStep::Pending { stage, preview: INFER_PREVIEW });
                }
                if can_emit_cold_column(snapshot, column) {
                    self.next_cold_column += 1;
                    return Ok(ArtifactCommandWorkStep::Progress { stage, preview: INFER_PREVIEW });
                }
                self.cold_column_refusal = Some("IFC export needs a completed render for columns with a nonrectangular or missing type, or a tilted axis.");
                cold_refusal = self.cold_column_refusal;
            } else {
                return self.step_original_ifc(format, schema, snapshot, cx);
            }
        }
        let writer = &mut self.writer;
        let mut stage = "bim-export-infer";
        let fuel = cx.fuel_remaining();
        let output = Analysis::read_rendered(&self.owner, &mut self.access, input.operation.canonical_base_revision, cx, |inferred, cx| {
            if !cx.charge_work_unit() {
                return Ok(None);
            }
            stage = "bim-export-encode";
            cx.set_stage(stage);
            let Some(schema) = schema else {
                return encode(format, snapshot, inferred).map(Some);
            };
            let writer = writer.get_or_insert_with(|| ControlledRetirement::retain(StagedExport::new(schema, snapshot)));
            let writer = writer.original_mut().expect("active IFC exporter retains its writer");
            if let Some(family) = writer.stage() {
                stage = family;
                cx.set_stage(stage);
            }
            advance_ifc(format, snapshot, inferred, writer).map(|progress| match progress {
                Advance::Done(output) => Some(output),
                Advance::Progress(_) => None,
            })
        }).map_err(|mut failure| {
            if let (Some(message), OriginalTurnPrimary::Value(error)) = (cold_refusal, &mut failure.primary) {
                if error.message == PROJECTION_UNAVAILABLE {
                    error.message = message.into();
                }
            }
            failure
        })?;
        if let Some(output) = output.transpose()?.flatten() {
            self.complete = true;
            return Ok(ArtifactCommandWorkStep::Complete(download(output)));
        }
        let preview = if stage == "bim-export-infer" { INFER_PREVIEW } else { ENCODE_PREVIEW };
        Ok(if fuel == cx.fuel_remaining() {
            ArtifactCommandWorkStep::Pending { stage, preview }
        } else {
            ArtifactCommandWorkStep::Progress { stage, preview }
        })
    }

    close_export_writer!(close_storeys);
}
//#endregion 🔖️Work

/// 📤️ The synchronous path: the file of the instance's inference session as one download.
pub fn handle(payload: &ExportModel, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if payload.format == "bcf" {
        return issues_output(doc.snapshot).map(download);
    }
    let ended=|error:protocol::InferenceError|fault("bim.export.inference",format!("the model could not be analysed: {error}"));
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(ctx.gestures.as_ref(),doc.snapshot,|inferred|encode(&payload.format,doc.snapshot,inferred)).map_err(ended)?.map(download)

}

