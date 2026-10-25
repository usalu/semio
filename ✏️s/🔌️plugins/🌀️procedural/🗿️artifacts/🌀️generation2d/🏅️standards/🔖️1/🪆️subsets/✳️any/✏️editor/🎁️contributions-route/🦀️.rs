//! 🎁️ Contributions route of the generation2d editor: the contributions contract, its work and job factory and the factory proofs. See [`super`].

use super::*;

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route — the exact twin of generation3d's
/// (`…/🧊️generation3d/…/✏️editor/🦀️.rs`), and for the same reason: the shell's whole
/// `flow.extension` closure is what makes this app's operators resolvable, and it is far too large
/// for the 8 KiB gesture quota the other 21 routes share.
pub const GENERATION2D_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];

const GENERATION2D_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "generation.2d.contributions-command.v1";

/// 📐️ The REAL wire ceiling of one contributions push: what the paged command ingress can deliver
/// into this guest at all, and nothing narrower — the same bound, for the same reason, as
/// generation3d's `GENERATION3D_CONTRIBUTIONS_RAW_BYTES`.
///
/// 🧊️ The shell sends the scoped pack WHOLE as page 0 of 1, pack-encoded through
/// `PluginRuntime.performInvocation`, and the shard streams it one 4 KiB page per turn — so the JSON
/// entry point's `PUBLIC_INVOCATION_STRING_BYTES` page run never applies to it. Declaring that
/// string bound (×escape factor + a 4 KiB envelope = 12 288 bytes) here refused the real 62 090-byte
/// push at the tool factory (`tool factory '…/setContributions' rejected 62090 raw bytes before
/// decoding; maximum is 12288`), so the plugin never received its operators (ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). The one bound that actually binds is
/// `COMMAND_MAXIMUM_BYTES`, the guest's assembled-command ceiling, and the contract reserves per
/// DECLARED extent, never per maximum, so naming it costs nothing.
pub const GENERATION2D_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

pub fn generation2d_contributions_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GENERATION2D_CONTRIBUTIONS_RAW_BYTES, GENERATION2D_RETAINED_DECODED_ITEMS, GENERATION2D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page against the app instance's retained session.
pub(super) struct Generation2dContributionsWork {
    pub(super) instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<EditorApp<Generation2dPlayApp>> for Generation2dContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &Generation2dCommand,
        snapshot: &Generation2dSnapshot,
        interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Generation2dPlayApp>>>,
    ) -> Option<usize> {
        generation2d_bounded_extent(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        command_work_step_demands::<EditorApp<Generation2dPlayApp>>(size_of::<Self>())
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Generation2dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Generation2dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("generation2d-contributions-work-repeated"));
        }
        self.consumed = true;
        let Generation2dCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("generation2d-contributions-route-rejected"));
        };
        let windows = generation2d_preview_windows(input.context.and_then(|context| context.view_state.as_ref()));
        let servable = semio_s_artifact_procedural_2d_preview_eval::may_rearm(&input.snapshot.host_snapshot);
        let emit = self.instance_owner.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
            let mut emit = Emit::default();
            let invalidated = {
                let (sessions, _) = owner.parts()?;
                set_contributions::install(payload, &mut [sessions.document, sessions.generation], input.operation.retained)?
            };
            if invalidated {
                owner.owe_attached_previews_carrying(&windows, servable, &mut emit, input.operation.retained)?;
            }
            Ok(emit)
        })?;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub struct Generation2dContributionsJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl Generation2dContributionsJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: GENERATION2D_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Generation2dContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Generation2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Generation2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GENERATION2D_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        generation2d_contributions_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: GENERATION2D_CONTRIBUTIONS_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Generation2dContributionsJobFactory {
    type Owner = EditorApp<Generation2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GENERATION2D_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

pub(super) struct Generation2dBoundedCommandJobFactoryProofs;

impl Generation2dBoundedCommandJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation2d@1/*#editor",
        artifact_schema: "generation.2d",
        factory: "Generation2dBoundedCommandJobFactory",
        factory_type: Generation2dBoundedCommandJobFactory,
        contract: generation2d_bounded_contract(),
        tools: [
            "nodeGraphViewport",
            "setShowMode",
            "setActiveExample",
            "generate",
            "canvasPointerDown",
            "canvasPointerMove",
            "canvasPointerUp",
            "canvasWheel",
            "addGeneration",
            "removeGeneration",
            "renameGeneration",
            "updateGenerationValues",
            "selectGeneration",
            "flowEvalTick",
            "flowEvalResolve",
            "nodeGraphEdit",
            "moveMediaNode",
            "addWidget",
            "removeWidget",
            "connectMediaPorts",
            "reorganize",
            "setEvalOutputs",
        ]
    }
}

pub(super) struct Generation2dContributionsJobFactoryProofs;

impl Generation2dContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Generation2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.procedural.generation2d@1/*#editor",
        artifact_schema: "generation.2d",
        factory: "Generation2dContributionsJobFactory",
        factory_type: Generation2dContributionsJobFactory,
        contract: generation2d_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute
