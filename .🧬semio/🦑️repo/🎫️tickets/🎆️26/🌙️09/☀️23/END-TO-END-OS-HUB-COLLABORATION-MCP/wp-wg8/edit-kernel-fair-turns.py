#!/usr/bin/env python3
"""WG8 s12 (coordinator-approved 18:0x): a long guest turn never monopolises the native kernel request loop and is never
killed by a fixed wall-clock limit. A preempted actor yields at its slice: it is suspended in the kernel scheduler, the
request at the head of the queue is served when it belongs to another instance (one request per slice, the fairness
contract `🧑‍🎨engine/🧫️fixtures/🧵️kernel-turn-fairness`), then it resumes. Only an explicit cancel ends it: the open's owner
dropping its create request (the abandoned response slot) or a realm close at the head of the queue (shutdown)."""
import pathlib

RENDERER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = RENDERER.read_text()


def swap(old, new):
    global text
    assert text.count(old) == 1, old[:120]
    text = text.replace(old, new)


# 1. The settle budget no longer kills a preempted guest: it only bounds how long MoreWork continuations are settled.
swap(
    """    /// ⏳️ How long one [`KernelPoolState::run_turn`] request may keep its actor's turn going —
    /// preemption resumes and settle continuations together. The same order as
    /// `RUN_TURN_OUTCOME_TIMEOUT`'s per-grant wait: every grant is itself bounded by the 100 ms
    /// `TURN_BUDGET` wall. A guest still preempted when it runs out is reported as wedged; a guest
    /// still answering `MoreWork` is handed back settled-as-far-as-it-got, never mid-call.
    const RUN_TURN_SETTLE_BUDGET: Duration = Duration::from_secs(30);
""",
    """    /// ⏳️ How long one [`KernelPoolState::run_turn`] request keeps settling a guest that COMPLETED its turn
    /// answering `MoreWork`: past it the guest is handed back settled-as-far-as-it-got, never mid-call. A
    /// guest preempted MID-call is never bounded by wall-clock time: it yields its slice to the other
    /// actors ([`KernelPoolState::yield_slice`]) and only an explicit cancel ends it
    /// ([`KernelPoolState::turn_cancelled`]).
    const RUN_TURN_SETTLE_BUDGET: Duration = Duration::from_secs(30);

    /// ⚖️ How many queued requests of other instances run between two slices of a mid-flight turn — the
    /// fairness contract's `requestsBetweenSlices` (`🧑‍🎨engine/🧫️fixtures/🧵️kernel-pool-future` `turnFairness`); a
    /// slice itself is [`TURN_BUDGET`]'s fuel and wall.
    const TURN_REQUESTS_BETWEEN_SLICES: usize = 1;
""",
)

# 2. Response slots know when their requester gave up (the open's cancel drops the create request's future).
swap(
    """    #[derive(Default)]
    struct ResponseSlot {
        state: Mutex<ResponseState>,
    }

    impl ResponseSlot {
""",
    """    #[derive(Default)]
    struct ResponseSlot {
        state: Mutex<ResponseState>,
        abandoned: std::sync::atomic::AtomicBool,
    }

    impl ResponseSlot {
        /// 🛑️ Whether the requester dropped its future before an outcome reached it — the explicit cancel of
        /// a request whose owner follows it with progress and a cancel (a document open's create request).
        fn abandoned(&self) -> bool {
            self.abandoned.load(std::sync::atomic::Ordering::Acquire)
        }

""",
)
swap(
    """    struct KernelFuture {
        slot: Arc<ResponseSlot>,
        request: Option<KernelRequest>,
        queue: Arc<KernelRequestQueue>,
    }
""",
    """    struct KernelFuture {
        slot: Arc<ResponseSlot>,
        request: Option<KernelRequest>,
        queue: Arc<KernelRequestQueue>,
        finished: bool,
    }

    impl Drop for KernelFuture {
        fn drop(&mut self) {
            if !self.finished && self.request.is_none() {
                self.slot.abandoned.store(true, std::sync::atomic::Ordering::Release);
            }
        }
    }
""",
)
swap(
    """            if let Some(outcome) = state.result.take() {
                let previous = state.waker.take();
                drop(state);
                drop(previous);
                return Poll::Ready(outcome);
            }""",
    """            if let Some(outcome) = state.result.take() {
                let previous = state.waker.take();
                drop(state);
                drop(previous);
                this.finished = true;
                return Poll::Ready(outcome);
            }""",
)
swap(
    "            KernelFuture { slot: Arc::new(ResponseSlot::default()), request: Some(request), queue: self.queue.clone() }",
    "            KernelFuture { slot: Arc::new(ResponseSlot::default()), request: Some(request), queue: self.queue.clone(), finished: false }",
)

