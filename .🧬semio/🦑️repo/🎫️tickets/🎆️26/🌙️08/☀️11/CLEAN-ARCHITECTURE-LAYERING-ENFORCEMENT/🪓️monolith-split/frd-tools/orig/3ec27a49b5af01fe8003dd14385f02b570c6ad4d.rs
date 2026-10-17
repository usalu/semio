/// 🕹️ Command surface: window transient isolation, preview sharing, command round trips, declared actions and chords, and the document io surface.

#[semio_framework_async_macros::async_test]
async fn preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app() {
    use semio_s_artifact_procedural_3d_mode_edit::windows::preview::transient::Generation3dPreviewWindowTransientOwner;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = Box::new(app_with_registry().await);
    let (left, right) = preview_views("generation3d-preview-left", "generation3d-preview-right");
    let config_before = app.config_pack().await.expect("Generation3d app config before preview evaluation");
    let left_only = semio_framework_plugin::ViewModel { window_instances: left.window_instances.iter().filter(|window| Some(&window.id) == left.window_id.as_ref()).cloned().collect(), ..left.clone() };
    drain_flow_eval_ticks_with_view(&mut app, &left_only).await;
    {
        let left_state = app.window_transient_snapshot(&left).expect("left preview transient snapshot").expect("left preview owner");
        let right_state = app.window_transient_snapshot(&right).expect("right preview transient snapshot").expect("right preview owner");
        assert!(left_state.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.as_deref()).is_some_and(|text| !text.is_empty()));
        assert!(right_state.get::<Generation3dPreviewWindowTransientOwner>().is_some_and(|state| state.preview_eval_text.is_none()));
    }
    let config_after = app.config_pack().await.expect("Generation3d app config after preview evaluation");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr));
    drain_flow_eval_ticks_with_view(&mut app, &right).await;
    assert!(app.window_transient_snapshot(&right).expect("right evaluated snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_some()));
    let document = app.document_pack().await.expect("Generation3d document before reload");
    semio_framework_plugin::artifact_app_laws::load_document(&mut app, &document).await.expect("same document reload resets preview window transient");
    for view in [&left, &right] {
        assert!(app.window_transient_snapshot(view).expect("reset preview snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_none()));
    }
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut **app);
}
//#region 🔖️CommandSurface
/// ⚖️ LAW: the brep kernel's mesh transfer unit fits the wire bound `flowTessellateResolve` actually
/// declares, and that bound belongs to the CHAIN's own route, never to the gesture quota. A factory
/// registers ONE `ToolExecutionContract` for all its keys, so while the chain shared
/// `Generation3dBoundedCommandJobFactory` the transfer unit was pinned to 8 KiB minus the envelope
/// header — 4 KiB of mesh body per `flowEvalTick` round trip, i.e. ten round trips for
/// `sphere-cut-with-torus` and no painted preview inside any patience window
/// (`📓️preview-mesh-delivery-2026-09-12.md`). Widening it on the gesture route would have widened
/// 24 unrelated interactive routes with it, so the chain owns
/// [`Generation3dFlowEvalJobFactory`] and this law pins ITS bound to the transfer unit.




/// ⚖️ LAW: every declared `Generation3dCommand` row is retained-owned by exactly one
/// factory — the gesture, preview, contributions, or document route
/// route — with an exact, nonempty publication-lane contract, the shape
/// `ArtifactToolFactoryRegistry::register` itself enforces
/// (`🧰️framework/…/🔌️plugin/🦀️.rs:12736-12748`), asserted here so a future command addition that
/// forgets its retained-tool-id/publication-contract row fails this test instead of silently
/// reintroducing `interactive-job.missing-owned-reducer` at dispatch. Mirrors generation2d's own
/// `retained_route_dispositions_are_exact_and_exhaustive` (`…/generation2d/…/✏️editor/🦀️.rs:1033`).


/// 📏️ The scoped contributions pack the procedural plugin actually pushed on 2026-09-14, measured in
/// the browser (`[TRACE] contributions scoped pack {"chars":272089,…}`) — the size a declared ceiling
/// has to admit, not a round number.
const GENERATION3D_MEASURED_CONTRIBUTIONS_PACK_CHARS: usize = 272_089;

/// ⚖️ LAW: the contributions route's declared wire ceiling ADMITS the pack the host really pushes and
/// is the transport's own assembled-command authority, not the JSON entry point's body cap.


/// 🎯️ Counts the artifact / config / app-transient publications ONE preview command produces, through
/// the framework's own settle ladder.
///
/// 🐛️ This used to be a hand-rolled copy of that ladder which never drained
/// `take_typed_operation_completion` — the terminal witness lands in its own outbox and
/// `has_pending_typed_operations` counts it, so an operation whose last page had already been ACKed
/// spun here until the 30 s deadline and reported `Generation3d preview operation did not finish`
/// about an operation that had in fact finished. `settle_registered_typed_operation` grew that drain
/// and its docstring names this exact trap; a second copy of a bounded protocol is how a fix lands in
/// one of them only (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
async fn drive_preview_operation(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>) -> Result<(u64, u64, u64), String> {
    use semio_framework_plugin::app::TypedOperationResultLane;
    let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, semio_framework_plugin::artifact_app_laws::meta("preview-owner").instance_id)
        .await
        .map_err(|error| format!("{error:?}"))?;
    let count = |lane: TypedOperationResultLane| receipt.lanes.iter().filter(|published| **published == lane).count() as u64;
    Ok((count(TypedOperationResultLane::Artifact), count(TypedOperationResultLane::Config), count(TypedOperationResultLane::Transient)))
}

#[semio_framework_async_macros::async_test]
async fn generation_preview_is_one_evaluation_shared_by_two_generation_windows() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let before_document = crate::SnapshotRead::new(app.snapshot().map_err(|error| format!("{error:?}"))?);
        app.dispatch_typed(Generation3dCommand::AddGeneration(add_generation::AddGeneration {}), &semio_framework_plugin::artifact_app_laws::meta("preview-owner")).await.map_err(|error| format!("{error:?}"))?;
        let published = drive_preview_operation(&mut app).await?;
        if published != (1, 1, 0) {
            return Err(format!("a generation command publishes its document edit and selection config exactly once and no app transient: artifact={} config={} transient={}", published.0, published.1, published.2));
        }
        if crate::SnapshotRead::new(app.snapshot().map_err(|error| format!("{error:?}"))?).generation.as_state().generations.len() != before_document.generation.as_state().generations.len() + 1 {
            return Err("addGeneration did not preserve its document behavior".into());
        }
        let view = semio_framework_plugin::ViewModel {
            window_instances: vec![
                semio_framework::ViewWindowInstance { id: "preview-a".into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
                semio_framework::ViewWindowInstance { id: "preview-b".into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
            ],
            ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let mut rendered = Vec::new();
        for window_id in ["preview-a", "preview-b"] {
            let context = view.for_window_instance(window_id).ok_or("missing generation preview window")?;
            let tree = app.render(generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW, None, &context).await.map_err(|error| format!("{error:?}"))?;
            rendered.push(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?);
        }
        if rendered[0] != rendered[1] {
            return Err("generation windows did not consume the same preview evaluation".into());
        }
        if crate::runtime_fixture::text("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json").contains("generationPreviewText") {
            return Err("config schema still owns computed preview output".into());
        }
        Ok(())
    }
    .await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    result.expect("Generation3d preview ownership runtime");
}

#[test]
fn every_command_round_trips_through_text_and_binary() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// explicitly per row since generation3d's wire keys frequently diverge from a mechanical
/// kebab-case of the command id (for example, `nodeGraphViewport` → `viewport`).
#[test]
fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let expected_keywords = [
        "active-example",
        "graph-edit",
        "delete-selection",
        "remove-widget",
        "add-widget",
        "patch-flow-widgets",
        "reorganize",
        "translate-selection",
        "rotate-selection",
        "scale-selection",
        "add-generation",
        "remove-generation",
        "rename-generation",
        "update-generation-values",
        "viewport",
        "lod-mode",
        "show-mode",
        "toggle-sun",
        "sun-azimuth",
        "sun-elevation",
        "sun-intensity",
        "camera",
        "select-generation",
        "flow-eval-tick",
        "flow-eval-resolve",
        "flow-tessellate-resolve",
        "flow-eval-release",
        "flow-tessellate-cancel-resolve",
        "set-contributions",
        "import-document-request",
        "import-document",
        "export-document",
        "cycle-show-mode",
        "cycle-lod-mode",
        "select-next-node",
        "select-previous-node",
        "select-upstream-node",
        "select-downstream-node",
        "activate-selection",
        "edit-mesh-selection",
        "knife-mesh-selection",
        "set-widget-input",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
    for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
        let printed = protocol::OpText::print_op(command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Generation3dCommand> {
    vec![
        Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }),
        Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "extrude".into() }),
        Generation3dCommand::AddWidget(add_widget::AddWidget { neuron_kind: None, format: None, action: None, kind: "inputSlider".into(), x: Some(10.0), y: None }),
        Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
        Generation3dCommand::Reorganize(reorganize::Reorganize {}),
        Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0, phase: None, reason: None, window_id: None }),
        Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5, phase: None, reason: None, window_id: None }),
        Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0, phase: None, reason: None, window_id: None }),
        Generation3dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "generation-1".into() }),
        Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }),
        Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: semio_framework_value::DslValue::float(5.0) }),
        Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 3.0 } }),
        Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
        Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
        Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
        Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
        Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
        Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
        Generation3dCommand::SetCamera(set_camera::SetCamera { camera: semio_s_artifact_procedural_3d_editor_config::Generation3dPreviewCamera::default() }),
        Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
        Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "procedural-preview-test".into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() }),
        Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: "procedural-preview-test".into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 7, output_json: "{}".into(), extension_id: String::new(), ok: true, fault_code: String::new(), fault_message: String::new() }),
        Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 9, output_json: "{}".into() }),
        Generation3dCommand::FlowEvalRelease(flow_eval_release::FlowEvalRelease { window_id: "w1".into(), window_kind_id: "procedural-preview".into() }),
        Generation3dCommand::FlowTessellateCancelResolve(flow_tessellate_cancel_resolve::FlowTessellateCancelResolve { window_id: "w1".into(), window_kind_id: "procedural-preview".into(), output_json: "{\"ok\":true,\"retired\":1}".into(), ok: true }),
        Generation3dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into(), page: 0, page_count: 1 }),
        Generation3dCommand::ImportDocumentRequest(import_document_request::ImportDocumentRequest { widget_id: None, channel: None, texture_id: None }),
        Generation3dCommand::ImportDocument(import_document::ImportDocument { name: "cube.stl".into(), payload: "data:model/stl;base64,aGVsbG8=".into(), widget_id: None, channel: None, texture_id: None }),
        Generation3dCommand::ExportDocument(export_document::ExportDocument { format: "stl".into(), widget_id: None }),
        Generation3dCommand::CycleShowMode(cycle_show_mode::CycleShowMode {}),
        Generation3dCommand::CycleLodMode(cycle_lod_mode::CycleLodMode {}),
        Generation3dCommand::SelectNextNode(select_next_node::SelectNextNode {}),
        Generation3dCommand::SelectPreviousNode(select_previous_node::SelectPreviousNode {}),
        Generation3dCommand::SelectUpstreamNode(select_upstream_node::SelectUpstreamNode {}),
        Generation3dCommand::SelectDownstreamNode(select_downstream_node::SelectDownstreamNode {}),
        Generation3dCommand::ActivateSelection(activate_selection::ActivateSelection {}),
        Generation3dCommand::EditMeshSelection(edit_mesh_selection::EditMeshSelection { cuts: 1, operation: "extrude".into(), amount: 0.1, dx: 0.0, dy: 0.0, dz: 0.0, ..Default::default() }),
        Generation3dCommand::KnifeMeshSelection(knife_mesh_selection::KnifeMeshSelection { start: [0.0, -1.0, 0.0], end: [0.0, 1.0, 0.0] }),
        Generation3dCommand::SetWidgetInput(set_widget_input::SetWidgetInput { widget_id: "shape".into(), channel: "width".into(), value: "2".into(), component: None, ..Default::default() }),
    ]
}

