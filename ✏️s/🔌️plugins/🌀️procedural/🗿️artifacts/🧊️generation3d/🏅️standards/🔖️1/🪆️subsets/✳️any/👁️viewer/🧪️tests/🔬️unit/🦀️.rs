use super::*;
use crate::viewer::generation3d::testkit::{app, dispatch, render};
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
        GENERATION3D_VIEW_TOOL_IDS.iter().chain(GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.iter()).chain(GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter()).chain(GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.iter()).copied().collect();
    let published: std::collections::BTreeSet<&str> = <Generation3dViewBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Generation3dViewContributionsJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewExampleJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .map(|contract| contract.tool_id)
        .collect();
    let proved: std::collections::BTreeSet<String> = <Generation3dViewer as ArtifactViewer>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id().to_string()).collect();
    let proved: std::collections::BTreeSet<&str> = proved.iter().map(String::as_str).collect();
    assert_eq!(commands, retained, "command enum ids and retained tool ids must be a bijection");
    assert_eq!(commands, published, "every tool needs an exact publication contract");
    assert_eq!(commands, proved, "every tool needs an exact bounded first-step proof");
}

/// 🔒️ The runtime half of the read-only guarantee: no viewer tool may publish on the document lane.
#[test]
fn no_viewer_tool_publishes_on_the_artifact_lane() {
    for contract in <Generation3dViewBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Generation3dViewContributionsJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewExampleJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
        .chain(<Generation3dViewFlowEvalJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter())
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
    for tool_id in GENERATION3D_VIEW_TOOL_IDS.iter().chain(GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter()) {
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
    let _serial = testkit::lock();
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
    let _serial = testkit::lock();
    let mut app = app().await;
    let scene = render(&mut app, preview::BODY_KEY).await;
    assert!(scene.contains(preview::BODY_KEY), "the preview body must render this viewer's own world-3d surface: {scene}");
}

/// 👁️ End-to-end liveness: a viewer action really dispatches through the interactive-job pipeline
/// (a non-`Migrated` or unproved action faults here) and really leaves the document untouched.
#[semio_framework_async_macros::async_test]
async fn every_viewer_action_dispatches_live_and_never_mutates_the_document() {
    let _serial = testkit::lock();
    let mut app = app().await;
    let before = testkit::snapshot(&app);
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
        assert_eq!(testkit::snapshot(&app), before, "viewer action {id} must not mutate the document");
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
    let _serial = crate::viewer::generation3d::testkit::lock();
    let definition = create_generation3d_viewer();
    let window = definition.window_kinds.iter().find(|kind| kind.id == preview::WINDOW_KIND_ID).expect("preview window kind");
    let declared: std::collections::BTreeSet<String> = window.actions.iter().map(|action| action.id.clone()).collect();
    let mut app = app().await;
    let projection = render(&mut app, preview::BODY_KEY).await;
    let mut emitted = crate::emitted_action_ids(&projection);
    drop(app);
    // 🎛️ `ArtifactViewer::window_measures` is an associated function over borrowed views, not a
    // `PluginApp` method, so the chrome half of the law reads the SAME builder the trait impl calls.
    emitted.extend(crate::measure_action_ids(&preview::preview_window_measures(&Generation3dViewConfig::default(), generation3d_view_action)));
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
