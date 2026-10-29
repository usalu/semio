//! 🧪️ Integration tests of the `s.fem.fem3d` play app editor that drive the whole mounted app through the shared `context` harness.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub mod context {
    use semio_s_artifact_fem_3d_editor::editor::fem3d::create_fem3d_app;
    use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
    use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp;
    use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_active_example;
    use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model::FEM3D_BODY_MODEL;
    use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model::FEM3D_WINDOW_MODEL;
    use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::FEM3D_WINDOW_RESULTS;
    use semio_framework_plugin::artifact_app_laws::meta;
    use semio_framework_plugin::artifact_app_laws::new_app_with_registry;
    use semio_framework_plugin::App;
    use semio_framework_plugin::EditorApp;
    use semio_framework_plugin::InvocationResult;
    use semio_framework_plugin::PluginApp;
    use semio_framework_plugin::VcsArtifactApp;
    use semio_framework_plugin::ViewModel;
    use semio_framework_plugin::ViewWindowInstance;

    /// 🧪️ The registered, MOUNTED fixture app every editor test drives — bound to
    /// [`FEM3D_TEST_INSTANCE`] so typed commands reach their retained routes, and retired through the
    /// framework's exact close loop when it goes out of scope (the store's `Drop` demands the
    /// terminal-empty witness; a test that already called [`close`] drops a closed shell, a no-op).
    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct Fem3dApp(pub VcsArtifactApp<EditorApp<Fem3dPlayApp>>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

    impl std::ops::Deref for Fem3dApp {
        type Target = VcsArtifactApp<EditorApp<Fem3dPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for Fem3dApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for Fem3dApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), self.1.cx()).expect("original caller paid app close");
            }
        }
    }

    /// 🪟️ A one-window view model addressing the named window kind.
    pub fn view(kind: &str) -> ViewModel {
        let id = if kind == edit::windows::model::FEM3D_WINDOW_MODEL { "model-left" } else { "results-left" };
        ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: kind.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }
    }

    pub(super) fn manifest() -> App { App { definition: create_fem3d_app(), examples: Vec::new() } }

    semio_framework_plugin::history_edit_acceptance_law!("fem", Fem3dPlayApp, manifest, "../../../../../../..");

    /// 🧪️ The instance id every `meta("local")` dispatch is stamped with.
    pub const FEM3D_TEST_INSTANCE: u32 = 1;

    /// 🧪️ A MOUNTED app: bound to [`FEM3D_TEST_INSTANCE`], so a typed command reaches its retained
    /// route instead of being refused with `interactive-job.live-instance`, and its publication
    /// actually settles into the stores. It boots on whatever `Fem3dPlayApp::initial_snapshot`
    /// boots on (the `concrete-forest` example); [`fem3d_demo_app`] loads the `demo` fixture.
    pub fn fem3d_app() -> Fem3dApp { fem3d_peer_app(semio_framework_os_kernel::LOCAL_ACTOR_ID) }

    /// 👥️ The mounted peer keeps its own original caller through every app operation.
    pub fn fem3d_peer_app(actor: &str) -> Fem3dApp {
        let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<Fem3dPlayApp>>(manifest, semio_framework_os_kernel::ActorId(actor.into()), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()));
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(FEM3D_TEST_INSTANCE));
        Fem3dApp(app, scope)
    }

    /// 🧪️ A mounted app on the bundled `demo` document — the app boots on `concrete-forest` (frames
    /// only), so every law stated over the demo's solids, supports and load cases (`sol1`, `ss_0`,
    /// `l2`, `hea200`, …) loads that fixture through the real `setActiveExample` door first.
    pub async fn fem3d_demo_app() -> Fem3dApp {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_3d_io::examples::demo::ID.into() })).await;
        assert!(!app.snapshot().expect("snapshot").solids.is_empty(), "the demo example must load the document carrying the meshed slab");
        app
    }

    /// 🧪️ A mounted app on the EMPTY document — the app boots on a bundled example, so a test that
    /// counts what it adds itself starts from nothing instead (any unknown example id loads the
    /// empty snapshot).
    pub async fn fem3d_empty_app() -> Fem3dApp {
        let mut app = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "empty".into() })).await;
        assert!(app.snapshot().expect("snapshot").nodes.is_empty(), "the empty example must load an empty document");
        app
    }

    /// 🧹️ Retires a mounted app the way the plugin host retires a closing instance.
    pub fn close(app: &mut Fem3dApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), app.1.cx()).expect("original caller paid app close");
    }

    /// 🪟️ The window kind a command is addressed to when the test names none.
    pub fn window_kind_for(command: &Fem3dCommand) -> &'static str {
        match command {
            Fem3dCommand::SetResultDisplay(_) | Fem3dCommand::SetResultAnimation(_) | Fem3dCommand::ResultAnimationTick(_) => edit::windows::results::FEM3D_WINDOW_RESULTS,
            _ => edit::windows::model::FEM3D_WINDOW_MODEL,
        }
    }

    pub async fn dispatch(app: &mut Fem3dApp, command: Fem3dCommand) -> InvocationResult {
        let kind = window_kind_for(&command);
        dispatch_in(app, command, view(kind)).await
    }

    /// 🧪️ Dispatches under an explicit view model (a chosen window, a selection-bearing interaction).
    pub async fn dispatch_in(app: &mut Fem3dApp, command: Fem3dCommand, view_state: ViewModel) -> InvocationResult {
        let mut action = meta("local");
        action.view_state = Some(view_state);
        let mut result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &action, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, FEM3D_TEST_INSTANCE, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        for effect in &result.requested_effects {
            if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
                let files = store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                semio_framework_plugin::artifact_app_laws::load_document(&mut app.0, &files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("test host applies load-document effect");
            }
        }
        result
    }

    /// 🧾️ Dispatches one typed command from the model window and drives its retained operation home, returning the
    /// history rows its completions upserted — the rows a gumball gesture's transaction landed as.
    pub async fn dispatch_rows(app: &mut Fem3dApp, command: Fem3dCommand) -> Vec<semio_framework::kernel::HistoryEntry> {
        let mut action = meta("local");
        action.view_state = Some(view(edit::windows::model::FEM3D_WINDOW_MODEL));
        semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &action, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut rows = Vec::new();
        while app.has_pending_typed_operations() {
            assert!(std::time::Instant::now() < deadline, "the typed operation settles within 30 seconds");
            app.0.maintenance_step(app.1.cx(), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy().maintenance).expect("maintenance step");
            app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("advance the publication");
            while let Some(page) = app.take_typed_operation_result_page(FEM3D_TEST_INSTANCE) {
                assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "the retained operation faulted: {}", String::from_utf8_lossy(page.bytes()));
                assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge the result page"), "the app accepts its own result token");
            }
            while app.take_typed_operation_effect().is_some() || app.take_typed_operation_event().is_some() || app.take_typed_operation_ui_scope().is_some() || app.take_typed_operation_composed_result().is_some() {}
            while let Some(completion) = app.take_typed_operation_completion().await.expect("take the completion") {
                rows.extend(completion.history_patch.into_iter().flat_map(|patch| patch.upserts).filter(|row| row.applied && !row.op_lines.is_empty()));
            }
        }
        rows
    }

    /// ↩️ Runs a framework-reserved history verb (`undo`/`redo`) and settles its publication.
    pub async fn history_verb(app: &mut Fem3dApp, verb: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, verb, FEM3D_TEST_INSTANCE, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }

    pub fn render(app: &mut Fem3dApp, body_key: &str) -> String {
        let kind = if body_key == edit::windows::model::FEM3D_BODY_MODEL { edit::windows::model::FEM3D_WINDOW_MODEL } else { edit::windows::results::FEM3D_WINDOW_RESULTS };
        ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, &view(kind), app.1.cx())).expect("fixture projection")
    }
}

#[path = "../../🎮️commands/📍️add-nodal-load/🧪️tests/🔬️unit/🦀️.rs"]
mod add_nodal_load;
#[path = "../../🎮️commands/⚪️add-node/🧪️tests/🔬️unit/🦀️.rs"]
mod add_node;
#[path = "../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod artifact_panel;
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod editor_unit;
#[path = "../../🎮️commands/🎯️focus-entity/🧪️tests/🔬️unit/🦀️.rs"]
mod focus_entity;
#[path = "../../🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs"]
mod gumball;
#[path = "../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod inspection_panel;
#[path = "../../🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs"]
mod model_window;
#[path = "../../🎮️commands/🩹️patch-combination/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_combination;
#[path = "../../🎮️commands/🩹️patch-element/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_element;
#[path = "../../🎮️commands/🩹️patch-load/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_load;
#[path = "../../🎮️commands/🩹️patch-load-case/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_load_case;
#[path = "../../🎮️commands/🩹️patch-material/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_material;
