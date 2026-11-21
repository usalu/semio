use semio_framework_pack_json::{JsonMemberPolicy, parse, to_dsl_value};
use semio_framework_value::DslValue;
use std::fs;
use terrain_descriptor::{TerrainDescriptorJson, TerrainOverlays, TerrainProjectOrigin, overlays::map_point, scene::terrain_scene_json};

fn read(path: &str) -> DslValue {
    to_dsl_value(&parse(&fs::read_to_string(path).expect("input file"), JsonMemberPolicy::Reject).expect("JSON fixture"))
}

fn project(map: &DslValue) -> TerrainOverlays {
    let records = |name| map.get(name).and_then(DslValue::as_array).unwrap_or_default();
    TerrainOverlays::from_map_features(records("positions"), records("routes"), records("regions"))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let fixture = read(&args[2]);
    let imported = project(fixture.get("imported").unwrap());
    let imported_origin = imported.center().unwrap();
    let imports = TerrainDescriptorJson { schema: "gis.terrain".into(), project_origin: imported_origin, overlays: imported, exaggeration: 1.5 };
    let actual = to_dsl_value(&parse(&terrain_scene_json(&imports, 6, 14), JsonMemberPolicy::Reject).expect("scene JSON"));
    assert_eq!(actual.get("overlays"), fixture.get("overlays"));
    let origin = fixture.get("origin").unwrap().as_array().unwrap();
    for (field, expected) in ["projectOriginLon", "projectOriginLat"].into_iter().zip(origin) {
        let coordinate = actual.get(field).unwrap().as_f64().unwrap();
        assert!((coordinate - expected.as_f64().unwrap()).abs() < 1e-9);
    }

    let scenery = read(&args[1]);
    let [lon, lat] = map_point(scenery.get("projectOrigin").unwrap()).unwrap();
    let demo = TerrainDescriptorJson {
        schema: "gis.terrain".into(),
        project_origin: TerrainProjectOrigin { lon, lat },
        overlays: project(&scenery),
        exaggeration: 1.5,
    };
    print!("{}", terrain_scene_json(&demo, 6, 14));
}
