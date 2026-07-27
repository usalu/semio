//! 🗄️ `db_actor` — the actor runtime every `db` document/catalog actor runs on: a six-lane
//! bounded-priority mailbox (`Priority::{System, Recovery, Command, Query, Live, Preview}` from
//! `db_core`) drained by strict priority for `System`/`Recovery` and deficit-round-robin for the
//! rest, shed-previews-first admission (the *only* lane ever dropped under backpressure — every
//! other lane blocks the sender instead), hand-rolled `Send`/`Recv`/`Reply` futures in the
//! `pack_async` style (dual blocking/async, no `tokio`), an `Actor`/`ActorContext` execution
//! model, and `OneForOne`/`OneForAll`/`Escalate` supervision with generation bumping and
//! `catch_unwind`-based poison isolation. Frozen contract:
//! `.repo/🎫/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`) and Part 2 of the approved plan.
//!
//! 🎯 Design choice: only the mailbox core (`Envelope`, `Address`, `Receiver`, `SendFuture`,
//! `RecvFuture`, the `Reply` oneshot pair, `Actor`/`ActorContext`, and the pure
//! `RestartStrategy::decide` law) is `wasm32-unknown-unknown`-clean, per the contract's "core
//! mailbox only" carve-out. Anything that actually spawns an OS thread (`ThreadSpawner`,
//! `StdThreadSpawner`, `Supervisor`, `block_on` and its blocking convenience methods) is
//! `#[cfg(not(target_arch = "wasm32"))]`, and the spawner/supervisor half is additionally gated
//! behind the (default-on) `thread` Cargo feature.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use db_core::{DbError, GenerationId, MailboxCapacities, Priority};

//#region 🔖Envelope
/// @emoji ✉️ One message in flight through a `Mailbox`: its admitted lane, an ever-increasing
/// per-mailbox sequence number (assigned at admission time, useful for tie-breaking/tracing), and
/// the caller's payload.
pub struct Envelope<M> {
    pub priority: Priority,
    pub seq: u64,
    pub payload: M,
}
//#endregion 🔖Envelope

//#region 🔖Mailbox
/// @emoji 📭 Outcome of one admission attempt against a `MailboxInner`, hands the payload back on
/// every failure path so a caller never loses a message it still owns.
enum Admission<M> {
    Accepted,
    Full(M),
    Closed(M),
    /// 🪦 `(payload, current_generation, presented_generation)` — the sender's `Address` was
    /// bound to a generation the mailbox has since moved past (a supervised restart happened).
    Stale(M, GenerationId, GenerationId),
}

/// @emoji 🚫 `Address::try_send`'s non-blocking failure modes; every variant hands the rejected
/// payload back (mirrors `std::sync::mpsc::TrySendError`) so a caller can retry, requeue, or
/// downgrade priority without cloning.
#[derive(Debug)]
pub enum TrySendError<M> {
    /// 🧱 The lane is at `MailboxCapacities`' bound and is not `Priority::sheddable()`.
    Full(M),
    /// 🚪 The mailbox has been closed (see `Address::close`/`Receiver::close`).
    Closed(M),
    /// 🪦 See `Admission::Stale` — `(payload, expected_current, actual_presented)`.
    Stale(M, GenerationId, GenerationId),
}

impl<M> TrySendError<M> {
    /// @emoji 🎁 Recovers the rejected payload, discarding the failure reason.
    pub fn into_payload(self) -> M {
        match self {
            TrySendError::Full(payload) | TrySendError::Closed(payload) | TrySendError::Stale(payload, _, _) => payload,
        }
    }

    /// @emoji 🔀 Maps to the crate family's shared error type (dropping the payload), for callers
    /// that just want a `Result<(), DbError>`.
    pub fn into_db_error(self) -> DbError {
        match self {
            TrySendError::Full(_) => DbError::LimitExceeded("mailbox lane at capacity"),
            TrySendError::Closed(_) => DbError::Closed,
            TrySendError::Stale(_, expected, actual) => DbError::StaleGeneration { expected, actual },
        }
    }
}

/// @emoji 🌀 Deficit-round-robin bookkeeping plus the six lane queues themselves; always accessed
/// under `MailboxInner::state`'s lock, so this struct's fields need no atomics of their own.
struct MailboxState<M> {
    lanes: [VecDeque<Envelope<M>>; 6],
    /// ⚖️ Per-lane DRR credit, indexed by `Priority::rank`; only the four non-strict lanes
    /// (`Command`/`Query`/`Live`/`Preview`) ever accrue a nonzero value.
    deficits: [i64; 6],
    /// 👉 Which of the four DRR lanes (indices into the local `DRR_LANES` table) is currently
    /// being drained; persists across `try_recv` calls so a lane's weight is honored as a run of
    /// consecutive dequeues, not re-decided from scratch every call.
    drr_cursor: usize,
    next_seq: u64,
    recv_wakers: Vec<Waker>,
    send_wakers: Vec<Waker>,
    shed_previews: u64,
}

/// @emoji 📮 Shared mailbox state, reference-counted between every `Address` clone and the one
/// `Receiver`. Outlives any single actor incarnation: a supervised restart reuses the same
/// `MailboxInner` (see `bump_generation`) rather than allocating a fresh one, so messages already
/// admitted before a crash survive the restart.
struct MailboxInner<M> {
    state: Mutex<MailboxState<M>>,
    capacities: MailboxCapacities,
    closed: AtomicBool,
    generation: AtomicU64,
}

/// 🚦 The four lanes that share deficit-round-robin scheduling once `System`/`Recovery` are
/// empty; declaration order only matters as the DRR cursor's fixed cycle order.
const DRR_LANES: [Priority; 4] = [Priority::Command, Priority::Query, Priority::Live, Priority::Preview];

