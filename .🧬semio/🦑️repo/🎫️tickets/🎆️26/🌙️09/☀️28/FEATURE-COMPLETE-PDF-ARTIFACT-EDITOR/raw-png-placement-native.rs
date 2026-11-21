//! 🧪 Checks paid projection and the late move of the canonical PNG Pack.
#[path = "paged-png-placement-identity.rs"]
mod plane_identity;
#[path = "paged-png-placement-pack.rs"]
mod plane_pack;
#[path = "raw-png-placement-samples.rs"]
mod samples;
#[path = "raw-png-placement-pack.rs"]
mod native;

use native::{OriginalPack, RawPngPlacementPack};
use plane_identity::{PlaneRole, PngPlanes};
use plane_pack::{PagedPngPlacementPayload, PngPlacementArchive};
use samples::{RawPngPlacementSamples, RgbaLoan};
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant, paged::ByteReadSource};
use std::io::Write;
use std::{future::Future, path::Path, task::{Context, Poll, Waker}};

const REVISION: [u8; 32] = [0x35; 32];
const CAP: usize = 32768;
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16, maximum_copy_bytes: 32768, maximum_capacity_bytes: 32768,
    maximum_release_bytes: 65536, maximum_depth: 32,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 20, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 22,
    maximum_release_bytes: 1 << 22, ..TURN
};

fn block<F: Future>(run: F) -> F::Output {
    let mut run = std::pin::pin!(run);
    match run.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("local archive test unexpectedly yielded"),
    }
}

fn sample(name: &str) -> Vec<u8> {
    std::fs::read(Path::new(file!()).parent().unwrap().join("🗑️generated/raw-png").join(name)).unwrap()
}

