//! 🧪 Exercises the actual SPR writer and mounted first archive under an Original caller.
#[path = "native-first-genesis-history.rs"]
mod first_genesis;
#[path = "native-first-genesis-archive.rs"]
mod first_archive;
#[path = "native-first-archive-mounted-owner.rs"]
mod mounted;

extern crate self as semio_os_spr;
#[cfg(first_archive_without_spr_cache)]
#[path = "native-first-history-reference.rs"]
mod cold_spr;
pub mod history { pub use crate::first_genesis::FirstGenesisHistory; }
pub mod channel {
    #[cfg(first_archive_without_spr_cache)]
    use crate::cold_spr;
    pub use cold_spr::channel::{DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_VERSION};
    pub const FIRST_GENESIS_ARCHIVE_PAGE_BYTES: usize = 4096;
    pub use cold_spr::AppDocumentIdentity;
    mod exchange { include!(env!("SEMIO_FIRST_ARCHIVE_EXCHANGE")); }
    pub use exchange::*;
}
pub mod wire_fixture { include!(env!("SEMIO_FIRST_ARCHIVE_WIRE")); }
#[cfg(not(first_archive_without_spr_cache))]
use cold_spr::history::{EncodeOptions, HistoryComposition, HistoryLog, REC_COMPOSITION};
#[cfg(first_archive_without_spr_cache)]
use cold_spr::REC_COMPOSITION;
use semio_framework_pack_source::PackSink;
use protocol::{format::{HEADER_SIZE, SprWriter, WriteOptions}, ProtocolError};
use first_archive::FirstGenesisArchiveCursor;
use first_genesis::FirstGenesisHistory;
use channel::{FirstArchiveAction as Action, FirstArchiveKey, FirstArchiveReply as Reply, FirstArchiveRequest};
use semio_framework_job::{
    CancelToken, Generation, JobPayloadAuthority, OperationId, OriginalCallerSourceAdmission,
    OriginalRetirementPump, OriginalTurnFailure, StepBudget,
};
use semio_framework_value::{RetainedCloneProgress, RetainedWorkPoll};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant};
use semio_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisPack, ArtifactGenesisPackPages};
use std::{future::Future, task::{Context, Poll, Waker}};

const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 65536,
    maximum_release_bytes: 65536, maximum_depth: 64,
};
const KEY: FirstArchiveKey = FirstArchiveKey {
    instance: 42, operation: 884, generation: 7, sequence: 9,
};

fn evaluate<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("in-memory reference codec unexpectedly suspended"),
    }
}

fn canonical(doc: &str, schema: &str, dialect: Option<[&str; 3]>) -> Vec<u8> {
    #[cfg(not(first_archive_without_spr_cache))]
    let log = HistoryLog {
        doc_id: doc.into(), schema: schema.into(),
        composition: dialect.map(|[a, b, c]| HistoryComposition { owner: None, dialect: Some((a.into(), b.into(), c.into())) }),
        ..Default::default()
    };
    #[cfg(not(first_archive_without_spr_cache))]
    let expected = evaluate(cold_spr::encode_history(&log, &EncodeOptions::default())).unwrap();
    #[cfg(first_archive_without_spr_cache)]
    let expected = evaluate(cold_spr::canonical(doc, schema, dialect)).unwrap();
    let plan = FirstGenesisHistory::new(doc, schema, dialect).unwrap();
    let mut actual = vec![0; plan.extent()];
    plan.encode(&mut actual).unwrap();
    assert_eq!(actual, expected);
    #[cfg(not(first_archive_without_spr_cache))]
    {
        let read = evaluate(cold_spr::decode_history(&actual, &cold_spr::DecodeOptions {
            verification: protocol::format::VerificationLevel::Full, ..Default::default()
        })).unwrap();
        assert_eq!(read.doc_id, doc);
        assert_eq!(read.schema, schema);
        assert_eq!(read.composition, log.composition);
    }
    actual
}

