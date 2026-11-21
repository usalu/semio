//! 🧰️ Builds the registered GIS Terrain editor for mounted artifact laws.

use super::artifact::editor::gis3d::{Gis3dCommand, Gis3dPlayApp, create_gis3d_app, modes::view::windows::terrain::GIS3D_PLAY_WINDOW_MAIN};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin::artifact_app_laws::{self as laws, OriginalFixtureCaller, TypedOperationFixtureReceipt, close_registered_fixture_app, meta, new_app_with_registry_boxed, settle_registered_typed_operation};
use semio_plugin::{ActionMeta, EditorApp, Media, MountedOwnerPolicyV1, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

/// 🗺️ The registered app and its original caller scope, which pays for terminal retirement.
pub struct Gis3dApp(pub Box<VcsArtifactApp<EditorApp<Gis3dPlayApp>>>, pub laws::OriginalFixtureScope, RetainedCloneGrant);

impl Gis3dApp {
    /// 🔁️ Issues the next fixture turn from its original allowance.
    pub fn renew_turn(&mut self) {
        laws::fixture_renew_turn(self.1.cx(), OriginalFixtureCaller::turn_fuel(), self.2).expect("fund Terrain fixture turn");
    }
}

impl std::ops::Deref for Gis3dApp {
    type Target = VcsArtifactApp<EditorApp<Gis3dPlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Gis3dApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 🧬️ Builds the real registered fixture and binds the instance addressed by [`meta`].
pub async fn app() -> Gis3dApp {
    let mut scope = laws::OriginalFixtureScope::admit();
    let grant = scope.cx().retained_grant();
    let mut app = new_app_with_registry_boxed::<EditorApp<Gis3dPlayApp>>(gis3d_app_manifest_for_tests, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut laws::fixture_identity()).await;
    app.bind_instance_id(meta("local").instance_id).await;
    Gis3dApp(app, scope, grant)
}

/// 📥️ Advances the registered media producer and publishes its artifact result.
pub async fn import_media(app: &mut Gis3dApp, port: &str, media: Media, actor: &ActionMeta) -> TypedOperationFixtureReceipt {
    let policy = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    app.renew_turn();
    let grant = app.2;
    let (app, scope) = (&mut *app.0, &mut app.1);
    let cx = scope.cx();
    laws::ensure_fixture_cancellation_issuer(app, cx).expect("birth Terrain media issuer");
    let mut media = Some(media);
    for _ in 0..OriginalFixtureCaller::maximum_turns() {
        laws::fixture_renew_turn(cx, OriginalFixtureCaller::turn_fuel(), grant).expect("fund Terrain media admission");
        if app.import_media(port, &mut media, actor, cx).await.expect("admit Terrain media input").is_some() {
            laws::fixture_renew_turn(cx, OriginalFixtureCaller::turn_fuel(), grant).expect("fund Terrain media publication");
            return settle_registered_typed_operation(app, actor.instance_id, policy, &mut laws::fixture_identity(), cx).await.unwrap_or_else(|failure| {
                panic!("publish Terrain media import {port}: {failure:?}, remaining grant {:?}, {}", cx.retained_grant(), app.typed_operation_diagnostic())
            });
        }
        app.maintenance_step(cx, policy.close).expect("advance Terrain media maintenance");
    }
    panic!("Terrain media input {port} exceeded the fixture turn limit: {}", app.typed_operation_diagnostic());
}

/// ✏️ Wraps the Terrain definition for the registered artifact test harness.
pub fn gis3d_app_manifest_for_tests() -> semio_plugin::App {
    semio_plugin::App { definition: create_gis3d_app(), examples: Vec::new() }
}

/// 🪟️ Targets the real Terrain window instance for render and command authority.
pub fn main_window_view() -> ViewModel {
    ViewModel {
        window_id: Some(GIS3D_PLAY_WINDOW_MAIN.into()),
        window_instances: vec![ViewWindowInstance {
            id: GIS3D_PLAY_WINDOW_MAIN.into(),
            window_kind_id: GIS3D_PLAY_WINDOW_MAIN.into(),
        }],
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }
}

/// 🧹️ Drives a Terrain fixture to its exact terminal-empty ownership witness.
pub fn close(app: &mut Gis3dApp) {
    app.renew_turn();
    close_registered_fixture_app(&mut *app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
}

/// 🎯️ Dispatches a typed Terrain command and completes its bounded host publication protocol.
pub async fn dispatch(app: &mut Gis3dApp, command: Gis3dCommand) -> TypedOperationFixtureReceipt {
    dispatch_at(app, command, main_window_view()).await
}

/// 🎯️ Dispatches through the exact addressed Terrain window captured by the retained operation.
pub async fn dispatch_at(app: &mut Gis3dApp, command: Gis3dCommand, view_state: ViewModel) -> TypedOperationFixtureReceipt {
    let mut action_meta = meta("local");
    action_meta.view_state = Some(semio_plugin::artifact_app_laws::receive_view_fixture(view_state, semio_plugin::artifact_app_laws::OriginalWorkFixture::stock()));
    app.renew_turn();
    let admission = semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut *app.0, &command, &action_meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
    assert!(admission.mutations.is_empty(), "retained Terrain commands publish only through acknowledged result pages");
    app.renew_turn();
    let issued = app.1.cx().original_turn_ceiling().0;
    let source_before = app.1.cx().original_source_total_grant();
    let budget_before = semio_framework_job::testkit::return_source_pinned(app.1.cx());
    let result = settle_registered_typed_operation(&mut *app.0, action_meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    result.unwrap_or_else(|error| {
        let source_after = app.1.cx().original_source_total_grant();
        let budget_after = semio_framework_job::testkit::return_source_pinned(app.1.cx());
        panic!("settle Terrain command {}: {error:?}; issued={issued:?} source_before={source_before:?} source_after={source_after:?} budget_before={budget_before:?} budget_after={budget_after:?}; {}", command.command_id(), app.typed_operation_diagnostic());
    })
}

/// ↩️ Drives a framework-reserved history verb (`undo`/`redo`) home. `handle_action` only ADMITS
/// such a verb and answers with the reserved job's `Effect::SpawnJob`; without settling that
/// admission the rewind never runs and the projection still shows the pre-undo document.
pub async fn history_verb(app: &mut Gis3dApp, action: &str) -> TypedOperationFixtureReceipt {
    app.renew_turn();
    let admitted = semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut *app.0, action, None, &meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("history verb");
    app.renew_turn();
    semio_plugin::app::settle_framework_reserved_admission(&mut *app.0, admitted, app.1.cx()).await.expect("settle the framework-reserved history verb");
    app.renew_turn();
    settle_registered_typed_operation(&mut *app.0, meta("local").instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the history verb publication")
}

pub async fn render(app: &mut Gis3dApp, body_key: &str) -> String {
    render_at(app, body_key, &main_window_view()).await
}

/// 🪟️ Renders from the persisted configuration of one exact Terrain window.
pub async fn render_at(app: &mut Gis3dApp, body_key: &str, view_state: &ViewModel) -> String {
    semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut *app.0, body_key, view_state, app.1.cx()).await.expect("render projection")
}
