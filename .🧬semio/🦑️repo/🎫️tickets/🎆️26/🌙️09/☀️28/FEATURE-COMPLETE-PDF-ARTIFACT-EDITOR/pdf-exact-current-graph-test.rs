//! 🔎️ Native integration law for stored COS plus actual typed edits, mounted with the staged PDF exports.
use crate::standards::v1_7::subsets::base::{
    io::import::decode_pdf,
    modules::{lexer::CosGrant, lift::ControlledLiftEvent},
    schema::{
        mutations::{PdfMutation, ReplaceContent, SetInfo, SetPageRotation},
        snapshot::{PdfOp, PdfSnapshot, PdfTextString},
    },
};
use semio_s_artifact_stdio_pdf_native_save::{
    exact_graph_proof::{projection_trailer, ExactGraphProof, ExactGraphTurn},
    graph::PinnedPdfGraph,
    ordinary_graft::{OrdinaryGraft, GRAFT_WORK},
};

const REVISION: [u8; 32] = [42; 32];

fn proves(plan: &PinnedPdfGraph, snapshot: &PdfSnapshot) -> bool {
    let loan = plan.loan(snapshot, REVISION).unwrap();
    let grant = CosGrant { work: 4096, allocation_bytes: 16_384, release_bytes: 16_384 };
    let (trailer, _) = projection_trailer(loan, grant).unwrap().unwrap();
    let mut proof = ExactGraphProof::new();
    let matching = (0..16384).find_map(|_| match proof.advance(loan, &trailer, snapshot, grant, false) {
        Ok(ExactGraphTurn::Compared { ready: true, .. }) => Some(true),
        Ok(ExactGraphTurn::Compared { .. }) => None,
        Ok(ExactGraphTurn::Lift(turn)) if matches!(turn.event, Ok(ControlledLiftEvent::Complete | ControlledLiftEvent::More)) => None,
        _ => Some(false),
    }).expect("controlled candidate proof reached terminal");
    proof.begin_close();
    assert!((0..16384).any(|_| proof.close_step(grant).event == Ok(ControlledLiftEvent::Closed)));
    assert!(proof.take_partial().page.is_none());
    matching
}

#[test]
fn controlled_existing_graph_refuses_stale_cos_and_accepts_exact_page_info_and_text_graft() {
    let fixture = std::env::var_os("SEMIO_PDF_EXACT_GRAPH_FIXTURE").expect("native graph fixture");
    let before = decode_pdf(&std::fs::read(fixture).unwrap()).unwrap();
    let mut info = before.info.clone();
    info.title = Some("Current revision".into());
    let at = before.pages[0].content.iter().position(|op| matches!(op, PdfOp::ShowText { .. })).unwrap();
    let mut changed = before.clone();
    for mutation in [
        PdfMutation::SetInfo(SetInfo { info }),
        PdfMutation::SetPageRotation(SetPageRotation { index: 0, rotation: 90 }),
        PdfMutation::ReplaceContent(ReplaceContent { index: 0, at, op: PdfOp::ShowText { text: PdfTextString::text("Current edited text") } }),
    ] {
        changed = semio_os_vcs::apply_mutation(&changed, &mutation).unwrap().0;
    }
    let stale = PinnedPdfGraph::retained_candidate(&changed, REVISION).unwrap();
    assert!(!proves(&stale, &changed));
    let exact = PinnedPdfGraph::prepare_reference(&changed, REVISION).unwrap();
    assert!(proves(&exact, &changed));
    assert!(exact.loan(&changed, [19; 32]).is_err());
    assert!(exact.loan(&before, REVISION).is_err());
    assert!(exact.loan(&changed, REVISION).unwrap().trailer.extra.iter().any(|entry| entry.key == "Private"));
}

#[test]
fn paid_rotation_and_info_graft_stays_on_the_exact_retained_graph() {
    let fixture = std::env::var_os("SEMIO_PDF_EXACT_GRAPH_FIXTURE").expect("native graph fixture");
    let before = decode_pdf(&std::fs::read(fixture).unwrap()).unwrap();
    let mut info = before.info.clone();
    info.title = Some("Paid updated title".into());
    let mut target = semio_os_vcs::apply_mutation(&before, &PdfMutation::SetInfo(SetInfo { info })).unwrap().0;
    target = semio_os_vcs::apply_mutation(&target, &PdfMutation::SetPageRotation(SetPageRotation { index: 0, rotation: 90 })).unwrap().0;
    let mut paid = PinnedPdfGraph::retained_candidate(&target, REVISION).unwrap();
    let raw = paid.loan(&target, REVISION).unwrap();
    let foreign = target.objects.iter().find(|object| object.id.num == 17).unwrap();
    let mut graft = OrdinaryGraft::default();
    let grant = CosGrant { work: GRAFT_WORK, allocation_bytes: 8192, release_bytes: 8192 };
    assert!((0..256).any(|_| graft.step(raw, &target, grant, 8192).unwrap().ready));
    paid.install_ordinary_graft(graft.into_rows().ok().unwrap());
    assert!(proves(&paid, &target));
    let loan = paid.loan(&target, REVISION).unwrap();
    let native = PinnedPdfGraph::prepare_reference(&target, REVISION).unwrap();
    let reference = native.loan(&target, REVISION).unwrap();
    assert_eq!(loan.trailer, reference.trailer);
    assert_eq!(loan.len(), reference.len());
    assert!((0..loan.len()).all(|index| loan.object(index) == reference.object(index)));
    assert!(std::ptr::eq(loan.lookup(foreign.id).unwrap(), foreign));
}