//#region 📄️DocumentIoSurface
/// 🧫️ The language-agnostic law both implementations drive — see the fixture's own `note`, and the
/// TypeScript twin at `🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts`.
fn document_io_fixture() -> serde_json::Value {
    serde_json::from_str(crate::runtime_fixture::text("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚪️io/🗿️artifact-surface.json")).expect("the artifact-surface fixture is valid json")
}

fn fixture_rows(value: &serde_json::Value, key: &str) -> Vec<serde_json::Value> {
    value[key].as_array().unwrap_or_else(|| panic!("fixture has a `{key}` array")).clone()
}

/// 📇️ One declared action of this app by id.
fn declared_editor_action(definition: &semio_framework_plugin::AppDefinition, id: &str) -> semio_framework_plugin::ActionDefinition {
    declared_actions(definition).find(|action| action.id == id).unwrap_or_else(|| panic!("{id} is declared by no generation3d editor window")).clone()
}

/// 📝️ A declared action's `select` options, in declaration order.
fn declared_select_options(action: &semio_framework_plugin::ActionDefinition, arg_id: &str) -> Vec<semio_framework_plugin::ActionArgOption> {
    let arg = action.args.iter().find(|arg| arg.id == arg_id).unwrap_or_else(|| panic!("{} carries no `{arg_id}` arg", action.id));
    match &arg.schema {
        semio_framework_plugin::ArgSchema::String { options, .. } => options.clone(),
        other => panic!("{}'s `{arg_id}` is not a string select, it is {other:?}", action.id),
    }
}

