use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::forms::modes::blueprint::windows::try_wizard::config::*;
use semio_s_artifact_forms_forms_commands_try::editor::forms::commands::{next_step, reset_try, set_try_value};
use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::{FormsTryWindowLease, FormsTryWindowTransient};
use crate::{forms_snapshot_with_state, FORMS_DOCUMENT_SCHEMA};
use semio_s_artifact_forms_forms_commands::FormStep;
use semio_framework::kernel::Effect;
use pack::snapshot::ArtifactPack;

fn block_on_forms_try_window<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn forms_try_window_ownership_runtime_isolates_reload_reset_and_continuations() {
    std::thread::Builder::new()
        .name("forms-try-window-ownership-law".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| block_on_forms_try_window(async {
            use crate::editor::forms::commands::set_contributions;
            use semio_s_artifact_forms_forms_commands_try::editor::forms::commands::{next_step, reset_try};
            use crate::editor::forms::{create_forms_app, FormsCommand, FormsPlayApp};
            use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::FormsTryWindowTransientOwner;
            use semio_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

            type FormsApp = VcsArtifactApp<EditorApp<FormsPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;

            async fn dispatch_try_window(app: &mut FormsApp, command: FormsCommand, view: &ViewModel, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt, String> {
                let meta = ActionMeta { instance_id: 91, view_state: Some(view.clone()), ..artifact_app_laws::meta("forms-try-window-ownership") };
                semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &command, &meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await.map_err(|error| format!("{error:?}"))
            }

            fn continuation(receipt: &semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt) -> Result<set_try_value::SetTryValueStep, String> {
                let args = receipt.effects.iter().find_map(|effect| match effect {
                    Effect::DispatchAction { action, args: Some(args), .. } if action == set_try_value::SET_TRY_VALUE_STEP_ACTION_ID => Some(args.clone()),
                    _ => None,
                }).ok_or_else(|| "missing Forms Try continuation effect".to_string())?;
                semio_framework_value::FromValue::from_value(args).map_err(|error| error.to_string())
            }

            fn transient(app: &mut FormsApp, view: &ViewModel) -> Result<FormsTryWindowTransient, String> {
                app.window_transient_snapshot(view)
                    .map_err(|error| format!("{error:?}"))?
                    .and_then(|snapshot| snapshot.get::<FormsTryWindowTransientOwner>().cloned())
                    .ok_or_else(|| "missing Forms Try transient owner".to_string())
            }

            let manifest = || App { definition: create_forms_app(), examples: Vec::new() };
            let all = ViewModel {
                window_instances: ["forms-try-left", "forms-try-right"]
                    .into_iter()
                    .map(|id| ViewWindowInstance { id: id.into(), window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into() })
                    .collect(),
                ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
            };
            let left = all.for_window_instance("forms-try-left").expect("left Forms Try window");
            let right = all.for_window_instance("forms-try-right").expect("right Forms Try window");
            let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
            let mut app = Box::new(artifact_app_laws::new_app_with_registry_and_members::<EditorApp<FormsPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await);
            app.bind_instance_id(91).await;
            let outcome: Result<(), String> = async {
                dispatch_try_window(
                    &mut app,
                    FormsCommand::SetSpecJson(crate::editor::forms::commands::set_spec_json::SetSpecJson {
                        json: semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&document()))),
                    }),
                    &left, scope.cx())
                .await?;
                dispatch_try_window(
                    &mut app,
                    FormsCommand::SetContributions(set_contributions::SetContributions { json: "[{\"pluginId\":\"host\",\"topicContribution\":{}}]".into() }),
                    &left, scope.cx())
                .await?;
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let app_config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;

                let advanced = dispatch_try_window(
                    &mut app,
                    FormsCommand::NextStep(next_step::NextStep { window_id: "forms-try-left".into(), window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into() }),
                    &left, scope.cx())
                .await?;
                if advanced.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::WindowConfig).count() != 1 {
                    return Err("Forms NextStep did not publish one exact window config lane".into());
                }
                let left_config = artifact_app_laws::capture_fixture_window_config::<FormsTryWindowConfigOwner, _, _>(&mut *app, &left)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "missing left Forms Try config".to_string())?;
                let right_config = artifact_app_laws::capture_fixture_window_config::<FormsTryWindowConfigOwner, _, _>(&mut *app, &right)
                    .await
                    .map_err(|error| format!("{error:?}"))?
                    .ok_or_else(|| "missing right Forms Try config".to_string())?;
                if left_config.current_step_index != 1 || right_config != FormsTryWindowConfig::default() {
                    return Err(format!("Forms step navigation crossed exact windows: left={left_config:?}, right={right_config:?}"));
                }

                let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                let mut reopened = Box::new(artifact_app_laws::new_app_with_registry_and_members::<EditorApp<FormsPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await);
                reopened.bind_instance_id(92).await;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let restored = artifact_app_laws::capture_fixture_window_config::<FormsTryWindowConfigOwner, _, _>(&mut *reopened, &left)
                    .await
                    .map_err(|error| format!("{error:?}"))?;
                if restored.as_ref() != Some(&left_config) { return Err("Forms exact Try config changed during reload".into()); }
                artifact_app_laws::close_registered_fixture_app(&mut *reopened, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");

                let left_start = dispatch_try_window(
                    &mut app,
                    FormsCommand::SetTryValue(set_try_value::SetTryValue {
                        key: "answer".into(),
                        value_json: Some("\"Ada\"".into()),
                        window_id: "forms-try-left".into(),
                        window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into(),
                        ..Default::default()
                    }),
                    &left, scope.cx())
                .await?;
                let right_start = dispatch_try_window(
                    &mut app,
                    FormsCommand::SetTryValue(set_try_value::SetTryValue {
                        key: "answer".into(),
                        value_json: Some("\"Grace\"".into()),
                        window_id: "forms-try-right".into(),
                        window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into(),
                        ..Default::default()
                    }),
                    &right, scope.cx())
                .await?;
                let left_step = continuation(&left_start)?;
                let right_step = continuation(&right_start)?;
                if left_step.window_id != "forms-try-left"
                    || right_step.window_id != "forms-try-right"
                    || left_step.window_kind_id != FormsTryWindowConfigOwner::WINDOW_KIND_ID
                    || right_step.window_kind_id != FormsTryWindowConfigOwner::WINDOW_KIND_ID
                    || left_step.operation_id == right_step.operation_id
                    || left_step.document_generation != right_step.document_generation
                {
                    return Err("Forms continuation leases lost their exact-window identity".into());
                }
                let left_commit = dispatch_try_window(&mut app, FormsCommand::SetTryValueStep(left_step), &left, scope.cx()).await?;
                if left_commit.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::WindowTransient).count() != 1 {
                    return Err("Forms left Try continuation did not publish one transient lane".into());
                }
                if transient(&mut *app, &left)?.try_values.is_empty() || !transient(&mut *app, &right)?.try_values.is_empty() {
                    return Err("Forms staged value crossed exact Try windows".into());
                }

                let reset = dispatch_try_window(
                    &mut app,
                    FormsCommand::ResetTry(reset_try::ResetTry { window_id: "forms-try-left".into(), window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into() }),
                    &left, scope.cx())
                .await?;
                let reset_config_lanes = reset.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::WindowConfig).count();
                let reset_transient_lanes = reset.lanes.iter().filter(|lane| **lane == semio_plugin::app::TypedOperationResultLane::WindowTransient).count();
                if (reset_config_lanes, reset_transient_lanes) != (1, 1) || !transient(&mut *app, &left)?.try_values.is_empty() || !transient(&mut *app, &right)?.try_values.is_empty() {
                    return Err("Forms reset did not clear only the exact left Try window".into());
                }
                dispatch_try_window(&mut app, FormsCommand::SetTryValueStep(right_step), &right, scope.cx()).await?;
                if !transient(&mut *app, &left)?.try_values.is_empty() || transient(&mut *app, &right)?.try_values.is_empty() {
                    return Err("Forms left reset cancelled or contaminated the right continuation".into());
                }

                let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let app_config_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                if document_before.pack != document_after.pack || document_before.spr != document_after.spr {
                    return Err("Forms Try window publication changed document bytes".into());
                }
                if app_config_before.pack != app_config_after.pack || app_config_before.spr != app_config_after.spr {
                    return Err("Forms Try window publication changed host contribution projection".into());
                }
                Ok(())
            }
            .await;
            if let Err(error) = &outcome { eprintln!("[TRACE] Forms Try exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
            outcome.expect("Forms Try exact-window ownership runtime law");
        }))
        .expect("spawn Forms Try window ownership law")
        .join()
        .expect("Forms Try window ownership law thread");
}

