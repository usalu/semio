//! 🧵️ Projects the addressed schedule into original segmented download ownership.
use super::MIME_TYPE;
use crate::editor::bim::{BimCommand, BimModelApp};
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement};
use semio_plugin::{
    ArtifactInstanceOperationOwnerHandle, EditorApp,
    app::{ArtifactDownloadOutput, ArtifactOutputChunks},
    retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep},
};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::{Analysis, MountedAnalysisAccess};
use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::ScheduleCsvCursor;
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::config::BimScheduleWindowConfigOwner;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

type App = EditorApp<BimModelApp>;
const PAGE_BYTES: usize = ArtifactOutputChunks::CHUNK_BYTES;
const CURSOR_BATCH: usize = 64;
const NAME_BYTES: usize = semio_plugin::plugin_app_close_prelude::semio_plugin_decl_media::ARTIFACT_OUTPUT_METADATA_BYTES - ".csv".len();
const PROJECTION: &str = "bim-schedule-csv-infer";
const ENCODING: &str = "bim-schedule-csv-encode";
const PROJECTION_PREVIEW: &[u8] = br#"{"en":"Reading the schedule","de":"Bauteilliste wird gelesen"}"#;
const ENCODING_PREVIEW: &[u8] = br#"{"en":"Writing the CSV file","de":"CSV-Datei wird geschrieben"}"#;

#[derive(semio_framework_value::RetireOwned)]
struct Owners {
    instance: Option<ArtifactInstanceOperationOwnerHandle>,
    access: MountedAnalysisAccess,
    chunks: Option<ArtifactOutputChunks>,
    page: Option<Vec<u8>>,
    filename: String,
    mime: String,
}

struct Name {
    stem: [u8; NAME_BYTES],
    written: usize,
    scanned: usize,
    separator: bool,
}

impl Name {
    fn new() -> Self {
        Self { stem: [0; NAME_BYTES], written: 0, scanned: 0, separator: false }
    }

    fn advance(&mut self, source: &str, cx: &mut StepContext<'_>) -> Result<bool, ValueError> {
        if !cx.charge_work_unit() {
            return Ok(false);
        }
        for ch in source[self.scanned..].chars().take(CURSOR_BATCH) {
            self.scanned += ch.len_utf8();
            if ch.is_alphanumeric() {
                let start = self.written + usize::from(self.separator);
                let end = start + ch.len_utf8();
                if end > self.stem.len() {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "BIM schedule download filename exceeds the output metadata cap"));
                }
                if self.separator {
                    self.stem[self.written] = b'-';
                }
                ch.encode_utf8(&mut self.stem[start..end]);
                self.written = end;
                self.separator = false;
            } else {
                self.separator = self.written != 0;
            }
        }
        Ok(self.scanned == source.len())
    }

    fn stem(&self) -> &str {
        if self.written == 0 { "schedule" } else { std::str::from_utf8(&self.stem[..self.written]).expect("schedule stem is UTF-8") }
    }
}

/// 📊️ Retains only the authenticated projection cursor and bounded output pages.
pub struct ScheduleCsvWork {
    tool_id: &'static str,
    owners: ControlledRetirement<Owners>,
    name: Name,
    cursor: ScheduleCsvCursor,
    named: bool,
    projected: bool,
    sealed: bool,
    closing: bool,
}

impl ScheduleCsvWork {
    pub fn new(tool_id: &'static str, instance: ArtifactInstanceOperationOwnerHandle) -> Self {
        let owners = Owners { instance: Some(instance), access: Default::default(), chunks: None, page: None, filename: String::new(), mime: String::new() };
        Self { tool_id, owners: ControlledRetirement::new(owners).unwrap_or_else(|_| unreachable!()), name: Name::new(), cursor: Default::default(), named: false, projected: false, sealed: false, closing: false }
    }

    fn progress(stage: &'static str, before: u64, cx: &mut StepContext<'_>) -> ArtifactCommandWorkStep<App> {
        let preview = if stage == PROJECTION { PROJECTION_PREVIEW } else { ENCODING_PREVIEW };
        if cx.fuel_remaining() == before {
            ArtifactCommandWorkStep::Pending { stage, preview }
        } else {
            cx.set_stage(stage);
            ArtifactCommandWorkStep::Progress { stage, preview }
        }
    }