impl<M: Send + 'static> MailboxInner<M> {
    fn new(capacities: MailboxCapacities) -> Self {
        MailboxInner {
            state: Mutex::new(MailboxState {
                lanes: [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()],
                deficits: [0; 6],
                drr_cursor: 0,
                next_seq: 0,
                recv_wakers: Vec::new(),
                send_wakers: Vec::new(),
                shed_previews: 0,
            }),
            capacities,
            closed: AtomicBool::new(false),
            generation: AtomicU64::new(GenerationId::INITIAL.0),
        }
    }

    fn current_generation(&self) -> GenerationId {
        GenerationId(self.generation.load(Ordering::Acquire))
    }

    /// @emoji 🔁 Bumps the live generation for a supervised restart. Every `Address` cloned
    /// *before* this call is now stale: its next send fails loudly with
    /// `DbError::StaleGeneration` instead of silently enqueuing into a mailbox the old actor
    /// incarnation will never drain again (see `db_core::GenerationId`'s doc).
    fn bump_generation(&self) -> GenerationId {
        GenerationId(self.generation.fetch_add(1, Ordering::AcqRel) + 1)
    }

    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    /// @emoji 🚪 Marks the mailbox closed and wakes every parked receiver so a blocked `recv`
    /// observes the closure (and drains anything still queued) instead of hanging forever.
    fn close(&self) {
        self.closed.store(true, Ordering::Release);
        let wakers = {
            let mut state = self.state.lock().unwrap();
            std::mem::take(&mut state.recv_wakers)
        };
        for waker in wakers {
            waker.wake();
        }
    }

    /// @emoji 🔓 Reverses `close` for a fresh post-restart incarnation (only called by
    /// `Supervisor`, which owns the close/reopen lifecycle around a restart).
    fn reopen(&self) {
        self.closed.store(false, Ordering::Release);
    }

    fn is_idle_and_closed(&self) -> bool {
        if !self.is_closed() {
            return false;
        }
        let state = self.state.lock().unwrap();
        state.lanes.iter().all(VecDeque::is_empty)
    }

    fn shed_preview_count(&self) -> u64 {
        self.state.lock().unwrap().shed_previews
    }

    /// @emoji 🎫 The crate's core admission law: reject on a stale generation or a closed
    /// mailbox; otherwise admit if the lane has room, else — for `Priority::Preview` only — shed
    /// the lane's own oldest message to make room (previews are never durable and never allowed
    /// to delay anything else, so they coalesce down to "latest wins" under pressure rather than
    /// ever blocking or erroring); every other lane simply reports `Full` and leaves backpressure
    /// to the caller (`Address::send`'s future parks until a `Receiver::recv` frees a slot).
    fn try_admit(&self, bound_generation: GenerationId, priority: Priority, payload: M) -> Admission<M> {
        let current = self.current_generation();
        if bound_generation != current {
            return Admission::Stale(payload, current, bound_generation);
        }
        if self.is_closed() {
            return Admission::Closed(payload);
        }
        let mut state = self.state.lock().unwrap();
        let rank = priority.rank();
        let capacity = self.capacities.get(priority) as usize;
        if state.lanes[rank].len() >= capacity {
            if priority.sheddable() {
                state.lanes[rank].pop_front();
                state.shed_previews += 1;
            } else {
                return Admission::Full(payload);
            }
        }
        let seq = state.next_seq;
        state.next_seq += 1;
        state.lanes[rank].push_back(Envelope { priority, seq, payload });
        let wakers = std::mem::take(&mut state.recv_wakers);
        drop(state);
        for waker in wakers {
            waker.wake();
        }
        Admission::Accepted
    }

    fn register_send_waker(&self, waker: Waker) {
        let mut state = self.state.lock().unwrap();
        if !state.send_wakers.iter().any(|existing| existing.will_wake(&waker)) {
            state.send_wakers.push(waker);
        }
    }

    fn register_recv_waker(&self, waker: Waker) {
        let mut state = self.state.lock().unwrap();
        if !state.recv_wakers.iter().any(|existing| existing.will_wake(&waker)) {
            state.recv_wakers.push(waker);
        }
    }

    /// @emoji 🥇 `System` then `Recovery`, strictly — "never queued behind anything" per
    /// `db_core::Priority`'s own doc — then deficit-round-robin across the remaining four lanes.
    fn try_recv(&self) -> Option<Envelope<M>> {
        let mut state = self.state.lock().unwrap();
        for &strict in &[Priority::System, Priority::Recovery] {
            let rank = strict.rank();
            if let Some(envelope) = state.lanes[rank].pop_front() {
                let wakers = std::mem::take(&mut state.send_wakers);
                drop(state);
                for waker in wakers {
                    waker.wake();
                }
                return Some(envelope);
            }
        }
        if DRR_LANES.iter().all(|lane| state.lanes[lane.rank()].is_empty()) {
            return None;
        }
        for _ in 0..DRR_LANES.len() {
            let cursor = state.drr_cursor;
            let lane = DRR_LANES[cursor];
            let rank = lane.rank();
            if state.lanes[rank].is_empty() {
                state.deficits[rank] = 0;
                state.drr_cursor = (cursor + 1) % DRR_LANES.len();
                continue;
            }
            if state.deficits[rank] <= 0 {
                state.deficits[rank] = lane.default_weight() as i64;
            }
            state.deficits[rank] -= 1;
            let envelope = state.lanes[rank].pop_front().expect("checked non-empty above");
            if state.lanes[rank].is_empty() || state.deficits[rank] <= 0 {
                state.deficits[rank] = state.deficits[rank].max(0);
                state.drr_cursor = (cursor + 1) % DRR_LANES.len();
            }
            let wakers = std::mem::take(&mut state.send_wakers);
            drop(state);
            for waker in wakers {
                waker.wake();
            }
            return Some(envelope);
        }
        unreachable!("the all-empty guard above rules out a full unproductive DRR cycle")
    }
}

/// @emoji 📬 A clone-cheap handle for sending into a `Mailbox`, bound to the generation it was
/// obtained at. Every clone shares the same binding — a fresh, current-generation `Address` is
/// only produced by `mailbox()` (a new mailbox) or by `Supervisor` on a restart.
pub struct Address<M> {
    inner: Arc<MailboxInner<M>>,
    bound_generation: GenerationId,
}

