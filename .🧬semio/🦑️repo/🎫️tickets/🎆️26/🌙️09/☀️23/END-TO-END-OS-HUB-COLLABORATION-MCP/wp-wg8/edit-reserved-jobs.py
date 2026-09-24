import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:120])
    text = text.replace(old, new)


IDLE = "ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle, typed_results: Vec::new() }"

replace(
    "    struct KernelPoolState {\n",
    """    /// 🧰️ One framework-reserved tool job (`Effect::SpawnJob` of kind
    /// `semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND`: undo, redo, copy/paste, selection and
    /// interaction verbs) this host drives to its end, the native twin of the React host's
    /// `driveReservedToolJob` (`🧱️elements/🔌️PluginRuntime/🟦️.tsx`). `step` is the exact shard-minted
    /// [`JobTurn`] of its next `Payload::JobStep`; `None` once the job ended or its spawn was refused, while
    /// the shard's deferred `Event::JobCompleted` turn (an outcome no grant asked for) is still owed.
    #[derive(Clone, Copy)]
    struct ReservedToolJob {
        actor: ActorId,
        job: u64,
        step: Option<JobTurn>,
    }

    struct KernelPoolState {
""",
)

replace(
    "        job_replays: [Option<MountedJobReplay>; JOB_PROGRESS_ACTIVE_CAPACITY],\n",
    "        job_replays: [Option<MountedJobReplay>; JOB_PROGRESS_ACTIVE_CAPACITY],\n        reserved_jobs: [Option<ReservedToolJob>; JOB_PROGRESS_ACTIVE_CAPACITY],\n",
)

replace(
    "                job_replays: std::array::from_fn(|_| None),\n",
    "                job_replays: std::array::from_fn(|_| None),\n                reserved_jobs: [None; JOB_PROGRESS_ACTIVE_CAPACITY],\n",
)

