/// 🔌️ Port laws of the generation2d editor routes: the export and import doors, the params-in patching and the declared media ports.

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn export_drawing_out_returns_vector_media() {
    let mut app = app().await;
    let media = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::export_fixture_media(&mut app.0, "drawing:out", app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).expect("export drawing:out");
    close(app);
    assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
}

#[semio_framework_async_macros::async_test]
async fn export_document_out_returns_flow_media() {
    let mut app = app().await;
    let media = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::export_fixture_media(&mut app.0, "artifact:out", app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).expect("export document:out");
    close(app);
    assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Flow });
    assert!(matches!(media.payload, semio_framework_plugin::MediaPayload::Structured { schema, .. } if schema == GENERATION_2D_SCHEMA));
}

#[semio_framework_async_macros::async_test]
async fn import_params_in_patches_matching_input_slider() {
    let mut app = app().await;
    crate::app_fixture::dispatch(&mut app, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, format: None, action: None, x: None, y: None })).await;
    let slider_id = snapshot_read(&app)
        .host_snapshot
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::InputSlider { id, .. } => Some(id.clone()),
            _ => None,
        })
        .expect("just-added input slider");
    let media = semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ slider_id.clone(): 42.0 }).to_string() },
    };
    app.0.import_media("params:in", media, &semio_framework_plugin::artifact_app_laws::meta("local"), app.1.cx()).await.expect("import params");
    let value = snapshot_read(&app).host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value),
        _ => None,
    });
    close(app);
    assert_eq!(value, Some(42.0));
}

#[semio_framework_async_macros::async_test]
async fn import_intrinsic_params_borrows_the_canonical_numeric_rows_and_retires_source() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📥️media/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let oracle = case["input"].as_object().unwrap().get("slider").and_then(serde_json::Value::as_f64).unwrap_or(fixture["initialValue"].as_f64().unwrap());
        assert_eq!(oracle, case["expected"].as_f64().unwrap());
        let mut app = app().await;
        crate::app_fixture::dispatch(&mut app, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, format: None, action: None, x: None, y: None })).await;
        let slider_id = snapshot_read(&app).host_snapshot.widgets.iter().rev().find_map(|widget| match widget { Widget::InputSlider { id, .. } => Some(id.clone()), _ => None }).unwrap();
        assert!(snapshot_read(&app).host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::InputSlider { id, value, .. } if id == &slider_id && *value == fixture["initialValue"].as_f64().unwrap())));
        let source = semio_framework_value::DslValue::Object(case["input"].as_object().unwrap().iter().map(|(key, value)| (if key == "slider" { slider_id.clone() } else { key.clone() }, semio_framework_value::DslValue::from(value))).collect());
        let media = semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: semio_framework_plugin::MediaPayload::Intrinsic { schema: fixture["schema"].as_str().unwrap().into(), value: source } };
        let result = app.0.import_media(fixture["port"].as_str().unwrap(), media, &semio_framework_plugin::artifact_app_laws::meta("local"), app.1.cx()).await;
        let value = snapshot_read(&app).host_snapshot.widgets.iter().find_map(|widget| match widget { Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value), _ => None });
        close(app);
        assert!(result.is_ok(), "{}: {result:?}", case["id"]);
        assert_eq!(value, Some(oracle), "{}", case["id"]);
    }
}

/// 🔐️ LAW: the app-owned resumable importer is the ONLY inbound media route, and it fails closed
/// on every payload it does not own — an unknown port, a non-structured payload and a non-object
/// JSON root all reject without publishing an operation.
#[semio_framework_async_macros::async_test]
async fn import_media_fails_closed_off_its_own_params_in_contract() {
    let structured = |json: &str| semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: json.into() },
    };
    for (port, media) in [
        ("params:out", structured("{}")),
        ("params:in", structured("[1, 2]")),
        ("params:in", structured("not json")),
        ("params:in", semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: semio_framework_plugin::MediaPayload::Intrinsic { schema: "params".into(), value: semio_framework_value::DslValue::String("invalid object".into()) } }),
        (
            "params:in",
            semio_framework_plugin::Media {
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                payload: semio_framework_plugin::MediaPayload::Binary { format_kind: "bin".into(), blob_hash: String::new() },
            },
        ),
    ] {
        let mut app = app().await;
        let before = snapshot_read(&app).host_snapshot.widgets.len();
        let rejected = app.0.import_media(port, media, &semio_framework_plugin::artifact_app_laws::meta("local"), app.1.cx()).await;
        let after = snapshot_read(&app).host_snapshot.widgets.len();
        close(app);
        assert!(rejected.is_err(), "generation2d import must reject '{port}' instead of publishing");
        assert_eq!(after, before, "a rejected import must leave the document untouched");
    }
}

/// 🔐️ LAW: an import whose keys match no `InputSlider` publishes nothing and leaves every widget
/// exactly as it was — unmatched ids and non-numeric values are skipped, never faulted.
#[semio_framework_async_macros::async_test]
async fn import_params_skips_unmatched_keys_and_non_numeric_values() {
    let mut app = app().await;
    let before: Vec<String> = snapshot_read(&app).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    let media = semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ "no-such-widget": 1.0, "another": "text" }).to_string() },
    };
    app.0.import_media("params:in", media, &semio_framework_plugin::artifact_app_laws::meta("local"), app.1.cx()).await.expect("import params");
    let after: Vec<String> = snapshot_read(&app).host_snapshot.widgets.iter().map(|widget| semio_s_artifact_procedural_2d_model::widget_id(widget).to_string()).collect();
    close(app);
    assert_eq!(after, before);
}

#[semio_framework_async_macros::async_test]
async fn media_ports_declare_params_in_and_drawing_out() {
    let ports = <Generation2dPlayApp as ArtifactEditor>::media_ports().await;
    assert!(ports.iter().any(|port| port.id == "artifact:in"));
    assert!(ports.iter().any(|port| port.id == "artifact:out"));
    let params_in = ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert_eq!(params_in.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
    let drawing_out = ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
    assert_eq!(drawing_out.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
    assert_eq!(drawing_out.kind_id.as_deref(), Some("2d.drawing"));
}

#[test]
fn generation2d_io_declares_the_params_and_drawing_ports() {
    let io = ::semio_framework_async::poll::resolve_ready(generation2d_io());
    assert_eq!(io.artifact_schema, "generation.2d");
    let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert!(!params.required);
    let drawing = io.ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
    assert_eq!(drawing.kind_id.as_deref(), Some("2d.drawing"));
}
//#endregion 🔖️PortTests
