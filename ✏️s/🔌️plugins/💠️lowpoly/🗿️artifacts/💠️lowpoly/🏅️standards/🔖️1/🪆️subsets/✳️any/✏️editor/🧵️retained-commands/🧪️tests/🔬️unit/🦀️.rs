//! 🧵️ Private retained command partition, replay, freshness, close and turn laws.

use super::*;
use semio_framework_3d::mesh::MeshModelingOps;
#[path = "../../../🧪️tests/🧰️support/commands.rs"]
mod command_fixture;
use command_fixture::every_command;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
fn retained_operation() -> AppOperationContext {
    AppOperationContext { app_instance_id: 7, parent_document_id: "lowpoly-retained-test".into(), operation_id: 11, generation: 13, canonical_base_revision: [17; 32], authoring_seed: "authoring-seed-test".into(), retained: semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance }
}

fn retained_context(transient: LowpolyTransient, transient_generation: u64) -> std::sync::Arc<ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>> {
    std::sync::Arc::new(ArtifactOwnedToolJobContext::new(
        7,
        None,
        [17; 32],
        0,
        transient_generation,
        semio_plugin::app::ArtifactOwnedToolJobSnapshots {
            children: std::sync::Arc::new(semio_plugin::ChildContentView::EMPTY),
            draft: std::sync::Arc::new(NoDraft::default()),
            transient: std::sync::Arc::new(transient),
            window_config: None,
            window_transient: None,
        },
    ))
}

#[test]
fn retained_route_partition_and_publication_are_exact() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    let declared = LowpolyCommand::TOOL_JOB_IDS.len();
    let all = every_command();
    let mut partition = LOWPOLY_MIGRATED_TOOL_IDS.to_vec();
    partition.extend_from_slice(LOWPOLY_IMPORT_TOOL_IDS);
    partition.sort_unstable();
    partition.dedup();
    assert_eq!(partition.len(), declared, "every generated command id is Migrated");
    assert_eq!(all.len(), partition.len());
    assert!(all.iter().all(|command| partition.binary_search(&command.command_id()).is_ok()));
    assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
    assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), declared);
    assert_eq!(<LowpolyCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len() + <LowpolyImportJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len(), declared);
    assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
    assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
}

