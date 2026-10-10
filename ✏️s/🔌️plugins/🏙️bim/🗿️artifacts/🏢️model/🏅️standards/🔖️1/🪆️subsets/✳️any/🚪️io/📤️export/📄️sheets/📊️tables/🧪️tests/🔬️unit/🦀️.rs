//! 🧪️ The export tables of the room: every sheet is listed with its clip windows, the title block texts carry the authored fields, and the PDF table lists one page per sheet in print order.

use super::*;
use crate::standards::v1::subsets::any::io::export::sheets::layouts;
use crate::standards::v1::subsets::any::io::export::sheets::testkit::{inferred, labels, room};

#[test]
fn the_svg_table_lists_every_sheet_with_its_windows_and_title_texts() {
    let (model, labels) = (room(), labels());
    let inferred = inferred(&model);
    let table: serde_json::Value = serde_json::from_str(&svg_table_json(&layouts(&model, &inferred, None), &labels)).expect("the table is JSON");
    assert_eq!(table.as_object().map(|sheets| sheets.keys().cloned().collect::<Vec<_>>()), Some(vec!["sh-custom".to_string(), "sh-notes".to_string(), "sh-plans".to_string()]));
    let plans = &table["sh-plans"];
    assert_eq!(plans["paper"], "A3");
    assert_eq!(plans["size"], serde_json::json!([420.0, 297.0]));
    assert_eq!(plans["viewports"]["vp-plan"]["window"], serde_json::json!([30.0, 30.0, 80.0, 60.0]));
    assert_eq!(plans["viewports"]["vp-south"]["scale"], 50);
    let texts: Vec<&str> = plans["title_texts"].as_array().expect("texts").iter().filter_map(|text| text.as_str()).collect();
    assert!(texts.contains(&"A-101") && texts.contains(&"Plan and elevation") && texts.contains(&"1:25, 1:50, 1:100") && texts.contains(&"B"), "{texts:?}");
    assert_eq!(plans["revision_texts"].as_array().map(Vec::len).unwrap_or(0) > 0, true);
}

#[test]
fn the_pdf_table_lists_one_page_per_sheet_in_print_order() {
    let model = room();
    let inferred = inferred(&model);
    let table: serde_json::Value = serde_json::from_str(&pdf_table_json(&layouts(&model, &inferred, None))).expect("the table is JSON");
    let pages = table["pages"].as_array().expect("pages");
    assert_eq!(pages.len(), 3);
    assert_eq!((pages[0]["width"].clone(), pages[0]["height"].clone()), (serde_json::json!(420.0), serde_json::json!(297.0)));
    assert_eq!((pages[1]["width"].clone(), pages[1]["height"].clone()), (serde_json::json!(210.0), serde_json::json!(297.0)));
    assert_eq!((pages[2]["width"].clone(), pages[2]["height"].clone()), (serde_json::json!(500.0), serde_json::json!(350.0)));
}
