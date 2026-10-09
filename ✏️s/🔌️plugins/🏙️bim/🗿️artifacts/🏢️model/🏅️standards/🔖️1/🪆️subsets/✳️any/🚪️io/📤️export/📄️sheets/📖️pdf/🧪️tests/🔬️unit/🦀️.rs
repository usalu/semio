//! 🧪️ The PDF of the sheet set read back by the PDF artifact: one page per sheet at the size of its paper in points, the clip of every viewport, the text of the title block and the captions, a deterministic file.

use super::super::testkit::{house_with_sheets, inferred, labels};
use super::*;
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::decode_pdf;

fn written() -> (Vec<u8>, crate::ModelSnapshot, crate::ModelInference) {
    let model = house_with_sheets();
    let inferred = inferred(&model);
    (super::super::sheets_pdf(&model, &inferred, None, &labels()).expect("the set writes"), model, inferred)
}

fn points(millimetres: f64) -> f64 {
    millimetres * 72.0 / 25.4
}

#[test]
fn the_set_is_one_pdf_with_one_page_per_sheet_at_the_size_of_its_paper() {
    let (bytes, _, _) = written();
    assert!(bytes.starts_with(b"%PDF-1.7"));
    let document = decode_pdf(&bytes).expect("the file reads back");
    let sizes: Vec<(f64, f64)> = document.pages.iter().map(|page| (page.width(), page.height())).collect();
    assert_eq!(sizes.len(), 3);
    let expected = [(420.0, 297.0), (297.0, 420.0), (594.0, 420.0)];
    for ((width, height), (paper_width, paper_height)) in sizes.into_iter().zip(expected) {
        assert!((width - points(paper_width)).abs() < 1e-6 && (height - points(paper_height)).abs() < 1e-6, "{width} x {height}");
    }
}

#[test]
fn every_page_prints_the_title_block_the_captions_and_the_revision_table_as_text() {
    let (bytes, _, _) = written();
    let document = decode_pdf(&bytes).expect("the file reads back");
    let plans = document.pages[0].text();
    for expected in ["A-101", "Floor plans", "House on the hill", "Sheet no.", "Plan Ground   1:100", "Plan First   1:100", "Issued for permit", "Window sizes"] {
        assert!(plans.contains(expected), "{expected} in {plans:?}");
    }
    assert!(document.pages[1].text().contains("Section A   1:100") && document.pages[1].text().contains("A-201"));
    assert!(!document.pages[1].text().contains("Issued for permit"));
}

#[test]
fn every_viewport_clips_to_its_window_and_paints_a_drawing() {
    let (bytes, model, _) = written();
    let document = decode_pdf(&bytes).expect("the file reads back");
    for (page, sheet) in document.pages.iter().zip(super::super::ordered(&model)) {
        let viewports = model.viewports.values().filter(|viewport| viewport.sheet == sheet).count();
        let clips = page.content.iter().filter(|op| matches!(op, PdfOp::Clip)).count();
        let strokes = page.content.iter().filter(|op| matches!(op, PdfOp::Stroke | PdfOp::FillStrokeEvenOdd)).count();
        assert_eq!(clips, viewports, "{sheet}");
        assert!(strokes > 5 * viewports, "{sheet}: {strokes} strokes");
    }
}

#[test]
fn the_file_is_deterministic() {
    assert_eq!(written().0, written().0);
}

#[test]
fn text_is_cut_down_to_what_winansi_can_show() {
    assert_eq!(win_ansi("Küche → Bad ⌒ 12 m² × 3"), "Küche -> Bad ~ 12 m² × 3");
    assert_eq!(win_ansi("a\tb\nc"), "a b c");
    assert_eq!(win_ansi("日本"), "??");
}

#[test]
fn the_pages_stream_in_the_order_they_are_given() {
    let (_, model, inferred) = written();
    let layouts = super::super::layouts(&model, &inferred, None);
    let drawings = super::super::drawings(&inferred);
    let mut pdf = SheetsPdf::begin("Streamed", layouts.len()).expect("begin");
    for layout in layouts.iter().rev() {
        pdf.page(layout, &drawings, &labels()).expect("a page");
    }
    let document = decode_pdf(&pdf.finish().expect("finish")).expect("the file reads back");
    assert!(document.pages[0].text().contains("A-301") && document.pages[2].text().contains("A-101"));
}
