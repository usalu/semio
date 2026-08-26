use semio_framework::{ActionBus, ToolOperationSpec};
use semio_framework_job::{
    allocate_operation_id, validate_commit, CancelToken, CommitCandidate, CommitValidation, Generation, InteractiveJob, InteractiveJobCloseStep, JobPayloadCloseStep, JobPayloadStream, Operation, RetainedJobPayload, RevisionId, StepBudget, StepContext, StepOutcome,
    JOB_PAYLOAD_PAGE_BYTES,
};
use semio_framework_plugin::{
    ArtifactApp, ArtifactReservedJob, ArtifactReservedToolJob, ArtifactView, ConfigView, DraftView, Emit, Fault, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, PluginCloseStep,
    UiAssemblyResult,
};
use semio_framework_plugin::plugin_app_close_prelude::{
    FrameworkClearSelectionJobFactory, FrameworkCopyJobFactory, FrameworkCutJobFactory, FrameworkInteractionHoverJobFactory, FrameworkInteractionSelectJobFactory, FrameworkNoteShellCommandJobFactory,
    FrameworkPasteJobFactory, FrameworkRecordTutorialJobFactory, FrameworkSelectAllJobFactory, FrameworkSetHistoryCommandFilterJobFactory, FrameworkSetInteractionGranularityJobFactory, FrameworkSetSelectionModeJobFactory, InteractionView,
};
use semio_framework_os_kernel::EngineHandles;
use std::any::type_name;

const CONTROLLER: &str = "harness.shared-actions";

#[derive(Default)]
struct HarnessApp;

impl ArtifactApp for HarnessApp {
    const APP_ID: &'static str = CONTROLLER;
    const DOCUMENT_SCHEMA: &'static str = "harness.shared-actions/v1";
    type Snapshot = NoConfig;
    type Mutation = NoConfigMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = NoConfigMutation;

    async fn initial_snapshot() -> Self::Snapshot {
        NoConfig::default()
    }

    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
        Ok(Emit::default())
    }

    async fn render(_body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        unreachable!("the shared-action harness never renders")
    }
}

struct HarnessJob {
    raw: Vec<u8>,
    stage: u8,
    closing: bool,
}

impl HarnessJob {
    fn new(bytes: usize) -> Self {
        Self { raw: vec![0x5A; bytes], stage: 0, closing: false }
    }
}

impl InteractiveJob for HarnessJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        match self.stage {
            0 => {
                self.stage = 1;
                StepOutcome::PreviewReady(RetainedJobPayload::empty(JobPayloadStream::Preview))
            }
            1 => {
                self.stage = 2;
                StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), applied_progress: 1 })
            }
            _ => StepOutcome::Complete(CommitCandidate {
                state: RetainedJobPayload::empty(JobPayloadStream::CommitState),
                output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
            }),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closing = true;
        if !self.raw.is_empty() {
            if maximum_items == 0 || maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            let released_bytes = self.raw.len().min(maximum_bytes);
            self.raw.truncate(self.raw.len() - released_bytes);
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.raw.is_empty()
    }
}

impl ArtifactReservedJob for HarnessJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(match InteractiveJob::close_step(self, maximum_items, maximum_bytes) {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
            InteractiveJobCloseStep::Blocked => PluginCloseStep::Blocked { reason: "harness close blocked" },
            InteractiveJobCloseStep::Complete => PluginCloseStep::Complete,
        })
    }

    fn terminal_is_empty(&self) -> bool {
        InteractiveJob::terminal_is_empty(self)
    }
}

#[derive(Clone, Copy)]
struct Route {
    id: &'static str,
    schema: &'static str,
    factory: &'static str,
    maximum: usize,
}