/// ⚖️ LAW: the document-IO route's wire ceiling is DERIVED from the bounds that actually bind, and
/// each one is reachable.
///
/// The framework reassembles a picked file before `importDocument` runs
/// (`semio_framework::kernel::ImportStaging`), so the route carries ONE whole import. An imported file
/// is planted in the graph as an `InputNote`'s text, so one import is bounded by one Artifact-lane
/// edit; the route's body must hold that import plus its envelope, and the framework staging must be
/// able to reassemble every import the route admits.


/// ⚖️ LAW: the editor really OFFERS import and export — the exact finding
/// `📓️audit-user-journey-gaps-2026-09-13.md` §6 raised was that nothing did. Every row the fixture
/// names is a declared action with the declared kind, palette visibility and BOTH labels, and the
/// two user-facing verbs carry the fixture's keyboard chord, so neither is mouse-only.
#[test]
fn the_editor_declares_every_io_action_and_chord_the_fixture_names() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let definition = create_generation3d_app();
    for row in fixture_rows(&document_io_fixture(), "editorActions") {
        let id = row["id"].as_str().expect("action id");
        let action = declared_editor_action(&definition, id);
        assert_eq!(format!("{:?}", action.kind), row["kind"].as_str().expect("kind"), "{id}: action kind");
        assert_eq!(action.in_palette, row["inPalette"].as_bool().expect("palette flag"), "{id}: palette visibility");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), row["labelEn"].as_str().expect("en label"), "{id}: English label");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), row["labelDe"].as_str().expect("de label"), "{id}: German label");
        if let Some(chord) = row["chord"].as_str() {
            let bound = definition.keybindings.iter().find(|binding| binding.keys == chord).unwrap_or_else(|| panic!("{id}: no keybinding on {chord}"));
            assert_eq!(bound.action.action, id, "{chord} must reach {id}");
        }
    }
}