replace(
    """            while let Some(batch) = batches.pop_front() {
                let (mut outcome, continuation) = self.run_turn_once(actor, instance, batch, deadline).await?;
                let mut owed = run_turn_continuation_turns(continuation);
                let mut quiet = 0usize;
                while let Some(events) = owed.pop_front() {
                    if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                        break;
                    }
                    let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                    quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                    outcome.absorb(later)?;
                    owed.extend(run_turn_continuation_turns(next));
                }
                match settled.as_mut() {
                    Some(earlier) => earlier.absorb(outcome)?,
                    None => settled = Some(outcome),
                }
            }
            settled.ok_or_else(|| "kernel: a turn with no batch produced no outcome".to_string())
        }
""",
    """            while let Some(batch) = batches.pop_front() {
                let first = self.run_turn_once(actor, instance, batch, deadline).await?;
                let outcome = self.settle_turn(actor, instance, first, deadline).await?;
                match settled.as_mut() {
                    Some(earlier) => earlier.absorb(outcome)?,
                    None => settled = Some(outcome),
                }
            }
            settled.ok_or_else(|| "kernel: a turn with no batch produced no outcome".to_string())
        }

        /// 🔁️ Continues one granted turn with what it still owes (see [`Self::run_turn`]) until it settled,
        /// [`run_turn_quiescent_continuations`] continuations in a row carried nothing, or `deadline` passed.
        async fn settle_turn(&mut self, actor: ActorId, instance: u32, (mut outcome, continuation): (ExchangeOutcome, Option<Vec<Event>>), deadline: std::time::Instant) -> Result<ExchangeOutcome, String> {
            let mut owed = run_turn_continuation_turns(continuation);
            let mut quiet = 0usize;
            while let Some(events) = owed.pop_front() {
                if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                    break;
                }
                let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                outcome.absorb(later)?;
                owed.extend(run_turn_continuation_turns(next));
            }
            Ok(outcome)
        }

        /// 🧰️ Drives every live [`ReservedToolJob`] of `actor` to its end once the request that spawned it
        /// settled, folding what the drive produced into `outcome`: the native host's half of the one
        /// reserved-job mechanism (React: `driveReservedToolJob`; guest: `admit_reserved_spawned_job`, ticket
        /// 26/09/23 slices C8 and WG8 §1.4).
        ///
        /// - The shard started the job when it admitted the `Effect::SpawnJob` and reported its exact
        ///   [`JobTurn`] with that turn ([`ShardOutcome::Turn`]'s `jobs`); one `Payload::JobStep` is granted
        ///   per turn until a step publication ends the job.
        /// - The shard then runs the guest's deferred `Event::JobCompleted` turn by itself; this host waits
        ///   for it and settles it like any turn, so the guest commits the job one unit per `MoreWork`
        ///   continuation and answers the verb (`Invocation { in_reply_to: 0 }`, `OperationCompleted`).
        async fn settle_reserved_jobs(&mut self, actor: ActorId, instance: u32, outcome: &mut ExchangeOutcome) -> Result<(), String> {
            let deadline = std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET;
            while self.reserved_jobs.iter().flatten().any(|job| job.actor == actor) {
                if std::time::Instant::now() >= deadline {
                    return Err(format!("kernel: actor {}'s reserved tool jobs did not end within {RUN_TURN_SETTLE_BUDGET:?}", actor.0));
                }
                let first = self.run_reserved_job_once(actor, instance, deadline).await?;
                let later = self.settle_turn(actor, instance, first, deadline).await?;
                outcome.absorb(later)?;
            }
            Ok(())
        }

        /// 🧾️ Registers every reserved-kind `Effect::SpawnJob` of `actor`'s settled turn with the
        /// shard-minted [`JobTurn`] its admission reported; a spawn the shard refused has none and only owes
        /// its deferred `Event::JobCompleted` refusal.
        fn admit_reserved_jobs(&mut self, actor: ActorId, effects: &[Effect], jobs: &[JobTurn]) -> Result<(), String> {
            for effect in effects {
                let Effect::SpawnJob { job, kind, .. } = effect else { continue };
                if kind != semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND {
                    continue;
                }
                let slot = self.reserved_jobs.iter_mut().find(|slot| slot.is_none()).ok_or_else(|| format!("kernel: actor {}'s reserved tool job {job} exceeds {JOB_PROGRESS_ACTIVE_CAPACITY} live reserved jobs", actor.0))?;
                *slot = Some(ReservedToolJob { actor, job: *job, step: jobs.iter().find(|turn| turn.job == *job).copied() });
            }
            Ok(())
        }
""",
)

replace(
    """            for envelope in &envelopes {
                let began_job = match &envelope.payload {
                    Payload::JobStep { turn } => self.begin_job_progress(actor, turn)?,
""",
    f"""            self.dispatch_turn(actor, instance, envelopes, replay_start_index, deadline).await
        }}

        /// 🧰️ One turn of `actor`'s oldest live [`ReservedToolJob`]: its next `Payload::JobStep`, or, once
        /// the job ended, no envelope at all and only the wait for the deferred `Event::JobCompleted` turn
        /// [`Self::dispatch_turn`] counts as owed.
        async fn run_reserved_job_once(&mut self, actor: ActorId, instance: u32, deadline: std::time::Instant) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {{
            let Some(oldest) = self.reserved_jobs.iter().flatten().filter(|job| job.actor == actor).min_by_key(|job| job.job).copied() else {{
                return Ok(({IDLE}, None));
            }};
            let envelopes = match oldest.step {{
                Some(turn) => vec![Envelope {{ to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq: next_seq()?, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep {{ turn }} }}],
                None => Vec::new(),
            }};
            self.dispatch_turn(actor, instance, envelopes, None, deadline).await
        }}

        /// 🚚️ Submits `envelopes` to `actor`, then grants and collects until nothing is left to grant and
        /// no deferred reserved-job completion of `actor` is owed. An outcome no grant of this call asked
        /// for is such a completion: the shard runs a job's `Event::JobCompleted` turn by itself.
        async fn dispatch_turn(&mut self, actor: ActorId, instance: u32, envelopes: Vec<Envelope>, replay_start_index: Option<usize>, deadline: std::time::Instant) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {{
            let stepped_reserved_job = envelopes.iter().find_map(|envelope| match &envelope.payload {{
                Payload::JobStep {{ turn }} => self.reserved_jobs.iter().flatten().any(|job| job.actor == actor && job.job == turn.job).then_some(turn.job),
                _ => None,
            }});
            for envelope in &envelopes {{
                let began_job = match &envelope.payload {{
                    Payload::JobStep {{ turn }} if stepped_reserved_job != Some(turn.job) => self.begin_job_progress(actor, turn)?,
""",
)

