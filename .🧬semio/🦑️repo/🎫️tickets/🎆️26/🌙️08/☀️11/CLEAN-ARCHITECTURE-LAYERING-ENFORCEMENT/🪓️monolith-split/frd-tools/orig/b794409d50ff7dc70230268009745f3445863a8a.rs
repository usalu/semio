//! 🌐️ Neutral scene projection compared through the independent serde JSON implementation.
use crate::schema::snapshot::Grid3dSnapshot;
use crate::standards::v1::subsets::any::io::text::inferences::scene_projection::*;
#[test]
fn nonuniform_scene_values_agree_with_independent_json_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🚪️io/📝️text/💡️inferences/🌐️scene-projection/🧪️tests/🧫️fixtures/🧱️nonuniform/🔣️.json")).unwrap();
    let mut snapshot = Grid3dSnapshot::default();
    snapshot.width = fixture["dimensions"][0].as_u64().unwrap() as u32;
    snapshot.height = fixture["dimensions"][1].as_u64().unwrap() as u32;
    snapshot.depth = fixture["dimensions"][2].as_u64().unwrap() as u32;
    let axis = |index:usize| fixture["sizes"][index].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()).collect();
    snapshot.cell_sizes_x = axis(0);
    snapshot.cell_sizes_y = axis(1);
    snapshot.cell_sizes_z = axis(2);
    snapshot.tiles.clear();
    snapshot.pinned.clear();
    snapshot.masked.clear();
    let encoded = grid_instances_json(&snapshot);
    let actual: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    let expected: serde_json::Value = serde_json::from_str(&serde_json::to_string(&fixture["instances"]).unwrap()).unwrap();
    assert_eq!(actual,expected);
    assert_eq!(preview_instances_json(&snapshot,&[]),"[]");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
