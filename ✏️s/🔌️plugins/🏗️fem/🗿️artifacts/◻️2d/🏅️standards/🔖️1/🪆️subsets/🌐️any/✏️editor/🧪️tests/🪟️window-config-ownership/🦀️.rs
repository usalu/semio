use semio_s_artifact_fem_2d_editor::editor::fem2d::*;
use semio_s_fem_app_surface::ResultDisplay;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_area_load;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_bar;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_beam;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_combination;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_material;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_member_udl;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_nodal_load;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_node;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_region;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_section;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_support;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_down;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_move;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::canvas_pointer_up;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::focus_entity;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_combination;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_element;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_load;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_material;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_node;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_region;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_section;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_support;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::remove_selection;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::result_animation_tick;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_analysis_settings;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_camera;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_animation;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_display;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_self_weight;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::rotate_selection;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::scale_selection;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::set_transform_gumball_flag;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::translate_selection;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_interaction_definition;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::artifact as artifact_panel;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::inspection as inspection_panel;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::results as results_panel;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_labels;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results::results_window_instance_id;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_s_artifact_fem_2d_editor::editor::fem2d::modes::edit;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
use semio_s_fem_model::Dof;
use semio_s_fem_model::ElementResult;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_pack_json::Value;
use semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::reset_document_effect;
use semio_framework::InteractiveJobClassification;
use semio_framework::ToolExecutionContract;
use semio_framework::ToolFactoryKey;
use semio_framework::ToolJobFactory;
use semio_framework::ToolJobFactoryError;
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::built_text_node;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigSpec;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;

