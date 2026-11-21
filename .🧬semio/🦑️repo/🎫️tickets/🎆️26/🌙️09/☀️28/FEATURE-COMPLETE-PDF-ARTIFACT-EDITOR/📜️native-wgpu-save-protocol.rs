//! 🧪 Tests checked native streaming with actual SPR AppCommand and AppFrame codecs.
use semio_framework::{MediaClass, MediaForm, MediaType};
use semio_framework_value::ToValue;
use semio_os_kernel::{AppCommand, AppDocumentIdentity, AppFrame, MediaExportHandleWire, MediaExportStateWire, pack_rt};
use semio_os_spr::{decode_app_frame, encode_app_command, encode_app_frame};
use semio_plugin_decl_app::MediaArtifactDescriptor;
use semio_framework::manifest::MediaWireFormat;
use super::driver::{self, NativeMediaFormat, NativeMediaPeer, NativeSaveOutcome};
use std::{cell::Cell, future::Future, path::Path, task::{Context, Poll, Waker}, time::Duration};

const MEDIA: MediaType = MediaType { class: MediaClass::Presentation, form: MediaForm::Any };
const REVISION: u64 = 0x2030_4050_6070_8010;
const PORT: &str = semio_framework::NATURAL_FILE_PORT_ID;
const FORMAT: &str = "s.stdio.pdf";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Law { Good, WrongDescriptor, WrongPageHandle, InlineTerminal, Short, Stale, CancelReady, CancelWaiting, BadDestination,
    Reference, ReferenceLateStale, ReferenceEarlyStale, ReferenceBadFormat, ReferenceCancelled, ReferenceUndeclared }

impl Law {
    fn declines_optimized(self) -> bool {
        matches!(self, Self::Reference | Self::ReferenceLateStale | Self::ReferenceEarlyStale |
            Self::ReferenceBadFormat | Self::ReferenceCancelled | Self::ReferenceUndeclared)
    }

    fn produces_reference(self) -> bool {
        self.declines_optimized() && !matches!(self, Self::ReferenceEarlyStale | Self::ReferenceUndeclared)
    }
}

fn wait<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..100 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) { return value; }
    }
    panic!("native Save fixture left its bounded transport schedule")
}

fn owner() -> AppDocumentIdentity {
    AppDocumentIdentity { app_instance_id: 51, base_revision: REVISION, parent_document_id: Some("pdf/fixture".into()) }
}

struct Peer<'a> {
    law: Law,
    sequence: u64,
    poll: usize,
    page: usize,
    stopped: usize,
    rechecks: usize,
    references: usize,
    cancel: &'a Cell<bool>,
}

impl Peer<'_> {
    fn handle(&self) -> MediaExportHandleWire {
        MediaExportHandleWire { app_instance_id: owner().app_instance_id, parent_document_id: owner().parent_document_id.unwrap(),
            operation_id: 0x2100_0000_0000_0003, base_revision: REVISION, generation: 0x3200_0000_0000_0005 }
    }

    fn descriptor(&self) -> Vec<u8> {
        let port = if self.law == Law::WrongDescriptor { "unrelated" } else { PORT };
        let descriptor = MediaArtifactDescriptor {
            edge_id: None, port_id: Some(port.into()), kind_id: None, media_type: Some(MEDIA),
            wire: MediaWireFormat::Binary { format_kind: FORMAT.into() }, blob_hash: None,
        };
        pack_rt::encode_wire_value(&descriptor.to_value())
    }
}

