//! 📐 Verifies ordinary page geometry uses the retained classic Save writer and its exact pinned graph.

use semio_s_artifact_stdio_pdf_native_save::{classic_graph_quote, classic_graph_writer::ClassicGraphWriter, graph::PinnedPdfGraph, standards};
use semio_s_artifact_stdio_pdf_native_save::exact_graph_proof::{ExactGraphProof, ExactGraphTurn, projection_trailer};
#[cfg(candidate)]
#[path = "box-graft.rs"]
mod ordinary_graft;
#[cfg(not(candidate))]
use semio_s_artifact_stdio_pdf_native_save::ordinary_graft;
use ordinary_graft::OrdinaryGraft;
use standards::v1_7::subsets::base::{
    io::import::decode_pdf, modules::{lexer::CosGrant, lift::ControlledLiftEvent},
    schema::snapshot::{PdfPage, PdfSnapshot},
};
use std::{fs, path::PathBuf};

#[cfg(candidate)]
impl ordinary_graft::OrdinaryRows {
    fn into_product_rows(self) -> semio_s_artifact_stdio_pdf_native_save::ordinary_graft::OrdinaryRows {
        semio_s_artifact_stdio_pdf_native_save::ordinary_graft::OrdinaryRows {
            page: self.page, info: self.info, remove: self.remove, info_ref: self.info_ref,
        }
    }
}

const REVISION: [u8; 32] = [51; 32];
const GRANT: CosGrant = CosGrant { work: 4096, allocation_bytes: 32768, release_bytes: 32768 };

fn edit(page: &mut PdfPage, law: &serde_json::Value) {
    let rect = |key| serde_json::from_value(law[key].clone()).unwrap();
    page.media_box = rect("mediaBox");
    page.crop_box = Some(rect("cropBox"));
    page.trim_box = Some(rect("trimBox"));
    page.bleed_box = Some(rect("bleedBox"));
    page.art_box = Some(rect("artBox"));
    page.user_unit = law["userUnit"].as_f64();
    page.rotate = law["rotate"].as_i64().unwrap() as i32;
}