fn operation() -> semio_plugin::AppOperationContext {
    semio_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "forms-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [0; 32], authoring_seed: "authoring-seed-test".into() }
}

fn lease(id: &str, generation: u64) -> FormsTryWindowLease {
    FormsTryWindowLease { window_id: id.into(), window_kind_id: crate::editor::forms::support::FORMS_PLAY_WINDOW_TRY.into(), window_generation: generation, document_generation: 13 }
}

fn step(output: &set_try_value::TryWindowCommandOutput) -> set_try_value::SetTryValueStep {
    let Effect::DispatchAction { args: Some(args), .. } = &output.emit.effects[0] else { panic!("Forms Try continuation") };
    semio_framework_value::FromValue::from_value(args.clone()).expect("typed Forms Try continuation")
}

fn document() -> crate::FormsSnapshot {
    forms_snapshot_with_state(
        FORMS_DOCUMENT_SCHEMA.into(),
        "forms-document".into(),
        "1".into(),
        None,
        &[
            FormStep { id: "first".into(), title: "First".into(), description: None, blocks: Vec::new() },
            FormStep { id: "second".into(), title: "Second".into(), description: None, blocks: Vec::new() },
        ],
    )
}

#[test]
fn forms_try_window_ownership_keeps_same_kind_windows_and_document_bytes_isolated() {
    let operation = operation();
    let left_lease = lease("forms-try-left", 7);
    let right_lease = lease("forms-try-right", 11);
    let document = document();
    let document_before = document.encode_pack();
    let app_config = crate::forms_config::FormsConfig { contributions_json: "[{\"pluginId\":\"host\"}]".into() };
    let app_config_reloaded = crate::forms_config::FormsConfig::decode_pack(&app_config.encode_pack()).expect("Forms app config reload");
    assert_eq!(app_config_reloaded, app_config);

    let left_config = FormsTryWindowConfig::default();
    let right_config = FormsTryWindowConfig::default();
    let left_transient = FormsTryWindowTransient::default();
    let right_transient = FormsTryWindowTransient::default();
    let advanced = next_step::handle_window(
        &next_step::NextStep { window_id: left_lease.window_id.clone(), window_kind_id: left_lease.window_kind_id.clone() },
        &document,
        &left_config,
        &left_transient,
    )
    .expect("advance exact left Try window");
    assert_eq!(advanced.current_step_index, 1);
    let reloaded = FormsTryWindowConfig::decode_pack(&advanced.encode_pack()).expect("Forms Try window config reload");
    assert_eq!(reloaded, advanced);

    let left_start = set_try_value::start_window(
        &set_try_value::SetTryValue {
            key: "answer".into(),
            value_json: Some("\"Ada\"".into()),
            window_id: left_lease.window_id.clone(),
            window_kind_id: left_lease.window_kind_id.clone(),
            ..Default::default()
        },
        &operation,
        &left_transient,
        &left_lease,
    )
    .expect("stage left Try value");
    let right_start = set_try_value::start_window(
        &set_try_value::SetTryValue {
            key: "answer".into(),
            value_json: Some("\"Grace\"".into()),
            window_id: right_lease.window_id.clone(),
            window_kind_id: right_lease.window_kind_id.clone(),
            ..Default::default()
        },
        &operation,
        &right_transient,
        &right_lease,
    )
    .expect("stage right Try value under the same app, document, and operation");
    let left_commit = set_try_value::advance_window(&document, &step(&left_start), &operation, &left_transient, &left_lease).expect("commit left Try value");
    let left_transient = left_commit.transient.expect("left transient publication");
    assert!(!left_transient.try_values.is_empty());
    assert!(right_transient.try_values.is_empty());

    let (reset, reset_config) = reset_try::handle_window(
        &reset_try::ResetTry { window_id: left_lease.window_id.clone(), window_kind_id: left_lease.window_kind_id.clone() },
        &operation,
        &left_lease,
    )
    .expect("reset left Try window");
    assert_eq!(reset_config, FormsTryWindowConfig::default());
    assert!(reset.transient.expect("left reset transient").try_values.is_empty());
    assert_eq!(right_config, FormsTryWindowConfig::default());
    assert!(right_transient.try_values.is_empty());
    assert_eq!(document.encode_pack(), document_before);

    let right_commit = set_try_value::advance_window(&document, &step(&right_start), &operation, &right_transient, &right_lease).expect("left reset must not cancel right continuation");
    assert!(!right_commit.transient.expect("right transient publication").try_values.is_empty());
    assert_eq!(document.encode_pack(), document_before);
}

