//! 🧪️ The sheet export job: bounded analysis steps then one sheet per step, the SVG of one sheet and the PDF of the set equal the writers' one-shot files, headings in the language asked for, refusals for an unknown format or sheet, cancelling.

use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::{stem, Output};
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet::title_labels;
use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_bim::editor::bim::commands::export_model::{INFER_PREVIEW, NODES_PER_STEP};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_bim::editor::bim::{BimCommand, BimModelApp};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::pdf::SheetsPdf;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::ink::TitleLabels;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
#[allow(unused_imports)]
use semio_plugin::{ArtifactView, ConfigView, Effect, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_bim::editor::bim::commands::export_sheets::{SheetsJob, Advance, ExportSheets, labels_of};
use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::testkit::{house_with_sheets, inferred};
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::import::decode_pdf;

use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::empty_instance;

fn sheets_job(format: &str, sheet: Option<&str>, locale: Option<&str>, instance: &semio_plugin::ArtifactInstanceOperationOwnerHandle) -> SheetsJob {
    let command = ExportSheets { format: format.into(), sheet: sheet.map(str::to_owned), locale: locale.map(str::to_owned), pressed: None };
    SheetsJob::new(command, Some(instance))
}

fn finished(job: &mut SheetsJob, snapshot: &ModelSnapshot) -> (Output, Vec<&'static str>) {
    let mut stages = Vec::new();
    for _ in 0..10_000 {
        match job.advance_cold(snapshot, 3).expect("a step") {
            Advance::Progress(stage) => stages.push(stage),
            Advance::Done(output) => return (output, stages),
        }
    }
    panic!("the job never finished");
}

#[test]
fn the_pdf_job_analyses_in_bounded_steps_then_writes_a_page_per_sheet_and_equals_the_one_shot_file() {
    let instance = empty_instance();
    let model = house_with_sheets();
    let mut job = sheets_job("pdf", None, None, &instance);
    let (output, stages) = finished(&mut job, &model);
    assert_eq!(stages.first(), Some(&"bim-export-infer"));
    assert!(stages.iter().filter(|stage| **stage == "bim-export-infer").count() >= 2, "three nodes a step need several steps");
    assert_eq!(stages.iter().filter(|stage| **stage == "bim-sheets-page").count(), 3);
    assert_eq!((output.mime_type, output.encoding), ("application/pdf", Some(MEDIA_EXPORT_BASE64_ENCODING)));
    assert_eq!(output.filename, format!("{}-sheets.pdf", stem(&model.project.name)));
    let bytes = semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::sheets_pdf(&model, &inferred(&model), None, &title_labels(&BimLabels::NATIVE_EN)).expect("the one-shot file");
    assert_eq!(output.data, semio_framework_io_base64::base64_standard_encode(&bytes));
    assert_eq!(decode_pdf(&bytes).expect("reads").pages.len(), 3);
}

#[test]
fn the_svg_job_writes_the_first_sheet_or_the_one_it_names() {
    let instance = empty_instance();
    let model = house_with_sheets();
    let (first, _) = finished(&mut sheets_job("svg", None, None, &instance), &model);
    assert_eq!((first.mime_type, first.encoding), ("image/svg+xml", None));
    assert!(first.filename.ends_with("-A-101.svg") && first.data.contains("data-sheet=\"sh-plans\""));
    let (named, _) = finished(&mut sheets_job("svg", Some("sh-sections"), None, &instance), &model);
    assert!(named.filename.ends_with("-A-201.svg") && named.data.contains("data-sheet=\"sh-sections\""));
}

#[test]
fn a_pdf_of_one_named_sheet_has_one_page() {
    let instance = empty_instance();
    let model = house_with_sheets();
    let (output, _) = finished(&mut sheets_job("pdf", Some("sh-elevations"), None, &instance), &model);
    let bytes = semio_framework_io_base64::base64_standard_decode(&output.data).expect("base64");
    assert_eq!(decode_pdf(&bytes).expect("reads").pages.len(), 1);
    assert!(output.filename.ends_with("-A-301.pdf"));
}

#[test]
fn the_headings_follow_the_language_of_the_command() {
    let instance = empty_instance();
    let model = house_with_sheets();
    let english = finished(&mut sheets_job("svg", None, Some("en"), &instance), &model).0.data;
    let german = finished(&mut sheets_job("svg", None, Some("de"), &instance), &model).0.data;
    assert!(english.contains(">Drawn by<") && !english.contains(">Gezeichnet<"));
    assert!(german.contains(">Gezeichnet<") && !german.contains(">Drawn by<"));
    assert_eq!(labels_of(Some("de")).window_sheet.as_str(), BimLabels::NATIVE_DE.window_sheet.as_str());
    assert_eq!(labels_of(None).window_sheet.as_str(), BimLabels::NATIVE_EN.window_sheet.as_str());
}

#[test]
fn an_unknown_format_an_unknown_sheet_and_a_model_without_sheets_are_refused() {
    let instance = empty_instance();
    let model = house_with_sheets();
    assert_eq!(sheets_job("dwg", None, None, &instance).advance_cold(&model, 3).expect_err("a refusal").code.0.as_str(), "bim.export.format-unknown");
    assert_eq!(sheets_job("pdf", Some("sh-gone"), None, &instance).advance_cold(&model, 3).expect_err("a refusal").code.0.as_str(), "bim.export.sheet-missing");
    assert_eq!(sheets_job("pdf", None, None, &instance).advance_cold(&semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::testkit::house(), 3).expect_err("a refusal").code.0.as_str(), "bim.export.sheets-none");
}

#[test]
fn cancelling_drops_the_pages_and_the_fraction_grows_with_the_work() {
    let instance = empty_instance();
    let model = house_with_sheets();
    let mut job = sheets_job("pdf", None, None, &instance);
    let mut seen = vec![job.fraction(&model)];
    for _ in 0..10_000 {
        let advance = job.advance_cold(&model, 3).expect("a step");
        seen.push(job.fraction(&model));
        if advance == Advance::Progress("bim-sheets-page") {
            break;
        }
    }
    assert!(seen.windows(2).all(|pair| pair[0] <= pair[1]) && seen[0] < *seen.last().unwrap());
    assert!(job.fraction(&model) > 0.5, "a page was staged before cancellation");
    job.cancel_cold(&model);
    assert_eq!(job.fraction(&model), 0.5, "the unpublished pages were discarded");
}