# 3. The queue can serve its head only when that head may run between another actor's slices.
swap(
    """        async fn next(&self) -> (KernelRequest, Arc<ResponseSlot>) {
            std::future::poll_fn(|cx| self.poll(cx)).await
        }
""",
    """        async fn next(&self) -> (KernelRequest, Arc<ResponseSlot>) {
            std::future::poll_fn(|cx| self.poll(cx)).await
        }

        /// 🎚️ Pops the head request only when `admit` accepts it — FIFO and the command credits stay exactly as
        /// [`Self::try_next`] keeps them; a head that may not run now stays the head.
        fn try_next_if(&self, admit: impl Fn(&KernelRequest) -> bool) -> Option<(KernelRequest, Arc<ResponseSlot>)> {
            {
                let state = self.state.try_lock().ok()?;
                let head = state.slots[state.read].as_ref().filter(|_| state.len != 0)?;
                if !admit(&head.0) {
                    return None;
                }
            }
            self.try_next()
        }

        /// 👀️ Whether the head request satisfies `test` (`false` for an empty or contended queue).
        fn head_is(&self, test: impl Fn(&KernelRequest) -> bool) -> bool {
            self.state.try_lock().is_ok_and(|state| state.len != 0 && state.slots[state.read].as_ref().is_some_and(|head| test(&head.0)))
        }
""",
)

# 4. Which requests may run between the slices of an actor preempted on `busy`'s turn.
swap(
    """    impl KernelRequest {
        fn command_credits(&self) -> (usize, usize) {""",
    """    /// ⚖️ Whether `request` may be served between two slices of a turn that is mid-flight on instance `busy`
    /// (the fairness contract `🧑‍🎨engine/🧫️fixtures/🧵️kernel-turn-fairness`): every request of ANOTHER instance
    /// that owns its outcome, and a new app; never one for the busy instance itself (its guest owns the next
    /// call), never a close or a retained-rejection drain (their maintenance invariants hold only between
    /// whole requests).
    fn kernel_request_interleavable(request: &KernelRequest, busy: u32) -> bool {
        match request {
            KernelRequest::Exchange { instance, .. } | KernelRequest::AdvanceRetained { instance, .. } | KernelRequest::AdvanceProductReplay { instance } | KernelRequest::ExchangeCommands { instance, .. } => *instance != busy,
            KernelRequest::MountProductReplay { owner } | KernelRequest::RetireProductReplay { owner } => owner.instance != busy,
            KernelRequest::CreateApp { .. } | KernelRequest::AcknowledgeJobProgress { .. } => true,
            KernelRequest::DestroyApp { .. } | KernelRequest::CloseRealm { .. } | KernelRequest::RetireProductReplayRefusal { .. } | KernelRequest::CloseRejectedCommandBuild { .. } | KernelRequest::CloseRejectedEvents { .. } => false,
        }
    }

    impl KernelRequest {
        fn command_credits(&self) -> (usize, usize) {""",
)

# 5. Pool state: the requests being served (innermost last) and whether a slice gap is being served.
swap(
    """        realm_progress_close_started: bool,
        semantic_close_document_lane: bool,
""",
    """        realm_progress_close_started: bool,
        semantic_close_document_lane: bool,
        /// 🎛️ The response slots of the requests being served, innermost last, each with whether its owner
        /// may cancel it mid-turn ([`Self::turn_cancelled`]).
        serving: Vec<(Arc<ResponseSlot>, bool)>,
        /// ⚖️ True while a request runs in another actor's slice gap ([`Self::yield_slice`]): a gap never nests.
        interleaving: bool,
""",
    )
swap(
    """                realm_progress_close_started: false,
                semantic_close_document_lane: false,
            }
        }
""",
    """                realm_progress_close_started: false,
                semantic_close_document_lane: false,
                serving: Vec::with_capacity(2),
                interleaving: false,
            }
        }
""",
)