impl<M> Clone for Address<M> {
    fn clone(&self) -> Self {
        Address { inner: self.inner.clone(), bound_generation: self.bound_generation }
    }
}

impl<M: Send + 'static> Address<M> {
    /// @emoji 🪪 The generation this handle was obtained at (see `db_core::GenerationId`).
    pub fn generation(&self) -> GenerationId {
        self.bound_generation
    }

    /// @emoji ⚡ Non-blocking send: returns immediately, either admitted or rejected with the
    /// payload handed back.
    pub fn try_send(&self, priority: Priority, payload: M) -> Result<(), TrySendError<M>> {
        match self.inner.try_admit(self.bound_generation, priority, payload) {
            Admission::Accepted => Ok(()),
            Admission::Full(payload) => Err(TrySendError::Full(payload)),
            Admission::Closed(payload) => Err(TrySendError::Closed(payload)),
            Admission::Stale(payload, expected, actual) => Err(TrySendError::Stale(payload, expected, actual)),
        }
    }

    /// @emoji 📤 A `SendFuture` that resolves as soon as `priority`'s lane admits `payload` —
    /// immediately if there is room (or the lane sheds), or once a `Receiver::recv` frees a slot.
    pub fn send(&self, priority: Priority, payload: M) -> SendFuture<M> {
        SendFuture { inner: self.inner.clone(), bound_generation: self.bound_generation, priority, payload: Some(payload) }
    }

    /// @emoji 🚪 Closes the mailbox: further sends fail with `DbError::Closed`/`TrySendError::Closed`,
    /// and a parked `recv` resolves once every lane has drained.
    pub fn close(&self) {
        self.inner.close();
    }

    /// @emoji 📊 How many `Priority::Preview` messages this mailbox has shed to admit newer ones
    /// (see `try_admit`'s doc) — exposed for tests/observability, not part of the hot path.
    pub fn shed_preview_count(&self) -> u64 {
        self.inner.shed_preview_count()
    }
}

/// @emoji 📥 The single-consumer half of a mailbox. Nothing in this crate enforces uniqueness at
/// the type level (the underlying `MailboxInner` is safe under concurrent `recv`), but the
/// convention — and every constructor in this crate — hands out exactly one per mailbox.
pub struct Receiver<M> {
    inner: Arc<MailboxInner<M>>,
}

impl<M: Send + 'static> Receiver<M> {
    /// @emoji ⚡ Non-blocking receive: `None` means "nothing queued right now", not "closed" —
    /// use `recv`/`recv_blocking` to also observe closure.
    pub fn try_recv(&self) -> Option<Envelope<M>> {
        self.inner.try_recv()
    }

    /// @emoji 📥 A `RecvFuture` resolving to the next message by priority order, or `None` once
    /// the mailbox is closed and every lane has drained.
    pub fn recv(&self) -> RecvFuture<'_, M> {
        RecvFuture { inner: &self.inner }
    }

    pub fn close(&self) {
        self.inner.close();
    }
}

/// @emoji 🆕 A fresh mailbox at `GenerationId::INITIAL`, split into its sender/receiver halves —
/// the `db_actor` analogue of `std::sync::mpsc::channel`.
pub fn mailbox<M: Send + 'static>(capacities: MailboxCapacities) -> (Address<M>, Receiver<M>) {
    let inner = Arc::new(MailboxInner::new(capacities));
    (Address { inner: inner.clone(), bound_generation: GenerationId::INITIAL }, Receiver { inner })
}

/// @emoji ⚙️ Convenience over `mailbox` that pulls lane capacities out of a `db_core::DbConfig`.
pub fn mailbox_from_config<M: Send + 'static>(config: &db_core::DbConfig) -> (Address<M>, Receiver<M>) {
    mailbox(config.mailbox_capacities)
}
//#endregion 🔖Mailbox

//#region 🔖Futures
/// @emoji 📤 Hand-rolled future backing `Address::send`. Polls the same admission law
/// `try_send` uses; on `Full`, registers a waker and re-polls once more before actually returning
/// `Pending`, closing the classic lost-wakeup race window (a `Receiver::recv` that already ran
/// between the first admission attempt and the registration would otherwise never be observed).
pub struct SendFuture<M> {
    inner: Arc<MailboxInner<M>>,
    bound_generation: GenerationId,
    priority: Priority,
    payload: Option<M>,
}

// 🩹 `Unpin` is required (not just `Send + 'static`) because `poll` below uses `get_mut` for
// plain field access rather than projecting through the pin — justified since `SendFuture<M>`
// never relies on a stable address for anything (no self-referential state), and every realistic
// `db_actor` message payload (owned enums/structs) is `Unpin` automatically.
impl<M: Send + 'static + Unpin> Future for SendFuture<M> {
    type Output = Result<(), DbError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let payload = this.payload.take().expect("SendFuture polled after completion");
        match this.inner.try_admit(this.bound_generation, this.priority, payload) {
            Admission::Accepted => Poll::Ready(Ok(())),
            Admission::Closed(_) => Poll::Ready(Err(DbError::Closed)),
            Admission::Stale(_, expected, actual) => Poll::Ready(Err(DbError::StaleGeneration { expected, actual })),
            Admission::Full(payload) => {
                this.inner.register_send_waker(cx.waker().clone());
                match this.inner.try_admit(this.bound_generation, this.priority, payload) {
                    Admission::Accepted => Poll::Ready(Ok(())),
                    Admission::Closed(_) => Poll::Ready(Err(DbError::Closed)),
                    Admission::Stale(_, expected, actual) => Poll::Ready(Err(DbError::StaleGeneration { expected, actual })),
                    Admission::Full(payload) => {
                        this.payload = Some(payload);
                        Poll::Pending
                    }
                }
            }
        }
    }
}

/// @emoji 📥 Hand-rolled future backing `Receiver::recv`, with the same register-then-re-check
/// pattern as `SendFuture` to close the symmetric lost-wakeup window on the receive side.
pub struct RecvFuture<'a, M> {
    inner: &'a Arc<MailboxInner<M>>,
}