    fn fill_page(
        cursor: &mut ScheduleCsvCursor,
        owners: &mut Owners,
        id: &str,
        revision: [u8; 32],
        schedule: &semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Schedule,
        cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        let page = owners.page.as_mut().expect("admitted page");
        Analysis::read_rendered(owners.instance.as_ref().expect("mounted instance"), &mut owners.access, revision, cx, |inference, cx| {
            let grant = cx.retained_grant();
            let room = (PAGE_BYTES - page.len()).min(grant.maximum_copy_bytes).min(CURSOR_BATCH);
            if room == 0 || grant.maximum_items == 0 || grant.maximum_depth == 0 || !cx.charge_work_unit() {
                return Ok(());
            }
            let mut bytes = [0; CURSOR_BATCH];
            let mut next = *cursor;
            let (written, _) = next.write(schedule, inference.schedules.get(id), &mut bytes[..room], CURSOR_BATCH);
            if written != 0 {
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: written, ..Default::default() })?;
                page.extend_from_slice(&bytes[..written]);
            }
            *cursor = next;
            Ok::<_, ValueError>(())
        })?
        .transpose()?;
        Ok(())
    }

    fn metadata_demand(&self) -> RetirementDemand {
        let extent = self.name.stem().len() + ".csv".len() + MIME_TYPE.len();
        RetirementDemand { copy_bytes: extent + size_of::<ArtifactDownloadOutput>(), capacity_bytes: extent, depth: 1, ..Default::default() }
    }

    fn publish(&mut self, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, OriginalTurnFailure> {
        let before = cx.fuel_remaining();
        let demand = self.metadata_demand();
        if !cx.retained_work_can_enter(demand) || !cx.charge_work_unit() {
            return Ok(Self::progress(ENCODING, before, cx));
        }
        let owners = self.owners.original_mut().expect("open schedule output");
        let stem = self.name.stem();
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let text = (|| -> Result<(), ValueError> {
            let allocation = |_| ValueError::literal(ValueRefusalKind::AllocationFailed, "BIM schedule download metadata allocation failed");
            owners.filename.try_reserve_exact(stem.len() + ".csv".len()).map_err(allocation)?;
            owners.filename.push_str(stem);
            owners.filename.push_str(".csv");
            progress.retained_capacity_bytes = owners.filename.capacity();
            progress.copied_bytes = owners.filename.len();
            owners.mime.try_reserve_exact(MIME_TYPE.len()).map_err(allocation)?;
            owners.mime.push_str(MIME_TYPE);
            progress.retained_capacity_bytes += owners.mime.capacity();
            progress.copied_bytes = demand.copy_bytes;
            Ok(())
        })();
        cx.consume_retained(progress)?;
        text?;
        let chunks = owners.chunks.take().expect("sealed schedule output");
        let download = ArtifactDownloadOutput::new(std::mem::take(&mut owners.filename), std::mem::take(&mut owners.mime), None, chunks)?;
        Ok(ArtifactCommandWorkStep::CompleteDownload { download, ephemeral: Default::default() })
    }
}

