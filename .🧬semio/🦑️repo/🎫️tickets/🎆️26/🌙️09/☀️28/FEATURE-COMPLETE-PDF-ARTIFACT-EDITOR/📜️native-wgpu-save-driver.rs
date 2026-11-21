//! 📥️ Drives the native Save client through the existing controlled SPR media exchange.

use super::sink::NativeDownloadSink;
use semio_os_kernel::{AppCommand, AppDocumentIdentity, AppFrame, MediaExportHandleWire, MediaExportStateWire, pack_rt};
use semio_plugin::app::{MediaArtifactDescriptor, MediaWireFormat};
use semio_framework_value::FromValue;
use std::{path::Path, time::{Duration, Instant}};

pub(super) const CHUNK: usize = semio_plugin::ArtifactOutputChunks::CHUNK_BYTES;
pub(super) const MAX_TOTAL: usize = semio_plugin::ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES;
const MAX_DESCRIPTOR: usize = 4096;
const DECLINED: &str = "media-export.reference-required";

pub(super) struct NativeMediaFormat<'a> {
    pub port: &'a str,
    pub kind: &'a str,
    pub mime: &'a str,
    pub media_type: semio_framework::MediaType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeSaveOutcome {
    Delivered,
    Cancelled,
    ReferenceRequired,
}

pub(super) trait NativeMediaPeer {
    fn next_seq(&mut self) -> u64;
    async fn request(&mut self, command: AppCommand, sequence: u64) -> Result<AppFrame, String>;
    async fn current(&mut self, instance: u32) -> Result<AppDocumentIdentity, String>;
}

pub(super) async fn save(
    peer: &mut impl NativeMediaPeer,
    owner: &AppDocumentIdentity,
    format: &NativeMediaFormat<'_>,
    path: &Path,
    duration: Duration,
    cancelled: impl Fn() -> bool,
    mut progress: impl FnMut(u64),
) -> Result<NativeSaveOutcome, String> {
    let deadline = Instant::now().checked_add(duration).ok_or("invalid Save duration")?;
    let current = || !cancelled() && Instant::now() < deadline;
    if !current() { return Ok(NativeSaveOutcome::Cancelled); }
    let document = owner.parent_document_id.as_deref().ok_or("Save requires an attached document")?;
    let seq = peer.next_seq();
    let submitted = peer.request(AppCommand::SubmitMediaExport {
        seq, port: format.port.into(), expected_parent_document_id: document.into(),
        expected_base_revision: owner.base_revision, checkpoint_id: None,
    }, seq).await?;
    let AppFrame::MediaExportSubmitted { in_reply_to, handle } = submitted else { return Err("invalid Save submission result".into()); };
    if in_reply_to != seq || handle.app_instance_id != owner.app_instance_id || handle.parent_document_id != document
        || handle.base_revision != owner.base_revision || handle.operation_id == 0 || handle.generation == 0
    { return Err("Save submission does not match the attached document".into()); }
    let mut settled = false;
    let result = drive(peer, owner, &handle, format, path, &current, &mut progress, &mut settled).await;
    if settled { return result; }
    let seq = peer.next_seq();
    let stopped = peer.request(AppCommand::CancelMediaExport { seq, handle }, seq).await;
    match (result, stopped) {
        (Ok(NativeSaveOutcome::Cancelled), Ok(AppFrame::Done { in_reply_to })) if in_reply_to == seq => Ok(NativeSaveOutcome::Cancelled),
        (Ok(NativeSaveOutcome::Cancelled), _) => Err("Save cancellation did not settle its operation".into()),
        (result, _) => result,
    }
}

