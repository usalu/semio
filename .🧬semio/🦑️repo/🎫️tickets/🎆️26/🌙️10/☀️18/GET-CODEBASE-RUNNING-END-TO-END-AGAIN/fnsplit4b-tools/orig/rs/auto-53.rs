//! 🌱️ Generation3d retained store initialization: the snapshot copy cursor and the store initialization authority that replays a document envelope into its store.
#![allow(clippy::result_large_err)]
#![allow(unused_imports)]
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_s_artifact_procedural_3d_mut_generations::change_generation_value::ChangeGenerationValue;
use semio_s_artifact_procedural_3d_mut_document::change_schema::ChangeSchema;
use semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse;
use semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration;
use semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget;
use semio_s_artifact_procedural_3d_mut_generations::delete_generation::DeleteGeneration;
use semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget;
use semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition;
use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;
use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;
use semio_s_artifact_procedural_3d_mut_generations::rename_generation::RenameGeneration;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_mut_document::update_camera::UpdateCamera;
use semio_s_artifact_procedural_3d_mut_synapses::update_synapse::UpdateSynapse;
use semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget;
use semio_s_artifact_procedural_3d_io_text::snapshot::camera_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::camera_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::form_generation_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::form_generation_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::layout_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::layout_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::synapse_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::synapse_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::widget_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::widget_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::CameraJsonDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::FormGenerationDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::SynapseSpecDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::WidgetDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::WidgetLayoutDsl;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::ChangeSliderValue;
use semio_s_artifact_procedural_3d_mut_transforms::drag_transforms::DragTransforms;
use semio_s_artifact_procedural_3d_mut_transforms::rotate_transforms::RotateTransforms;
use semio_s_artifact_procedural_3d_mut_transforms::scale_transforms::ScaleTransforms;
use semio_s_artifact_procedural_3d_mut_widgets::move_nodes::MoveNodes;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::ChangeWidgetInput;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
use semio_s_artifact_procedural_3d_mut_generations::change_generation_preview::ChangeGenerationPreview;
use semio_s_artifact_procedural_3d_mut_generations::select_generation::SelectGeneration;
use protocol::OpBinary;
use store::ErasedSnapshotRetirement;
use semio_s_artifact_procedural_3d_host::GENERATION3D_MAXIMUM_DOMAIN_ITEMS;
use semio_s_artifact_procedural_3d_host::GENERATION3D_OWNER_BYTES;
use semio_s_artifact_procedural_3d_host::GENERATION3D_RETAINED_STACK_CAPACITY;
use semio_s_artifact_procedural_3d_host::Generation3dReplayDisplaced;
use semio_s_artifact_procedural_3d_host::generation3d_admit_app_publication_authority;
use semio_s_artifact_procedural_3d_host::generation3d_apply_initialization_mutation;
use semio_s_artifact_procedural_3d_host::generation3d_copy_generation;
use semio_s_artifact_procedural_3d_host::generation3d_copy_string;
use semio_s_artifact_procedural_3d_host::generation3d_copy_synapse;
use semio_s_artifact_procedural_3d_host::generation3d_copy_widget;
use semio_s_artifact_procedural_3d_host::generation3d_release_app_publication_authority;
use semio_s_artifact_procedural_3d_host::generation3d_validate_publication_authority;

//#region 🔖️RetainedStoreInitialization


fn generation3d_copy_neural_value(source: &semio_framework_artifact_flow_flow::neural::Value, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Value, &'static str> {
    if depth >= GENERATION3D_RETAINED_STACK_CAPACITY {
        return Err("generation3d-initializer.neural-depth");
    }
    Ok(match source {
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null) => semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null),
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(generation3d_copy_string(value)?))
        }
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(value) => semio_framework_artifact_flow_flow::neural::Value::Dictionary(generation3d_copy_dictionary(value, depth + 1)?),
    })
}

