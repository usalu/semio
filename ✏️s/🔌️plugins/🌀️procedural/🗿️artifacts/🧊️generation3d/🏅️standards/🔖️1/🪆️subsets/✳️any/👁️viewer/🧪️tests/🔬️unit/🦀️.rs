pub(crate) mod context {
    //! 👁️ Viewer test harness — the read-only twin of the sibling surface's own test context.
    //!
    //! `ViewerApp<Generation3dViewer>` is the real `ArtifactApp` implementor `VcsArtifactApp` wraps,
    //! exactly the way `PluginBuilder::viewer::<Generation3dViewer>` builds it, so every assertion below
    //! runs against the production adapter rather than a hand-rolled stand-in.
    
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};

    /// 🧵️ `tessellate_geometry` and the flow-eval neuron kernel cache behind it are process-wide, so
    /// every viewer test that evaluates a flow fixture or tessellates BRep geometry — directly, or
    /// indirectly through the Preview window's `render()` — acquires the crate's ONE serial lock
    /// first ([`crate::test_serial`], not an editor module, so viewer purity is untouched).
    ///
    /// 🐛️ This door used to own a SECOND mutex over that same process-global state, so a viewer law
    /// and an editor law ran at the same time over one kernel cache and one registry
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub(crate) fn lock() -> crate::test_serial::TestSerialGuard {
        crate::test_serial::lock()
    }
    
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type Generation3dViewerHarness = VcsArtifactApp<ViewerApp<Generation3dViewer>>;
    
    pub fn generation3d_viewer_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_generation3d_viewer(), examples: Vec::new() }
    }
    
    /// 🧹️ A live viewer fixture that CLOSES itself — the read-only twin of the editor test context's
    /// `Generation3dAppFixture`, and for the same reason: `VcsArtifactApp`'s `ArtifactStore` owns a
    /// disposer whose `Drop` asserts terminal-empty ownership, so a plainly-dropped harness aborts the
    /// whole test binary (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub struct Generation3dViewerFixture(Generation3dViewerHarness);
    
    impl std::ops::Deref for Generation3dViewerFixture {
        type Target = Generation3dViewerHarness;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl std::ops::DerefMut for Generation3dViewerFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    
    impl Drop for Generation3dViewerFixture {
        fn drop(&mut self) {
            let witness = crate::editor::generation3d::unit_tests::context::close_ladder_witness(&mut self.0);
            assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "Generation3d viewer fixture did not reach its terminal-empty close witness{witness}");
        }
    }
    
    pub async fn app() -> Generation3dViewerFixture {
        let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_tests).await;
        app.bind_instance_id(1).await;
        Generation3dViewerFixture(app)
    }
    
    /// 📸️ Reads the live projection into a self-retiring [`Generation3dSnapshotRead`] — the read-only
    /// twin of the editor test context's own, and for the same reason (an owned `Generation3dSnapshot`
    /// aborts the binary on a bare drop).
    pub fn snapshot(app: &Generation3dViewerHarness) -> crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead {
        crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead::new(app.snapshot().expect("snapshot"))
    }
    
    pub async fn dispatch(app: &mut Generation3dViewerHarness, command: Generation3dViewCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut Generation3dViewerHarness, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }
    
    //#region ⏱️FlowEvalChain
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::artifact_app_laws::{settle_registered_typed_operation, TypedOperationFixtureReceipt};
    use semio_framework_plugin::{ActionMeta, Effect, ViewWindowInstance};
    
    /// 🏛️ The SHELL's own roster for the read-only surface. This viewer mounts exactly ONE window, so
    /// the preview window is also the current one — which is precisely why a window-scoped route here
    /// must still find its own window on the roster rather than assume it: the addressing law is the
    /// same law the sibling surface runs with a flow window in front of it.
    pub fn view_shell_view(preview: &str) -> ViewModel {
        let roster = vec![ViewWindowInstance { id: preview.into(), window_kind_id: preview::WINDOW_KIND_ID.into() }];
        ViewModel { window_instances: roster, ..Default::default() }.for_window_instance(preview).expect("viewer preview window instance")
    }
    
    /// 🏛️ Redispatches one armed `Effect::DispatchAction` exactly the way `makeEffectDispatchOne`
    /// (`🛠️ShellHelpers/🟦️.tsx`) does: an action id the app declares as a COMMAND re-enters the typed
    /// command channel with the shell's own live view attached, never the scoped action channel.
    pub async fn dispatch_effect_command(app: &mut Generation3dViewerHarness, command_id: &str, args: Option<&dsl::DslValue>, action_meta: &ActionMeta) -> Result<(), semio_framework_plugin::Fault> {
        crate::brep_extension::dispatch_effect_command(app, command_id, args, action_meta).await
    }
    
    pub async fn dispatch_with_view(app: &mut Generation3dViewerHarness, command: Generation3dViewCommand, view_state: ViewModel) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
        let action_meta = ActionMeta { view_state: Some(view_state), ..meta("local") };
        app.dispatch_typed(command, &action_meta).await?;
        settle_registered_typed_operation(app, action_meta.instance_id).await
    }
    
    pub use crate::brep_extension::{run_actions, PreviewRunReceipt};

    /// 🚦️ The framework run actions a refresh over `shell_view` owes the `previewEval` run right now.
    pub async fn owed_run_actions(app: &mut Generation3dViewerHarness, shell_view: &ViewModel) -> Vec<String> {
        crate::brep_extension::owed_run_actions(app, shell_view).await
    }

    /// 🔁️ The served `previewEval` loop on the read-only surface, answered by the in-process kernel
    /// ([`crate::brep_extension::drive_preview_run`]).
    pub async fn drive_preview_run(app: &mut Generation3dViewerHarness, shell_view: &ViewModel, initial: &[Effect]) -> PreviewRunReceipt {
        crate::brep_extension::drive_preview_run(app, shell_view, initial, &mut crate::brep_extension::serve).await
    }

    pub async fn drain_armed_flow_eval_ticks(app: &mut Generation3dViewerHarness, shell_view: &ViewModel) -> usize {
        drive_preview_run(app, shell_view, &[]).await.hops
    }

    /// 🔁️ The same loop, started from an effect list a caller already has in hand — the twin of
    /// [`drain_armed_flow_eval_ticks`] for a gesture's own receipt.
    pub async fn drain_armed_flow_eval_ticks_from(app: &mut Generation3dViewerHarness, shell_view: &ViewModel, initial: &[Effect]) -> usize {
        drive_preview_run(app, shell_view, initial).await.hops
    }

    /// 🎯️ The window every `flowEvalTick` in an effect list was addressed to, in order — what a law
    /// asserting "this gesture armed exactly these previews" reads. Pure over the effects, so it says
    /// nothing about how the chain is driven.
    pub fn armed_window_ids(effects: &[Effect]) -> Vec<String> {
        effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::DispatchAction { action, args, .. } if action == "flowEvalTick" => Some(args.as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str).unwrap_or_default().to_string()),
                _ => None,
            })
            .collect()
    }

    pub async fn render_with_view(app: &mut Generation3dViewerHarness, body_key: &str, view_state: &ViewModel) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
    }
    
    /// 🧊️ The mesh count a rendered viewer preview body actually carries.
    pub fn preview_mesh_count(projection: &str) -> usize {
        let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(projection).expect("the viewer preview body must decode as an assembled world-3d scene");
        let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
        meshes.as_array().map_or(0, Vec::len)
    }
    //#endregion ⏱️FlowEvalChain
}

