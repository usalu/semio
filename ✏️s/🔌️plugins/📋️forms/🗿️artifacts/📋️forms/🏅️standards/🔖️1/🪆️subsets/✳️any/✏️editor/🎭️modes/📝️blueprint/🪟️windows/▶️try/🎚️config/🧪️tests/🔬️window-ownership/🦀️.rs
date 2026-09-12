use super::*;
use crate::editor::forms::commands::{next_step, reset_try, set_try_value};
use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::{FormsTryWindowLease, FormsTryWindowTransient};
use crate::{forms_snapshot_with_state, FormStep, FORMS_DOCUMENT_SCHEMA};
use semio_framework::kernel::Effect;
use store::ArtifactPack;

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
            use crate::editor::forms::commands::{next_step, reset_try, set_contributions};
            use crate::editor::forms::{create_forms_app, FormsCommand, FormsPlayApp};
            use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::FormsTryWindowTransientOwner;
            use semio_framework_plugin::{artifact_app_laws, ActionMeta, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance, WindowConfigOwner};

            type FormsApp = VcsArtifactApp<EditorApp<FormsPlayApp>>;

            async fn dispatch(app: &mut FormsApp, command: FormsCommand, view: &ViewModel) -> Result<semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt, String> {
                let meta = ActionMeta { instance_id: 91, view_state: Some(view.clone()), ..artifact_app_laws::meta("forms-try-window-ownership") };
                app.dispatch_typed(command, &meta).await.map_err(|error| format!("{error:?}"))?;
                artifact_app_laws::settle_registered_typed_operation(app, meta.instance_id).await.map_err(|error| format!("{error:?}"))
            }

            fn continuation(receipt: &semio_framework_plugin::artifact_app_laws::TypedOperationFixtureReceipt) -> Result<set_try_value::SetTryValueStep, String> {
                let args = receipt.effects.iter().find_map(|effect| match effect {
                    Effect::DispatchAction { action, args: Some(args), .. } if action == set_try_value::SET_TRY_VALUE_STEP_ACTION_ID => Some(args.clone()),
                    _ => None,
                }).ok_or_else(|| "missing Forms Try continuation effect".to_string())?;
                dsl::FromValue::from_value(args).map_err(|error| error.to_string())
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
                ..Default::default()
            };
            let left = all.for_window_instance("forms-try-left").expect("left Forms Try window");
            let right = all.for_window_instance("forms-try-right").expect("right Forms Try window");
            let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<FormsPlayApp>>(manifest).await);
            app.bind_instance_id(91).await;
            let outcome: Result<(), String> = async {
                dispatch(
                    &mut app,
                    FormsCommand::SetContributions(set_contributions::SetContributions { json: "[{\"pluginId\":\"host\",\"topicContribution\":{}}]".into() }),
                    &left,
                )
                .await?;
                let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                let app_config_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;

                let advanced = dispatch(
                    &mut app,
                    FormsCommand::NextStep(next_step::NextStep { window_id: "forms-try-left".into(), window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into() }),
                    &left,
                )
                .await?;
                if advanced.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count() != 1 {
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
                let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<FormsPlayApp>>(manifest).await);
                reopened.bind_instance_id(92).await;
                for pack in packs { reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?; }
                let restored = artifact_app_laws::capture_fixture_window_config::<FormsTryWindowConfigOwner, _, _>(&mut *reopened, &left)
                    .await
                    .map_err(|error| format!("{error:?}"))?;
                if restored.as_ref() != Some(&left_config) { return Err("Forms exact Try config changed during reload".into()); }
                artifact_app_laws::close_registered_fixture_app(&mut *reopened);

                let left_start = dispatch(
                    &mut app,
                    FormsCommand::SetTryValue(set_try_value::SetTryValue {
                        key: "answer".into(),
                        value_json: Some("\"Ada\"".into()),
                        window_id: "forms-try-left".into(),
                        window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into(),
                        ..Default::default()
                    }),
                    &left,
                )
                .await?;
                let right_start = dispatch(
                    &mut app,
                    FormsCommand::SetTryValue(set_try_value::SetTryValue {
                        key: "answer".into(),
                        value_json: Some("\"Grace\"".into()),
                        window_id: "forms-try-right".into(),
                        window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into(),
                        ..Default::default()
                    }),
                    &right,
                )
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
                let left_commit = dispatch(&mut app, FormsCommand::SetTryValueStep(left_step), &left).await?;
                if left_commit.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient).count() != 1 {
                    return Err("Forms left Try continuation did not publish one transient lane".into());
                }
                if transient(&mut *app, &left)?.try_values.is_empty() || !transient(&mut *app, &right)?.try_values.is_empty() {
                    return Err("Forms staged value crossed exact Try windows".into());
                }

                let reset = dispatch(
                    &mut app,
                    FormsCommand::ResetTry(reset_try::ResetTry { window_id: "forms-try-left".into(), window_kind_id: FormsTryWindowConfigOwner::WINDOW_KIND_ID.into() }),
                    &left,
                )
                .await?;
                let reset_config_lanes = reset.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count();
                let reset_transient_lanes = reset.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowTransient).count();
                if (reset_config_lanes, reset_transient_lanes) != (1, 1) || !transient(&mut *app, &left)?.try_values.is_empty() || !transient(&mut *app, &right)?.try_values.is_empty() {
                    return Err("Forms reset did not clear only the exact left Try window".into());
                }
                dispatch(&mut app, FormsCommand::SetTryValueStep(right_step), &right).await?;
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
            if let Err(error) = &outcome { eprintln!("[DEBUG] Forms Try exact-window runtime failure before close: {error}"); }
            artifact_app_laws::close_registered_fixture_app(&mut *app);
            outcome.expect("Forms Try exact-window ownership runtime law");
            eprintln!("[DEBUG] Forms runtime isolated two Try windows, restored config, cleared reset transient, preserved the other continuation and document/app bytes, and closed terminal-empty");
        }))
        .expect("spawn Forms Try window ownership law")
        .join()
        .expect("Forms Try window ownership law thread");
}

