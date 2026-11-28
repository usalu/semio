//! 🔁️ Live cleanup target: the mounted fem3d editor inside the production runtime, so the zero-progress credit of its live maintenance pump is exercised on its own.
#![recursion_limit = "256"]

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;

use semio_plugin::artifact_app_laws::{fixture_identity, fixture_mounted_policy, meta, new_app_with_registry, plugin_admit_document_load, plugin_poll_document_load, settle_registered_typed_operation, FixtureUiArena, LiveCleanupFixture, OriginalFixtureScope};
use semio_plugin::{EditorApp, PluginApp as _, ViewModel, ViewWindowInstance};
use semio_s_artifact_fem_3d_editor::editor::fem3d::{Fem3dCommand, Fem3dPlayApp};
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_node::AddNode;
use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model::FEM3D_WINDOW_MODEL;

const INSTANCE: u32 = 1;
const TURNS: usize = 400;
const STUCK_TURNS: usize = 200;

#[path = "../🚀️initialized-manifest/🦀️.rs"]
mod initialized_manifest;
use initialized_manifest::manifest;

fn view() -> ViewModel {
    ViewModel { window_id: Some("model-left".into()), window_instances: vec![ViewWindowInstance { id: "model-left".into(), window_kind_id: FEM3D_WINDOW_MODEL.into() }], ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

/// ⚖️ LAW: the live maintenance pump of a freshly mounted fem3d editor, and of the same editor after one committed edit, never exhausts its zero-progress credit over hundreds of host turns.
#[semio_framework_async_macros::async_test]
async fn fem3d_live_cleanup_never_reaches_the_zero_progress_verdict() {
    let _arena = FixtureUiArena::admit();
    let mut scope = OriginalFixtureScope::admit();
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), fixture_mounted_policy(), &mut fixture_identity()));
    ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(INSTANCE));
    let fixture = LiveCleanupFixture::admit(app, INSTANCE);
    let booted = fixture.pump(TURNS, scope.cx()).unwrap_or_else(|refusal| panic!("live cleanup faulted after boot: {refusal}"));
    assert!(fixture.app().app.maintenance_terminal_is_empty(), "the booted editor must settle every live maintenance frontier: {booted:?}");
    {
        let mut guard = fixture.app();
        let mut action = meta(semio_os_kernel::LOCAL_ACTOR_ID);
        action.view_state = Some(std::sync::Arc::new(view()));
        semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut guard.app, &Fem3dCommand::AddNode(AddNode { x: 1.0, y: 2.0, z: 3.0 }), &action, fixture_mounted_policy(), &mut fixture_identity(), scope.cx()).await.expect("dispatch the edit");
        settle_registered_typed_operation(&mut guard.app, INSTANCE, fixture_mounted_policy(), &mut fixture_identity(), scope.cx()).await.expect("settle the edit");
    }
    let edited = fixture.pump(TURNS, scope.cx()).unwrap_or_else(|refusal| panic!("live cleanup faulted after the edit: {refusal}"));
    assert_eq!((booted.final_stall, edited.final_stall), (0, 0), "an idle editor must leave the zero-progress counter at rest: {booted:?} {edited:?}");
    assert!(fixture.app().app.maintenance_terminal_is_empty(), "the idle editor must settle every live maintenance frontier");
    fixture.retire(scope.cx()).await;
}

/// ⚖️ LAW: a snapshot read lease held across a whole-document replacement (the live visual job of the old document in the browser) learns the old store is displaced: its commit authority is revoked the moment the replacement commits. The law first proves the failure mode, a holder that ignores the revocation keeps the replacement running for 200 turns, then that returning the lease lets the load reach `Ready`.
#[semio_framework_async_macros::async_test]
async fn fem3d_replacement_revokes_the_read_leases_of_the_displaced_store() {
    let _arena = FixtureUiArena::admit();
    let mut scope = OriginalFixtureScope::admit();
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), fixture_mounted_policy(), &mut fixture_identity()));
    ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(INSTANCE));
    let fixture = LiveCleanupFixture::admit(app, INSTANCE);
    fixture.pump(TURNS, scope.cx()).unwrap_or_else(|refusal| panic!("live cleanup faulted after boot: {refusal}"));
    let (files, mut lease, generation, revision) = {
        let guard = fixture.app();
        let archive = semio_plugin::PluginApp::document_archive(&guard.app).await.expect("archive the booted document");
        let files = store::os_store::serialization::ArtifactPackFiles { pack: archive.parent_pack, spr: archive.parent_spr, ops: String::new() };
        let (generation, revision) = (guard.app.stores.store.generation_now(), guard.app.stores.store.content_revision());
        (files, Some(guard.app.stores.store.snapshot_read().expect("lease the booted document")), generation, revision)
    };
    assert!(lease.as_ref().is_some_and(|read| read.commit_authority_matches(generation, revision)), "a fresh lease holds the live commit authority");
    plugin_admit_document_load(fixture.runtime(), INSTANCE, &files, &mut fixture_identity(), scope.cx()).await.expect("admit the replacement load");
    let (mut revoked_at, mut terminal, mut stuck_turns) = (None, None, 0usize);
    for turn in 0..TURNS * 20 {
        fixture.pump(1, scope.cx()).unwrap_or_else(|refusal| panic!("live cleanup faulted during the replacement: {refusal}"));
        let status = plugin_poll_document_load(fixture.runtime(), INSTANCE, &mut fixture_identity(), scope.cx()).await.expect("poll the replacement load");
        let running = matches!(status.state, store::DocumentArchiveLoadState::Pending | store::DocumentArchiveLoadState::Running);
        if lease.is_some() && !lease.as_ref().is_some_and(|read| read.commit_authority_matches(generation, revision)) {
            revoked_at.get_or_insert(turn);
            if stuck_turns < STUCK_TURNS {
                assert!(running, "a load whose displaced store still has an outstanding lease must not finish: {status:?}");
                stuck_turns += 1;
            } else {
                assert!(lease.take().expect("held lease").return_to_registry(), "the revoked holder returns its lease");
            }
        }
        if !running {
            terminal = Some(status.state);
            break;
        }
    }
    assert!(revoked_at.is_some(), "the displaced store never revoked the lease held across its replacement");
    assert_eq!(stuck_turns, STUCK_TURNS, "the replacement must stay blocked on the unreturned lease for every one of its {STUCK_TURNS} observed turns");
    assert_eq!(terminal, Some(store::DocumentArchiveLoadState::Ready), "the replacement load must reach Ready once the revoked lease is returned");
    drop(lease);
    fixture.retire(scope.cx()).await;
}
