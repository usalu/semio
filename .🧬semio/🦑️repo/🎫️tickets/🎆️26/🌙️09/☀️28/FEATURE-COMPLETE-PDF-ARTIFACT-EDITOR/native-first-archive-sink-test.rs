//! 🧪 Drives the native spool and append with independently authored canonical archive pages.
extern crate self as semio_os_vcs;
mod disk { include!(env!("SEMIO_DISK_APPEND")); }
mod drive { include!(env!("SEMIO_DISK_DRIVE")); }
mod archive_readback { include!(env!("SEMIO_DISK_READBACK")); }
mod receipt { include!(env!("SEMIO_DISK_RECEIPT")); }
#[allow(dead_code)]
mod first_archive_exchange { include!(env!("SEMIO_DISK_EXCHANGE")); }
use first_archive_exchange::{FirstArchiveAction, FirstArchiveKey, FirstArchiveReply};
mod protocol { pub mod wire { #[allow(dead_code)] pub mod command_ingress { include!(env!("SEMIO_DISK_NATURAL_WIRE")); } } }
mod proof_wire {
    pub use super::first_archive_exchange::*;
    pub use super::receipt::*;
    pub const DOCUMENT_ARCHIVE_VERSION: u8 = 1;
}
use disk::{FirstArchiveCommit, NativeFirstArchiveCapture};
use protocol::wire::command_ingress::*;
use receipt::{FirstGenesisArchiveOffer, FirstGenesisArchiveScope, FirstGenesisArchiveSeal};
use semio_framework_hash::Sha256;
use std::{
    alloc::{GlobalAlloc, Layout, System}, fs::{self, File}, future::{Future, ready}, io::Read, path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll, Waker},
};
include!(env!("SEMIO_DISK_FOLDER"));
const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppDocumentIdentity { app_instance_id: u32, base_revision: u64, parent_document_id: Option<String> }
#[derive(Debug)]
pub enum VcsError { Backbone(String) }
impl std::fmt::Display for VcsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Backbone(message) => f.write_str(message) }
    }
}
async fn now_ms() -> u64 { 1 }
fn is_repository_root(_: &Path) -> Result<bool, String> { Ok(false) }

struct Meter;
static MAXIMUM: AtomicUsize = AtomicUsize::new(0);
#[global_allocator]
static ALLOCATOR: Meter = Meter;
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        MAXIMUM.fetch_max(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, value: *mut u8, layout: Layout) { unsafe { System.dealloc(value, layout); } }
    unsafe fn realloc(&self, value: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        MAXIMUM.fetch_max(size, Ordering::Relaxed);
        unsafe { System.realloc(value, layout, size) }
    }
}

fn scope(seq: u64) -> FirstGenesisArchiveScope {
    let declaration = NaturalSourceWireDeclaration { format_kind: "s.stdio.pdf@1.7".into(),
        extension: ".pdf".into(), media_type: "application/pdf".into(), binary: true };
    let extent = 9_437_877;
    let purpose = ActorInstanceOpenPurpose::NaturalSource { declaration: declaration.clone(), extent };
    let loan = NaturalSourceWireLoan { operation_id: "0020000000000002".into(), generation: "0020000000000003".into(),
        instance_id: 42, declaration, extent };
    let mut ack = NaturalSourceWireAck {
        loan: Some(loan.clone()), owner: 42, seq, kind: NATURAL_SOURCE_COMMAND_KIND, extent,
        phase: NaturalSourceWirePhase::Complete, page_index: None, result: NaturalSourceWireResult::Accepted,
    };
    let revision = [5; 32];
    let identity = AppDocumentIdentity { app_instance_id: 42, parent_document_id: Some("pdf-document".into()),
        base_revision: u64::from_be_bytes(revision[..8].try_into().unwrap()) };
    assert!(FirstGenesisArchiveScope::from_completed_source(
        &ActorInstanceOpenPurpose::Ordinary, 42, seq, &loan, &ack, identity.clone(), 7, revision,
    ).is_none());
    assert!(FirstGenesisArchiveScope::from_completed_source(&purpose, 42, seq + 1, &loan, &ack, identity.clone(), 7, revision).is_none());
    ack.loan.as_mut().unwrap().generation = "0020000000000004".into();
    assert!(FirstGenesisArchiveScope::from_completed_source(&purpose, 42, seq, &loan, &ack, identity.clone(), 7, revision).is_none());
    ack.loan = Some(loan.clone());
    let output = FirstGenesisArchiveScope::from_completed_source(&purpose, 42, seq, &loan, &ack, identity.clone(), 7, revision).unwrap();
    assert!(output.matches_current(&identity, 7, &revision));
    let mut changed = revision;
    changed[31] = 0;
    assert!(!output.matches_current(&identity, 7, &changed));
    output
}

