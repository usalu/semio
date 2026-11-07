//! 🧵️ Retained command route of the WFC bitmap editor: the bounded publication contracts, the command work and the job factories. See [`super`].

use super::*;
use std::mem::size_of;

//#region 🧵️RetainedCommands
/// 🧬️ The payload schema id every bitmap retained command job is admitted under.
const BITMAP_RETAINED_COMMAND_SCHEMA: &str = "s.wfc.bitmap/v1.tool-command.v1";

/// 🎒️ Wire ceiling for one bitmap tool dispatch. The largest payload is a `set-input-pixels` region
/// covering the schema maximum 512 × 512 pixels, plus a bounded command envelope.
pub(super) const BITMAP_RETAINED_RAW_BYTES: usize = pin_solution::BITMAP_MAXIMUM_OUTPUT_CELLS.div_ceil(3) * 4 + 16_384;

/// 📬️ The batch envelope allows 64 bytes per forward and inverse pin plus its frame.
pub(super) const BITMAP_MUTATION_OUTPUT_MAXIMUM_BYTES: usize = pin_solution::BITMAP_MAXIMUM_OUTPUT_CELLS * 128 + 16_384;

/// 🧮️ One semantic solution replacement and its exact restoration occupy two staged rows.
pub(super) const BITMAP_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🚦️ Per-tool publication lanes, read straight off each command's own emit.
const BITMAP_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    artifact_route("change-seed"),
    artifact_route("resize-input"),
    artifact_route("set-input-pixels"),
    artifact_route("add-palette-color"),
    artifact_route("change-palette-color"),
    artifact_route("remove-palette-color"),
    artifact_route("resize-output"),
    artifact_route("change-model"),
    artifact_route("pin-pixel"),
    artifact_route("unpin-pixel"),
    window_config_route("set-active-color"),
    brush_route("paint-stroke"),
    ArtifactToolPublicationContract { tool_id: "solve", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "commit-fill-solve", lanes: &[ArtifactToolPublicationLane::Transient] },
    host_only_route("setActiveExample"),
    artifact_route("pin-solution"),
    artifact_route("pin-preview"),
    brush_route("canvasPointerDown"),
    brush_route("canvasPointerMove"),
    brush_route("canvasPointerUp"),
    host_only_route("canvasDoubleClick"),
    host_only_route("setCamera"),
];

const fn artifact_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact] }
}

const fn window_config_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::WindowConfig] }
}

const fn brush_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] }
}

const fn host_only_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::HostOnly] }
}

fn bitmap_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BITMAP_RETAINED_RAW_BYTES, BITMAP_RETAINED_WORK_ITEMS, pin_solution::BITMAP_PIN_WORK_UNITS_PER_TURN as u64, BITMAP_MUTATION_OUTPUT_MAXIMUM_BYTES, 7_500)
}

/// 🧵️ The one retained work every bitmap verb rides. `Solve` is the only arm that answers
/// `CompleteWithEphemeral`: it runs the artifact's own headless inference adapter and hands the
/// collapse to the transient lane the output window renders from.
pub(super) struct BitmapCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) consumed: bool,
    pin: Option<pin_solution::BitmapPinCursor>,
    remnant: Option<semio_framework_value::retirement::controlled::ControlledRetirement<pin_solution::BitmapPinCursor>>,
    closing: bool,
}