fn routes() -> [Route; 12] {
    [
        Route { id: "copy", schema: "framework.reserved.copy.v1", factory: type_name::<FrameworkCopyJobFactory<HarnessApp>>(), maximum: 8_192 },
        Route { id: "cut", schema: "framework.reserved.cut.v1", factory: type_name::<FrameworkCutJobFactory<HarnessApp>>(), maximum: 8_192 },
        Route { id: "paste", schema: "framework.reserved.paste.v1", factory: type_name::<FrameworkPasteJobFactory<HarnessApp>>(), maximum: 1_048_576 },
        Route { id: "noteShellCommand", schema: "framework.reserved.noteShellCommand.v1", factory: type_name::<FrameworkNoteShellCommandJobFactory<HarnessApp>>(), maximum: 65_536 },
        Route { id: "setHistoryCommandFilter", schema: "framework.reserved.setHistoryCommandFilter.v1", factory: type_name::<FrameworkSetHistoryCommandFilterJobFactory<HarnessApp>>(), maximum: 4_096 },
        Route { id: "recordTutorial", schema: "framework.reserved.recordTutorial.v1", factory: type_name::<FrameworkRecordTutorialJobFactory<HarnessApp>>(), maximum: 4_096 },
        Route { id: "clearSelection", schema: "framework.reserved.clearSelection.v1", factory: type_name::<FrameworkClearSelectionJobFactory<HarnessApp>>(), maximum: 4_096 },
        Route { id: "interactionHover", schema: "framework.reserved.interactionHover.v1", factory: type_name::<FrameworkInteractionHoverJobFactory<HarnessApp>>(), maximum: 65_536 },
        Route { id: "interactionSelect", schema: "framework.reserved.interactionSelect.v1", factory: type_name::<FrameworkInteractionSelectJobFactory<HarnessApp>>(), maximum: 65_536 },
        Route { id: "selectAll", schema: "framework.reserved.selectAll.v1", factory: type_name::<FrameworkSelectAllJobFactory<HarnessApp>>(), maximum: 4_096 },
        Route { id: "setInteractionGranularity", schema: "framework.reserved.setInteractionGranularity.v1", factory: type_name::<FrameworkSetInteractionGranularityJobFactory<HarnessApp>>(), maximum: 16_384 },
        Route { id: "setSelectionMode", schema: "framework.reserved.setSelectionMode.v1", factory: type_name::<FrameworkSetSelectionModeJobFactory<HarnessApp>>(), maximum: 16_384 },
    ]
}

fn operation(generation: u64) -> Operation {
    Operation::new(allocate_operation_id(), RevisionId(41), Generation(generation), 73)
}

fn context<'a>(operation: Operation, cancel: CancelToken, preview: &'a mut u64) -> StepContext<'a> {
    StepContext::new(operation.operation, operation.generation, StepBudget::new(4_096, u64::MAX), cancel, || 0, preview)
}

fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        assert!(matches!(payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES), JobPayloadCloseStep::Pending { .. }));
    }
}

