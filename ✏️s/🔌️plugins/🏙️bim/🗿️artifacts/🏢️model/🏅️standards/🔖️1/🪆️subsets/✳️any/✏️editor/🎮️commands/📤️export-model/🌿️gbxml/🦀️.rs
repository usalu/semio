//! 🌿️ The mounted gbXML export borrows its exact committed model projection and delivers original pages.

use super::*;
use semio_framework_value::RetainedCloneProgress;
use semio_plugin::{ArtifactInstanceOperationOwnerHandle, InteractionState, app::{ArtifactDownloadOutput, ArtifactOutputChunks}};
use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::gbxml::OriginalGbxml;

type App = EditorApp<BimModelApp>;
const MIME: &str = "application/xml";
const SUFFIX: &str = ".gbxml";
const NAME_BYTES: usize = semio_plugin::plugin_app_close_prelude::semio_plugin_decl_media::ARTIFACT_OUTPUT_METADATA_BYTES - SUFFIX.len();

#[derive(semio_framework_value::RetireOwned)]
struct Owners {
    instance: ArtifactInstanceOperationOwnerHandle,
    filename: String,
    mime: String,
}

struct Filename {
    bytes: [u8; NAME_BYTES],
    len: usize,
    source: usize,
    separator: bool,
}

impl Filename {
    fn advance(&mut self, name: &str, cx: &mut StepContext<'_>) -> Result<bool, ValueError> {
        if !cx.charge_work_unit() { return Ok(false); }
        for character in name[self.source..].chars().take(64) {
            self.source += character.len_utf8();
            if character.is_alphanumeric() {
                let start = self.len + usize::from(self.separator);
                let end = start + character.len_utf8();
                if end > NAME_BYTES {
                    return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "BIM gbXML filename exceeds the download metadata cap"));
                }
                if self.separator { self.bytes[self.len] = b'-'; }
                character.encode_utf8(&mut self.bytes[start..end]);
                self.len = end;
                self.separator = false;
            } else {
                self.separator = self.len != 0;
            }
        }
        Ok(self.source == name.len())
    }

    fn stem(&self) -> &str {
        if self.len == 0 { "model" }
        else { std::str::from_utf8(&self.bytes[..self.len]).expect("the original filename is UTF-8") }
    }
}

/// 📤️ The registered mounted gbXML producer and its original retirement cursor.
pub struct GbxmlExportWork {
    tool_id: &'static str,
    owners: ControlledRetirement<Owners>,
    access: MountedAnalysisAccess,
    original: OriginalGbxml,
    filename: Filename,
    named: bool,
    sealed: bool,
    complete: bool,
    closing: bool,
}

impl GbxmlExportWork {
    /// 🏗️ Sets up the fixed-size work frame for the admitted document instance.
    pub fn new(tool_id: &'static str, instance: ArtifactInstanceOperationOwnerHandle) -> Self {
        Self {
            tool_id, owners: ControlledRetirement::retain(Owners { instance, filename: String::new(), mime: String::new() }), access: Default::default(),
            original: Default::default(), filename: Filename { bytes: [0; NAME_BYTES], len: 0, source: 0, separator: false },
            named: false, sealed: false, complete: false, closing: false,
        }
    }

    fn metadata_demand(&self) -> RetirementDemand {
        let bytes = self.filename.stem().len() + SUFFIX.len() + MIME.len();
        RetirementDemand { copy_bytes: bytes + size_of::<ArtifactDownloadOutput>(), capacity_bytes: bytes, depth: 1, ..Default::default() }
    }

    fn progress(stage: &'static str, fuel: u64, cx: &mut StepContext<'_>) -> ArtifactCommandWorkStep<App> {
        let preview = if stage == "bim-export-encode" { ENCODE_PREVIEW } else { INFER_PREVIEW };
        if fuel == cx.fuel_remaining() { ArtifactCommandWorkStep::Pending { stage, preview } }
        else { ArtifactCommandWorkStep::Progress { stage, preview } }
    }

    fn publish(&mut self, cx: &mut StepContext<'_>) -> Result<Option<ArtifactDownloadOutput>, OriginalTurnFailure> {
        let demand = self.metadata_demand();
        let owners = self.owners.original_mut().ok_or_else(|| ValueError::invariant("gbXML metadata is retiring"))?;
        if owners.mime.is_empty() {
            if !cx.retained_work_can_enter(demand) || !cx.charge_work_unit() { return Ok(None); }
            let allocation = |_| ValueError::literal(ValueRefusalKind::AllocationFailed, "BIM gbXML download metadata allocation failed");
            let stem = self.filename.stem();
            let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            let result = (|| {
                owners.filename.try_reserve_exact(stem.len() + SUFFIX.len()).map_err(allocation)?;
                progress.retained_capacity_bytes = owners.filename.capacity();
                owners.filename.push_str(stem);
                owners.filename.push_str(SUFFIX);
                progress.copied_bytes = owners.filename.len();
                owners.mime.try_reserve_exact(MIME.len()).map_err(allocation)?;
                progress.retained_capacity_bytes += owners.mime.capacity();
                owners.mime.push_str(MIME);
                progress.copied_bytes = demand.copy_bytes;
                Ok::<_, ValueError>(())
            })();
            cx.consume_retained(progress)?;
            result?;
            return Ok(None);
        }
        let Some(chunks) = self.original.take(cx)? else { return Ok(None); };
        let download = ArtifactDownloadOutput::new(std::mem::take(&mut owners.filename), std::mem::take(&mut owners.mime), None, chunks)?;
        self.complete = true;
        Ok(Some(download))
    }