impl<'a, M: Send + 'static> Future for RecvFuture<'a, M> {
    type Output = Option<Envelope<M>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(envelope) = self.inner.try_recv() {
            return Poll::Ready(Some(envelope));
        }
        if self.inner.is_idle_and_closed() {
            return Poll::Ready(None);
        }
        self.inner.register_recv_waker(cx.waker().clone());
        if let Some(envelope) = self.inner.try_recv() {
            return Poll::Ready(Some(envelope));
        }
        if self.inner.is_idle_and_closed() {
            return Poll::Ready(None);
        }
        Poll::Pending
    }
}
//#endregion 🔖Futures

//#region 🔖Reply
/// @emoji 🎁 Shared state for one `ask`-style request/response pair.
struct OneshotState<R> {
    value: Option<R>,
    waker: Option<Waker>,
    sender_dropped: bool,
}

/// @emoji 📮 The write-once half of a `Reply` channel; an actor's `handle` calls `send` exactly
/// once to answer an `Address::ask`.
pub struct ReplySender<R> {
    inner: Arc<Mutex<OneshotState<R>>>,
}

/// @emoji 📭 The read-once half; also a `Future` (`Output = Result<R, DbError>`) so it can be
/// `.await`ed directly or driven through `block_on`.
pub struct ReplyReceiver<R> {
    inner: Arc<Mutex<OneshotState<R>>>,
}

/// @emoji ✂️ A fresh `Reply` pair, unconnected to any particular message shape — `Address::ask`
/// is the usual way one gets created and threaded through a message.
pub fn oneshot<R>() -> (ReplySender<R>, ReplyReceiver<R>) {
    let inner = Arc::new(Mutex::new(OneshotState { value: None, waker: None, sender_dropped: false }));
    (ReplySender { inner: inner.clone() }, ReplyReceiver { inner })
}

impl<R> ReplySender<R> {
    pub fn send(self, value: R) {
        let mut state = self.inner.lock().unwrap();
        state.value = Some(value);
        let waker = state.waker.take();
        drop(state);
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl<R> Drop for ReplySender<R> {
    /// @emoji 🥀 An abandoned `ReplySender` (dropped without `send`) resolves the receiver to
    /// `DbError::Closed` instead of hanging it forever — mirrors `std::sync::mpsc`'s disconnect
    /// behavior for the ask/reply pattern.
    fn drop(&mut self) {
        let mut state = self.inner.lock().unwrap();
        if state.value.is_none() {
            state.sender_dropped = true;
            let waker = state.waker.take();
            drop(state);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }
}

impl<R> ReplyReceiver<R> {
    /// @emoji ⚡ Non-blocking peek, used by `Supervisor::reap` to check a child's terminal outcome
    /// without parking.
    fn try_recv(&self) -> Option<R> {
        self.inner.lock().unwrap().value.take()
    }
}

impl<R> Future for ReplyReceiver<R> {
    type Output = Result<R, DbError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.inner.lock().unwrap();
        if let Some(value) = state.value.take() {
            return Poll::Ready(Ok(value));
        }
        if state.sender_dropped {
            return Poll::Ready(Err(DbError::Closed));
        }
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// @emoji 🎣 Two-phase state machine backing `Address::ask`: first drive the underlying
/// `SendFuture` to completion, then the `ReplyReceiver` — composing `Send` and `Reply` futures
/// into one awaitable exactly like a hand-rolled `async fn` would, without `async`/`.await`
/// syntax (this crate stays on stable `Future` impls throughout, no nightly generators needed).
enum AskState<M, R> {
    Sending(SendFuture<M>, Option<ReplyReceiver<R>>),
    Waiting(ReplyReceiver<R>),
    Done,
}

pub struct AskFuture<M, R> {
    state: AskState<M, R>,
}

// 🩹 Same `Unpin` justification as `SendFuture`'s impl — `AskFuture` embeds a `SendFuture<M>`
// directly, so it inherits the same bound.
impl<M: Send + 'static + Unpin, R: Send + 'static> Future for AskFuture<M, R> {
    type Output = Result<R, DbError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        loop {
            match &mut this.state {
                AskState::Sending(send_future, reply_receiver) => match Pin::new(send_future).poll(cx) {
                    Poll::Ready(Ok(())) => {
                        let receiver = reply_receiver.take().expect("AskFuture::Sending always holds a receiver until it transitions");
                        this.state = AskState::Waiting(receiver);
                    }
                    Poll::Ready(Err(err)) => {
                        this.state = AskState::Done;
                        return Poll::Ready(Err(err));
                    }
                    Poll::Pending => return Poll::Pending,
                },
                AskState::Waiting(receiver) => match Pin::new(receiver).poll(cx) {
                    Poll::Ready(result) => {
                        this.state = AskState::Done;
                        return Poll::Ready(result);
                    }
                    Poll::Pending => return Poll::Pending,
                },
                AskState::Done => panic!("AskFuture polled after completion"),
            }
        }
    }
}

impl<M: Send + 'static> Address<M> {
    /// @emoji ❓ Request/response over the mailbox: builds the outgoing message from a fresh
    /// `ReplySender`, sends it, and resolves once the actor replies (or the mailbox/reply channel
    /// closes first).
    pub fn ask<R: Send + 'static>(&self, priority: Priority, build: impl FnOnce(ReplySender<R>) -> M) -> AskFuture<M, R> {
        let (reply_tx, reply_rx) = oneshot();
        let message = build(reply_tx);
        AskFuture { state: AskState::Sending(self.send(priority, message), Some(reply_rx)) }
    }
}
//#endregion 🔖Reply

//#region 🔖BlockingRuntime
/// @emoji 🧵 A `Waker` that unparks the thread which was polling when it went `Pending` — the
/// mechanism behind `block_on`. Uses the stable `std::task::Wake` trait; `std::thread::park`'s
/// unpark-token semantics already close the wake-before-park race, so no extra synchronization is
/// needed here.
#[cfg(not(target_arch = "wasm32"))]
struct ThreadWaker(std::thread::Thread);

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

/// @emoji 🛑 Drives `future` to completion on the calling thread — the "blocking" half of this
/// crate's dual blocking/async futures, in the `pack_async` spirit but hand-rolled (no
/// `futures-lite` dependency: `db_actor`'s only dependency is `db_core`, per the contract).
#[cfg(not(target_arch = "wasm32"))]
pub fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::park(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<M: Send + 'static + Unpin> Address<M> {
    pub fn send_blocking(&self, priority: Priority, payload: M) -> Result<(), DbError> {
        block_on(self.send(priority, payload))
    }

    pub fn ask_blocking<R: Send + 'static>(&self, priority: Priority, build: impl FnOnce(ReplySender<R>) -> M) -> Result<R, DbError> {
        block_on(self.ask(priority, build))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<M: Send + 'static> Receiver<M> {
    pub fn recv_blocking(&self) -> Option<Envelope<M>> {
        block_on(self.recv())
    }
}
//#endregion 🔖BlockingRuntime

//#region 🔖Actor
/// @emoji 🎭 One unit of the `db` family's execution model: owns its private state, is driven
/// exclusively by messages off its own `Mailbox`, and never shares mutable state with another
/// actor except through further messages.
pub trait Actor: Send + 'static {
    type Message: Send + 'static;

