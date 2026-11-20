use crate::modes::edit::windows::outliner::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_OUTLINER);
    assert_eq!(definition().id, WINDOW_OUTLINER);
}

#[semio_framework_async_macros::async_test]
async fn renders_every_collection_with_a_select_row() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_OUTLINER).await;
    assert!(rendered.contains("member-b1") && rendered.contains("joint-j1"), "outliner lists entities: {rendered}");
    let kinds = semio_s_artifact_norm_en1993_scene::ENTITY_KINDS;
    let projection = semio_framework_value::ToValue::to_value(&app.snapshot().expect("projection"));
    let groups: Vec<_> = kinds.iter().filter(|kind| matches!(projection.get(kind.collection), Some(semio_framework_value::DslValue::Array(_)))).collect();
    assert!(rendered.contains(&format!("\"total\":{}", groups.len())), "the outliner declares all collection groups: {rendered}");
    let final_group = context::render_with_windows(&mut app, BODY_OUTLINER, vec![semio_framework::TreeWindowRequest {
        body_key: BODY_OUTLINER.into(),
        node_key: "norm-outliner-root".into(),
        open: Some(true),
        offset: (groups.len() - 1) as u32,
        rows: 1,
    }]).await;
    assert!(final_group.contains(groups.last().expect("outliner group").label_en), "the host can reach the final collection: {final_group}");
    assert!(!final_group.contains("member-b1"), "the final-group request only renders that group");
    context::close(&mut app);
}
