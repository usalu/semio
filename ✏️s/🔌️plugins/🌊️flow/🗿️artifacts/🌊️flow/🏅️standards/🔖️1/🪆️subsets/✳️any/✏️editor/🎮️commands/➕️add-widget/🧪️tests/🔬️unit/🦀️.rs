use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;
use store::{ArtifactPack, SpaceMember};

#[test]
fn child_add_widget_uses_the_smallest_available_identity_and_the_descriptor_default_payload() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::apply_semio_flow_mutation;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowNode;

    let existing = |id: &str| FlowNode { id: id.into(), kind: "inputNote".into(), label: "inputNote".into(), params: vec![], position: Default::default() };
    let mut content = SemioFlowSnapshot { nodes: vec![existing("note_2"), existing("note_4")], ..Default::default() };
    let before = content.clone();
    let descriptor = r#"{"kind":"inputNote"}"#;
    let descriptor_reference: serde_json::Value = serde_json::from_str(descriptor).expect("serde descriptor reference");
    assert_eq!(descriptor_reference, serde_json::json!({ "kind": "inputNote" }));

    let mutation = child_add_widget_mutation(&content, descriptor, 40.0, 51.0).expect("host-free child mutation");
    apply_semio_flow_mutation(&mut content, &mutation);

    assert_eq!(&content.nodes[..before.nodes.len()], before.nodes.as_slice());
    assert_eq!(content.edges, before.edges);
    let inserted = content.nodes.last().expect("appended node");
    assert_eq!(inserted.position, SemioPoint2 { x: 40.0, y: 51.0 });
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&flow::os_pack::json::to_json_string(&dsl::ToValue::to_value(inserted))).expect("the owned encoder must emit oracle-parsable JSON"),
        serde_json::json!({
            "id": "note_3",
            "kind": "inputNote",
            "label": "inputNote",
            "params": [{ "key": "text", "value": "" }],
            "position": { "x": 40.0, "y": 51.0 }
        })
    );
}

