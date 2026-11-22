//! 🧪 Transfers a real paid genesis Pack to the staged native folder append.
#[path = "native-first-genesis-archive.rs"]
mod writer;
use writer::FirstGenesisArchive;
mod scoped { include!(env!("SEMIO_DISK_SCOPED")); }
mod disk { include!(env!("SEMIO_DISK_APPEND")); }
mod archive_readback { include!(env!("SEMIO_DISK_READBACK")); }
mod receipt { include!(env!("SEMIO_DISK_RECEIPT")); }
#[allow(dead_code)]
mod first_archive_exchange { include!(env!("SEMIO_DISK_EXCHANGE")); }
use first_archive_exchange::FirstArchiveKey;
mod protocol { pub mod wire { pub mod command_ingress { include!(env!("SEMIO_DISK_NATURAL_WIRE")); } } }
mod proof_wire {
    pub use semio_os_spr::channel::DOCUMENT_ARCHIVE_VERSION;
    pub use super::receipt::*;
    pub use super::first_archive_exchange::*;
}
use semio_os_spr::{AppDocumentIdentity, DOCUMENT_ARCHIVE_MAXIMUM_BYTES};
include!(env!("SEMIO_DISK_FOLDER"));

use disk::{FirstArchiveCommit, NativeFirstArchiveCapture};
use protocol::wire::command_ingress::*;
use receipt::{FirstGenesisArchiveOffer, FirstGenesisArchivePageReceipt, FirstGenesisArchiveScope, FirstGenesisArchiveSeal};
use scoped::ScopedFirstGenesisArchive;
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant};
use semio_framework_value_testing_allocator::{LARGEST_ALLOCATION, OBSERVE_ALLOCATIONS};
use semio_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisPack, ArtifactGenesisPackPages};
use std::{fs::{self, File}, future::Future, path::PathBuf, sync::atomic::Ordering, task::{Context, Poll, Waker}};

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 4096,
};

async fn now_ms() -> u64 { 1 }
fn is_repository_root(_: &std::path::Path) -> Result<bool, String> { Ok(false) }

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

fn complete<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop { if let Poll::Ready(output) = future.as_mut().poll(&mut cx) { return output; } }
}