replace("let mut turn_result: Option<TurnResult> = None;", "let mut turn_results: Vec<TurnResult> = Vec::new();")

replace(
    """            loop {
                self.now_ms += 1;
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |_actor| crate::actor_budget_from_turn_budget(TURN_BUDGET, Lane::Interactive)).await;
                if decision.run.is_empty() {
                    break;
                }
                let outcomes = self.runtime.wait_for_outcomes(decision.run.len(), RUN_TURN_OUTCOME_TIMEOUT);
                if outcomes.len() < decision.run.len() {
                    return Err("kernel: shard produced no outcome for this turn".to_string());
                }
                for outcome in outcomes {
                    match outcome {
                        ShardOutcome::Turn { actor: reported, result, .. } => {
                            let _ = self.runtime.complete_actor(ActorId(reported), &result, self.now_ms).await;
                            if reported == actor.0 {
                                turn_result = Some(decode_actor_turn_result(&result, reported)?);
                            } else {""",
    """            loop {
                self.now_ms += 1;
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |_actor| crate::actor_budget_from_turn_budget(TURN_BUDGET, Lane::Interactive)).await;
                let mut granted: Vec<u64> = decision.run.iter().map(|grant| grant.actor.0).collect();
                let expected = granted.len() + self.reserved_jobs.iter().flatten().filter(|job| job.actor == actor && job.step.is_none()).count();
                if expected == 0 {
                    break;
                }
                let outcomes = self.runtime.wait_for_outcomes(expected, RUN_TURN_OUTCOME_TIMEOUT);
                if outcomes.len() < granted.len() {
                    return Err("kernel: shard produced no outcome for this turn".to_string());
                }
                if outcomes.len() < expected {
                    return Err(format!("kernel: actor {}'s deferred reserved-job completion turn never arrived", actor.0));
                }
                for outcome in outcomes {
                    let reported = outcome.actor();
                    if let Some(index) = granted.iter().position(|granted| *granted == reported) {
                        granted.swap_remove(index);
                    } else if let Some(slot) = self.reserved_jobs.iter_mut().find(|slot| slot.as_ref().is_some_and(|job| job.actor.0 == reported && job.step.is_none())) {
                        *slot = None;
                    }
                    match outcome {
                        ShardOutcome::Turn { actor: reported, result, jobs } => {
                            let _ = self.runtime.complete_actor(ActorId(reported), &result, self.now_ms).await;
                            if reported == actor.0 {
                                let decoded = decode_actor_turn_result(&result, reported)?;
                                self.admit_reserved_jobs(actor, &decoded.effects, &jobs)?;
                                turn_results.push(decoded);
                            } else {""",
)

replace(
    """                        ShardOutcome::Job { actor: reported, authority, request, placement, publication } => {""",
    """                        ShardOutcome::Job { actor: reported, authority, publication, .. } if self.reserved_jobs.iter().flatten().any(|job| job.actor.0 == reported && job.job == authority.job) => {
                            let next = match publication.outcome {
                                semio_framework_actor::JobStepOutcome::Complete { .. } | semio_framework_actor::JobStepOutcome::Cancelled | semio_framework_actor::JobStepOutcome::Fault { .. } => None,
                                semio_framework_actor::JobStepOutcome::Yield | semio_framework_actor::JobStepOutcome::PreviewReady { .. } | semio_framework_actor::JobStepOutcome::CheckpointReady { .. } => {
                                    Some(JobTurn { step_sequence: publication.turn.step_sequence.saturating_add(1), ..publication.turn })
                                }
                            };
                            if let Some(job) = self.reserved_jobs.iter_mut().flatten().find(|job| job.actor.0 == reported && job.job == authority.job) {
                                job.step = next;
                            }
                        }
                        ShardOutcome::Job { actor: reported, authority, request, placement, publication } => {""",
)

