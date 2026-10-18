//! 🧰️ Direct store command work: the artifact command work implementation of the direct store lane with its stepping, restore and close. See [`super`].

use super::*;

//#region 🧵️DirectStoreLaneRoutes
impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowDirectStoreWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if command.command_id() != self.tool_id || !FLOW_DIRECT_STORE_TOOL_IDS.contains(&self.tool_id) {
            return None;
        }
        match command {
            FlowCommand::SetPreviewOff(payload) if payload.ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && payload.ids.iter().map(String::len).fold(0, usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES => {
                Some(payload.ids.len().saturating_mul(FLOW_STORE_MAX_SCENE_ITEMS.max(1)).max(1))
            }
            FlowCommand::SetPreviewOff(_) => None,
            FlowCommand::RemoveWidget(_) => flow_content_child(snapshot, &context?.children).filter(|child| child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| child.nodes.len().max(1)),
            FlowCommand::Disconnect(_) => flow_content_child(snapshot, &context?.children).filter(|child| child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| child.edges.len().max(1)),
            FlowCommand::DeleteSelection(_) => match interaction.selection.get(FLOW_INTERACTION_GRAPH) {
                Some(selection) if selection.ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && selection.ids.iter().map(String::len).fold(0, usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES => {
                    flow_content_child(snapshot, &context?.children).filter(|child| child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS && child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| {
                        selection
                            .ids
                            .iter()
                            .fold(0usize, |extent, target| {
                                extent.saturating_add(if target.starts_with(FLOW_GRAPH_EDGE_TARGET_PREFIX) {
                                    child.edges.len().max(1)
                                } else if target.starts_with(FLOW_GRAPH_NODE_TARGET_PREFIX) {
                                    child.nodes.len().max(1)
                                } else {
                                    1
                                })
                            })
                            .max(1)
                    })
                }
                Some(_) => None,
                None => Some(1),
            },
            FlowCommand::PatchFlowWidgets(payload)
                if payload.widget_ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && payload.widget_ids.iter().map(String::len).fold(payload.field.len().saturating_add(payload.value.len()), usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES =>
            {
                flow_content_child(snapshot, &context?.children).filter(|child| child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| child.nodes.len().saturating_mul(payload.widget_ids.len().max(1)).max(1))
            }
            FlowCommand::PatchFlowWidgets(_) => None,
            FlowCommand::DuplicateWidget(payload) if !payload.widget_id.is_empty() && payload.widget_id.len() <= 256 => {
                let child = flow_content_child(snapshot, &context?.children)?;
                (child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS && child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS)
                    .then_some(child.nodes.len().saturating_add(1).saturating_mul(child.nodes.len().saturating_add(child.edges.len()).saturating_add(2)).max(1))
            }
            FlowCommand::DuplicateWidget(_) => None,
            _ => Some(1),
        }
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        let context = context.ok_or_else(|| Fault::from("flow-window-context-required"))?;
        let view = context.view_state.as_ref().ok_or_else(|| Fault::from("flow-window-view-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        if self.completed || self.closing {
            return Err(Fault::from("flow-retained-direct-work-terminal"));
        }
        if let FlowCommand::DuplicateWidget(payload) = command {
            if payload.widget_id.is_empty() || payload.widget_id.len() > 256 {
                return Err(Fault::from("flow-duplicate-widget-id-capacity"));
            }
            let window = context.window_transient.as_ref();
            if window.is_some_and(|window| view.window_id.as_deref() != Some(window.window_id())) {
                return Err(Fault::from("flow-duplicate-window-transient-stale"));
            }
            let child_id = &snapshot.content.child_id;
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-duplicate-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-duplicate-child-capacity"));
            }
            if self.duplicate_phase == 0 {
                if let Some(node) = child.nodes.get(self.cursor) {
                    if node.id == payload.widget_id {
                        self.duplicate_source = Some(self.cursor);
                        self.duplicate_phase = 1;
                        self.scan_cursor = 0;
                    } else {
                        self.cursor += 1;
                    }
                    return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-source", preview: br#"{"en":"Finding source widget","de":"Quell-Widget wird gesucht"}"# });
                }
                self.completed = true;
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("duplicateWidget found no widget \"{}\"", payload.widget_id)));
            }
            let source_index = self.duplicate_source.ok_or_else(|| Fault::from("flow-duplicate-source-owner"))?;
            let source = child.nodes.get(source_index).filter(|node| node.id == payload.widget_id).ok_or_else(|| Fault::from("flow-duplicate-source-stale"))?;
            let target_id = duplicate_widget_id(&payload.widget_id, self.duplicate_suffix);
            if target_id.len() > 256 {
                return Err(Fault::from("flow-duplicate-target-id-capacity"));
            }
            if self.duplicate_phase == 1 {
                if let Some(node) = child.nodes.get(self.scan_cursor) {
                    self.scan_cursor += 1;
                    if node.id == target_id {
                        self.duplicate_suffix = self.duplicate_suffix.checked_add(1).ok_or_else(|| Fault::from("flow-duplicate-suffix-overflow"))?;
                        self.scan_cursor = 0;
                    }
                    return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-node-collision", preview: br#"{"en":"Checking node identity","de":"Knotenidentitaet wird geprueft"}"# });
                }
                self.duplicate_phase = 2;
                self.scan_cursor = 0;
                return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-edge-start", preview: br#"{"en":"Checking edge identity","de":"Kantenidentitaet wird geprueft"}"# });
            }
            let edge_id = duplicate_edge_id(&payload.widget_id, &target_id);
            if edge_id.len() > 512 {
                return Err(Fault::from("flow-duplicate-edge-id-capacity"));
            }
            if let Some(edge) = child.edges.get(self.scan_cursor) {
                self.scan_cursor += 1;
                if edge.id == edge_id {
                    self.duplicate_suffix = self.duplicate_suffix.checked_add(1).ok_or_else(|| Fault::from("flow-duplicate-suffix-overflow"))?;
                    self.duplicate_phase = 1;
                    self.scan_cursor = 0;
                }
                return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-edge-collision", preview: br#"{"en":"Checking edge identity","de":"Kantenidentitaet wird geprueft"}"# });
            }
            let mut node = source.clone();
            node.id = target_id.clone();
            let edge = FlowEdge { id: edge_id, from: PortRef { node: payload.widget_id.clone(), port: String::new() }, to: PortRef { node: target_id, port: String::new() }, kind: "data".into() };
            let emit = Emit {
                child_preparations: std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot, _>("content", child_id, vec![SemioFlowMutation::InsertNode(insert_node::InsertNode::new(node)), SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(edge))])]),
                ..Default::default()
            };
            self.completed = true;
            let Some(window) = window else { return Ok(ArtifactCommandWorkStep::Complete(emit)) };
            let mut transient = main::transient::from_snapshot(Some(window));
            transient.duplicate_widget_progress_json.clear();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![main::transient::addressed(view, transient)?] },
            });
        }
        if let FlowCommand::FocusSelection(_) = command {
            let (nodes, _) = flow_graph_selection_domains(interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice()));
            let mut next = config.clone();
            let session = FlowEvalSession::new();
            let composed = crate::flow_composed_snapshot(snapshot, &context.children)?;
            let camera = focus_selection_camera(&composed, &config, &session, &nodes);
            session.retire_cold();
            if let Some(camera) = camera {
                next.camera = camera;
            }
            self.completed = true;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: main::config::addressed(view, &config, next)?, ..Default::default() }));
        }
        if matches!(
            command,
            FlowCommand::AddGeneration(_)
                | FlowCommand::RemoveGeneration(_)
                | FlowCommand::SelectGeneration(_)
                | FlowCommand::RenameGeneration(_)
                | FlowCommand::UpdateGenerationValues(_)
        ) {
            let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("flow-generation-window-transient-required"))?;
            if view.window_id.as_deref() != Some(window.window_id()) {
                return Err(Fault::from("flow-generation-window-transient-stale"));
            }
            let current = main::transient::from_snapshot(Some(window));
            let composed = crate::flow_composed_snapshot(snapshot, &context.children)?;
            let mutation = generation_window_transient(command, &composed, &config, &current, view)?;
            self.completed = true;
            let emit = Emit::default();
            let ephemeral = semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: mutation.into_iter().collect() };
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral });
        }
        if matches!(command, FlowCommand::RemoveWidget(_) | FlowCommand::Disconnect(_)) {
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            let (length, matched, target, found) = match command {
                FlowCommand::RemoveWidget(payload) => (child.nodes.len(), child.nodes.get(self.cursor).is_some_and(|node| node.id == payload.widget_id), &payload.widget_id, self.node_ids.get_or_insert_with(Vec::new)),
                FlowCommand::Disconnect(payload) => (child.edges.len(), child.edges.get(self.cursor).is_some_and(|edge| edge.id == payload.synapse_id), &payload.synapse_id, self.edge_ids.get_or_insert_with(Vec::new)),
                _ => return Err(Fault::from("flow-retained-direct-route-mismatch")),
            };
            if self.cursor < length {
                if matched {
                    found.push(target.clone());
                    self.cursor = length;
                } else {
                    self.cursor += 1;
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-direct-artifact-scan-replay", preview: br#"{"en":"Restoring artifact scan","de":"Artefaktsuche wird wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-direct-artifact-scan", preview: br#"{"en":"Scanning one artifact item","de":"Ein Artefaktelement wird geprueft"}"# }
                });
            }
            let (node_ids, edge_ids) = (self.node_ids.take().unwrap_or_default(), self.edge_ids.take().unwrap_or_default());
            self.completed = true;
            if node_ids.is_empty() && edge_ids.is_empty() {
                return Err(match command {
                    FlowCommand::RemoveWidget(payload) => Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("removeWidget found no widget \"{}\"", payload.widget_id)),
                    FlowCommand::Disconnect(payload) => Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("disconnect found no synapse \"{}\"", payload.synapse_id)),
                    _ => Fault::from("flow-retained-direct-route-mismatch"),
                });
            }
            return Ok(ArtifactCommandWorkStep::Complete(flow_content_leaves_emit(&snapshot.content.child_id, flow_removal_leaves(&child, &node_ids, &edge_ids))));
        }
        if matches!(command, FlowCommand::DeleteSelection(_)) {
            let selected = interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice());
            if selected.len() > FLOW_STORE_MAX_MUTATION_ITEMS || selected.iter().map(String::len).sum::<usize>() > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-delete-selection-capacity"));
            }
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            self.edge_ids.get_or_insert_with(Vec::new);
            self.node_ids.get_or_insert_with(Vec::new);
            if let Some(target) = selected.get(self.cursor) {
                if let Some(id) = target.strip_prefix(FLOW_GRAPH_EDGE_TARGET_PREFIX) {
                    if let Some(edge) = child.edges.get(self.scan_cursor) {
                        self.scan_cursor += 1;
                        let matched = edge.id == id;
                        if matched {
                            self.edge_ids.as_mut().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?.push(id.to_string());
                        }
                        if matched || self.scan_cursor == child.edges.len() {
                            self.cursor += 1;
                            self.scan_cursor = 0;
                        }
                    } else {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else if let Some(id) = target.strip_prefix(FLOW_GRAPH_NODE_TARGET_PREFIX) {
                    if let Some(node) = child.nodes.get(self.scan_cursor) {
                        self.scan_cursor += 1;
                        let matched = node.id == id;
                        if matched {
                            self.node_ids.as_mut().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?.push(id.to_string());
                        }
                        if matched || self.scan_cursor == child.nodes.len() {
                            self.cursor += 1;
                            self.scan_cursor = 0;
                        }
                    } else {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else {
                    self.cursor += 1;
                    self.scan_cursor = 0;
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-delete-selection-replay", preview: br#"{"en":"Restoring selected deletion","de":"Auswahlloeschung wird wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-delete-selection", preview: br#"{"en":"Preparing selected deletion","de":"Auswahlloeschung wird vorbereitet"}"# }
                });
            }
            let edge_ids = self.edge_ids.take().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?;
            let node_ids = self.node_ids.take().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?;
            self.completed = true;
            if node_ids.is_empty() && edge_ids.is_empty() {
                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.delete-selection-empty"), "deleteSelection needs at least one selected widget or synapse"));
            }
            return Ok(ArtifactCommandWorkStep::Complete(flow_content_leaves_emit(&snapshot.content.child_id, flow_removal_leaves(&child, &node_ids, &edge_ids))));
        }
        if let FlowCommand::SetPreviewOff(payload) = command {
            if self.preview_off.is_none() {
                if config.preview_off_node_ids.len() > FLOW_STORE_MAX_SCENE_ITEMS || config.preview_off_node_ids.iter().map(String::len).sum::<usize>() > FLOW_STORE_MAX_TEXT_BYTES {
                    return Err(Fault::from("flow-retained-preview-off-capacity"));
                }
                self.preview_off = Some(config.preview_off_node_ids.clone());
            }
            if let Some(id) = payload.ids.get(self.cursor) {
                self.preview_next.get_or_insert_with(Vec::new);
                let source_len = self.preview_off.as_ref().ok_or_else(|| Fault::from("flow-retained-preview-off-owner"))?.len();
                if let Some(existing) = self.preview_off.as_ref().and_then(|source| source.get(self.scan_cursor)).cloned() {
                    let matched = existing == *id;
                    self.preview_found |= matched;
                    if payload.value || !matched {
                        self.preview_next.as_mut().ok_or_else(|| Fault::from("flow-retained-preview-off-next-owner"))?.push(existing);
                    }
                    self.scan_cursor += 1;
                }
                if self.scan_cursor == source_len {
                    if payload.value && !self.preview_found {
                        let next = self.preview_next.as_mut().ok_or_else(|| Fault::from("flow-retained-preview-off-next-owner"))?;
                        if next.len() == FLOW_STORE_MAX_SCENE_ITEMS {
                            return Err(Fault::from("flow-retained-preview-off-item-capacity"));
                        }
                        next.push(id.clone());
                    }
                    self.preview_off = self.preview_next.take();
                    self.preview_found = false;
                    self.scan_cursor = 0;
                    self.cursor += 1;
                } else {
                    if self.scan_cursor > source_len {
                        return Err(Fault::from("flow-retained-preview-off-cursor"));
                    }
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-preview-off-replay", preview: br#"{"en":"Restoring preview visibility","de":"Vorschau-Sichtbarkeit wird wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-preview-off", preview: br#"{"en":"Updating preview visibility","de":"Vorschau-Sichtbarkeit wird aktualisiert"}"# }
                });
            }
            let node_ids = self.preview_off.take().ok_or_else(|| Fault::from("flow-retained-preview-off-owner"))?;
            self.completed = true;
            let mut next = config.clone();
            next.preview_off_node_ids = node_ids;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: main::config::addressed(view, &config, next)?, ..Default::default() }));
        }
        if let FlowCommand::PatchFlowWidgets(payload) = command {
            let input_bytes = payload.widget_ids.iter().map(String::len).fold(payload.field.len().saturating_add(payload.value.len()), usize::saturating_add);
            if payload.widget_ids.len() > FLOW_STORE_MAX_MUTATION_ITEMS || input_bytes > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-patch-widgets-capacity"));
            }
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            if let Some(node) = child.nodes.get(self.cursor) {
                if let Some(id) = payload.widget_ids.get(self.scan_cursor) {
                    self.scan_cursor += 1;
                    let matched = *id == node.id;
                    if matched || self.scan_cursor == payload.widget_ids.len() {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else {
                    self.cursor += 1;
                    self.scan_cursor = 0;
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-patch-widgets-replay", preview: br#"{"en":"Restoring widget patches","de":"Widget-Aktualisierungen werden wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-patch-widgets", preview: br#"{"en":"Preparing widget patches","de":"Widget-Aktualisierungen werden vorbereitet"}"# }
                });
            }
            self.completed = true;
            let leaves = patch_flow_widgets::patch_flow_widgets_leaves(&child, payload);
            return Ok(ArtifactCommandWorkStep::Complete(patch_flow_widgets::widget_leaves_emit(&snapshot.content.child_id, leaves)));
        }
        self.completed = true;
        flow_direct_store_emit(command, &config, view).map(ArtifactCommandWorkStep::Complete)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self,index:usize)->Option<u8>{match index{0=>Some(u8::from(self.completed)),1..=8=>Some(((self.cursor as u64)>>((index-1)*8))as u8),9..=16=>Some(((self.scan_cursor as u64)>>((index-9)*8))as u8),17=>Some(self.duplicate_phase),18..=25=>Some(((self.duplicate_source.unwrap_or(usize::MAX)as u64)>>((index-18)*8))as u8),26..=33=>Some((self.duplicate_suffix>>((index-26)*8))as u8),_=>None}}

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 34 || checkpoint[0] > 1 || checkpoint[17] > 2 {
            return Err(Fault::from("flow-retained-direct-checkpoint-invalid"));
        }
        if self.closing || !self.retirement.is_empty() || self.preview_off.is_some() || self.preview_next.is_some() || self.edge_ids.is_some() || self.node_ids.is_some() {
            return Err(Fault::from("flow-retained-direct-restore-requires-empty-owner"));
        }
        self.completed = checkpoint[0] == 1;
        self.replay_target = usize::try_from(u64::from_le_bytes(checkpoint[1..9].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.replay_scan_target = usize::try_from(u64::from_le_bytes(checkpoint[9..17].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.cursor = if self.tool_id == "duplicateWidget" { self.replay_target } else { 0 };
        self.scan_cursor = if self.tool_id == "duplicateWidget" { self.replay_scan_target } else { 0 };
        self.duplicate_phase = checkpoint[17];
        let duplicate_source = usize::try_from(u64::from_le_bytes(checkpoint[18..26].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.duplicate_source = (duplicate_source != usize::MAX).then_some(duplicate_source);
        self.duplicate_suffix = u64::from_le_bytes(checkpoint[26..34].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?);
        self.preview_off = None;
        self.preview_next = None;
        self.preview_found = false;
        self.edge_ids = None;
        self.node_ids = None;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !self.retirement.is_empty() {
            return self.retirement.step(grant);
        }
        if grant.maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        if let Some(values) = self.preview_off.take().or_else(|| self.preview_next.take()).or_else(|| self.edge_ids.take()).or_else(|| self.node_ids.take()) {
            self.retirement.push(values);
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.retirement.copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        self.retirement.capacity_byte_demand(maximum_copy_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.retirement.release_byte_demand()
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.retirement.is_empty() {
            Ok(usize::from(self.preview_off.is_some() || self.preview_next.is_some() || self.edge_ids.is_some() || self.node_ids.is_some()))
        } else {
            self.retirement.depth_demand()
        }
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.retirement.is_empty() && self.preview_off.is_none() && self.preview_next.is_none() && self.edge_ids.is_none() && self.node_ids.is_none()
    }
}
//#endregion 🧵️DirectStoreLaneRoutes