# 6. The deadline no longer reaches a preempted guest: drop it from the single-turn drivers.
swap("                let first = self.run_turn_once(actor, instance, batch, deadline).await?;", "                let first = self.run_turn_once(actor, instance, batch).await?;")
swap("                let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;", "                let (later, next) = self.run_turn_once(actor, instance, events).await?;")
swap(
    "        async fn run_turn_once(&mut self, actor: ActorId, instance: u32, events: Vec<Event>, deadline: std::time::Instant) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {",
    "        async fn run_turn_once(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {",
)
swap("            self.dispatch_turn(actor, instance, envelopes, replay_start_index, deadline).await\n", "            self.dispatch_turn(actor, instance, envelopes, replay_start_index).await\n")
swap(
    "        async fn run_reserved_job_once(&mut self, actor: ActorId, instance: u32, deadline: std::time::Instant) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {",
    "        async fn run_reserved_job_once(&mut self, actor: ActorId, instance: u32) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {",
)
swap("            self.dispatch_turn(actor, instance, envelopes, None, deadline).await\n", "            self.dispatch_turn(actor, instance, envelopes, None).await\n")
swap(
    "        async fn dispatch_turn(&mut self, actor: ActorId, instance: u32, envelopes: Vec<Envelope>, replay_start_index: Option<usize>, deadline: std::time::Instant) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {",
    "        async fn dispatch_turn(&mut self, actor: ActorId, instance: u32, envelopes: Vec<Envelope>, replay_start_index: Option<usize>) -> Result<(ExchangeOutcome, Option<Vec<Event>>), String> {",
)
swap(
    """        /// here with an `Event::Wake` envelope (the shard's "resume with no events"), so the tick loop
        /// keeps granting it until its turn returns; `deadline` bounds how long that may take.""",
    """        /// here with an `Event::Wake` envelope (the shard's "resume with no events"), so the tick loop
        /// keeps granting it until its turn returns — after each of OUR actor's slices the request at the
        /// head of the queue may run first ([`Self::yield_slice`]), and only an explicit cancel ends the turn
        /// ([`Self::turn_cancelled`]).""",
)

# 7. The preempted branch: cancel → retire; else yield the slice, then resume.
swap(
    """                        ShardOutcome::Preempted { actor: reported } => {
                            if std::time::Instant::now() >= deadline {
                                return Err(format!("kernel: actor {reported} stayed preempted past its {RUN_TURN_SETTLE_BUDGET:?} turn budget"));
                            }
""",
    """                        ShardOutcome::Preempted { actor: reported } => {
                            if reported == actor.0 {
                                if self.turn_cancelled() {
                                    return Err(self.retire_cancelled_turn(actor).await);
                                }
                                self.yield_slice(actor, instance).await;
                            }
""",
)

# 8. Reserved tool jobs: no wall-clock kill either; they yield between steps and end on an explicit cancel.
swap(
    """        async fn settle_reserved_jobs(&mut self, actor: ActorId, instance: u32, outcome: &mut ExchangeOutcome) -> Result<(), String> {
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
""",
    """        async fn settle_reserved_jobs(&mut self, actor: ActorId, instance: u32, outcome: &mut ExchangeOutcome) -> Result<(), String> {
            while self.reserved_jobs.iter().flatten().any(|job| job.actor == actor) {
                if self.turn_cancelled() {
                    return Err(self.retire_cancelled_turn(actor).await);
                }
                let first = self.run_reserved_job_once(actor, instance).await?;
                let later = self.settle_turn(actor, instance, first, std::time::Instant::now() + RUN_TURN_SETTLE_BUDGET).await?;
                outcome.absorb(later)?;
                if !self.reserved_jobs.iter().flatten().any(|job| job.actor == actor && job.step.is_none()) {
                    self.yield_slice(actor, instance).await;
                }
            }
            Ok(())
        }

        /// 🛑️ Whether the turn being driven must end now: its requester cancelled it (a cancellable request whose
        /// response slot was abandoned — a document open's create) or the realm is closing (a realm close waits
        /// at the head of the queue). Nothing else ends a turn: a slow guest is never killed by wall-clock time.
        fn turn_cancelled(&self) -> bool {
            self.serving.last().is_some_and(|(slot, cancellable)| *cancellable && slot.abandoned()) || self.request_queue.head_is(|request| matches!(request, KernelRequest::CloseRealm { .. }))
        }

        /// ✂️ Retires an actor whose turn was cancelled mid-call: a mid-flight guest can never be handed back,
        /// so its shard instance is dropped, the scheduler stops granting it, and every host record of it goes.
        /// Answers the error its request reports.
        async fn retire_cancelled_turn(&mut self, actor: ActorId) -> String {
            let _ = self.runtime.kernel_mut().suspend(actor, None).await;
            self.runtime.unregister(actor).await;
            self.instances.retain(|_, instance_actor| *instance_actor != actor);
            for route in self.replay_routes.iter_mut().filter(|route| route.is_some_and(|route| route.actor == actor)) {
                *route = None;
            }
            for job in self.reserved_jobs.iter_mut().filter(|job| job.is_some_and(|job| job.actor == actor)) {
                *job = None;
            }
            self.begin_fault_close(actor);
            format!("kernel: actor {}'s turn was cancelled by its owner", actor.0)
        }

        /// ⚖️ Gives the other actors one request between two slices of `actor`'s turn on instance `busy` (the
        /// fairness contract's `requestsBetweenSlices`): the head of the queue runs when it may
        /// ([`kernel_request_interleavable`]) while `actor` is suspended in the scheduler, so no grant of that
        /// request can reach the mid-flight guest; then `actor` is resumed. A request running in a slice gap
        /// never opens another gap.
        async fn yield_slice(&mut self, actor: ActorId, busy: u32) {
            if self.interleaving || !self.request_queue.head_is(|request| kernel_request_interleavable(request, busy)) {
                return;
            }
            let suspended = self.runtime.kernel_mut().suspend(actor, None).await.is_ok();
            self.interleaving = true;
            for _ in 0..TURN_REQUESTS_BETWEEN_SLICES {
                let Some((request, slot)) = self.request_queue.try_next_if(|request| kernel_request_interleavable(request, busy)) else { break };
                if let Some(outcome) = self.serve(request, &slot).await {
                    slot.deliver(outcome);
                }
            }
            self.interleaving = false;
            if suspended {
                let _ = self.runtime.kernel_mut().resume(actor).await;
            }
        }
""",
)

