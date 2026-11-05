//! 🪧️ Statik pane smoke target: the mounted fem3d editor app on its own, so the pane law compiles without the stale integration targets.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;

pub mod context {
    use semio_plugin::artifact_app_laws::{new_app_with_registry, OriginalFixtureScope};
    use semio_plugin::{App, EditorApp, PluginApp as _, VcsArtifactApp};
    use semio_s_artifact_fem_3d_editor::editor::fem3d::{create_fem3d_app, Fem3dPlayApp};

    /// 🏗️ The mounted fixture app and its one original caller scope.
    pub struct Fem3dApp(pub VcsArtifactApp<EditorApp<Fem3dPlayApp>>, pub OriginalFixtureScope);

    impl std::ops::Deref for Fem3dApp {
        type Target = VcsArtifactApp<EditorApp<Fem3dPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Drop for Fem3dApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_plugin::artifact_app_laws::fixture_mounted_policy(), self.1.cx()).expect("original caller paid app close");
            }
        }
    }

    fn manifest() -> App {
        App { definition: create_fem3d_app(), examples: Vec::new() }
    }

    /// 🧪️ A MOUNTED app bound to instance 1, booting on the concrete-forest example.
    pub fn fem3d_app() -> Fem3dApp {
        let scope = OriginalFixtureScope::admit();
        let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_plugin::artifact_app_laws::fixture_identity()));
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(1));
        Fem3dApp(app, scope)
    }
}

#[path = "../🪧️pane-smoke/🦀️.rs"]
mod pane_smoke;