replace(
    """                            let _ = self.runtime.complete_actor(ActorId(reported), &faulted, self.now_ms).await;
                            if reported == actor.0 {
                                fault = Some(message);
                            }""",
    """                            let _ = self.runtime.complete_actor(ActorId(reported), &faulted, self.now_ms).await;
                            if reported == actor.0 {
                                if let Some(slot) = self.reserved_jobs.iter_mut().find(|slot| slot.as_ref().is_some_and(|job| job.actor == actor && Some(job.job) == stepped_reserved_job)) {
                                    *slot = None;
                                }
                                fault = Some(message);
                            }""",
)

old_tail_start = text.index("            match turn_result {")
old_tail_end = text.index("        async fn apply_turn_result(", old_tail_start)
old_tail = text[old_tail_start:old_tail_end]
assert "None if replay_capture_started" in old_tail and old_tail.count("Ok((outcome, continuation))") == 1
text = (
    text[:old_tail_start]
    + f"""            if turn_results.is_empty() {{
                return if replay_capture_started || stepped_reserved_job.is_some() {{ Ok(({IDLE}, None)) }} else {{ Err("kernel: shard produced no outcome for this turn".to_string()) }};
            }}
            let mut settled: Option<ExchangeOutcome> = None;
            let mut owed = Vec::new();
            let mut more_work = false;
            for result in turn_results {{
                owed.extend(result.lifecycle_receipt.map(|receipt| Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck {{ receipt }})));
                more_work = matches!(result.status, semio_framework::kernel::TurnStatus::MoreWork)
                    && matches!(result.command_ingress, semio_framework::kernel::CommandIngressStatus::Idle)
                    && matches!(result.cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Idle);
                let outcome = self.apply_turn_result(actor, instance, result).await?;
                owed.extend(outcome.typed_results.iter().map(|page| Event::Message {{ source: MessageEndpoint::Shell {{ instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }}, payload: TypedOperationResultPage::encode_ack(page.token) }}));
                match settled.as_mut() {{
                    Some(earlier) => earlier.absorb(outcome)?,
                    None => settled = Some(outcome),
                }}
            }}
            let outcome = settled.ok_or_else(|| "kernel: settled turn results produced no outcome".to_string())?;
            let continuation = if !owed.is_empty() {{ Some(owed) }} else {{ more_work.then(Vec::new) }};
            Ok((outcome, continuation))
        }}

"""
    + text[old_tail_end:]
)

replace(
    """            let mut typed_results = Vec::new();
            for effect in result.effects {
""",
    """            let mut typed_results = Vec::new();
            for effect in result.effects {
                if matches!(&effect, Effect::SpawnJob { kind, .. } if kind == semio_framework_plugin::app::FRAMEWORK_RESERVED_JOB_KIND) {
                    continue;
                }
""",
)

replace(
    "            self.run_turn(actor, instance, events).await\n",
    "            let mut outcome = self.run_turn(actor, instance, events).await?;\n            self.settle_reserved_jobs(actor, instance, &mut outcome).await?;\n            Ok(outcome)\n",
)

replace(
    "                        self.command_document_closes.publish_batch(key, generation, &mut combined.surfaces);\n",
    "                        self.command_document_closes.publish_batch(key, generation, &mut combined.surfaces);\n                        self.settle_reserved_jobs(actor, instance, &mut combined).await?;\n",
)

replace(
    "let removed = self.runtime.kernel_mut().deactivate(actor).await.unwrap_or_else(|_| vec![actor]);\n",
    """let removed = self.runtime.kernel_mut().deactivate(actor).await.unwrap_or_else(|_| vec![actor]);
                    for slot in &mut self.reserved_jobs {
                        if slot.as_ref().is_some_and(|job| removed.contains(&job.actor)) {
                            *slot = None;
                        }
                    }
""",
)

path.write_text(text)
print("ok")