impl BitmapCommandWork {
    pub(super) fn new(tool_id: &'static str) -> Self { Self { tool_id, consumed: false, pin: None, remnant: None, closing: false } }

    fn pin_step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BitmapEditor>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BitmapEditor>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() || cx.fuel_exhausted() || cx.deadline_exceeded() { return Ok(ArtifactCommandWorkStep::Pending { stage: "bitmap-pin-yield", preview: b"" }); }
        if self.consumed { return Err(Fault::from("wfc-bitmap-retained-work-repeated").into()); }
        if bitmap_command_id(input.command) != self.tool_id { return Err(Fault::from("wfc-bitmap-retained-route-mismatch").into()); }
        let pixels = match input.command {
            BitmapEditorCommand::PinPreview => {
                let context = input.context.ok_or_else(|| pin_solution::preview_fault("unavailable"))?;
                if self.pin.is_none() {
                    pin_solution::require_preview_window(context.view_state.as_ref(), output::WFC_BITMAP_WINDOW_OUTPUT)?;
                    pin_solution::preview_pixels(input.snapshot, context.transient.as_ref(), &context.canonical_base_revision)?
                } else { context.transient.output_pixels.as_deref().expect("captured preview") }
            }
            BitmapEditorCommand::PinSolution { pixels, contradiction } => {
                if *contradiction { return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.solution.contradiction"), "wfc.bitmap.solution.contradiction").into()); }
                pixels.as_str()
            }
            _ => return Err(Fault::from("wfc-bitmap-retained-route-mismatch").into()),
        };
        if self.pin.is_none() {
            let cells = pin_solution::pin_cells(input.snapshot).ok_or_else(|| Fault::from("wfc-bitmap-output-capacity"))?;
            if pixels.len() != cells.div_ceil(3) * 4 { return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.solution.pixels-invalid"), "wfc.bitmap.solution.pixels-invalid").into()); }
            self.pin = Some(pin_solution::BitmapPinCursor::new(cells));
        }
        if cx.is_cancelled() { return Ok(ArtifactCommandWorkStep::Pending { stage: "bitmap-pin-cancel", preview: b"" }); }
        let mut grant = cx.retained_grant();
        if self.pin.as_ref().is_some_and(pin_solution::BitmapPinCursor::ready_to_publish) && (cx.fuel_remaining() < 2 || grant.maximum_items < 2 || grant.maximum_capacity_bytes < size_of::<BitmapMutation>() || grant.maximum_copy_bytes < size_of::<BitmapMutation>()) { return Ok(ArtifactCommandWorkStep::Pending { stage: "bitmap-pin-publication", preview: b"" }); }
        grant.maximum_items = grant.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
        let (mutations, progress) = self.pin.as_mut().expect("pin workspace").advance(input.snapshot, pixels, grant)?;
        cx.consume_retained(progress)?;
        cx.consume_fuel(progress.copied_items.max(1) as u64);
        match mutations {
            Some(pins) => {
                self.consumed = true;
                if pins.is_empty() { return Ok(ArtifactCommandWorkStep::Complete(Emit::default())); }
                let mut mutations = Vec::with_capacity(1);
                mutations.push(crate::mutations::pin_solution(pins));
                cx.consume_retained(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<BitmapMutation>(), retained_capacity_bytes: mutations.capacity() * size_of::<BitmapMutation>(), ..Default::default() })?;
                cx.consume_fuel(1);
                Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(mutations)))
            }
            None => Ok(ArtifactCommandWorkStep::Progress { stage: "bitmap-pin-materialize", preview: b"{\"en\":\"Preparing output pins\",\"de\":\"Ausgabeanheftungen vorbereiten\"}" }),
        }
    }
}