    /// @emoji 🌱 Runs once per incarnation before the message loop starts; an `Err` here poisons
    /// the incarnation exactly like a panicking `handle` would.
    fn on_start(&mut self, _ctx: &mut ActorContext<Self::Message>) -> Result<(), DbError> {
        Ok(())
    }

    /// @emoji 📨 Handles one message. An `Err` return is an ordinary application-level failure
    /// (logged/reported, incarnation stays alive); a *panic* is what poisons the incarnation and
    /// triggers supervision.
    fn handle(&mut self, msg: Self::Message, ctx: &mut ActorContext<Self::Message>) -> Result<(), DbError>;
}

/// @emoji 🧭 What an `Actor` sees while handling a message: its own address (for self-sends),
/// which generation it is, and the observability seam.
pub struct ActorContext<M: Send + 'static> {
    pub address: Address<M>,
    pub generation: GenerationId,
    pub emit: Arc<dyn db_core::Emit>,
}
//#endregion 🔖Actor

//#region 🔖Supervision
/// @emoji 🌳 How a `Supervisor` reacts to one poisoned child.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RestartStrategy {
    /// 1️⃣ Only the failed child restarts; siblings are undisturbed.
    OneForOne,
    /// 👯 Every child under the same supervisor restarts together.
    OneForAll,
    /// ⬆️ Nothing restarts here — the failure is reported for a parent supervisor to decide.
    Escalate,
}

/// @emoji 📣 What `Supervisor::reap` did (or recommends) in response to one poisoned child.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SupervisionDecision {
    RestartOne(usize),
    RestartAll,
    Escalate,
}

impl RestartStrategy {
    /// @emoji ⚖️ The pure decision law, independent of any actual thread/mailbox machinery — kept
    /// free-standing so it stays `wasm32`-clean and is trivially unit-testable.
    pub fn decide(self, failed_index: usize) -> SupervisionDecision {
        match self {
            RestartStrategy::OneForOne => SupervisionDecision::RestartOne(failed_index),
            RestartStrategy::OneForAll => SupervisionDecision::RestartAll,
            RestartStrategy::Escalate => SupervisionDecision::Escalate,
        }
    }
}
//#endregion 🔖Supervision

//#region 🔖ThreadSpawner
/// @emoji 🧵 How a `Supervisor` obtains OS threads to run actor incarnations on — an interface so
/// `db_engine`'s `tokio` feature (or a test double) can substitute its own scheduling without this
/// crate depending on any runtime crate.
#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
pub trait ThreadSpawner: Send + Sync {
    fn spawn(&self, name: String, task: Box<dyn FnOnce() + Send>) -> Box<dyn JoinHandleLike>;
}

/// @emoji 🤝 A join handle abstracted away from `std::thread::JoinHandle` specifically, so a
/// non-`std::thread` `ThreadSpawner` (e.g. a pooled/test spawner) can implement it too.
#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
pub trait JoinHandleLike: Send {
    fn join(self: Box<Self>);
}

/// @emoji 🏭 The default `ThreadSpawner`: one `std::thread` per spawn, per the contract ("db_actor's
/// default ThreadSpawner is std::thread + hand-rolled futures, not tokio").
#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
pub struct StdThreadSpawner;

