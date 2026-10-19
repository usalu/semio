//! 🛠️ Retained command work and job factory: the stepping of the retained command work, its artifact command work implementation, the command job factory and its tool job factory implementations. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
impl EquationRetainedCommandWork {
    pub(super) fn new(tool_id: &'static str, operation_identity: u64, extent: usize) -> Self {
        Self {
            tool_id,
            operation_identity,
            extent,
            phase: EquationWorkPhase::Initialize,
            item_cursor: 0,
            cursor: 0,
            digest: 0xcbf2_9ce4_8422_2325,
            replay_target: None,
            graph: None,
            points: Vec::new(),
            operations: Vec::new(),
            leaves: Vec::new(),
            gesture: None,
            closing: false,
        }
    }

    fn observe(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.digest = (self.digest ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
        }
    }

    pub(super) fn progress<A: semio_framework_plugin::ArtifactApp>(&mut self, bytes: &[u8], stage: &'static str) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        self.observe(bytes);
        self.cursor = self.cursor.checked_add(1).ok_or_else(|| Fault::from("equation-work-cursor-overflow"))?;
        if self.cursor > self.extent {
            return Err(Fault::from("equation-work-extent-overflow"));
        }
        if let Some((target, expected)) = self.replay_target {
            if self.cursor == target {
                if self.digest != expected {
                    return Err(Fault::from("equation-work-replay-drift"));
                }
                self.replay_target = None;
            }
            return Ok(ArtifactCommandWorkStep::Replay { stage: "equation-command-replay", preview: br#"{"en":"Restoring Equation command","de":"Gleichungs-Befehl wird wiederhergestellt"}"# });
        }
        Ok(ArtifactCommandWorkStep::Progress { stage, preview: br#"{"en":"Preparing Equation command","de":"Gleichungs-Befehl wird vorbereitet"}"# })
    }

    fn initialize(&mut self, command: &EquationCommand, snapshot: &EquationSnapshot) -> Result<(), Fault> {
        let source = snapshot;
        match command {
            EquationCommand::SetActiveExample(_) => return Err(Fault::from("equation-set-active-example-is-not-a-phased-command")),
            EquationCommand::SetAlgorithm(payload) => {
                self.leaves = set_algorithm::set_algorithm_leaves(payload, &source.graph);
                self.phase = EquationWorkPhase::Finish;
            }
            EquationCommand::SetDirected(payload) => {
                self.leaves = set_directed::set_directed_leaves(payload, &source.graph);
                self.phase = EquationWorkPhase::Finish;
            }
            EquationCommand::AddNode(payload) => {
                self.leaves = add_node::add_node_leaves(payload, &source.graph)?;
                self.phase = EquationWorkPhase::Finish;
            }
            EquationCommand::NodeGraphEdit(_) => {
                self.graph = Some(source.graph.clone());
                self.phase = EquationWorkPhase::JsonBytes;
            }
            EquationCommand::EditEquation(payload) => {
                let (directed, algorithm, seed) = payload.graph.retained_metadata();
                let mut graph = EquationGraph { directed, nodes: Vec::new(), edges: Vec::new(), algorithm: algorithm.to_string(), algorithm_seed: seed.map(str::to_string) };
                graph.nodes.try_reserve_exact(payload.graph.retained_node_count()).map_err(|_| Fault::from("equation-command-node-reserve"))?;
                graph.edges.try_reserve_exact(payload.graph.retained_edge_count()).map_err(|_| Fault::from("equation-command-edge-reserve"))?;
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("equation-command-point-reserve"))?;
                self.graph = Some(graph);
                self.phase = EquationWorkPhase::Nodes;
            }
            EquationCommand::EditPoints(payload) => {
                self.points.try_reserve_exact(payload.geometry.points.len()).map_err(|_| Fault::from("equation-command-point-reserve"))?;
                self.phase = EquationWorkPhase::Points;
            }
            EquationCommand::NodeGraphViewport(_) => self.phase = EquationWorkPhase::Finish,
        }
        Ok(())
    }

    fn finish(&mut self, command: &EquationCommand, source: &EquationSnapshot, context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EquationPlayApp>>>, operation: &AppOperationContext) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
        Ok(match command {
            EquationCommand::SetActiveExample(_) => return Err(Fault::from("equation-set-active-example-is-not-a-phased-command")),
            EquationCommand::SetAlgorithm(_) | EquationCommand::SetDirected(_) | EquationCommand::AddNode(_) => Emit::mutations(std::mem::take(&mut self.leaves)),
            EquationCommand::NodeGraphEdit(_) => node_graph_edit::equation_edit_emit(&operation.authoring_seed, self.gesture.take().as_deref(), std::mem::take(&mut self.leaves)),
            EquationCommand::EditPoints(_) => Emit::mutations(edit_equation::equation_point_edit_leaves(&source.geometry.points, &std::mem::take(&mut self.points))),
            EquationCommand::EditEquation(_) => {
                let graph = self.graph.take().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                Emit::mutations(edit_equation::equation_graph_edit_leaves(&source.graph, &graph).into_iter().chain(edit_equation::equation_point_edit_leaves(&source.geometry.points, &std::mem::take(&mut self.points))).collect())
            }
            EquationCommand::NodeGraphViewport(payload) => {
                let view = context.and_then(|context| context.view_state.as_ref()).ok_or_else(|| Fault::from("equation-graph-window-context-required"))?;
                let mut emit = Emit::default();
                emit.window_config_mutations.push(graph_window::config::addressed(view, EquationGraphWindowConfigMutation::SetCamera(graph_window::config::SetCamera {
                    camera: EquationCamera { x: payload.viewport.x, y: payload.viewport.y, zoom: payload.viewport.zoom },
                }))?);
                emit
            }
        })
    }

    fn backing_bytes<T>(values: &Vec<T>) -> usize {
        if values.is_empty() { values.capacity().saturating_mul(size_of::<T>()) } else { 0 }
    }

    fn next_close_unit(&self) -> Option<(EquationCloseUnit, usize)> {
        if let Some(operation) = self.operations.last() {
            return Some((EquationCloseUnit::Operation, operation.retained_bytes()));
        }
        let bytes = Self::backing_bytes(&self.operations);
        if bytes != 0 {
            return Some((EquationCloseUnit::OperationsBacking, bytes));
        }
        if let Some(leaf) = self.leaves.last() {
            return Some((EquationCloseUnit::Leaf, equation_leaf_retained_bytes(leaf)));
        }
        let bytes = Self::backing_bytes(&self.leaves);
        if bytes != 0 {
            return Some((EquationCloseUnit::LeavesBacking, bytes));
        }
        if let Some(gesture) = self.gesture.as_ref() {
            return Some((EquationCloseUnit::Gesture, gesture.capacity()));
        }
        if let Some(point) = self.points.last() {
            return Some((EquationCloseUnit::Point, size_of_val(point)));
        }
        let bytes = Self::backing_bytes(&self.points);
        if bytes != 0 {
            return Some((EquationCloseUnit::PointsBacking, bytes));
        }
        let graph = self.graph.as_ref()?;
        if let Some(edge) = graph.edges.last() {
            return Some((EquationCloseUnit::Edge, size_of_val(edge) + edge.id.capacity() + edge.source.capacity() + edge.target.capacity()));
        }
        let bytes = Self::backing_bytes(&graph.edges);
        if bytes != 0 {
            return Some((EquationCloseUnit::EdgesBacking, bytes));
        }
        if let Some(node) = graph.nodes.last() {
            return Some((EquationCloseUnit::Node, size_of_val(node) + node.id.capacity() + node.label.capacity()));
        }
        let bytes = Self::backing_bytes(&graph.nodes);
        if bytes != 0 {
            return Some((EquationCloseUnit::NodesBacking, bytes));
        }
        Some((EquationCloseUnit::Graph, size_of::<EquationGraph>() + graph.algorithm.capacity() + graph.algorithm_seed.as_ref().map_or(0, String::capacity)))
    }

    fn close_unit(&mut self, unit: EquationCloseUnit) {
        match unit {
            EquationCloseUnit::Operation => drop(self.operations.pop()),
            EquationCloseUnit::OperationsBacking => self.operations = Vec::new(),
            EquationCloseUnit::Leaf => drop(self.leaves.pop()),
            EquationCloseUnit::LeavesBacking => self.leaves = Vec::new(),
            EquationCloseUnit::Gesture => self.gesture = None,
            EquationCloseUnit::Point => drop(self.points.pop()),
            EquationCloseUnit::PointsBacking => self.points = Vec::new(),
            EquationCloseUnit::Edge => drop(self.graph.as_mut().and_then(|graph| graph.edges.pop())),
            EquationCloseUnit::EdgesBacking => drop(self.graph.as_mut().map(|graph| std::mem::take(&mut graph.edges))),
            EquationCloseUnit::Node => drop(self.graph.as_mut().and_then(|graph| graph.nodes.pop())),
            EquationCloseUnit::NodesBacking => drop(self.graph.as_mut().map(|graph| std::mem::take(&mut graph.nodes))),
            EquationCloseUnit::Graph => self.graph = None,
        }
    }

    fn close_demand(&self) -> semio_framework_value::RetirementDemand {
        self.next_close_unit().map_or_else(Default::default, |(_, bytes)| semio_framework_value::RetirementDemand { release_bytes: bytes, depth: 1, ..Default::default() })
    }
}

impl ArtifactCommandWork<EditorApp<EquationPlayApp>> for EquationRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.operation_identity ^ (self.extent as u64).rotate_left(17)
    }

