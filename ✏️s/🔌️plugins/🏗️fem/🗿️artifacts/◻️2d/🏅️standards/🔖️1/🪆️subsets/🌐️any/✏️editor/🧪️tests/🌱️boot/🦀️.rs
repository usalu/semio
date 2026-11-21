#[path = "../🔬️media-ports/🦀️.rs"]
mod media_ports;

use semio_framework_value::{DslValue, FromValue};
use semio_os_kernel::{ActorId, ArtifactDsl, LOCAL_ACTOR_ID};
use semio_os_kernel::os_store::serialization::ArtifactPackFiles;
use semio_plugin::artifact_app_laws::{
    OriginalFixtureScope, close_registered_fixture_app, dispatch_fixture_action,
    fixture_identity, fixture_mounted_policy, fixture_renew_turn, load_document, meta, new_app_with_registry_boxed,
    settle_framework_reserved_admission, settle_registered_typed_operation,
};
use semio_plugin::{App, ArtifactEditor, EditorApp, Effect, PluginApp, ViewModel, ViewWindowInstance};
use semio_s_artifact_fem_2d_editor::editor::fem2d::{Fem2dPlayApp, create_fem2d_app};
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::{
    io::text::snapshot::FEM2D_EXAMPLE_TEXT, schema::snapshot::Fem2dSnapshot,
};

fn manifest() -> App {
    App { definition: create_fem2d_app(), examples: Vec::new() }
}

/// 🌱️ A new registered editor mounts the bundled example geometry.
#[semio_framework_async_macros::async_test]
async fn the_editor_boots_on_the_bundled_example_document() {
    let expected = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_TEXT).expect("parse the bundled example");
    let birth = <Fem2dPlayApp as ArtifactEditor>::reference_birth().expect("reference snapshot producer");
    let boot = birth().expect("create the reference snapshot");
    let matches_example = boot == expected && !boot.nodes.is_empty() && !boot.elements.is_empty();
    Fem2dSnapshot::retire_decoded(boot);
    Fem2dSnapshot::retire_decoded(expected);
    assert!(matches_example, "the reference snapshot contains the bundled example geometry");

    let mut scope = OriginalFixtureScope::admit();
    let policy = fixture_mounted_policy();
    let mut identity = fixture_identity();
    let mut app = new_app_with_registry_boxed::<EditorApp<Fem2dPlayApp>>(
        manifest, ActorId(LOCAL_ACTOR_ID.into()), policy, &mut identity,
    ).await;
    app.bind_instance_id(1).await;
    let boot_geometry = (app.snapshot_ref().nodes.len(), app.snapshot_ref().elements.len());

    let mut action = meta("local");
    action.view_state = Some(ViewModel {
        window_id: Some("model-left".into()),
        window_instances: vec![
            ViewWindowInstance { id: "model-left".into(), window_kind_id: model::WINDOW_KIND_ID.into() },
            ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() },
        ],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }.into());
    let args = DslValue::object([(
        "exampleId".to_owned(),
        DslValue::String(semio_s_artifact_fem_2d_io::examples::demo::ID.into()),
    )]);
    let invoked = dispatch_fixture_action(
        &mut *app, "setActiveExample", Some(args), &action, &mut identity, scope.cx(), policy.maintenance,
    ).await.expect("dispatch the bundled example");
    let settled = settle_registered_typed_operation(&mut *app, 1, policy, &mut identity, scope.cx())
        .await.expect("settle the bundled example");
    let effects = invoked.requested_effects.into_iter().chain(settled.effects);
    let fixture = effects.filter_map(|effect| match effect {
        Effect::LoadDocument { pack, spr } => Some(ArtifactPackFiles { pack, spr, ops: String::new() }),
        _ => None,
    }).next().expect("the example produces a document load");
    let packed = <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&fixture.pack)
        .expect("decode the emitted example pack");
    let packed_geometry = (packed.nodes.len(), packed.elements.len());
    Fem2dSnapshot::retire_decoded(packed);
    let cx = scope.cx();
    let grant = cx.retained_grant();
    let fuel = cx.fuel_remaining();
    fixture_renew_turn(cx, fuel, semio_framework_value::RetainedCloneGrant { maximum_depth: 64, ..grant })
        .expect("lend the browser archive depth");
    assert_eq!(cx.retained_grant().maximum_depth, 64);
    load_document(&mut *app, &fixture, &mut identity, cx).await.expect("admit the example document at browser depth");
    let fuel = cx.fuel_remaining();
    fixture_renew_turn(cx, fuel, grant).expect("restore the native fixture grant");
    let loaded_geometry = (app.snapshot_ref().nodes.len(), app.snapshot_ref().elements.len());
    let previous_revision = app.test_document_revision();
    let panel_note = DslValue::from(&serde_json::json!({
        "commandId": "shell.panelTab",
        "label": { "en": "Artifact", "de": "Artefakt" },
        "detail": { "tabId": "artifact" }
    }));
    let admitted = dispatch_fixture_action(
        &mut *app, "noteShellCommand", Some(panel_note), &action, &mut identity, scope.cx(), policy.maintenance,
    ).await.expect("dispatch the shell's Artifact panel history entry");
    settle_framework_reserved_admission(&mut *app, admitted, scope.cx()).await.expect("run the reserved shell history job");
    settle_registered_typed_operation(&mut *app, 1, policy, &mut identity, scope.cx())
        .await.expect("settle the shell panel history entry");
    let unchanged_revision = app.test_document_revision();
    let original_x = app.snapshot_ref().nodes.iter().find(|node| node.id.as_str() == "n1").expect("bundled node n1").x;
    let args = DslValue::object([("id", "n1"), ("field", "x"), ("value", "1.5")]
        .map(|(key, value)| (key.to_owned(), DslValue::String(value.into()))));
    dispatch_fixture_action(
        &mut *app, "patchNode", Some(args), &action, &mut identity, scope.cx(), policy.maintenance,
    ).await.expect("dispatch the inspector edit");
    settle_registered_typed_operation(&mut *app, 1, policy, &mut identity, scope.cx())
        .await.expect("settle the inspector edit");
    let changed_revision = app.test_document_revision();
    let edited_x = app.snapshot_ref().nodes.iter().find(|node| node.id.as_str() == "n1").expect("edited node n1").x;
    close_registered_fixture_app(&mut *app, policy, scope.cx()).expect("retire the mounted editor");
    assert!(boot_geometry.0 > 0 && boot_geometry.1 > 0, "a new editor has geometry to render");
    assert_eq!(packed_geometry, boot_geometry, "the example pack contains the bundled geometry");
    assert_eq!(loaded_geometry, packed_geometry, "the retained load preserves the packed geometry");
    assert_eq!(unchanged_revision, previous_revision, "shell history leaves the document revision unchanged");
    assert_eq!((original_x, edited_x), (0.0, 1.5));
    assert_ne!(previous_revision, changed_revision, "analysis can observe the committed edit");
}
