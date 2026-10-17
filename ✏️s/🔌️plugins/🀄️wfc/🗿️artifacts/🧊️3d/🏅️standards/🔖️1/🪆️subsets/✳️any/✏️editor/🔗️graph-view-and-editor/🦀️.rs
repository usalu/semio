//! 🔗️ Graph view and artifact editor of the WFC 3D editor: the slot graph view, the editor type with its `ArtifactEditor` implementation, command emission and the body renderer. See [`super`].

use super::*;

//#region 🔖️GraphView
/// 🕸️ The `wfc-graph` window's whole view of this document — the projection lives HERE, not in the
/// window, which is what lets `wfc2d` and `wfc3d` share one window file byte for byte. `z` is
/// deliberately dropped: the graph canvas is a 2d plan of an arbitrary graph, and the third axis is
/// the preview pane's business.
pub struct Wfc3dGraphView<'a>(pub &'a Wfc3dSnapshot);

impl graph::SlotGraphView for Wfc3dGraphView<'_> {
    fn graph_slots(&self) -> Vec<graph::GraphSlotView> {
        self.0
            .slots
            .iter()
            .map(|slot| graph::GraphSlotView {
                id: slot.id.clone(),
                x: slot.x * WFC_3D_GRAPH_UNIT,
                y: slot.y * WFC_3D_GRAPH_UNIT,
                width: slot.width * WFC_3D_GRAPH_UNIT,
                height: slot.height * WFC_3D_GRAPH_UNIT,
                pinned_tile_id: slot.pinned_tile_id.clone(),
            })
            .collect()
    }
    fn graph_edges(&self) -> Vec<graph::GraphEdgeView> {
        self.0.edges.iter().map(|edge| graph::GraphEdgeView { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone(), relation: edge.relation.clone() }).collect()
    }
}
//#endregion 🔖️GraphView

//#region 🔖️Editor
#[derive(Default, Clone, Copy)]
pub struct Wfc3dEditor;