use super::*;
use crate::viewer::generation3d::unit_tests::context::{app, dispatch, render};
use semio_framework_plugin::ArtifactViewer;

#[test]
fn create_generation3d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_generation3d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, GENERATION3D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Generation3dViewer as ArtifactViewer>::DIALECT, GENERATION3D_DIALECT);
}

/// ⚖️ LAW: the brep kernel's mesh transfer unit fits the wire bound the VIEWER's own
/// `flowTessellateResolve` declares — the same law the editor surface states about its own chain
/// route, so an LOD the editor can paint is never one the viewer silently cannot. While this route
/// carried the 8 KiB gesture quota the transfer unit collapsed to 4 KiB of base64 per whole
/// `flowEvalTick`, i.e. ten round trips for one `sphere-cut-with-torus` body
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️preview-mesh-delivery-2026-09-12.md`).
#[test]
fn view_tessellate_envelope_fits_the_declared_wire_bound() {
    let maximum = semio_framework_os_flow::brep_geometry::tessellate_envelope_maximum_bytes();
    assert!(maximum <= GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES, "one tessellate step envelope is at most {maximum} bytes but the viewer's declared wire bound is {GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES}");
    assert!(maximum > GENERATION3D_VIEW_RAW_BYTES, "a transfer unit that still fits the gesture quota needs no route of its own");
    assert_eq!(generation3d_view_flow_eval_contract().max_raw_wire_bytes, GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES, "the registered contract and the factory-side wire cap are one bound");
}

/// 🧾️ The four tables that must agree or a viewer route is silently dead: the typed command enum's
/// own ids, the three retained tool-id lists (view actions, the host contributions push and the
/// runtime evaluation chain), the factories' per-tool publication contracts, and the bounded
/// first-step proofs.
#[test]
fn every_viewer_tool_id_is_declared_in_all_four_tables() {
    let commands: std::collections::BTreeSet<&str> = Generation3dViewCommand::TOOL_JOB_IDS.iter().copied().collect();
    let retained: std::collections::BTreeSet<&str> =
        GENERATION3D_VIEW_TOOL_IDS
            .iter()
            .chain(GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.iter())
            .chain(GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter())
            .chain(GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.iter())
            .chain(GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.iter())
            .copied()
            .collect();
    let published: std::collections::BTreeSet<&str> = <Generation3dViewBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Generation3dViewContributionsJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewExampleJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewDocumentIoJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .map(|contract| contract.tool_id)
        .collect();
    let proved: std::collections::BTreeSet<String> = <Generation3dViewer as ArtifactViewer>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id().to_string()).collect();
    let proved: std::collections::BTreeSet<&str> = proved.iter().map(String::as_str).collect();
    assert_eq!(commands, retained, "command enum ids and retained tool ids must be a bijection");
    assert_eq!(commands, published, "every tool needs an exact publication contract");
    assert_eq!(commands, proved, "every tool needs an exact bounded first-step proof");
}

/// ⚖️ LAW: a reader can take the document away with them. `📓️audit-user-journey-gaps-2026-09-13.md`
/// §6 found this viewer's eight commands were all view-only and named none of the artifact's nine
/// `🚪️io` leaves. The export verb is declared, palette-visible, keyboard-reachable, offers the io
/// module's OWN format roster in both languages — and IMPORT is deliberately absent, because
/// replacing a document is not something a read-only surface may do.
#[test]
fn the_viewer_offers_export_in_both_languages_and_never_offers_import() {
    use crate::standards::v1::subsets::any::io::document_io;
    use semio_framework_plugin::{ArgSchema, Locale, Terminology};
    let definition = create_generation3d_viewer();
    let window = definition.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("the viewer declares its preview window kind");
    let action = window.actions.iter().find(|action| action.id == "exportDocument").expect("the viewer declares exportDocument");
    assert!(action.in_palette, "a reader has to be able to find it");
    assert_eq!(action.kind, ActionKind::View, "ShellHost's read-only gate swallows a Mutation-kind action on a viewer session");
    // 🪪️ NO trailing ellipsis in the declared label: the shell appends one itself for every action
    // that carries staged args (`🛠️ShellHelpers/🟦️.tsx`, `label: `${action.label}…``), so a declared
    // one painted "Export Document……" / "Dokument exportieren……" in the Actions pane (ticket
    // 26/09/09/PROCEDURAL-3D-END-TO-END, `🐍️react-gap-probe.mjs` step `actions-pane-de`).
    assert_eq!(action.label.resolve(Terminology::Native, Locale::En), "Export Document");
    assert_eq!(action.label.resolve(Terminology::Native, Locale::De), "Dokument exportieren");
    let arg = action.args.iter().find(|arg| arg.id == "format").expect("exportDocument carries a format arg");
    let ArgSchema::String { options, .. } = &arg.schema else { panic!("the format arg is a string select") };
    assert_eq!(options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>(), document_io::EXPORT_FORMATS.iter().map(|row| row.id).collect::<Vec<_>>(), "the viewer's picker and the io roster are one list");
    for (option, row) in options.iter().zip(document_io::EXPORT_FORMATS.iter()) {
        assert_eq!(option.label.resolve(Terminology::Native, Locale::De), row.label_de, "{}: German option label", row.id);
    }
    assert!(definition.keybindings.iter().any(|binding| binding.keys == "mod+shift+e" && binding.action.action == "exportDocument"), "the export verb is keyboard-reachable, not mouse-only");
    for forbidden in ["importDocument", "importDocumentRequest"] {
        assert!(!window.actions.iter().any(|action| action.id == forbidden), "a viewer must never declare {forbidden}");
        assert!(!Generation3dViewCommand::TOOL_JOB_IDS.contains(&forbidden), "a viewer must never own the {forbidden} tool");
    }
}

/// 🔒️ The runtime half of the read-only guarantee: no viewer tool may publish on the document lane.
#[test]
fn no_viewer_tool_publishes_on_the_artifact_lane() {
    for contract in <Generation3dViewBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Generation3dViewContributionsJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewExampleJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        // 📤️ The io route is in this law deliberately: `exportDocument` is the one io direction a
        // viewer may own, and the reason it may is that it publishes on NO store lane at all.
        .chain(<Generation3dViewDocumentIoJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
    {
        assert!(!contract.lanes.is_empty(), "{} declares no publication lane", contract.tool_id);
        assert!(!contract.lanes.contains(&ArtifactToolPublicationLane::Artifact), "viewer tool {} must never publish on the artifact lane", contract.tool_id);
        assert!(!contract.lanes.contains(&ArtifactToolPublicationLane::Draft), "viewer tool {} must never publish on the draft lane", contract.tool_id);
    }
}

/// 🕹️ Every declared action must be `Migrated`, the only UI-dispatchable classification — an
/// unclassified viewer action is rejected at dispatch with `interactive-job.not-ui-safe`.
///
/// 🎨️ `setActiveExample` is in this law too: it is an app-scoped verb `build_definition` copies onto
/// every window kind, so it reaches the SAME `window.actions` table the seven window verbs do — and
/// its `ActionKind` must be `View`, never `Mutation`, or `ShellHost`'s read-only gate swallows every
/// pick on this surface (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn every_declared_viewer_action_is_migrated() {
    let def = create_generation3d_viewer();
    let window = def.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("the viewer declares its preview window kind");
    for tool_id in GENERATION3D_VIEW_TOOL_IDS.iter().chain(GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter()).chain(GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.iter()) {
        let action = window.actions.iter().find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("tool {tool_id} has no declared action"));
        assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "action {tool_id} is not Migrated");
        assert_eq!(action.kind, ActionKind::View, "a viewer action must be a View action");
    }
}

/// 🕸️ The `graph` domain must be declared AND bound to the Preview window, or a world pick has
/// nowhere to land and `PreviewInteractionMarks`-style painting never sees an id.
#[test]
fn the_preview_window_is_bound_to_the_graph_interaction_domain() {
    let def = create_generation3d_viewer();
    let domain = def.interactions.iter().find(|interaction| interaction.id == "graph").expect("the viewer declares the graph interaction domain");
    let granularities: Vec<&str> = domain.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
    assert_eq!(granularities, vec!["node", "edge", "handle"]);
    assert!(domain.hover.transitive, "hovering a cluster must transitively light its nested widgets");
    assert!(domain.selection.broadcast, "a co-viewer must see what this viewer selected");
    let window = def.window_kinds.iter().find(|window| window.id == preview::WINDOW_KIND_ID).expect("the viewer declares its preview window kind");
    assert_eq!(window.surface_kind, semio_framework_plugin::SurfaceKind::World3d, "hover/selection picking needs a world-3d surface");
    assert!(window.interactions.iter().any(|item| item.as_str() == "graph"), "the preview window must be bound to the graph interaction domain");
}

/// 🕸️ The declared topology must contain the exact channel-qualified `handle` ids a world pick
/// reports, or `validate_state` prunes every hover the user produces.
#[test]
fn the_interaction_topology_declares_node_handle_and_edge_targets() {
    let _serial = context::lock();
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = Generation3dViewConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = <Generation3dViewer as ArtifactViewer>::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get("graph").expect("graph domain");
    let granularities: std::collections::BTreeSet<&str> = domain.ordered.iter().map(|node| node.granularity.as_str()).collect();
    assert!(granularities.contains("node"), "every widget must be a declared node target");
    assert!(granularities.contains("handle"), "every visible port must be a declared handle target");
    snapshot.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn the_viewer_renders_a_world3d_preview_for_the_default_document() {
    let _serial = context::lock();
    let mut app = app().await;
    let scene = render(&mut app, preview::BODY_KEY).await;
    assert!(scene.contains(preview::BODY_KEY), "the preview body must render this viewer's own world-3d surface: {scene}");
}

/// 👁️ End-to-end liveness: a viewer action really dispatches through the interactive-job pipeline
/// (a non-`Migrated` or unproved action faults here) and really leaves the document untouched.
#[semio_framework_async_macros::async_test]
async fn every_viewer_action_dispatches_live_and_never_mutates_the_document() {
    let _serial = context::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    let commands = vec![
        Generation3dViewCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
        Generation3dViewCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
        Generation3dViewCommand::SetCamera(set_camera::SetCamera { camera: crate::viewer::generation3d::config::Generation3dViewCamera { position: [9.0, 8.0, 7.0], target: [0.0, 0.0, 0.0], fov: 33.0 } }),
        Generation3dViewCommand::ToggleSun(toggle_sun::ToggleSun {}),
        Generation3dViewCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 120.0 }),
        Generation3dViewCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 42.0 }),
        Generation3dViewCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 2.0 }),
        // 🎨️ The navbar example picker's verb belongs in this law too: loading an example into a
        // read-only surface is a CONFIG edit, and it must stay one
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        Generation3dViewCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL.into() }),
        Generation3dViewCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }),
    ];
    for command in commands {
        let id = command.command_id();
        dispatch(&mut app, command).await;
        assert_eq!(context::snapshot(&app), before, "viewer action {id} must not mutate the document");
    }
}

//#region 📇️WindowActionLawTests
/// 📇️ THE window-kind action law for the read-only surface (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// This viewer has ONE window kind, and its whole chrome — `setShowMode`/`setLodMode` and the sun group
/// from `preview::preview_window_measures`, plus the world host's own `setCamera` — belongs to it, so the
/// law here is an equality rather than a containment: every action the app declares is dispatched by that
/// window, and the window declares every action it dispatches. `WindowKindDefinition.actions` is what
/// `ShellHost`'s `declaredAction` gate reads before it will call `plugin.handleAction`
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5691`).
#[semio_framework_async_macros::async_test]
async fn every_emitted_action_is_declared_on_the_preview_window_kind() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let definition = create_generation3d_viewer();
    let window = definition.window_kinds.iter().find(|kind| kind.id == preview::WINDOW_KIND_ID).expect("preview window kind");
    let declared: std::collections::BTreeSet<String> = window.actions.iter().map(|action| action.id.clone()).collect();
    let mut app = app().await;
    let projection = render(&mut app, preview::BODY_KEY).await;
    let mut emitted = crate::emitted_action_ids(&projection);
    drop(app);
    // 🎛️ `ArtifactViewer::window_measures` is an associated function over borrowed views, not a
    // `PluginApp` method, so the chrome half of the law reads the SAME builder the trait impl calls.
    emitted.extend(crate::measure_action_ids(&preview::preview_window_measures(&Generation3dViewConfig::default(), false, generation3d_view_action)));
    // 📷️ `setCamera` is dispatched by the world host's own viewport gesture (`World3dHost/🟦️.tsx`), not by
    // a measure or a `UiNode` binding, so the render/measure walk can never observe it — it is asserted
    // against the declaration directly instead of being dropped from the law.
    emitted.insert("setCamera".into());
    println!("[STATS] window-actions kind={} declared={} emitted={} emits={emitted:?}", preview::WINDOW_KIND_ID, declared.len(), emitted.len());
    for action in &emitted {
        assert!(declared.contains(action), "{} emits {action} but never declares it — ShellHost's declaredAction gate drops it", preview::WINDOW_KIND_ID);
    }
    // 🧾️ Exactness, expressed against the APP-AUTHORED verbs only: `build_definition` also injects the
    // framework's own history/clipboard/tutorial/interaction ids into every window kind, and those are
    // never a plugin's to scope. `GENERATION3D_VIEW_TOOL_IDS` is this viewer's own action roster.
    assert_eq!(emitted, GENERATION3D_VIEW_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<String>>(), "the viewer's one window kind must dispatch exactly the app's own view actions");
}
//#endregion 📇️WindowActionLawTests