impl ArtifactCommandWork<App> for ScheduleCsvWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn receive_original_output_chunks(&mut self, chunks: &mut Option<ArtifactOutputChunks>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        cx.original_source_parent()?;
        let owners = self.owners.original_mut().expect("original schedule output open");
        if owners.chunks.is_some() {
            return Ok(true);
        }
        let bytes = size_of::<Option<ArtifactOutputChunks>>() * 2;
        if cx.is_cancelled() || !cx.retained_work_can_enter(RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }) || !cx.charge_work_unit() {
            return Ok(false);
        }
        if chunks.is_none() { return Err(ValueError::invariant("BIM schedule export lost its original output queue").into()); }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        owners.chunks = chunks.take();
        Ok(true)
    }

    fn extent(&self, command: &BimCommand, _: &ModelSnapshot, _: &protocol::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
        matches!(command, BimCommand::ExportScheduleCsv(_)).then_some(1)
    }

    fn work_demands(&self, _: &ArtifactCommandInputs<'_, App>, _: usize) -> Result<RetirementDemand, ValueError> {
        let Some(owners) = self.owners.original() else {
            return Ok(Default::default());
        };
        if self.sealed {
            return Ok(self.metadata_demand());
        }
        if !self.named {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if !self.projected {
            return owners.instance.as_ref().expect("original instance").inference_admission_demands(&owners.access);
        }
        if owners.page.as_ref().is_some_and(|page| page.len() == PAGE_BYTES || self.cursor.is_complete() && !page.is_empty()) {
            return Ok(ArtifactOutputChunks::original_push_demands());
        }
        if self.cursor.is_complete() {
            return Ok(ArtifactOutputChunks::original_seal_demands());
        }
        if owners.page.is_none() {
            return Ok(RetirementDemand { capacity_bytes: PAGE_BYTES, depth: 1, ..Default::default() });
        }
        owners.instance.as_ref().expect("original instance").inference_admission_demands(&owners.access)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, OriginalTurnFailure> {
        cx.original_source_parent()?;
        let before = cx.fuel_remaining();
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "BIM schedule export was cancelled").into());
        }
        let BimCommand::ExportScheduleCsv(command) = input.command else {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "BIM schedule export command changed").into());
        };
        if self.sealed {
            return self.publish(cx);
        }
        let id = if command.id.is_empty() { input.context.and_then(|context| context.window_config.as_ref()).and_then(|window| window.get::<BimScheduleWindowConfigOwner>()).map_or("", |config| config.schedule.as_str()) } else { &command.id };
        let schedule = input.snapshot.schedules.get(id).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "BIM document has no selected schedule to export"))?;
        if !self.named {
            self.named = self.name.advance(&schedule.name, cx)?;
            return Ok(Self::progress(PROJECTION, before, cx));
        }
        let owners = self.owners.original_mut().expect("open schedule owners");
        if !self.projected {
            self.projected = Analysis::read_rendered(owners.instance.as_ref().expect("original instance"), &mut owners.access, input.operation.canonical_base_revision, cx, |_, cx| cx.charge_work_unit())?.unwrap_or(false);
            return Ok(Self::progress(PROJECTION, before, cx));
        }
        let chunks = owners.chunks.as_ref().ok_or_else(|| ValueError::invariant("BIM schedule export requires its original output queue"))?;
        if owners.page.as_ref().is_some_and(|page| page.len() == PAGE_BYTES || self.cursor.is_complete() && !page.is_empty()) {
            chunks.push_original(&mut owners.page, None, cx)?;
        } else if self.cursor.is_complete() {
            self.sealed = chunks.seal_original(cx)?;
        } else if owners.page.is_none() {
            let demand = RetirementDemand { capacity_bytes: PAGE_BYTES, depth: 1, ..Default::default() };
            if cx.retained_work_can_enter(demand) && cx.charge_work_unit() {
                let mut page = Vec::new();
                page.try_reserve_exact(PAGE_BYTES).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "BIM schedule CSV page allocation failed"))?;
                let capacity = page.capacity();
                owners.page = Some(page);
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() })?;
            }
        } else {
            Self::fill_page(&mut self.cursor, owners, id, input.operation.canonical_base_revision, schedule, cx)?;
        }
        Ok(Self::progress(ENCODING, before, cx))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.owners.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        self.owners.next_capacity_byte_demand(body)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        self.owners.next_release_byte_demand()
    }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        self.owners.next_depth_demand()
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.closing {
            return Ok(InteractiveJobCloseStep::Blocked);
        }
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() });
        }
        let demand = RetirementDemand {
            copy_bytes: self.next_close_copy_byte_demand()?,
            capacity_bytes: self.next_close_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,
            release_bytes: self.next_close_release_byte_demand()?,
            depth: self.next_close_depth_demand()?,
        };
        semio_framework_job::close_original_effect_step(cx, demand, |effect| self.owners.step(effect.grant().maximum_copy_bytes, effect))
    }
}
