//! ✏️ `FlowHost` construction, snapshot and catalogue intake, evaluation baseline publication, camera and pointer interaction and graph editing.

use super::*;

impl FlowHost {
    /// 📔️ Supplies immutable operator implementations owned by this host's composition.
    pub fn with_operator_registry(mut self, registry: neural::SharedRegistry) -> Self {
        assert!(self.operator_registry.is_none(), "operator registry is supplied once");
        self.set_neuron_kind_info_map(Arc::new(registry.operator_infos().map(|info| (info.id.clone(),info.clone())).collect()));
        self.operator_registry = Some(registry);
        self.baseline_registry_generation = 0;
        self
    }
    /// 🔗️ Binds the actual preborn registry authority without constructing a global catalogue.
    pub fn bind_operator_registry_step(&mut self,registry:&neural::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if self.operator_registry.as_ref().is_some_and(|source|source.owner_identity()==registry.owner_identity()){return Ok(Default::default())}
        if self.operator_registry.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original registry authority must close before replacement"))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original registry lease requires one admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original registry lease requires admitted depth"))}
        self.operator_registry=Some(registry.clone());self.evaluation_revision=self.evaluation_revision.wrapping_add(1);Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    pub fn operator_registry(&self) -> neural::SharedRegistry { self.operator_registry.clone().unwrap_or_else(flow_operator_registry) }
    pub(super) fn operator_registry_generation(&self) -> u64 { if self.operator_registry.is_some() { 0 } else { flow_extension_registry_generation() } }
    /// 🔌️ Supplies one geometry authority that this owner retains and explicitly closes.
    pub fn with_geometry_port(mut self, port: Box<dyn semio_framework_os_flow_geometry::GeometryPort>) -> Self {
        assert!(self.geometry_port.is_none(), "geometry authority is supplied once");
        self.geometry_port = Some(port);
        self
    }
    pub fn geometry_port(&self) -> Result<&dyn semio_framework_os_flow_geometry::GeometryPort, String> {
        self.geometry_port.as_deref().ok_or_else(|| "flow.geometry-port-missing".into())
    }
    /// 🏭️ Selects the actual baseline Store producer supplied by this host's child composition.
    pub fn with_history_store_factory(mut self,producer:semio_framework_os_kernel::os_store::OriginalStoreGenesisFactory<FlowHostSnapshot,FlowMutation>)->Self{assert!(self.history_store_factory.is_none()&&self.history_store.is_none(),"original history producer is supplied once before Store birth");self.history_store_factory=Some(producer);self}
    pub fn from_host_snapshot(host_snapshot: FlowHostSnapshot) -> Self {
        Self::from_host_snapshot_with_cache(host_snapshot, Arc::new(NeuralCache::new()))
    }

    /// 🧠️ Builds a host sharing an existing [`NeuralCache`] — lets a long-lived caller (e.g. a
    /// stateless request/response program boundary that reconstructs `FlowHost` on every call)
    /// keep per-node memoization alive across those reconstructions instead of discarding it.
    pub fn from_host_snapshot_with_cache(host_snapshot: FlowHostSnapshot, neural_cache: Arc<NeuralCache>) -> Self {
        Self::from_host_snapshot_with_cache_and_infos(host_snapshot, neural_cache, Arc::default())
    }

    /// 🏠️ Builds a host that ALREADY indexes `kind_infos`. The ONE construction path for a caller
    /// that would set the operator catalogue immediately afterwards: the bare constructor builds its
    /// dag against an empty catalogue, and the setter's own `rebuild_dag` then throws that work away
    /// — twice per evaluation tick (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn from_host_snapshot_with_cache_and_infos(mut host_snapshot: FlowHostSnapshot, neural_cache: Arc<NeuralCache>, kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) -> Self {
        dedupe_host_snapshot_widgets(&mut host_snapshot);
        let mut host = Self {
            geometry_port: None,
            operator_registry: None,
            host_snapshot: host_snapshot,
            dag: DagHost::from_host_snapshot(DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_os_infinite_directed_presentation::DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] }),
            current_channels: None,
            export_payloads: HistoryFoldIndex::new(),
            last_eval_json: String::new(),
            host_catalogue_json: String::new(),
            kind_infos,
            neural_cache,
            previous_snapshot: None,
            previous_channels: None,
            pending_baseline_publication:None,
            baseline_retirement:None,
            baseline_publication_progress:Default::default(),
            pending_evaluation:None,
            evaluation_revision:0,
            evaluation_progress:Default::default(),
            baseline_registry_generation: flow_extension_registry_generation(),
            next_widget_serial: 1,
            next_synapse_serial: 100,
            viewport_w: 1,
            viewport_h: 1,
            viewport_dpr: 1.0,
            pan_anchor: None,
            ghost_node: None,
            history_store: None,
            history_store_factory:None,
            pending_history_baseline: None,
            pending_change: false,
            pending_leaves: Vec::new(),
            history_fault: None,
            #[cfg(test)]
            recorded: Vec::new(),
            journal_mark: 0,
            edited_note: None,
            gesture_active: false,
            pending_extension_evals: Vec::new(),
            pending_remaining: Vec::new(),
            interaction_revision: 0,
            interaction_projection: None,
            displaced: neural::ValueRetirement::default(),
        };
        host.rebuild_dag();
        host.refresh_interaction_projection();
        host
    }

    /// 📥️ Loads fixture content while keeping catalogue, operator metadata, eval bridge, and the live camera. A load is a history
    /// reset: no edit survives it.
    pub fn replace_host_snapshot(&mut self, host_snapshot: FlowHostSnapshot) {
        self.apply_host_snapshot(host_snapshot, true, false);
    }

    /// 📥️ Scene resync: reloads fixture layout/content without discarding eval baseline or cached outputs. A scene that merely
    /// echoes the live content leaves the history alone; a scene whose content differs is a load and resets the history, because
    /// a content change may only enter the history as concrete leaves.
    pub fn resync_host_snapshot_from_scene(&mut self, host_snapshot: FlowHostSnapshot) {
        self.apply_host_snapshot(host_snapshot, false, true);
    }

    fn apply_host_snapshot(&mut self, mut host_snapshot: FlowHostSnapshot, force_reset: bool, preserve_eval: bool) {
        self.evaluation_revision=self.evaluation_revision.wrapping_add(1);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        dedupe_host_snapshot_widgets(&mut host_snapshot);
        let load = force_reset || self.host_snapshot.widgets != host_snapshot.widgets || self.host_snapshot.synapses != host_snapshot.synapses || self.host_snapshot.layout != host_snapshot.layout;
        // 🎥️ Camera is ephemeral view state (same as undo/redo) — never snap the live pan/zoom when a
        // scene resync reloads fixture content (hover, eval tick, remote operations, …).
        let camera = self.host_snapshot.camera.clone();
        host_snapshot.camera = camera;
        std::mem::replace(&mut self.host_snapshot, host_snapshot).retire_cold();
        if !preserve_eval {
            self.displace_eval_state().expect("cold fixture load requires settled original baseline custody");
            self.last_eval_json.clear();
        }
        self.pan_anchor = None;
        self.ghost_node = None;
        self.rebuild_dag();
        self.refresh_interaction_projection();
        if load {
            self.reset_history_store();
            self.pending_change = false;
            self.gesture_active = false;
            self.edited_note = None;
            self.discard_pending_leaves();
        }
    }

    /// 🧹️ Restarts the history at the live content: the store is reset to it (when one exists) and the armed baseline is
    /// retired. Used by every load and by a refused history row, since the store can no longer represent the live content.
    pub(super) fn reset_history_store(&mut self) {
        if let Some(store) = self.history_store.as_mut() {
            let producer=self.history_store_factory.as_ref().expect("existing history Store retains its original producer");
            let envelope=(producer.envelope)(self.host_snapshot.clone()).expect("original child history genesis reset");
            ::semio_framework_async::poll::resolve_ready(store.reset(envelope)).expect("failed to reset flow history store");
        }
        if let Some(stale) = self.pending_history_baseline.take() {
            stale.retire_cold();
        }
    }

    pub fn parse_host_snapshot_json(json: &str) -> Result<FlowHostSnapshot, FlowCoreError> {
        Ok(semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?)
    }

    pub fn host_snapshot_json(&self) -> Result<String, FlowCoreError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }

    pub fn document(&self) -> FlowArtifact {
        self.host_snapshot.to_artifact()
    }

    pub fn catalogue_json(&self) -> Result<String, FlowCoreError> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        Ok(semio_framework_pack_json::to_json_string(&sections))
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    /// 📦️ Installs the palette sections from a content-addressed payload — see
    /// [`resolve_flow_shared_payload`]. Several parts are concatenated section by section.
    pub fn set_host_catalogue_payload(&mut self, payload: &str) -> Result<(), FlowCoreError> {
        let parts = resolve_flow_shared_payload_parts(payload)?;
        if let [single] = parts.as_slice() {
            self.set_host_catalogue_json(single);
            return Ok(());
        }
        let mut sections: Vec<CatalogueSection> = Vec::new();
        for part in &parts {
            if part.trim().is_empty() {
                continue;
            }
            sections.extend(semio_framework_pack_json::from_json_str::<Vec<CatalogueSection>>(part, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default());
        }
        self.set_host_catalogue_json(&semio_framework_pack_json::to_json_string(&sections));
        Ok(())
    }

    /// 🔎️ The operator kind ids this host indexes, sorted — the public read-back a law composes a
    /// table against without reaching into `kind_infos`.
    pub fn neuron_kind_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.kind_infos.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// 📦️ Installs the operator kind infos from a content-addressed payload — see
    /// [`resolve_flow_shared_payload`].
    ///
    /// The table is composed of its parts in order (the app-static catalogue, then whatever records
    /// the scene itself derived), so a later id wins over an earlier one exactly as it did when the
    /// host concatenated the two sources into one body itself.
    pub fn set_neuron_kind_infos_payload(&mut self, payload: &str) -> Result<(), FlowCoreError> {
        let parts = resolve_flow_shared_payload_parts(payload)?;
        let mut infos: HistoryFoldIndex<String, OperatorInfo> = HistoryFoldIndex::new();
        for part in &parts {
            if part.trim().is_empty() {
                continue;
            }
            for info in semio_framework_pack_json::from_json_str::<Vec<OperatorInfo>>(part, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default() {
                infos.insert(info.id.clone(), info);
            }
        }
        self.install_kind_infos(Arc::new(infos));
        Ok(())
    }

    /// 🧹️ Installs a new operator catalogue and retires the one it displaces — but ONLY when this
    /// host was its last owner. The catalogue is shared by every live host
    /// ([`FlowHost::kind_infos`]), and its `ChannelSpec::default` values are `Value`s that fail
    /// closed on a bare drop, so a plain assignment aborted the process the moment a host swapped
    /// a uniquely-owned catalogue out (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn install_kind_infos(&mut self, kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) {
        if let Some(displaced) = Arc::into_inner(std::mem::replace(&mut self.kind_infos, kind_infos)) {
            displaced.retire_cold();
        }
        self.rebuild_dag();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.install_kind_infos(Arc::new(if json.trim().is_empty() { HistoryFoldIndex::new() } else { semio_framework_pack_json::from_json_str::<Vec<OperatorInfo>>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect()).unwrap_or_default() }));
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the already-built id-keyed map — the ONE
    /// in-process path, because the JSON form of this catalogue is ~108 kB and an evaluation tick
    /// that serialized and re-parsed it spent 11-26 ms per tick doing nothing else
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn set_neuron_kind_info_map(&mut self, infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) {
        self.install_kind_infos(infos);
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the typed `NodeGraphScene.operators` records.
    pub fn set_neuron_kind_infos(&mut self, infos: &[ui_wgpu::wgpu::NodeGraphOperatorRecord]) {
        self.install_kind_infos(Arc::new(infos.iter().map(|record| (record.id.clone(), node_graph_operator_record_to_operator_info(record))).collect()));
    }

    pub fn evaluate(&mut self) -> Result<String, FlowCoreError> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }

    /// 📥️ Applies channel-structured eval JSON from an off-thread worker without re-running operators.
    pub fn apply_eval_outputs_json(&mut self, json: &str)->Result<(),ValueError> {
        if self.pending_baseline_publication.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must advance before another intake"))}
        if is_global_eval_error_json(json) {
            self.dag.paint.clear_computing();
            return Ok(());
        }
        let outputs = outputs_from_channel_eval_json(json);
        let inputs = inputs_from_channel_eval_json(json);
        let channels = EvalChannels { outputs, inputs };
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_deref(), &snapshot);
        let evaluated_generation = self.operator_registry_generation();
        let converged = self.probe_eval_outputs_converged(&tree, &seeds, &dirty, &channels);
        tree.retire_cold();
        seeds.retire_cold();
        self.last_eval_json = json.to_string();
        if converged {
            self.apply_preview_outputs(&channels.outputs);
            self.apply_export_outputs(&channels.outputs);
            self.begin_baseline_publication(snapshot,channels,evaluated_generation).unwrap_or_else(|_|unreachable!("original baseline intake was checked before constructing owners"));
            self.dag.paint.clear_computing();
        } else {
            channels.retire_cold();
            self.refresh_computing_chrome_from_pending();
        }
        Ok(())
    }

    fn probe_eval_outputs_converged(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, dirty: &HashSet<String>, channels: &EvalChannels) -> bool {
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("apply_eval_outputs_json probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(tree, seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, dirty, Some(channels), EvalStepBudget::PROBE,&|_|true) {
            Ok(BudgetedEval { remaining, channels, .. }) => {
                channels.retire_cold();
                remaining.is_empty()
            }
            Err(_) => false,
        }
    }

    /// 🧵️ Installs a durable eval baseline from an off-thread driver onto this ephemeral host.
    ///
    /// `registry_generation` is the flow extension registry replacement the driver's baseline was
    /// computed against, and it travels WITH the baseline rather than being read from the live
    /// registry here: an ephemeral host is rebuilt after the install that bumped the generation, so
    /// reading "now" would tell every such host its inherited baseline is current when it is
    /// precisely the one that is not (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn install_eval_baseline(&mut self,snapshot:Option<Arc<TreeSnapshot>>,channels:Option<Arc<EvalChannels>>,registry_generation:u64,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Option<Arc<TreeSnapshot>>,Option<Arc<EvalChannels>>)>{
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"baseline lease handoff requires one admitted item"))}
            else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"baseline lease handoff requires source depth"))}
            else if !self.baseline_publication_terminal_is_empty(){Some((ValueRefusalKind::WorkLimit,"baseline publication must complete before installing another lease"))}
            else if self.current_channels.is_some(){Some((ValueRefusalKind::UnsupportedOwner,"baseline installation requires original output lease handoff"))}else{None};
        if let Some((kind,message))=refusal{return Err((ValueError::literal(kind,message),snapshot,channels))}
        let original=FlowBaselineLeases{snapshot:self.previous_snapshot.take(),channels:self.previous_channels.take(),current:self.current_channels.take()};
        if original.snapshot.is_some()||original.channels.is_some()||original.current.is_some(){self.baseline_retirement=Some(ControlledRetirement::new(FlowPublicationDisplaced{publication:None,text:None,display:None,exports:None,pending:Vec::new(),remaining:Vec::new(),snapshot:None,leases:original}).unwrap_or_else(|_|unreachable!("original baseline leases are supported")));}
        self.previous_snapshot=snapshot;self.current_channels=channels.clone();self.previous_channels=channels;self.baseline_registry_generation=registry_generation;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🔎️ Reads the original active output owner or its immutable installed baseline lease.
    pub fn output_channels(&self,widget_id:&str)->Option<&Dictionary>{
        self.current_channels.as_deref().or(self.previous_channels.as_deref())?.outputs.get(widget_id)
    }
    /// 📖️ Borrows actual output rows without restoring a copied map.
    pub fn output_entries(&self)->impl Iterator<Item=(&String,&Dictionary)>{
        self.current_channels.as_deref().or(self.previous_channels.as_deref()).into_iter().flat_map(|channels|channels.outputs.iter())
    }

    fn displace_dictionary(&mut self, value: Dictionary) {
        let grant = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4096 };
        if let Err((_, value)) = self.displaced.push_dictionary(value, grant) { value.retire_cold(); }
    }

    /// 🧹️ Hands every value displaced since the last drain to the artifact's bounded retirement
    /// ladder. Called at the START of an evaluation tick, so a root the CURRENT tick's `NeuralCache`
    /// or `previous_channels` still reads is never torn down under it — the frontier therefore holds
    /// at most one tick's displacement, and what is left when the host closes is taken over by
    /// [`FlowHostRetirement::new`].
    pub(super) fn drain_displaced(&mut self) {
        neural::retirement::retire_value_cold(std::mem::take(&mut self.displaced));
    }

    /// 📥️ Retains the original baseline and export arenas inline before funded retirement.
    fn displace_eval_state(&mut self)->Result<(),ValueError>{
        if self.pending_baseline_publication.is_some()||self.baseline_retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must finish before displacement"))}
        let original=FlowPublicationDisplaced{publication:None,text:None,display:None,pending:std::mem::take(&mut self.pending_extension_evals),remaining:std::mem::take(&mut self.pending_remaining),exports:Some(std::mem::take(&mut self.export_payloads)),snapshot:None,leases:FlowBaselineLeases{snapshot:self.previous_snapshot.take(),channels:self.previous_channels.take(),current:self.current_channels.take()}};
        self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original baseline fields declare typed retirement")));Ok(())
    }

    /// 🧵️ Captures this host's eval baseline for persistence on a durable driver.
    pub fn eval_baseline(&self) -> (Option<Arc<TreeSnapshot>>, Option<Arc<EvalChannels>>) {
        (self.previous_snapshot.clone(), self.previous_channels.clone())
    }

    /// 📥️ Retains the evaluator's genuine computed owners before admitting shared publication.
    pub fn begin_baseline_publication(&mut self,snapshot:TreeSnapshot,channels:EvalChannels,generation:u64)->Result<(),(ValueError,TreeSnapshot,EvalChannels)>{
        self.begin_eval_publication(snapshot,channels,generation,true)
    }

    pub(super) fn begin_eval_publication(&mut self,snapshot:TreeSnapshot,channels:EvalChannels,generation:u64,converged:bool)->Result<(),(ValueError,TreeSnapshot,EvalChannels)>{
        if !self.baseline_publication_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original evaluation publication is already pending"),snapshot,channels))}
        self.pending_baseline_publication=Some(FlowBaselinePublication{revision:self.evaluation_revision,display:Some(FlowDisplayCursor::new()),snapshot:Some(snapshot),channels:Some(channels),shared_snapshot:None,shared_channels:None,generation,converged,pending:Vec::new(),remaining:Vec::new()});Ok(())
    }
    /// 🌊️ Moves the evaluator's original wave and remaining rows together with its channel authority.
    pub(super) fn begin_work_publication(&mut self,output:semio_framework_os_flow_host_source::work::FlowEvaluationCompletion)->Result<(),(ValueError,semio_framework_os_flow_host_source::work::FlowEvaluationCompletion)>{
        if !self.baseline_publication_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original Work publication is already pending"),output))}
        let semio_framework_os_flow_host_source::work::FlowEvaluationCompletion{snapshot,channels,remaining,pending_extensions,generation}=output;
        self.pending_baseline_publication=Some(FlowBaselinePublication{revision:self.evaluation_revision,display:Some(FlowDisplayCursor::new()),snapshot:Some(snapshot),channels:Some(channels),shared_snapshot:None,shared_channels:None,generation,converged:remaining.is_empty(),pending:pending_extensions,remaining});Ok(())
    }
    pub fn pending_evaluation_ids(&self)->&[String]{&self.pending_remaining}
    /// 📏️ Borrows each independent currency of the actual publication owner.
    pub fn next_baseline_publication_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.baseline_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        let Some(owner)=self.pending_baseline_publication.as_ref()else{return Ok(Default::default())};
        if owner.revision!=self.evaluation_revision{return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(display)=owner.display.as_ref(){if !display.complete(){return display.demands(&self.host_snapshot,owner.channels.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original display channel source was transferred"))?,&self.kind_infos)}}
        let capacity_bytes=if owner.converged&&owner.snapshot.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<TreeSnapshot>()}else if owner.channels.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<EvalChannels>()}else{0};
        Ok(RetirementDemand{capacity_bytes,depth:1,..Default::default()})
    }
    /// 🎟️ Creates one admitted Arc header or transfers the same original baseline owners.
    pub fn baseline_publication_step_progress(&self)->RetainedCloneProgress{self.baseline_publication_progress}

    pub fn baseline_publication_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.baseline_publication_progress=Default::default();
        let empty=RetainedCloneProgress::default();
        if self.baseline_publication_terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.next_baseline_publication_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original baseline publication exceeds admitted depth"))}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.baseline_retirement.as_mut(){
            let result=owner.step(grant);self.baseline_publication_progress=owner.step_progress();let step=result.map_err(|error|error.with_retained_progress(self.baseline_publication_progress))?;
            let progress=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"Flow original baseline source")?.progress();
            if owner.terminal_is_empty(){self.baseline_retirement=None;}
            return Ok(if self.baseline_publication_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        }
        if self.pending_baseline_publication.as_ref().is_some_and(|owner|owner.revision!=self.evaluation_revision){let original=FlowPublicationDisplaced{publication:self.pending_baseline_publication.take(),text:None,display:None,exports:None,pending:Vec::new(),remaining:Vec::new(),snapshot:None,leases:FlowBaselineLeases{snapshot:None,channels:None,current:None}};self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original canceled publication source")));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}
        let owner=self.pending_baseline_publication.as_mut().unwrap();
        if let Some(display)=owner.display.as_mut(){if !display.complete(){let result=display.step(&self.host_snapshot,owner.channels.as_ref().unwrap(),&self.kind_infos,grant);match result{Ok(progress)=>{self.baseline_publication_progress=progress;return Ok(RetainedCloneStep::Progress(progress))},Err(error)=>{self.baseline_publication_progress=error.retained_progress();return Err(error)}}}}
        if owner.converged{if let Some(source)=owner.snapshot.take(){owner.shared_snapshot=Some(FlowPublicationShared(Arc::new(source)));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..empty};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}}
        if let Some(source)=owner.channels.take(){owner.shared_channels=Some(FlowPublicationShared(Arc::new(source)));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..empty};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}
        let (display,text)=match owner.display.take(){Some(mut display)=>{let text=display.take_text();(Some(display),text)},None=>(None,None)};
        let previous_text=text.map(|text|std::mem::replace(&mut self.last_eval_json,text));
        let original=FlowPublicationDisplaced{publication:None,text:previous_text,display,exports:None,pending:std::mem::take(&mut self.pending_extension_evals),remaining:std::mem::take(&mut self.pending_remaining),snapshot:owner.snapshot.take(),leases:FlowBaselineLeases{
            snapshot:if owner.converged{self.previous_snapshot.take()}else{None},channels:if owner.converged{self.previous_channels.take()}else{None},current:self.current_channels.take(),
        }};
        if original.text.is_some()||original.display.is_some()||original.pending.capacity()!=0||original.remaining.capacity()!=0||original.snapshot.is_some()||original.leases.snapshot.is_some()||original.leases.channels.is_some()||original.leases.current.is_some(){self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original publication sources are supported")));}
        self.pending_extension_evals=std::mem::take(&mut owner.pending);
        self.pending_remaining=std::mem::take(&mut owner.remaining);
        self.current_channels=owner.shared_channels.take().map(|lease|lease.0);
        if owner.converged{self.previous_snapshot=owner.shared_snapshot.take().map(|lease|lease.0);self.previous_channels=self.current_channels.clone();self.baseline_registry_generation=owner.generation;}
        self.pending_baseline_publication=None;
        let progress=RetainedCloneProgress{copied_items:1,..empty};self.baseline_publication_progress=progress;
        Ok(if self.baseline_publication_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    /// 🏁️ Publication completes only after all displaced original leases close.
    pub fn baseline_publication_terminal_is_empty(&self)->bool{self.pending_baseline_publication.is_none()&&self.baseline_retirement.is_none()}

    /// 🔢 The flow extension registry replacement this host's eval baseline was computed against.
    pub fn eval_baseline_registry_generation(&self) -> u64 {
        self.baseline_registry_generation
    }

    /// 🔢 Whether the incremental baseline may still be believed — i.e. whether the operator table
    /// it was dispatched through is the one installed right now.
    ///
    /// `compute_dirty_set` diffs the TREE, and a contributed operator arriving in a registry
    /// replacement changes no tree at all: the operator table is process-wide state swapped out
    /// underneath a live host. So a stale generation must make the WHOLE tree dirty and drop the
    /// previous channels, not merely refuse the skip — a refused skip that still diffs against the
    /// old snapshot computes an empty dirty set and dispatches nothing, which is the same stale
    /// answer by a longer road (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn baseline_is_current(&self) -> bool {
        self.baseline_registry_generation == self.operator_registry_generation()
    }

    /// ⏮️ The snapshot to diff this evaluation against — `None` once the registry it was computed
    /// against has been replaced, which makes every node dirty.
    pub(super) fn current_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_deref().filter(|_| self.baseline_is_current())
    }

    /// ⏮️ The channels this evaluation may free-ride on, under the same condition.
    pub(super) fn current_baseline_channels(&self) -> Option<&EvalChannels> {
        self.previous_channels.as_deref().filter(|_| self.baseline_is_current())
    }

    /// ⚡️ Whether the incremental fast path may skip this evaluation entirely: nothing is dirty, a
    /// full channel baseline is held, and outputs are published.
    ///
    /// One predicate, read by every caller that owns a fast path ([`FlowHost::evaluate_step`] and
    /// [`FlowHost::pending_eval_widget_ids`]), so the two can never drift apart.
    pub(super) fn baseline_answers_everything(&self, dirty: &HashSet<String>) -> bool {
        dirty.is_empty() && self.current_baseline_channels().is_some() && self.output_entries().next().is_some()
    }

    /// ⚙️ Probes pending nodes and paints active/stale computing chrome on the DAG canvas.
    pub fn refresh_computing_chrome_from_pending(&mut self) {
        let remaining = self.pending_eval_widget_ids();
        if remaining.is_empty() {
            self.dag.paint.clear_computing();
            return;
        }
        let active = remaining.first().map(|id| id.as_str());
        let stale = remaining.get(1..).unwrap_or(&[]).to_vec();
        set_computing_progress(&mut self.dag.graph, &mut self.dag.paint, active, &stale);
    }

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_node_statuses(&mut self,statuses:&DagNodeStatuses){set_node_statuses(&mut self.dag.graph, &mut self.dag.paint, statuses);}

    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        set_computing_progress(&mut self.dag.graph, &mut self.dag.paint, active_widget_id, stale_widget_ids);
    }

    /// ✅️ Clears computing chrome from all widgets.
    pub fn clear_computing_widget_ids(&mut self) {
        self.dag.paint.clear_computing();
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport_w = width.max(1);
        self.viewport_h = height.max(1);
        self.viewport_dpr = dpr.max(1.0);
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn world_from_screen(&self, sx: f64, sy: f64) -> (f64, f64) {
        let p = self.screen_to_world_point(sx, sy);
        (p.x, p.y)
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.host_snapshot.camera = CameraJson { x, y, zoom: zoom.clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX) };
        self.dag.graph.set_camera(x, y, self.host_snapshot.camera.zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    /// 📷️ The ONE camera of a flow surface. `self.host_snapshot.camera` is the authority — it is what
    /// `screen_to_world_point` projects with, what `build_dag_host_snapshot_v1` re-seeds the dag copy from
    /// on every rebuild, and what the renderer publishes as `nodeGraphViewport`. The dag's own
    /// `host_snapshot.camera` is a derived paint copy; reading it instead is how a fit got published stale.
    pub fn camera(&self) -> [f64; 3] {
        [self.host_snapshot.camera.x, self.host_snapshot.camera.y, self.host_snapshot.camera.zoom]
    }

    /// 📷️ Adopts a camera the DAG computed for itself (a fit, a refit, an opening decision) into the
    /// authority, so the published, projected and painted cameras cannot drift apart.
    fn adopt_dag_camera(&mut self) {
        self.host_snapshot.camera = CameraJson { x: self.dag.graph.host_snapshot.camera.x, y: self.dag.graph.host_snapshot.camera.y, zoom: self.dag.graph.host_snapshot.camera.zoom };
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    /// 🖼️ Frames the whole graph and keeps the authority on the fitted camera.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::fit_camera_to_content`
    pub fn fit_camera_to_content(&mut self) -> bool {
        let fitted = fit_camera_to_content(&mut self.dag.viewport, &mut self.dag.graph);
        if fitted {
            self.adopt_dag_camera();
        }
        fitted
    }

    /// 🖼️ The opening-camera decision, with the authority kept on whichever camera won.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::adopt_camera_or_fit`
    pub fn adopt_camera_or_fit(&mut self, x: f64, y: f64, zoom: f64) -> bool {
        let fitted = adopt_camera_or_fit(&mut self.dag.viewport, &mut self.dag.graph, x, y, zoom);
        self.adopt_dag_camera();
        fitted
    }

    /// 🖼️ Re-fits a graph that changed under a live camera, with the authority kept on the result.
    ///
    /// @see `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagHost::refit_camera_if_content_left_view`
    pub fn refit_camera_if_content_left_view(&mut self) -> bool {
        let refitted = refit_camera_if_content_left_view(&mut self.dag.viewport, &mut self.dag.graph);
        if refitted {
            self.adopt_dag_camera();
        }
        refitted
    }

    fn refresh_interaction_projection(&mut self) {
        self.interaction_projection = self.dag.graph.bounded_interaction_projection(self.interaction_revision).ok();
    }

    /// 🖱️ Re-seeds the bounded pointer-plan projection after a SCREEN-path gesture mutated this host
    /// directly (drawing a wire, dragging the minimap viewport, dragging an inline widget) — without
    /// it `plan_pointer` answers `Unsupported` for the rest of the session, because its stored
    /// projection is a revision behind and nothing else on that path bumps it back into step.
    pub fn resync_interaction_projection(&mut self) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
    }

    pub fn plan_wheel(&self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) -> FlowWheelPlan {
        let camera = &self.host_snapshot.camera;
        let expected = [camera.x, camera.y, camera.zoom];
        let next = if zoom_gesture {
            use canvas::camera::{screen_to_world, Camera, Viewport};
            let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
            let before_camera = Camera { x: camera.x, y: camera.y, zoom: camera.zoom };
            let before = screen_to_world(&before_camera, &viewport, canvas::Point::new(sx, sy));
            let zoom = (camera.zoom * if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR })
                .clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
            let after_camera = Camera { x: camera.x, y: camera.y, zoom };
            let after = screen_to_world(&after_camera, &viewport, canvas::Point::new(sx, sy));
            [camera.x + before.x - after.x, camera.y + before.y - after.y, zoom]
        } else {
            [camera.x - delta_x / camera.zoom, camera.y - delta_y / camera.zoom, camera.zoom]
        };
        FlowWheelPlan { revision: self.interaction_revision, expected, next }
    }

    pub fn commit_wheel(&mut self, plan: FlowWheelPlan) -> bool {
        let camera = &self.host_snapshot.camera;
        if self.interaction_revision != plan.revision || [camera.x.to_bits(), camera.y.to_bits(), camera.zoom.to_bits()] != [plan.expected[0].to_bits(), plan.expected[1].to_bits(), plan.expected[2].to_bits()] {
            return false;
        }
        self.host_snapshot.camera = CameraJson { x: plan.next[0], y: plan.next[1], zoom: plan.next[2] };
        self.dag.graph.set_camera(plan.next[0], plan.next[1], plan.next[2]);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.refresh_interaction_projection();
        true
    }

    /// 🫳️ Whether this host is mid-way through a BOUNDED gesture — a node drag, a marquee or a pan
    /// started by `commit_pointer` and not yet released. A gesture belongs to the path that started it.
    pub fn bounded_pointer_gesture_active(&self) -> bool {
        self.interaction_projection.is_some_and(|projection| projection.gesture_active())
    }

    pub fn plan_pointer(&self, intent: dag::DagPointerIntent) -> Result<dag::DagPointerPlan, dag::DagInteractionPlanFault> {
        let projection = self.interaction_projection.ok_or(dag::DagInteractionPlanFault::NodeCredits)?;
        if projection.revision() != self.interaction_revision {
            return Err(dag::DagInteractionPlanFault::Unsupported);
        }
        derive_pointer_plan(&self.dag.viewport, &self.dag.graph, &self.dag.minimap, projection, intent)
    }

    pub fn commit_pointer(&mut self, plan: dag::DagPointerPlan) -> bool {
        if self.interaction_revision != plan.expected_revision() {
            return false;
        }
        if !plan.previous_gesture_active() && plan.gesture_active() {
            self.begin_gesture();
        }
        for index in 0..plan.move_len() {
            let Some((id, x, y)) = self.dag.graph.pointer_plan_move(&plan, index) else {
                continue;
            };
            if self.host_snapshot.layout.contains_key(id) {
                self.host_snapshot.layout.insert(id.to_owned(), WidgetLayout { x, y });
            }
        }
        self.dag.graph.apply_pointer_plan(&plan);
        if plan.previous_gesture_active() && !plan.gesture_active() {
            self.commit_gesture_history();
        }
        self.interaction_projection = Some(*plan.projection());
        self.interaction_revision = plan.projection().revision();
        true
    }

    pub fn pointer_projection_snapshot(&self, plan: &dag::DagPointerPlan) -> Result<dag::DagPointerSnapshot, dag::DagInteractionPlanFault> {
        let projection = plan.projection();
        let mut bytes = 0usize;
        for id in self.dag.graph.projection_selected_id_refs(projection).chain(self.dag.graph.projection_hovered_id_ref(projection)) {
            bytes = bytes.checked_add(id.len()).ok_or(dag::DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(dag::DagInteractionPlanFault::StringCredits);
            }
        }
        Ok(dag::DagPointerSnapshot { node_ids: self.dag.graph.projection_selected_id_refs(projection).map(str::to_owned).collect(), hovered_id: self.dag.graph.projection_hovered_id_ref(projection).map(str::to_owned), camera: projection.camera() })
    }

    pub fn wheel_zoom_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let before = self.screen_to_world_point(sx, sy);
        let factor = if delta_y < 0.0 { ui_styling::metrics::camera::WHEEL_ZOOM_IN_FACTOR } else { ui_styling::metrics::camera::WHEEL_ZOOM_OUT_FACTOR };
        let zoom = (self.host_snapshot.camera.zoom * factor).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        self.host_snapshot.camera.zoom = zoom;
        self.dag.graph.set_camera(self.host_snapshot.camera.x, self.host_snapshot.camera.y, zoom);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let after = self.screen_to_world_point(sx, sy);
        self.host_snapshot.camera.x += before.x - after.x;
        self.host_snapshot.camera.y += before.y - after.y;
        self.dag.graph.set_camera(self.host_snapshot.camera.x, self.host_snapshot.camera.y, zoom);
    }

    pub fn wheel_pan_screen(&mut self, delta_x: f64, delta_y: f64) {
        let zoom = self.host_snapshot.camera.zoom;
        let x = self.host_snapshot.camera.x - delta_x / zoom;
        let y = self.host_snapshot.camera.y - delta_y / zoom;
        self.set_camera(x, y, zoom);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_x: f64, delta_y: f64, zoom_gesture: bool) {
        if zoom_gesture {
            self.wheel_zoom_screen(sx, sy, delta_y);
        } else {
            self.wheel_pan_screen(delta_x, delta_y);
        }
    }

    pub fn set_ghost_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<(), FlowCoreError> {
        let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let id: String = "__ghost__".into();
        let widget = widget_from_descriptor(&descriptor, id.clone(), &self.kind_infos);
        let mut layout = protocol::value::ordered::OrderedMap::new();
        layout.insert(id, WidgetLayout { x: world_x, y: world_y });
        let mut node = widget_to_dag_node(&widget, 0, &layout, &[], &self.kind_infos, widget_node_size(&widget, &[], &self.kind_infos));
        widget.retire_cold();
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push_cold(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(layout));
        retirement.retire_cold();
        fit_node_size(&mut node);
        self.ghost_node = Some(node.clone());
        self.dag.pointer.set_ghost_node(Some(node));
        Ok(())
    }

    pub fn clear_ghost_widget(&mut self) {
        self.ghost_node = None;
        self.dag.pointer.set_ghost_node(None);
    }

    pub fn add_widget(&mut self, descriptor_json: &str, world_x: f64, world_y: f64) -> Result<String, FlowCoreError> {
        let descriptor: WidgetDescriptor = semio_framework_pack_json::from_json_str(descriptor_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        self.add_widget_descriptor(&descriptor, world_x, world_y)
    }

    /// 🧩️ Creates a widget from the admitted native descriptor.
    pub fn add_widget_descriptor(&mut self, descriptor: &WidgetDescriptor, world_x: f64, world_y: f64) -> Result<String, FlowCoreError> {
        self.begin_change();
        self.clear_ghost_widget();
        let id = descriptor_explicit_id(descriptor).unwrap_or_else(|| self.next_widget_id(descriptor));
        if self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id) {
            return Err(FlowCoreError::WidgetIdExists(id));
        }
        let widget = widget_from_descriptor(descriptor, id.clone(), &self.kind_infos);
        let added = FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: widget.clone() });
        self.host_snapshot.widgets.push(widget);
        self.host_snapshot.layout.insert(id.clone(), WidgetLayout { x: world_x, y: world_y });
        self.note_leaves([added].into_iter().chain(self.layout_leaf([id.as_str()])));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(id)
    }

    pub fn remove_widget(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.host_snapshot.widgets.len();
        let touching: Vec<String> = self.host_snapshot.synapses.iter().filter(|s| s.from == widget_id || s.to == widget_id).map(|s| s.id.clone()).collect();
        let had_layout = self.host_snapshot.layout.contains_key(widget_id);
        self.host_snapshot.widgets.retain(|w| widget_id_for(w) != widget_id);
        if self.host_snapshot.widgets.len() == before {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.host_snapshot.layout.remove(widget_id);
        self.host_snapshot.synapses.retain(|s| s.from != widget_id && s.to != widget_id);
        let unplaced = had_layout.then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: widget_id.to_string(), layout: None }] }));
        self.note_leaves(touching.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain([FlowMutation::RemoveWidget(RemoveWidget { id: widget_id.to_string() })]));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn move_widget(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), FlowCoreError> {
        if !self.host_snapshot.widgets.iter().any(|w| widget_id_for(w) == widget_id) {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        }
        self.begin_change();
        self.host_snapshot.layout.insert(widget_id.to_string(), WidgetLayout { x, y });
        let moved = self.layout_leaf([widget_id]);
        self.note_leaves(moved);
        self.dag.graph.set_widget_position(widget_id, x, y)?;
        self.checked_change()?;
        Ok(())
    }

    pub fn connect(&mut self, from_id: &str, to_id: &str) -> Result<String, FlowCoreError> {
        let from_port = first_output_port(from_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        let to_port = first_input_port(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        self.connect_ports(from_id, &from_port, to_id, &to_port)
    }

    pub(super) fn next_synapse_id(&mut self) -> String {
        loop {
            self.next_synapse_serial = self.next_synapse_serial.wrapping_add(1);
            let id = format!("s{}", self.next_synapse_serial);
            if self.host_snapshot.synapses.iter().all(|synapse| synapse.id != id) {
                return id;
            }
        }
    }

    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, FlowCoreError> {
        self.begin_change();
        if from_id == to_id {
            return Err(FlowCoreError::SelfConnection);
        }
        if !widget_has_output(from_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(from_id.to_string()));
        }
        if !widget_has_input(to_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(to_id.to_string()));
        }
        if !widget_has_declared_port(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::UnknownOutputPort(from_port.to_string()));
        }
        if !widget_has_declared_port(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::UnknownInputPort(to_port.to_string()));
        }
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        if would_create_cycle(&existing, from_id, to_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        if self.host_snapshot.synapses.iter().any(|s| s.from == from_id && s.from_port == from_port && s.to == to_id && s.to_port == to_port) {
            return Err(FlowCoreError::ConnectionAlreadyExists);
        }
        let source_types = widget_port_value_types(from_id, from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        let target_types = widget_port_value_types(to_id, to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos);
        if !port_value_types_compatible(&source_types, &target_types) {
            return Err(FlowCoreError::IncompatiblePortTypes {
                source: format!("{from_id}@{from_port}"),
                source_type: source_types.join(","),
                target: format!("{to_id}@{to_port}"),
                target_type: target_types.join(","),
            });
        }
        let displaced: Vec<String> = self.host_snapshot.synapses.iter().filter(|s| s.to == to_id && s.to_port == to_port).map(|s| s.id.clone()).collect();
        self.host_snapshot.synapses.retain(|s| !(s.to == to_id && s.to_port == to_port));
        let synapse_id = self.next_synapse_id();
        let synapse = SynapseSpec { id: synapse_id.clone(), from: from_id.to_string(), to: to_id.to_string(), from_port: from_port.to_string(), to_port: to_port.to_string() };
        let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(self.host_snapshot.synapses.len()).unwrap_or(u32::MAX), synapse: synapse.clone() });
        self.host_snapshot.synapses.push(synapse);
        self.note_leaves(displaced.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain([added]));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(synapse_id)
    }

    pub fn add_input_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxInputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.to_port = (old_index + 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        *input_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn remove_input_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_input.clone()).ok_or_else(|| FlowCoreError::NotVariadicInput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { input_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_input_ports(&neuron_kind, input_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinInputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownInputPort(port_id.to_string()));
        };
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.to == widget_id && synapse.to_port == port_id).map(|synapse| synapse.id.clone()).collect();
        self.host_snapshot.synapses.retain(|synapse| !(synapse.to == widget_id && synapse.to_port == port_id));
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.to != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.to_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.to_port = (old_index - 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *input_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse }))).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn add_output_port(&mut self, widget_id: &str, index: usize) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let mut ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if let Some(max) = spec.max {
            if ports.len() >= max {
                return Err(FlowCoreError::MaxOutputPortsReached(widget_id.to_string()));
            }
        }
        let insert_at = index.min(ports.len());
        ports.insert(insert_at, insert_at.to_string());
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index >= insert_at {
                    synapse.from_port = (old_index + 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        *output_ports = (0..ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn remove_output_port(&mut self, widget_id: &str, port_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let neuron_kind = self
            .host_snapshot
            .widgets
            .iter()
            .find_map(|widget| match widget {
                Widget::Neuron { id, neuron_kind, .. } if id == widget_id => Some(neuron_kind.clone()),
                _ => None,
            })
            .ok_or_else(|| FlowCoreError::UnknownNeuronWidget(widget_id.to_string()))?;
        let spec = self.kind_infos.get(&neuron_kind).and_then(|info| info.variadic_output.clone()).ok_or_else(|| FlowCoreError::NotVariadicOutput(widget_id.to_string()))?;
        let widget = self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id).ok_or_else(|| FlowCoreError::UnknownWidget(widget_id.to_string()))?;
        let Widget::Neuron { output_ports, .. } = widget else {
            return Err(FlowCoreError::NotNeuron(widget_id.to_string()));
        };
        let ports = default_neuron_output_ports(&neuron_kind, output_ports, &self.kind_infos);
        if ports.len() <= spec.min {
            return Err(FlowCoreError::MinOutputPorts { widget: widget_id.to_string(), min: spec.min });
        }
        let Some(remove_index) = ports.iter().position(|port| port == port_id) else {
            return Err(FlowCoreError::UnknownOutputPort(port_id.to_string()));
        };
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.from == widget_id && synapse.from_port == port_id).map(|synapse| synapse.id.clone()).collect();
        self.host_snapshot.synapses.retain(|synapse| !(synapse.from == widget_id && synapse.from_port == port_id));
        let mut renumbered = Vec::new();
        for synapse in &mut self.host_snapshot.synapses {
            if synapse.from != widget_id {
                continue;
            }
            if let Ok(old_index) = synapse.from_port.parse::<usize>() {
                if old_index > remove_index {
                    synapse.from_port = (old_index - 1).to_string();
                    renumbered.push(synapse.clone());
                }
            }
        }
        let mut next_ports = ports;
        next_ports.remove(remove_index);
        *output_ports = (0..next_ports.len()).map(|slot| slot.to_string()).collect();
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(renumbered.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse }))).chain(changed));
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    pub fn disconnect(&mut self, synapse_id: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let before = self.host_snapshot.synapses.len();
        self.host_snapshot.synapses.retain(|s| s.id != synapse_id);
        if self.host_snapshot.synapses.len() == before {
            return Err(FlowCoreError::UnknownSynapse(synapse_id.to_string()));
        }
        self.note_leaves([FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.to_string() })]);
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    // #region GumballEditing
    /// 🔀️ Splices `mid_id` between `anchor_id` and its downstream consumers on `anchor_out_port`.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        if !self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == anchor_id) {
            return Err(FlowCoreError::UnknownWidget(anchor_id.to_string()));
        }
        if !self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == mid_id) {
            return Err(FlowCoreError::UnknownWidget(mid_id.to_string()));
        }
        if anchor_id == mid_id {
            return Err(FlowCoreError::SelfInsertion);
        }
        if !widget_has_output(anchor_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(anchor_id.to_string()));
        }
        if !widget_has_input(mid_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(mid_id.to_string()));
        }
        if !widget_has_output(mid_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(mid_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.to.clone())).collect();
        if would_create_cycle(&existing, anchor_id, mid_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        let mid_has_input = self.host_snapshot.synapses.iter().any(|synapse| synapse.to == mid_id);
        let mut rerouted = Vec::new();
        if !mid_has_input {
            for synapse in &mut self.host_snapshot.synapses {
                if synapse.from == anchor_id && synapse.from_port == anchor_out_port {
                    synapse.from = mid_id.to_string();
                    synapse.from_port = mid_out_port.to_string();
                    rerouted.push(synapse.clone());
                }
            }
        }
        self.note_leaves(rerouted.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })));
        if self.host_snapshot.synapses.iter().any(|synapse| synapse.from == anchor_id && synapse.from_port == anchor_out_port && synapse.to == mid_id && synapse.to_port == mid_in_port) {
            self.rebuild_dag();
            self.checked_change()?;
            return Ok(());
        }
        let synapse_id = self.next_synapse_id();
        let synapse = SynapseSpec { id: synapse_id, from: anchor_id.to_string(), to: mid_id.to_string(), from_port: anchor_out_port.to_string(), to_port: mid_in_port.to_string() };
        let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(self.host_snapshot.synapses.len()).unwrap_or(u32::MAX), synapse: synapse.clone() });
        self.host_snapshot.synapses.push(synapse);
        self.note_leaves([added]);
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    /// ↔ Shifts widgets to the right of `anchor_id` to open layout space for inserted nodes.
    pub fn make_space(&mut self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), FlowCoreError> {
        self.begin_change();
        let anchor_x = self.host_snapshot.layout.get(anchor_id).map(|layout| layout.x).ok_or_else(|| FlowCoreError::UnknownWidgetLayout(anchor_id.to_string()))?;
        let previous = std::mem::take(&mut self.host_snapshot.layout);
        let mut shifted = Vec::new();
        for (widget_id, layout) in &previous {
            let mut layout = layout.clone();
            if layout.x > anchor_x {
                layout.x += dx;
                layout.y += dy;
                shifted.push(widget_id.clone());
            }
            let _ = self.dag.graph.set_widget_position(widget_id, layout.x, layout.y);
            self.host_snapshot.layout.insert(widget_id.clone(), layout);
        }
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push_cold(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(previous));
        retirement.retire_cold();
        let moved = self.layout_leaf(shifted.iter().map(String::as_str));
        self.note_leaves(moved);
        self.checked_change()?;
        Ok(())
    }

    /// 🧬️ Merges JSON params into a neuron widget for compact transform values.
    pub fn set_neuron_params(&mut self, widget_id: &str, params_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let patch: Dictionary = semio_framework_pack_json::from_json_str(params_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let merged = match self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id) {
            Some(Widget::Neuron { params, .. }) => Ok(std::mem::replace(params, params.merge(&patch))),
            Some(_) => Err(FlowCoreError::NotNeuron(widget_id.to_string())),
            None => Err(FlowCoreError::UnknownWidget(widget_id.to_string())),
        };
        self.displace_dictionary(patch);
        self.displace_dictionary(merged?);
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.checked_change()?;
        Ok(())
    }
    // #endregion GumballEditing

    /// 🌳️ Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self,opts:&DagLayoutOptions,control:&mut semio_framework_os_infinite::board::schema::layout::LayoutControl<'_>)->Result<(),FlowCoreError>{
        let mut dag=DagHost::from_host_snapshot_without_layout(self.build_dag_host_snapshot_v1());dag.viewport.canvas_theme=self.dag.viewport.canvas_theme;
        dag.graph.reorganize(opts,control)?;self.begin_change();self.dag=dag;self.sync_from_dag();
        let placed: Vec<String> = self.host_snapshot.layout.keys().cloned().collect();
        let moved = self.layout_leaf(placed.iter().map(String::as_str));
        self.note_leaves(moved);
        self.checked_change()?;
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "pointer-event handler mirroring this file's other screen-space input methods (pointer_move_screen/pointer_up_screen/wheel_screen) — position + button + modifier-key flags is the natural shape for this UI event, not a bundling candidate on its own without also restructuring its siblings"
    )]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if pan {
            self.pan_anchor = Some((sx, sy, self.host_snapshot.camera.x, self.host_snapshot.camera.y));
            return;
        }
        self.clear_ghost_widget();
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.begin_gesture();
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, false);
        if let Some((side, widget_id, index)) = self.dag.pointer.take_pending_port_insert() {
            let inserted = match side {
                dag::DagPortSide::Input => self.add_input_port(&widget_id, index),
                dag::DagPortSide::Output => self.add_output_port(&widget_id, index),
            };
            if inserted.is_ok() {
                self.dag.graph.journal_port_insert(widget_id, side, index);
            }
            return;
        }
        self.sync_from_dag();
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.host_snapshot.camera.zoom;
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            return;
        }
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
    }

    pub fn widget_drag_active(&self) -> bool {
        self.dag.pointer.widget_drag_active()
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.pan_anchor = None;
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    pub fn pointer_cancel_screen(&mut self) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.pan_anchor = None;
        self.dag.pointer_cancel_screen();
        if !self.gesture_active {
            return;
        }
        self.gesture_active = false;
        self.discard_pending_leaves();
        self.edited_note = None;
        let Some(mut baseline) = self.pending_history_baseline.take() else {
            return;
        };
        baseline.camera = self.host_snapshot.camera.clone();
        let stale = std::mem::replace(&mut self.host_snapshot, baseline);
        stale.retire_cold();
        self.rebuild_dag();
    }

    /// 🔗️ Drains the graph edits the last gesture performed as the `nodeGraphEdit` arguments the renderer dispatches
    /// (`dag::dag_graph_edit_rows_json`: `connect`, `disconnect`, `move` — the node-graph gesture record of design §13.3 —
    /// `setSlider`, `insertPort`). The journal narrates EVERY content change a gesture makes, so the renderer never
    /// re-publishes the whole fixture; a plain click, a marquee, a pan and a press that grabbed nothing journal nothing
    /// and are owed no dispatch at all.
    pub fn take_graph_edits_json(&mut self) -> String {
        let edits = self.dag.graph.take_graph_edits();
        let refusal = self.dag.graph.take_journal_refusal();
        dag::dag_graph_edit_rows_json(&edits, refusal)
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str) {
        self.dag.graph.set_selection_options(method, mode, true, true, true);
    }

    pub fn selection_preview_points_json(&self) -> String {
        self.dag.graph.selection_preview_points_json()
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.dag.graph.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.dag.graph.selection_preview_method()
    }

    pub fn preselect_widget_ids_json(&self) -> String {
        semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("ids".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&self.dag.graph.preselect_widget_ids()))),
            ("removedIds".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&self.dag.graph.preselect_removed_widget_ids()))),
        ]))
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let cancelled = self.dag.graph.cancel_area_select();
        if cancelled {
            self.sync_from_dag();
        }
        cancelled
    }

    pub fn delete_selection(&mut self) -> Result<(), FlowCoreError> {
        if !self.dag.graph.has_selection() {
            return Ok(());
        }
        self.begin_change();
        let nodes: Vec<String> = self.dag.graph.selected_node_ids().into_iter().filter(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id)).collect();
        let edges = self.dag.graph.selected_edge_ids();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| edges.contains(&synapse.id) || nodes.contains(&synapse.from) || nodes.contains(&synapse.to)).map(|synapse| synapse.id.clone()).collect();
        let unplaced: Vec<FlowLayoutEntry> = nodes.iter().filter(|id| self.host_snapshot.layout.contains_key(id.as_str())).map(|id| FlowLayoutEntry { id: id.clone(), layout: None }).collect();
        self.dag.graph.delete_selected();
        self.sync_from_dag();
        let unplaced = (!unplaced.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: unplaced }));
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain(nodes.into_iter().map(|id| FlowMutation::RemoveWidget(RemoveWidget { id }))));
        self.checked_change()?;
        Ok(())
    }

    /// ✅️ Whether the canvas has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        self.dag.graph.has_selection()
    }

    pub fn select_all(&mut self) {
        self.dag.graph.select_all();
        self.sync_from_dag();
    }
}
