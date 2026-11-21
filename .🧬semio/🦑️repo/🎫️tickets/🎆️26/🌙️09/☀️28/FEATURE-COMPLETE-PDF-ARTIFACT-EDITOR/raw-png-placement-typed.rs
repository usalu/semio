//! 📚 Verifies bounded paid PNG Pack placement against the real PDF and history codecs.
#[path = "paged-png-placement-identity.rs"] mod plane_identity;
#[path = "paged-png-placement-pack.rs"] mod plane_pack;
#[path = "paged-png-placement-rows.rs"] mod plane_rows;
#[path = "raw-png-placement-samples.rs"] mod samples;
#[path = "raw-png-placement-pack.rs"] mod raw_pack;
#[path = "raw-png-placement-product.rs"] mod product;

use plane_identity::PlaneRole;
use plane_pack::PagedPngPlacementPayload;
use plane_rows::PngPlacement;
use product::{PDF_RAW_PNG_ARCHIVE_BYTES, PdfRawPngRows, prepare_raw_png_rows};
use raw_pack::RawPngPlacementPack;
use samples::{RawPngPlacementSamples, RgbaLoan};
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
use semio_os_spr::{ColdMutation, OpBinary, history::{DecodeOptions, EncodeOptions, HistoryEdit, HistoryEdits, HistoryLog, OpPayload, decode_history, encode_history}};
use semio_os_vcs::apply_mutation;
use semio_s_artifact_stdio_pdf_editor_document::{ImagePlacement, check_image_placement};
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
    mutations::PdfMutation, snapshot::{PdfImage, PdfImageBody, PdfOp, PdfPage, PdfSnapshot},
};
use std::{future::Future, task::{Context, Poll, Waker}};

const REVISION: [u8; 32] = [0x35; 32];
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 65536,
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
        Poll::Pending => panic!("local typed placement test unexpectedly yielded"),
    }
}

fn history(before: &PdfSnapshot, applied: &PdfSnapshot, forwards: &[&PdfMutation], inverse: &[Vec<PdfMutation>]) {
    let payload = |op: &PdfMutation| OpPayload { text: None, binary: Some(op.encode_op().unwrap()) };
    let edit = HistoryEdit {
        id: "raw-png-placement".into(), actor: None, started_at: "2026-11-21T00:00:00Z".into(),
        finished_at: None, verb: None, line: None, ops: forwards.iter().copied().map(payload).collect(),
        inverse: inverse.iter().rev().flatten().map(payload).collect(), meta: None, lane: None,
    };
    let log = HistoryLog { doc_id: "pdf-raw-png-placement-law".into(), schema: before.schema.clone(),
        edits: HistoryEdits::try_from_iter([edit]).ok().unwrap(), ..Default::default() };
    let wire = block(encode_history(&log, &EncodeOptions { write_backwards_section: true, ..Default::default() })).unwrap();
    let reopened = block(decode_history(&wire, &DecodeOptions::default())).unwrap();
    let edit = reopened.edits.iter().next().unwrap();
    let mut current = before.clone();
    for (rows, expected) in [(&edit.ops, applied), (&edit.inverse, before), (&edit.ops, applied)] {
        for row in rows {
            let op = PdfMutation::decode_op(row.binary.as_deref().unwrap()).unwrap();
            current = apply_mutation(&current, &op).unwrap().0;
        }
        assert_eq!(&current, expected);
    }
}