async fn drive(
    peer: &mut impl NativeMediaPeer,
    owner: &AppDocumentIdentity,
    handle: &MediaExportHandleWire,
    format: &NativeMediaFormat<'_>,
    path: &Path,
    current: &impl Fn() -> bool,
    progress: &mut impl FnMut(u64),
    settled: &mut bool,
) -> Result<NativeSaveOutcome, String> {
    let mut applied = 0;
    let (total, mime_type, descriptor) = loop {
        if !current() { return Ok(NativeSaveOutcome::Cancelled); }
        let seq = peer.next_seq();
        let frame = peer.request(AppCommand::PollMediaExport { seq, handle: handle.clone() }, seq).await?;
        let AppFrame::MediaExportStatus { in_reply_to, handle: answered, state, applied_progress, mime_type, total_bytes, descriptor, detail, .. } = frame else {
            return Err("invalid Save status result".into());
        };
        if in_reply_to != seq || answered != *handle { return Err("foreign Save status result".into()); }
        applied = applied.max(applied_progress);
        progress(applied);
        match state {
            MediaExportStateWire::Running => semio_framework_async::yield_once().await,
            MediaExportStateWire::Cancelled => { *settled = true; return Ok(NativeSaveOutcome::Cancelled); }
            MediaExportStateWire::Failed => {
                *settled = true;
                if total_bytes == 0 && descriptor.is_empty() && detail == DECLINED { return Ok(NativeSaveOutcome::ReferenceRequired); }
                return Err(detail);
            }
            MediaExportStateWire::Complete => break (total_bytes, mime_type, descriptor),
        }
    };
    let mut refusal = if mime_type != format.mime || total > MAX_TOTAL as u64 || !descriptor_matches(&descriptor, format) {
        Some("Save output does not match the declared media format".to_owned())
    } else { None };
    let mut sink = if current() && refusal.is_none() {
        match NativeDownloadSink::create(path,total) {
            Ok(sink) => Some(sink),
            Err(error) => { refusal = Some(error.to_string()); None }
        }
    } else { None };
    let mut written = 0;
    let maximum_pages = MAX_TOTAL / CHUNK;
    for page in 0..=maximum_pages {
        let seq = peer.next_seq();
        let frame = peer.request(AppCommand::TakeMediaExportChunk { seq, handle: handle.clone() }, seq).await?;
        let AppFrame::MediaExportChunk { in_reply_to, handle: answered, data, terminal } = frame else { return Err("invalid Save page result".into()); };
        if in_reply_to != seq || answered != *handle { return Err("foreign Save page result".into()); }
        if data.len() > CHUNK || page == maximum_pages && (!terminal || !data.is_empty()) || !terminal && data.is_empty() {
            refusal.get_or_insert_with(|| "Save page exceeds its admitted bounds".into());
        }
        written = u64::try_from(data.len()).ok().and_then(|extent| u64::checked_add(written, extent))
            .ok_or("Save length overflow")?;
        if written > total || terminal && written != total {
            refusal.get_or_insert_with(|| "incomplete Save page sequence".into());
        }
        if current() && refusal.is_none() && !data.is_empty()
            && let Some(active) = sink.as_mut()
            && let Err(error) = active.write_page(&data)
        {
            refusal = Some(error.to_string());
        }
        progress(written.max(applied));
        if terminal {
            *settled = true;
            if !current() { return Ok(NativeSaveOutcome::Cancelled); }
            if let Some(error) = refusal { return Err(error); }
            let ready = sink.ok_or("Save lost its staged destination")?.prepare().map_err(|error| error.to_string())?;
            if !current() || peer.current(owner.app_instance_id).await? != *owner || !current() {
                return Ok(NativeSaveOutcome::Cancelled);
            }
            ready.commit().map_err(|error| error.to_string())?;
            return Ok(NativeSaveOutcome::Delivered);
        }
        semio_framework_async::yield_once().await;
    }
    Err(refusal.unwrap_or_else(|| "Save page sequence has no terminal reply".into()))
}

fn descriptor_matches(bytes: &[u8], format: &NativeMediaFormat<'_>) -> bool {
    if bytes.is_empty() || bytes.len() > MAX_DESCRIPTOR { return false; }
    let Ok(value) = pack_rt::decode_wire_value(bytes) else { return false; };
    let Ok(descriptor) = MediaArtifactDescriptor::from_value(value) else { return false; };
    descriptor.port_id.as_deref() == Some(format.port) && descriptor.media_type == Some(format.media_type)
        && matches!(descriptor.wire, MediaWireFormat::Binary { format_kind } if format_kind == format.kind)
}