fn main() {
    let doc = "test:42090833-e98c-45cf-8040-dfa9008138a9";
    let dialect = Some(["document", "pdf", "1.7"]);
    let schema = "s.pdf.document/1";
    let spr = canonical(doc, schema, dialect);
    let repeated = canonical("repeated", "repeated", Some(["repeated", "repeated", "repeated"]));
    let unicode = "ü".repeat(135);
    let long = canonical(&unicode, schema, None);
    assert!(FirstGenesisHistory::new(&"x".repeat(4096), schema, None).is_err());
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    std::fs::write(&args[0], [&spr[..], &repeated, &long].concat()).unwrap();

    let pack = std::fs::read(&args[1]).unwrap();
    #[cfg(not(first_archive_without_spr_cache))]
    let expected = cold_spr::encode_document_archive_bytes(&cold_spr::DocumentArchivePack {
        parent_pack: pack.clone(), parent_spr: spr, members: Vec::new(),
    }).unwrap();
    #[cfg(first_archive_without_spr_cache)]
    let expected = cold_spr::archive(&pack, &spr);
    let mut note = |_| true;
    let mut control = NativeEncodeControl::new(1 << 25, &mut note);
    let pages = ArtifactGenesisPackPages::copy_encoded(&pack, &mut control).unwrap();
    let digest = *semio_framework_hash::hash(&pack).as_bytes();
    let (genesis, _) = AdmittedArtifactGenesis::admit_verified_paged_pack((), pages, digest,
        RetainedCloneGrant { maximum_capacity_bytes: 1 << 27, ..TURN }).unwrap();
    assert!(matches!(genesis.pack(), ArtifactGenesisPack::Paged(_)));

    let stock = RetainedCloneGrant { maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28,
        maximum_capacity_bytes: 1 << 27, maximum_release_bytes: 1 << 27, maximum_depth: 64 };
    let clock = || Some(1);
    let mut cancel = Some(CancelToken::admit_root(stock).result.unwrap().unwrap().0);
    let mut actual = None;
    let mut authority = Some(JobPayloadAuthority::admit(OperationId(KEY.operation), Generation(KEY.generation),
        stock, &mut actual).result.unwrap().unwrap().0);
    let mut admission = OriginalCallerSourceAdmission::new();
    let mut parent = None;
    for stage in 0..3 {
        let mut pump = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, clock, stock).unwrap();
        let (receipt, _) = pump.with_effect(|effect| match stage {
            0 => admission.prepare(&mut authority, &mut cancel, effect),
            1 => admission.admit_backing(effect),
            _ => admission.publish(&mut parent, effect),
        }).unwrap().unwrap();
        receipt.result.unwrap();
    }
    let mut parent = parent.expect("fixture issued the genuine retained Original source");
    let dispatch = parent.issue_dispatch().unwrap();
    let mut sequence = 0;
    let mut spent = RetainedCloneProgress::default();
    let mut app = mounted::MountedFirstArchive::default();
    let frontier = (7, [5; 32]);
    let plan = FirstGenesisHistory::new(doc, schema, dialect).unwrap();
    let mut output = Vec::with_capacity(4140);
    let mut turn = |app: &mut mounted::MountedFirstArchive, action, key, current, grant| {
        output.clear();
        let request = FirstArchiveRequest { key, action };
        let mut cx = parent.borrow_context_for_turn(&dispatch, StepBudget::new(1 << 27, u64::MAX, stock),
            clock, &mut sequence, &mut spent).unwrap();
        cx.bind_original_clock_binding(1).unwrap();
        let mut progress = RetainedCloneProgress::default();
        cx.with_original_work_loan(1 << 20, u64::MAX, |cx| cx.with_retained_loan(grant, |cx| {
            let before = cx.retained_progress();
            app.exchange(request, doc, current, frontier, genesis.pack(), Some(&plan), &mut output, cx)?;
            let after = cx.retained_progress();
            progress.retained_capacity_bytes = after.retained_capacity_bytes - before.retained_capacity_bytes;
            progress.released_bytes = after.released_bytes - before.released_bytes;
            Ok::<(), OriginalTurnFailure>(())
        })).unwrap();
        (request, output.clone(), progress)
    };
    let denied = RetainedCloneGrant { maximum_capacity_bytes: 0, ..TURN };
    let (_, output, progress) = turn(&mut app, Action::Open, KEY, frontier, denied);
    assert!(output.is_empty() && !app.has_resident());
    assert_eq!(progress.retained_capacity_bytes, 0);
    let (request, output, progress) = turn(&mut app, Action::Open, KEY, frontier, TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Opened { extent, .. }) if extent as usize == expected.len()));
    assert!(progress.retained_capacity_bytes < 16384);
    let other = FirstArchiveKey { sequence: KEY.sequence + 1, ..KEY };
    let (request, output, _) = turn(&mut app, Action::Preview { offset: 0 }, other, frontier, TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Refused)));

    let mut delivered = 0;
    let mut digest = semio_framework_hash::Sha256::new();
    while delivered < expected.len() {
        let offset = delivered as u32;
        let (request, output, progress) = turn(&mut app, Action::Preview { offset }, KEY, frontier, TURN);
        let Some(Reply::Page(bytes)) = Reply::decode(request, &output) else { panic!("expected bounded archive page"); };
        assert_eq!(progress.retained_capacity_bytes, 0);
        assert!(bytes.len() <= 4096 && bytes == &expected[delivered..delivered + bytes.len()]);
        digest.update(bytes);
        let length = bytes.len() as u16;
        if offset == 0 {
            let (request, output, _) = turn(&mut app, Action::Preview { offset }, KEY, frontier, denied);
            assert!(matches!(Reply::decode(request, &output), Some(Reply::Page(retry)) if retry == bytes));
            let action = Action::Acknowledge { offset, length: length - 1 };
            let (request, output, _) = turn(&mut app, action, KEY, frontier, TURN);
            assert!(matches!(Reply::decode(request, &output), Some(Reply::Refused)));
        }
        let action = Action::Acknowledge { offset, length };
        let (request, output, _) = turn(&mut app, action, KEY, frontier, TURN);
        assert!(matches!(Reply::decode(request, &output), Some(Reply::Acknowledged)));
        delivered += length as usize;
    }
    let expected_digest = digest.finalize();
    let (request, output, _) = turn(&mut app, Action::Seal, KEY, frontier, TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Sealed(digest)) if digest == expected_digest));
    let (request, output, _) = turn(&mut app, Action::Seal, KEY, (8, frontier.1), TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Refused)));
    let denied_release = RetainedCloneGrant { maximum_release_bytes: 0, ..TURN };
    let (request, output, _) = turn(&mut app, Action::Close, KEY, frontier, denied_release);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Pending)) && app.has_resident());
    let (request, output, progress) = turn(&mut app, Action::Close, KEY, frontier, TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Closed)) && !app.has_resident());
    assert!(progress.released_bytes < 16384);
    let (request, output, _) = turn(&mut app, Action::Close, KEY, frontier, TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Closed)));
    let mut cancelled = mounted::MountedFirstArchive::default();
    turn(&mut cancelled, Action::Open, KEY, frontier, TURN);
    turn(&mut cancelled, Action::Preview { offset: 0 }, KEY, frontier, TURN);
    let (request, output, _) = turn(&mut cancelled, Action::Cancel, KEY, (8, frontier.1), TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Closed)) && !cancelled.has_resident());
    let (request, output, _) = turn(&mut cancelled, Action::Seal, KEY, frontier, TURN);
    assert!(matches!(Reply::decode(request, &output), Some(Reply::Refused)));

    drop(dispatch);
    for _ in 0..128 {
        if parent.terminal_is_empty() { break; }
        let mut pump = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, clock, stock).unwrap();
        let (receipt, _) = pump.with_effect(|effect| parent.close_step(effect)).unwrap().unwrap();
        match receipt {
            RetainedWorkPoll::Complete(result) => { result.result.unwrap(); }
            RetainedWorkPoll::Pending { .. } => {}
        }
    }
    assert!(parent.terminal_is_empty());
    println!("canonical SPR and paid mounted archive agree across {delivered} bytes");
}