/// 👁️ A viewer exports WHAT IT SHOWS. With an active example picked, every other viewer surface reads
/// `Generation3dViewedDocument::resolve(snapshot, config)` and paints that example; the export path
/// alone read the opened snapshot raw, so it encoded a document the retained session never evaluated
/// and every one of the seven formats failed with `generation3d export: the document evaluates to no
/// preview geometry (no positions)` while three meshes stood on screen (measured on :6023,
/// ticket 26/09/09/PROCEDURAL-3D-END-TO-END gap F2).
#[test]
fn the_viewer_exports_the_example_it_is_showing_not_the_opened_document() {
    let opened = crate::standards::v1::subsets::any::schema::empty_generation3d_snapshot();
    for example_id in ["hexagonal-mushroom-column", "rectangle-extrude-volume"] {
        let config = Generation3dViewConfig { active_example_id: Some(example_id.to_string()), ..Default::default() };
        let viewed = super::Generation3dViewedDocument::resolve(&opened, &config);
        let widgets = viewed.snapshot().fixture.widgets.len();
        println!("[DEBUG] viewer export document for {example_id}: widgets={widgets}");
        assert!(widgets > 0, "{example_id}: the viewed document must carry the example's widgets, not the opened document's none");
        viewed.retire();
    }
    let none = Generation3dViewConfig { active_example_id: None, ..Default::default() };
    let viewed = super::Generation3dViewedDocument::resolve(&opened, &none);
    assert_eq!(viewed.snapshot().fixture.widgets.len(), opened.fixture.widgets.len(), "with no example picked the viewer exports the opened document itself");
    viewed.retire();
    opened.retire_cold();
}