    fn close_demand(&self, bytes: usize) -> Result<RetirementDemand, ValueError> {
        if !self.original.terminal_is_empty() { self.original.close_demand(bytes) }
        else { self.owners.selected_demand(bytes) }
    }
}

impl ArtifactCommandWork<App> for GbxmlExportWork {
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(&self, command: &BimCommand, _: &ModelSnapshot, _: &InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
        (!self.complete && matches!(command, BimCommand::ExportModel(command) if command.format == "gbxml")).then_some(1)
    }

    fn receive_original_output_chunks(&mut self, chunks: &mut Option<ArtifactOutputChunks>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.original.receive(chunks, cx)
    }

    fn work_demands(&self, _: &ArtifactCommandInputs<'_, App>, _: usize) -> Result<RetirementDemand, ValueError> {
        if !self.named { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        let owners = self.owners.original().ok_or_else(|| ValueError::invariant("gbXML metadata is retiring"))?;
        let inference = owners.instance.inference_admission_demands(&self.access)?;
        let work = if self.sealed && owners.mime.is_empty() { Some(self.metadata_demand()) }
            else { self.original.encoding_demand() };
        Ok(work.map_or(inference, |work| RetirementDemand {
            copy_bytes: inference.copy_bytes.max(work.copy_bytes),
            capacity_bytes: inference.capacity_bytes.max(work.capacity_bytes),
            release_bytes: inference.release_bytes.max(work.release_bytes),
            depth: inference.depth.saturating_add(work.depth),
        }))
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, OriginalTurnFailure> {
        cx.original_source_parent()?;
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "BIM gbXML export was cancelled").into()); }
        if self.complete || !matches!(input.command, BimCommand::ExportModel(command) if command.format == "gbxml") {
            return Err(ValueError::invariant("BIM gbXML export command changed or completed").into());
        }
        let fuel = cx.fuel_remaining();
        let stage = self.original.stage();
        cx.set_stage(stage);
        if !self.named {
            self.named = self.filename.advance(&input.snapshot.project.name, cx)?;
        } else if self.sealed {
            let owners = self.owners.original_mut().ok_or_else(|| ValueError::invariant("gbXML projection is retiring"))?;
            let exact = Analysis::read_rendered(&owners.instance, &mut self.access, input.operation.canonical_base_revision, cx, |_, _| ())?;
            if exact.is_some() {
                if let Some(download) = self.publish(cx)? {
                    return Ok(ArtifactCommandWorkStep::CompleteDownload { download, ephemeral: Default::default() });
                }
            }
        } else {
            let owners = self.owners.original_mut().ok_or_else(|| ValueError::invariant("gbXML projection is retiring"))?;
            self.sealed = Analysis::read_rendered(&owners.instance, &mut self.access, input.operation.canonical_base_revision, cx, |inferred, cx| {
                for _ in 0..MOUNTED_WORK_UNITS {
                    let fuel = cx.fuel_remaining();
                    if self.original.advance(input.snapshot, inferred, cx)? { return Ok::<_, OriginalTurnFailure>(true); }
                    if cx.fuel_remaining() == fuel || cx.should_yield() || self.original.stage() != stage { break; }
                }
                Ok(false)
            })?.transpose()?.unwrap_or(false);
        }
        Ok(Self::progress(stage, fuel, cx))
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn terminal_is_empty(&self) -> bool { self.original.terminal_is_empty() && self.owners.terminal_is_empty() && self.access.terminal_is_empty() }

    fn terminal_frame_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }

    fn next_close_capacity_byte_demand(&self, bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demand(bytes)?.capacity_bytes) }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.closing { return Ok(InteractiveJobCloseStep::Blocked); }
        if !self.original.terminal_is_empty() {
            self.original.close_step(cx)?;
            return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() });
        }
        let demand = self.close_demand(cx.retained_grant().maximum_copy_bytes)?;
        semio_framework_job::close_original_effect_step(cx, demand, |effect| self.owners.step(effect.grant().maximum_copy_bytes, effect))
    }
}
