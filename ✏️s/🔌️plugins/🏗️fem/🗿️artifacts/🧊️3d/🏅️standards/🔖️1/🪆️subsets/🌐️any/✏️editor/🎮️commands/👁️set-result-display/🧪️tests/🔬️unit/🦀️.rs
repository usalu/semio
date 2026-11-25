use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_result_display::*;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use crate::context::dispatch;
use crate::context::fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_result_display_writes_config_not_artifact_mutations() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(SetResultDisplay { source_id: Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead").unwrap()), mode: "modal".into(), mode_index: 1, field: None, value: None, window_id: None })).await;
}

fn results_view() -> semio_plugin::ViewModel {
    semio_plugin::ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![semio_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() }],
        ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }
}

async fn published(app: &mut crate::context::Fem3dApp) -> results::config::Fem3dResultsWindowConfig {
    semio_plugin::artifact_app_laws::capture_fixture_window_config::<results::config::Fem3dResultsWindowConfigOwner, _, _>(app, &results_view()).await.expect("capture results window config").unwrap_or_default()
}

fn control(field: &str, value: &str) -> Fem3dCommand {
    Fem3dCommand::SetResultDisplay(SetResultDisplay { source_id: None, mode: String::new(), mode_index: 0, field: Some(field.into()), value: Some(value.into()), window_id: Some("results-left".into()) })
}

/// 👁️ LAW: every source the Results panel offers is a canonical result-source identity the command admits, so choosing it lands in the window and the select shows it again; a bare case id is refused instead of silently stored.
#[semio_framework_async_macros::async_test]
async fn every_panel_source_option_is_admitted_and_selected() {
    let mut app = crate::context::fem3d_demo_app().await;
    let snapshot = app.snapshot().expect("snapshot");
    let live = snapshot.load_cases.iter().map(|case| ("loadCase", case.id.clone())).chain(snapshot.combinations.iter().map(|combination| ("combination", combination.id.clone()))).collect::<Vec<_>>();
    assert!(live.len() >= 2, "the demo offers several sources");
    for (kind, id) in live {
        let identity = semio_s_fem_analyses_model::encode_result_source_parts(kind, &id).expect("canonical identity");
        dispatch(&mut app, control("sourceId", &identity)).await;
        assert_eq!(published(&mut app).await.result_source_id.as_deref(), Some(identity.as_str()), "{kind} {id}");
    }
}

/// 👁️ LAW: Mode and Mode Index land in the window one by one, and the Results panel renders them back as the selected values.
#[semio_framework_async_macros::async_test]
async fn mode_and_mode_index_land_in_the_window_and_render_back() {
    let mut app = crate::context::fem3d_demo_app().await;
    dispatch(&mut app, control("mode", "modal")).await;
    dispatch(&mut app, control("modeIndex", "1")).await;
    let config = published(&mut app).await;
    assert_eq!((config.result_mode, config.result_mode_index), (semio_s_fem_app_surface::ResultMode::Modal, 1));
}

/// 🪞️ LAW: a panel control used while another pane holds focus still lands in the results window, keeps what the panel last showed (so the Mode select reads Modal, not Static) and publishes only the field it named.
#[semio_framework_async_macros::async_test]
async fn set_result_display_from_an_unfocused_panel_is_remembered_and_sparse_3d() {
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let view = semio_plugin::ViewModel {
        window_id: Some("results-shadow".into()),
        window_instances: vec![semio_plugin::ViewWindowInstance { id: "results-shadow".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() }],
        ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let pick = |field: &str, value: &str| SetResultDisplay { source_id: None, mode: String::new(), mode_index: 0, field: Some(field.into()), value: Some(value.into()), window_id: Some("results-shadow".into()) };
    handle_window(&pick("mode", "modal"), &cfg, &view).expect("mode");
    assert_eq!(results::config::remembered("results-shadow").expect("remembered").result_mode, semio_s_fem_app_surface::ResultMode::Modal);
    handle_window(&pick("modeIndex", "2"), &cfg, &view).expect("index");
    let remembered = results::config::remembered("results-shadow").expect("remembered");
    assert_eq!((remembered.result_mode, remembered.result_mode_index), (semio_s_fem_app_surface::ResultMode::Modal, 2), "a later field keeps the earlier one");
    let sparse = results::config::addressed_fields("results-shadow", &remembered, Some("mode"));
    assert_eq!(sparse.window_id(), "results-shadow");
}

fn palette_view() -> semio_plugin::ViewModel {
    semio_plugin::ViewModel {
        window_id: Some("model-left".into()),
        window_instances: vec![
            semio_plugin::ViewWindowInstance { id: "model-left".into(), window_kind_id: semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model::FEM3D_WINDOW_MODEL.into() },
            semio_plugin::ViewWindowInstance { id: "results-left".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() },
        ],
        ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }
}

/// 🎛️ LAW (3D): the Actions palette of the Model window speaks for the open results window, and its free-text Source accepts the name a load case carries — the Source, Mode and Mode Index it submits land in the results window's configuration instead of being refused or lost.
#[semio_framework_async_macros::async_test]
async fn result_display_from_the_model_window_palette_reaches_the_results_window_3d() {
    let mut app = crate::context::fem3d_demo_app().await;
    let snapshot = app.snapshot().expect("snapshot");
    let case = snapshot.load_cases.last().expect("the demo has load cases");
    let (id, name) = (case.id.clone(), case.name.clone());
    crate::context::dispatch_in(&mut app, Fem3dCommand::SetResultDisplay(SetResultDisplay { source_id: Some(name), mode: "modal".into(), mode_index: 1, field: None, value: None, window_id: None }), palette_view()).await;
    let config = published(&mut app).await;
    assert_eq!(config.result_source_id, Some(semio_s_fem_analyses_model::encode_result_source_parts("loadCase", &id).expect("canonical identity")));
    assert_eq!((config.result_mode, config.result_mode_index), (semio_s_fem_app_surface::ResultMode::Modal, 1));
}

/// 🎛️ LAW (3D): a panel control's single `sourceId` field also resolves a case name or id, keeps a canonical identity, and leaves an unknown name as typed for validation to refuse.
#[test]
fn a_source_name_resolves_to_its_canonical_identity_3d() {
    let snapshot = crate::context::Retiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo snapshot"));
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    let case = snapshot.load_cases.last().expect("the demo has load cases");
    let field = |value: &str| SetResultDisplay { source_id: None, mode: String::new(), mode_index: 0, field: Some("sourceId".into()), value: Some(value.into()), window_id: None };
    let canonical = semio_s_fem_analyses_model::encode_result_source_parts("loadCase", &case.id).expect("canonical identity");
    assert_eq!(resolved(&field(&case.name), &doc).value, Some(canonical.clone()));
    assert_eq!(resolved(&field(&case.id.to_uppercase()), &doc).value, Some(canonical.clone()));
    assert_eq!(resolved(&field(&canonical), &doc).value, Some(canonical));
    assert_eq!(resolved(&field("no such case"), &doc).value.as_deref(), Some("no such case"));
}