fn qualify(plan: &PinnedPdfGraph, current: &PdfSnapshot) -> bool {
    let graph = plan.loan(current, REVISION).unwrap();
    let (trailer, _) = projection_trailer(graph, GRANT).unwrap().unwrap();
    let mut proof = ExactGraphProof::new();
    let decision = (0..24000).find_map(|_| match proof.advance(graph, &trailer, current, GRANT, false) {
        Ok(ExactGraphTurn::Compared { ready: true, .. }) => Some(true),
        Ok(ExactGraphTurn::Compared { .. }) => None,
        Ok(ExactGraphTurn::Lift(step)) if matches!(step.event, Ok(ControlledLiftEvent::Complete | ControlledLiftEvent::More)) => None,
        _ => Some(false),
    }).expect("qualification is bounded");
    proof.begin_close();
    assert!((0..24000).any(|_| proof.close_step(GRANT).event == Ok(ControlledLiftEvent::Closed)));
    assert!(proof.take_partial().page.is_none());
    decision
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
fn changed_page_geometry_uses_retained_writer() {
    let work = PathBuf::from(std::env::var_os("SEMIO_PDF_BOX_FIXTURE").unwrap());
    let law: serde_json::Value = serde_json::from_slice(&fs::read(work.join("law.json")).unwrap()).unwrap();
    let input = fs::read(work.join("in.pdf")).unwrap();
    let mut current = decode_pdf(&input).unwrap();
    edit(&mut current.pages[law["page"].as_u64().unwrap() as usize], &law);
    let mut plan = PinnedPdfGraph::retained_candidate(&current, REVISION).unwrap();
    let mut graft = OrdinaryGraft::default();
    let graph = plan.loan(&current, REVISION).unwrap();
    for _ in 0..32 {
        let denied = graft.step(graph, &current, CosGrant { allocation_bytes: 0, ..GRANT }, 0).unwrap();
        assert_eq!((denied.allocated, denied.copied_bytes), (0, 0));
        if denied.needs.is_some() {
            let paid = graft.step(graph, &current, GRANT, GRANT.allocation_bytes).unwrap();
            assert!(paid.allocated > 0 && paid.allocated <= denied.needs.unwrap());
            break;
        }
    }
    assert!((0..32).any(|_| graft.step(graph, &current, GRANT, GRANT.allocation_bytes).unwrap().ready));
    let rows = graft.into_rows().ok().unwrap();
    #[cfg(candidate)]
    let rows = rows.into_product_rows();
    plan.install_ordinary_graft(rows);
    assert!(qualify(&plan, &current), "retained page geometry qualification");
    let graph = plan.loan(&current, REVISION).unwrap();
    let mut quote = classic_graph_quote::GraphQuote::start(graph, 32 * 1024 * 1024).unwrap();
    assert!((0..256).any(|_| quote.next(graph, classic_graph_quote::QUOTE_WORK).unwrap()));
    let mut writer = ClassicGraphWriter::start(&quote, Vec::with_capacity(classic_graph_quote::MAX_PREFIX)).unwrap();
    let mut bytes = Vec::new();
    assert!((0..4000).any(|_| writer.step(graph, &mut bytes, classic_graph_quote::MAX_PREFIX).unwrap().done));
    save_output(&work, &bytes);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
fn refuses_a_second_changed_page_and_a_foreign_revision() {
    let work = PathBuf::from(std::env::var_os("SEMIO_PDF_BOX_FIXTURE").unwrap());
    let mut current = decode_pdf(&fs::read(work.join("in.pdf")).unwrap()).unwrap();
    current.pages[0].rotate = 90;
    current.pages[1].rotate = 180;
    let plan = PinnedPdfGraph::retained_candidate(&current, REVISION).unwrap();
    let foreign = plan.loan(&current, [52; 32]);
    assert!(matches!(foreign, Err(classic_graph_quote::GraphRefusal::SourceChanged)));
    let graph = plan.loan(&current, REVISION).unwrap();
    let mut graft = OrdinaryGraft::default();
    assert!((0..8).any(|_| graft.step(graph, &current, GRANT, GRANT.allocation_bytes).is_err()));
}

#[cfg(candidate)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn refuses_duplicate_edited_dictionary_entries_before_allocation() {
    use standards::v1_7::subsets::base::schema::snapshot::PdfObject;
    let work = PathBuf::from(std::env::var_os("SEMIO_PDF_BOX_FIXTURE").unwrap());
    let mut current = decode_pdf(&fs::read(work.join("in.pdf")).unwrap()).unwrap();
    let dictionary = current.objects.iter_mut().find_map(|row| match &mut row.value {
        PdfObject::Dict(entries) if entries.iter().any(|entry| entry.key == "MediaBox") => Some(entries),
        _ => None,
    }).unwrap();
    dictionary.push(dictionary.iter().find(|entry| entry.key == "MediaBox").unwrap().clone());
    for page in &mut current.pages { page.media_box[2] += 10.0; }
    let plan = PinnedPdfGraph::retained_candidate(&current, REVISION).unwrap();
    let loan = plan.loan(&current, REVISION).unwrap();
    let mut graft = OrdinaryGraft::default();
    assert!(graft.step(loan, &current, GRANT, GRANT.allocation_bytes).is_ok());
    assert!(matches!(graft.step(loan, &current, GRANT, GRANT.allocation_bytes), Err(ordinary_graft::GraftRefusal::Geometry)));
    assert!(graft.take_partial().page.is_none());
}

#[cfg(target_arch = "wasm32")]
fn main() {
    refuses_a_second_changed_page_and_a_foreign_revision();
    #[cfg(candidate)]
    refuses_duplicate_edited_dictionary_entries_before_allocation();
    changed_page_geometry_uses_retained_writer();
}

#[path = "law-output.rs"]
mod law_output;
use law_output::save_output;
