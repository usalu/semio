use super::*;
use crate::sample_plugin;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_block_list_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, ARCHITECT_BODY_REGISTER);
    assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
}

#[semio_framework_async_macros::async_test]
async fn the_active_registers_rows_become_block_list_steps() {
    let node = render(&sample_plugin(), &ArchitectConfig::default()).expect("register");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("register surface") };
    let scene: BlockListScene = semio_framework_ui_scene::decode(props).expect("packed register");
    let steps: serde_json::Value = serde_json::from_str(&scene.steps_json).expect("independent step oracle");
    assert!(steps.to_string().contains("Reception"));
    crate::editor::architect::unit_tests::context::project_render(Ok(node));
}

#[semio_framework_async_macros::async_test]
async fn an_empty_register_renders_the_placeholder() {
    let cfg = ArchitectConfig { active_register: "benchmarks".into(), ..ArchitectConfig::default() };
    let json = crate::editor::architect::unit_tests::context::project_render(render(&sample_plugin(), &cfg));
    assert!(json.contains("No entities in register 'benchmarks'"));
}