fn generation3d_copy_dictionary(source: &semio_framework_artifact_flow_flow::neural::Dictionary, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Dictionary, &'static str> {
    let mut target = semio_framework_artifact_flow_flow::neural::Dictionary::new();
    for key in source.keys() {
        let value = source.get(key).ok_or("generation3d-initializer.dictionary-owner")?;
        target = target.insert(generation3d_copy_string(key)?, generation3d_copy_neural_value(value, depth + 1)?);
    }
    Ok(target)
}

fn generation3d_copy_tree(source: &semio_framework_artifact_flow_flow::neural::Tree, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Tree, &'static str> {
    if depth >= GENERATION3D_RETAINED_STACK_CAPACITY {
        return Err("generation3d-initializer.tree-depth");
    }
    let mut neurons = Vec::new();
    neurons.try_reserve_exact(source.neurons.len()).map_err(|_| "generation3d-initializer.neurons-preflight")?;
    for neuron in &source.neurons {
        neurons.push(semio_framework_artifact_flow_flow::neural::Neuron {
            id: generation3d_copy_string(&neuron.id)?,
            kind: generation3d_copy_string(&neuron.kind)?,
            params: generation3d_copy_dictionary(&neuron.params, depth + 1)?,
            tree: match neuron.tree.as_deref() {
                Some(tree) => Some(Box::new(generation3d_copy_tree(tree, depth + 1)?)),
                None => None,
            },
        });
    }
    let mut synapses = Vec::new();
    synapses.try_reserve_exact(source.synapses.len()).map_err(|_| "generation3d-initializer.tree-synapses-preflight")?;
    for synapse in &source.synapses {
        synapses.push(semio_framework_artifact_flow_flow::neural::Synapse {
            id: generation3d_copy_string(&synapse.id)?,
            from: generation3d_copy_string(&synapse.from)?,
            to: generation3d_copy_string(&synapse.to)?,
            from_port: generation3d_copy_string(&synapse.from_port)?,
            to_port: generation3d_copy_string(&synapse.to_port)?,
        });
    }
    Ok(semio_framework_artifact_flow_flow::neural::Tree { neurons, synapses })
}

fn generation3d_copy_flow_ui(source: &semio_framework_artifact_flow_flow::FlowGui) -> Result<semio_framework_artifact_flow_flow::FlowGui, &'static str> {
    let mut nodes = semio_framework_artifact_flow_flow::OrderedMap::new();
    for (id, node) in &source.nodes {
        let chrome = match &node.chrome {
            semio_framework_artifact_flow_flow::NodeChrome::Plain { preview } => semio_framework_artifact_flow_flow::NodeChrome::Plain { preview: *preview },
            semio_framework_artifact_flow_flow::NodeChrome::Slider { label, min, max, step, value } => {
                semio_framework_artifact_flow_flow::NodeChrome::Slider { label: generation3d_copy_string(label)?, min: *min, max: *max, step: *step, value: *value }
            }
            semio_framework_artifact_flow_flow::NodeChrome::Note { text } => semio_framework_artifact_flow_flow::NodeChrome::Note { text: generation3d_copy_string(text)? },
            semio_framework_artifact_flow_flow::NodeChrome::Image { src } => semio_framework_artifact_flow_flow::NodeChrome::Image { src: generation3d_copy_string(src)? },
            semio_framework_artifact_flow_flow::NodeChrome::Variable { name, schema } => semio_framework_artifact_flow_flow::NodeChrome::Variable { name: generation3d_copy_string(name)?, schema: generation3d_copy_string(schema)? },
        };
        nodes.insert(generation3d_copy_string(id)?, semio_framework_artifact_flow_flow::FlowNodeGui { layout: semio_framework_artifact_flow_flow::WidgetLayout { x: node.layout.x, y: node.layout.y }, chrome });
    }
    let mut previews = Vec::new();
    previews.try_reserve_exact(source.previews.len()).map_err(|_| "generation3d-initializer.previews-preflight")?;
    for preview in &source.previews {
        let source = match &preview.source {
            Some(source) => Some(semio_framework_artifact_flow_flow::FlowChannelRef { neuron: generation3d_copy_string(&source.neuron)?, channel: generation3d_copy_string(&source.channel)? }),
            None => None,
        };
        let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
        for value in &preview.expanded {
            expanded.insert(generation3d_copy_string(value)?);
        }
        previews.push(semio_framework_artifact_flow_flow::FlowPreviewGui {
            id: generation3d_copy_string(&preview.id)?,
            source,
            mode: generation3d_copy_string(&preview.mode)?,
            preview: generation3d_copy_dictionary(&preview.preview, 0)?,
            expanded,
            layout: preview.layout.as_ref().map(|layout| semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x, y: layout.y }),
        });
    }
    Ok(semio_framework_artifact_flow_flow::FlowUi { camera: semio_framework_artifact_flow_flow::CameraJson { x: source.camera.x, y: source.camera.y, zoom: source.camera.zoom }, nodes, previews })
}




