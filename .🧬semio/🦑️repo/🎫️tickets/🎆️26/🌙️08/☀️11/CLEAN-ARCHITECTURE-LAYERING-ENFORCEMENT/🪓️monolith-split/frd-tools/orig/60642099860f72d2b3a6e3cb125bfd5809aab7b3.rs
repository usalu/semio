//! 🔬️ Full actual Terrain owners are measured against System allocator requests.
use semio_framework_test_original_snapshot_owners as original_owners;
use super::*;
use semio_framework_io_sqlite_snapshot_testing_backing as observed;
#[test]
fn sqlite_snapshot_gis_terrain_full_concrete_requests_preserve_every_owned_field(){
 let plan:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🗺️imported-map/🔣️.json")).unwrap();
 for word in plan["words"].as_array().unwrap(){let bits=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();for presence in 0..3{let mut snapshot=complete_fixture(bits);if presence==0{snapshot.imported_map=None;}else if presence==1{snapshot.imported_map=Some(crate::schema::imported_map::ImportedMap::default());}let limits=SqliteDatabaseLimits::default();let mut accepted=|_|true;let database=original_owners::with_sqlite_recipient(limits,&mut accepted,|sqlite_control|snapshot.to_sqlite_database(sqlite_control)).unwrap();observed::verify_snapshot_backing_by(&snapshot,&database,literal_equal);snapshot.retire_sqlite_snapshot();}}
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
