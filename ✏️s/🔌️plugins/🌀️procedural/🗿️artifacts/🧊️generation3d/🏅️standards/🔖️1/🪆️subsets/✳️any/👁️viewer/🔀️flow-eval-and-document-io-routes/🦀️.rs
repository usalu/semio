//! 🔀️ Flow eval and document IO routes: the flow eval job factory and the document IO contract, work and job factory with their proofs. See [`super`].

use super::*;

//#region ⏱️FlowEvalRoute
pub(super) fn generation3d_view_flow_eval_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// ⏱️ The three runtime chain routes, on their OWN factory: their publication lanes are nothing
/// like a view command's. `flowEvalTick`'s only store lane is the ADDRESSED preview window's own
/// transient (never config, never presence); the two resolves write no store lane at all.
pub(super) struct Generation3dViewFlowEvalJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewFlowEvalJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewFlowEvalJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_FLOW_EVAL_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_flow_eval_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer flow-eval command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewFlowEvalJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_FLOW_EVAL_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🔒️ Still read-only: `WindowTransient` is an EPHEMERAL lane. Naming
    /// `ArtifactToolPublicationLane::Artifact` here would be rejected against a viewer's own emit,
    /// and no row does.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateResolve", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "flowEvalRelease", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "flowTessellateCancelResolve", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}

pub(super) struct Generation3dViewFlowEvalJobFactoryProofs;

impl Generation3dViewFlowEvalJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        artifact_schema: "generation.3d",
        factory: "Generation3dViewFlowEvalJobFactory",
        factory_type: Generation3dViewFlowEvalJobFactory,
        contract: generation3d_view_flow_eval_contract(),
        tools: ["flowEvalTick", "flowEvalResolve", "flowTessellateResolve", "flowEvalRelease", "flowTessellateCancelResolve"]
    }
}
//#endregion ⏱️FlowEvalRoute

//#region 📤️DocumentIoRoute
fn generation3d_view_document_io_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_DOCUMENT_IO_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 📤️ One export hop. Deliberately NOT [`Generation3dViewCommandWork`]: that work broadcasts the
/// camera and show mode as ephemeral presence after every command, and an export changes neither —
/// publishing presence for it would tell every co-viewer something moved when nothing did.
pub(super) struct Generation3dViewDocumentIoWork {
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewDocumentIoWork {
    fn tool_id(&self) -> &'static str {
        "exportDocument"
    }

    fn extent(
        &self,
        command: &Generation3dViewCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<Generation3dViewer>>>,
    ) -> Option<usize> {
        generation3d_view_bounded_extent(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<ViewerApp<Generation3dViewer>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ViewerApp<Generation3dViewer>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<ViewerApp<Generation3dViewer>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("generation3d-view-document-io-work-repeated"));
        }
        self.consumed = true;
        let Generation3dViewCommand::ExportDocument(payload) = input.command else {
            return Err(Fault::from("generation3d-view-document-io-route-rejected"));
        };
        /// 👁️ A viewer exports WHAT IT SHOWS. Every other viewer surface resolves the viewed document
        /// through `Generation3dViewedDocument::resolve` — the active example replaces the opened
        /// snapshot — and this one alone read `input.snapshot` raw. With an example picked, the export
        /// therefore encoded a document the session never evaluated, and every format failed with
        /// `the document evaluates to no preview geometry (no positions)` while three meshes stood on
        /// screen (measured on :6023, ticket 26/09/09/PROCEDURAL-3D-END-TO-END gap F2).
        let viewed = Generation3dViewedDocument::resolve(input.snapshot, input.config);
        let cfg = ConfigView { snapshot: input.config, window: None };
        let emitted = {
            let doc = ArtifactView::with_operation(viewed.snapshot(), input.history, input.operation.clone());
            if payload.format == "txt" { export_document::emit(payload, &doc, None) } else { self.instance_owner
                .with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| owner.with_session(|session| export_document::retained_meshes(&doc, &cfg, session)))
                .and_then(|meshes| meshes.and_then(|meshes| export_document::emit(payload, &doc, meshes.as_deref()))) }
        };
        viewed.retire();
        let view_emit = emitted?;
        Ok(ArtifactCommandWorkStep::Complete(Emit { effects: view_emit.effects, ui_scope: view_emit.ui_dirty, ..Default::default() }))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct Generation3dViewDocumentIoJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewDocumentIoJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewDocumentIoJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_DOCUMENT_IO_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_document_io_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_DOCUMENT_IO_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer document-io command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewDocumentIoJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_DOCUMENT_IO_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🔒️ `HostOnly` — an export writes no store lane at all, which is precisely why a VIEWER may
    /// own it. `no_viewer_tool_publishes_on_the_artifact_lane` reads this row like every other.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "exportDocument", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

pub(super) struct Generation3dViewDocumentIoJobFactoryProofs;

impl Generation3dViewDocumentIoJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        artifact_schema: "generation.3d",
        factory: "Generation3dViewDocumentIoJobFactory",
        factory_type: Generation3dViewDocumentIoJobFactory,
        contract: generation3d_view_document_io_contract(),
        tools: ["exportDocument"]
    }
}
//#endregion 📤️DocumentIoRoute
