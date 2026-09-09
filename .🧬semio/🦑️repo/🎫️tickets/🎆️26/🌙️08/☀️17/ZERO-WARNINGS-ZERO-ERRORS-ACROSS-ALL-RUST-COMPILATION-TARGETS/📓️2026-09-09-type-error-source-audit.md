# Type Error Source Audit

Read-only comparison of completed wasm1044 error contexts with current source. A changed context is not proof of a fix; fresh strict compilation remains required.

- E0425 ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs; unchanged context: true

    let retained_bytes = puzzle3d_window_transient_retained_bytes(transient).ok_or_else(|| "Puzzle 3D window transient footprint overflowed".to_string())?;

- E0277 ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs; unchanged context: true

        config: Box<CadConfig>,

- E0433 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: false

        Some(Box::new(semio_framework_plugin::NoTransientStoreDisposer::new()))

- E0433 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs; unchanged context: false

        Some(Box::new(semio_framework_plugin::NoTransientStoreDisposer::new()))

- E0425 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs; unchanged context: true

pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, NoConfigMutation> {

- E0425 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏁️flow-eval-resolve/🦀️.rs; unchanged context: true

pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, NoConfigMutation> {

- E0425 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs; unchanged context: true

pub fn evaluate_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &mut FlowEvalSession) -> Emit<FlowMutation, NoConfigMutation> {

- E0425 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs; unchanged context: true

fn node_graph_edit_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp], selected_nodes: &[String]) -> Emit<FlowMutation, NoConfigMutation> {

- E0425 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs; unchanged context: true

fn node_graph_edit_result(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, operations: &[FlowNodeGraphEditOp], selected_nodes: &[String]) -> Emit<FlowMutation, NoConfigMutation> {

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: false

                    FlowCommand::FlowEvalTick(command) => flow_eval_tick::handle(&command, &view, &ConfigView { snapshot: &payload.config, window: None }, session),

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs; unchanged context: false

                    FlowCommand::FlowEvalResolve(command) => flow_eval_resolve::handle(&command, &view, &ConfigView { snapshot: &payload.config, window: None }, session),

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connect-media-ports/🦀️.rs; unchanged context: false

    Ok(Emit::mutations(host_operations(doc.snapshot, cfg.snapshot, session, |host| host.connect_ports(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id).is_ok())))

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs; unchanged context: false

    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️disconnect/🦀️.rs; unchanged context: false

    Ok(Emit::mutations(host_operations(doc.snapshot, cfg.snapshot, session, |host| host.disconnect(&payload.synapse_id).is_ok())))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

    Ok(Emit { config_mutations: vec![NoConfigMutation::SetDuplicateWidgetProgress { json }], effects: vec![queue(&step)], ui_scope: UiDirtyScope::Full, ..Default::default() })

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

        config_mutations: vec![NoConfigMutation::SetDuplicateWidgetProgress { json: String::new() }],

- E0609 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

    if cfg.snapshot.duplicate_widget_progress_json.len() > MAX_CHECKPOINT_BYTES {

- E0609 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

    if dsl::os_pack::json::from_json_str::<DuplicateWidgetStep>(&cfg.snapshot.duplicate_widget_progress_json).ok().as_ref() != Some(payload)

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

        return Ok(Emit { config_mutations: vec![NoConfigMutation::CancelDuplicateWidget { generation: payload.generation }], ..Default::default() });

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

        SearchOutcome::Cancel => Ok(Emit { config_mutations: vec![NoConfigMutation::SetDuplicateWidgetProgress { json: String::new() }], ..Default::default() }),

- E0609 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

    if let Some(generation) = checkpoint_generation(&cfg.snapshot.duplicate_widget_progress_json) {

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs; unchanged context: false

        emit.config_mutations.insert(0, NoConfigMutation::CancelDuplicateWidget { generation });

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs; unchanged context: false

    let mut host = host_from_snapshot(doc.snapshot, cfg.snapshot, session);

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs; unchanged context: false

    match focus_selection_camera(doc.snapshot, cfg.snapshot, session, &nodes) {

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs; unchanged context: false

        Some(camera) => Ok(Emit::config(vec![NoConfigMutation::SetCamera { camera }])),

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs; unchanged context: false

    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetCamera { camera: payload.camera.clone() }]))

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-widget/🦀️.rs; unchanged context: false

    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| host.remove_widget(target_id).is_ok());

- E0308 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️.rs; unchanged context: false

    host_operations(doc.snapshot, cfg.snapshot, session, |host| host.reorganize(REORGANIZE_OPTIONS_JSON).is_ok())

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-extension-action/🦀️.rs; unchanged context: false

    if !cfg.snapshot.automation_enabled().get(*id).copied().unwrap_or(false) {

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-catalogue-sections/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetCatalogueSections { sections_json: payload.sections_json.clone() }]))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️set-grid-factor/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetGridFactor { value: payload.value.clamp(0.5, 50.0) }]))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetGridSnapEnabled { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_snap_enabled) }]))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetGridVisible { value: payload.pressed.unwrap_or(!cfg.snapshot.grid_visible) }]))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs; unchanged context: false

        Ok(Emit::config(vec![NoConfigMutation::SetLodMode { value: payload.value.clone() }]))

- E0609 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs; unchanged context: false

    let mut next = cfg.snapshot.preview_off_node_ids.clone();

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetPreviewOff { node_ids: next }]))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-proximity-distance/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetProximityDistance { value: payload.value.max(0.0) }]))

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs; unchanged context: false

    let mut map = cfg.snapshot.automation_enabled();

- E0599 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs; unchanged context: false

    Ok(Emit::config(vec![NoConfigMutation::SetAutomationEnabled { json: serde_json::to_string(&map).unwrap_or_default() }]))

- E0277 ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs; unchanged context: false

            return match preflight.close_step(maximum_bytes)? {
