use super::*;
use semio_framework_plugin::HistoryView;

#[test]
fn dropping_a_pdf_links_the_native_artifact_on_an_image_frame() {
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let config = NoConfig::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(
        &CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: "pdf".into(), x: 10.0, y: 10.0, width: 800.0, height: 600.0, artifact_ref: "doc-pdf".into(), proxy_data_url: String::new() },
        &doc,
        &cfg,
    )
    .expect("drop");
    let LayoutMutation::CreateLink(created) = &emit.artifact_mutations[0] else { panic!("link") };
    assert_eq!(created.link.artifact_kind, "s.stdio.pdf");
    assert_eq!(created.link.artifact_ref, "doc-pdf");
    let LayoutMutation::CreateFrame(frame) = &emit.artifact_mutations[1] else { panic!("frame") };
    let crate::Frame::Image { link_id, .. } = &frame.frame else { panic!("image frame") };
    assert_eq!(link_id, &created.link.id);
}

#[test]
fn every_placeable_artifact_kind_links_its_native_dialect() {
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let config = NoConfig::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    for (kind, _, artifact_kind) in crate::editor::layout::panels::catalogue::NATIVE_PLACEMENTS {
        let emit = handle(&CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: (*kind).into(), x: 0.0, y: 0.0, width: 100.0, height: 100.0, artifact_ref: format!("ref-{kind}"), proxy_data_url: String::new() }, &doc, &cfg).expect(kind);
        let LayoutMutation::CreateLink(created) = &emit.artifact_mutations[0] else { panic!("{kind} link") };
        assert_eq!(created.link.artifact_kind, *artifact_kind, "{kind}");
        assert_eq!(created.link.artifact_ref, format!("ref-{kind}"));
    }
}

#[test]
fn a_drop_payload_keeps_the_native_preview_image() {
    use crate::editor::layout::LayoutPlayApp;
    use semio_framework_plugin::ArtifactEditor;
    let drag = r#"{"kind":"png","artifactRef":"shot","proxyDataUrl":"data:image/png;base64,AA=="}"#;
    let args = dsl::DslValue::Object(vec![
        ("dragData".into(), dsl::DslValue::String(drag.into())),
        ("x".into(), dsl::DslValue::Number(dsl::Number::Float(12.0))),
        ("y".into(), dsl::DslValue::Number(dsl::Number::Float(24.0))),
        ("width".into(), dsl::DslValue::Number(dsl::Number::Float(800.0))),
        ("height".into(), dsl::DslValue::Number(dsl::Number::Float(600.0))),
    ]);
    let command = LayoutPlayApp::command_from_action("canvasDrop", Some(&args)).expect("bridge");
    let crate::editor::layout::LayoutCommand::CanvasDrop(payload) = command else { panic!("drop") };
    assert_eq!(payload.kind, "png");
    assert_eq!(payload.artifact_ref, "shot");
    assert_eq!(payload.proxy_data_url, "data:image/png;base64,AA==");
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let config = NoConfig::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&payload, &doc, &cfg).expect("place");
    let LayoutMutation::CreateLink(created) = &emit.artifact_mutations[0] else { panic!("link") };
    assert_eq!(created.link.artifact_kind, "s.stdio.png");
    assert_eq!(created.link.proxy_data_url.as_deref(), Some("data:image/png;base64,AA=="));
    assert_eq!(created.link.state.as_deref(), Some("ready"));
}
