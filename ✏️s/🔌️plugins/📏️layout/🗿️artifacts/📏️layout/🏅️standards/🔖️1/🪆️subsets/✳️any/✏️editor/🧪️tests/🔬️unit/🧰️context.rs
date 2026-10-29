    use semio_s_artifact_layout_layout_editor_panels::document::LAYOUT_PLAY_BODY_ARTIFACT;
    use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::{LAYOUT_PLAY_BODY_BLUEPRINT, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_WINDOW_BLUEPRINT};
    use semio_s_artifact_layout_layout_editor_panels::catalogue::LAYOUT_PLAY_BODY_CATALOGUE;
    use semio_s_artifact_layout_layout_editor_panels::inspection::LAYOUT_PLAY_BODY_INSPECTION;
    use semio_s_artifact_layout_layout_editor_panels::preflight::{LAYOUT_PLAY_BODY_PREFLIGHT, run_layout_preflight};
    use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::{LAYOUT_PLAY_BODY_PREVIEW, LAYOUT_PLAY_WINDOW_PREVIEW};
    use crate::{LayoutCommand, LayoutPlayApp, create_layout_app};
    use semio_s_artifact_layout_layout_editor_panels::terminology::layout_labels;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, render_registered_fixture_app, settle_registered_typed_operation, OriginalFixtureScope};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// 🪪️ The one live instance every fixture app binds. A registry-backed app refuses every typed
    /// command until it knows its instance (`interactive-job.live-instance`) — `meta("local")`
    /// addresses exactly this id, so binding it is what makes the dispatch route live.
    pub const INSTANCE: u32 = 1;

    /// 🧪️ The app instance every test builds — registry-backed, instance-bound, and closing its
    /// stores on drop. `LayoutPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the
    /// runtime `ArtifactApp`; `EditorApp<LayoutPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<LayoutPlayApp>` builds it. The guard exists because the artifact
    /// store's drop witness panics for any fixture that reaches `Drop` without a terminal-empty
    /// shallow shell — the same guard lowpoly's fixture installs. Field `1` is the fixture's ONE original
    /// caller scope: every typed-operation, render, close and media call of the app threads its `cx`
    /// (`app.1.cx()`), and the drop guard pays the caller's terminal retirement after the paid app close.
    pub struct LayoutApp(pub VcsArtifactApp<EditorApp<LayoutPlayApp>>, pub OriginalFixtureScope);

    impl LayoutApp {
        /// 🧹️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
        pub fn close_fixture(&mut self) {
            close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
        }
    }

    impl std::ops::Deref for LayoutApp {
        type Target = VcsArtifactApp<EditorApp<LayoutPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for LayoutApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for LayoutApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                self.close_fixture();
                self.1.retire();
            }
        }
    }

    /// 🧪️ Adapts `create_layout_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry` still expects — framework test context gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    fn layout_app_manifest_for_tests() -> App {
        App { definition: create_layout_app(), examples: Vec::new() }
    }

    semio_framework_plugin::history_edit_acceptance_law!("layout", LayoutPlayApp, layout_app_manifest_for_tests, "../..");

    pub async fn layout_app() -> LayoutApp {
        layout_app_with_registry().await
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    /// `EditorApp<LayoutPlayApp>` publishes a `bounded_first_step_tool_proofs!` roster, and
    /// `with_registry_on_bus` joins that roster against the registry's `Migrated` tool ids
    /// (`AppActionRegistry::validate_tool_job_rows`): an empty registry declares none of them, so the
    /// registry-LESS `artifact_app_laws::new_app` fails construction outright with
    /// `interactive-job.catalog-authority … generated_migrated=false, migrated={}`.
    pub async fn layout_app_with_registry() -> LayoutApp {
        let mut app = new_app_with_registry::<EditorApp<LayoutPlayApp>>(layout_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        app.bind_instance_id(INSTANCE).await;
        LayoutApp(app, OriginalFixtureScope::admit())
    }

    /// 🪟️ The blueprint window instance every fixture dispatch rides. `ShellHost` NEVER dispatches
    /// without an addressed window — it stamps `ActionMeta.view_state` with the focused window
    /// instance — and layout's window-lane verbs (`setCamera`, the canvas pointer gestures,
    /// `focusPreflightIssue`) refuse a viewless dispatch with `layout-window-view-required`
    /// (`✏️editor/🦀️.rs`'s `addressed_window`). A fixture dispatching with bare `meta("local")`
    /// therefore put the app in a state the running shell can never produce, and every window-lane
    /// verb in this file was refused before its assertion ever ran.
    pub const WINDOW: &str = "layout-blueprint-1";

    /// 👁️ The preview window instance mounted beside {@link WINDOW}; it owns its own camera and config.
    pub const PREVIEW_WINDOW: &str = "layout-preview-1";

    /// 🪟️ The view the shell would send: both window kinds are mounted, as in the play pane.
    pub fn windowed_view() -> ViewModel {
        ViewModel {
            window_instances: vec![
                semio_framework_plugin::ViewWindowInstance { id: WINDOW.into(), window_kind_id: LAYOUT_PLAY_WINDOW_BLUEPRINT.into() },
                semio_framework_plugin::ViewWindowInstance { id: PREVIEW_WINDOW.into(), window_kind_id: LAYOUT_PLAY_WINDOW_PREVIEW.into() },
            ],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        }
    }

    /// 🎯️ `meta("local")` bound to {@link WINDOW}, exactly as `ShellHost` binds it.
    pub fn windowed_meta() -> semio_framework_plugin::ActionMeta {
        window_meta(WINDOW)
    }

    /// 🎯️ `meta("local")` bound to one mounted window instance of {@link windowed_view}.
    pub fn window_meta(window: &str) -> semio_framework_plugin::ActionMeta {
        semio_framework_plugin::ActionMeta { view_state: windowed_view().for_window_instance(window), ..meta("local") }
    }

    /// 🪟️ The view a body renders under: a window body inside its own window instance, as `ShellHost`
    /// renders it, so the window's config and transient lanes are the ones it reads; panels unaddressed.
    pub fn body_view(body_key: &str) -> ViewModel {
        let window = match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => WINDOW,
            LAYOUT_PLAY_BODY_PREVIEW => PREVIEW_WINDOW,
            _ => return ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
        };
        windowed_view().for_window_instance(window).expect("fixture window instance is mounted")
    }

    /// 🎯️ Dispatches one typed command through the retained route and settles its typed operation,
    /// so the snapshot a test reads next is the committed one.
    pub async fn dispatch(app: &mut LayoutApp, command: LayoutCommand) -> InvocationResult {
        dispatch_in(app, command, WINDOW).await
    }

    /// 🎯️ {@link dispatch} addressed to one mounted window instance.
    pub async fn dispatch_in(app: &mut LayoutApp, command: LayoutCommand, window: &str) -> InvocationResult {
        let verb = command.command_id().to_string();
        let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &window_meta(window), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("{verb} refused: {fault:?}"));
        settle_registered_typed_operation(&mut app.0, INSTANCE, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("{verb} did not settle: {fault:?}"));
        result
    }

    pub async fn render(app: &mut LayoutApp, body_key: &str) -> String {
        render_in(app, body_key, &body_view(body_key)).await
    }

    /// 🌐️ The same projection, rendered for one locale — a body's labels are resolved against the
    /// VIEW's locale, so a translation assertion that reuses the default view reads English text no
    /// matter which language it is asserting.
    pub async fn render_localized(app: &mut LayoutApp, body_key: &str, locale: &str) -> String {
        let view = ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag(locale).expect("declared fixture locale"), ..ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag(locale).expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
        render_in(app, body_key, &view).await
    }

    async fn render_in(app: &mut LayoutApp, body_key: &str, view: &ViewModel) -> String {
        render_registered_fixture_app(&mut app.0, body_key, view, app.1.cx()).await.expect("fixture projection")
    }

    /// 🎬️ The ASSEMBLED canvas scene behind a window body. A built surface carries its scene as a
    /// binary pack inside `SurfaceProps.doc` (plus out-of-doc payload lanes), so the projected tree's
    /// JSON text contains none of the scene's own strings — a test that greps the projection for a
    /// layer id reads an empty field no matter what the window rendered.
    pub async fn scene(app: &mut LayoutApp, body_key: &str) -> semio_framework_plugin::Canvas2dScene {
        let projection = render(app, body_key).await;
        semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).expect("window body carries an assembled canvas scene")
    }

    pub fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
        let camera = semio_framework_canvas::camera::Camera { x: camera_x, y: camera_y, zoom };
        let viewport = semio_framework_canvas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
        let screen = semio_framework_canvas::camera::world_to_screen(&camera, &viewport, semio_framework_canvas::Point::new(world_x, world_y));
        (screen.x, screen.y)
    }
