//! 🧪️ Actual native page ranges match the closed neutral corpus and refuse noncanonical offsets.
use super::*;
#[test]
fn native_inspector_page_ranges_match_neutral_fields_and_choices(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-pages/🔣️.json")).unwrap();
 for row in law["fields"].as_array().unwrap(){
  let page=NormInspectorFieldPage::admit(row["count"].as_u64().unwrap() as usize,row["offset"].as_u64().unwrap() as usize).unwrap();
  assert_eq!(serde_json::json!({"start":page.start,"end":page.end,"previous":page.previous,"next":page.next}),row["expected"]);
 }
 for row in law["choices"].as_array().unwrap(){
  let page=NormInspectorChoicePage::admit(row["count"].as_u64().unwrap() as usize,row["offset"].as_u64().unwrap() as usize,Some(row["selected"].as_u64().unwrap() as usize)).unwrap();
  assert_eq!(serde_json::json!({"start":page.start,"end":page.end,"previous":page.previous,"next":page.next,"resident":page.resident,"length":page.len()}),row["expected"]);
 }
 for(count,offset)in[(0,1),(1,1),(16,17)]{assert!(NormInspectorFieldPage::admit(count,offset).is_err());}
 for(count,offset,selected)in[(0,0,None),(65,1,None),(65,93,None),(2,1,None),(2,0,Some(2))]{assert!(NormInspectorChoicePage::admit(count,offset,selected).is_err());}
}