fn capture(
    store: &FolderEventLogStorage, pack: &ArtifactGenesisPack, spr: &[u8], scope: &FirstGenesisArchiveScope,
    caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner,
) -> (NativeFirstArchiveCapture, FirstGenesisArchiveSeal) {
    let mut egress = ScopedFirstGenesisArchive::new(scope.clone(), FirstGenesisArchive::new(pack, spr).unwrap()).unwrap();
    let header = egress.header().clone();
    assert!(store.capture_first_genesis_archive(header.clone(), &self::scope(1)).is_err());
    let mut staged = store.capture_first_genesis_archive(header.clone(), scope).unwrap();
    let other = FirstGenesisArchiveOffer::new(self::scope(1), header.extent as usize).unwrap();
    while egress.seal().is_none() {
        let receipt = caller.turn_with_decoder(TURN, decoder, |cx| {
            let (offset, bytes) = egress.offer_page(cx)?.unwrap();
            let receipt = staged.accept_page(&header, offset, bytes).unwrap();
            if offset == 0 {
                assert!(staged.accept_page(&other, offset, bytes).is_err());
                assert_eq!(staged.accept_page(&header, offset, bytes).unwrap(), receipt);
                let mut changed = [0; 4096];
                changed[..bytes.len()].copy_from_slice(bytes);
                changed[0] ^= 1;
                assert!(staged.accept_page(&header, offset, &changed[..bytes.len()]).is_err());
                let foreign = FirstGenesisArchivePageReceipt { offer: other.clone(), ..receipt.clone() };
                assert!(egress.acknowledge(&foreign, cx).is_err());
            }
            Ok(receipt)
        }).unwrap();
        caller.turn_with_decoder(TURN, decoder, |cx| { assert!(egress.acknowledge(&receipt, cx)?); Ok(()) }).unwrap();
    }
    (staged, egress.seal().unwrap())
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let pack = fs::read(args.next().unwrap()).unwrap();
    let store = FolderEventLogStorage::new(PathBuf::from(args.next().unwrap()));
    let log = semio_os_spr::HistoryLog { doc_id: "pdf-document".into(), schema: "s.stdio.pdf@1.7".into(), ..Default::default() };
    let spr = complete(semio_os_spr::encode_history(&log, &Default::default())).unwrap();
    let expected = semio_os_spr::encode_document_archive_bytes(&semio_os_spr::DocumentArchivePack {
        parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new(),
    }).unwrap();
    let paged = {
        let mut progress = |_| true;
        let mut control = NativeEncodeControl::new(1 << 25, &mut progress);
        ArtifactGenesisPackPages::copy_encoded(&pack, &mut control).unwrap()
    };
    let digest = *semio_framework_hash::hash(&pack).as_bytes();
    let (genesis, _) = AdmittedArtifactGenesis::admit_verified_paged_pack((), paged, digest, TURN).unwrap();
    assert!(matches!(genesis.pack(), ArtifactGenesisPack::Paged(_)));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(884), Generation(7), RetainedCloneGrant {
        maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 27,
        maximum_release_bytes: 1 << 27, maximum_depth: 4096,
    }, semio_framework_job::default_now_us, u64::MAX, 1 << 27);
    caller.retain_dispatch().unwrap();
    let mut decoder = caller.admitted_decoder();
    let scope = scope(0x0020_0000_0000_0005);
    #[cfg(archive_wire_serde)]
    {
        let scope_wire = serde_json::to_value(&scope).unwrap();
        assert_eq!(scope_wire["seq"], (0x0020_0000_0000_0005u64).to_string());
        assert_eq!(serde_json::from_value::<FirstGenesisArchiveScope>(scope_wire.clone()).unwrap(), scope);
        let mut alternate = scope_wire;
        alternate["seq"] = 0x0020_0000_0000_0005u64.into();
        assert!(serde_json::from_value::<FirstGenesisArchiveScope>(alternate).is_err());
    }
    let schema = "s.stdio.pdf@1.7";

    let (staged, seal) = capture(&store, genesis.pack(), &spr, &scope, &mut caller, &mut decoder);
    let mut bad = seal.clone();
    bad.sha256[0] ^= 1;
    assert!(store.commit_first_genesis_archive_at(staged, &bad, &scope, schema, 1, || true).is_err());
    assert!(!store.event_path().exists());
    let (staged, seal) = capture(&store, genesis.pack(), &spr, &scope, &mut caller, &mut decoder);
    let cancel_during_append = || fs::metadata(store.event_path()).map_or(true, |file| file.len() <= 4096);
    assert!(store.commit_first_genesis_archive_at(staged, &seal, &scope, schema, 2, cancel_during_append).is_err());
    assert_eq!(fs::metadata(store.event_path()).unwrap().len(), 0);

    LARGEST_ALLOCATION.store(0, Ordering::Relaxed);
    OBSERVE_ALLOCATIONS.store(true, Ordering::Relaxed);
    let (staged, seal) = capture(&store, genesis.pack(), &spr, &scope, &mut caller, &mut decoder);
    assert_eq!(
        complete(store.commit_first_genesis_archive(staged, &seal, &scope, schema, || true)).unwrap(),
        FirstArchiveCommit::Created,
    );
    OBSERVE_ALLOCATIONS.store(false, Ordering::Relaxed);
    let maximum = LARGEST_ALLOCATION.load(Ordering::Relaxed);
    assert!(maximum <= 4096, "native archive allocated {maximum} bytes");
    assert!(archive_readback::archive_matches(&mut File::open(store.event_path()).unwrap(),
        fs::metadata(store.event_path()).unwrap().len(), "pdf-document", &expected, || false).unwrap());
    let events = FolderEventLogStorage::events_from(&mut File::open(store.event_path()).unwrap()).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].primary, expected);
    assert_eq!(events[0].metadata, schema);
    let (staged, seal) = capture(&store, genesis.pack(), &spr, &scope, &mut caller, &mut decoder);
    assert_eq!(store.commit_first_genesis_archive_at(staged, &seal, &scope, schema, 4, || true).unwrap(), FirstArchiveCommit::AlreadyPresent);
    let (staged, seal) = capture(&store, genesis.pack(), &spr, &scope, &mut caller, &mut decoder);
    assert!(store.commit_first_genesis_archive_at(staged, &seal, &scope, "different", 5, || true).is_err());
    assert_eq!(fs::read_dir(store.event_path().parent().unwrap()).unwrap().count(), 1);
    let hex = seal.sha256.into_iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    println!("archive {} bytes, maximum allocation {maximum}, SHA-256 {hex}", seal.offer.extent);
    for _ in 0..semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS {
        let (quote, _) = semio_framework_job::original_decoder_teardown_quote(&decoder).unwrap();
        caller.turn(quote, |cx| { decoder.close_on_source(cx)?; Ok(()) }).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(TURN);
}