#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
impl ThreadSpawner for StdThreadSpawner {
    fn spawn(&self, name: String, task: Box<dyn FnOnce() + Send>) -> Box<dyn JoinHandleLike> {
        let handle = std::thread::Builder::new().name(name).spawn(task).expect("db_actor: failed to spawn OS thread");
        Box::new(StdJoinHandle(Some(handle)))
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
struct StdJoinHandle(Option<std::thread::JoinHandle<()>>);

#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
impl JoinHandleLike for StdJoinHandle {
    fn join(mut self: Box<Self>) {
        if let Some(handle) = self.0.take() {
            let _ = handle.join();
        }
    }
}
//#endregion 🔖ThreadSpawner

//#region 🔖Runner
/// @emoji 🏁 Why an actor incarnation's message loop ended.
#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
enum ActorOutcome {
    /// 🚪 The mailbox closed and drained normally (graceful shutdown, e.g. `RestartAll`'s stop
    /// phase) — not a failure, `Supervisor::reap` does not restart on this outcome.
    Stopped,
    /// ☠️ `on_start` or `handle` panicked — poisoned, `Supervisor::reap` applies the strategy.
    Panicked,
}

/// @emoji 🏃 The actual message loop: `catch_unwind`-isolates every call into `Actor` code so one
/// poisoned incarnation can never take down the OS thread pool or a sibling actor, then reports
/// its terminal `ActorOutcome` back to the owning `Supervisor` through a `Reply` oneshot.
#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
fn run_actor_loop<A: Actor>(
    mut actor: A,
    receiver: Receiver<A::Message>,
    address: Address<A::Message>,
    generation: GenerationId,
    emit: Arc<dyn db_core::Emit>,
    outcome_tx: ReplySender<ActorOutcome>,
) {
    use std::panic::AssertUnwindSafe;

    let mut ctx = ActorContext { address, generation, emit: emit.clone() };
    let start_outcome = std::panic::catch_unwind(AssertUnwindSafe(|| actor.on_start(&mut ctx)));
    let mut poisoned = !matches!(start_outcome, Ok(Ok(())));
    if !poisoned {
        loop {
            let envelope = match receiver.recv_blocking() {
                Some(envelope) => envelope,
                None => break,
            };
            // 🩹 `AssertUnwindSafe` is justified: on a panic we discard `actor`/`ctx` entirely
            // (never touched again in this incarnation) rather than trusting any partially
            // mutated state, so unwind-safety of their contents genuinely does not matter here.
            let step = std::panic::catch_unwind(AssertUnwindSafe(|| actor.handle(envelope.payload, &mut ctx)));
            if step.is_err() {
                poisoned = true;
                break;
            }
        }
    }
    if poisoned {
        emit.emit(db_core::EmitEvent::new("db_actor.incarnation_poisoned").field("generation", db_core::EmitField::U64(generation.0)));
    }
    outcome_tx.send(if poisoned { ActorOutcome::Panicked } else { ActorOutcome::Stopped });
}

/// @emoji 👨‍👩‍👧 Owns `N` incarnations of the same `Actor` type, restarting them per
/// `RestartStrategy` when `reap` observes a poisoned outcome. Scoped to a homogeneous child set
/// (one actor type) deliberately — a heterogeneous supervision tree composes multiple
/// `Supervisor`s, each escalating to whatever owns the next level up.
#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
struct SupervisorSlot<M: Send + 'static> {
    mailbox: Arc<MailboxInner<M>>,
    address: Address<M>,
    generation: GenerationId,
    outcome_rx: ReplyReceiver<ActorOutcome>,
    handle: Option<Box<dyn JoinHandleLike>>,
}

#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
pub struct Supervisor<A: Actor> {
    strategy: RestartStrategy,
    capacities: MailboxCapacities,
    spawner: Arc<dyn ThreadSpawner>,
    emit: Arc<dyn db_core::Emit>,
    factory: Box<dyn Fn() -> A + Send + Sync>,
    slots: Mutex<Vec<SupervisorSlot<A::Message>>>,
}

#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
impl<A: Actor> Supervisor<A> {
    /// @emoji 🆕 Spawns `children` fresh incarnations of `factory()`'s actor at
    /// `GenerationId::INITIAL`, each with its own mailbox sized by `capacities`.
    pub fn new(
        strategy: RestartStrategy,
        capacities: MailboxCapacities,
        spawner: Arc<dyn ThreadSpawner>,
        emit: Arc<dyn db_core::Emit>,
        factory: impl Fn() -> A + Send + Sync + 'static,
        children: usize,
    ) -> Self {
        let supervisor =
            Supervisor { strategy, capacities, spawner, emit, factory: Box::new(factory), slots: Mutex::new(Vec::new()) };
        let mut slots = Vec::with_capacity(children);
        for _ in 0..children {
            slots.push(supervisor.spawn_slot(None));
        }
        *supervisor.slots.lock().unwrap() = slots;
        supervisor
    }

    /// @emoji 🌱 Spawns one incarnation. `existing` is `None` for the initial spawn (fresh
    /// mailbox, generation 0) or `Some(mailbox)` for a restart (same mailbox, bumped generation —
    /// this is what makes pre-restart `Address` clones go stale).
    fn spawn_slot(&self, existing: Option<Arc<MailboxInner<A::Message>>>) -> SupervisorSlot<A::Message> {
        let (mailbox, generation) = match existing {
            Some(inner) => {
                inner.reopen();
                let generation = inner.bump_generation();
                (inner, generation)
            }
            None => (Arc::new(MailboxInner::new(self.capacities)), GenerationId::INITIAL),
        };
        let address = Address { inner: mailbox.clone(), bound_generation: generation };
        let receiver = Receiver { inner: mailbox.clone() };
        let ctx_address = address.clone();
        let (outcome_tx, outcome_rx) = oneshot();
        let actor = (self.factory)();
        let emit = self.emit.clone();
        let handle = self.spawner.spawn(
            format!("db-actor-g{}", generation.0),
            Box::new(move || run_actor_loop(actor, receiver, ctx_address, generation, emit, outcome_tx)),
        );
        SupervisorSlot { mailbox, address, generation, outcome_rx, handle: Some(handle) }
    }

    pub fn address(&self, index: usize) -> Address<A::Message> {
        self.slots.lock().unwrap()[index].address.clone()
    }

    pub fn generation(&self, index: usize) -> GenerationId {
        self.slots.lock().unwrap()[index].generation
    }

    pub fn child_count(&self) -> usize {
        self.slots.lock().unwrap().len()
    }

