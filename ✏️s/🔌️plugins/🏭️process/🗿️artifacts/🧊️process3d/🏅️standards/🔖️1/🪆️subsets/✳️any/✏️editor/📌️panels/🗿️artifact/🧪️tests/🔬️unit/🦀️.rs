
use super::*;
use crate::editor::process3d::testkit;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_DOCUMENT));
}

#[semio_framework_async_macros::async_test]
async fn document_panel_lists_stock_and_steps() {
    let mut app = testkit::app();
    let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_DOCUMENT);
    assert!(rendered.contains("process3d-play-document.stock"));
    assert!(rendered.contains("process3d-play-document.steps"));
}

/// 🎞️ `render` must list every `step_payloads` entry, in timeline order — the authoritative
/// record since wave 4, not the unresolvable `steps` child handle.
#[semio_framework_async_macros::async_test]
async fn document_panel_lists_every_step_payload_in_order() {
    use crate::{ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, Workshop, process_working_scene_to_snapshot};
    let scene = ProcessWorkingScene {
        stock: Stock::default(),
        steps: vec![
            ProcessStep { id: "step-rip".into(), label: "Rip Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: Default::default(), pose: Default::default() } },
            ProcessStep { id: "step-bore".into(), label: "Bore Hole".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.01, depth: 0.02, pose: Default::default() } },
            ProcessStep { id: "step-dowel".into(), label: "Attach Dowel".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: Default::default(), pose: Default::default() } },
        ],
    };
    let fixture = process_working_scene_to_snapshot(&scene, Workshop::default(), None);
    let labels = crate::editor::process3d::terminology::process3d_labels(&crate::editor::process3d::config::Process3dConfig::default());
    let node = render(&fixture, labels).expect("document tree renders");
    let rendered = serde_json::to_string(&node).expect("render json");
    let rip_index = rendered.find("step-rip").expect("step-rip present");
    let bore_index = rendered.find("step-bore").expect("step-bore present");
    let dowel_index = rendered.find("step-dowel").expect("step-dowel present");
    assert!(rip_index < bore_index && bore_index < dowel_index, "expected steps in timeline order: {rendered}");
}
