//! 🧪️ The sheet export job: bounded analysis steps then one sheet per step, the SVG of one sheet and the PDF of the set equal the writers' one-shot files, headings in the language asked for, refusals for an unknown format or sheet, cancelling.

use super::*;
use crate::standards::v1::subsets::any::io::export::sheets::testkit::{house_with_sheets, inferred, labels};
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::decode_pdf;

fn finished(job: &mut SheetsJob, snapshot: &ModelSnapshot) -> (Output, Vec<&'static str>) {
    let mut stages = Vec::new();
    for _ in 0..10_000 {
        match job.advance(snapshot).expect("a step") {
            Advance::Progress(stage) => stages.push(stage),
            Advance::Done(output) => return (output, stages),
        }
    }
    panic!("the job never finished");
}

#[test]
fn the_pdf_job_analyses_in_bounded_steps_then_writes_a_page_per_sheet_and_equals_the_one_shot_file() {
    let model = house_with_sheets();
    let mut job = SheetsJob::with_steps("pdf", None, None, Some(80), 3);
    let (output, stages) = finished(&mut job, &model);
    assert_eq!(stages.first(), Some(&"bim-export-infer"));
    assert!(stages.iter().filter(|stage| **stage == "bim-export-infer").count() >= 2, "three nodes a step need several steps");
    assert_eq!(stages.iter().filter(|stage| **stage == "bim-sheets-page").count(), 3);
    assert_eq!((output.mime_type, output.encoding), ("application/pdf", Some(MEDIA_EXPORT_BASE64_ENCODING)));
    assert_eq!(output.filename, format!("{}-sheets.pdf", stem(&model.project.name)));
    let bytes = sheets::sheets_pdf(&model, &inferred(&model), None, &title_labels_en()).expect("the one-shot file");
    assert_eq!(output.data, semio_framework_io_base64::base64_standard_encode(&bytes));
    assert_eq!(decode_pdf(&bytes).expect("reads").pages.len(), 3);
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::close(80);
}

fn title_labels_en() -> TitleLabels {
    let _ = labels();
    title_labels(&BimLabels::NATIVE_EN)
}

#[test]
fn the_svg_job_writes_the_first_sheet_or_the_one_it_names() {
    let model = house_with_sheets();
    let (first, _) = finished(&mut SheetsJob::new("svg", None, None, Some(81)), &model);
    assert_eq!((first.mime_type, first.encoding), ("image/svg+xml", None));
    assert!(first.filename.ends_with("-A-101.svg") && first.data.contains("data-sheet=\"sh-plans\""));
    let (named, _) = finished(&mut SheetsJob::new("svg", Some("sh-sections"), None, Some(81)), &model);
    assert!(named.filename.ends_with("-A-201.svg") && named.data.contains("data-sheet=\"sh-sections\""));
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::close(81);
}

#[test]
fn a_pdf_of_one_named_sheet_has_one_page() {
    let model = house_with_sheets();
    let (output, _) = finished(&mut SheetsJob::new("pdf", Some("sh-elevations"), None, Some(82)), &model);
    let bytes = semio_framework_io_base64::base64_standard_decode(&output.data).expect("base64");
    assert_eq!(decode_pdf(&bytes).expect("reads").pages.len(), 1);
    assert!(output.filename.ends_with("-A-301.pdf"));
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::close(82);
}

#[test]
fn the_headings_follow_the_language_of_the_command() {
    let model = house_with_sheets();
    let english = finished(&mut SheetsJob::new("svg", None, Some("en"), Some(83)), &model).0.data;
    let german = finished(&mut SheetsJob::new("svg", None, Some("de"), Some(83)), &model).0.data;
    assert!(english.contains(">Drawn by<") && !english.contains(">Gezeichnet<"));
    assert!(german.contains(">Gezeichnet<") && !german.contains(">Drawn by<"));
    assert_eq!(labels_of(Some("de")).window_sheet.as_str(), BimLabels::NATIVE_DE.window_sheet.as_str());
    assert_eq!(labels_of(None).window_sheet.as_str(), BimLabels::NATIVE_EN.window_sheet.as_str());
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::close(83);
}

#[test]
fn an_unknown_format_an_unknown_sheet_and_a_model_without_sheets_are_refused() {
    let model = house_with_sheets();
    assert_eq!(SheetsJob::new("dwg", None, None, Some(84)).advance(&model).expect_err("a refusal").code.as_str(), "bim.export.format-unknown");
    assert_eq!(SheetsJob::new("pdf", Some("sh-gone"), None, Some(84)).advance(&model).expect_err("a refusal").code.as_str(), "bim.export.sheet-missing");
    assert_eq!(SheetsJob::new("pdf", None, None, Some(84)).advance(&crate::standards::v1::subsets::any::io::export::svg::testkit::house()).expect_err("a refusal").code.as_str(), "bim.export.sheets-none");
}

#[test]
fn cancelling_drops_the_pages_and_the_fraction_grows_with_the_work() {
    let model = house_with_sheets();
    let mut job = SheetsJob::with_steps("pdf", None, None, Some(85), 3);
    let mut seen = vec![job.fraction(&model)];
    for _ in 0..4 {
        job.advance(&model).expect("a step");
        seen.push(job.fraction(&model));
    }
    assert!(seen.windows(2).all(|pair| pair[0] <= pair[1]) && seen[0] < *seen.last().unwrap());
    job.cancel(&model);
    assert!(job.writer.is_none());
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::close(85);
}