fn close(wallet: &mut AuthoringWallet, owner: &mut OriginalPack<CAP>, retained: usize) {
    let mut totals = [retained, 0];
    for _ in 0..1024 {
        if owner.terminal_is_empty() { break; }
        let grant = TURN.restricted_by(wallet.remaining());
        let (turn, _) = wallet.run_settled(grant, 65536, |effect| owner.step(grant.maximum_copy_bytes, effect)).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        totals[0] += receipt.retained_capacity_bytes;
        totals[1] += receipt.released_bytes;
        wallet.post(receipt).unwrap();
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(totals[0], totals[1]);
    println!("[DEBUG] paid Pack backing retired {} bytes", totals[1]);
}

#[test]
fn bounded_projection_and_paged_pack_match_the_canonical_writer() {
    let mut png = sample("rgba.native");
    let mut record = REVISION.to_vec();
    record.extend_from_slice(b"typed placement control");
    for alpha in [true, false] {
        let raster = RgbaLoan { width: 32, height: 32, pixels: &png };
        let mut wallet = AuthoringWallet::new(STOCK, 1 << 25);
        let denied = RetainedCloneGrant { maximum_copy_bytes: RawPngPlacementSamples::birth_demand().copy_bytes - 1, ..TURN };
        let mut project = None;
        let denied = wallet.run(denied, 65536, |effect| RawPngPlacementSamples::admit(&mut project, raster, REVISION, 4096, effect)).unwrap().unwrap();
        assert_eq!(denied.performed, 0);
        assert!(project.is_none());
        let birth = wallet.run(TURN, 65536, |effect| RawPngPlacementSamples::admit(&mut project, raster, REVISION, 4096, effect)).unwrap().unwrap();
        wallet.post(birth.output.unwrap()).unwrap();
        let mut project = project.unwrap();
        assert!(project.sealed(raster, &REVISION, false).unwrap().is_none());
        let (facts, projection) = (0..1024).find_map(|_| {
            let grant = RetainedCloneGrant { maximum_copy_bytes: 257, ..TURN }.restricted_by(wallet.remaining());
            let turn = wallet.run(grant, 65536, |effect| project.advance(raster, &REVISION, false, effect)).unwrap().unwrap();
            let receipt = turn.output.unwrap();
            assert!(receipt.copied_bytes <= 257);
            assert_eq!(receipt.retained_capacity_bytes, 0);
            wallet.post(receipt).unwrap();
            project.sealed(raster, &REVISION, false).unwrap()
        }).unwrap();
        let planes = projection.planes();
        assert_eq!(facts.shape.has_alpha(), alpha);
        assert_eq!(planes.loan(PlaneRole::Alpha).byte_len(), if alpha { 1024 } else { 0 });
        let mut pack = None;
        let birth = wallet.run(TURN, 65536, |effect| RawPngPlacementPack::<CAP>::admit(&mut pack, facts, &record, REVISION, effect)).unwrap().unwrap();
        wallet.post(birth.output.unwrap()).unwrap();
        let mut pack = pack.unwrap();
        let mut published = None;
        let refused = wallet.run(TURN, 65536, |effect| pack.take(&REVISION, false, &mut published, effect)).unwrap().unwrap();
        assert_eq!(refused.performed, 0);
        let mut retained = 0;
        for _ in 0..1024 {
            if pack.is_ready() { break; }
            let grant = TURN.restricted_by(wallet.remaining());
            let turn = wallet.run(grant, 65536, |effect| pack.advance(planes, &record, &REVISION, false, effect)).unwrap().unwrap();
            let receipt = turn.output.unwrap();
            retained += receipt.retained_capacity_bytes;
            wallet.post(receipt).unwrap();
        }
        assert!(pack.is_ready());
        let ready = wallet.run(TURN, 65536, |effect| pack.take(&REVISION, false, &mut published, effect)).unwrap().unwrap();
        wallet.post(ready.output.unwrap()).unwrap();
        let mut published = published.unwrap();
        assert!(pack.terminal_is_empty());
        let existing = block(plane_pack::seal(Vec::new(), planes, facts, &record, async |_| Ok(()))).unwrap();
        let source = published.original().unwrap();
        assert!(source.iter().eq(existing.iter().copied()));
        let output = if alpha { "archive-alpha.pack" } else { "archive-opaque.pack" };
        let path = Path::new(file!()).parent().unwrap().join("🗑️generated/raw-png").join(output);
        let mut file = std::fs::File::create(path).unwrap();
        let mut page = [0; 4096];
        for offset in (0..source.len()).step_by(page.len()) {
            let count = source.read_bytes(offset, &mut page);
            file.write_all(&page[..count]).unwrap();
        }
        let reference = facts.reference(PlaneRole::Color).unwrap();
        let reopened = block(PngPlacementArchive::open(PagedPngPlacementPayload(source), &reference)).unwrap();
        assert_eq!(reopened.facts(), facts);
        assert_eq!(block(reopened.operation()).unwrap(), record);
        let mut read = reopened.planes();
        let mut bytes = [0; 512];
        let mut count = 0;
        while !read.verified() {
            if let Some((_, size)) = block(read.step(&mut bytes)).unwrap() { count += size; }
        }
        assert_eq!(count, if alpha { 4096 } else { 3072 });
        drop(reopened);
        close(&mut wallet, &mut published, retained);
        for pixel in png.chunks_exact_mut(4) { pixel[3] = 255; }
    }
}

#[test]
fn revoke_partial_placement_before_any_original_can_publish() {
    let pixels = sample("rgba.native");
    let raster = RgbaLoan { width: 32, height: 32, pixels: &pixels };
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 25);
    let mut project = None;
    let birth = wallet.run(TURN, 65536, |effect| RawPngPlacementSamples::admit(&mut project, raster, REVISION, 4096, effect)).unwrap().unwrap();
    wallet.post(birth.output.unwrap()).unwrap();
    let mut project = project.unwrap();
    let (facts, projection) = (0..16).find_map(|_| {
        let turn = wallet.run(TURN, 65536, |effect| project.advance(raster, &REVISION, false, effect)).unwrap().unwrap();
        wallet.post(turn.output.unwrap()).unwrap();
        project.sealed(raster, &REVISION, false).unwrap()
    }).unwrap();
    let wrong = pixels.clone();
    let foreign = RgbaLoan { pixels: &wrong, ..raster };
    assert!(project.sealed(foreign, &REVISION, false).is_err());
    let mut changed = REVISION;
    changed[31] ^= 1;
    let mut record = REVISION.to_vec();
    record.push(1);
    let planes: PngPlanes<'_> = projection.planes();
    for canceled in [false, true] {
        let mut partial = None;
        let birth = wallet.run(TURN, 65536, |effect| RawPngPlacementPack::<CAP>::admit(&mut partial, facts, &record, REVISION, effect)).unwrap().unwrap();
        wallet.post(birth.output.unwrap()).unwrap();
        let mut partial = partial.unwrap();
        let mut totals = [0, 0];
        for _ in 0..7 {
            let turn = wallet.run(TURN, 65536, |effect| partial.advance(planes, &record, &REVISION, false, effect)).unwrap().unwrap();
            let receipt = turn.output.unwrap();
            totals[0] += receipt.retained_capacity_bytes;
            wallet.post(receipt).unwrap();
        }
        assert!(!partial.is_ready() && totals[0] > 0);
        let mut published = None;
        let live = if canceled { &REVISION } else { &changed };
        let stale = wallet.run(TURN, 65536, |effect| partial.take(live, canceled, &mut published, effect)).unwrap().unwrap();
        assert!(stale.output.is_err());
        assert!(published.is_none());
        let fresh = wallet.run(TURN, 65536, |effect| partial.take(&REVISION, false, &mut published, effect)).unwrap().unwrap();
        assert!(fresh.output.is_err());
        for _ in 0..1024 {
            if partial.terminal_is_empty() { break; }
            let grant = TURN.restricted_by(wallet.remaining());
            let (turn, _) = wallet.run_settled(grant, 65536, |effect| partial.retire(grant.maximum_copy_bytes, effect).unwrap()).unwrap().unwrap();
            let receipt = turn.result.unwrap().progress();
            totals[0] += receipt.retained_capacity_bytes;
            totals[1] += receipt.released_bytes;
            wallet.post(receipt).unwrap();
        }
        assert!(partial.terminal_is_empty());
        assert_eq!(totals[0], totals[1]);
    }
}
