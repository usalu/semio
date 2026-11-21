use semio_framework::ActionBus;
use semio_plugin::{ArtifactApp, ArtifactToolFactoryRegistry, EditorApp};
use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::Gis2dPlayApp;
use semio_s_artifact_gis_gisterrain_editor::editor::gis3d::Gis3dPlayApp;

fn inspect<A: ArtifactApp>(controller: &str, expected: &[&str]) {
    let bus = ActionBus::new();
    let mut registry = ArtifactToolFactoryRegistry::<A>::new(&bus, controller);
    A::register_tool_job_factories(&mut registry).expect("registered editor tool owners");
    let mut exports = Vec::new();
    for (id, registration) in registry.finish() {
        if id.starts_with("export-media:") {
            assert_eq!(registration.contract.max_step_micros, 2_000);
            println!("{} {} {}", controller, id, registration.contract.max_step_micros);
            exports.push(id);
        }
    }
    assert_eq!(exports, expected);
}

fn main() {
    inspect::<EditorApp<Gis2dPlayApp>>("s.gis.gismap@1/*#editor", &["export-media:map:out"]);
    inspect::<EditorApp<Gis3dPlayApp>>("s.gis.gisterrain@1/*#editor", &["export-media:elevation:out", "export-media:scene:out"]);
}