/// ⚖️ LAW: every keyboard verb `🧫️fixtures/⌨️keyboard-reachability.json` names is really bound, to
/// really that action, and — unless the fixture marks the row `staged` — that action takes NO
/// required argument. `ShellHost`'s keybinding loop fires an arg-free action straight through
/// `onAction`; anything with a required arg opens a staged form instead, which is slower than the
/// control the chord replaced. Before this, the thirty editor verbs had exactly two chords between
/// them (`mod+z`/`mod+shift+z`) and everything else was mouse-only
/// (`📓️audit-user-journey-gaps-2026-09-13.md` §3). The fixture is the language-agnostic statement;
/// this test is one implementation's reading of it.
#[test]
fn the_editor_binds_every_keyboard_verb_the_fixture_names() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture: serde_json::Value = serde_json::from_str(keyboard_reachability_fixture_json()).expect("keyboard fixture");
    let definition = create_generation3d_app();
    let rows = fixture["bindings"].as_array().expect("bindings");
    // 🧾️ Exactness, expressed against the APP-AUTHORED chords only: `build_definition` also mints a
    // keybinding for every framework `History`/`Clipboard`/`Interaction` action that carries its own
    // `keys` — `undo`/`redo`, and the interaction domain's `escape`→`clearSelection` and
    // `mod+a`→`selectAll` — and those are never a plugin's to declare. Exempting them by KIND rather
    // than by id is deliberate: a framework that mints a fourth such chord tomorrow must not break a
    // plugin fixture that never had a say in it.
    let named: std::collections::BTreeSet<&str> = rows.iter().map(|row| row["chord"].as_str().expect("chord")).collect();
    for binding in &definition.keybindings {
        if named.contains(binding.keys.as_str()) {
            continue;
        }
        let action = declared_actions(&definition).find(|action| action.id == binding.action.action);
        let framework_minted = action.is_some_and(|action| matches!(action.kind, semio_framework_plugin::ActionKind::History | semio_framework_plugin::ActionKind::Clipboard | semio_framework_plugin::ActionKind::Interaction));
        assert!(framework_minted, "{} reaches {} but the fixture never names it — the fixture is the WHOLE app keyboard surface", binding.keys, binding.action.action);
    }
    for row in rows {
        let chord = row["chord"].as_str().expect("chord");
        let id = row["action"].as_str().expect("action");
        let bound = definition.keybindings.iter().find(|binding| binding.keys == chord).unwrap_or_else(|| panic!("{id}: nothing is bound to {chord}"));
        assert_eq!(bound.action.action, id, "{chord} must reach {id}");
        if row["framework"].as_bool() == Some(true) {
            continue;
        }
        let requires_argument = declared_actions(&definition)
            .find(|action| action.id == id)
            .map(|action| action.args.iter().any(|arg| arg.required))
            .or_else(|| definition.commands.iter().find(|command| command.id == id).map(|command| command.args.iter().any(|arg| arg.required)))
            .unwrap_or_else(|| panic!("{id} is offered by no generation3d editor window and declared by no command"));
        assert_eq!(requires_argument, row["staged"].as_bool().unwrap_or(false), "{id}: a chord may only reach a required-argument verb when the fixture marks it staged");
    }
    if let Some(shell_chrome) = fixture.get("shellChrome").and_then(|value| value.as_array()) {
        for row in shell_chrome {
            let chord = row["chord"].as_str().expect("chord");
            assert!(!definition.keybindings.iter().any(|binding| binding.keys == chord), "{chord} belongs to the shell's own chrome; the app must not re-bind it");
        }
    }
}

