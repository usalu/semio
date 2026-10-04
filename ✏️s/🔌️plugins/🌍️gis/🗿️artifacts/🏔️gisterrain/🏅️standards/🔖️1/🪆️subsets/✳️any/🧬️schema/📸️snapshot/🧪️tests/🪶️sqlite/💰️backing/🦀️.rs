//! 🔬️ Full actual Terrain owners are measured against System allocator requests.
use super::*;
#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;
#[test]
fn sqlite_snapshot_gis_terrain_full_concrete_requests_preserve_every_owned_field(){
 let plan:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🗺️imported-map/🔣️.json")).unwrap();
 for word in plan["words"].as_array().unwrap(){let bits=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();for presence in 0..3{let mut snapshot=complete_fixture(bits);if presence==0{snapshot.imported_map=None;}else if presence==1{snapshot.imported_map=Some(crate::schema::ImportedMap::default());}let limits=SqliteDatabaseLimits::default();let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);let database=snapshot.to_sqlite_database(&mut control).unwrap();observed::verify_snapshot_backing_by(&snapshot,&database,literal_equal);snapshot.retire_sqlite_snapshot();}}
}