impl ArtifactEditor for Wfc3dEditor {
    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        use semio_framework_ui_locale::LocalizedLabel;
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 4]> = std::sync::LazyLock::new(|| {
            [
            ("wfc3d.tile.unknown-pin", LocalizedLabel::native("This pin does not exist.", "Diese Fixierung existiert nicht.")),
            ("wfc3d.example.unknown", LocalizedLabel::native("This example does not exist.", "Dieses Beispiel existiert nicht.")),
            ("wfc3d.action.unknown", LocalizedLabel::native("This action is not known.", "Diese Aktion ist unbekannt.")),
            ("wfc3d.retained.extent", LocalizedLabel::native("The document is too large for this action.", "Das Dokument ist für diese Aktion zu groß.")),
            ]
        });
        &*NOTICES
    }

    type Snapshot = Wfc3dSnapshot;
    type Mutation = Wfc3dMutation;
    type Config = Wfc3dConfig;
    type ConfigMutation = Wfc3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = Wfc3dTransient;
    type TransientMutation = Wfc3dTransientMutation;
    type Command = Wfc3dEditorCommand;

    const DIALECT: Dialect = WFC3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC3D_DOCUMENT_SCHEMA;

    /// 👥️🫧️ The close ladder retires the presence and transient ROOTS through installed factories and
    /// refuses to teardown without them; this editor carries no presence at all, so the `No*` factories
    /// are its exact terminal.
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
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

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::bounded_transient_store_disposer::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    /// 📬️ The document lane's one-item retained preparation. Without one, EVERY route declaring
    /// `ArtifactToolPublicationLane::Artifact` is registered with an unsupported publication contract
    /// and stays dispatch-dead however it is classified.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// 📬️ The same, for the pane config the camera and armed-tile verbs publish into.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    fn initial_snapshot() -> Wfc3dSnapshot {
        crate::examples::two_room_corridor::snapshot()
    }

    fn command_id(command: &Self::Command) -> &'static str {
        wfc3d_command_id(command)
    }

    /// 🕹️ The `slot` domain's addressable ids — the framework validates every selection write against
    /// this, so a node the canvas picks is only ever selectable while the document still declares it.
    /// Flat: adjacency is a peer relation, never a parent one.
    fn interaction_topology(doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let ordered = doc.snapshot.slots.iter().map(|slot| protocol::TopologyNode { id: slot.id.clone(), granularity: "slot".into(), parent: None }).collect();
        protocol::InteractionTopology { domains: [(WFC_3D_INTERACTION_GRAPH.to_string(), protocol::DomainTopology { ordered })].into_iter().collect() }
    
})())
}

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Wfc3dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !WFC_3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        let tool_id = wfc3d_command_id(&request.command);
        if tool_id != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "WFC 3D command does not match its exact registered tool"));
        }
        if wfc3d_retained_extent(&request.command, &request.snapshot).is_none() {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.retained.extent"), "WFC 3D bounded route exceeded its declared work extent"));
        }
        let operation_context = semio_framework_plugin::AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            retained: request.retained,
            authoring_seed: request.authoring_seed.clone(),
        };
        let work: Box<dyn semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Self>>> = Box::new(Wfc3dCommandWork::new(tool_id));
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            wfc3d_command_id,
            WFC_3D_RETAINED_RAW_BYTES,
            WFC_3D_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Wfc3dEditor>,
        owner_file: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.wfc.wfc3d@1/*#editor",
        artifact_schema: "s.wfc.wfc3d",
        factory: "Wfc3dRetainedCommandJobFactory",
        factory_type: Wfc3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "change-seed", "create-slot", "delete-slot", "move-slot", "resize-slot", "connect-slots", "disconnect-slots", "pin-slot", "unpin-slot",
            "create-tile", "delete-tile", "change-tile-weight", "change-tile-media", "create-rule", "delete-rule",
            "change-camera", "change-active-tile", "solve", "commit-fill", "setActiveExample", "nodeGraphEdit", "nodeGraphViewport"
        ]
    }

    /// 🎯️ Maps the shell's stringly `{action, args}` wire onto `Wfc3dEditorCommand`. The trait default
    /// REFUSES every app action ("dispatched exclusively through the typed command channel"), which is
    /// why — before this bridge existed — every Actions-pane row, the navbar example picker and every
    /// canvas gesture answered `dispatch-failed` in the live playground. Each key below is the
    /// `ActionArgDef.id` that action declares.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        let number = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
        let flag = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
        let viewport = |key: &str| args.and_then(|value| value.get("viewport")).and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
        match action {
            "setActiveExample" => Ok(Wfc3dEditorCommand::SetActiveExample { example_id: text("exampleId").or_else(|| text("id")).unwrap_or_default() }),
            "nodeGraphEdit" => Ok(Wfc3dEditorCommand::NodeGraphEdit { operations_json: graph_operations_json(args) }),
            "nodeGraphViewport" => Ok(Wfc3dEditorCommand::NodeGraphViewport {
                x: viewport("x").or_else(|| number("x")).unwrap_or_default(),
                y: viewport("y").or_else(|| number("y")).unwrap_or_default(),
                zoom: viewport("zoom").or_else(|| number("zoom")).filter(|zoom| *zoom > 0.0).unwrap_or(1.0),
            }),
            "change-seed" => Ok(Wfc3dEditorCommand::ChangeSeed { seed: number("seed").filter(|seed| *seed >= 0.0).unwrap_or_default() as u64 }),
            "create-slot" => Ok(Wfc3dEditorCommand::CreateSlot {
                id: text("id").unwrap_or_default(),
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                z: number("z").unwrap_or_default(),
                width: number("width").unwrap_or(1.0),
                height: number("height").unwrap_or(1.0),
                depth: number("depth").unwrap_or(1.0),
            }),
            "delete-slot" => Ok(Wfc3dEditorCommand::DeleteSlot { id: text("id").unwrap_or_default() }),
            "move-slot" => Ok(Wfc3dEditorCommand::MoveSlot { id: text("id").unwrap_or_default(), x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), z: number("z") }),
            "resize-slot" => Ok(Wfc3dEditorCommand::ResizeSlot { id: text("id").unwrap_or_default(), width: number("width").unwrap_or(1.0), height: number("height").unwrap_or(1.0), depth: number("depth") }),
            "connect-slots" => Ok(Wfc3dEditorCommand::ConnectSlots {
                id: text("id").unwrap_or_default(),
                from_slot_id: text("fromSlotId").unwrap_or_default(),
                to_slot_id: text("toSlotId").unwrap_or_default(),
                relation: text("relation").unwrap_or_default(),
            }),
            "disconnect-slots" => Ok(Wfc3dEditorCommand::DisconnectSlots { id: text("id").unwrap_or_default() }),
            "pin-slot" => Ok(Wfc3dEditorCommand::PinSlot { id: text("id").unwrap_or_default(), tile_id: text("tileId").unwrap_or_default() }),
            "unpin-slot" => Ok(Wfc3dEditorCommand::UnpinSlot { id: text("id").unwrap_or_default() }),
            "create-tile" => Ok(Wfc3dEditorCommand::CreateTile { id: text("id").unwrap_or_default(), label: text("label").filter(|label| !label.is_empty()), weight: number("weight").unwrap_or(1.0) }),
            "delete-tile" => Ok(Wfc3dEditorCommand::DeleteTile { id: text("id").unwrap_or_default() }),
            "change-tile-weight" => Ok(Wfc3dEditorCommand::ChangeTileWeight { tile_id: text("tileId").unwrap_or_default(), weight: number("weight").unwrap_or(1.0) }),
            "change-tile-media" => Ok(Wfc3dEditorCommand::ChangeTileMedia { tile_id: text("tileId").unwrap_or_default() }),
            "create-rule" => Ok(Wfc3dEditorCommand::CreateRule {
                id: text("id").unwrap_or_default(),
                tile_a_id: text("tileAId").unwrap_or_default(),
                tile_b_id: text("tileBId").unwrap_or_default(),
                relation: text("relation").filter(|relation| !relation.is_empty()),
                allowed: flag("allowed").unwrap_or(true),
            }),
            "delete-rule" => Ok(Wfc3dEditorCommand::DeleteRule { id: text("id").unwrap_or_default() }),
            "change-camera" => Ok(Wfc3dEditorCommand::ChangeCamera { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), zoom: number("zoom").filter(|zoom| *zoom > 0.0).unwrap_or(1.0) }),
            "change-active-tile" => Ok(Wfc3dEditorCommand::ChangeActiveTile { tile_id: text("tileId").unwrap_or_default() }),
            "solve" => Ok(Wfc3dEditorCommand::Solve),
            action if action == fill::COMMIT_FILL_ACTION_ID => {
                let payload_json = text("payloadJson").or_else(|| text("payload_json")).or_else(|| text("value")).ok_or_else(|| Fault::from("wfc3d-commit-fill-args"))?;
                Ok(Wfc3dEditorCommand::CommitFill { payload_json })
            }
            other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.action.unknown"), format!("wfc3d declares no action '{other}'"))),
        }
    }

    /// ✏️ Dispatches straight onto the schema tree's own mutation builders. A pin gesture reads the
    /// PANE's armed tile rather than a global one, so two panes can pin different tiles.
    fn handle(
        command: &Self::Command,
        doc: &ArtifactView<'_, Self::Snapshot>,
        cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation>, Fault> {
        command_emit(command, doc.snapshot, cfg.snapshot, doc.operation_optional().map(|operation| operation.authoring_seed.as_str()).unwrap_or_default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc.snapshot, cfg.snapshot, &Wfc3dTransient::default(), None)
    }

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
        render_body(body_key, doc.snapshot, cfg.snapshot, transient.snapshot, doc.tool_run())

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<ToolRunJob>, Fault> {
        fill::build_tool_run_job(request)
    }
}

/// ✏️ The whole command→mutation mapping, as a PURE function of the document and the pane config —
/// `handle` is the thin `ArtifactView`/`ConfigView` adapter over it. Split out so the mapping is
/// reachable from a unit test: `ArtifactView`'s own fields are private, so nothing outside the
/// framework can build one. A node-graph gesture commits through the drag tool as ONE transaction minted from
/// `authoring_seed` (the admission's; empty for a view without command authority, which publishes plainly).
pub fn command_emit(command: &Wfc3dEditorCommand, document: &Wfc3dSnapshot, config: &Wfc3dConfig, authoring_seed: &str) -> Result<Emit<Wfc3dMutation, Wfc3dConfigMutation>, Fault> {
    {
        let mutation = match command {
            Wfc3dEditorCommand::ChangeCamera { x, y, zoom } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeCamera(crate::editor::wfc3d::config::ChangeCamera { x: *x, y: *y, zoom: *zoom })];
                return Ok(Emit::config(emitted));
            }
            Wfc3dEditorCommand::NodeGraphViewport { x, y, zoom } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeCamera(crate::editor::wfc3d::config::ChangeCamera { x: *x, y: *y, zoom: if *zoom > 0.0 { *zoom } else { 1.0 } })];
                return Ok(Emit::config(emitted));
            }
            Wfc3dEditorCommand::ChangeActiveTile { tile_id } => {
                let emitted = vec![Wfc3dConfigMutation::ChangeActiveTile(crate::editor::wfc3d::config::ChangeActiveTile { tile_id: tile_id.clone() })];
                return Ok(Emit::config(emitted));
            }
            Wfc3dEditorCommand::Solve => {
                return Ok(Emit { effects: fill::start_fill_effects(), ..Default::default() });
            }
            Wfc3dEditorCommand::CommitFill { .. } => {
                return Ok(Emit::default());
            }
            Wfc3dEditorCommand::SetActiveExample { example_id } => {
                let next = example_snapshot(example_id)?;
                return Ok(Emit::effect(load_document_effect(&next)));
            }
            Wfc3dEditorCommand::NodeGraphEdit { operations_json } => {
                let gesture = graph_edit_gesture(document, operations_json)?;
                return Ok(drag::wfc3d_drag_tool(WFC_3D_NODE_GRAPH_EDIT, authoring_seed, document, gesture.prepared, &gesture.records, WFC_3D_GRAPH_UNIT).into());
            }
            Wfc3dEditorCommand::ChangeSeed { seed } => change_seed(*seed),
            Wfc3dEditorCommand::CreateSlot { id, x, y, z, width, height, depth } => create_slot(
                canonical_slot_index(document, id),
                Slot3d { id: id.clone(), x: *x, y: *y, z: *z, width: positive_or(*width), height: positive_or(*height), depth: positive_or(*depth), pinned_tile_id: None },
            ),
            Wfc3dEditorCommand::DeleteSlot { id } => delete_slot(id.clone()),
            Wfc3dEditorCommand::MoveSlot { id, x, y, z } => {
                let authored = document.slots.iter().find(|slot| &slot.id == id).map(|slot| slot.z).unwrap_or_default();
                move_slot(id.clone(), *x, *y, z.unwrap_or(authored))
            }
            Wfc3dEditorCommand::ResizeSlot { id, width, height, depth } => {
                let authored = document.slots.iter().find(|slot| &slot.id == id).map_or(1.0, |slot| slot.depth);
                resize_slot(id.clone(), positive_or(*width), positive_or(*height), positive_or(depth.unwrap_or(authored)))
            }
            Wfc3dEditorCommand::ConnectSlots { id, from_slot_id, to_slot_id, relation } => connect_slots(
                canonical_edge_index(document, id),
                SlotEdge { id: id.clone(), from_slot_id: from_slot_id.clone(), to_slot_id: to_slot_id.clone(), relation: if relation.is_empty() { WFC_3D_DEFAULT_RELATION.into() } else { relation.clone() } },
            ),
            Wfc3dEditorCommand::DisconnectSlots { id } => disconnect_slots(id.clone()),
            Wfc3dEditorCommand::PinSlot { id, tile_id } => {
                let tile = if tile_id.is_empty() { wfc3d_active_tile_id(config, document) } else { Some(tile_id.clone()) };
                let Some(tile) = tile else { return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.tile.unknown-pin"), "No tile is armed and the document declares none.")) };
                pin_slot(id.clone(), tile.clone())
            }
            Wfc3dEditorCommand::UnpinSlot { id } => unpin_slot(id.clone()),
            Wfc3dEditorCommand::CreateTile { id, label, weight } => create_tile(canonical_tile_index(document, id), Tile { id: id.clone(), label: label.clone(), weight: positive_or(*weight), media: TileMedia3d::default() }),
            Wfc3dEditorCommand::DeleteTile { id } => delete_tile(id.clone()),
            Wfc3dEditorCommand::ChangeTileWeight { tile_id, weight } => change_tile_weight(tile_id.clone(), positive_or(*weight)),
            Wfc3dEditorCommand::ChangeTileMedia { tile_id } => change_tile_media(tile_id.clone(), crate::unit_box_media(None)),
            Wfc3dEditorCommand::CreateRule { id, tile_a_id, tile_b_id, relation, allowed } => create_rule(canonical_rule_index(document, id), GraphRule { id: id.clone(), tile_a_id: tile_a_id.clone(), tile_b_id: tile_b_id.clone(), relation: relation.clone(), allowed: *allowed }),
            Wfc3dEditorCommand::DeleteRule { id } => delete_rule(id.clone()),
        };
        Ok(Emit::mutations(vec![mutation]))
    }
}

/// 🖼️ The whole per-window render, as a PURE function of the document and the pane config — the same
/// split, for the same reason.
/// 🎥️ The preview frames the document itself and deliberately does NOT read `camera_zoom`:
/// that field is the GRAPH pane's viewport, and the two panes share one config instance, so
/// feeding it here made a pan or a wheel on the graph canvas warp the 3d view (measured live:
/// one graph pan put the preview inside a tile). `camera_json`'s own framing distance is the
/// pose authority.
pub fn render_body(body_key: &str, document: &Wfc3dSnapshot, config: &Wfc3dConfig, transient: &Wfc3dTransient, tool_run: Option<&ToolRunView>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        graph::WFC_GRAPH_BODY => {
            let camera = graph::GraphCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom };
            graph::render(&Wfc3dGraphView(document), camera, &[]).map(semio_framework_plugin::built_to_component_tree)
        }
        preview::WFC_3D_PREVIEW_BODY => {
            let fill = preview::live_fill_payload(tool_run);
            let paint = fill.as_ref().map(|payload| {
                let mut paint = payload.clone().into_transient();
                for event in &payload.trace {
                    paint.assignments.retain(|assignment| assignment.slot_id != event.slot_id);
                    paint.assignments.push(crate::editor::wfc3d::transient::Wfc3dAssignment { slot_id: event.slot_id.clone(), tile_id: event.tile_id.clone() });
                }
                paint
            }).unwrap_or_else(|| transient.clone());
            preview::render(document, &paint, 1.0).map(semio_framework_plugin::built_to_component_tree)
        }
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}
//#endregion 🔖️Editor
