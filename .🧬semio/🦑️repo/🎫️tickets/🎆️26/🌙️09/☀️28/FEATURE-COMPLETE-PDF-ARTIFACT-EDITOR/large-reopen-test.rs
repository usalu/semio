//! 🧪 Checks exact receiving and retirement of an independently framed archive on disk.
#[path = "ingress.rs"]
mod ingress;
pub mod genesis { pub use semio_os_vcs::{ArtifactGenesis, ArtifactGenesisWitness}; }
#[allow(dead_code)]
#[path = "genesis.rs"]
mod physical_genesis;
#[allow(dead_code)]
#[path = "schema.rs"]
mod archive_schema;
use ingress::*;
use semio_framework_value::{AuthoringWallet, ByteReadSource, RetainedCloneGrant};
use semio_framework_value_testing_allocator::{LARGEST_ALLOCATION, OBSERVE_ALLOCATIONS};
use std::{fs::File, io::{Read, Write}, path::Path, sync::atomic::Ordering};

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 4096, maximum_copy_bytes: 1 << 18, maximum_capacity_bytes: 4096,
    maximum_release_bytes: 8192, maximum_depth: 32,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 26, maximum_copy_bytes: 1 << 31, maximum_capacity_bytes: 1 << 27,
    maximum_release_bytes: 1 << 27, maximum_depth: 32,
};
const WORK: u64 = 1 << 16;