struct Generation3dSnapshotCopyCursor {
    target: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    phase: u8,
    index: usize,
    handed_back: bool,
}

impl Generation3dSnapshotCopyCursor {
    fn new(source: &Generation3dSnapshot) -> Result<Self, &'static str> {
        let mut target = Generation3dSnapshot {
            host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot {
                schema: String::new(),
                camera: semio_framework_artifact_flow_flow::CameraJson::default(),
                widgets: Vec::new(),
                synapses: Vec::new(),
                layout: semio_framework_artifact_flow_flow::OrderedMap::new(),
            },
            generation: semio_framework_artifact_playbook_playbook::GenerationPlayState::default().into(),
        };
        target.host_snapshot.widgets.try_reserve_exact(source.host_snapshot.widgets.len()).map_err(|_| "generation3d-initializer.widgets-preflight")?;
        target.host_snapshot.synapses.try_reserve_exact(source.host_snapshot.synapses.len()).map_err(|_| "generation3d-initializer.synapses-preflight")?;
        target.generation.cold_builder_mut()?.generations.try_reserve_exact(source.generation.generations.len()).map_err(|_| "generation3d-initializer.generations-preflight")?;
        Ok(Self { target: std::mem::ManuallyDrop::new(Some(target)), phase: 0, index: 0, handed_back: false })
    }

    fn step(&mut self, source: &Generation3dSnapshot) -> Result<bool, &'static str> {
        let target = self.target.as_mut().ok_or("generation3d-initializer.copy-owner")?;
        match self.phase {
            0 => {
                target.host_snapshot.schema = generation3d_copy_string(&source.host_snapshot.schema)?;
                self.phase = 1;
            }
            1 => {
                target.host_snapshot.camera.x = source.host_snapshot.camera.x;
                self.phase = 2;
            }
            2 => {
                target.host_snapshot.camera.y = source.host_snapshot.camera.y;
                self.phase = 3;
            }
            3 => {
                target.host_snapshot.camera.zoom = source.host_snapshot.camera.zoom;
                self.phase = 4;
            }
            4 if self.index < source.host_snapshot.widgets.len() => {
                target.host_snapshot.widgets.push(generation3d_copy_widget(&source.host_snapshot.widgets[self.index])?);
                self.index += 1;
            }
            4 => {
                self.phase = 5;
                self.index = 0;
            }
            5 if self.index < source.host_snapshot.synapses.len() => {
                target.host_snapshot.synapses.push(generation3d_copy_synapse(&source.host_snapshot.synapses[self.index])?);
                self.index += 1;
            }
            5 => {
                self.phase = 6;
                self.index = 0;
            }
            6 if self.index < source.host_snapshot.layout.len() => {
                let (id, layout) = source.host_snapshot.layout.iter().nth(self.index).ok_or("generation3d-initializer.layout-owner")?;
                target.host_snapshot.layout.insert(generation3d_copy_string(id)?, semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x, y: layout.y });
                self.index += 1;
            }
            6 => {
                self.phase = 7;
                self.index = 0;
            }
            7 if self.index < source.generation.generations.len() => {
                target.generation.cold_builder_mut()?.generations.push(generation3d_copy_generation(&source.generation.generations[self.index])?);
                self.index += 1;
            }
            7 => {
                target.generation.cold_builder_mut()?.selected_generation_id = match source.generation.selected_generation_id.as_deref() {
                    Some(value) => Some(generation3d_copy_string(value)?),
                    None => None,
                };
                self.phase = 8;
            }
            8 => {
                target.generation.cold_builder_mut()?.preview_text = match source.generation.preview_text.as_deref() {
                    Some(value) => Some(generation3d_copy_string(value)?),
                    None => None,
                };
                self.phase = 9;
            }
            _ => return Ok(true),
        }
        Ok(self.phase == 9)
    }

    fn take(&mut self) -> Option<Generation3dSnapshot> {
        if self.phase != 9 || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.target.take()
    }

    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.target.is_none() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.target)
    }

    fn abandon(&mut self, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        if self.target.is_none() {
            self.handed_back = true;
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.target, active, grant)?;
        if self.target.is_none() {
            self.handed_back = true;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.target.is_none()
    }
}