fn law(alpha: bool, replace: bool) {
    let mut before = PdfSnapshot::default();
    before.pages.push(PdfPage::new(128., 128.));
    let placement = if replace {
        before.images.push(PdfImage::rgb8("Existing", 1, 1, vec![0, 0, 0]));
        before.pages[0].content.push(PdfOp::PaintXObject { name: "Existing".into() });
        ImagePlacement::Replace { page: 0, operator: 0 }
    } else {
        ImagePlacement::Insert { page: 0, matrix: [32., 0., 0., 32., 4., 6.] }
    };
    let mut pixels = *include_bytes!("🗑️generated/raw-png/rgba.native");
    if !alpha { for pixel in pixels.chunks_exact_mut(4) { pixel[3] = 255; } }
    let raster = RgbaLoan { width: 32, height: 32, pixels: &pixels };
    let mut wallet = AuthoringWallet::new(STOCK, 1 << 25);
    let mut samples = None;
    let birth = wallet.run(TURN, 65536, |effect| RawPngPlacementSamples::admit(&mut samples, raster, REVISION, 4096, effect)).unwrap().unwrap();
    wallet.post(birth.output.unwrap()).unwrap();
    let mut samples = samples.unwrap();
    let (facts, projection) = (0..16).find_map(|_| {
        let turn = wallet.run(TURN, 65536, |effect| samples.advance(raster, &REVISION, false, effect)).unwrap().unwrap();
        wallet.post(turn.output.unwrap()).unwrap();
        samples.sealed(raster, &REVISION, false).unwrap()
    }).unwrap();
    let rows = prepare_raw_png_rows(&before, placement, REVISION, facts, 13, 21, 8).unwrap();
    let mut rows = PdfRawPngRows::new(rows).unwrap();
    let record = rows.record().unwrap();
    let mut prepared = None;
    let birth = wallet.run(TURN, 65536, |effect| RawPngPlacementPack::<PDF_RAW_PNG_ARCHIVE_BYTES>::admit(&mut prepared, facts, record, REVISION, effect)).unwrap().unwrap();
    wallet.post(birth.output.unwrap()).unwrap();
    let mut prepared = prepared.unwrap();
    for _ in 0..128 {
        if prepared.is_ready() { break; }
        let turn = wallet.run(TURN, 65536, |effect| prepared.advance(projection.planes(), record, &REVISION, false, effect)).unwrap().unwrap();
        wallet.post(turn.output.unwrap()).unwrap();
    }
    assert!(prepared.is_ready());
    let mut archive = None;
    let take = wallet.run(TURN, 65536, |effect| prepared.take(&REVISION, false, &mut archive, effect)).unwrap().unwrap();
    wallet.post(take.output.unwrap()).unwrap();
    let mut archive = archive.unwrap();
    let bytes = archive.original().unwrap();
    let reference = facts.reference(PlaneRole::Color).unwrap();
    let received = block(PngPlacement::open(PagedPngPlacementPayload(bytes), &reference)).unwrap();
    let mut proof = received.planes();
    assert!(received.authorize_insertion(REVISION, &proof).is_err());
    let mut scratch = [0; 4096];
    while !proof.verified() { block(proof.step(&mut scratch)).unwrap(); }
    let mut changed = REVISION;
    changed[31] ^= 1;
    assert!(received.authorize_insertion(changed, &proof).is_err());
    let operations: Vec<_> = received.authorize_insertion(REVISION, &proof).unwrap().collect();
    assert_eq!(operations.len(), 2 + usize::from(alpha));
    let mut after = before.clone();
    let mut inverse = Vec::new();
    for row in &operations {
        inverse.push(ColdMutation::inverse(*row, &after).unwrap());
        after = apply_mutation(&after, *row).unwrap().0;
    }
    assert_eq!(after.images.len(), before.images.len() + 1 + usize::from(alpha));
    let color = after.images.iter().find(|image| matches!(&image.body, PdfImageBody::Artifact { reference: key } if *key == reference)).unwrap();
    let mask = color.soft_mask.as_deref().and_then(|id| after.image(id));
    assert_eq!(mask.is_some(), alpha);
    assert!(received.authorize_save(color, mask, &proof).is_ok());
    assert!(after.objects.is_empty());
    history(&before, &after, &operations, &inverse);
    drop(received);
    while !archive.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| archive.step(TURN.maximum_copy_bytes, effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    while !rows.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(TURN, 65536, |effect| rows.retire(TURN.maximum_copy_bytes, effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    println!("[DEBUG] alpha={alpha} replace={replace}: paid Pack, typed SetImage/paint, and persisted apply/undo/redo agree");
}

fn main() {
    for alpha in [false, true] {
        for replace in [false, true] { law(alpha, replace); }
    }
}
