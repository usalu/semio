//! 🔬️ Complete Layout ownership is checked against actual System allocation requests.
use super::*;
use store::ArtifactSqliteSnapshot;
#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;
fn literal_equal(actual:&LayoutSnapshot,expected:&LayoutSnapshot){assert_eq!(actual.data_fields,expected.data_fields,"complete typed dictionary tags, words, octets and member order");assert_eq!(store::ArtifactPack::encode_pack(actual),store::ArtifactPack::encode_pack(expected),"complete Layout native fields and raw words");}
#[test]
fn sqlite_snapshot_layout_full_concrete_requests_preserve_every_owned_field(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🧾️dictionary/🔣️.json")).unwrap();assert_eq!(plan["allocation"],"fullConcreteRequests");assert_eq!(plan["retirementRefund"],false);
 let words:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
 for(index,hex)in words["float64Bits"].as_array().unwrap().iter().enumerate(){
  let bits=u64::from_str_radix(hex.as_str().unwrap(),16).unwrap();let color=u32::from_str_radix(words["float32Bits"][index].as_str().unwrap(),16).unwrap();
  let expected=complete_fixture(f64::from_bits(bits),f32::from_bits(color));let mut accepted=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut accepted,store::sqlite_snapshot::SqliteDatabaseLimits::default());
  let database=expected.to_sqlite_database(&mut control).unwrap();observed::verify_snapshot_backing_by(&expected,&database,literal_equal);expected.retire_sqlite_snapshot();
 }
}