impl Drop for Generation3dSnapshotCopyCursor {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation3d snapshot copy cursor reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation3dStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CensusHistory { edit: usize, mutation: usize },
    CopyInitial,
    AdoptWorkspace,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildOwners,
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

pub struct Generation3dStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    history_items: usize,
    envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>>>,
    copy: std::mem::ManuallyDrop<Option<Generation3dSnapshotCopyCursor>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<Generation3dSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>>>,
    owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<Generation3dSnapshot, Generation3dMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    displaced: std::mem::ManuallyDrop<Option<Generation3dReplayDisplaced>>,
    rejected: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Generation3dSnapshot, Generation3dMutation>>>,
    closer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactStoreInitializationJob<Generation3dSnapshot, Generation3dMutation>>>,
    snapshot_factory: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Generation3dSnapshot>>>>,
    factory_close: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    publication: semio_framework_job_publication::RetainedJobPublication,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    pub phase: Generation3dStoreInitializationPhase,
    resume_phase: Option<Generation3dStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl Generation3dStoreInitializationAuthority {
    pub fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        if generation3d_validate_publication_authority(operation, generation).is_err() {
            let _ = generation3d_admit_app_publication_authority(operation, generation);
        }
        let (base_revision, parent_revision) = generation3d_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));
        Self {
            actor,
            operation,
            generation,
            base_revision,
            parent_revision,
            history_items: 0,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            copy: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            owners: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            displaced: std::mem::ManuallyDrop::new(None),
            rejected: std::mem::ManuallyDrop::new(None),
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            closer: std::mem::ManuallyDrop::new(None),
            snapshot_factory: std::mem::ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()))),
            factory_close: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job_publication::RetainedJobPublication::new(),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: Generation3dStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn fail(&mut self, code: &'static [u8]) {
        let mut value = Vec::new();
        if value.try_reserve_exact(code.len()).is_ok() {
            value.extend_from_slice(code);
        }
        self.fault = Some(value);
        self.phase = Generation3dStoreInitializationPhase::RetireFault;
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn pump_owned(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        if self.active.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.displaced.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.rejected.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected, &mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        Ok(false)
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "generation3d initializer close depth overflow"))?;
            Ok(demand)
        };
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if let Some(owner) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.displaced.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.displaced);
        }
        if self.rejected.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.rejected);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            return match self.candidate_disposer.as_ref() {
                Some(disposer) => nested(disposer.retirement_demands(candidate, body)?),
                None => Ok(RetirementDemand { depth: 1, ..Default::default() }),
            };
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return nested(runtime.initialization_retirement_demands(body)?);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.constructor_is_complete() && owners.uninstalled_owners_terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else if owners.constructor_is_complete() { nested(owners.uninstalled_owners_demands(body)?) } else { nested(owners.constructor_demands(body)?) };
        }
        if let Some(copy) = self.copy.as_ref() {
            return copy.close_demands();
        }
        if let Some(closer) = self.closer.as_ref() {
            return nested(closer.retirement_demands(body)?);
        }
        if self.envelope.is_some() {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(factory) = self.factory_close.as_ref() {
            return nested(factory.demands(body)?);
        }
        if self.snapshot_factory.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        use semio_framework_value::{retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress}, FactoryAuthority, ValueError, ValueRefusalKind};
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if demand == Default::default() {
            return Ok(Default::default());
        }
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "generation3d initializer close exceeds its admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(Default::default());
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| step.progress());
        }
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| step.progress());
        }
        if self.displaced.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant).map(|step| step.progress());
        }
        if self.rejected.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.rejected, &mut self.active, grant).map(|step| step.progress());
        }
        if self.candidate.is_some() {
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(item);
            }
            let candidate = self.candidate.as_mut().expect("observed original candidate store");
            let disposer = self.candidate_disposer.as_mut().expect("observed original candidate disposer");
            let step = disposer.close_step(candidate, child).map_err(|error| ValueError::new(ValueRefusalKind::InvariantViolated, error.describe()))?;
            if disposer.terminal_is_empty(candidate) {
                *self.candidate_disposer = None;
                drop(self.candidate.take());
            }
            return Ok(step.progress().unwrap_or_default());
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory = self.snapshot_factory.as_ref().expect("initializer snapshot issuer outlives its runtime");
            let step = runtime.close_step(factory, child)?;
            let step = admit_retained_clone_close(child, step, runtime.terminal_is_empty(), "generation3d initializer runtime close")?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(step.progress());
        }
        if let Some(owners) = self.owners.as_mut() {
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(item);
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            let step = admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "generation3d initializer uninstalled owners close")?;
            return Ok(step.progress());
        }
        if let Some(copy) = self.copy.as_mut() {
            let step = copy.abandon(&mut self.active, grant)?;
            if copy.terminal_is_empty() && self.active.is_none() {
                drop(self.copy.take());
            }
            return Ok(step.progress());
        }
        if self.closer.is_some() {
            let closer = self.closer.as_mut().expect("observed original envelope closer");
            semio_framework_job::InteractiveJob::begin_close(closer);
            let demand = closer.retirement_demands(child.maximum_copy_bytes)?;
            let step = closer.close_granted(child, demand)?;
            if closer.terminal_is_empty() {
                drop(self.closer.take());
            }
            return Ok(step.progress());
        }
        if let Some(envelope) = self.envelope.take() {
            *self.closer = Some(semio_framework_plugin::bounded_document_store_initialization_job::<_,_,::semio_framework_os_kernel::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>(envelope, semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA, self.operation, self.generation, self.actor.clone()));
            return Ok(item);
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            let step = admit_retained_clone_close(child, step, factory.terminal_is_empty(), "generation3d initializer issuer close")?;
            if factory.terminal_is_empty() {
                *self.factory_close = None;
            }
            return Ok(step.progress());
        }
        if let Some(factory) = self.snapshot_factory.take() {
            let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.factory_close = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_is_empty(&self) -> bool {
        self.envelope.is_none()
            && self.copy.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.displaced.is_none()
            && self.rejected.is_none()
            && self.candidate_disposer.is_none()
            && self.closer.is_none()
            && self.snapshot_factory.is_none()
            && self.factory_close.is_none()
            && self.publication.terminal_is_empty()
    }

    fn close_is_complete(&self) -> bool {
        self.close_is_empty()
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.close_is_empty()
    }
}