fn operation() -> semio_framework_plugin::AppOperationContext {
    semio_framework_plugin::AppOperationContext { app_instance_id: 1, parent_document_id: "forms-document".into(), operation_id: 41, generation: 3, canonical_base_revision: [0; 32] }
}

fn lease(id: &str, generation: u64) -> FormsTryWindowLease {
    FormsTryWindowLease { window_id: id.into(), window_kind_id: super::super::FORMS_PLAY_WINDOW_TRY.into(), window_generation: generation, document_generation: 13 }
}

fn step(output: &set_try_value::TryWindowCommandOutput) -> set_try_value::SetTryValueStep {
    let Effect::DispatchAction { args: Some(args), .. } = &output.emit.effects[0] else { panic!("Forms Try continuation") };
    dsl::FromValue::from_value(args.clone()).expect("typed Forms Try continuation")
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
    let app_config = crate::editor::forms::config::FormsConfig { contributions_json: "[{\"pluginId\":\"host\"}]".into() };
    let app_config_reloaded = crate::editor::forms::config::FormsConfig::decode_pack(&app_config.encode_pack()).expect("Forms app config reload");
    assert_eq!(app_config_reloaded, app_config);

    let left_config = FormsTryWindowConfig::default();
    let right_config = FormsTryWindowConfig { current_step_index: 0 };
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
    let left_commit = set_try_value::advance_window(&step(&left_start), &operation, &left_transient, &left_lease).expect("commit left Try value");
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
    assert_eq!(right_config, FormsTryWindowConfig { current_step_index: 0 });
    assert!(right_transient.try_values.is_empty());
    assert_eq!(document.encode_pack(), document_before);

    let right_commit = set_try_value::advance_window(&step(&right_start), &operation, &right_transient, &right_lease).expect("left reset must not cancel right continuation");
    assert!(!right_commit.transient.expect("right transient publication").try_values.is_empty());
    assert_eq!(document.encode_pack(), document_before);
}

#[test]
fn forms_try_window_ownership_mutations_match_neutral_fixture_and_codecs() {
    use crate::editor::forms::modes::blueprint::windows::try_wizard::transient::FormsTryWindowTransientMutation;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window-ownership/🔣️.json")).expect("Forms Try ownership fixture JSON");
    let base_config: FormsTryWindowConfig = dsl::json::from_json_str(&fixture["windows"]["left"]["config"].to_string()).expect("Forms Try base config");
    let config_mutation: FormsTryWindowConfigMutation = dsl::json::from_json_str(&fixture["leftMutations"]["advance"].to_string()).expect("Forms Try config mutation");
    let after_config = config_mutation.diff(&base_config).diff().apply(&base_config).expect("apply Forms Try config mutation");
    let restored_config = config_mutation.inverse(&base_config).into_iter().fold(after_config, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("restore Forms Try config"));
    assert_eq!(restored_config, base_config);
    assert_eq!(FormsTryWindowConfigMutation::parse_op(&config_mutation.print_op()).expect("Forms Try config text codec"), config_mutation);
    assert_eq!(FormsTryWindowConfigMutation::decode_op(&config_mutation.encode_op().expect("Forms Try config binary encode")).expect("Forms Try config binary decode"), config_mutation);

    let base_transient: FormsTryWindowTransient = dsl::json::from_json_str(&fixture["windows"]["left"]["transient"].to_string()).expect("Forms Try base transient");
    let transient_mutation: FormsTryWindowTransientMutation = dsl::json::from_json_str(&fixture["leftMutations"]["stage"].to_string()).expect("Forms Try transient mutation");
    let after_transient = transient_mutation.diff(&base_transient).diff().apply(&base_transient).expect("apply Forms Try transient mutation");
    let restored_transient = transient_mutation.inverse(&base_transient).into_iter().fold(after_transient, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("restore Forms Try transient"));
    assert_eq!(restored_transient, base_transient);
    assert_eq!(FormsTryWindowTransientMutation::parse_op(&transient_mutation.print_op()).expect("Forms Try transient text codec"), transient_mutation);
    assert_eq!(FormsTryWindowTransientMutation::decode_op(&transient_mutation.encode_op().expect("Forms Try transient binary encode")).expect("Forms Try transient binary decode"), transient_mutation);
    eprintln!("[DEBUG] Forms Try config/transient mutations matched neutral fixture inverse, text, and binary laws");
}
