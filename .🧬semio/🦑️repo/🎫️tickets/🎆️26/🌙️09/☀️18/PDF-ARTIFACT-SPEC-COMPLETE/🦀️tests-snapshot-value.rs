use super::*;
use pack::value::{DslValue, FromValue, ToValue};

fn populated_snapshot() -> PdfSnapshot {
    let root = ObjRef { num: 1, gen: 0 };
    let mut page = PdfPage::new(612.0, 792.0);
    page.crop_box = Some([12.0, 12.0, 600.0, 780.0]);
    page.rotate = 90;
    page.content = vec![PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 12.0 }, PdfOp::MoveText { tx: 72.0, ty: 720.0 }, PdfOp::ShowText { text: PdfTextString::text("Semio") }, PdfOp::EndText, PdfOp::Rectangle { x: 1.0, y: 2.0, width: 3.0, height: 4.5 }, PdfOp::Fill];
    page.annotations.push(PdfAnnotation::link([0.0, 0.0, 10.0, 10.0], "https://semio-tech.com"));
    PdfSnapshot {
        pages: vec![page],
        fonts: vec![PdfFont::standard("F1", "Helvetica")],
        images: vec![PdfImage::gray8("Im1", 2, 1, vec![0, 255])],
        outlines: vec![PdfOutlineItem::to_page("Start", 0)],
        info: PdfInfo { title: Some("Value path".to_string()), producer: Some("semio".to_string()), creation_date: Some(PdfDate { year: 2026, month: 9, day: 18, hour: 12, minute: 0, second: 0, offset_minutes: Some(120) }), ..PdfInfo::default() },
        objects: vec![PdfIndirectObject { id: root, value: PdfObject::Name("Catalog".to_string()) }],
        trailer: vec![PdfDictEntry { key: "Root".to_string(), value: PdfObject::Ref(root) }],
        ..PdfSnapshot::default()
    }
}

#[test]
fn populated_snapshot_round_trips_through_value() {
    let snapshot = populated_snapshot();
    assert_eq!(PdfSnapshot::from_value(snapshot.to_value()), Ok(snapshot));
}

#[test]
fn missing_optional_fields_keep_the_derived_defaults() {
    let value = DslValue::object([("schema".to_string(), DslValue::String("stdio.pdf.1.7".to_string()))]);
    assert_eq!(PdfSnapshot::from_value(value), Ok(PdfSnapshot { declared_version: String::new(), ..PdfSnapshot::default() }));
}

#[test]
fn missing_schema_reports_the_exact_field() {
    let error = PdfSnapshot::from_value(DslValue::object([])).unwrap_err();
    assert_eq!(error.to_string(), "missing field `schema`");
}

#[test]
fn camel_case_json_shape_agrees_with_serde_json_oracle() {
    let snapshot = populated_snapshot();
    let actual: serde_json::Value = snapshot.to_value().into();
    assert_eq!(actual["pages"][0]["mediaBox"], serde_json::json!([0.0, 0.0, 612.0, 792.0]));
    assert_eq!(actual["pages"][0]["content"][1], serde_json::json!({"op": "setFont", "name": "F1", "size": 12.0}));
    assert_eq!(actual["pages"][0]["content"][3], serde_json::json!({"op": "showText", "text": {"kind": "text", "text": "Semio"}}));
    assert_eq!(actual["fonts"][0]["kind"]["kind"], serde_json::json!("type1"));
    assert_eq!(actual["fonts"][0]["kind"]["encoding"]["base"], serde_json::json!("winAnsi"));
    assert_eq!(actual["info"]["creationDate"]["offsetMinutes"], serde_json::json!(120));
    assert_eq!(actual["pages"][0]["annotations"][0]["kind"]["action"]["kind"], serde_json::json!({"kind": "uri", "uri": "https://semio-tech.com", "isMap": false}));
}

#[test]
fn page_text_joins_shown_runs() {
    let snapshot = populated_snapshot();
    assert_eq!(snapshot.pages[0].text(), "Semio");
}

#[test]
fn dates_parse_and_print_every_form() {
    let date = PdfDate::parse("D:20260918120000+02'00'").unwrap();
    assert_eq!(date.to_string(), "D:20260918120000+02'00'");
    assert_eq!(PdfDate::parse("D:2026").unwrap(), PdfDate { year: 2026, month: 1, day: 1, hour: 0, minute: 0, second: 0, offset_minutes: None });
    assert_eq!(PdfDate::parse("D:20260918Z").unwrap().offset_minutes, Some(0));
    assert_eq!(PdfDate::parse("garbage"), None);
}

#[test]
fn fresh_ids_avoid_every_collection() {
    let snapshot = populated_snapshot();
    assert_eq!(snapshot.fresh_id("F"), "F2");
    assert_eq!(snapshot.fresh_id("Im"), "Im2");
}
