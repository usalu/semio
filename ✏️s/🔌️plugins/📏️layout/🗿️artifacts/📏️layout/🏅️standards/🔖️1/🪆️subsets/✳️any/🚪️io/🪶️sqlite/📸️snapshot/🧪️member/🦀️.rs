//! 🧪️ Shared reference-only ownership witnesses drive actual native relational projection and reconstruction.
#![cfg(test)]

extern crate semio_framework_os_kernel as store;
use semio_s_artifact_layout_layout_model::{LayoutDrawingMember,snapshot::LayoutSnapshot};
use store::sqlite_snapshot::SqliteSnapshotControl;
use pack::snapshot::ArtifactSqliteSnapshot;
fn control<'a>(callback:&'a mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool)->SqliteSnapshotControl<'a>{SqliteSnapshotControl::new(callback,Default::default())}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../../../✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/📄️page-canvas/🔣️.json")).unwrap()}
fn document(case:&serde_json::Value)->LayoutSnapshot{let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json")).unwrap();let mut snapshot:LayoutSnapshot=semio_framework_pack_json::from_json_str(&corpus["base"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();while snapshot.pages.len()<2{let mut page=snapshot.pages[0].clone();page.id=format!("page-{}",snapshot.pages.len());snapshot.pages.push(page)}for row in case["members"].as_array().unwrap(){let member=LayoutDrawingMember{handle:store::os_store::artifact_child::ArtifactChild::new(row["childId"].as_str().unwrap().into(),semio_framework_artifact_reference::ArtifactRef{dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.draw.drawing".into(),standard:"1".into(),subset:"*".into()},artifact_id:row["artifactId"].as_str().unwrap().into()})};if row["owner"]=="document"{snapshot.background_drawing=Some(member)}else{snapshot.pages[row["page"].as_u64().unwrap()as usize].background_drawing=Some(member)}}snapshot}
#[test]
fn genuine_reference_rows_round_trip_without_child_geometry_or_inline_codec(){
 let fixture=fixture();let mut callback=|_|true;
 for case in fixture["sqliteCases"].as_array().unwrap(){
  let source=document(case);let database=source.to_sqlite_database(&mut control(&mut callback)).unwrap();
  assert!(database.tables.iter().all(|table|table.name.starts_with("layout_")));
  let table=database.table("layout_drawing_member").unwrap();assert_eq!(table.rows.len(),case["members"].as_array().unwrap().len());
  for(row,expected)in table.rows.iter().zip(case["members"].as_array().unwrap()){assert_eq!(row.rowid,expected["id"].as_i64().unwrap());assert_eq!(row.text(3).unwrap(),expected["childId"].as_str().unwrap());assert_eq!(row.text(7).unwrap(),expected["artifactId"].as_str().unwrap());}
  assert_eq!(LayoutSnapshot::from_sqlite_database(&database,&mut control(&mut callback)).unwrap(),source);
 }
 let source=document(&fixture["sqliteCases"][0]);let database=source.to_sqlite_database(&mut control(&mut callback)).unwrap();
 for(case,name)in fixture["sqliteRefusals"].as_array().unwrap().iter().enumerate(){
  let mut broken=database.clone();let table=broken.tables.iter_mut().find(|table|table.name=="layout_drawing_member").unwrap();
  use store::sqlite_snapshot::SqliteValue;
  match case{
   0=>table.rows[2].values[2]=table.rows[1].values[2].clone(),
   1=>table.rows[1].values[2]=SqliteValue::Integer(999),
   2=>table.rows[1].values[3]=table.rows[0].values[3].clone(),
   3=>table.rows[1].values[7]=table.rows[0].values[7].clone(),
   4=>table.rows[1].values[4]=SqliteValue::Text("s.stdio.semio".into()),
   5=>table.rows[0].values[2]=SqliteValue::Integer(1),
   6=>table.rows[1].values[3]=SqliteValue::Text(String::new()),
   7=>table.rows[1].values[7]=SqliteValue::Text(String::new()),
   _=>panic!("unknown shared member refusal"),
  }
  assert!(LayoutSnapshot::from_sqlite_database(&broken,&mut control(&mut callback)).is_err(),"{name}");
 }
}