fn capture(
    store: &FolderEventLogStorage, input: &Path, scope: &FirstGenesisArchiveScope,
) -> (NativeFirstArchiveCapture, FirstGenesisArchiveSeal) {
    let mut source = File::open(input).unwrap();
    let extent = source.metadata().unwrap().len() as usize;
    let header = FirstGenesisArchiveOffer::new(scope.clone(), extent).unwrap();
    assert!(FirstGenesisArchiveOffer::new(scope.clone(), DOCUMENT_ARCHIVE_MAXIMUM_BYTES + 1).is_none());
    let other = FirstGenesisArchiveOffer::new(self::scope(1), extent).unwrap();
    assert!(store.capture_first_genesis_archive(header.clone(), &other.scope).is_err());
    let mut staged = store.capture_first_genesis_archive(header.clone(), scope).unwrap();
    let mut page = [0; 4096];
    let mut offset = 0;
    let mut hash = Sha256::new();
    while let Some(length) = header.page_length(offset) {
        source.read_exact(&mut page[..length]).unwrap();
        let receipt = staged.accept_page(&header, offset, &page[..length]).unwrap();
        assert!(receipt.matches(&header, offset, length));
        if offset == 0 {
            assert!(staged.accept_page(&other, offset, &page[..length]).is_err());
            assert_eq!(staged.accept_page(&header, offset, &page[..length]).unwrap(), receipt);
            page[0] ^= 1;
            assert!(staged.accept_page(&header, offset, &page[..length]).is_err());
            page[0] ^= 1;
        }
        hash.update(&page[..length]);
        offset += length as u32;
    }
    assert_eq!(offset, header.extent);
    (staged, FirstGenesisArchiveSeal { offer: header, sha256: hash.finalize() })
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let input = PathBuf::from(args.next().unwrap());
    let store = FolderEventLogStorage::new(PathBuf::from(args.next().unwrap()));
    let scope = scope(0x0020_0000_0000_0005);
    let schema = "s.stdio.pdf@1.7";

    let (staged, seal) = capture(&store, &input, &scope);
    let mut bad = seal.clone();
    bad.sha256[0] ^= 1;
    assert!(store.commit_first_genesis_archive_at(staged, &bad, &scope, schema, 1, || true).is_err());
    assert!(!store.event_path().exists());
    let (staged, seal) = capture(&store, &input, &scope);
    let cancel_during_append = || fs::metadata(store.event_path()).map_or(true, |file| file.len() <= 4096);
    assert!(store.commit_first_genesis_archive_at(staged, &seal, &scope, schema, 2, cancel_during_append).is_err());
    assert_eq!(fs::metadata(store.event_path()).unwrap().len(), 0);

    MAXIMUM.store(0, Ordering::Relaxed);
    let (staged, seal) = capture(&store, &input, &scope);
    assert_eq!(store.commit_first_genesis_archive_at(staged, &seal, &scope, schema, 3, || true).unwrap(), FirstArchiveCommit::Created);
    let maximum = MAXIMUM.load(Ordering::Relaxed);
    assert!(maximum <= 4096, "native archive allocated {maximum} bytes");
    let expected = fs::read(&input).unwrap();
    assert!(archive_readback::archive_matches(&mut File::open(store.event_path()).unwrap(),
        fs::metadata(store.event_path()).unwrap().len(), "pdf-document", &expected, || false).unwrap());
    let events = FolderEventLogStorage::events_from(&mut File::open(store.event_path()).unwrap()).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].primary, expected);
    assert_eq!(events[0].metadata, schema);
    assert_eq!(events[0].key, "pdf-document");
    assert_eq!(events[0].updated_at_ms, 3);
    assert!(events[0].secondary.is_empty());
    let (staged, seal) = capture(&store, &input, &scope);
    assert_eq!(store.commit_first_genesis_archive_at(staged, &seal, &scope, schema, 4, || true).unwrap(), FirstArchiveCommit::AlreadyPresent);
    let (staged, seal) = capture(&store, &input, &scope);
    assert!(store.commit_first_genesis_archive_at(staged, &seal, &scope, "different", 5, || true).is_err());
    assert_eq!(fs::read_dir(store.event_path().parent().unwrap()).unwrap().count(), 1);
    let via_bridge = FolderEventLogStorage::new(store.folder.join("wire"));
    drive_exact_queue(&via_bridge, &expected, seal.sha256);
    let via = FolderEventLogStorage::events_from(&mut File::open(via_bridge.event_path()).unwrap()).unwrap();
    assert_eq!(via.len(), 1);
    assert_eq!(via[0].primary, expected);
    let hex = seal.sha256.into_iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    println!("archive {} bytes, maximum allocation {maximum}, SHA-256 {hex}", seal.offer.extent);
}