fn run(
    input: &Path, extent: usize, limits: DocumentArchiveIngressLimits, digest: [u8; 32], mode: &str, output: &Path,
) -> (bool, usize) {
    let mut file = File::open(input).unwrap();
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 29);
    let mut cursor = match PagedDocumentArchiveIngress::new(DocumentArchiveIngressSeal { bytes: extent, sha256: digest }, limits) {
        Ok(cursor) => cursor,
        Err(_) => return (false, 0),
    };
    let mut scratch = [0; DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES];
    let mut read = 0;
    let mut fed = 0;
    let mut refused = false;
    let mut checked_body_denial = false;
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, maximum_capacity_bytes: 0, ..GRANT };
    LARGEST_ALLOCATION.store(0, Ordering::Relaxed);
    OBSERVE_ALLOCATIONS.store(true, Ordering::Relaxed);
    while !cursor.ready() {
        if fed == read {
            fed = 0;
            read = file.read(&mut scratch).unwrap();
        }
        if mode == "cancel" && cursor.accepted() > 8192 {
            cursor.cancel();
        }
        let offset = cursor.accepted();
        if mode != "foreignOffset" && (offset == 0 || offset > 8192 && !checked_body_denial && mode != "cancel") {
            let turn = wallet.run(denied, WORK, |effect| cursor.step(offset, &scratch[fed..read], effect)).unwrap().unwrap().output;
            assert_eq!(turn.work, 0);
            assert_eq!(turn.result.unwrap(), Default::default());
            assert_eq!(cursor.accepted(), offset);
            checked_body_denial = offset > 8192;
        }
        let source_offset = offset + usize::from(mode == "foreignOffset");
        let turn = wallet.run(GRANT, WORK, |effect| {
            if read == 0 && mode != "foreignOffset" {
                cursor.finish(effect)
            } else {
                cursor.step(source_offset, &scratch[fed..read], effect)
            }
        }).unwrap().unwrap().output;
        match turn.result {
            Ok(receipt) => {
                fed += cursor.accepted() - offset;
                assert!(receipt.fits(GRANT));
                assert!(turn.work > 0, "no progress at {offset} ({mode})");
                wallet.post(receipt).unwrap();
            }
            Err(error) => {
                wallet.post(error.retained_progress()).unwrap();
                refused = true;
                break;
            }
        }
    }

    let ready = cursor.ready();
    let mut pages = if ready {
        let before = cursor.accepted();
        let turn = wallet.run(GRANT, WORK, |effect| cursor.step(before, &[], effect)).unwrap().unwrap().output;
        assert_eq!(turn.work, 0);
        assert!(cursor.ready());
        let (parent, receipt) = wallet.run(GRANT, WORK, |effect| cursor.take_verified(effect)).unwrap().unwrap().output.unwrap().unwrap();
        wallet.post(receipt).unwrap();
        assert_eq!(parent.seal().sha256, digest);
        parent.into_pages()
    } else {
        assert!(refused);
        let (partial, receipt) = wallet.run(GRANT, WORK, |effect| cursor.take_unverified(effect)).unwrap().unwrap().output.unwrap().unwrap();
        wallet.post(receipt).unwrap();
        partial
    };
    if ready {
        assert_eq!(cursor.accepted(), extent);
        use physical_genesis::{AdmittedArtifactGenesis, ArtifactGenesisPack};
        let original = pages.pack.retained_bytes().contiguous_at(0).unwrap().as_ptr();
        let mut hash = semio_framework_hash::Hasher::new();
        let mut offset = 0;
        while offset < pages.pack.len() {
            let count = pages.pack.read_bytes(offset, &mut scratch);
            hash.update(&scratch[..count]);
            offset += count;
        }
        let digest = *hash.finalize().as_bytes();
        let pack = std::mem::take(&mut pages.pack);
        let (_, (), refused) = AdmittedArtifactGenesis::admit_verified_physical_pack(
            (), pack.into(), digest, RetainedCloneGrant { maximum_capacity_bytes: 0, ..GRANT },
        ).unwrap_err();
        let (genesis, receipt) = AdmittedArtifactGenesis::admit_verified_physical_pack((), refused, digest, GRANT).unwrap();
        assert!(receipt.fits(GRANT));
        let ArtifactGenesisPack::Paged(stored) = genesis.pack() else { panic!("verified physical Pack was flattened"); };
        assert_eq!(stored.retained_bytes().contiguous_at(0).unwrap().as_ptr(), original);
        let (_, actual) = genesis.into_owners();
        let ArtifactGenesisPack::Paged(stored) = std::sync::Arc::into_inner(actual).unwrap() else { unreachable!() };
        pages.pack = stored;
        for (label, source) in [("actual.pack", &pages.pack), ("actual.spr", &pages.spr)] {
            let mut file = File::create(output.join(label)).unwrap();
            let mut offset = 0;
            while offset < source.byte_len() {
                let read = source.read_bytes(offset, &mut scratch);
                assert!(read > 0);
                file.write_all(&scratch[..read]).unwrap();
                offset += read;
            }
        }
    }
    OBSERVE_ALLOCATIONS.store(false, Ordering::Relaxed);
    let heap = LARGEST_ALLOCATION.load(Ordering::Relaxed);
    assert!(heap <= DOCUMENT_ARCHIVE_INGRESS_PAGE_BYTES, "largest allocation {heap}");
    let mut controlled = semio_framework_value::retirement::controlled::ControlledRetirement::retain(pages);
    let mut remaining = 40_000;
    while !controlled.terminal_is_empty() {
        assert!(remaining > 0, "{:?}", controlled.next_demand(GRANT.maximum_copy_bytes));
        remaining -= 1;
        let turn = wallet.run_settled(GRANT, WORK, |effect| controlled.step(GRANT.maximum_copy_bytes, effect)).unwrap().unwrap();
        let progress = turn.0.result.unwrap().progress();
        wallet.post(progress).unwrap();
    }
    (ready, heap)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args[1] == "prepare" {
        use pack::snapshot::ArtifactPack;
        let image_pdf = std::fs::read(&args[2]).unwrap();
        let typed = semio_framework_value::DslValue::Bytes(image_pdf);
        std::fs::write(&args[3], typed.encode_pack()).unwrap();
        let history = semio_os_spr::HistoryLog {
            doc_id: "high-entropy-reopen-fixture".into(), schema: "stdio.pdf.1.7".into(), ..Default::default()
        };
        let options = semio_os_spr::EncodeOptions::default();
        let mut encoding = std::pin::pin!(semio_os_spr::encode_history(&history, &options));
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        let spr = loop {
            if let std::task::Poll::Ready(spr) = std::future::Future::poll(encoding.as_mut(), &mut cx) {
                break spr.unwrap();
            }
        };
        std::fs::write(&args[4], spr).unwrap();
        return;
    }
    let n = |i: usize| args[i].parse::<usize>().unwrap();
    let mut digest = [0; 32];
    for (index, byte) in digest.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&args[6][2 * index..2 * index + 2], 16).unwrap();
    }
    let limits = DocumentArchiveIngressLimits { archive_bytes: n(3), pack_bytes: n(4), spr_bytes: n(5) };
    let (ready, heap) = run(Path::new(&args[1]), n(2), limits, digest, &args[7], Path::new(&args[8]));
    println!("{} {heap}", if ready { "ready" } else if args[7] == "cancel" { "cancelled" } else { "refused" });
}