impl NativeMediaPeer for Peer<'_> {
    fn next_seq(&mut self) -> u64 { self.sequence += 1; self.sequence }

    async fn request(&mut self, command: AppCommand, seq: u64) -> Result<AppFrame, String> {
        assert!(encode_app_command(&command).await.is_ok());
        let expected = self.handle();
        let reply = match command {
            AppCommand::SubmitMediaExport { seq: got, port, expected_parent_document_id, expected_base_revision, checkpoint_id } => {
                assert_eq!((got, port.as_str(), expected_parent_document_id.as_str(), expected_base_revision, checkpoint_id),
                    (seq, PORT, "pdf/fixture", REVISION, None));
                AppFrame::MediaExportSubmitted { in_reply_to: seq, handle: expected }
            }
            AppCommand::PollMediaExport { seq: got, handle } => {
                assert_eq!((got, &handle), (seq, &expected));
                self.poll += 1;
                let ready = self.poll > 1;
                if ready && self.law.declines_optimized() {
                    let refused = AppFrame::MediaExportStatus { in_reply_to: seq, handle,
                        applied_progress: 0, state: MediaExportStateWire::Failed, checkpoint_available: false,
                        mime_type: String::new(), total_bytes: 0, detail: "media-export.reference-required".into(), descriptor: vec![],
                    };
                    return decode_app_frame(&encode_app_frame(&refused).await).await.map_err(|error| error.to_string());
                }
                if self.law == Law::CancelWaiting { self.cancel.set(true); }
                AppFrame::MediaExportStatus { in_reply_to: seq, handle, applied_progress: self.poll as u64,
                    state: if ready { MediaExportStateWire::Complete } else { MediaExportStateWire::Running },
                    checkpoint_available: false, mime_type: if ready { "application/pdf".into() } else { String::new() },
                    total_bytes: if ready { 13 } else { 0 }, detail: String::new(),
                    descriptor: if ready { self.descriptor() } else { vec![] },
                }
            }
            AppCommand::TakeMediaExportChunk { seq: got, mut handle } => {
                assert_eq!((got, &handle), (seq, &expected));
                let (data, terminal) = match (self.page, self.law) {
                    (0, Law::WrongPageHandle) => { handle.generation ^= 1; (b"%PDF-1.".to_vec(), false) }
                    (0, Law::InlineTerminal) => (b"%PDF-1.7\nEOF\n".to_vec(), true),
                    (0, _) => (b"%PDF-1.".to_vec(), false),
                    (1, Law::Short) => (vec![], true),
                    (1, _) => (b"7\nEOF\n".to_vec(), false),
                    (2, _) => (vec![], true),
                    _ => panic!("attempted another page after terminal"),
                };
                self.page += 1;
                if self.law == Law::CancelReady { self.cancel.set(true); }
                AppFrame::MediaExportChunk { in_reply_to: seq, handle, data, terminal }
            }
            AppCommand::MediaOut { seq: got, port, request } => {
                assert_eq!((got, port.as_str(), request.is_empty()), (seq, PORT, true));
                assert!(self.law.produces_reference());
                self.references += 1;
                let kind = if self.law == Law::ReferenceBadFormat { "foreign" } else { FORMAT };
                let descriptor = MediaArtifactDescriptor {
                    edge_id: None, port_id: Some(port.clone()), kind_id: Some(kind.into()), media_type: None,
                    wire: MediaWireFormat::Binary { format_kind: kind.into() }, blob_hash: None,
                };
                if self.law == Law::ReferenceCancelled { self.cancel.set(true); }
                AppFrame::Media { in_reply_to: seq, port, descriptor: pack_rt::encode_wire_value(&descriptor.to_value()),
                    data: b"%PDF-1.7\nREF\n".to_vec() }
            }
            AppCommand::CancelMediaExport { seq: got, handle } => {
                assert_eq!((got, &handle), (seq, &expected));
                self.stopped += 1;
                AppFrame::Done { in_reply_to: seq }
            }
            _ => panic!("foreign command in native Save fixture"),
        };
        decode_app_frame(&encode_app_frame(&reply).await).await.map_err(|error| error.to_string())
    }

    async fn current(&mut self, instance: u32) -> Result<AppDocumentIdentity, String> {
        assert_eq!(instance, owner().app_instance_id);
        self.rechecks += 1;
        let mut current = owner();
        if matches!(self.law, Law::Stale | Law::ReferenceEarlyStale)
            || self.law == Law::ReferenceLateStale && self.rechecks > 1 { current.base_revision ^= 1; }
        Ok(current)
    }
}

fn run(law: Law, filename: &str) -> (Result<NativeSaveOutcome, String>, usize, usize, usize, Vec<u8>) {
    let directory = Path::new(file!()).parent().unwrap().join("🗑️generated/native-wgpu-save/law").join(filename);
    std::fs::create_dir_all(&directory).unwrap();
    let path = directory.join("saved.pdf");
    std::fs::write(&path, b"previous file").unwrap();
    let destination = if law == Law::BadDestination { path.join("nested.pdf") } else { path.clone() };
    let cancel = Cell::new(false);
    let mut peer = Peer { law, sequence: 0x2000_0000_0000_0000, poll: 0, page: 0, stopped: 0, rechecks: 0, references: 0, cancel: &cancel };
    let format = NativeMediaFormat { port: PORT, kind: FORMAT, mime: "application/pdf", media_type: MEDIA,
        reference: law.declines_optimized() && law != Law::ReferenceUndeclared };
    let mut progress = 0;
    let result = wait(driver::save(&mut peer, &owner(), &format, &destination, Duration::from_secs(5),
        || cancel.get(), |current| { assert!(progress <= current); progress = current; }));
    assert_eq!(std::fs::read_dir(&directory).unwrap().count(), 1, "staged path must retire on every terminal");
    assert_eq!(peer.references, usize::from(law.produces_reference()));
    (result, peer.stopped, peer.rechecks, peer.page, std::fs::read(path).unwrap())
}