# 9. One place serves a request — the loop and a slice gap alike.
LOOP_OLD_HEAD = """            let (request, slot) = match ready {
                Some(ready) => ready,
                None => queue.next().await,
            };
            let outcome = match request {
                KernelRequest::CreateApp { owner } => {
                    let (wasm_path, plugin_id, app_id, artifact_schema) = owner.into_parts();
                    KernelOutcome::Created(state.create_app(wasm_path, plugin_id, app_id, artifact_schema).await)
                }
"""
start = text.index(LOOP_OLD_HEAD)
end_marker = """            };
            slot.deliver(outcome);
        }
    }
"""
end = text.index(end_marker, start) + len(end_marker)
body = text[start + len(LOOP_OLD_HEAD):end - len(end_marker)]
assert text.count(LOOP_OLD_HEAD) == 1 and text.count(end_marker) == 1
arms = body.replace("state.", "self.").replace("continue;", "return None;")
serve = (
    """    impl KernelPoolState {
        /// 🎛️ Serves one request and answers the outcome its slot receives — `None` for a request that owns its own
        /// completion (a close step, an acknowledgement, a retained rejection drain). The loop and a slice gap
        /// ([`Self::yield_slice`]) both serve through here; the returned future is boxed because a request served
        /// in a gap is itself served by a turn this future drives.
        fn serve<'a>(&'a mut self, request: KernelRequest, slot: &Arc<ResponseSlot>) -> Pin<Box<dyn Future<Output = Option<KernelOutcome>> + Send + 'a>> {
            let cancellable = matches!(request, KernelRequest::CreateApp { .. });
            self.serving.push((slot.clone(), cancellable));
            Box::pin(async move {
                let outcome = self.serve_request(request).await;
                self.serving.pop();
                outcome
            })
        }

        async fn serve_request(&mut self, request: KernelRequest) -> Option<KernelOutcome> {
            Some(match request {
                KernelRequest::CreateApp { owner } => {
                    let (wasm_path, plugin_id, app_id, artifact_schema) = owner.into_parts();
                    KernelOutcome::Created(self.create_app(wasm_path, plugin_id, app_id, artifact_schema).await)
                }
"""
    + arms
    + """            })
        }
    }
"""
)
loop_tail = """            let (request, slot) = match ready {
                Some(ready) => ready,
                None => queue.next().await,
            };
            if let Some(outcome) = state.serve(request, &slot).await {
                slot.deliver(outcome);
            }
        }
    }

"""
text = text[:start] + loop_tail + serve + text[end:]
RENDERER.write_text(text)
print("fair kernel turns applied")
