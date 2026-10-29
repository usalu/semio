//! 🧭️ Dispatch and artifact editor: the command dispatch, the render body and the `ArtifactEditor` implementation with its transient render. See [`super`].

use super::*;

//#region 🔖️Editor
/// ✏️ The whole command→emission rule set, as a free function over plain values. `ArtifactEditor::handle`
/// only adapts the framework's view bundle onto this: `InteractionView` has `pub(crate)` fields, so
/// this crate's own tests cannot construct one and would otherwise have no way to exercise dispatch
/// at all (cad's own test-harness precedent, `📐️cad/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`).
///
/// A pin gesture reads the PANE's armed tile rather than a global one, so two panes can pin different
/// tiles. Camera and armed-tile verbs go to the pane config and never touch the document lanes. A node-graph gesture
/// commits through the drag tool as ONE transaction minted from `authoring_seed` (the admission's; empty for a view
/// without command authority, which publishes plainly).
pub fn dispatch(command: &Wfc2dEditorCommand, document: &Wfc2dSnapshot, config: &Wfc2dConfig, authoring_seed: &str) -> Result<Emit<Wfc2dMutation, Wfc2dConfigMutation>, Fault> {
    let mutation = match command {
        Wfc2dEditorCommand::ChangeCamera { x, y, zoom } => {
            let mutations = vec![Wfc2dConfigMutation::ChangeCamera(crate::editor::wfc2d::config::ChangeCamera { x: *x, y: *y, zoom: *zoom })];
            return Ok(Emit::config(mutations));
        }
        Wfc2dEditorCommand::ChangeActiveTile { tile_id } => {
            let mutations = vec![Wfc2dConfigMutation::ChangeActiveTile(crate::editor::wfc2d::config::ChangeActiveTile { tile_id: tile_id.clone() })];
            return Ok(Emit::config(mutations));
        }
        Wfc2dEditorCommand::Solve => {
            return Ok(Emit { effects: fill::start_fill_effects(), ..Default::default() });
        }
        Wfc2dEditorCommand::CommitFill { .. } => {
            return Ok(Emit::default());
        }
        Wfc2dEditorCommand::NodeGraphEdit { operations_json } => {
            let gesture = wfc2d_node_graph_edit(document, operations_json)?;
            return Ok(drag::wfc2d_drag_tool(WFC_2D_NODE_GRAPH_EDIT, authoring_seed, document, gesture.prepared, &gesture.records, WFC_2D_GRAPH_VIEW_SCALE).into());
        }
        Wfc2dEditorCommand::SetActiveExample { example_id } => {
            let Some(next) = wfc2d_example_document(example_id) else {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.example.unknown"), format!("wfc 2d has no example '{example_id}'")));
            };
            return Ok(Emit::effect(wfc2d_load_document_effect(&next)));
        }
        Wfc2dEditorCommand::ChangeSeed { seed } => change_seed(*seed),
        Wfc2dEditorCommand::CreateSlot { id, x, y, width, height } => {
            require_fresh(document.slots.iter().any(|slot| &slot.id == id), "slot", id)?;
            create_slot(Wfc2dSlot { id: id.clone(), x: *x, y: *y, width: *width, height: *height, pinned_tile_id: None })
        }
        Wfc2dEditorCommand::DeleteSlot { id } => {
            require_slot(document, id)?;
            delete_slot(id.clone())
        }
        Wfc2dEditorCommand::MoveSlot { id, x, y } => {
            require_slot(document, id)?;
            move_slot(id.clone(), *x, *y)
        }
        Wfc2dEditorCommand::ResizeSlot { id, width, height } => {
            require_slot(document, id)?;
            resize_slot(id.clone(), *width, *height)
        }
        Wfc2dEditorCommand::ConnectSlots { id, from_slot_id, to_slot_id, relation } => {
            require_slot(document, from_slot_id)?;
            require_slot(document, to_slot_id)?;
            require_fresh(document.edges.iter().any(|edge| &edge.id == id), "adjacency", id)?;
            connect_slots(Wfc2dSlotEdge {
                id: id.clone(),
                from_slot_id: from_slot_id.clone(),
                to_slot_id: to_slot_id.clone(),
                relation: if relation.is_empty() { crate::schema::snapshot::WFC_2D_DEFAULT_RELATION.into() } else { relation.clone() },
            })
        }
        Wfc2dEditorCommand::DisconnectSlots { id } => {
            require_edge(document, id)?;
            disconnect_slots(id.clone())
        }
        Wfc2dEditorCommand::PinSlot { id, tile_id } => {
            require_slot(document, id)?;
            let tile = if tile_id.is_empty() { wfc2d_active_tile_id(config, document) } else { Some(tile_id.clone()) };
            let Some(tile) = tile else {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::Plugin, semio_framework_plugin::FaultCode::new("wfc2d.tile.unknown-pin"), "No tile is armed and the document declares none."));
            };
            require_tile(document, &tile)?;
            pin_slot(id.clone(), tile.clone())
        }
        Wfc2dEditorCommand::UnpinSlot { id } => {
            require_slot(document, id)?;
            unpin_slot(id.clone())
        }
        Wfc2dEditorCommand::CreateTile { id, label, weight } => {
            require_fresh(document.tiles.iter().any(|tile| &tile.id == id), "tile", id)?;
            create_tile(Wfc2dTile { id: id.clone(), label: label.clone(), weight: *weight, media: Wfc2dTileMedia::default() })
        }
        Wfc2dEditorCommand::DeleteTile { id } => {
            require_tile(document, id)?;
            delete_tile(id.clone())
        }
        Wfc2dEditorCommand::ChangeTileWeight { tile_id, weight } => {
            require_tile(document, tile_id)?;
            change_tile_weight(tile_id.clone(), *weight)
        }
        Wfc2dEditorCommand::ChangeTileMedia { tile_id } => {
            require_tile(document, tile_id)?;
            change_tile_media(tile_id.clone(), Wfc2dTileMedia::default())
        }
        Wfc2dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, relation, allowed } => {
            require_tile(document, tile_a_id)?;
            require_tile(document, tile_b_id)?;
            require_fresh(document.rules.iter().any(|rule| &rule.id == id), "rule", id)?;
            create_rule(Wfc2dRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), relation: relation.clone(), allowed: *allowed })
        }
        Wfc2dEditorCommand::DeleteRule { id } => {
            require_rule(document, id)?;
            delete_rule(id.clone())
        }
    };
    Ok(Emit::mutations(vec![mutation]))
}