impl ArtifactCommandWork<EditorApp<BitmapEditor>> for BitmapCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &BitmapEditorCommand,
        snapshot: &BitmapSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BitmapEditor>>>,
    ) -> Option<usize> {
        match command {
            BitmapEditorCommand::PinSolution { .. } | BitmapEditorCommand::PinPreview => pin_solution::pin_work_items(snapshot),
            _ => BITMAP_TOOL_IDS.contains(&bitmap_command_id(command)).then_some(1),
        }
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<BitmapEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let mut demand = if matches!(input.command, BitmapEditorCommand::PinPreview | BitmapEditorCommand::PinSolution { .. }) { self.pin.as_ref().map_or(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }, pin_solution::BitmapPinCursor::demands) } else { semio_framework_value::RetirementDemand { depth: 1, ..Default::default() } };
        if self.pin.as_ref().is_some_and(pin_solution::BitmapPinCursor::ready_to_publish) { demand.capacity_bytes += size_of::<BitmapMutation>(); demand.copy_bytes += size_of::<BitmapMutation>(); }
        Ok(demand)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BitmapEditor>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BitmapEditor>>, semio_framework_job::OriginalTurnFailure> {
        if matches!(input.command, BitmapEditorCommand::PinPreview | BitmapEditorCommand::PinSolution { .. }) { return self.pin_step(input, cx); }
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("wfc-bitmap-retained-work-repeated"));
        }
        self.consumed = true;
        if bitmap_command_id(input.command) != self.tool_id {
            return Err(Fault::from("wfc-bitmap-retained-route-mismatch"));
        }
        if let BitmapEditorCommand::CommitFillSolve { source_revision, .. } = input.command {
            if !pin_solution::completion_matches(source_revision, Some(&input.operation.canonical_base_revision)) { return Ok(ArtifactCommandWorkStep::Complete(Emit::default())); }
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let view_state = input.context.and_then(|context| context.view_state.as_deref());
        let pointer = input.command.brush_pointer();
        if pointer.is_some() || matches!(input.command, BitmapEditorCommand::PaintStroke { .. }) {
            let detached = semio_plugin::app::GestureSlot::detached();
            let slot = input.context.map_or(&detached, |context| context.gesture());
            let stroke = match &pointer {
                Some(pointer) => BitmapEditor::pointer_stroke(pointer, doc.snapshot, slot.open().is_some()),
                None => Some((input.command.clone(), false)),
            };
            let Some((stroke, interrupt)) = stroke else { return Ok(ArtifactCommandWorkStep::Complete(Emit::default())) };
            return Ok(ArtifactCommandWorkStep::Complete(BitmapEditor::paint_stroke(&stroke, &doc, &cfg, slot, interrupt)?));
        }
        let emit = BitmapEditor::dispatch(input.command, &doc, &cfg, view_state)?;
        match input.command {
            BitmapEditorCommand::Solve => Ok(ArtifactCommandWorkStep::Complete(Emit {
                effects: vec![fill_tool::start_fill_effect()],
                ui_scope: semio_framework::kernel::UiDirtyScope::Full,
                ..Default::default()
            })),
            BitmapEditorCommand::CommitFillSolve { pixels, contradiction, width, height, source_revision } => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: EphemeralEmit {
                    transient: vec![BitmapTransientMutation::SetSolve(SetSolve {
                        output_pixels: if *contradiction || pixels.is_empty() { None } else { Some(pixels.clone()) },
                        contradiction: *contradiction,
                        output_width: *width,
                        output_height: *height,
                        source_revision: source_revision.clone(),
                    })],
                    ..Default::default()
                },
            }),
            _ => Ok(ArtifactCommandWorkStep::Complete(emit)),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        semio_framework_job::close_original_granted_step(cx, |grant| {
            let mut work = 0;
            let result = (|| {
            use semio_framework_job::InteractiveJobCloseStep;
            use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
            if !self.closing { return Ok(InteractiveJobCloseStep::Blocked); }
            if let Some(owner) = self.remnant.as_mut() {
                let step = owner.step(grant);
                work = owner.last_turn_work();
                let step = step?;
                if owner.terminal_is_empty() { self.remnant = None; }
                return Ok(match step { RetainedCloneStep::Complete(progress) => InteractiveJobCloseStep::Complete { progress }, RetainedCloneStep::Progress(progress) => InteractiveJobCloseStep::Pending { progress } });
            }
            let Some(_) = self.pin.as_ref() else { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); };
            let bytes = size_of::<pin_solution::BitmapPinCursor>();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < bytes { return Ok(InteractiveJobCloseStep::Pending { progress: Default::default() }); }
            let cursor = self.pin.take().expect("pin workspace");
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(cursor) {
                Ok(owner) => { self.remnant = Some(owner); work = 1; Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } }) }
                Err((error, cursor)) => { self.pin = Some(cursor); Err(error) }
            }
            })();
            semio_framework_value::RetainedWorkOutcome { result, work }
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.remnant.as_ref().map_or(Ok(if self.pin.is_some() { size_of::<pin_solution::BitmapPinCursor>() } else { 0 }), |owner| owner.next_copy_byte_demand()) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if let Some(owner) = self.remnant.as_ref() { return owner.next_capacity_byte_demand(maximum_copy_bytes); }
        Ok(0)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.remnant.as_ref().map_or(Ok(0), |owner| owner.next_release_byte_demand()) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.remnant.as_ref().map_or(Ok(usize::from(self.pin.is_some())), |owner| owner.next_depth_demand()) }
    fn terminal_frame_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
    fn terminal_is_empty(&self) -> bool { self.pin.is_none() && self.remnant.is_none() }
}