#[semio_framework_async_macros::async_test]
async fn retained_progress_replay_freshness_and_close_are_exact() {
    let command = LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {});
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 19);
    let context_identity = context.identity_digest();
    let mut uninterrupted = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(matches!(
        uninterrupted
            .step(&semio_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("progress"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    let checkpoint: Vec<u8> = (0..).map_while(|index| uninterrupted.checkpoint_byte(index)).collect();
    assert_eq!(checkpoint.len(), 72);
    let mut wrong_base = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, [18; 32], context_identity);
    assert!(wrong_base.restore(&checkpoint).is_err());
    let mut replayed = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    replayed.restore(&checkpoint).expect("work restore");
    assert!(matches!(
        replayed
            .step(&semio_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("replay"),
        ArtifactCommandWorkStep::Replay { .. }
    ));
    assert!(matches!(
        replayed
            .step(&semio_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("complete"),
        ArtifactCommandWorkStep::Complete(_)
    ));
    let drifted = AppOperationContext { generation: operation.generation + 1, ..operation.clone() };
    let mut rejected = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(rejected
        .step(&semio_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&drifted), command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: Some(&context), operation: &drifted }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
        .is_err());
    let drifted_context = retained_context(LowpolyTransient::default(), 20);
    assert!(rejected
        .step(&semio_plugin::retained_command::ArtifactCommandInputs {
            snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command,
            snapshot: &snapshot,
            config: &config,
            history: &history,
            interaction: &interaction,
            hover: &hover,
            context: Some(&drifted_context),
            operation: &operation
        }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
        .is_err());
    assert!(matches!(
        rejected
            .step(&semio_plugin::retained_command::ArtifactCommandInputs {
                snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
            .expect("exact retry"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(0), |cx| replayed.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Blocked);
    replayed.begin_close();
    assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(1), |cx| replayed.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
    assert!(replayed.terminal_is_empty());
}

fn close_grant(maximum_items: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 8 }
}

/// ⏱️ The interactive-step law over the UNIT BOX, not the concrete-forest default document: on 195
/// faces a debug build measures `mirror` at 42 ms, `loopCut` 18 ms, `toggleSmooth` 10 ms and even a
/// whole-object `translateSelection` 9 ms — the JSON parse + kernel pass + re-encode floor of one
/// monolithic mesh edit, which the runtime records as a single admitted overrun (`StepOverrunLedger`
/// quarantines only four CONSECUTIVE ones). Over the box every command's cost is its dispatch
/// overhead, which is what this law guards: `addPrimitive` re-encoding every untouched mesh (11.6 ms)
/// was such a bug (2026-09-18). Chunking whole-mesh kernel ops into resumable steps is a kernel job.
///
/// ⏱️ The ceiling is the runtime's own `INTERACTIVE_STEP_CEILING_US` in every build. What is measured is
/// the command's intrinsic turn cost: on a host oversubscribed three to four times (load 26–41 on ten
/// cores) a 5 ms turn takes 15–40 ms of wall time, so the law keeps the fastest of up to
/// `INTERACTIVE_TURN_ATTEMPTS` runs. A descheduled attempt is retried; a command whose own cost exceeds
/// the ceiling fails every attempt. Measured 2026-09-24 in a debug build at load 33: slowest command
/// `paintFill` 5.3 ms, every other command under 3.1 ms.
const INTERACTIVE_TURN_CEILING: std::time::Duration = std::time::Duration::from_micros(semio_framework_job::INTERACTIVE_STEP_CEILING_US);
const INTERACTIVE_TURN_ATTEMPTS: usize = 16;
#[semio_framework_async_macros::async_test]
async fn retained_migrated_turns_stay_below_eight_milliseconds() {
    let unit_box = semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("box json");
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json(&unit_box, "obj-1", "Unit Box");
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 29);
    for command in every_command().into_iter().filter(|command| LOWPOLY_MIGRATED_TOOL_IDS.contains(&command.command_id())) {
        let tool_id = command.command_id();
        let disposition = lowpoly_command_disposition(tool_id).expect("migrated disposition");
        let mut best = std::time::Duration::MAX;
        for _attempt in 0..INTERACTIVE_TURN_ATTEMPTS {
            let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
            let mut slowest = std::time::Duration::ZERO;
            let mut refused = false;
            loop {
                let started = std::time::Instant::now();
                let step = work.step(&semio_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation),
                    command: &command,
                    snapshot: &snapshot,
                    config: &config,
                    history: &history,
                    interaction: &interaction,
                    hover: &hover,
                    context: Some(&context),
                    operation: &operation,
                }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()));
                slowest = slowest.max(started.elapsed());
                match step {
                    Ok(ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) => break,
                    Ok(_) => {}
                    Err(fault) => {
                        assert!(fault.message.contains("selected") || fault.message.contains("selection") || fault.message.contains("at least one object"), "{tool_id} refused for a reason other than the empty selection: {fault:?}");
                        refused = true;
                        break;
                    }
                }
            }
            best = best.min(slowest);
            if !refused {
                work.begin_close();
                assert_eq!(semio_framework_job::close_with_self_funded_context(close_grant(1), |cx| work.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
            }
            if best < INTERACTIVE_TURN_CEILING {
                break;
            }
        }
        assert!(best < INTERACTIVE_TURN_CEILING, "{tool_id} turn exceeded {INTERACTIVE_TURN_CEILING:?} on every attempt: best {best:?}");
    }
}


fn fill_caller(operation: &AppOperationContext) -> semio_framework_job::OriginalJobFixtureCaller {
    let stock = semio_framework_job::RetainedCloneGrant { maximum_items: 1 << 26, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 29, maximum_release_bytes: 1 << 29, maximum_depth: 4096 };
    semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(operation.operation_id), semio_framework_job::Generation(operation.generation), stock, semio_framework_job::default_now_us, u64::MAX, 1 << 30)
}

fn fill_turn(caller: &mut semio_framework_job::OriginalJobFixtureCaller, decoder: &mut semio_framework_job::OriginalJobDecodeOwner, work: &mut LowpolyRetainedCommandWork, input: &semio_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LowpolyPlayApp>>, grant: semio_framework_job::RetainedCloneGrant, fuel: u64) -> (ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, u128) {
    let mut result = None;
    let mut spent = 0;
    caller.turn_with_decoder(grant, decoder, |cx| {
        let deadline = cx.original_absolute_deadline_us();
        cx.with_original_work_loan(fuel, deadline, |cx| {
            let before = cx.original_work_units_spent();
            result = Some(work.step(input, cx)?);
            spent = cx.original_work_units_spent() - before;
            Ok(())
        })
    }).expect("original fill turn");
    (result.expect("fill returned one step"), spent)
}

fn retire_fill_owner<T: semio_framework_value::retirement::RetireOwned>(caller: &mut semio_framework_job::OriginalJobFixtureCaller, decoder: &mut semio_framework_job::OriginalJobDecodeOwner, owner: T, grant: semio_framework_job::RetainedCloneGrant, maximum_turns: u64) {
    let mut owner = Some(owner);
    let mut retirement = semio_framework_value::retained_clone::RetainedCloneClose::default();
    for _ in 0..maximum_turns {
        if owner.is_none() && retirement.is_empty() { return; }
        caller.turn_with_decoder(grant, decoder, |cx| cx.with_original_retirement(|cx| {
            let demand = if retirement.is_empty() { retirement.next_owner_demands::<T>(true, &None, grant.maximum_copy_bytes)? } else { retirement.demands(grant.maximum_copy_bytes)? };
            semio_framework_job::close_original_retained_step(cx, demand, |paid| {
                if retirement.is_empty() { let turn = retirement.begin_granted(&mut owner, paid); return semio_framework_value::RetainedWorkOutcome { result: turn.result.map(|step| step.expect("owner close returns a step")), work: turn.work }; }
                match retirement.select(grant.maximum_copy_bytes) { Ok(selected) => selected.step(paid), Err(error) => semio_framework_value::RetainedWorkOutcome { result: Err(error), work: 0 } }
            })?;
            Ok(())
        })).expect("paid emitted owner close");
    }
    panic!("emitted fill owner did not retire");
}

fn finish_fill(mut caller: semio_framework_job::OriginalJobFixtureCaller, decoder: &mut semio_framework_job::OriginalJobDecodeOwner, work: &mut LowpolyRetainedCommandWork, grant: semio_framework_job::RetainedCloneGrant, maximum_turns: u64) {
    work.begin_close();
    let mut closed = false;
    for _ in 0..maximum_turns {
        let step = caller.turn_with_decoder(grant, decoder, |cx| cx.with_original_retirement(|cx| work.close_step(cx))).expect("original fill close");
        if matches!(step, InteractiveJobCloseStep::Complete { .. }) { closed = true; break; }
    }
    assert!(closed && work.terminal_is_empty(), "fill workspace retires through bounded original close turns");
    for _ in 0..maximum_turns {
        if decoder.terminal_is_empty() { caller.retire(grant); return; }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            let demand = decoder.retirement_demands(grant.maximum_copy_bytes)?;
            decoder.close_on_source(cx)?;
            Ok(())
        })).expect("original fill decoder close");
    }
    panic!("fill decoder did not retire");
}

#[test]
fn retained_fill_pays_for_pixel_work_and_exposes_cancellation_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/fill-budget.json")).unwrap();
    let size = fixture["textureSize"].as_u64().unwrap() as usize;
    assert_eq!(size, semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_PAINT_TEXTURE_SIZE);
    let color: [u8; 4] = std::array::from_fn(|index| fixture["color"][index].as_u64().unwrap() as u8);
    let background: [u8; 4] = std::array::from_fn(|index| fixture["background"][index].as_u64().unwrap() as u8);
    let mesh = semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap().to_json().unwrap();
    let mut snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json(&mesh, fixture["objectId"].as_str().unwrap(), "Budget Mesh / Budget-Netz");
    let mut layer = semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolyPaintLayer::new("Base / Basis");
    layer.pixels = background.repeat(size * size);
    snapshot.objects[0].paint_layers = vec![layer];
    let config = LowpolyConfig { paint_color_r: color[0], paint_color_g: color[1], paint_color_b: color[2], paint_color_a: color[3], active_paint_layer: 0, ..Default::default() };
    let command = LowpolyCommand::PaintFill(paint_fill::PaintFill {
        object_id: Some(fixture["objectId"].as_str().unwrap().into()),
        u: Some(fixture["startUv"][0].as_f64().unwrap() as f32),
        v: Some(fixture["startUv"][1].as_f64().unwrap() as f32),
        x: None,
        y: None,
    });
    assert_eq!(command.command_id(), fixture["action"].as_str().unwrap());
    let interaction = protocol::InteractionState::default();
    let hover = semio_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 31);
    let input = semio_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: Some(&context), operation: &operation };
    let mut work = LowpolyRetainedCommandWork::new(command.command_id(), lowpoly_command_disposition(command.command_id()).unwrap(), operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
    let fuel = fixture["turn"]["fuel"].as_u64().unwrap();
    let grant = semio_framework_job::RetainedCloneGrant {
        maximum_items: fixture["turn"]["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes: fixture["turn"]["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: fixture["turn"]["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes: fixture["turn"]["maximumReleaseBytes"].as_u64().unwrap() as usize,
        maximum_depth: fixture["turn"]["maximumDepth"].as_u64().unwrap() as usize,
    };
    assert_eq!(grant, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance);
    let mut paid_work = 0_u128;
    let mut progress_turns = 0_u64;
    let mut completed = None;
    let mut caller = fill_caller(&operation);
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
        let (step, spent) = fill_turn(&mut caller, &mut decoder, &mut work, &input, grant, fuel);
        assert!(spent <= u128::from(fuel), "one turn exceeds its original fuel");
        paid_work += spent;
        match step {
            ArtifactCommandWorkStep::Complete(emit) => { completed = Some(emit); break; }
            ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("one-shot fill has no ephemeral publication lane"),
            ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::Replay { .. } => progress_turns += 1,
            ArtifactCommandWorkStep::Pending { .. } => {},
            ArtifactCommandWorkStep::CompleteDownload { .. } => panic!("fill does not export a download"),
        }
    }
    let emit = completed.expect("bounded fill completes within its declared extent");
    let mut pixels = background.repeat(size * size);
    for mutation in &emit.artifact_mutations {
        let LowpolyMutation::EditPaintLayer(edit) = mutation else { panic!("fill publishes one paint-layer delta") };
        assert_eq!(edit.object_id, fixture["objectId"].as_str().unwrap());
        assert_eq!(edit.layer_index, 0);
        for run in &edit.runs {
            let offset = run.offset as usize;
            let end = offset.checked_add(run.bytes.len()).unwrap();
            assert!(end <= pixels.len());
            pixels[offset..end].copy_from_slice(&run.bytes);
        }
    }
    let sums: [u64; 4] = std::array::from_fn(|channel| pixels.chunks_exact(4).map(|pixel| u64::from(pixel[channel])).sum());
    let observed = serde_json::json!({ "publishedMutations": emit.artifact_mutations.len(), "changedPixels": pixels.chunks_exact(4).filter(|pixel| *pixel != background.as_slice()).count(), "channelSums": sums });
    assert_eq!(observed, fixture["expected"]);
    assert!(pixels.chunks_exact(4).all(|pixel| pixel == color.as_slice()));
    assert!(paid_work >= u128::from(fixture["minimumPaidWork"].as_u64().unwrap()), "fill must charge its real pixel traversal");
    assert!(progress_turns >= fixture["minimumProgressTurns"].as_u64().unwrap(), "fill must yield throughout domain work");
    let maximum_turns = fixture["maximumTurns"].as_u64().unwrap();
    retire_fill_owner(&mut caller, &mut decoder, emit, grant, maximum_turns);
    finish_fill(caller, &mut decoder, &mut work, grant, maximum_turns);
}

#[test]
fn import_factory_owns_only_import_payload_capacity() {
    use protocol::OpBinary;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🔌️mounted/import.json")).unwrap();
    assert_eq!(media::LOWPOLY_MESH_FILE_BYTES, fixture["limits"]["payloadBytes"].as_u64().unwrap() as usize);
    assert_eq!(LOWPOLY_RETAINED_FIELD_BYTES, fixture["limits"]["nameBytes"].as_u64().unwrap() as usize);
    assert_eq!(LOWPOLY_IMPORT_RAW_BYTES, fixture["limits"]["importCommandBytes"].as_u64().unwrap() as usize);
    let ordinary = LowpolyCommandJobFactory::new("fixture");
    let import = LowpolyImportJobFactory::new("fixture");
    assert!(!LOWPOLY_MIGRATED_TOOL_IDS.contains(&"importMeshFile"));
    assert_eq!(LOWPOLY_IMPORT_TOOL_IDS, &["importMeshFile", "importSnapshotJson", "replaceSnapshotJson"]);
    assert_eq!(ordinary.execution_contract().max_raw_wire_bytes, fixture["limits"]["ordinaryCommandBytes"].as_u64().unwrap() as usize);
    assert_eq!(import.execution_contract().max_raw_wire_bytes, LOWPOLY_IMPORT_RAW_BYTES);
    let proofs = <LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs();
    for proof in proofs {
        if LOWPOLY_IMPORT_TOOL_IDS.contains(&proof.tool_id) {
            assert_eq!(proof.factory_type_id, Some(std::any::TypeId::of::<LowpolyImportJobFactory>()));
            assert_eq!(proof.contract.max_raw_wire_bytes, LOWPOLY_IMPORT_RAW_BYTES);
        } else {
            assert_eq!(proof.factory_type_id, Some(std::any::TypeId::of::<LowpolyCommandJobFactory>()));
            assert_eq!(proof.contract.max_raw_wire_bytes, 16_384);
        }
    }
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let config = LowpolyConfig::default();
    let mut command = LowpolyCommand::ImportMeshFile(media::import_mesh_file::ImportMeshFile { name: "a".repeat(LOWPOLY_RETAINED_FIELD_BYTES), payload: "#".repeat(media::LOWPOLY_MESH_FILE_BYTES) });
    assert!(lowpoly_command_admitted(&command, &snapshot, &config));
    assert_eq!(lowpoly_command_maximum_raw_bytes(&command), LOWPOLY_IMPORT_RAW_BYTES);
    assert!(command.encode_op().unwrap().len() <= LOWPOLY_IMPORT_RAW_BYTES);
    if let LowpolyCommand::ImportMeshFile(payload) = &mut command { payload.payload.push('x'); }
    assert!(!lowpoly_command_admitted(&command, &snapshot, &config));
}

#[test]
fn retained_fill_comb_preserves_sparse_pixel_topology() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/fill-comb.json")).unwrap();
    let width = fixture["width"].as_u64().unwrap() as usize;
    let height = fixture["height"].as_u64().unwrap() as usize;
    let stride = fixture["whiteColumnStride"].as_u64().unwrap() as usize;
    let connector = fixture["connectorRow"].as_u64().unwrap() as usize;
    let rgba = |key: &str| -> [u8; 4] { std::array::from_fn(|channel| fixture[key][channel].as_u64().unwrap() as u8) };
    let target = rgba("target");
    let background = rgba("background");
    let replacement = rgba("replacement");
    let mut original = background.repeat(width * height);
    let mut expected = original.clone();
    for y in 0..height {
        for x in 0..width {
            if x % stride == 0 || y == connector {
                let offset = (y * width + x) * 4;
                original[offset..offset + 4].copy_from_slice(&target);
                expected[offset..offset + 4].copy_from_slice(&replacement);
            }
        }
    }
    let mesh = semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap().to_json().unwrap();
    let mut snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::snapshot_from_mesh_json(&mesh, "comb", "Comb / Kamm");
    let mut layer = semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolyPaintLayer::new("Comb / Kamm");
    layer.pixels = original.clone();
    snapshot.objects[0].paint_layers = vec![layer];
    let config = LowpolyConfig { paint_color_r: replacement[0], paint_color_g: replacement[1], paint_color_b: replacement[2], paint_color_a: replacement[3], active_paint_layer: 0, ..Default::default() };
    let command = LowpolyCommand::PaintFill(paint_fill::PaintFill {
        object_id: Some("comb".into()),
        u: Some(fixture["seed"][0].as_u64().unwrap() as f32 / (width - 1) as f32),
        v: Some(1.0 - fixture["seed"][1].as_u64().unwrap() as f32 / (height - 1) as f32),
        x: None, y: None,
    });
    let interaction = protocol::InteractionState::default();
    let hover = semio_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 32);
    let input = semio_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, snapshot_read: semio_plugin::retained_command::ArtifactCommandSnapshotRead::unavailable(&operation), command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: Some(&context), operation: &operation };
    let mut work = LowpolyRetainedCommandWork::new(command.command_id(), lowpoly_command_disposition(command.command_id()).unwrap(), operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
    let grant = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance;
    let fuel = 256;
    let maximum_turns = 16_384;
    let mut caller = fill_caller(&operation);
    let mut decoder = semio_framework_job::OriginalJobDecodeOwner::empty();
    let mut completed = None;
    let mut spent_total = 0_u128;
    for _ in 0..maximum_turns {
        let (step, spent) = fill_turn(&mut caller, &mut decoder, &mut work, &input, grant, fuel);
        assert!(spent <= u128::from(fuel));
        spent_total += spent;
        match step {
            ArtifactCommandWorkStep::Complete(emit) => { completed = Some(emit); break; },
            ArtifactCommandWorkStep::Progress { .. } | ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Pending { .. } => {},
            _ => panic!("one-shot fill publishes only a document delta"),
        }
    }
    let emit = completed.expect("valid connected comb completes without an arbitrary sparse-run guard");
    assert_eq!(emit.artifact_mutations.len(), 1);
    let LowpolyMutation::EditPaintLayer(edit) = &emit.artifact_mutations[0] else { panic!("paint layer delta") };
    assert_eq!(edit.runs.len(), fixture["expected"]["sparseRuns"].as_u64().unwrap() as usize);
    assert!(edit.runs.len() <= fixture["expected"]["maximumSparseRuns"].as_u64().unwrap() as usize);
    let mut observed = original.clone();
    for run in &edit.runs {
        let offset = run.offset as usize;
        assert_eq!(offset % 4, 0);
        assert_eq!(run.bytes.len() % 4, 0);
        for (pixel, bytes) in run.bytes.chunks_exact(4).enumerate() {
            let start = offset + pixel * 4;
            assert_eq!(&original[start..start + 4], target.as_slice(), "a sparse run must not span unchanged gaps");
            assert_eq!(bytes, replacement.as_slice());
        }
        observed[offset..offset + run.bytes.len()].copy_from_slice(&run.bytes);
    }
    assert_eq!(observed, expected);
    assert_eq!(observed.chunks_exact(4).zip(original.chunks_exact(4)).filter(|(after, before)| after != before).count(), fixture["expected"]["changedPixels"].as_u64().unwrap() as usize);
    assert!(spent_total >= u128::from(fixture["expected"]["changedPixels"].as_u64().unwrap()));
    retire_fill_owner(&mut caller, &mut decoder, emit, grant, maximum_turns);
    finish_fill(caller, &mut decoder, &mut work, grant, maximum_turns);
}

