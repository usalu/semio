//! 🧪️ Integration tests of the `s.fem.fem2d` play app editor that drive the whole mounted app through the shared `context` harness.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub mod context {
    use semio_s_artifact_fem_2d_editor::editor::fem2d::create_fem2d_app;
    use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
    use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp;
    use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example;
    use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
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
    /// [`FEM2D_TEST_INSTANCE`] so typed commands reach their retained routes, and retired through the
    /// framework's exact close loop when it goes out of scope (the store's `Drop` demands the
    /// terminal-empty witness; a test that already called [`close`] drops a closed shell, a no-op).
    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct Fem2dApp(pub VcsArtifactApp<EditorApp<Fem2dPlayApp>>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);

    impl std::ops::Deref for Fem2dApp {
        type Target = VcsArtifactApp<EditorApp<Fem2dPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for Fem2dApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for Fem2dApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), self.1.cx()).expect("original caller paid app close");
            }
        }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    fn view(kind: &str) -> ViewModel {
        let id = if kind == semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID { "model-left" } else { "results-left" };
        ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: kind.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }
    }

    pub(super) fn manifest() -> App { App { definition: create_fem2d_app(), examples: Vec::new() } }

    semio_framework_plugin::history_edit_acceptance_law!("fem", Fem2dPlayApp, manifest, "../../../../../../..");

    pub fn fem2d_app() -> Fem2dApp {
        let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<Fem2dPlayApp>>(manifest, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()));
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(FEM2D_TEST_INSTANCE));
        Fem2dApp(app, scope)
    }

    /// 🧪️ The instance id every `meta("local")` dispatch is stamped with.
    pub const FEM2D_TEST_INSTANCE: u32 = 1;

    /// 🧪️ A MOUNTED app: bound to [`FEM2D_TEST_INSTANCE`], so a typed command reaches its retained
    /// route instead of being refused with `interactive-job.live-instance`, and its publication
    /// actually settles into the stores.
    ///
    /// ⚠️ A bound app owes the store a terminal close — dropping one unclosed trips
    /// "artifact store reached Drop without its exact terminal-empty shallow-shell witness". Every
    /// caller must finish with [`close`], which is why this is a SEPARATE constructor: the plain
    /// [`fem2d_app`] stays droppable, and the dozens of render-only tests keep working unchanged.
    pub fn fem2d_mounted_app() -> Fem2dApp {
        fem2d_app()
    }

    /// 🧪️ A mounted app on the EMPTY document — the app boots on the bundled demo, so a test that
    /// counts what it adds itself starts from nothing instead (any example id but the demo's loads
    /// the empty snapshot).
    pub fn fem2d_empty_app() -> Fem2dApp {
        let mut app = fem2d_app();
        ::semio_framework_async::poll::resolve_ready(dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "empty".into() })));
        assert!(app.snapshot().expect("snapshot").nodes.is_empty(), "the empty example must load an empty document");
        app
    }

    /// 🧹️ Retires a [`fem2d_mounted_app`] the way the plugin host retires a closing instance.
    pub fn close(app: &mut Fem2dApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), app.1.cx()).expect("original caller paid app close");
    }

    pub async fn dispatch(app: &mut Fem2dApp, command: Fem2dCommand) -> InvocationResult {
        let kind = match &command {
            Fem2dCommand::SetCamera(_) => semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID,
            Fem2dCommand::SetResultDisplay(_) | Fem2dCommand::SetResultAnimation(_) | Fem2dCommand::ResultAnimationTick(_) => semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID,
            _ => semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID,
        };
        let mut action = meta("local");
        action.view_state = Some(view(kind));
        let mut result = app.0.dispatch_typed(command, &action, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, FEM2D_TEST_INSTANCE, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        for effect in &result.requested_effects {
            if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
                let files = store::os_store::serialization::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                semio_framework_plugin::artifact_app_laws::load_document(&mut app.0, &files, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("test host applies load-document effect");
            }
        }
        result
    }

    /// 🧾️ Dispatches `command` from the model window like [`dispatch`] and drives its retained publication home,
    /// answering the applied history rows with document ops its completions upserted — how a law reads the one row a
    /// gesture leaves.
    pub async fn dispatch_rows(app: &mut Fem2dApp, command: Fem2dCommand) -> Vec<semio_framework::kernel::HistoryEntry> {
        let mut action = meta("local");
        action.view_state = Some(view(model_window::WINDOW_KIND_ID));
        app.0.dispatch_typed(command, &action, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        let mut rows = Vec::new();
        while app.has_pending_typed_operations() {
            assert!(std::time::Instant::now() < deadline, "the typed operation settles within 30 seconds");
            app.0.maintenance_step(app.1.cx(), semio_framework_plugin::artifact_app_laws::fixture_mounted_policy().maintenance).expect("maintenance step");
            app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("advance the publication");
            while let Some(page) = app.take_typed_operation_result_page(FEM2D_TEST_INSTANCE) {
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
    pub async fn history_verb(app: &mut Fem2dApp, verb: &str) {
        semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut app.0, verb, FEM2D_TEST_INSTANCE, semio_framework_plugin::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    }

    pub fn render(app: &mut Fem2dApp, body_key: &str) -> String {
        let kind = if body_key == semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::BODY_KEY { semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID } else { semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID };
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
#[path = "../../🎮️commands/🩹️patch-node/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_node;
#[path = "../../🎮️commands/🩹️patch-region/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_region;
#[path = "../../🎮️commands/🩹️patch-section/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_section;
#[path = "../../🎮️commands/🩹️patch-support/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_support;
#[path = "../../🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod remove_selection;
#[path = "../../🎮️commands/⏱️result-animation-tick/🧪️tests/🔬️unit/🦀️.rs"]
mod result_animation_tick;
#[path = "../../🎮️commands/📚️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod set_active_example;
#[path = "../../🎮️commands/🧮️set-analysis-settings/🧪️tests/🔬️unit/🦀️.rs"]
mod set_analysis_settings;
#[path = "../../🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
mod set_camera;
#[path = "../../🎮️commands/⏯️set-result-animation/🧪️tests/🔬️unit/🦀️.rs"]
mod set_result_animation;
#[path = "../../🧪️tests/🪟️window-config-ownership/🦀️.rs"]
mod window_config_ownership;
