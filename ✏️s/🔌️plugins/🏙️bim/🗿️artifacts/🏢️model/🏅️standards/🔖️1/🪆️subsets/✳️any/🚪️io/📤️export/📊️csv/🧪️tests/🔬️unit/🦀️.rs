use super::codec::read_records;
use super::*;
use crate::schedule_kit::preset;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::DiagnosticCode;
use protocol::Inference;

fn demo() -> (ModelSnapshot, ModelInference) {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    snapshot.schedules.insert("sch-walls".into(), preset("wall", "Walls").expect("the wall preset"));
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    (snapshot, inferred)
}

#[test]
fn numbers_have_at_most_six_decimals_and_no_trailing_zeros() {
    assert_eq!(number_text(8.0), "8");
    assert_eq!(number_text(2.5), "2.5");
    assert_eq!(number_text(1.0 / 3.0), "0.333333");
    assert_eq!(number_text(-0.0000001), "0", "no negative zero");
    assert_eq!(number_text(-2.25), "-2.25");
}

#[test]
fn a_schedule_table_is_a_header_and_a_record_per_row_with_a_total_label() {
    let (snapshot, inferred) = demo();
    let text = schedule_csv(&snapshot, &inferred, "sch-walls").expect("the schedule");
    assert!(text.contains("\r\n") && !text.replace("\r\n", "").contains('\n'), "CRLF line endings throughout");
    let records = read_records(&text);
    assert_eq!(records.len(), inferred.schedules["sch-walls"].rows.len() + 1);
    assert_eq!(records[0], snapshot.schedules["sch-walls"].columns.iter().map(|column| column.key.token()).collect::<Vec<_>>(), "a column without a heading speaks its key token");
    assert!(records.iter().all(|record| record.len() == records[0].len()), "every record has one field per column");
    assert_eq!(records.last().expect("records")[0], "total", "the grand total row is labelled");
    assert_eq!(schedule_csv(&snapshot, &inferred, "no-such-schedule"), None);
}

#[test]
fn headings_with_separators_and_quotes_are_quoted_and_round_trip_through_the_decoder() {
    let (mut snapshot, _) = demo();
    snapshot.schedules.get_mut("sch-walls").expect("the schedule").columns[0].heading = Some("Wall, \"name\"".into());
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let text = schedule_csv(&snapshot, &inferred, "sch-walls").expect("the schedule");
    assert!(text.starts_with("\"Wall, \"\"name\"\"\","), "{text}");
    assert_eq!(read_records(&text)[0][0], "Wall, \"name\"");
}

#[test]
fn diagnostics_are_a_record_per_finding_with_both_languages() {
    let mut finding = Diagnostic::new(DiagnosticCode::ClashWallWall, &["w-a", "w-b"]);
    finding.storey = Some("st-ground".into());
    let records = diagnostics_records(&[finding.clone()]);
    assert_eq!(records[0], ["severity", "code", "storey", "elements", "missing", "message_en", "message_de"]);
    assert_eq!(records.len(), 2);
    assert_eq!((records[1][0].as_str(), records[1][1].as_str(), records[1][2].as_str(), records[1][3].as_str()), (format!("{:?}", finding.severity).to_lowercase().as_str(), DiagnosticCode::ClashWallWall.slug(), "st-ground", "w-a+w-b"));
    assert_ne!(records[1][5], records[1][6], "the English and German messages differ");
    let inferred = ModelInference { diagnostics: vec![finding], ..ModelInference::default() };
    assert_eq!(read_records(&diagnostics_csv(&inferred)), records);
}

#[test]
fn the_report_lists_every_schedule_then_the_diagnostics_and_is_deterministic() {
    let (snapshot, inferred) = demo();
    let report = report_csv(&snapshot, &inferred);
    assert_eq!(report, report_csv(&snapshot, &ModelInference::infer(&snapshot).expect("infers")));
    let records = read_records(&report);
    assert_eq!(records[0], ["schedule", "sch-walls", "Walls", "wall"]);
    let diagnostics = records.iter().position(|record| record[0] == "diagnostics").expect("the diagnostics section");
    assert_eq!(records[diagnostics][1], inferred.diagnostics.len().to_string());
    assert_eq!(records[diagnostics - 1], vec![String::new()], "sections are separated by an empty record");
    assert_eq!(records[diagnostics + 1][0], "severity");
}

const DEFECTS: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed exports of the findings of the defect house: the files the python `csv` and `json` modules read back in the export case.
const EXPORT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/⚠️diagnostics/💥️defects/📤️export");

fn defects() -> ModelInference {
    let snapshot: ModelSnapshot = semio_framework_pack_json::from_json_str(DEFECTS, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the defect house decodes");
    ModelInference::infer(&snapshot).expect("infers")
}

#[test]
fn every_finding_of_the_defect_house_is_one_record_that_reads_back_with_its_code_storey_and_elements() {
    let inferred = defects();
    let records = read_records(&diagnostics_csv(&inferred));
    assert_eq!(records.len(), inferred.diagnostics.len() + 1);
    for (record, found) in records[1..].iter().zip(&inferred.diagnostics) {
        assert_eq!((record[1].as_str(), record[2].as_str(), record[3].as_str(), record[4].as_str()), (found.code.slug(), found.storey.as_deref().unwrap_or(""), found.elements.join("+").as_str(), found.missing.join("+").as_str()));
        assert_eq!(record[0], format!("{:?}", found.severity).to_lowercase());
        assert_eq!(Some(record[5].clone()), found.text("en"));
        assert_eq!(Some(record[6].clone()), found.text("de"));
    }
}

#[test]
fn the_committed_csv_file_is_the_current_export_of_the_defect_house() {
    let text = diagnostics_csv(&defects());
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(EXPORT_DIR).expect("the fixture directory");
        std::fs::write(format!("{EXPORT_DIR}/⚠️diagnostics.csv"), text.as_bytes()).expect("the file is written");
    }
    let committed = std::fs::read(format!("{EXPORT_DIR}/⚠️diagnostics.csv")).unwrap_or_else(|error| panic!("{error}. Run the test with BIM_BLESS=1 to write the file."));
    assert_eq!(String::from_utf8(committed).expect("UTF-8"), text, "the committed export drifted: rewrite it with BIM_BLESS=1");
}
