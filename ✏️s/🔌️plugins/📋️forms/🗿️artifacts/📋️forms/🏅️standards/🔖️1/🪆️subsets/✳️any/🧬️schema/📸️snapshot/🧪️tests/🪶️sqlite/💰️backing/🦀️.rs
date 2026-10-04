//! 🔬️ Actual Forms SQL ownership settles full requests while comparing every literal variant.
use super::*;
use semio_framework_value::ToValue;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits}};
#[path = "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;
#[path = "📏️preflight/🦀️.rs"]
mod preflight;
fn verify_literals(actual:&FormsSnapshot,expected:&FormsSnapshot){
 let actual=actual.to_value();let expected=expected.to_value();let mut pending=vec![(&actual,&expected)];
 while let Some((actual,expected))=pending.pop(){match(actual,expected){
  (semio_framework_value::DslValue::Null,semio_framework_value::DslValue::Null)=>{},
  (semio_framework_value::DslValue::Bool(actual),semio_framework_value::DslValue::Bool(expected))=>assert_eq!(actual,expected),
  (semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(actual)),semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(expected)))=>assert_eq!(actual,expected),
  (semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(actual)),semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(expected)))=>assert_eq!(actual,expected),
  (semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(actual)),semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(expected)))=>assert_eq!(actual.to_bits(),expected.to_bits()),
  (semio_framework_value::DslValue::String(actual),semio_framework_value::DslValue::String(expected))=>assert_eq!(actual,expected),
  (semio_framework_value::DslValue::Bytes(actual),semio_framework_value::DslValue::Bytes(expected))=>assert_eq!(actual,expected),
  (semio_framework_value::DslValue::Array(actual),semio_framework_value::DslValue::Array(expected))=>{assert_eq!(actual.len(),expected.len());pending.extend(actual.iter().zip(expected));},
  (semio_framework_value::DslValue::Object(actual),semio_framework_value::DslValue::Object(expected))=>{assert_eq!(actual.len(),expected.len());for((actual_key,actual_value),(expected_key,expected_value))in actual.iter().zip(expected){assert_eq!(actual_key,expected_key);pending.push((actual_value,expected_value));}},
  _=>panic!("Forms actual typed literal variant changed")
 }}
}
#[test]
fn sqlite_snapshot_forms_relational_owned_requests_match_actual_allocator(){
 let contract:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/💰️backing/🔬️requests/🔣️.json")).unwrap();
 assert_eq!(contract["phases"],serde_json::json!(["projectSnapshot","reconstructSnapshot"]));
 let snapshot=specimen();let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(database.tables.len(),usize::try_from(contract["tableCount"].as_u64().unwrap()).unwrap());
 observed::verify_snapshot_backing_by::<FormsSnapshot>(&snapshot,&database,verify_literals);
 snapshot.retire_sqlite_snapshot();
}
