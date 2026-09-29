use super::*;
use protocol::Mutation;

#[test]
fn submission_routes_errors_and_prevents_duplicate_responses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📨️response/🧫️fixtures/🔣️submission.json")).unwrap();
    let definition: crate::schema::definition::FormsDefinition = dsl::json::from_json_str(&fixture["definition"].to_string()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut spec = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "submission".into(), "1".into(), None, &definition.steps);
        let chunks = case["values"].as_object().unwrap().iter().map(|(key, value)| (key.clone(), dsl::DslValue::Array(vec![dsl::DslValue::String(value.to_string())]))).collect();
        let transient = FormsTryWindowTransient { try_values: dsl::FromValue::from_value(dsl::DslValue::Object(chunks)).unwrap() };
        let config = FormsTryWindowConfig { current_step_index: 1, submitted_response_id: None };
        let (emit, next) = handle_window(&spec, &config, &transient, "revision-a".into()).unwrap();
        if !case["expected"]["errors"].as_array().unwrap().is_empty() {
            assert!(emit.artifact_mutations.is_empty());
            let id = case["expected"]["errors"][0]["questionId"].as_str().unwrap();
            let index = definition.steps.iter().position(|step| step.blocks.iter().any(|question| question.id == id)).unwrap();
            assert_eq!(next.current_step_index, index as u32);
            assert_eq!(next.submitted_response_id, None);
            continue;
        }
        assert_eq!(emit.artifact_mutations.len(), 1);
        emit.artifact_mutations[0].diff(&spec).apply_to(&mut spec);
        assert_eq!(spec.responses.len(), 1);
        assert_eq!(next.submitted_response_id.as_deref(), Some(spec.responses[0].id.as_str()));
        let answers: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&spec.responses[0].answers)).unwrap();
        assert_eq!(answers, case["expected"]["response"]["answers"]);
        let packed = <FormsTryWindowConfig as store::ArtifactPack>::encode_pack(&next);
        let restored = <FormsTryWindowConfig as store::ArtifactPack>::decode_pack(&packed).unwrap();
        assert_eq!(restored, next);
        assert!(handle_window(&spec, &restored, &transient, "revision-b".into()).unwrap().0.artifact_mutations.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn mounted_submission_persists_reopens_exports_and_discards() {
    use crate::editor::forms::{FormsCommand, modes::responses::results};
    use crate::editor::forms::commands::{set_spec_json, set_try_values, set_try_value, export_responses, discard_response};
    use crate::editor::forms::unit_tests::context::{forms_app, settle};
    use semio_framework_plugin::{artifact_app_laws, ActionMeta, PluginApp, ViewModel, ViewWindowInstance};
    use semio_framework::kernel::Effect;
    let mut app = forms_app().await;
    let outcome: Result<(), String> = async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️schema/📨️response/🧫️fixtures/🔣️submission.json")).map_err(|error| error.to_string())?;
        let definition: crate::schema::definition::FormsDefinition = dsl::json::from_json_str(&fixture["definition"].to_string()).map_err(|error| error.to_string())?;
        let specification = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "submission-test".into(), "1".into(), Some("Response Test".into()), &definition.steps);
        app.dispatch_typed(FormsCommand::SetSpecJson(set_spec_json::SetSpecJson { json: dsl::json::to_json_string(&specification) }), &artifact_app_laws::meta("local")).await.map_err(|error| format!("{error:?}"))?;
        settle(&mut app).await;
        let view = ViewModel { active_mode_id: Some(crate::editor::forms::modes::fill::MODE.into()), window_id: Some("responses-try".into()), window_instances: vec![ViewWindowInstance { id: "responses-try".into(), window_kind_id: crate::editor::forms::modes::blueprint::windows::try_wizard::FORMS_PLAY_WINDOW_TRY.into() }], tree_viewport_rows: Some(32), ..Default::default() };
        let kind = view.window_instances[0].window_kind_id.clone();
        let meta = ActionMeta { view_state: Some(view.clone()), ..artifact_app_laws::meta("local") };
        app.dispatch_typed(FormsCommand::SetTryValues(set_try_values::SetTryValues {
            values_json: fixture["cases"][0]["values"].to_string().into(), window_id: "responses-try".into(), window_kind_id: kind.clone(), ..Default::default()
        }), &meta).await.map_err(|error| format!("{error:?}"))?;
        let mut receipt = settle(&mut app).await;
        loop {
            let next = receipt.effects.iter().find_map(|effect| match effect { Effect::DispatchAction { action, args: Some(args), .. } if action == set_try_value::SET_TRY_VALUE_STEP_ACTION_ID => Some(args.clone()), _ => None });
            let Some(args) = next else { break; };
            let command = dsl::FromValue::from_value(args).map_err(|error| format!("{error:?}"))?;
            app.dispatch_typed(FormsCommand::SetTryValueStep(command), &meta).await.map_err(|error| format!("{error:?}"))?;
            receipt = settle(&mut app).await;
        }
        let submit = || FormsCommand::Submit(Submit { window_id: "responses-try".into(), window_kind_id: kind.clone() });
        app.dispatch_typed(submit(), &meta).await.map_err(|error| format!("{error:?}"))?;
        settle(&mut app).await;
        let snapshot = app.snapshot().map_err(|error| format!("{error:?}"))?;
        if snapshot.responses.len() != 1 { return Err(format!("expected one persisted response, got {}", snapshot.responses.len())); }
        let answers: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&snapshot.responses[0].answers)).map_err(|error| error.to_string())?;
        if answers != fixture["cases"][0]["expected"]["response"]["answers"] { return Err("persisted answers differ from shared submission fixture".into()); }
        app.dispatch_typed(submit(), &meta).await.map_err(|error| format!("{error:?}"))?;
        settle(&mut app).await;
        if app.snapshot().map_err(|error| format!("{error:?}"))?.responses.len() != 1 { return Err("repeat submit duplicated a response".into()); }
        let files = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
        let mut reopened = forms_app().await;
        let reopened_result = reopened.load_document_pack(&files).await.map_err(|error| format!("{error:?}"));
        if let Err(error) = reopened_result { reopened.close(); return Err(error); }
        let restored = reopened.snapshot().map_err(|error| format!("{error:?}"))?;
        reopened.close();
        if restored.responses != snapshot.responses { return Err("reopening lost a submitted response".into()); }
        let tree = app.render(results::BODY, None, &ViewModel { tree_viewport_rows: Some(32), ..Default::default() }).await.map_err(|error| format!("{error:?}"))?;
        let rendered = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(|error| format!("{error:?}"))?;
        if !rendered.contains("Ada") || !rendered.contains("exportResponses") { return Err("responses window did not display saved answers and export".into()); }
        for format in ["json", "csv"] {
            app.dispatch_typed(FormsCommand::ExportResponses(export_responses::ExportResponses { format: format.into() }), &meta).await.map_err(|error| format!("{error:?}"))?;
            let receipt = settle(&mut app).await;
            let data = receipt.effects.iter().find_map(|effect| match effect { Effect::DownloadMediaExport { data, .. } => Some(data), _ => None }).ok_or("missing response download")?;
            if format == "json" {
                let decoded: Vec<crate::schema::response::FormsResponse> = dsl::json::from_json_str(data).map_err(|error| format!("{error:?}"))?;
                if decoded != snapshot.responses { return Err("JSON download lost responses".into()); }
            } else {
                let rows: Vec<_> = csv::Reader::from_reader(data.as_bytes()).records().collect::<Result<_, _>>().map_err(|error| error.to_string())?;
                if rows.len() != snapshot.responses[0].answers.len() { return Err("CSV download lost answer rows".into()); }
            }
        }
        app.dispatch_typed(FormsCommand::DiscardResponse(discard_response::DiscardResponse { id: snapshot.responses[0].id.clone() }), &meta).await.map_err(|error| format!("{error:?}"))?;
        settle(&mut app).await;
        if !app.snapshot().map_err(|error| format!("{error:?}"))?.responses.is_empty() { return Err("discard response did not persist".into()); }
        Ok(())
    }.await;
    app.close();
    outcome.expect("mounted Forms response lifecycle");
}
