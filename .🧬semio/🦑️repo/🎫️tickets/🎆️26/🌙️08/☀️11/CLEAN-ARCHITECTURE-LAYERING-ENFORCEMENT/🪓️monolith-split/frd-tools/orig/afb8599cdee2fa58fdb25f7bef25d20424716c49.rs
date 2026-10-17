use super::*;
#[test]
fn painted_geometry_fixture_is_incremental_and_bounded() {
    let cases:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧮️geometry/🎯️picking/🔣️.json")).unwrap();
    for sample in cases.as_array().unwrap(){
        let segments:Vec<PathSegment>=serde_json::from_value(sample["segments"].clone()).unwrap();
        let mut cursor=PathHitCursor::new(serde_json::from_value(sample["point"].clone()).unwrap(),serde_json::from_value(sample["matrix"].clone()).unwrap(),sample["radius"].as_f64().unwrap(),0.001);
        let mut steps=0;
        while !cursor.step(&segments){steps+=1;assert!(steps<100000);}
        assert_eq!(cursor.contains(sample["fill"].as_bool().unwrap(),sample["stroke"].as_bool().unwrap(),false),sample["expected"].as_bool().unwrap(),"{}",sample["name"]);
        assert!(cursor.maximum_depth<=33);
    }
}

#[test]
fn curve_picking_yields_with_bounded_storage_and_refuses_unresolved_geometry() {
    let segments=[PathSegment::Move {to:[0.0,0.0]},PathSegment::Cubic {ctrl1:[0.0,1e100],ctrl2:[1e100,1e100],to:[1e100,0.0]}];
    let mut cursor=PathHitCursor::new([0.0,0.0],[1.0,0.0,0.0,1.0,0.0,0.0],1.0,0.001);
    for _ in 0..16 {assert!(!cursor.step(&segments));}
    let mut steps=16;while !cursor.step(&segments){steps+=1;assert!(steps<100);}
    assert!(cursor.failed());assert!(!cursor.contains(true,true,false));assert!(cursor.maximum_depth<=33);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