const GENERATION3D_INITIALIZER_DEFAULT_FAULT: &[u8] = b"generation3d-store.initializer-fault";

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<Generation3dSnapshot, Generation3dMutation> for Generation3dStoreInitializationAuthority {
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(body)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"generation3d-store.initializer-stale-aba");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, Generation3dStoreInitializationPhase::RetireCancelled | Generation3dStoreInitializationPhase::Cancelled) {
            self.phase = Generation3dStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(None);
        }
        match self.pump_owned(cx) {
            Ok(true) => {
                cx.consume_fuel(1);
                return Ok(None);
            }
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = Generation3dStoreInitializationPhase::RetireFault;
            }
        }
        if !matches!(self.phase, Generation3dStoreInitializationPhase::RetireCancelled | Generation3dStoreInitializationPhase::RetireFault | Generation3dStoreInitializationPhase::Cancelled | Generation3dStoreInitializationPhase::Fault | Generation3dStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) => {}
                    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)) => {
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(1);
                        return Ok(None);
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = Generation3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
        }
        match self.phase {
            Generation3dStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone()));
                self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                return Ok(None);
            }
            Generation3dStoreInitializationPhase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA && !envelope.id.is_empty() && envelope.id.len() <= GENERATION3D_OWNER_BYTES);
                if valid {
                    self.phase = Generation3dStoreInitializationPhase::ValidateEdit { index: 0 };
                } else {
                    self.fail(b"generation3d-store.initializer-envelope-invalid");
                }
            }
            Generation3dStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("P3 envelope retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = Generation3dStoreInitializationPhase::CensusHistory { edit: 0, mutation: 0 },
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = Generation3dStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"generation3d-store.initializer-duplicate-edit"),
                }
            }
            Generation3dStoreInitializationPhase::CensusHistory { edit, mutation } => {
                let envelope = self.envelope.as_ref().expect("P3 envelope retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation3dStoreInitializationPhase::BindGenesis;
                    return Ok(None);
                };
                if entry.forwards.get(mutation).is_some() {
                    self.history_items = match self.history_items.checked_add(1) {
                        Some(value) if value <= GENERATION3D_MAXIMUM_DOMAIN_ITEMS => value,
                        _ => {
                            self.fail(b"generation3d-store.initializer-history-capacity");
                            return Ok(None);
                        }
                    };
                    self.phase = Generation3dStoreInitializationPhase::CensusHistory { edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation3dStoreInitializationPhase::CensusHistory { edit: edit + 1, mutation: 0 };
                }
            }
            Generation3dStoreInitializationPhase::CopyInitial => {
                let source = &self.envelope.as_ref().expect("P3 initializer envelope").vcs.genesis.facts().snapshot();
                match self.copy.as_mut().expect("P3 copy retained").step(source) {
                    Ok(true) => self.phase = Generation3dStoreInitializationPhase::AdoptWorkspace,
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Generation3dStoreInitializationPhase::AdoptWorkspace => {
                let initial = self.copy.as_mut().expect("P3 copy retained").take().expect("P3 copy handoff");
                drop(self.copy.take());
                let factory = std::sync::Arc::clone(self.snapshot_factory.as_ref().expect("P3 snapshot issuer retained"));
                match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, factory) {
                    Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                    Err(initial) => {
                        *self.rejected = Some(initial);
                        self.fail(b"initializer-owned-workspace-adoption");
                    }
                }
            }
            Generation3dStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("P3 history retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation3dStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("P3 runtime retained");
                match lane {
                    0 => match runtime.seed_mutation(protocol::MutationId(generation3d_copy_string(&entry.id).unwrap_or_default())) {
                        Ok(()) => {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                        Err(error) => {
                            self.fault = Some(error.into_bytes());
                            self.phase = Generation3dStoreInitializationPhase::RetireFault;
                        }
                    },
                    1 if index < entry.forwards.len() => {
                        let id = entry
                            .mutation_meta
                            .get(index)
                            .and_then(|meta| meta.mutation_id.as_ref()).map_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)), |id| protocol::MutationId(generation3d_copy_string(&id.0).unwrap_or_default()));
                        match runtime.seed_edit_operation(&entry.id, id) {
                            Ok(()) => self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 },
                            Err(error) => {
                                self.fault = Some(error.into_bytes());
                                self.phase = Generation3dStoreInitializationPhase::RetireFault;
                            }
                        }
                    }
                    1 => self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
            }
            Generation3dStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("P3 envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("P3 runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = Generation3dStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = Generation3dStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Generation3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            Generation3dStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self
                        .envelope
                        .as_ref()
                        .and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| &checkpoint.id)))
                        .and_then(|id| generation3d_copy_string(id).ok());
                    self.runtime.as_mut().expect("P3 runtime retained").set_current_checkpoint_id(checkpoint);
                    self.phase = Generation3dStoreInitializationPhase::FindRedo { position: 0 };
                    cx.consume_fuel(1);
                    return Ok(None);
                };
                let scan = self.edit_index.position(id).unwrap_or(usize::MAX);
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                    }
                    Some(_) => self.fail(b"generation3d-store.initializer-applied-missing"),
                    None => self.fail(b"generation3d-store.initializer-applied-missing"),
                }
            }
            Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    match Generation3dSnapshotCopyCursor::new(self.envelope.as_ref().expect("retained genesis").vcs.genesis.facts().snapshot()) { Ok(copy) => *self.copy = Some(copy), Err(code) => { self.fail(code.as_bytes()); return Ok(None); } }
                    self.phase = Generation3dStoreInitializationPhase::CopyInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
                let Some(_operation) = operation else {
                    self.phase = Generation3dStoreInitializationPhase::CommitApplied { position, edit };
                    return Ok(None);
                };
                let envelope = self.envelope.as_ref().expect("P3 envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("P3 applied edit remains retained");
                let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("P3 applied forward remains retained");
                if effective.operation().is_none() {
                    drop(effective);
                    self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    return Ok(None);
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("P3 runtime current retained");
                let applied = effective.operation().map(|operation| generation3d_apply_initialization_mutation(current, operation));
                drop(effective);
                match applied {
                    Some(Ok(displaced)) => {
                        *self.displaced = displaced;
                        self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    }
                    None => self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 },
                    Some(Err(code)) => self.fail(code.as_bytes()),
                }
            }
            Generation3dStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P3 applied edit retained");
                let runtime = self.runtime.as_mut().expect("P3 runtime retained");
                match runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => {
                        runtime.observe_sequence(entry.sequence_number);

                        self.phase = Generation3dStoreInitializationPhase::FindApplied { position: position + 1 };
                    }
                    Err(_) => self.fail(b"generation3d-store.initializer-applied-capacity"),
                }
            }
            Generation3dStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = Generation3dStoreInitializationPhase::BuildOwners;
                    cx.consume_fuel(1);
                    return Ok(None);
                };
                let scan = self.edit_index.position(id).unwrap_or(usize::MAX);
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        self.phase = Generation3dStoreInitializationPhase::CommitRedo { position, edit: scan };
                    }
                    Some(_) => self.fail(b"generation3d-store.initializer-redo-missing"),
                    None => self.fail(b"generation3d-store.initializer-redo-missing"),
                }
            }
            Generation3dStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P3 redo edit retained");
                match self.runtime.as_mut().expect("P3 runtime retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => self.phase = Generation3dStoreInitializationPhase::FindRedo { position: position + 1 },
                    Err(_) => self.fail(b"generation3d-store.initializer-redo-capacity"),
                }
            }
            Generation3dStoreInitializationPhase::BuildOwners => {
                if self.owners.is_none() {
                    match store::os_store::document_store_owners::bounded_artifact_store_owners::<Generation3dSnapshot, Generation3dMutation>(cx.retained_grant()) {
                        Ok((owners, progress)) => {
                            *self.owners = Some(owners);
                            cx.consume_retained(progress)?;
                        }
                        Err(refused) => {
                            *self.owners = refused.owners;
                            if refused.error.retained_progress() != Default::default() {
                                cx.consume_retained(refused.error.retained_progress())?;
                            }
                        }
                    }
                } else if let Some(owners) = self.owners.as_mut().filter(|owners| !owners.constructor_is_complete()) {
                    match owners.admit_constructor(cx.retained_grant()) {
                        Ok(progress) => cx.consume_retained(progress)?,
                        Err((error, progress)) => {
                            cx.consume_retained(progress)?;
                            return Err(error);
                        }
                    }
                } else {
                    self.phase = Generation3dStoreInitializationPhase::BuildCandidate;
                }
                cx.consume_fuel(1);
                return Ok(None);
            }
            Generation3dStoreInitializationPhase::BuildCandidate => {
                let authority = generation3d_validate_publication_authority(self.operation, self.generation);
                let fresh =
                    cx.operation() == self.operation && cx.generation() == self.generation && authority == Ok((self.base_revision, self.parent_revision)) && self.base_revision == self.parent_revision && self.parent_revision == self.generation.0;
                let Some(candidate_generation) = self.parent_revision.checked_add(1) else {
                    self.fail(b"generation3d-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                if !fresh {
                    self.fail(b"generation3d-store.initializer-parent-stale-aba");
                    return Ok(None);
                }
                let envelope = self.envelope.take().expect("P3 envelope retained until atomic publication");
                let runtime = self.runtime.take().expect("P3 runtime retained until atomic publication");
                let owners = self.owners.take().expect("P3 owners retained until atomic publication");
                *self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners));
                self.phase = Generation3dStoreInitializationPhase::Complete;
                return JobOutcomeBorrow::admit_complete(cx, None, None);
            }
            Generation3dStoreInitializationPhase::RetireCancelled | Generation3dStoreInitializationPhase::RetireFault => match self.close_turn(cx.retained_grant()) {
                Ok(progress) => {
                    cx.consume_retained(progress)?;
                    if !self.close_is_complete() {
                        return Ok(None);
                    }
                    generation3d_release_app_publication_authority(self.operation);
                    self.terminal_handoff = true;
                    if self.phase == Generation3dStoreInitializationPhase::RetireCancelled {
                        self.phase = Generation3dStoreInitializationPhase::Cancelled;
                        return JobOutcomeBorrow::admit_cancelled(cx);
                    }
                    self.phase = Generation3dStoreInitializationPhase::Fault;
                    return self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(GENERATION3D_INITIALIZER_DEFAULT_FAULT), cx);
                }
                Err(_) => self.fail(b"generation3d-store.initializer-close"),
            },
            Generation3dStoreInitializationPhase::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None),
            Generation3dStoreInitializationPhase::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
            Generation3dStoreInitializationPhase::Fault => return self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(GENERATION3D_INITIALIZER_DEFAULT_FAULT), cx),
        }
        cx.consume_fuel(1);
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }
    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>> {
        if self.phase != Generation3dStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, Generation3dStoreInitializationPhase::Cancelled | Generation3dStoreInitializationPhase::Fault) {
            self.phase = Generation3dStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneStep;
        self.begin_close();
        let progress = self.close_turn(grant)?;
        if self.close_is_complete() {
            generation3d_release_app_publication_authority(self.operation);
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(progress));
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for Generation3dStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty_inner()), "Generation3d initializer reached Drop before candidate handoff or terminal-empty close");
    }
}

pub fn generation3d_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<Generation3dSnapshot, Generation3dMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(Generation3dStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}
//#endregion 🔖️RetainedStoreInitialization