#[test]
fn replaces_a_destination_after_empty_terminal_and_identity_recheck() {
    let (result, stopped, checked, pages, bytes) = run(Law::Good, "good");
    assert_eq!(result.unwrap(), NativeSaveOutcome::Delivered);
    assert_eq!((stopped, checked, pages), (0, 1, 3));
    assert_eq!(bytes, b"%PDF-1.7\nEOF\n");
}

#[test]
fn refuses_invalid_descriptor_handle_terminal_and_extent() {
    for (law, file, stops, drained) in [(Law::WrongDescriptor, "descriptor", 0, 3), (Law::WrongPageHandle, "handle", 1, 1),
        (Law::InlineTerminal, "inline-terminal", 0, 1), (Law::Short, "short", 0, 2),
        (Law::BadDestination, "destination", 0, 3)] {
        let (result, stopped, checked, pages, bytes) = run(law, file);
        assert!(result.is_err(), "{file}");
        assert_eq!((stopped, checked, pages), (stops, 0, drained));
        assert_eq!(bytes, b"previous file");
    }
}

#[test]
fn drains_ready_save_on_stale_or_cancelled_owner_and_cancels_pending_save() {
    for (law, file, stops, checks, drained) in [(Law::Stale, "stale", 0, 1, 3),
        (Law::CancelReady, "cancel-ready", 0, 0, 3), (Law::CancelWaiting, "cancel-waiting", 1, 0, 0)] {
        let (result, stopped, checked, pages, bytes) = run(law, file);
        assert_eq!(result.unwrap(), NativeSaveOutcome::Cancelled);
        assert_eq!((stopped, checked, pages), (stops, checks, drained));
        assert_eq!(bytes, b"previous file");
    }
}

#[test]
fn opens_reference_only_after_exact_refusal_and_keeps_the_original_revision() {
    let (result, stopped, checked, pages, bytes) = run(Law::Reference, "reference");
    assert_eq!(result.unwrap(), NativeSaveOutcome::Delivered);
    assert_eq!((stopped, checked, pages), (0, 2, 0));
    assert_eq!(bytes, b"%PDF-1.7\nREF\n");
    for (law, name, checks) in [
        (Law::ReferenceLateStale, "reference-late-stale", 2),
        (Law::ReferenceEarlyStale, "reference-early-stale", 1),
        (Law::ReferenceCancelled, "reference-cancelled", 1),
    ] {
        let (result, stopped, checked, pages, bytes) = run(law, name);
        assert_eq!(result.unwrap(), NativeSaveOutcome::Cancelled);
        assert_eq!((stopped, checked, pages), (0, checks, 0));
        assert_eq!(bytes, b"previous file");
    }
    let (result, _, checked, _, bytes) = run(Law::ReferenceBadFormat, "reference-wrong-format");
    assert!(result.is_err());
    assert_eq!((checked, bytes.as_slice()), (1, &b"previous file"[..]));
    let (result, _, checked, _, bytes) = run(Law::ReferenceUndeclared, "reference-undeclared");
    assert_eq!(result.unwrap(), NativeSaveOutcome::ReferenceRequired);
    assert_eq!((checked, bytes.as_slice()), (0, &b"previous file"[..]));
}

#[test]
fn accepts_only_the_correlated_reply_from_the_kernel_frame_batch() {
    let seq = 0x2000_0000_0000_0001;
    let done = || AppFrame::Done { in_reply_to: seq };
    let extra = AppFrame::DocumentIdentity { in_reply_to: seq + 1, identity: owner() };
    let fault = |_: &[u8], _: &[u8]| "operational refusal".to_string();
    assert!(matches!(driver::one_save_reply(vec![extra, done()], seq, fault).unwrap(),
        AppFrame::Done { in_reply_to } if in_reply_to == seq));
    assert!(driver::one_save_reply(vec![done(), done()], seq, fault).is_err());
    assert!(driver::one_save_reply(vec![done()], seq + 1, fault).is_err());
    assert!(driver::one_save_reply(vec![], seq, fault).is_err());
    let media = AppFrame::Media { in_reply_to: seq, port: "artifact:native".into(), descriptor: vec![1], data: vec![2] };
    assert!(matches!(driver::one_save_reply(vec![media], seq, fault).unwrap(), AppFrame::Media { .. }));
    let refused = || AppFrame::Error { in_reply_to: Some(seq), fault: b"paid".to_vec(), report: b"fault".to_vec() };
    let message = |body: &[u8], report: &[u8]| String::from_utf8([body, report].concat()).unwrap();
    assert_eq!(driver::one_save_reply(vec![refused()], seq, message).err().as_deref(), Some("paidfault"));
    assert_eq!(driver::one_save_reply(vec![refused()], seq + 1, message).err().as_deref(),
        Some("native Save received a foreign protocol answer"));
}
