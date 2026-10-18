//! 🤝️ Bounded proofs and contributions route: the bounded command job factory proofs and the contributions contract, work and job factory. See [`super`].

use super::*;

//#region 🔖️Generation3dPlayApp
//#region 🧵️RetainedCommands
pub(super) struct Generation3dBoundedCommandJobFactoryProofs;

impl Generation3dBoundedCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dBoundedCommandJobFactory",
        factory_type: Generation3dBoundedCommandJobFactory,
        contract: generation3d_bounded_contract(),
        tools: [
            "setActiveExample",
            "nodeGraphEdit",
            "setWidgetInput",
            "deleteSelection",
            "removeWidget",
            "addWidget",
            "patchFlowWidgets",
            "reorganize",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
            "addGeneration",
            "removeGeneration",
            "renameGeneration",
            "updateGenerationValues",
            "nodeGraphViewport",
            "setLodMode",
            "setShowMode",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setCamera",
            "selectGeneration",
            "cycleShowMode",
            "cycleLodMode",
            "selectNextNode",
            "selectPreviousNode",
            "selectUpstreamNode",
            "selectDownstreamNode",
            "activateSelection",
            "editMeshSelection",
            "knifeMeshSelection",
        ]
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route: its OWN tool, its OWN factory and its OWN execution
/// contract, because the payload it carries is nothing like a gesture. `setContributions` delivers
/// the shell's whole `flow.extension` closure — 293 642 characters for this app's nine staged
/// extension plugins — into the plugin's process-wide flow extension registry, which is what makes
/// `brep`/`math` resolvable at all; without it the served app has NO operators and every preview
/// faults `flow.extension-not-contributed` (`📓️extension-addressing-2026-09-10.md` §6).
pub const GENERATION3D_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];

const GENERATION3D_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.3d.contributions-command.v1";

/// 📐️ The REAL wire ceiling of one contributions push: what the paged command ingress can deliver
/// into this guest at all, and nothing narrower.
///
/// 🧊️ The pack crosses WHOLE and pack-encoded — `PluginRuntime.performInvocation` encodes the
/// invocation and the shard streams it one 4 KiB page per turn — so the JSON entry point's
/// `PUBLIC_INVOCATION_STRING_BYTES` page run never applies to it and
/// `PUBLIC_INVOCATION_BODY_BYTES` is not this route's bound. Declaring that JSON body cap here
/// refused a 273 136-byte pack-encoded contributions command at the tool factory
/// (`tool factory '…/setContributions' rejected 273136 raw bytes before decoding; maximum is
/// 262144`) even after the transport itself had delivered all 67 pages — ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14. The one bound that actually binds is
/// `COMMAND_MAXIMUM_BYTES`, itself the guest linear-memory budget's assembled-answer ceiling, and
/// the contract reserves per DECLARED extent, never per maximum, so naming it costs nothing.
pub const GENERATION3D_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

/// 🧾️ The contributions route's ONE execution contract — deliberately NOT
/// [`generation3d_bounded_contract`]: raising the 8 KiB gesture quota every interactive command
/// lives under, just to let a boot-time host push through, would widen 29 unrelated routes.
pub fn generation3d_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION3D_CONTRIBUTIONS_RAW_BYTES, GENERATION3D_RETAINED_DECODED_ITEMS, GENERATION3D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page. Runs against the app instance's RETAINED session for
/// symmetry with every other retained route, but touches nothing in it: the registry the page
/// assembles into is process-wide, shared by every app of this component, which is exactly why one
/// app owning this route is enough to give the whole plugin its operators.
pub(super) struct Generation3dContributionsWork {
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<EditorApp<Generation3dPlayApp>> for Generation3dContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &Generation3dCommand,
        snapshot: &Generation3dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation3dPlayApp>>>,
    ) -> Option<usize> {
        generation3d_bounded_extent(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation3dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("generation3d-contributions-work-repeated"));
        }
        self.consumed = true;
        let Generation3dCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("generation3d-contributions-route-rejected"));
        };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: None };
        // 🪟️ An install that moved the registry owes the windows the SHELL says are attached an
        // evaluation — the same trusted `ViewModel` roster `pending_effects` starts the run off, never a
        // window this route invents (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let windows = generation3d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let _ = (&doc, &cfg);
        let mut emit = Emit::default();
        self.instance_owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
            if owner.with_session(|session| set_contributions::install(payload, session, input.operation.retained))?? {
                let servable = flow_eval_tick::may_rearm(&input.snapshot.host_snapshot);
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
            }
            Ok(())
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub struct Generation3dContributionsJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation3dContributionsJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION3D_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation3dContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION3D_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation3d_contributions_contract()
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
        if input.declared_bytes() > GENERATION3D_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Generation3d contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation3dContributionsJobFactory {
    type Owner = EditorApp<Generation3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION3D_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_3D_SCHEMA;
    /// 🧩️ `HostOnly`: the flow extension registry is process-wide runtime state, never a document,
    /// config, transient or window-transient lane — the route emits an empty `Emit`.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

pub(super) struct Generation3dContributionsJobFactoryProofs;

impl Generation3dContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation3d@1/*#editor",
        artifact_schema: "generation.3d",
        factory: "Generation3dContributionsJobFactory",
        factory_type: Generation3dContributionsJobFactory,
        contract: generation3d_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute
//#endregion 🔖️Generation3dPlayApp