#[test]
fn child_add_widget_preserves_every_descriptor_payload_and_neuron_port_default() {
    use flow::neural::{ChannelSpec, OperatorInfo, VariadicSpec};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::apply_semio_flow_mutation;

    struct Case<'a> {
        descriptor: &'a str,
        info: Option<&'a OperatorInfo>,
        id: &'a str,
        kind: &'a str,
        label: &'a str,
        params: &'a [(&'a str, &'a str)],
    }

    let named_info = OperatorInfo { id: "owned.named".into(), inputs: vec![ChannelSpec::named("A", "A", "a", "A"), ChannelSpec::named("B", "B", "b", "B")], ..Default::default() };
    let variadic_info =
        OperatorInfo { id: "owned.variadic".into(), variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }), variadic_output: Some(VariadicSpec { slot_key: "value".into(), min: 1, max: None }), ..Default::default() };
    let cases = [
        Case {
            descriptor: r#"{"kind":"neuron","neuronKind":"owned.named"}"#,
            info: Some(&named_info),
            id: "owned_named_2",
            kind: "neuron",
            label: "neuron",
            params: &[("neuronKind", "owned.named"), ("params", "{}"), ("inputPorts", r#"["a","b"]"#), ("outputPorts", "[]"), ("preview", "true")],
        },
        Case {
            descriptor: r#"{"kind":"neuron","neuronKind":"owned.variadic"}"#,
            info: Some(&variadic_info),
            id: "owned_variadic_2",
            kind: "neuron",
            label: "neuron",
            params: &[("neuronKind", "owned.variadic"), ("params", "{}"), ("inputPorts", r#"["0","1"]"#), ("outputPorts", r#"["0"]"#), ("preview", "true")],
        },
        Case { descriptor: r#"{"kind":"inputSlider","label":"Gain"}"#, info: None, id: "slider_2", kind: "inputSlider", label: "Gain", params: &[("label", "Gain"), ("value", "3"), ("min", "0"), ("max", "10"), ("step", "0.1")] },
        Case { descriptor: r#"{"kind":"inputNote"}"#, info: None, id: "note_2", kind: "inputNote", label: "inputNote", params: &[("text", "")] },
        Case { descriptor: r#"{"kind":"inputImage"}"#, info: None, id: "image_2", kind: "inputImage", label: "inputImage", params: &[("src", "")] },
        Case { descriptor: r#"{"kind":"outputPreview"}"#, info: None, id: "preview_2", kind: "outputPreview", label: "outputPreview", params: &[("preview", "{}"), ("expanded", "[]")] },
        Case { descriptor: r#"{"kind":"outputAction"}"#, info: None, id: "action_2", kind: "outputAction", label: "outputAction", params: &[("action", "log")] },
        Case { descriptor: r#"{"kind":"outputExport"}"#, info: None, id: "export_2", kind: "outputExport", label: "outputExport", params: &[("format", "svg")] },
        Case { descriptor: r#"{"kind":"variable"}"#, info: None, id: "variable_2", kind: "variable", label: "variable", params: &[("name", "value"), ("schema", "dictionary")] },
    ];

    for case in cases {
        let serde_descriptor: serde_json::Value = serde_json::from_str(case.descriptor).expect("serde descriptor reference");
        assert_eq!(serde_descriptor["kind"], case.kind);
        let descriptor: semio_framework_artifact_flow_flow::WidgetDescriptor = flow::os_pack::json::from_json_str(case.descriptor).expect("typed descriptor");
        let mut content = SemioFlowSnapshot::default();
        let mutation = child_add_widget_mutation_from_descriptor(&content, &descriptor, case.info, 12.0, 34.0).expect("typed child mutation");
        apply_semio_flow_mutation(&mut content, &mutation);
        let inserted = content.nodes.last().expect("appended node");
        let params = case.params.iter().map(|(key, value)| serde_json::json!({ "key": key, "value": value })).collect::<Vec<_>>();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&flow::os_pack::json::to_json_string(&dsl::ToValue::to_value(inserted))).expect("the owned encoder must emit oracle-parsable JSON"),
            serde_json::json!({
                "id": case.id,
                "kind": case.kind,
                "label": case.label,
                "params": params,
                "position": { "x": 12.0, "y": 34.0 }
            }),
            "descriptor {}",
            case.descriptor
        );
    }
}

#[test]
fn child_add_widget_rejects_an_explicit_identity_collision() {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowNode;

    let content = SemioFlowSnapshot { nodes: vec![FlowNode { id: "taken".into(), kind: "inputNote".into(), label: "inputNote".into(), params: vec![], position: Default::default() }], ..Default::default() };
    let descriptor = flow::os_pack::json::from_json_str(r#"{"kind":"inputNote","id":"taken"}"#).expect("typed descriptor");
    let error = child_add_widget_mutation_from_descriptor(&content, &descriptor, None, 0.0, 0.0).expect_err("duplicate identity must fail");
    assert!(error.message.contains("widget id already exists: taken"), "{error:?}");
}

#[semio_framework_async_macros::async_test]
async fn add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::testkit::{close_registered_fixture_app, meta, settle_registered_typed_operation};
    use semio_framework_plugin::PluginApp;

    let mut app = flow_app().await;
    let parent_before = app.snapshot().expect("snapshot");
    let child_id = parent_before.content.child_id.clone();
    let content_before = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
    let result = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) })).await;
    assert!(result.mutations.is_empty(), "admission must retain the child publication");
    let first = settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("first child publication");
    let repeated = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(50.0), y: Some(51.0) })).await;
    assert!(repeated.mutations.is_empty(), "repeated admission must retain the child publication");
    let second = settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("repeated child publication");
    let parent_after = app.snapshot().expect("snapshot");
    let content_after = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
    for receipt in [first, second] {
        assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Terminal], "each command must publish exactly one acknowledged child group followed by terminal");
    }
    assert_eq!(parent_after.content, parent_before.content, "addWidget must preserve the exact parent content coordinate");
    assert_eq!(content_after.nodes.len(), content_before.nodes.len() + 2);
    let inserted = &content_after.nodes[content_before.nodes.len()..];
    assert_eq!((inserted[0].position.x, inserted[0].position.y), (40.0, 40.0));
    assert_eq!((inserted[1].position.x, inserted[1].position.y), (50.0, 51.0));
    assert_eq!((inserted[0].id.as_str(), inserted[1].id.as_str()), ("note_2", "note_3"));
    let mut after_first_undo = content_before.clone();
    after_first_undo.nodes.push(inserted[0].clone());
    for expected in [after_first_undo, content_before] {
        app.handle_action("undo", None, &meta("local")).await.expect("undo child group");
        let content = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child after undo").document_pack_bytes().await.expect("Flow child pack after undo")).expect("Flow child snapshot after undo");
        assert_eq!(content, expected, "each inverse must restore the complete preceding child document in reverse insertion order");
    }
    close_registered_fixture_app(&mut app);
    eprintln!("[DEBUG] two acknowledged Flow child groups preserve parent identity and undo to the original child document");
}

#[semio_framework_async_macros::async_test]
async fn rename_rejects_blank_unchanged_and_taken_ids() {
    let mut app = flow_app().await;
    for value in ["", " ", "slider"] {
        let result = dispatch(&mut app, FlowCommand::RenameFlowWidget(crate::editor::flow::commands::rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: value.into() })).await;
        assert!(result.mutations.is_empty(), "rename to {value:?} must be a no-operation");
    }
}

#[semio_framework_async_macros::async_test]
async fn patch_flow_widgets_parses_the_raw_value_string_into_the_slider() {
    let mut app = flow_app().await;
    dispatch(&mut app, FlowCommand::PatchFlowWidgets(crate::editor::flow::commands::patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["slider".into()], field: "value".into(), value: "7.5".into() })).await;
    let patched = app.snapshot().expect("snapshot");
    let patched_widgets = patched.to_fixture().widgets;
    assert!(
        patched_widgets.iter().any(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if id == "slider" && (value - 7.5).abs() < f64::EPSILON)),
        "slider must carry the parsed value: {patched_widgets:?}"
    );
}
