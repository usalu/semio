//! 🧪 Streams an actual paged Pack through an Original caller and the canonical archive codec.
#[path = "native-first-genesis-archive.rs"]
mod first_archive;

use first_archive::FirstGenesisArchive;
use semio_framework_hash::Sha256;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant};
use semio_framework_value_testing_allocator::{LARGEST_ALLOCATION, OBSERVE_ALLOCATIONS};
use semio_os_spr::{DocumentArchivePack, EncodeOptions, HistoryLog};
use semio_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisPack, ArtifactGenesisPackPages};
use std::{
    fs::File, future::Future, io::Write, sync::atomic::Ordering, task::{Context, Poll, Waker},
};

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 1 << 20,
    maximum_release_bytes: 1 << 20, maximum_depth: 4096,
};

fn block<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(result) = future.as_mut().poll(&mut cx) { return result; }
    }
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let pack = std::fs::read(args.next().unwrap()).unwrap();
    let mut output = File::create(args.next().unwrap()).unwrap();
    let log = HistoryLog { doc_id: "pdf-document".into(), schema: "stdio.pdf.1.7".into(), ..Default::default() };
    let spr = block(semio_os_spr::encode_history(&log, &EncodeOptions::default())).unwrap();
    let expected = semio_os_spr::encode_document_archive_bytes(&DocumentArchivePack {
        parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new(),
    }).unwrap();
    let digest = *semio_framework_hash::hash(&pack).as_bytes();
    let confirmed = {
        let mut observe = |_| true;
        let mut control = NativeEncodeControl::new(1 << 25, &mut observe);
        ArtifactGenesisPackPages::copy_encoded(&pack, &mut control).unwrap()
    };
    let grant = RetainedCloneGrant { maximum_capacity_bytes: 1 << 27, ..TURN };
    let (genesis, _) = AdmittedArtifactGenesis::admit_verified_paged_pack((), confirmed, digest, grant).unwrap();
    assert!(matches!(genesis.pack(), ArtifactGenesisPack::Paged(_)));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(884), Generation(7), RetainedCloneGrant {
        maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 27,
        maximum_release_bytes: 1 << 27, maximum_depth: 4096,
    }, semio_framework_job::default_now_us, u64::MAX, 1 << 26);
    caller.retain_dispatch().unwrap();
    let mut decoder = caller.admitted_decoder();
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };

    let mut erased = FirstGenesisArchive::new(genesis.pack(), &spr).unwrap();
    caller.turn_with_decoder(TURN, &mut decoder, |cx| {
        assert!(erased.offer(cx)?.is_some());
        erased.cancel();
        assert!(erased.offer(cx)?.is_none());
        assert!(erased.digest().is_none());
        Ok(())
    }).unwrap();

    LARGEST_ALLOCATION.store(0, Ordering::Relaxed);
    OBSERVE_ALLOCATIONS.store(true, Ordering::Relaxed);
    let mut archive = FirstGenesisArchive::new(genesis.pack(), &spr).unwrap();
    assert_eq!(archive.extent(), expected.len());
    let mut delivered = 0;
    let mut oracle_digest = Sha256::new();
    caller.turn_with_decoder(denied, &mut decoder, |cx| {
        let before = cx.retained_progress();
        assert!(archive.offer(cx)?.is_none());
        assert_eq!(cx.retained_progress(), before);
        Ok(())
    }).unwrap();
    while delivered < expected.len() {
        let length = caller.turn_with_decoder(TURN, &mut decoder, |cx| {
            let (offset, bytes) = archive.offer(cx)?.expect("funded page");
            assert_eq!(delivered, offset);
            assert!(bytes.len() <= 4096);
            assert_eq!(bytes, &expected[offset..offset + bytes.len()]);
            output.write_all(bytes).unwrap();
            oracle_digest.update(bytes);
            Ok(bytes.len())
        }).unwrap();
        if delivered == 0 {
            caller.turn_with_decoder(denied, &mut decoder, |cx| {
                let (position, retained) = archive.offer(cx)?.unwrap();
                assert_eq!(position, 0);
                assert_eq!(retained, &expected[..length]);
                assert!(!archive.acknowledge(delivered, length, cx)?);
                assert!(archive.acknowledge(delivered + 1, length, cx).is_err());
                assert_eq!(archive.accepted(), 0);
                assert!(archive.digest().is_none());
                Ok(())
            }).unwrap();
        }
        caller.turn_with_decoder(TURN, &mut decoder, |cx| {
            assert!(archive.acknowledge(delivered, length, cx)?);
            Ok(())
        }).unwrap();
        delivered += length;
    }
    OBSERVE_ALLOCATIONS.store(false, Ordering::Relaxed);
    let heap = LARGEST_ALLOCATION.load(Ordering::Relaxed);
    assert!(heap <= 4096, "archive egress allocated {heap} bytes");
    assert_eq!(archive.digest(), Some(oracle_digest.finalize()));
    let digest = archive.digest().unwrap().into_iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    println!("genesis archive {delivered} bytes, maximum egress allocation {heap}, SHA-256 {digest}");
    for _ in 0..semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS {
        let (quote, _) = semio_framework_job::original_decoder_teardown_quote(&decoder).unwrap();
        caller.turn(quote, |cx| { decoder.close_on_source(cx)?; Ok(()) }).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(TURN);
}