fn complete<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop { if let Poll::Ready(output) = future.as_mut().poll(&mut cx) { return output; } }
}

fn drive_exact_queue(storage: &FolderEventLogStorage, canonical: &[u8], digest: [u8; 32]) {
    let declaration = NaturalSourceWireDeclaration { format_kind: "s.stdio.pdf@1.7".into(),
        extension: ".pdf".into(), media_type: "application/pdf".into(), binary: true };
    let extent = 9_437_877;
    let purpose = ActorInstanceOpenPurpose::NaturalSource { declaration: declaration.clone(), extent };
    let loan = NaturalSourceWireLoan { operation_id: "0020000000000002".into(), generation: "0020000000000003".into(),
        instance_id: 42, declaration, extent };
    let accepted = NaturalSourceWireAck { loan: Some(loan), owner: 42, seq: 0x0020_0000_0000_0005,
        kind: NATURAL_SOURCE_COMMAND_KIND, extent, phase: NaturalSourceWirePhase::Complete,
        page_index: None, result: NaturalSourceWireResult::Accepted };
    let key = FirstArchiveKey::completed(&accepted).unwrap();
    let revision = [5; 32];
    let mut next = 0;
    let mut blocked = false;
    let mut closed = false;
    let identity = FirstArchiveReply::opened_identity(42, "pdf-document", &revision);
    let archived = complete(drive::persist(storage, &purpose, &accepted, &identity, "s.stdio.pdf@1.7", |request, closing| {
        assert_eq!(request.key, key);
        let body = match request.action {
            FirstArchiveAction::Open => FirstArchiveReply::Opened { extent: canonical.len() as u32,
                document: "pdf-document", generation: 7, revision },
            FirstArchiveAction::Preview { offset } if !blocked => {
                assert_eq!(offset, 0);
                blocked = true;
                FirstArchiveReply::Pending
            }
            FirstArchiveAction::Preview { offset } => {
                assert_eq!(offset, next);
                FirstArchiveReply::Page(&canonical[offset as usize..(offset as usize + 4096).min(canonical.len())])
            }
            FirstArchiveAction::Acknowledge { offset, length } => {
                assert_eq!(offset, next);
                assert_eq!(usize::from(length), (canonical.len() - next as usize).min(4096));
                next += u32::from(length);
                FirstArchiveReply::Acknowledged
            }
            FirstArchiveAction::Seal => {
                assert_eq!(next as usize, canonical.len());
                FirstArchiveReply::Sealed(digest)
            }
            FirstArchiveAction::Close => {
                assert!(closing && storage.event_path().exists());
                closed = true;
                FirstArchiveReply::Closed
            }
            FirstArchiveAction::Cancel => panic!("successful archive cannot cancel"),
        };
        let mut packet = Vec::with_capacity(body.wire_length());
        assert!(body.encode_into(request, &mut packet));
        ready(Ok(Some(packet)))
    }, || true)).unwrap();
    assert!(closed && blocked);
    assert_eq!(archived, Some(identity));
}