/// 🖼️ The whole render rule set, likewise free of the framework's view bundle so a test can call it.
pub fn render_body(
    body_key: &str,
    document: &Wfc2dSnapshot,
    config: &Wfc2dConfig,
    transient: &Wfc2dTransient,
    tool_run: Option<&semio_framework_plugin::ToolRunView>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        graph::WFC_GRAPH_BODY => {
            let camera = graph::GraphCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom };
            graph::render(&Wfc2dGraphView(document), camera, &[]).map(semio_framework_plugin::built_to_component_tree)
        }
        preview::WFC_2D_PREVIEW_BODY => preview::render(document, transient, tool_run, config.camera_x, config.camera_y, config.camera_zoom).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

impl ArtifactEditor for Wfc2dEditor {
    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        use semio_framework_ui_locale::LocalizedLabel;
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 8]> = std::sync::LazyLock::new(|| {
            [
            ("wfc2d.tile.unknown-tile", LocalizedLabel::native("This tile does not exist.", "Diese Kachel existiert nicht.")),
            ("wfc2d.tile.unknown-pin", LocalizedLabel::native("This pin does not exist.", "Diese Fixierung existiert nicht.")),
            ("wfc2d.slot.unknown-slot", LocalizedLabel::native("This slot does not exist.", "Dieser Steckplatz existiert nicht.")),
            ("wfc2d.rule.unknown-rule", LocalizedLabel::native("This rule does not exist.", "Diese Regel existiert nicht.")),
            ("wfc2d.edge.unknown-edge", LocalizedLabel::native("This edge does not exist.", "Diese Kante existiert nicht.")),
            ("wfc2d.id.taken", LocalizedLabel::native("This id is already in use.", "Diese Kennung ist bereits vergeben.")),
            ("wfc2d.example.unknown", LocalizedLabel::native("This example does not exist.", "Dieses Beispiel existiert nicht.")),
            ("wfc2d.retained.extent", LocalizedLabel::native("The document is too large for this action.", "Das Dokument ist für diese Aktion zu groß.")),
            ]
        });
        &*NOTICES
    }

    type Snapshot = Wfc2dSnapshot;
    type Mutation = Wfc2dMutation;
    type Config = Wfc2dConfig;
    type ConfigMutation = Wfc2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = Wfc2dTransient;
    type TransientMutation = Wfc2dTransientMutation;
    type Command = Wfc2dEditorCommand;

    const DIALECT: Dialect = WFC_2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_2D_DOCUMENT_SCHEMA;

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    /// 🗃️ The bounded retirement catalog every store lane is released THROUGH: a store built without
    /// owners answers `artifact store has no owner-supplied bounded disposer` the moment the close
    /// ladder reaches it, so the owners and the disposer below are one declaration in two halves.



    /// ♻️ The instance close ladder walks one owned store lane per stage and faults the whole close
    /// with `interactive-job.close-owned-disposer-missing` the moment a lane answers `None`, so an
    /// editor must declare how EVERY store it types is released, not only the ones it edits.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    /// 🫧️ `Wfc2dTransient` carries the solved assignment, so its lane needs the BOUNDED disposer that
    /// retires a real root, not the `NoTransient` shim.
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    /// 📬️ The document lane's one-item retained preparation. Without one, EVERY route declaring
    /// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract
    /// and stays dispatch-dead however it is classified.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// 📬️ The same, for the pane config the camera and armed-tile verbs publish into.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    /// 🫧️ The transient lane's own preparation and root retirement — `solve` declares
    /// `ArtifactToolPublicationLane::Transient`, and the lane is refused as unsupported unless BOTH
    /// exist.
    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        fill::build_tool_run_job(request)
    }

    fn reference_birth()->Option<fn()->Result<Wfc2dSnapshot,semio_framework_plugin::Fault>>{Some(||{
        Ok(crate::examples::two_room_corridor::document())
    })}

    fn command_id(command: &Self::Command) -> &'static str {
        wfc2d_command_id(command)
    }

    /// 🕹️ The `slot` domain's addressable ids — the framework validates every selection write against
    /// this, so a node the canvas picks is only ever selectable while the document still declares it.
    /// Flat: adjacency is a peer relation, never a parent one.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let ordered = doc.snapshot.slots.iter().map(|slot| TopologyNode { id: slot.id.clone(), granularity: "slot".into(), parent: None }).collect();
        protocol::InteractionTopology { domains: [("slot".to_string(), protocol::DomainTopology { ordered })].into_iter().collect() }
    
})())
}

    /// 🌉️ The args bridge every UI dispatch crosses — typed against `semio_framework_value::DslValue`, never
    /// `serde_json::Value`: without this override the trait's own default answers
    /// `app.command.unsupported` and every button in every pane is inert.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        Ok(match action {
            "change-seed" => Wfc2dEditorCommand::ChangeSeed { seed: arg_u64(args, "seed", 0) },
            "create-slot" => Wfc2dEditorCommand::CreateSlot {
                id: arg_string(args, "id"),
                x: arg_f64(args, "x", 0.0),
                y: arg_f64(args, "y", 0.0),
                width: arg_f64(args, "width", 1.0),
                height: arg_f64(args, "height", 1.0),
            },
            "delete-slot" => Wfc2dEditorCommand::DeleteSlot { id: arg_string(args, "id") },
            "move-slot" => Wfc2dEditorCommand::MoveSlot { id: arg_string(args, "id"), x: arg_f64(args, "x", 0.0), y: arg_f64(args, "y", 0.0) },
            "resize-slot" => Wfc2dEditorCommand::ResizeSlot { id: arg_string(args, "id"), width: arg_f64(args, "width", 1.0), height: arg_f64(args, "height", 1.0) },
            "connect-slots" => Wfc2dEditorCommand::ConnectSlots {
                id: arg_string(args, "id"),
                from_slot_id: arg_string(args, "fromSlotId"),
                to_slot_id: arg_string(args, "toSlotId"),
                relation: arg_string(args, "relation"),
            },
            "disconnect-slots" => Wfc2dEditorCommand::DisconnectSlots { id: arg_string(args, "id") },
            "pin-slot" => Wfc2dEditorCommand::PinSlot { id: arg_string(args, "id"), tile_id: arg_string(args, "tileId") },
            "unpin-slot" => Wfc2dEditorCommand::UnpinSlot { id: arg_string(args, "id") },
            "create-tile" => Wfc2dEditorCommand::CreateTile { id: arg_string(args, "id"), label: arg_optional_string(args, "label"), weight: arg_f64(args, "weight", 1.0) },
            "delete-tile" => Wfc2dEditorCommand::DeleteTile { id: arg_string(args, "id") },
            "change-tile-weight" => Wfc2dEditorCommand::ChangeTileWeight { tile_id: arg_string(args, "tileId"), weight: arg_f64(args, "weight", 1.0) },
            "change-tile-media" => Wfc2dEditorCommand::ChangeTileMedia { tile_id: arg_string(args, "tileId") },
            "create-rule" => Wfc2dEditorCommand::CreateRule {
                id: arg_string(args, "id"),
                tile_a_id: arg_string(args, "tileAId"),
                tile_b_id: arg_string(args, "tileBId"),
                relation: arg_optional_string(args, "relation"),
                allowed: arg_bool(args, "allowed", true),
            },
            "delete-rule" => Wfc2dEditorCommand::DeleteRule { id: arg_string(args, "id") },
            "change-camera" => Wfc2dEditorCommand::ChangeCamera { x: arg_f64(args, "x", 0.0), y: arg_f64(args, "y", 0.0), zoom: arg_f64(args, "zoom", 1.0) },
            "change-active-tile" => Wfc2dEditorCommand::ChangeActiveTile { tile_id: arg_string(args, "tileId") },
            "solve" => Wfc2dEditorCommand::Solve,
            fill::COMMIT_FILL_ACTION_ID => {
                let payload_json = {
                    let camel = arg_string(args, "payloadJson");
                    if !camel.is_empty() {
                        camel
                    } else {
                        let snake = arg_string(args, "payload_json");
                        if !snake.is_empty() {
                            snake
                        } else {
                            arg_string(args, "value")
                        }
                    }
                };
                if payload_json.is_empty() {
                    return Err(Fault::from("wfc2d-commit-fill-args"));
                }
                Wfc2dEditorCommand::CommitFill { payload_json }
            },
            WFC_2D_NODE_GRAPH_EDIT => Wfc2dEditorCommand::NodeGraphEdit { operations_json: arg_string(args, "operations") },
            WFC_2D_SET_ACTIVE_EXAMPLE => {
                let requested = arg_string(args, "exampleId");
                let example_id = if requested.is_empty() { crate::examples::two_room_corridor::ID.to_string() } else { requested };
                Wfc2dEditorCommand::SetActiveExample { example_id }
            }
            other => return Err(Fault::from(format!("wfc2d-unknown-action:{other}"))),
        })
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Wfc2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !WFC_2D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        let tool_id = wfc2d_command_id(&original.command);
        if tool_id != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "WFC 2D command does not match its exact registered tool").into());
        }
        if wfc2d_retained_extent(&original.command, &original.snapshot).is_none() {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "WFC 2D bounded route exceeded its declared work extent").into());
        }
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, wfc2d_command_id, WFC_2D_RETAINED_RAW_BYTES, WFC_2D_RETAINED_WORK_ITEMS, false, |request| Wfc2dCommandWork::new(tool_id))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Wfc2dEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.wfc2d@1/*#editor",
        artifact_schema: "s.wfc.wfc2d",
        factory: "Wfc2dRetainedCommandJobFactory",
        factory_type: Wfc2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "change-seed", "create-slot", "delete-slot", "move-slot", "resize-slot", "connect-slots", "disconnect-slots", "pin-slot", "unpin-slot",
            "create-tile", "delete-tile", "change-tile-weight", "change-tile-media", "create-rule", "delete-rule",
            "change-camera", "change-active-tile", "solve", "commit-fill", "setActiveExample", "nodeGraphEdit"
        ]
    }

    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation>, Fault> {
        dispatch(command, doc.snapshot, cfg.snapshot, doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default())
    }

    /// 🫧️ The bare `render` is the trait's transient-LESS entry point: its signature carries no
    /// transient lane at all, so this path can only paint what the DOCUMENT says (authored pins).
    /// Every host request goes through `render_with_request_context` below, which does carry the
    /// lane; this exists for callers that have no request context to offer.
    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot, &Wfc2dTransient::default(), None)
    }

    /// 🫧️ The HOST-FACING render: the framework hands the app-local transient store in here, so this
    /// is where the solved assignment actually reaches `wfc-2d-preview`. Delegates to the free
    /// `render_with_transient` below so a test can drive the same code without an `InteractionView`
    /// (whose fields are `pub(crate)` in the framework crate and cannot be built from here).
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        render_with_transient(body_key, doc, cfg, transient)

        })();
        rendered.map(Some).map_err(Into::into)
        }
}

/// 🫧️ The exact projection `render_with_request_context` performs, minus the two arguments a test
/// cannot construct. `transient.snapshot` IS the app-local `Wfc2dTransient` the solve job publishes
/// into, so the preview window paints the LAST SOLVE here and authored pins only where the solve has
/// not reached yet.
pub fn render_with_transient(
    body_key: &str,
    doc: &ArtifactView<'_, Wfc2dSnapshot>,
    cfg: &ConfigView<'_, Wfc2dConfig>,
    transient: &semio_framework_plugin::TransientView<'_, Wfc2dTransient>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    render_body(body_key, doc.snapshot, cfg.snapshot, transient.snapshot, doc.tool_run())
}
//#endregion 🔖️Editor