/// 🪟️ Every window kind a mode's layout mounts, in layout order — the set `ShellHost`'s keybinding
/// loop scopes an app-wide chord against (`🏛️ShellHost/⌨️window-scope/🟦️.ts`, `mounted`).
fn mode_window_kind_ids(definition: &semio_framework_plugin::AppDefinition, mode_id: &str) -> Vec<String> {
    use semio_framework_plugin::{WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode};
    fn stack(node: &WindowLayoutStackNode, out: &mut Vec<String>) {
        out.extend(node.children.iter().map(|window| window.window_kind_id.clone()));
    }
    fn axis(node: &WindowLayoutAxisNode, out: &mut Vec<String>) {
        for child in &node.children {
            match child {
                WindowLayoutChild::Axis(node) => axis(node, out),
                WindowLayoutChild::Stack(node) => stack(node, out),
            }
        }
    }
    let mut out = Vec::new();
    if let Some(layout) = semio_framework::resolve_layout_for_mode(definition, mode_id) {
        match &layout.root {
            WindowLayoutRoot::Axis(node) => axis(node, &mut out),
            WindowLayoutRoot::Stack(node) => stack(node, &mut out),
        }
    }
    out
}

/// ⚖️ LAW: every WINDOW-SCOPED chord the fixture names has exactly ONE owning window kind, that kind
/// is the one the fixture names, and the modes whose layouts mount it are exactly the fixture's
/// `liveInModes`. This is the app half of the chord contract the shell's resolver implements: the
/// resolver targets the owner whether or not it is focused, so an app-wide chord for a window-owned
/// verb is live in exactly the modes that mount its owner and a hinted no-operation everywhere else.
///
/// 🐛️ Regression guard: `mod+shift+g` reached NOTHING in either mode — `addGeneration` was declared
/// only on the Generations window (`window_kind_action_refs`) while the chord was app-wide, and the
/// shell resolved a chord against the focused window kind alone, with no window active at all until
/// the user clicked one (`📓️react-generate-chord-restage-2026-09-14.md`).
#[test]
fn every_window_scoped_chord_names_its_owning_window_kind_and_the_modes_that_mount_it() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture: serde_json::Value = serde_json::from_str(keyboard_reachability_fixture_json()).expect("keyboard fixture");
    let definition = create_generation3d_app();
    let mode_ids: Vec<String> = definition.modes.iter().map(|mode| mode.id.clone()).collect();
    let mut scoped = 0;
    for row in fixture["bindings"].as_array().expect("bindings") {
        let chord = row["chord"].as_str().expect("chord");
        let id = row["action"].as_str().expect("action");
        if row["windowScoped"].as_bool() != Some(true) {
            assert!(row["ownerWindowKindId"].is_null(), "{chord}: only a window-scoped chord names an owner");
            continue;
        }
        scoped += 1;
        let owner = row["ownerWindowKindId"].as_str().unwrap_or_else(|| panic!("{chord}: a window-scoped chord must name its owning window kind"));
        let declaring: Vec<&str> = definition.window_kinds.iter().filter(|window| window.actions.iter().any(|action| action.id == id)).map(|window| window.id.as_str()).collect();
        assert_eq!(declaring, vec![owner], "{chord}: {id} must be declared by exactly the window kind the fixture names");
        let live: Vec<&String> = mode_ids.iter().filter(|mode_id| mode_window_kind_ids(&definition, mode_id).iter().any(|kind| kind == owner)).collect();
        let expected: Vec<&str> = row["liveInModes"].as_array().unwrap_or_else(|| panic!("{chord}: a window-scoped chord must name the modes it is live in")).iter().map(|value| value.as_str().expect("mode id")).collect();
        assert_eq!(live.iter().map(|id| id.as_str()).collect::<Vec<_>>(), expected, "{chord}: the modes whose layout mounts {owner}");
        assert!(!expected.is_empty(), "{chord}: a chord no mode can ever reach is a dead key, not a scope");
        let dead: Vec<&String> = mode_ids.iter().filter(|mode_id| !expected.contains(&mode_id.as_str())).collect();
        for mode_id in dead {
            assert!(!mode_window_kind_ids(&definition, mode_id).iter().any(|kind| kind == owner), "{chord}: {mode_id} must not mount {owner}, or the fixture under-states liveInModes");
        }
    }
    assert!(scoped >= 5, "the fixture must keep stating the window-scoped chords: {scoped}");
}