/// 🏭️ The app-owned bounded tool-job factory. `qualified_tool_proof` refuses every typed command
/// that resolves to anything else, so this registration is what makes the whole roster reachable.
pub struct BitmapCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BitmapCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: BITMAP_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }

    pub(super) fn register(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<BitmapEditor>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Self::new(&controller))
    }
}

impl semio_framework::ToolJobFactory for BitmapCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BitmapEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BitmapEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BITMAP_RETAINED_COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        bitmap_retained_contract()
    }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: BITMAP_RETAINED_RAW_BYTES, checkpoint_resume: false);
}

impl semio_plugin::ArtifactOwnedToolJobFactory for BitmapCommandJobFactory {
    type Owner = EditorApp<BitmapEditor>;
    const TOOL_IDS: &'static [&'static str] = BITMAP_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_BITMAP_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BITMAP_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

#[cfg(test)]
mod pin_retirement_tests {
    use super::*;
    use semio_framework_value_testing_allocator::observe_retirement_allocations;

    /// 🧾️ Actual cancelled pin jobs account for every heap event and release through the original caller.
    #[test]
    fn cancelled_pin_work_closes_actual_allocations_through_its_original_caller() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🎮️commands/📌️pin-solution/🧪️tests/🧫️preview/🔣️.json")).unwrap();
        let mut grant = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance;
        grant.maximum_release_bytes = grant.maximum_capacity_bytes;
        for case in fixture["cursorCases"].as_array().unwrap() {
            let mut snapshot = BitmapSnapshot::reference_scene();
            snapshot.output.width = case["width"].as_u64().unwrap() as u32;
            snapshot.output.height = case["height"].as_u64().unwrap() as u32;
            snapshot.pinned.clear();
            let cells = pin_solution::pin_cells(&snapshot).unwrap();
            let pixels = crate::standards::v1::subsets::any::io::text::snapshot::encode_base64(&vec![0; cells]);
            for stop in 0..=2 {
                let mut work = BitmapCommandWork::new("pin-preview");
                work.pin = Some(pin_solution::BitmapPinCursor::new(cells));
                let mut caller = semio_plugin::artifact_app_laws::OriginalWorkFixture::new();
                let mut allocated = 0;
                for _ in 0..stop {
                    caller.work_turn(grant, pin_solution::BITMAP_PIN_WORK_UNITS_PER_TURN as u64, |cx| {
                        let (turn, heap) = observe_retirement_allocations(|| work.pin.as_mut().unwrap().advance(&snapshot, &pixels, cx.retained_grant()));
                        let (output, progress) = turn?;
                        assert!(output.is_none());
                        assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes));
                        assert!(progress.fits(grant));
                        allocated += heap.0;
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(progress.copied_items.max(1) as u64);
                        Ok(())
                    }).unwrap();
                }
                assert_eq!(allocated, if stop == 0 { 0 } else { cells * size_of::<crate::schema::snapshot::BitmapPinnedPixel>() });
                caller.cancel().cancel();
                work.begin_close();
                let mut released = 0;
                let mut close_turns = 0;
                for _ in 0..100000 {
                    if work.terminal_is_empty() { break; }
                    caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
                        let (turn, heap) = observe_retirement_allocations(|| work.close_step(cx));
                        let step = turn?;
                        let progress = step.progress();
                        assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes));
                        assert_eq!(heap.0, 0, "inline retirement handoff must not invent a backing allocation");
                        released += heap.1;
                        close_turns += 1;
                        Ok(step)
                    })).unwrap();
                }
                assert!(work.terminal_is_empty());
                assert!(close_turns > 1);
                assert_eq!(released, allocated, "{} cancellation frontier {stop}", case["id"]);
                assert!(snapshot.pinned.is_empty());
                caller.finish();
            }
        }
    }
    /// 📨️ The actual completion work emits no rows and preserves the complete newer cache for a stale source.
    #[test]
    fn stale_completion_cannot_overwrite_a_fresh_bitmap_cache() {
        use semio_plugin::retained_command::ArtifactCommandSnapshotRead;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
        let snapshot = BitmapSnapshot::reference_scene();
        let history = semio_plugin::HistoryView::empty();
        let config = NoConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = Default::default();
        let grant = semio_plugin::artifact_app_laws::fixture_retained_turn().grant;
        let cells = (snapshot.output.width * snapshot.output.height) as usize;
        let encode = crate::standards::v1::subsets::any::io::text::snapshot::encode_base64;
        for case in fixture["cases"].as_array().unwrap() {
            let source: Vec<u8> = serde_json::from_value(case["sourceRevision"].clone()).unwrap();
            let current: Option<[u8;32]> = serde_json::from_value(case["currentRevision"].clone()).unwrap();
            let Some(revision) = current else { assert!(!pin_solution::completion_matches(&encode(&source), None)); continue; };
            let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "bitmap".into(), operation_id: 1, generation: 1, canonical_base_revision: revision, retained: grant, authoring_seed: "completion".into() };
            let command = BitmapEditorCommand::CommitFillSolve { pixels: encode(&vec![1;cells]), contradiction: false, width: snapshot.output.width, height: snapshot.output.height, source_revision: encode(&source) };
            let input = ArtifactCommandInputs::<EditorApp<BitmapEditor>> { command: &command, snapshot: &snapshot, snapshot_owner: None, snapshot_read: ArtifactCommandSnapshotRead::unavailable(&operation), config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation };
            let mut cache = BitmapTransient { output_pixels: Some(encode(&vec![0;cells])), contradiction: false, output_width: snapshot.output.width, output_height: snapshot.output.height, source_revision: encode(&revision) };
            let before = cache.clone();
            let mut work = BitmapCommandWork::new("commit-fill-solve");
            let mut caller = semio_plugin::artifact_app_laws::OriginalWorkFixture::new();
            let step = caller.work_turn(grant, 1, |cx| { cx.consume_fuel(1); work.step(&input, cx) }).unwrap();
            let rows = match step { ArtifactCommandWorkStep::CompleteWithEphemeral { ephemeral, .. } => ephemeral.transient, ArtifactCommandWorkStep::Complete(_) => Vec::new(), _ => panic!("completion must settle") };
            let visible = case["visible"].as_bool().unwrap();
            assert_eq!(rows.len(), usize::from(visible), "{}", case["id"]);
            for row in rows { cache = vcs::apply_mutation(&cache, &row).unwrap().0; }
            assert_eq!(cache != before, visible, "{}", case["id"]);
            if !visible { assert_eq!(cache, before); }
            assert!(work.terminal_is_empty());
            caller.finish();
        }
    }

}

semio_plugin::retained_command::terminal_frame_bound!(BitmapCommandWork);