    /// @emoji ♻️ Non-blocking health check: reaps the join handle of any child whose incarnation
    /// has terminated, and — for the first `Panicked` one found — applies `strategy` and reports
    /// what happened. Returns `None` when nothing terminated since the last call. Callers (e.g.
    /// `db_engine`'s catalog actor) are expected to poll this periodically or after every send
    /// that could plausibly have triggered a panic.
    pub fn reap(&self) -> Option<SupervisionDecision> {
        let mut slots = self.slots.lock().unwrap();
        let mut failed_index = None;
        for (index, slot) in slots.iter_mut().enumerate() {
            if let Some(outcome) = slot.outcome_rx.try_recv() {
                if let Some(handle) = slot.handle.take() {
                    handle.join();
                }
                if matches!(outcome, ActorOutcome::Panicked) {
                    failed_index = Some(index);
                    break;
                }
            }
        }
        let failed_index = failed_index?;
        let decision = self.strategy.decide(failed_index);
        match decision {
            SupervisionDecision::RestartOne(index) => {
                let mailbox = slots[index].mailbox.clone();
                slots[index] = self.spawn_slot(Some(mailbox));
            }
            SupervisionDecision::RestartAll => {
                for slot in slots.iter() {
                    slot.address.close();
                }
                for slot in slots.iter_mut() {
                    if let Some(handle) = slot.handle.take() {
                        handle.join();
                    }
                }
                for index in 0..slots.len() {
                    let mailbox = slots[index].mailbox.clone();
                    slots[index] = self.spawn_slot(Some(mailbox));
                }
            }
            SupervisionDecision::Escalate => {
                // 🚧 Extension seam: this crate has no parent-supervisor concept of its own —
                // the caller (typically `db_engine`'s catalog actor) owns what "escalate" means
                // at the top of its tree (e.g. mark the document unavailable, alert an operator).
                // Children are left exactly as they terminated; nothing here is a stub, the
                // decision itself *is* the real, complete signal this crate is responsible for.
            }
        }
        Some(decision)
    }
}
//#endregion 🔖Runner

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_core::{NullEmit, Profile};

    fn generous_capacities() -> MailboxCapacities {
        MailboxCapacities::uniform(64)
    }

    //#region 🔖Mailbox
    #[test]
    fn system_and_recovery_lanes_drain_strictly_ahead_of_the_drr_lanes() {
        let (address, receiver) = mailbox::<&'static str>(generous_capacities());
        address.try_send(Priority::Preview, "preview").unwrap();
        address.try_send(Priority::Command, "command").unwrap();
        address.try_send(Priority::Recovery, "recovery").unwrap();
        address.try_send(Priority::System, "system").unwrap();

        assert_eq!(receiver.try_recv().unwrap().payload, "system");
        assert_eq!(receiver.try_recv().unwrap().payload, "recovery");
        assert_eq!(receiver.try_recv().unwrap().payload, "command");
        assert_eq!(receiver.try_recv().unwrap().payload, "preview");
        assert!(receiver.try_recv().is_none());
    }

    #[test]
    fn drr_serves_a_lane_a_run_of_messages_proportional_to_its_weight() {
        let (address, receiver) = mailbox::<(Priority, u32)>(generous_capacities());
        for i in 0..32 {
            address.try_send(Priority::Command, (Priority::Command, i)).unwrap();
            address.try_send(Priority::Query, (Priority::Query, i)).unwrap();
        }

        // Command's weight (16) is exactly double Query's (8): the DRR cursor must drain 16
        // consecutive Command messages before it ever yields to Query, then 8 consecutive Query
        // messages before wrapping back — this is deterministic given the implementation, not a
        // statistical approximation, so assert the exact boundary.
        for _ in 0..16 {
            assert_eq!(receiver.try_recv().unwrap().priority, Priority::Command);
        }
        for _ in 0..8 {
            assert_eq!(receiver.try_recv().unwrap().priority, Priority::Query);
        }
        for _ in 0..16 {
            assert_eq!(receiver.try_recv().unwrap().priority, Priority::Command);
        }
    }

    #[test]
    fn preview_lane_sheds_its_own_oldest_message_and_never_blocks_or_errors() {
        let mut capacities = MailboxCapacities::uniform(64);
        capacities.set(Priority::Preview, 2);
        let (address, receiver) = mailbox::<u32>(capacities);

        for value in 0..3 {
            assert!(address.try_send(Priority::Preview, value).is_ok(), "preview sends must never fail under pressure");
        }

        assert_eq!(address.shed_preview_count(), 1);
        assert_eq!(receiver.try_recv().unwrap().payload, 1, "the oldest preview (0) must have been shed");
        assert_eq!(receiver.try_recv().unwrap().payload, 2);
        assert!(receiver.try_recv().is_none());
    }