#[test]
fn fem2d_window_config_document_admission_rejects_window_and_os_fields() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🧪️tests/🧫️fixtures/🧬️document-admission/🔣️.json")).expect("document admission fixture");
    let row = fixture["cases"].as_array().expect("document cases").iter().find(|row| row["dimension"] == "2d").expect("dimension");
    let base = row["document"].to_string();
    let _ = semio_framework_pack_json::from_json_str::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::Fem2dArtifact>(&base, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral FEM artifact admission");
    let _ = semio_framework_pack_json::from_json_str::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot>(&base, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral FEM snapshot admission");
    for field in row["foreignFields"].as_array().expect("foreign fields") {
        let key = field["key"].as_str().expect("field key");
        let mut candidate = row["document"].clone();
        candidate[key] = field["value"].clone();
        let text = candidate.to_string();
        assert!(semio_framework_pack_json::from_json_str::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::Fem2dArtifact>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "artifact admitted {key}");
        assert!(semio_framework_pack_json::from_json_str::<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "snapshot admitted {key}");
    }
}

fn block_on_fem_window_ownership<F: std::future::Future>(mut future: std::pin::Pin<Box<F>>) -> F::Output {
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
fn fem2d_window_config_runtime_isolates_same_kind_instances_and_restores_packs() {
    std::thread::Builder::new()
        .name("fem2d-window-ownership-law".into())
        .stack_size(128 * 1024 * 1024)
        .spawn(|| {
            block_on_fem_window_ownership(Box::pin(async {
                use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example;
                use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_camera;
                use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::set_result_display;
                use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model;
                use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results;
                use semio_framework_plugin::artifact_app_laws;
                use semio_framework_plugin::ActionMeta;
                use semio_framework_plugin::App;
                use semio_framework_plugin::EditorApp;
                use semio_framework_plugin::PluginApp;
                use semio_framework_plugin::VcsArtifactApp;
                use semio_framework_plugin::ViewModel;
                use semio_framework_plugin::ViewWindowInstance;

                fn manifest() -> App {
                    App { definition: create_fem2d_app(), examples: Vec::new() }
                }

                async fn dispatch(app: &mut VcsArtifactApp<EditorApp<Fem2dPlayApp>>, view: &ViewModel, command: Fem2dCommand, expected_windows: usize, cx: &mut semio_framework_plugin::StepContext<'_>) -> Result<artifact_app_laws::TypedOperationFixtureReceipt, String> {
                    let command_id = command.command_id();
                    let meta = ActionMeta { instance_id: 71, view_state: Some(view.clone()), ..artifact_app_laws::meta("fem2d-window-ownership") };
                    let mut identity = semio_framework_plugin::artifact_app_laws::fixture_identity();
                    let dispatch = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app, &command, &meta, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut identity, cx);
                    Box::pin(dispatch).await.map_err(|error| format!("{error:?}"))?;
                    let receipt = Box::pin(artifact_app_laws::settle_registered_typed_operation(app, 71, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx)).await.map_err(|error| format!("{command_id}: {error:?}"))?;
                    let actual = receipt.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::WindowConfig).count();
                    if actual != expected_windows {
                        return Err(format!("expected {expected_windows} FEM window publications, got {actual}"));
                    }
                    Ok(receipt)
                }

                let all = ViewModel {
                    window_instances: vec![
                        ViewWindowInstance { id: "model-left".into(), window_kind_id: model::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: "model-right".into(), window_kind_id: model::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: "results-right".into(), window_kind_id: results::WINDOW_KIND_ID.into() },
                        ViewWindowInstance { id: "foreign".into(), window_kind_id: "fem-panel".into() },
                    ],
                    ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
                };
                let model_left = all.for_window_instance("model-left").expect("model-left");
                let results_left = all.for_window_instance("results-left").expect("results-left");
                let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
                let mut app = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Fem2dPlayApp>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await);
                app.bind_instance_id(71).await;
                let outcome: Result<(), String> = Box::pin(async {
                    let document_before = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_before = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    dispatch(&mut app, &model_left, Fem2dCommand::SetCamera(set_camera::SetCamera { x: 7.0, y: -3.0, zoom: 2.5 }), 1, scope.cx()).await?;
                    dispatch(&mut app, &results_left, Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").expect("source")), mode: "modal".into(), mode_index: 2, field: None, value: None, window_id: None }), 1, scope.cx()).await?;
                    let document_after = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
                    let app_after = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
                    if document_before.pack != document_after.pack || document_before.spr != document_after.spr {
                        return Err("FEM window commands changed document bytes".into());
                    }
                    if app_before.pack != app_after.pack || app_before.spr != app_after.spr {
                        return Err("FEM window commands changed app-config bytes".into());
                    }
                    let packs = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if packs.len() != 2 || packs.iter().any(|pack| pack.window_id.ends_with("right")) {
                        return Err("FEM window state crossed into a same-kind sibling".into());
                    }
                    let model_state = artifact_app_laws::capture_fixture_window_config::<model::config::Fem2dModelWindowConfigOwner, _, _>(&mut *app, &model_left).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing exact FEM model state".to_string())?;
                    if model_state.camera != (semio_framework_os_kernel::Viewport2d { x: 7.0, y: -3.0, zoom: 2.5 }) {
                        return Err("FEM model camera did not persist on its exact owner".into());
                    }
                    let result_state = artifact_app_laws::capture_fixture_window_config::<results::config::Fem2dResultsWindowConfigOwner, _, _>(&mut *app, &results_left).await.map_err(|error| format!("{error:?}"))?.ok_or_else(|| "missing exact FEM results state".to_string())?;
                    if result_state.result_source_id.as_deref() != Some("dead") || result_state.result_mode != semio_s_fem_app_surface::ResultMode::Modal || result_state.result_mode_index != 2 {
                        return Err("FEM result display did not persist on its exact owner".into());
                    }
                    let before_example: Vec<(String, Vec<u8>)> = packs.iter().map(|pack| (pack.window_id.clone(), pack.files.pack.clone())).collect();
                    let reset = dispatch(&mut app, &model_left, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() }), 0, scope.cx()).await?;
                    let [semio_framework::kernel::Effect::LoadDocument { pack, spr }] = reset.effects.as_slice() else {
                        return Err("FEM example command did not emit one host-owned document load".into());
                    };
                    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|error| format!("{error:?}"))?;
                    let after_example = app.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    let after_example: Vec<(String, Vec<u8>)> = after_example.iter().map(|pack| (pack.window_id.clone(), pack.files.pack.clone())).collect();
                    if before_example != after_example {
                        return Err("FEM example replacement changed exact window preferences".into());
                    }
                    if artifact_app_laws::capture_fixture_window_config::<model::config::Fem2dModelWindowConfigOwner, _, _>(&mut *app, &model_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&model_state)
                        || artifact_app_laws::capture_fixture_window_config::<results::config::Fem2dResultsWindowConfigOwner, _, _>(&mut *app, &results_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&result_state) {
                        return Err("FEM window values changed during document reset".into());
                    }
                    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
                    let mut reopened = Box::new(artifact_app_laws::new_app_with_registry::<EditorApp<Fem2dPlayApp>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await);
                    reopened.bind_instance_id(72).await;
                    let expected: std::collections::BTreeMap<_, _> = packs.iter().map(|pack| ((pack.window_id.clone(), pack.window_kind_id.clone()), (pack.files.pack.clone(), pack.files.spr.clone()))).collect();
                    for pack in packs {
                        reopened.load_window_config_pack(pack).await.map_err(|error| format!("{error:?}"))?;
                    }
                    let restored = reopened.window_config_packs().await.map_err(|error| format!("{error:?}"))?;
                    if restored.len() != 2 || restored.iter().any(|pack| pack.window_id.ends_with("right")) {
                        return Err("FEM window packs did not restore on exact owners".into());
                    }
                    let restored: std::collections::BTreeMap<_, _> = restored.into_iter().map(|pack| ((pack.window_id, pack.window_kind_id), (pack.files.pack, pack.files.spr))).collect();
                    if restored != expected {
                        return Err("FEM exact window Pack and SPR bytes changed during reload".into());
                    }
                    if artifact_app_laws::capture_fixture_window_config::<model::config::Fem2dModelWindowConfigOwner, _, _>(&mut *reopened, &model_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&model_state)
                        || artifact_app_laws::capture_fixture_window_config::<results::config::Fem2dResultsWindowConfigOwner, _, _>(&mut *reopened, &results_left).await.map_err(|error| format!("{error:?}"))?.as_ref() != Some(&result_state) {
                        return Err("FEM window values changed during exact config reload".into());
                    }
                    artifact_app_laws::close_registered_fixture_app(&mut *reopened, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), scope.cx()).expect("original caller paid app close");
                    let stale = ViewModel { window_id: Some("missing".into()), window_instances: all.window_instances.clone(), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
                    if model::config::addressed(&stale, Default::default()).is_ok() {
                        return Err("FEM accepted a stale model window id".into());
                    }
                    let foreign = all.for_window_instance("foreign").expect("foreign");
                    if results::config::addressed(&foreign, Default::default()).is_ok() {
                        return Err("FEM accepted a foreign results window kind".into());
                    }
                    Ok(())
                })
                .await;
                artifact_app_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), scope.cx()).expect("original caller paid app close");
                outcome.expect("FEM exact-window ownership runtime law");
            }))
        })
        .expect("spawn FEM window ownership law")
        .join()
        .expect("FEM window ownership law thread");
}