#[test]
fn snapshot_json_import_budget_is_joined_without_widening_ordinary_commands() {
    use protocol::OpBinary;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🔌️mounted/snapshot.json")).unwrap();
    assert_eq!(LOWPOLY_SNAPSHOT_JSON_BYTES, fixture["limits"]["jsonBytes"].as_u64().unwrap() as usize);
    assert_eq!(LOWPOLY_SNAPSHOT_JSON_RAW_BYTES, fixture["limits"]["jsonCommandBytes"].as_u64().unwrap() as usize);
    assert_eq!(lowpoly_contract().max_raw_wire_bytes, fixture["limits"]["ordinaryBytes"].as_u64().unwrap() as usize);
    assert_eq!(lowpoly_import_contract().max_raw_wire_bytes, fixture["limits"]["importFactoryBytes"].as_u64().unwrap() as usize);
    let snapshot = semio_s_artifact_lowpoly_lowpoly_model::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let config = LowpolyConfig::default();
    for action in fixture["commands"].as_array().unwrap() {
        let id = action.as_str().unwrap();
        assert!(LOWPOLY_IMPORT_TOOL_IDS.contains(&id));
        assert!(!LOWPOLY_MIGRATED_TOOL_IDS.contains(&id));
        let command = |length| match id {
            "importSnapshotJson" => LowpolyCommand::ImportSnapshotJson(set_snapshot_json::ImportSnapshotJson { json: " ".repeat(length) }),
            "replaceSnapshotJson" => LowpolyCommand::ReplaceSnapshotJson(replace_snapshot_json::ReplaceSnapshotJson { json: " ".repeat(length) }),
            _ => unreachable!(),
        };
        let at = command(LOWPOLY_SNAPSHOT_JSON_BYTES);
        assert_eq!(lowpoly_command_maximum_raw_bytes(&at), LOWPOLY_SNAPSHOT_JSON_RAW_BYTES);
        assert!(lowpoly_command_admitted(&at, &snapshot, &config));
        assert!(at.encode_op().unwrap().len() <= LOWPOLY_SNAPSHOT_JSON_RAW_BYTES);
        assert!(!lowpoly_command_admitted(&command(LOWPOLY_SNAPSHOT_JSON_BYTES + 1), &snapshot, &config));
        let proofs = <LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs();
        let proof = proofs.iter().find(|proof| proof.tool_id == id).unwrap();
        assert_eq!(proof.factory_type_id, Some(std::any::TypeId::of::<LowpolyImportJobFactory>()));
        assert_eq!(proof.contract, lowpoly_import_contract());
    }
}