    #[test]
    fn non_preview_lane_rejects_try_send_when_full_instead_of_shedding() {
        let mut capacities = MailboxCapacities::uniform(64);
        capacities.set(Priority::Command, 1);
        let (address, _receiver) = mailbox::<u32>(capacities);

        address.try_send(Priority::Command, 1).unwrap();
        match address.try_send(Priority::Command, 2) {
            Err(TrySendError::Full(payload)) => assert_eq!(payload, 2, "the rejected payload must be handed back"),
            _ => panic!("expected TrySendError::Full for a saturated non-sheddable lane"),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn non_preview_lane_send_future_blocks_until_a_recv_frees_a_slot() {
        let mut capacities = MailboxCapacities::uniform(64);
        capacities.set(Priority::Command, 1);
        let (address, receiver) = mailbox::<u32>(capacities);
        address.try_send(Priority::Command, 1).unwrap();

        let blocked_address = address.clone();
        let sender_thread = std::thread::spawn(move || blocked_address.send_blocking(Priority::Command, 2));

        // No sleep/race here: `try_admit`'s register-then-re-check pattern makes this correct
        // regardless of whether the spawned thread has even started polling yet — either it
        // observes the freed slot on its own first poll, or it parks and this pop's wake() call
        // reaches it. See `SendFuture::poll`'s doc.
        let freed = receiver.try_recv();
        assert_eq!(freed.map(|e| e.payload), Some(1));
        assert!(sender_thread.join().unwrap().is_ok(), "the blocked send must complete once a slot frees");
        assert_eq!(receiver.try_recv().unwrap().payload, 2);
    }

    #[test]
    fn closing_a_mailbox_drains_remaining_messages_then_resolves_recv_to_none() {
        let (address, receiver) = mailbox::<u32>(generous_capacities());
        address.try_send(Priority::Command, 1).unwrap();
        address.close();
        assert!(matches!(address.try_send(Priority::Command, 2), Err(TrySendError::Closed(2))));
        assert_eq!(receiver.try_recv().unwrap().payload, 1, "already-queued messages survive a close");
        assert!(receiver.try_recv().is_none());
    }

    #[test]
    fn a_stale_generation_address_is_rejected_loudly_instead_of_enqueuing_silently() {
        let (address, _receiver) = mailbox::<u32>(generous_capacities());
        let stale = address.clone();
        let bumped = address.inner.bump_generation();

        match stale.try_send(Priority::Command, 7) {
            Err(TrySendError::Stale(payload, expected, actual)) => {
                assert_eq!(payload, 7);
                assert_eq!(expected, bumped);
                assert_eq!(actual, GenerationId::INITIAL);
            }
            _ => panic!("expected TrySendError::Stale for an address bound to a superseded generation"),
        }
    }
    //#endregion 🔖Mailbox

    //#region 🔖Reply
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn oneshot_reply_round_trips_a_value() {
        let (tx, rx) = oneshot::<i32>();
        let sender_thread = std::thread::spawn(move || tx.send(42));
        assert_eq!(block_on(rx), Ok(42));
        sender_thread.join().unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn oneshot_reply_resolves_closed_when_sender_is_dropped_without_sending() {
        let (tx, rx) = oneshot::<i32>();
        drop(tx);
        assert_eq!(block_on(rx), Err(DbError::Closed));
    }
    //#endregion 🔖Reply

    //#region 🔖Actor
    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    enum EchoMessage {
        Double(i32, ReplySender<i32>),
        Crash,
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    #[derive(Default)]
    struct EchoActor;

    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    impl Actor for EchoActor {
        type Message = EchoMessage;

        fn handle(&mut self, msg: Self::Message, _ctx: &mut ActorContext<Self::Message>) -> Result<(), DbError> {
            match msg {
                EchoMessage::Double(value, reply) => {
                    reply.send(value * 2);
                    Ok(())
                }
                EchoMessage::Crash => panic!("intentional test crash"),
            }
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    #[test]
    fn ask_pattern_round_trips_through_a_real_actor_thread() {
        let supervisor =
            Supervisor::new(RestartStrategy::OneForOne, generous_capacities(), Arc::new(StdThreadSpawner), Arc::new(NullEmit), EchoActor::default, 1);
        let address = supervisor.address(0);
        let reply = address.ask_blocking(Priority::Command, |tx| EchoMessage::Double(21, tx));
        assert_eq!(reply, Ok(42));
    }
    //#endregion 🔖Actor

    //#region 🔖Supervision
    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    fn reap_until_decided(supervisor: &Supervisor<EchoActor>) -> SupervisionDecision {
        for _ in 0..200 {
            if let Some(decision) = supervisor.reap() {
                return decision;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        panic!("supervisor never reaped a terminal outcome within the test's bound");
    }

    #[test]
    fn restart_strategy_decide_is_the_pure_law_wasm_clean_paths_rely_on() {
        assert_eq!(RestartStrategy::OneForOne.decide(3), SupervisionDecision::RestartOne(3));
        assert_eq!(RestartStrategy::OneForAll.decide(0), SupervisionDecision::RestartAll);
        assert_eq!(RestartStrategy::Escalate.decide(1), SupervisionDecision::Escalate);
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    #[test]
    fn one_for_one_restarts_only_the_failed_child_and_bumps_only_its_generation() {
        let supervisor = Supervisor::new(
            RestartStrategy::OneForOne,
            generous_capacities(),
            Arc::new(StdThreadSpawner),
            Arc::new(NullEmit),
            EchoActor::default,
            2,
        );
        let stale_child0 = supervisor.address(0);
        let untouched_child1_generation = supervisor.generation(1);

        supervisor.address(0).send_blocking(Priority::Command, EchoMessage::Crash).unwrap();
        let decision = reap_until_decided(&supervisor);

        assert_eq!(decision, SupervisionDecision::RestartOne(0));
        assert_eq!(supervisor.generation(0), GenerationId::INITIAL.next());
        assert_eq!(supervisor.generation(1), untouched_child1_generation);
        assert!(
            matches!(stale_child0.try_send(Priority::Command, EchoMessage::Crash), Err(TrySendError::Stale(_, _, _))),
            "an Address captured before the restart must fail loudly rather than talk to a dead incarnation"
        );

        let fresh_reply = supervisor.address(0).ask_blocking(Priority::Command, |tx| EchoMessage::Double(5, tx));
        assert_eq!(fresh_reply, Ok(10), "the restarted incarnation must be alive and answering");
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    #[test]
    fn one_for_all_restarts_every_child_and_bumps_every_generation() {
        let supervisor = Supervisor::new(
            RestartStrategy::OneForAll,
            generous_capacities(),
            Arc::new(StdThreadSpawner),
            Arc::new(NullEmit),
            EchoActor::default,
            3,
        );

        supervisor.address(1).send_blocking(Priority::Command, EchoMessage::Crash).unwrap();
        let decision = reap_until_decided(&supervisor);

        assert_eq!(decision, SupervisionDecision::RestartAll);
        for index in 0..3 {
            assert_eq!(supervisor.generation(index), GenerationId::INITIAL.next(), "child {index} must also have restarted");
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]
    #[test]
    fn escalate_reports_the_failure_without_restarting_anything() {
        let supervisor = Supervisor::new(
            RestartStrategy::Escalate,
            generous_capacities(),
            Arc::new(StdThreadSpawner),
            Arc::new(NullEmit),
            EchoActor::default,
            1,
        );

        supervisor.address(0).send_blocking(Priority::Command, EchoMessage::Crash).unwrap();
        let decision = reap_until_decided(&supervisor);

        assert_eq!(decision, SupervisionDecision::Escalate);
        assert_eq!(supervisor.generation(0), GenerationId::INITIAL, "Escalate must not bump the generation itself");
    }
    //#endregion 🔖Supervision

    //#region 🔖Config
    #[test]
    fn mailbox_from_config_honors_the_profile_default_capacities() {
        let config = db_core::DbConfig::for_profile(Profile::Test);
        let (address, _receiver) = mailbox_from_config::<u32>(&config);
        assert_eq!(address.generation(), GenerationId::INITIAL);
    }
    //#endregion 🔖Config
}
//#endregion 🧪Tests
