//! 🎨️ Contributions and example routes: the contributions contract, work and job factory and the example contract, job factory and proofs. See [`super`].

use super::*;

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route on the READ-ONLY surface. The viewer's preview window
/// evaluates its geometry through the SAME addressed `flowEvalTick` chain the sibling surface runs
/// (`🧵️preview-eval`), whose `evaluate`/`tessellate` hops are answered by the contributed brep and
/// math extensions — so it needs exactly the same contributed operators. The registry it installs
/// into is process-wide, but a session that only ever opens this app is never handed the sibling
/// surface's own route, which is why this one exists
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) const GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];

const GENERATION3D_VIEW_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.3d.view-contributions-command.v1";

/// 📐️ The REAL wire ceiling of one contributions page: the framework's own public-invocation string
/// bound — which no tool contract can widen, because `validate_public_json_envelope` runs before the
/// addressed tool's contract — at its worst-case escaped width, plus the addressed envelope.
/// 📐️ The viewer's own contributions ceiling — the same one the editor route declares, and for the
/// same reason: the pack crosses whole over the paged command ingress, so the bound is what that
/// ingress can assemble, not the JSON entry point's body cap.
pub(super) const GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

fn generation3d_view_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page. Publishes nothing at all — not even the config lane every
/// other viewer tool writes.
pub(super) struct Generation3dViewContributionsWork {
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<ViewerApp<Generation3dViewer>> for Generation3dViewContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
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
            return Err(Fault::from("generation3d-view-contributions-work-repeated"));
        }
        self.consumed = true;
        let Generation3dViewCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("generation3d-view-contributions-route-rejected"));
        };
        // 🪟️ An install that moved the registry owes the windows the SHELL says are attached an
        // evaluation — the same trusted `ViewModel` roster `pending_effects` starts the run off.
        let windows = generation3d_view_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let mut emit = Emit::default();
        self.instance_owner.with_mut::<Generation3dViewInstanceOperationOwner, _>(|owner| {
            if owner.with_session(|session| set_contributions::install(payload, session, input.operation.retained))?? {
                let viewed = Generation3dViewedDocument::resolve(input.snapshot, input.config);
                let servable = preview_eval::may_rearm(&viewed.snapshot().host_snapshot);
                viewed.retire();
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
            }
            Ok(())
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct Generation3dViewContributionsJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewContributionsJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_contributions_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewContributionsJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

pub(super) struct Generation3dViewBoundedCommandJobFactoryProofs;

impl Generation3dViewBoundedCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        artifact_schema: "generation.3d",
        factory: "Generation3dViewBoundedCommandJobFactory",
        factory_type: Generation3dViewBoundedCommandJobFactory,
        contract: generation3d_view_bounded_contract(),
        tools: ["setShowMode", "setLodMode", "setCamera", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"]
    }
}

pub(super) struct Generation3dViewContributionsJobFactoryProofs;

impl Generation3dViewContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        artifact_schema: "generation.3d",
        factory: "Generation3dViewContributionsJobFactory",
        factory_type: Generation3dViewContributionsJobFactory,
        contract: generation3d_view_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute

//#region 🎨️ExampleRoute
fn generation3d_view_example_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_VIEW_EXAMPLE_RAW_BYTES, 32, 32, 16_384, 7_500)
}

/// 🎨️ The navbar example picker's own factory. It runs the very same
/// [`Generation3dViewCommandWork`] every window gesture does — so the switch re-arms every attached
/// preview's chain from its own emit, never from a host `refresh-ui` — but it is registered
/// separately because it is an APP-scoped verb, not one this viewer's single window dispatches.
pub(super) struct Generation3dViewExampleJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dViewExampleJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_VIEW_EXAMPLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Generation3dViewExampleJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<Generation3dViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<Generation3dViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_VIEW_EXAMPLE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_view_example_contract()
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
        if input.declared_bytes() > GENERATION3D_VIEW_EXAMPLE_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d viewer example command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Generation3dViewExampleJobFactory {
    type Owner = ViewerApp<Generation3dViewer>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_VIEW_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🔒️ Still read-only, and this is the row that proves it: loading an example on a viewer is a
    /// CONFIG edit plus an ephemeral presence broadcast. The sibling surface's own
    /// `setActiveExample` names `ArtifactToolPublicationLane::Artifact` here; this one cannot, and
    /// `no_viewer_tool_publishes_on_the_artifact_lane` holds it to that.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] =
        &[ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Presence, ArtifactToolPublicationLane::Transient] }];
}

pub(super) struct Generation3dViewExampleJobFactoryProofs;

impl Generation3dViewExampleJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<Generation3dViewer>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#viewer",
        artifact_schema: "generation.3d",
        factory: "Generation3dViewExampleJobFactory",
        factory_type: Generation3dViewExampleJobFactory,
        contract: generation3d_view_example_contract(),
        tools: ["setActiveExample"]
    }
}
//#endregion 🎨️ExampleRoute