#[test]
fn forms_try_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::FormsTryWindowTransientMutation;
    use protocol::{ColdMutation, Mutation, OpBinary, OpText};

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🔬️window/🔣️.json")).expect("Forms Try ownership fixture JSON");
    let base_config: FormsTryWindowConfig = semio_framework_pack_json::from_json_str(&fixture["windows"]["left"]["config"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Forms Try base config");
    let config_mutation: FormsTryWindowConfigMutation = semio_framework_pack_json::from_json_str(&fixture["leftMutations"]["advance"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Forms Try config mutation");
    let after_config = protocol::apply_diff_cold(config_mutation.diff(&base_config).diff(), &base_config).expect("apply Forms Try config mutation");
    let restored_config = config_mutation.inverse(&base_config).expect("valid retained mutation inverse fixture").into_iter().fold(after_config, |state, inverse| protocol::apply_diff_cold(inverse.diff(&state).diff(), &state).expect("restore Forms Try config"));
    assert_eq!(restored_config, base_config);
    assert_eq!(FormsTryWindowConfigMutation::parse_op(&config_mutation.print_op()).expect("Forms Try config text codec"), config_mutation);
    assert_eq!(FormsTryWindowConfigMutation::decode_op(&config_mutation.encode_op().expect("Forms Try config binary encode")).expect("Forms Try config binary decode"), config_mutation);

    let base_transient: FormsTryWindowTransient = semio_framework_pack_json::from_json_str(&fixture["windows"]["left"]["transient"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Forms Try base transient");
    let transient_mutation: FormsTryWindowTransientMutation = semio_framework_pack_json::from_json_str(&fixture["leftMutations"]["stage"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("Forms Try transient mutation");
    let after_transient = protocol::apply_diff_cold(transient_mutation.diff(&base_transient).diff(), &base_transient).expect("apply Forms Try transient mutation");
    let restored_transient = transient_mutation.inverse(&base_transient).expect("valid retained mutation inverse fixture").into_iter().fold(after_transient, |state, inverse| protocol::apply_diff_cold(inverse.diff(&state).diff(), &state).expect("restore Forms Try transient"));
    assert_eq!(restored_transient, base_transient);
    assert_eq!(FormsTryWindowTransientMutation::parse_op(&transient_mutation.print_op()).expect("Forms Try transient text codec"), transient_mutation);
    assert_eq!(FormsTryWindowTransientMutation::decode_op(&transient_mutation.encode_op().expect("Forms Try transient binary encode")).expect("Forms Try transient binary decode"), transient_mutation);
}

#[test]
fn forms_try_window_parameter_defaults_match_shared_vectors_and_submit() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🎮️commands/🎯️set-try-value/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let question = semio_framework_pack_json::from_json_str(&case["question"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let document = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms-document".into(), "1".into(), None, &[FormStep { id: "s".into(), title: "Parameters".into(), description: None, blocks: vec![question] }]);
        let mut transient = FormsTryWindowTransient::default();
        if let Some(before) = case.get("before") {
            let raw = before.to_string();
            let chunks = vec![semio_framework_value::SharedUtf8::from(raw)];
            transient.try_values = transient.try_values.with_chunks("building", crate::editor::forms::modes::blueprint::windows::try_wizard::transient::try_value_content_id(&chunks), std::sync::Arc::new(chunks));
        }
        let operation = operation();
        let lease = lease("forms-try-parameters", 1);
        let payload = set_try_value::SetTryValue { key: "building".into(), value_json: Some(case["value"].to_string().into()), param_key: Some(case["paramKey"].as_str().unwrap().into()), initial_value_json_chunks: case.get("initialValueJson").and_then(|value| value.as_str()).map(|value| vec![value.to_owned()]), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
        let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
        let output = set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).unwrap();
        let answered = output.transient.unwrap();
        let raw: String = answered.try_values.iter_chunks().into_iter().find(|(key, _)| key == "building").unwrap().1.iter().map(|chunk| chunk.as_str()).collect();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&raw).unwrap(), case["expected"], "{}", case["name"]);
        let values = semio_framework_pack_json::from_json_str(&format!("{{\"building\":{raw}}}"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let response = semio_s_artifact_forms_forms_commands::schema::response::prepare_response(&document.definition, &values, "response".into(), 0, "v".into()).unwrap();
        let json = semio_framework_pack_json::to_json_string(&response);
        let actual: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(actual["answers"][0]["value"], case["expected"], "{}", case["name"]);
    }
}

#[test]
fn forms_try_window_parameter_defaults_obey_complete_answer_byte_limit() {
    let raw = serde_json::json!({"id":"building","kind":"cad.parameter","label":"Building","params":{"width":2,"padding":"ä".repeat(8192)}});
    let question = semio_framework_pack_json::from_json_str(&raw.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let document = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms-document".into(), "1".into(), None, &[FormStep { id: "s".into(), title: "Parameters".into(), description: None, blocks: vec![question] }]);
    let transient = FormsTryWindowTransient::default();
    let operation = operation();
    let lease = lease("forms-try-large-parameters", 1);
    let payload = set_try_value::SetTryValue { key: "building".into(), value_json: Some("5".into()), param_key: Some("width".into()), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
    let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
    assert!(set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).is_err());
    assert!(transient.try_values.is_empty());
    let payload = set_try_value::SetTryValue { initial_value_json_chunks: Some(vec!["{\"width\":2}".into()]), ..payload };
    let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
    let output = set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).unwrap();
    let answer = output.transient.unwrap();
    let raw: String = answer.try_values.iter_chunks().into_iter().find(|(key, _)| key == "building").unwrap().1.iter().map(|chunk| chunk.as_str()).collect();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&raw).unwrap(), serde_json::json!({"width":5}));
}

#[test]
fn forms_try_window_parameter_initial_value_refusals_match_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🎮️commands/🎯️set-try-value/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let question = semio_framework_pack_json::from_json_str(&fixture["cases"][0]["question"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let document = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms-document".into(), "1".into(), None, &[FormStep { id: "s".into(), title: "Parameters".into(), description: None, blocks: vec![question] }]);
    for case in fixture["invalid"].as_array().unwrap() {
        let transient = FormsTryWindowTransient::default();
        let operation = operation();
        let lease = lease("forms-try-initial-refusal", 1);
        let payload = set_try_value::SetTryValue { key: "building".into(), value_json: Some("5".into()), param_key: Some("width".into()), initial_value_json_chunks: Some(vec![case["initialValueJson"].as_str().unwrap().into()]), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
        let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
        assert!(set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).is_err(), "{}", case["name"]);
        assert!(transient.try_values.is_empty());
    }
}

#[test]
fn forms_try_window_complete_text_chunks_match_neutral_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🎮️commands/🎯️set-try-value/🧪️tests/🧫️fixtures/🔣️text-source.json")).unwrap();
    let question = crate::editor::forms::questions::question_shell("text".into(), "Text".into(), "longText".into());
    let document = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms-document".into(), "1".into(), None, &[FormStep { id: "s".into(), title: "Text".into(), description: None, blocks: vec![question] }]);
    for row in fixture["cases"].as_array().unwrap() {
        let text = row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let raw = serde_json::to_string(&text).unwrap();
        assert!(raw.len() > set_try_value::MAX_TRY_VALUE_BYTES_PER_STEP);
        let encoded = semio_ui_contract::UiValue::try_text_chunks(&raw, row["maximumBytes"].as_u64().unwrap() as usize).unwrap();
        let chunks: Vec<String> = serde_json::from_value(serde_json::to_value(&encoded).unwrap()).unwrap();
        let transient = FormsTryWindowTransient::default();
        let operation = operation();
        let lease = lease("forms-try-complete-text", 1);
        let payload = set_try_value::SetTryValue { key: "text".into(), value_json_chunks: Some(chunks), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
        let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
        let output = set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).unwrap();
        let answer = output.transient.unwrap();
        let actual: String = answer.try_values.iter_chunks().into_iter().find(|(key, _)| key == "text").unwrap().1.iter().map(|chunk| chunk.as_str()).collect();
        assert_eq!(serde_json::from_str::<String>(&actual).unwrap(), text, "{}", row["name"]);
        let values = semio_framework_pack_json::from_json_str(&format!("{{\"text\":{actual}}}"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let response = semio_s_artifact_forms_forms_commands::schema::response::prepare_response(&document.definition, &values, "response".into(), 0, "v".into()).unwrap();
        let independent: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&response)).unwrap();
        assert_eq!(independent["answers"][0]["value"].as_str().unwrap(), text);
    }
    for row in fixture["refusals"].as_array().unwrap() {
        let text = row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let raw = serde_json::to_string(&text).unwrap();
        let encoded = semio_ui_contract::UiValue::try_text_chunks(&raw, semio_ui_contract::UI_TEXT_MAX_BYTES * semio_ui_contract::UI_VALUE_MAX_ITEMS).unwrap();
        let chunks: Vec<String> = serde_json::from_value(serde_json::to_value(&encoded).unwrap()).unwrap();
        let transient = FormsTryWindowTransient::default();
        let operation = operation();
        let lease = lease("forms-try-large-source", 1);
        let payload = set_try_value::SetTryValue { key: "text".into(), value_json_chunks: Some(chunks), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
        assert!(set_try_value::start_window(&payload, &operation, &transient, &lease).is_err(), "{}", row["name"]);
        assert!(transient.try_values.is_empty());
    }
    for row in fixture["invalidJson"].as_array().unwrap() {
        let raw = row["source"].as_str().unwrap();
        assert!(serde_json::from_str::<serde_json::Value>(raw).is_err());
        let transient = FormsTryWindowTransient::default();
        let operation = operation();
        let lease = lease("forms-try-invalid-source", 1);
        let payload = set_try_value::SetTryValue { key: "text".into(), value_json_chunks: Some(vec![raw.into()]), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
        let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
        assert!(set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).is_err(), "{}", row["name"]);
        assert!(transient.try_values.is_empty());
    }


}

#[test]
fn forms_try_window_partial_edits_preserve_displayed_defaults_and_submit() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🎮️commands/🎯️set-try-value/🧪️tests/🧫️fixtures/🔣️partial-answer.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let key = case["question"]["id"].as_str().unwrap();
        let question = semio_framework_pack_json::from_json_str(&case["question"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let document = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms-document".into(), "1".into(), None, &[FormStep { id: "s".into(), title: "Controls".into(), description: None, blocks: vec![question] }]);
        let mut transient = FormsTryWindowTransient::default();
        if let Some(before) = case.get("before") {
            let chunks = vec![semio_framework_value::SharedUtf8::from(before.to_string())];
            transient.try_values = transient.try_values.with_chunks(key, crate::editor::forms::modes::blueprint::windows::try_wizard::transient::try_value_content_id(&chunks), std::sync::Arc::new(chunks));
        }
        let operation = operation();
        let lease = lease("forms-try-partial-defaults", 1);
        let payload = set_try_value::SetTryValue {
            key: key.into(), value_json: Some(case["value"].to_string().into()),
            vector_index: case["change"].get("vectorIndex").and_then(serde_json::Value::as_u64),
            option_value: case["change"].get("optionValue").and_then(serde_json::Value::as_str).map(str::to_owned),
            window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default()
        };
        let start = set_try_value::start_window(&payload, &operation, &transient, &lease).unwrap();
        let output = set_try_value::advance_window(&document, &step(&start), &operation, &transient, &lease).unwrap();
        let answered = output.transient.unwrap();
        let raw: String = answered.try_values.iter_chunks().into_iter().find(|(name, _)| name == key).unwrap().1.iter().map(|chunk| chunk.as_str()).collect();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&raw).unwrap(), case["expected"], "{}", case["name"]);
        let values = semio_framework_pack_json::from_json_str(&format!("{{{}:{raw}}}", serde_json::to_string(key).unwrap()), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let response = semio_s_artifact_forms_forms_commands::schema::response::prepare_response(&document.definition, &values, "response".into(), 0, "v".into()).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&response)).unwrap();
        assert_eq!(actual["answers"][0]["value"], case["expected"], "{}", case["name"]);
    }
}

#[test]
fn forms_try_window_ambiguous_partial_edits_are_refused_before_staging() {
    let transient = FormsTryWindowTransient::default();
    let operation = operation();
    let lease = lease("forms-try-mixed-partial", 1);
    let payload = set_try_value::SetTryValue { key: "vector".into(), value_json: Some("9".into()), vector_index: Some(1), option_value: Some("a".into()), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
    assert!(set_try_value::start_window(&payload, &operation, &transient, &lease).is_err());
    assert!(transient.try_values.is_empty());
}

#[test]
fn forms_try_window_target_metadata_matches_shared_byte_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🎮️commands/🎯️set-try-value/🧪️tests/🧫️fixtures/🔣️target.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let value = row["fragment"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let mut payload = set_try_value::SetTryValue { key: "answer".into(), value_json: Some("1".into()), window_id: "left".into(), window_kind_id: "forms-try".into(), ..Default::default() };
        match row["field"].as_str().unwrap() {
            "key" => payload.key = value,
            "windowId" => payload.window_id = value,
            "windowKindId" => payload.window_kind_id = value,
            "inputId" => payload.input_id = Some(value),
            "paramKey" => payload.param_key = Some(value),
            "optionValue" => payload.option_value = Some(value),
            _ => panic!("undeclared target field"),
        }
        let lease = lease(&payload.window_id, 1);
        let operation = operation();
        let result = set_try_value::start_window(&payload, &operation, &FormsTryWindowTransient::default(), &lease);
        assert_eq!(result.is_ok(), row["valid"].as_bool().unwrap(), "{}", row["name"]);
        set_try_value::cancel_pending_generations(&operation, &lease);
    }
}

#[test]
fn forms_try_window_near_bound_answer_sources_keep_complete_chunks() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../../🎮️commands/🎯️set-try-value/🧪️tests/🧫️fixtures/🔣️target.json")).unwrap();
    for row in fixture["paged"].as_array().unwrap() {
        let fragment = row["fragment"].as_str().unwrap();
        let answer = serde_json::to_string(&fragment.repeat(row["answerRepeat"].as_u64().unwrap() as usize)).unwrap();
        let baseline = serde_json::json!({"note":fragment.repeat(row["baselineRepeat"].as_u64().unwrap() as usize)}).to_string();
        let chunks = |source: &str| -> Vec<String> { let value = semio_ui_contract::UiValue::try_text_chunks(source, 16_384).unwrap(); serde_json::from_value(serde_json::to_value(&value).unwrap()).unwrap() };
        let answer_chunks = chunks(&answer);
        let baseline_chunks = chunks(&baseline);
        assert_eq!(semio_ui_contract::UiValue::join_text_chunks(answer_chunks.iter().map(String::as_str), 16_384).unwrap(), answer);
        assert_eq!(semio_ui_contract::UiValue::join_text_chunks(baseline_chunks.iter().map(String::as_str), 16_384).unwrap(), baseline);
        let operation = operation();
        let lease = lease("near-bound-answer", 1);
        let payload = set_try_value::SetTryValue { key: "answer".into(), value_json_chunks: Some(answer_chunks), initial_value_json_chunks: Some(baseline_chunks), window_id: lease.window_id.clone(), window_kind_id: lease.window_kind_id.clone(), ..Default::default() };
        assert!(set_try_value::start_window(&payload, &operation, &FormsTryWindowTransient::default(), &lease).is_ok(), "{}", row["name"]);
        set_try_value::cancel_pending_generations(&operation, &lease);
    }
}