    fn extent(&self, command: &EquationCommand, snapshot: &EquationSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<EquationPlayApp>>>) -> Option<usize> {
        let extent = equation_command_extent(command, snapshot)?;
        (extent == self.extent).then_some(extent)
    }

    /// 🧮️ One bounded microturn. The job owns its command and snapshot for the whole operation and the
    /// framework measures [`Self::extent`] once in its preflight phase, so a turn never re-derives it:
    /// doing so cloned the whole working scene and re-parsed the edit JSON on every per-item turn,
    /// which made each microturn cost O(document) and the operation quadratic.
    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<EquationPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<EquationPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation } = *input;
        if self.cursor > self.extent {
            return Err(Fault::from("equation-command-extent-overflow"));
        }
        if let EquationCommand::SetActiveExample(payload) = command {
            return set_active_example::emit(&payload.example_id).map(ArtifactCommandWorkStep::Complete);
        }
        let source = snapshot;
        match self.phase {
            EquationWorkPhase::Initialize => {
                self.initialize(command, snapshot)?;
                self.progress::<EditorApp<EquationPlayApp>>(self.tool_id.as_bytes(), "equation-command-initialize")
            }
            EquationWorkPhase::Nodes => {
                let (count, node) = match command {
                    EquationCommand::EditEquation(payload) => (payload.graph.retained_node_count(), payload.graph.retained_node(self.item_cursor).cloned()),
                    _ => (source.graph.nodes.len(), source.graph.nodes.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Edges;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"nodes-complete", "equation-command-node-boundary");
                }
                let node = node.ok_or_else(|| Fault::from("equation-command-node-cursor"))?;
                if node.id.len() > EQUATION_MAX_TEXT_BYTES || node.label.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-command-node-text-capacity"));
                }
                self.observe(node.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.nodes.push(node);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-node")
            }
            EquationWorkPhase::Edges => {
                let (count, edge) = match command {
                    EquationCommand::EditEquation(payload) => (payload.graph.retained_edge_count(), (self.item_cursor < payload.graph.retained_edge_count()).then(|| payload.graph.retained_edge(self.item_cursor)).transpose().map_err(Fault::from)?),
                    _ => (source.graph.edges.len(), source.graph.edges.get(self.item_cursor).cloned()),
                };
                if self.item_cursor >= count {
                    self.item_cursor = 0;
                    self.phase = match command {
                        EquationCommand::EditEquation(_) => EquationWorkPhase::Points,
                        _ => EquationWorkPhase::Finish,
                    };
                    return self.progress::<EditorApp<EquationPlayApp>>(b"edges-complete", "equation-command-edge-boundary");
                }
                let edge = edge.ok_or_else(|| Fault::from("equation-command-edge-cursor"))?;
                if edge.id.len() > EQUATION_MAX_TEXT_BYTES || edge.source.len() > EQUATION_MAX_TEXT_BYTES || edge.target.len() > EQUATION_MAX_TEXT_BYTES {
                    return Err(Fault::from("equation-command-edge-text-capacity"));
                }
                self.observe(edge.id.as_bytes());
                self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?.edges.push(edge);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-edge")
            }
            EquationWorkPhase::Points => {
                let points = match command {
                    EquationCommand::EditEquation(payload) => &payload.geometry.points,
                    EquationCommand::EditPoints(payload) => &payload.geometry.points,
                    _ => return Err(Fault::from("equation-command-point-phase")),
                };
                if self.item_cursor >= points.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Finish;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"points-complete", "equation-command-point-boundary");
                }
                let point = points[self.item_cursor].clone();
                self.observe(&point.x.to_le_bytes());
                self.observe(&point.y.to_le_bytes());
                self.points.push(point);
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&self.item_cursor.to_le_bytes(), "equation-command-point")
            }
            EquationWorkPhase::JsonBytes => {
                let EquationCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("equation-command-json-phase")) };
                if self.item_cursor >= payload.operations_json.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::JsonDecode;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"json-complete", "equation-command-json-boundary");
                }
                let byte = payload.operations_json.as_bytes()[self.item_cursor];
                self.item_cursor += 1;
                self.progress::<EditorApp<EquationPlayApp>>(&[byte], "equation-command-json-byte")
            }
            EquationWorkPhase::JsonDecode => {
                let EquationCommand::NodeGraphEdit(payload) = command else { return Err(Fault::from("equation-command-json-decode")) };
                let values = semio_framework_pack_json::parse(&payload.operations_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec())).unwrap_or_default();
                if values.len() > EQUATION_MAX_EDIT_OPERATIONS {
                    return Err(Fault::from("equation-command-operation-capacity"));
                }
                self.operations.try_reserve_exact(values.len()).map_err(|_| Fault::from("equation-command-operation-reserve"))?;
                for value in &values {
                    self.operations.push(EquationEditOperation::from_value(value)?);
                }
                self.item_cursor = 0;
                self.phase = EquationWorkPhase::Operations;
                self.progress::<EditorApp<EquationPlayApp>>(&(values.len() as u64).to_le_bytes(), "equation-command-json-decode")
            }
            EquationWorkPhase::Operations => {
                if self.item_cursor >= self.operations.len() {
                    self.item_cursor = 0;
                    self.phase = EquationWorkPhase::Finish;
                    return self.progress::<EditorApp<EquationPlayApp>>(b"operations-complete", "equation-command-operation-boundary");
                }
                let edit = self.operations[self.item_cursor].clone();
                let graph = self.graph.as_mut().ok_or_else(|| Fault::from("equation-command-graph-owner"))?;
                let (leaves, gesture) = node_graph_edit::equation_edit_operation_leaves(graph, &edit);
                self.leaves.try_reserve(leaves.len()).map_err(|_| Fault::from("equation-command-leaf-reserve"))?;
                self.leaves.extend(leaves);
                if self.gesture.is_none() {
                    self.gesture = gesture;
                }
                self.item_cursor += 1;
                let observed = [self.item_cursor.to_le_bytes(), self.leaves.len().to_le_bytes()].concat();
                self.progress::<EditorApp<EquationPlayApp>>(&observed, "equation-command-operation")
            }
            EquationWorkPhase::Finish => self.finish(command, source, context, operation).map(ArtifactCommandWorkStep::Complete),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self,index:usize)->Option<u8>{match index{0..=3=>Some(b"MRC1"[index]),4=>Some(self.phase as u8),5..=7=>Some(0),8..=15=>Some(((self.cursor as u64)>>((index-8)*8))as u8),16..=23=>Some((self.digest>>((index-16)*8))as u8),24..=31=>Some((self.operation_identity>>((index-24)*8))as u8),32..=39=>Some(((self.extent as u64)>>((index-32)*8))as u8),_=>None}}

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 40
            || &checkpoint[..4] != b"MRC1"
            || checkpoint[5..8] != [0, 0, 0]
            || self.graph.is_some()
            || !self.points.is_empty()
            || !self.operations.is_empty()
            || !self.leaves.is_empty()
            || self.gesture.is_some()
        {
            return Err(Fault::from("equation-command-checkpoint-invalid"));
        }
        let identity = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("equation-command-checkpoint-identity"))?);
        let extent = u64::from_le_bytes(checkpoint[32..40].try_into().map_err(|_| Fault::from("equation-command-checkpoint-extent"))?);
        let cursor = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("equation-command-checkpoint-cursor"))?);
        if identity != self.operation_identity || extent != self.extent as u64 || cursor > self.extent as u64 {
            return Err(Fault::from("equation-command-checkpoint-mismatch"));
        }
        self.phase = EquationWorkPhase::Initialize;
        self.item_cursor = 0;
        self.cursor = 0;
        self.digest = 0xcbf2_9ce4_8422_2325;
        self.replay_target = (cursor != 0).then_some((cursor as usize, u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("equation-command-checkpoint-digest"))?)));
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            if !self.closing {
                return Ok(InteractiveJobCloseStep::Blocked);
            }
            let Some((unit, bytes)) = self.next_close_unit() else { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() })};
            if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes {
                return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() });
            }
            if grant.maximum_depth == 0 {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "interactive job close exceeds its admitted depth"));
            }
            self.close_unit(unit);
            Ok(InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() } })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demand().depth)
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.graph.is_none()
            && self.points.is_empty()
            && self.points.capacity() == 0
            && self.operations.is_empty()
            && self.operations.capacity() == 0
            && self.leaves.is_empty()
            && self.leaves.capacity() == 0
            && self.gesture.is_none()
    }
}

pub struct EquationCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl EquationCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: EQUATION_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for EquationCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<EquationPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<EquationPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        EQUATION_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        equation_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > EQUATION_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Equation retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for EquationCommandJobFactory {
    type Owner = EditorApp<EquationPlayApp>;
    const TOOL_IDS: &'static [&'static str] = EQUATION_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = EQUATION_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
