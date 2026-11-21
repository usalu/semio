//! 🔎 Actual retained graph qualification and segmented writer law for visible PDF programs.

pub use semio_s_artifact_stdio_pdf_native_save::{classic_graph_quote, standards};
#[cfg(candidate)]
#[path = "proof.rs"]
mod exact_graph_proof;
#[cfg(not(candidate))]
use semio_s_artifact_stdio_pdf_native_save::exact_graph_proof;
use exact_graph_proof::{ExactGraphProof, ExactGraphTurn, projection_trailer};
use semio_s_artifact_stdio_pdf_native_save::{classic_graph_writer::ClassicGraphWriter, graph::PinnedPdfGraph};
use standards::v1_7::subsets::base::{
    io::import::decode_pdf,
    modules::{lexer::CosGrant, lift::ControlledLiftEvent},
    schema::snapshot::{PdfImageBody, PdfOp, PdfSnapshot},
};
use std::{fs, path::PathBuf};

const REVISION: [u8; 32] = [42; 32];
const TURN: CosGrant = CosGrant { work: 4096, allocation_bytes: 32_768, release_bytes: 32_768 };

struct BoundedSink<'a> { output: &'a mut Vec<u8>, maximum: usize }

impl std::io::Write for BoundedSink<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let size = bytes.len().min(self.maximum);
        self.output.extend_from_slice(&bytes[..size]);
        Ok(size)
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn proves(snapshot: &PdfSnapshot) -> bool {
    let plan = PinnedPdfGraph::retained_candidate(snapshot, REVISION).unwrap();
    let loan = plan.loan(snapshot, REVISION).unwrap();
    let (trailer, _) = projection_trailer(loan, TURN).unwrap().unwrap();
    let mut proof = ExactGraphProof::new();
    let accepted = (0..24_000).find_map(|_| match proof.advance(loan, &trailer, snapshot, TURN, false) {
        Ok(ExactGraphTurn::Compared { ready: true, .. }) => Some(true),
        Ok(ExactGraphTurn::Compared { .. }) => None,
        Ok(ExactGraphTurn::Lift(turn)) if matches!(turn.event, Ok(ControlledLiftEvent::Complete | ControlledLiftEvent::More)) => None,
        other => { eprintln!("[DEBUG] Visible candidate qualification ended: {other:?}"); Some(false) }
    }).expect("visible proof is bounded");
    proof.begin_close();
    assert!((0..24_000).any(|_| proof.close_step(TURN).event == Ok(ControlledLiftEvent::Closed)));
    assert!(proof.take_partial().page.is_none());
    accepted
}

#[cfg_attr(not(target_arch = "wasm32"),test)]
fn unchanged_visible_images_and_vector_program_use_retained_save() {
    let work = PathBuf::from(std::env::var_os("SEMIO_PDF_VISIBLE_FIXTURE").unwrap());
    let neutral: serde_json::Value = serde_json::from_slice(&fs::read(work.join("law.json")).unwrap()).unwrap();
    let snapshot = decode_pdf(&fs::read(work.join("in.pdf")).unwrap()).unwrap();
    assert_eq!(snapshot.images.len(), 2);
    assert!(snapshot.pages.iter().all(|page| page.content.iter().any(|op| matches!(op, PdfOp::PaintXObject { .. }))));
    assert!(snapshot.pages.iter().all(|page| page.content.iter().any(|op| matches!(op, PdfOp::Stroke))));
    assert!(proves(&snapshot), "retained qualified visible graph");

    let mut altered = snapshot.clone();
    let image = altered.images.iter_mut().find(|image| image.id.as_str() == neutral["sampled"]["name"].as_str().unwrap()).unwrap();
    let PdfImageBody::Samples { values } = &mut image.body else { panic!("sampled image body") };
    values[neutral["sampled"]["sampleProbeIndex"].as_u64().unwrap() as usize] ^= 1;
    assert!(!proves(&altered), "sample mismatch across a paid comparison turn refuses stale COS");

    let mut edited = snapshot.clone();
    let image = edited.images.iter_mut().find(|image| image.id.as_str() == neutral["coded"]["name"].as_str().unwrap()).unwrap();
    let PdfImageBody::Artifact { reference } = &mut image.body else { panic!("coded image body") };
    reference.artifact_id = format!("{}x", reference.artifact_id);
    assert!(!proves(&edited), "exact coded artifact identity required");

    let graph = PinnedPdfGraph::retained_candidate(&snapshot, REVISION).unwrap();
    let loan = graph.loan(&snapshot, REVISION).unwrap();
    let mut quote = classic_graph_quote::GraphQuote::start(loan, 32 * 1024 * 1024).unwrap();
    assert!((0..256).any(|_| quote.next(loan, classic_graph_quote::QUOTE_WORK).unwrap()));
    let mut writer = ClassicGraphWriter::start(&quote, Vec::with_capacity(classic_graph_quote::MAX_PREFIX)).unwrap();
    let mut output = Vec::new();
    let page = neutral["writerPage"].as_u64().unwrap() as usize;
    for _ in 0..12_000 {
        let mut sink = BoundedSink { output: &mut output, maximum: page };
        let step = writer.step(loan, &mut sink, classic_graph_quote::MAX_PREFIX).unwrap();
        assert!(step.emitted <= page);
        if step.done {
            save_output(&work, &output);
            return;
        }
    }
    panic!("visible graph writer did not complete");
}

#[cfg(target_arch = "wasm32")]
fn main() { unchanged_visible_images_and_vector_program_use_retained_save(); }

#[path = "law-output.rs"]
mod law_output;
use law_output::save_output;