fn close_job(job: &mut impl InteractiveJob) -> Vec<(usize, usize)> {
    job.begin_close();
    assert_eq!(job.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    let mut releases = Vec::new();
    loop {
        match job.close_step(1, 4_096) {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => releases.push((released_items, released_bytes)),
            InteractiveJobCloseStep::Complete => break,
            InteractiveJobCloseStep::Blocked => panic!("shared route close blocked"),
        }
    }
    assert!(job.terminal_is_empty());
    assert_eq!(job.close_step(1, 4_096), InteractiveJobCloseStep::Complete);
    releases
}

fn main() {
    let bus = ActionBus::new();
    semio_framework_plugin::testkit::register_framework_reserved_action_factories::<HarnessApp>(&bus, CONTROLLER).expect("production shared factory registration");
    let registered = bus.keys();
    let cohort = routes();
    assert!(cohort.iter().all(|route| registered.iter().any(|key| key.controller_id == CONTROLLER && key.tool_id == route.id)));

    for route in cohort {
        let exact = vec![0xA5; route.maximum];
        let admission = bus.admit_exact_wire(CONTROLLER, route.id, route.schema, &exact).expect("exact maximum admission");
        assert_eq!(admission.key.tool_id, route.id);
        assert_eq!(admission.factory_type_name, route.factory);
        assert_eq!(admission.contract.max_raw_wire_bytes, route.maximum);
        assert!(bus.admit_exact_wire(CONTROLLER, route.id, route.schema, &vec![0; route.maximum + 1]).is_err());

        let (_, mut retained) = bus.begin_exact_wire(CONTROLLER, route.id, route.schema, route.maximum).expect("pre-admit exact retained owner");
        for page in exact.chunks(semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES) {
            retained.admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(page).expect("fixed exact page")).unwrap_or_else(|_| panic!("exact page owner rejected"));
        }
        let returned = retained
            .admit_page(semio_framework::action_bus::ToolWirePage::try_copy_from(&[0xEE]).expect("plus-one page owner"))
            .expect_err("maximum plus one must return its exact owner")
            .1;
        assert_eq!(returned.as_slice(), &[0xEE]);
        while !retained.terminal_is_empty() {
            let _ = retained.close_step(1, semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES);
        }

        let op = operation(1);
        let shared = ArtifactReservedToolJob::new(HarnessJob::new(8_193));
        let mut dispatch = bus.dispatch(ToolOperationSpec::new(CONTROLLER, route.id, route.schema, shared.clone(), op)).expect("exact typed factory dispatch");
        assert!(bus.dispatch(ToolOperationSpec::new(CONTROLLER, route.id, route.schema, shared, operation(2))).is_err(), "ABA replacement must not rebind one live payload owner");
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let mut preview = 0;
        assert!(matches!(dispatch.job.step(&mut context(op, cancel, &mut preview)), StepOutcome::Cancelled));
        let releases = close_job(&mut dispatch.job);
        assert_eq!(releases, vec![(1, 4_096), (1, 4_096), (1, 1)]);

        let op = operation(3);
        let mut dispatch = bus.dispatch(ToolOperationSpec::new(CONTROLLER, route.id, route.schema, ArtifactReservedToolJob::new(HarnessJob::new(8_193)), op)).expect("cancel-after-transfer dispatch");
        let cancel = CancelToken::root_now();
        let mut preview = 0;
        match dispatch.job.step(&mut context(op, cancel.clone(), &mut preview)) {
            StepOutcome::PreviewReady(payload) => close_payload(payload),
            _ => panic!("first shared route step must expose preview"),
        }
        cancel.cancel_now();
        assert!(matches!(dispatch.job.step(&mut context(op, cancel, &mut preview)), StepOutcome::Cancelled));
        close_job(&mut dispatch.job);

        let op = operation(4);
        let mut dispatch = bus.dispatch(ToolOperationSpec::new(CONTROLLER, route.id, route.schema, ArtifactReservedToolJob::new(HarnessJob::new(8_193)), op)).expect("interrupted-resume dispatch");
        let cancel = CancelToken::root_now();
        let mut preview = 0;
        match dispatch.job.step(&mut context(op, cancel.clone(), &mut preview)) {
            StepOutcome::PreviewReady(payload) => close_payload(payload),
            _ => panic!("resumable shared route first step must expose preview"),
        }
        match dispatch.job.step(&mut context(op, cancel.clone(), &mut preview)) {
            StepOutcome::CheckpointReady(checkpoint) => {
                assert_eq!(checkpoint.applied_progress, 1);
                close_payload(checkpoint.state);
            }
            _ => panic!("resumable shared route second step must expose progress checkpoint"),
        }
        match dispatch.job.step(&mut context(op, cancel, &mut preview)) {
            StepOutcome::Complete(candidate) => {
                close_payload(candidate.state);
                close_payload(candidate.output);
            }
            _ => panic!("resumable shared route third step must complete"),
        }
        close_job(&mut dispatch.job);

        let op = operation(5);
        assert_eq!(validate_commit(&op, RevisionId(41), Generation(5)), CommitValidation::Accepted);
        assert_ne!(validate_commit(&op, RevisionId(42), Generation(5)), CommitValidation::Accepted);
        assert_ne!(validate_commit(&op, RevisionId(41), Generation(6)), CommitValidation::Accepted);
        println!("[DEBUG] route={} factory={} exact={} plus_one=rejected-owner-returned cancel=before+after aba=rejected resume=preview+checkpoint(1)+complete close=interrupted+repeated freshness=exact", route.id, route.factory, route.maximum);
    }
    assert_eq!(bus.dispatch_count(), 36);
    println!("[DEBUG] shared-action-runtime-harness routes=12 registered=exact factory=exact joined=exact result=pass");
}