/// ⚖️ LAW: the example picker offers the bundled flow-fixture examples, and never the
/// `✏️editor/📚️examples/🎬️demo-session` leaf. That leaf is a `.cmd.semio` command REPLAY, not a
/// document carrier, and `setActiveExample`'s only vocabulary is "load a registered example
/// document" — the same reason `🖨️raster` and `🧩️puzzle` mount and test their `demo-session` leaves
/// without registering them. `📓️audit-user-journey-gaps-2026-09-13.md` §9 item 12 read the omission
/// as an oversight; this law makes it a statement.
#[test]
fn the_example_picker_offers_the_flow_examples_and_never_the_command_session() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let offered: Vec<String> = examples().iter().map(|example| example.id().to_string()).collect();
    assert_eq!(offered.len(), 9, "nine bundled flow examples: {offered:?}");
    assert!(!offered.iter().any(|id| id == semio_s_artifact_procedural_3d_editor::examples::demo_session::ID), "the command-replay leaf is not a document example");
    for id in &offered {
        assert!(semio_s_artifact_procedural_3d_model::is_generation3d_example_id(id), "{id} must be a registered generation3d example id");
    }
    let definition = create_generation3d_app();
    let action = declared_editor_action(&definition, "setActiveExample");
    assert_eq!(declared_select_options(&action, "exampleId").iter().map(|option| option.value.clone()).collect::<Vec<_>>(), offered, "the picker's options are `examples()`, in its order");
}

/// ⚖️ LAW: the export picker's options are the io module's OWN roster, in its order, with both
/// labels — not a second list that can drift from the formats this artifact actually writes.
#[test]
fn the_export_action_offers_every_declared_format_in_both_languages() {
    use semio_s_artifact_procedural_3d_io::document_io;
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let definition = create_generation3d_app();
    let action = declared_editor_action(&definition, "exportDocument");
    let options = declared_select_options(&action, "format");
    let offered: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    let declared: Vec<&str> = document_io::EXPORT_FORMATS.iter().map(|row| row.id).collect();
    assert_eq!(offered, declared, "the picker and the io roster are one list");
    for (option, row) in options.iter().zip(document_io::EXPORT_FORMATS.iter()) {
        assert_eq!(option.label.resolve(Terminology::Native, Locale::En), row.label_en, "{}: English option label", row.id);
        assert_eq!(option.label.resolve(Terminology::Native, Locale::De), row.label_de, "{}: German option label", row.id);
    }
    for row in fixture_rows(&document_io_fixture(), "exportFormats") {
        assert!(declared.contains(&row["id"].as_str().expect("id")), "the fixture's {} is offered", row["id"]);
    }
}

//#endregion 📄️DocumentIoSurface

//#endregion 🔖️CommandSurface
