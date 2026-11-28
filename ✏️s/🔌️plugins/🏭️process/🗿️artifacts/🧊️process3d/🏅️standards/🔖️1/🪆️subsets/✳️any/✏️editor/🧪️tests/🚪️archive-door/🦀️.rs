//! 🚪️ Process3d member-less example archive door: every registered example loads through the host's document archive door.

extern crate semio_os_kernel as protocol;

use semio_s_artifact_process_process3d_harness as context;

use context::action;
use pack::snapshot::ArtifactPack;
use semio_framework::{kernel::Effect, DslValue};
use semio_plugin::PluginApp;
use semio_s_artifact_process_process3d_kit::{PROCESS3D_EXAMPLE_CONCRETE_FOREST, PROCESS3D_EXAMPLE_PLATE, PROCESS3D_EXAMPLE_DEMO};
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

//#region 🚪️ExampleArchiveDoor
/// 🚪️ Settles one member-less archive load exactly the way the react shell's `loadDocumentPair` does.
fn settle_member_less_archive(app: &mut context::Process3dApp, operation: u64, parent_pack: Vec<u8>, parent_spr: Vec<u8>) -> protocol::DocumentArchiveLoadStatus {
    use semio_plugin::artifact_app_laws::{OriginalFixtureCaller, admit_document_archive_load, fixture_renew_turn};
    let fuel = OriginalFixtureCaller::turn_fuel();
    let grant = app.1.cx().retained_grant();
    admit_document_archive_load(&mut app.0, operation, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }, app.1.cx()).expect("archive admission");
    for _ in 0..1_000_000 {
        fixture_renew_turn(app.1.cx(), fuel, grant).expect("archive caller renewal");
        let polled = ::semio_framework_async::poll::resolve_ready(PluginApp::poll_document_archive_load(&mut app.0, operation, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            PluginApp::acknowledge_document_archive_load(&mut app.0, operation).expect("archive acknowledgement");
            return polled;
        }
        let _ = PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("archive maintenance step");
    }
    panic!("archive {operation} never reached a terminal state: {} / {}", PluginApp::close_diagnostic(&app.0), PluginApp::document_load_diagnostic(&app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance));
}

/// 🔁️ Dispatches `setActiveExample` and drives its typed operation to the published
/// `Effect::LoadDocument`, the way the host's publication ladder does.
fn published_example_load(app: &mut context::Process3dApp, example: &str) -> (Vec<u8>, Vec<u8>) {
    let instance = semio_plugin::artifact_app_laws::meta("local").instance_id;
    let result = action(app, "setActiveExample", Some(&DslValue::from(&serde_json::json!({ "exampleId": example }))));
    let mut loaded = result.requested_effects.into_iter().find_map(|effect| match effect {
        Effect::LoadDocument { pack, spr } => Some((pack, spr)),
        _ => None,
    });
    for _ in 0..100_000 {
        if loaded.is_some() && !app.has_pending_typed_operations() {
            break;
        }
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("example maintenance step");
        ::semio_framework_async::poll::resolve_ready(app.0.advance_typed_operation_publication(&mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("example publication");
        if let (Some(page), _) = semio_plugin::artifact_app_laws::take_fixture_result_turn(&mut app.0, instance, semio_plugin::artifact_app_laws::OriginalWorkFixture::stock()).expect("example result page") {
            assert!(app.acknowledge_typed_operation_result(page.token).expect("example result ack"));
        }
        if let Some(Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
            loaded = Some((pack, spr));
        }
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
    }
    loaded.unwrap_or_else(|| panic!("{example}: setActiveExample must publish a LoadDocument effect"))
}

/// 📤️ LAW: every registered example loads through the host's document archive door — `setActiveExample`
/// publishes `Effect::LoadDocument`, the shell hands its pack/spr back with NO members, and the
/// app's `genesis_child_pack` must complete the composed roster so the ownership closure is not
/// `Incomplete` (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP).
#[test]
fn every_example_loads_through_the_member_less_archive_door() {
    for (operation, example) in [(71_u64, PROCESS3D_EXAMPLE_DEMO), (72, PROCESS3D_EXAMPLE_PLATE), (73, PROCESS3D_EXAMPLE_CONCRETE_FOREST)] {
        let mut app = context::unseeded_app_with_registry();
        let (pack, spr) = published_example_load(&mut app, example);
        let expected = <Process3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("example pack decodes");
        let status = settle_member_less_archive(&mut app, operation, pack, spr);
        assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{example}: {}", String::from_utf8_lossy(&status.fault));
        let loaded = app.snapshot().expect("loaded snapshot");
        assert_eq!(loaded.stock_solid.child_id, expected.stock_solid.child_id, "{example}: the example replaced the boot document");
        assert_eq!(loaded.step_payloads, expected.step_payloads, "{example}: the example's steps are live");
    }
}

/// 👶️ LAW: every shipped example is CANONICAL — its composed-child handles are exactly the
/// content-addressed ids `process_working_scene_to_snapshot` mints from its own inline records today.
/// `genesis_process3d_child_pack` re-derives each `toolSolids` id from the step's solid, so a fixture
/// minted under an older B-Rep serialization names children genesis can no longer derive: the timber
/// and plate fixtures had drifted, and the member-less archive load of timber closed `Incomplete`
/// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). Regenerate with `print_dsl` of the remint.
#[test]
fn every_example_fixture_carries_its_canonical_child_handles() {
    for (example, document) in [(PROCESS3D_EXAMPLE_DEMO, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document()), (PROCESS3D_EXAMPLE_PLATE, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::plate_document()), (PROCESS3D_EXAMPLE_CONCRETE_FOREST, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::concrete_forest_document())] {
        let reminted = semio_s_artifact_process_process3d_model::snapshot::process_working_scene_to_snapshot(&semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(&document), document.workshop.clone());
        assert_eq!(document.stock_solid, reminted.stock_solid, "{example}: stale stockSolid handle");
        assert_eq!(document.steps, reminted.steps, "{example}: stale steps handle");
        assert_eq!(document.tool_solids, reminted.tool_solids, "{example}: stale toolSolids handles");
        assert_eq!(document, reminted, "{example}: the fixture is exactly its own remint");
    }
}
//#endregion 🚪️ExampleArchiveDoor
