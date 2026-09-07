# Terra Lifecycle Deadline Retained-Input Implementation Map

Status: source-only audit on 2026-09-06. No build or product edit was run. This is intentionally narrower than the broader reactor/lifecycle audit: it maps the exact owner that must survive `plugin.reactor-turn-deadline`, and separately records the newly observed native close-worker 8 ms failure.

## Finding

The reducer now makes a narrowly retryable promise, but no production host retains the corresponding input:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:108,770-791` creates `plugin.reactor-turn-deadline` only when there is no command page and every event is `InstanceOpen`, `InstanceClose`, or `InstanceLifecycleAck`. It returns the uncommitted typed patch to its owner before the fault.
- The direct native test helper retries this exact fault only in `.../⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime.rs:13-24`. No mounted host uses that loop.
- The wire has a lossless guest fault carrier: `plugin/🧬️schema/📜️.wit:20-22,1154` declares `plugin-error.fault(pack)`, and the guest macro emits `dsl::encode_fault_bytes` in `plugin/🦀️.rs:36-39`. The reusable decoder is `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs:625-632`.

Merely passing a tagged fault through the transport does **not** retain the event. Conversely, retaining an untyped string failure does not establish that this is the safe lifecycle-only retry class. Both pieces are required.

## Current dispatch ownership map

| Surface | Exact call/owner today | What happens to a guest `Fault(pack)` | Retained input available after the result? |
| --- | --- | --- | --- |
| Native Wasmtime | `.../🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1619-1658,1759-1919`; `DeferredAuthority::Event` is popped from `FixedOwnerRing` | The landed bounded decoder preserves WIT `plugin-error.fault` as `TurnFault::Guest(Fault)` | No. `execute_turn_for(actor,event)` owns the one local `event`; the ring entry was removed before invocation. |
| Native owned interpreter | `OwnedRuntime::execute_actor_turn` at `.../🖥️host/🦀️.rs:1231-1249` | The landed bounded decoder preserves `Err(Vec<u8>)` as `TurnFault::Guest(Fault)` | No. `OwnedInstanceState.pending` only covers the interpreter call; after deallocation it is gone, and a decode error has no original event holder. |
| Async Wasmtime component | `AsyncActorCommand::Poll` at `.../🖥️host/⏳️runtime/🦀️.rs:232-242`; `AsyncActorTask::spawn` at `:319-545` | `PollTask` now carries `Result<KernelTurnResult, TurnFault>` and preserves the bounded guest fault | No. The command has moved through an unbounded channel into an `AccessorTask`, and this runtime is explicitly unmounted (`:54-58`). |
| Browser shard client, generic turn | `ShardClient.captureActorActivation` at `.../🎭️actor/📮️shard-client/🟦️.ts:1455-1480` and `turn` at `:1859-1866` | inbound `ok:false` is only `{error:string,...}` (`:377-380`, `:1266-1292`) | No. It deletes the pending request then rejects a string-shaped error. |
| Browser instance lifecycle | `captureInstanceLifecycle` at `.../🎭️actor/📮️shard-client/🟦️.ts:1484-1523`, direct `sendInstanceLifecycle` at `:1724-1757` | no typed fault decode; a rejected worker result becomes `worker-refused` | No generic input owner. `ShardInstanceOwner.interruptedTurn` keeps only a completed result, never the open/close/ACK events. |

`CapturedReturn` (`shard-client.ts:665-706,1642-1671`) is deliberately **not** reusable: it owns a guest response/page reservation after a successful output admission. Its `retry` operates an output-return protocol, not lifecycle input; using it would invent an origin/page reservation for a request that has not produced output.

## Native: smallest retained authority

`ShardLoop` already has the right physical fairness owner. `pending_interactive` and `pending_background` are fixed `FixedOwnerRing<DeferredAuthority, _>` queues; `pump_primed` (`🧵️shard/🦀️.rs:1619-1658`) performs exactly one authority opportunity, rechecks generation before execution, and defers later work instead of looping inline.

The one missing datum is byte accounting. `FixedOwnerRing::pop_front` (`:817-819`) returns an `OwnerKey` and owner only, while `DeferredAuthority::Event` (`:861-865`) contains no byte cost. Do not guess it from the decoded `Event`: the original frame must retain its admission accounting.

### Exact changes

1. Change the event authority to retain its admission cost:

   ```rust
   DeferredAuthority::Event {
       actor: u64,
       event: Event,
       owner_bytes: usize,
   }
   ```

   `dispatch_envelope` already receives `owner_bytes` at `.../🧵️shard/🦀️.rs:2056-2080`; copy it into the event variant. Update `defer_completion`, `take_terminal_completion`, and every enum pattern/fixture constructor to carry or ignore that field deliberately. A generated completion has a known local allocation size; it must never inherit a foreign frame's cost.

2. Extract a strict predicate beside `execute_turn_for`, over the original retained `Event`, not merely the guest's advertised fault:

   ```rust
   fn lifecycle_only_retryable(event: &Event, fault: &Fault) -> bool {
       matches!(event,
           Event::InstanceOpen { .. }
           | Event::InstanceClose(_)
           | Event::InstanceLifecycleAck(_))
       && fault.code.0 == "plugin.reactor-turn-deadline"
       && fault.retryable
   }
   ```

   The exact `Fault` field spelling should use the existing struct rather than a second boolean/wire parser. A command ingress page, `Wake`, `JobCompleted`, app command, malformed fault, or any different code remains a normal failure.

3. Add `TurnFault::Guest(Fault)` beside the current five variants in `.../🖥️host/🦀️.rs:602-625`. Decode both Wasmtime `plugin-error.fault` and owned `Result<T,Vec<u8>>` with `dsl::decode_fault_bytes`; preserve all other inner errors as traps. This is transport only, not a retry.

4. Let `execute_turn_for` return a small private disposition after its mutable instance borrow has ended:

   ```rust
   enum TurnDisposition { Published, RequeueLifecycle }
   ```

   In the `RequeueLifecycle` case, `pump_primed` puts the **same** `DeferredAuthority::Event { actor, event, owner_bytes }` back onto the same lane before returning `Ok(1)`. The popped slot freed exactly one item and exactly `owner_bytes`; no external frame can mutate this single-owned `ShardLoop` while its guest future is awaited, so reinsertion must be an invariant-checked success. It must not surface an outcome, unregister the actor, or reconstruct input.

5. Recheck `actor_generation_is_current` when the requeued authority is selected. Existing `pump_primed` already does that. Unregister/replacement therefore retires the queued retry rather than applying an old lifecycle receipt to a successor.

This is a genuine retained continuation: it retains the decoded event plus the original ring byte charge, gives other ring authorities the next scheduling opportunities, and binds the retry to the exact actor generation. A new timer, a cloned external request, or a call to the native test helper is not an equivalent repair.

### Native laws

Add them to the existing native shard/lifetime selector, with a mock guest that first returns the canonical encoded retryable fault and then a valid captured/retired receipt.

1. `lifecycle_guest_deadline_requeues_exact_event_after_one_fair_opportunity`: same actor sees identical event twice; an equal-lane second actor runs between attempts; no fault outcome or effect/patch is published on attempt one.
2. `lifecycle_guest_deadline_requeue_preserves_fixed_ring_bytes`: queue bytes/items return to baseline after completion; assert the original frame byte count, not `size_of::<Event>()`.
3. `lifecycle_guest_deadline_cancel_or_generation_replacement_retires_without_second_guest_call`: enqueue retry, unregister or replace, then pump; no second invocation and no output on the successor.
4. `lifecycle_guest_deadline_never_retries_command_or_mixed_event`: a command-page/mixed turn with the same fault is reported as a structured guest fault and is not re-enqueued.
5. `owned_runtime_decodes_retryable_fault_bytes_without_loss`: an owned `Err(Vec<u8>)` decodes into `TurnFault::Guest` with code/message/retryable intact. A sibling Wasmtime WIT-law must prove the same inner result branch.

### Existing outer epoch deadline is separate

`execute_turn_for` maps `TurnFault::DeadlineExceeded` to `TurnStatus::MoreWork` at `.../🧵️shard/🦀️.rs:1883-1916`, but no authority/input is requeued. That is not the reducer's certified `plugin.reactor-turn-deadline` path and must not be blindly replayed: a Wasmtime epoch interruption can occur after guest work has begun. Do not merge this branch into the safe lifecycle replay predicate. It needs its own atomicity/resumption contract before claiming it is a retry.

## Async component runtime: typed transport now, retained owner only when mounted

`AsyncActorTask` is explicitly uncalled. It is not safe to hide retry inside its `PollTask`: the task permits concurrent accessor exports, has an unbounded command receiver, and an inner retry can race a later close/ACK/command from the still-unstructured outer caller.

The smallest coherent preparation is:

1. Replace `oneshot::Sender<Result<KernelTurnResult, String>>` in `AsyncActorCommand::Poll` with a private typed outcome such as:

   ```rust
   enum AsyncPollReply {
       Turn(KernelTurnResult),
       GuestFault(Fault),
       HostFault(PluginHostError),
   }
   ```

   `PollTask` decodes `plugin-error.fault(pack)` to `GuestFault`; traps remain host faults. This establishes transport fidelity only.

2. When an `AsyncActorTask` is actually admitted to an outer shard/actor scheduler, that outer owner must hold:

   ```rust
   struct RetainedAsyncLifecycleInput {
       actor: RuntimeActorId,
       generation: Generation,
       event: Event,
       budget: Budget,
       // exact request/receipt identity owned by the enclosing lifecycle cell
   }
   ```

   It sends an owned clone/view to `AsyncActorCommand::Poll`, but retains the original until a successful turn commits. On the exact guest predicate above it schedules the same holder for one later outer opportunity. Cancellation, `Shutdown`, store replacement, or a different generation closes the holder; it must never send a retry after `AsyncActorTask::cancel` aborts/drops its Store.

3. Do not use the `OwnedInstanceState.pending` interpreter field as this owner. It is an internal ABI-call continuation and is consumed before result decoding. It does not own scheduler admission, lifecycle generation, or a caller-visible retry promise.

Until a real async DRR/shard caller exists, the correct executable scope is one typed async-host law which proves the `Fault(pack)` survives `PollTask` without stringification. A retention law belongs with the future mounted scheduler, not a dormant component helper.

## Browser shard client: reusable scheduler, but a different lifecycle owner

There **is** safe reusable scheduling machinery: `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts:67-227` provides `TurnScheduler`. It has one bounded mailbox per actor, preserves lane selection, serializes an actor, and exposes `cancelQueued`/`teardownActor`. `ActivationRegistry` owns one at `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:1916-1942`, with generation checks in `runQueuedTurn` at `:2058-2081`.

That registry scheduler cannot directly solve lifecycle receipts: `ShardClient.captureInstanceLifecycle` and private `sendInstanceLifecycle` bypass it, and only `ShardInstanceOwner` knows the open request, captured lifetime, close request, ACK precondition, host-retirement witness, and active worker slot. Routing a receipt through `ActivationRegistry` would discard that authority.

### Minimal browser ownership seam

Reuse `TurnScheduler` inside `ShardClient`, not `CapturedReturn`, for a private one-slot-per-lifecycle-owner work type:

```ts
type RetainedLifecycleTurn = {
  readonly owner: ShardInstanceOwner;
  readonly activation: ShardActivation;
  readonly activationGeneration: bigint;
  readonly operationGeneration: bigint;
  readonly events: readonly ShardEventEnvelope[]; // captured before first post
  readonly acknowledged: ActorInstanceLifecycleReceipt | undefined;
  readonly budget: ShardBudget;
  readonly resolve: (value: unknown) => void;
  readonly reject: (error: unknown) => void;
};
```

`sendInstanceLifecycle` captures this object before asynchronous transport and returns its promise. It keeps `owner.inFlight = true` across retry rather than clearing it after a first guest fault. `runRetainedLifecycleTurn` does one `send` with a fresh `requestId`; it performs current activation/slot/owner/receipt assertions before posting and after a response.

On a decoded `plugin.reactor-turn-deadline` that passes the strict event predicate, it re-enqueues the **same** holder into the private lifecycle `TurnScheduler` and resolves that scheduler operation without resolving the public promise. A single lifecycle owner already rejects a second open/close/ACK while `inFlight`; therefore a mailbox capacity of one is sufficient and the retry cannot lose its slot to a later lifecycle operation. It is still scheduled after the current turn settles, not called recursively.

The holder must capture a structured-clone snapshot of the event(s) before the first `postMessage`; `open` accepts caller-owned `config/assets/capabilities/quotas`, so retaining the original references would permit post-return mutation to change the retry. Lifecycle event builders are internal thereafter. `acknowledged` and the activation/operation generations must remain exact values, never re-derived from current owner state.

`dispose`, shard loss, activation replacement, and lifecycle terminal close must call the scheduler's `teardownActor(actorId, rejectHolder)` before deleting/replacing the activation. A retry result that arrives after this must fail the captured identity assertion and retire; it must not enqueue against a successor with the same string actor id. The existing `captureActorActivation` checks at `shard-client.ts:1455-1480` are the authority predicate to reuse.

The generic `TurnScheduler` currently microtask-schedules every pump. A returned retry must be a queued later opportunity, never inline recursion. For the worker path, the awaited `postMessage` reply gives a browser task boundary; nevertheless a test adapter can synchronously resolve indefinitely. Add a scheduler continuation hook for retry work (or a standard `MessageChannel` task hook supplied by the real host) if a deterministic harness proves microtask-only repeated replies can starve cancellation. Do not use an unbounded `setTimeout` retry loop.

### Browser laws

The existing `ShardClient` fake-worker lifecycle suite around `shard-client.ts:2867-2871,4380-4410` supplies the right retained receipt test seam.

1. First open/close/ACK reply is a structured retryable guest fault, second is a valid receipt: caller promise settles only once, two workers messages carry byte-identical captured lifecycle input, and no `worker-refused` state is recorded.
2. Mutate the original `open` config/assets after the first post, then trigger retry: second message equals the captured snapshot, not the mutated caller data.
3. Close/dispose/replacement between first fault and retry: retained promise rejects revoked; no second message is posted; new same-id activation receives no old receipt.
4. Mixed/command event or malformed/nonretryable guest fault: no requeue; original error surfaces with decoded code/message.
5. Two actors, same lane: a retry for A does not block B's one lifecycle operation. Assert at most one lifecycle post per owner.
6. Repeated retryable faults under a fake synchronous worker use the continuation hook and permit a queued cancellation to run; no microtask spin.

## Native close worker: independent late-ceiling P0

The new law (`native lifecycle rerun 53408/ge8MXp`, first ten rows green) measures close-worker elapsed time at 10,439 µs. This is a different path from the reducer fault and must not replay the original close callback.

`plugin/🦀️.rs:30183-30353` first advances the retained `RuntimeCloseCleanupPump`: it may consume a `StepOutcome`, retain it in `pump.outcome`, move/close its `BatchJobSession`, and perform an app close step. Then `runtime_close_publish_turn` at `:30497-30500` replaces the computed `Ready`, `Complete`, or `ExternalWait` with terminal `Fault(InteractiveCeiling)` solely because elapsed time exceeded 8 ms. `plugin_step_close_cleanup` (`:30590-30613`) subsequently returns that error forever. The exact `cell`, `pump`, session, and outcome remain quarantined, but the scheduler has no legal next opportunity.

The repair is a separate retained-pump status, for example `RuntimeCloseStatus::DeadlineYield`, with a stored exact post-turn candidate status in `RuntimeCloseWorkerState`:

1. If the inner candidate is already a real `Fault`, preserve that fault; do not mask it as retryable.
2. If timing alone overruns, store the computed `Ready|Complete|ExternalWait` candidate and publish `DeadlineYield`; retain the existing cell/pump unchanged.
3. On the next `plugin_step_close_cleanup` opportunity, atomically consume `DeadlineYield` into its stored candidate. `Ready` schedules the next WorkerPool turn; `ExternalWait` follows the existing external-owner pathway; `Complete` may perform the normal quarantine removal.
4. The next worker invocation runs `runtime_close_cleanup_pump_one` on the retained `pump`. It drains `pump.outcome`/session or performs the next `close_step`; it does not reconstruct the old callback or repeat the already checked-out physical work.

Deterministic clock laws belong at `run_runtime_close_turn_with_clock`:

- drive a test app once with `0, 0, 8001` clock values; assert one physical close invocation, retained pump/state, and `DeadlineYield` rather than terminal fault;
- next explicit cleanup opportunity drains the retained outcome and reaches terminal emptiness without a second invocation of that physical app close step;
- cover `Ready`, `Complete`, and `ExternalWait` candidates, plus a genuine app/ABI fault that remains terminal even when its elapsed time is high;
- prove cancellation/quarantine removal cannot race the yielded state into a reused instance id.

This late-close repair preserves already completed physical work once. It must not be represented as `plugin.reactor-turn-deadline` or retried by re-submitting `InstanceClose`.

## Implementation order

1. Add typed `TurnFault::Guest(Fault)` and exact WIT/owned decoders with unit vectors; this is non-behavioral transport preparation.
2. Add native `DeferredAuthority::Event.owner_bytes` and the safe ring-held lifecycle requeue, with fairness/cancel laws.
3. Add the independent `RuntimeCloseStatus::DeadlineYield` retained-pump continuation and deterministic clock laws.
4. Add the browser typed worker fault frame and private one-slot `ShardClient` lifecycle holder using `TurnScheduler`; route teardown/replacement through holder retirement.
5. Change the async component reply to typed fault now, but defer actual retained retry until its promised outer DRR/shard owner is mounted.

No stage should claim lifecycle deadline recovery from a string-only transport, a test-only retry loop, `CapturedReturn`, a fresh timer request, or a broad replay of command/epoch-interrupted turns.

## 2026-09-06 Addendum: landed typed transport and close `DeadlineYield`

The current source has now landed the transport half correctly:

- `TurnFault::Guest(Fault)` is present in `.../🔌️plugin/🖥️host/🦀️.rs:602-650`.
- Both WIT `plugin-error.fault` and the owned-result decoder go through the shared bounded 64 KiB decoder, and `retryable_lifecycle_turn` constrains retry to at most one `InstanceOpen`, `InstanceClose`, or `InstanceLifecycleAck` event.
- The async `PollTask` reply is now `Result<KernelTurnResult, TurnFault>` rather than a string. This is necessary fidelity, but the runtime remains unmounted and still owns no outer retained request.

The native close worker has also landed `RuntimeCloseStatus::DeadlineYield`, an atomic `deadline_resume`, and the intended publication-only remeasurement path (`plugin/🦀️.rs:28897-28931,30183-30193,30497-30516`). Its core ordering is sound for `Ready` and `Complete`: the worker CASes `Queued -> Running`, sees a retained post-turn candidate before locking the pump, and a second late verdict writes that same candidate before publishing `DeadlineYield`. A real `Fault` bypasses that conversion.

### Correction: `ExternalWait` resumes after a successful candidate publication

An earlier version of this report incorrectly claimed that a yielded `ExternalWait` candidate was replayed forever. That is not true in current source. `runtime_close_publish_turn` clears `deadline_resume` in its successful (non-over-ceiling, non-fault) branch before it publishes the retained candidate (`plugin/🦀️.rs:30505-30513`). Therefore the next `ExternalWait → Ready` cleanup scheduling runs the ordinary pump/retirement probe, and can observe a cleared `PluginCloseStep::Blocked` condition or a dropped external `Arc`.

The retained-byte status representation is adequate for the three deliberately stored post-turn candidates (`Ready`, `Complete`, `ExternalWait`) because it is consumed only as a publication-only step. The real safety requirements are:

1. Write the candidate before `DeadlineYield` (the current sequentially consistent stores do so).
2. Clear it only on a non-late candidate publication (the current else branch does so).
3. Keep a genuine pump fault terminal even when elapsed time is high (the current fault guard does so).
4. Treat clock absence/regression as terminal timing-authority faults; do not retry after an unknown measurement.

Repeated over-ceiling publication-only callbacks can remain in `DeadlineYield`, but that is strict-budget yielding rather than a stale-candidate loop: no pump/app callback has been replayed and the exact candidate remains retained. The deterministic law matrix should cover late `Ready`, `Complete`, and `ExternalWait`, then a sub-ceiling publication followed by ordinary continuation; a real pump fault must stay `Fault`. The nonfault encode/decode test array at `:30455-30458` still needs to include `DeadlineYield`.

`deadline_elapsed_us` preserves the first breach diagnostic while allowing later publication-only measurements to proceed; `last_callback_elapsed_us` alone otherwise obscures the original violation after the resumed turn succeeds.

## 2026-09-06 Addendum: ShardLoop lifecycle retry must carry an allocation identity and a per-actor barrier

### Current source trace

This is a read-only source design packet. No build or runtime law was executed in this audit.

The native transport can now decode the strict retryable guest fault, but `ShardLoop` has neither a retained retry owner nor a safe identity to attach one to:

- `ShardLoop.instances` is `HashMap<u64, GuestInstance>` at [`shard/🦀️.rs:327-382`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:327). `register` is exactly `self.instances.insert(actor.0, instance)` at [`:1469-1475`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1469). A same-raw-id registration overwrites and Rust-drops the old `GuestInstance`; it does **not** await `GuestRuntime::drop_instance`.
- The existing [`actor_generation_is_current`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:2037) only compares the external packed `ActorId`. It returns true for a same-raw replacement and also returns true when there is no same logical actor at all (`current.is_none_or`). It cannot prove that a queued/retried owner still belongs to its original guest instance.
- `DeferredAuthority::Event` stores only `{ actor: u64, event }` ([`:861-870`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:861)); `dispatch_envelope` resolves that raw id only when it admits the envelope ([`:2056-2079`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:2056)). A later `execute_turn_for` again receives only that raw id.
- `pump_primed` takes the front deferred authority of the interactive ring before the background ring ([`:1619-1623`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1619)); `pop_next_authority` is FIFO except for consecutive exclusive job steps ([`:978-998`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:978)). A naïve retry pushed at the tail lets a queued later event for A execute first; a push at the head starves B.
- `FixedOwnerRing::pop_at` subtracts the owner byte credit and returns only `(OwnerKey, T)` ([`:817-845`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:817)). A retry held outside the ring therefore escapes both the 256-item and 16 MiB pending admission accounting unless its exact credit is retained explicitly.
- The native executor supplies a primed inbound frame only if `can_accept_primed_frame()` says there is no pending work ([`executor/🦀️.rs:605-611`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:605)); that method is currently `!has_pending_work()` ([`shard/🦀️.rs:1523-1533`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1523)). Thus an already-retained retry would otherwise prevent a subsequently arrived B or cancellation frame from ever being admitted before the retry.

The existing executor property itself demonstrates that raw-id overwrite is intentional in a test but not safe ownership: it makes a fresh instance with the same `ActorId` and calls `executor.register(actor, fresh_instance)` without unregistering the first at [`executor/🦀️.rs:1008-1045`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:1008). That path must become explicit replacement/unregister semantics; it cannot remain a hidden map overwrite.

### Required allocation identity

Keep the wire `ActorId` unchanged, but make its registration one private allocation:

```rust
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct ShardActorAllocation {
    actor: ActorId,
    registration: NonZeroU64,
}

struct RegisteredShardActor {
    allocation: ShardActorAllocation,
    instance: GuestInstance,
    retry: Option<RetriedLifecycleAuthority>,
}
```

`ShardLoop` should hold `HashMap<u64, RegisteredShardActor>` and an overflow-checked, never-zero `next_registration`. `current_allocation(actor)` returns an allocation only for a live exact raw `ActorId`; `allocation_is_current(token)` requires map entry equality. Do not retain the current absent-is-current rule for any owner-bearing authority.

Every authority decoded from an envelope must capture this token at admission: `Event`, `JobStep`, `JobReplay`, `Suspend`, `Resume`, and `CancelCursor`; generated `Event::JobCompleted` must capture the allocation current when it is synthesized. The job/replay maps, lane/budget cache, and replay seed lookup must either use `ShardActorAllocation` as their key or validate it before doing any guest call. An old raw `u64` must never be sufficient to call the replacement instance.

Make bare registration a pure refusal boundary:

```rust
fn try_register(
    &mut self,
    actor: ActorId,
    instance: GuestInstance,
) -> Result<ShardActorAllocation, ShardRegistrationRejected>;
```

It refuses a live same-raw actor and returns the exact incoming `GuestInstance`; it does not drop either instance. `ShardExecutor::register` and its fixed `registrations` queue must expose/retain that rejected owner even when registration is drained later, rather than preserving its present false `Admitted` result. The caller has a bounded terminal close route through `runtime.drop_instance` after it receives the refusal.

The current suspend/resume property must choose one explicit operation instead of relying on overwrite:

1. `unregister(old_actor)` drains/revokes old allocation owners and awaits `drop_instance(old)`;
2. only then `try_register` accepts the fresh instance, minting a new internal registration serial even if the external packed `ActorId` is deliberately reused; or
3. a separately named retained `replace_after_unregistered` cursor owns the old instance until its async drop terminally completes, then installs the new allocation.

Do not add an async `register` which awaits while `ShardExecutor.state` is locked. Do not invent a compatibility overload that silently replaces. The existing `ActorId::next_generation` remains the preferred externally visible restart identity, but the private serial is still necessary for same-raw reuse after an old queued owner existed.

### Retried lifecycle owner, barrier, and exact credits

Only `retryable_lifecycle_turn(&fault, &[event])` may transition an event to retry. `TurnFault::DeadlineExceeded`, fuel exhaustion, ordinary guest faults, a mixed event bundle, and a lifecycle fault without the exact retryable code remain on their current terminal paths.

Use one fixed retained retry entry per admitted actor allocation, not a tail requeue:

```rust
struct RetriedLifecycleAuthority {
    allocation: ShardActorAllocation,
    event: Event,
    lane: Lane,
    owner_bytes: usize,
    yield_one_peer: bool,
}
```

The entry belongs in `RegisteredShardActor.retry`; a fixed 256-slot `retry_order` stores allocation order. At most one retry can exist for an allocation, since no later actor authority is selectable through its barrier. Extend `FixedOwnerRing` with a private `pop_at_with_credit` or return a `RetainedDeferredAuthority { owner, bytes }`; do not reconstruct byte cost from `Event::size_of`.

Moving a popped authority into `retry` does not release its ownership budget. Add `retry_interactive_{items,bytes}` and `retry_background_{items,bytes}` to the corresponding queue admission calculation. `preflight_frame`, `enqueue_authority`, generated completion admission, interrupted `CancelCursor` handback, `has_pending_work`, and the one-frame terminal/full decision must all see `pending + retry` as one 256-item / 16 MiB lane budget. Transitioning queue → retry keeps the total unchanged; successful retry, cancellation, unregister, and stale-allocation discard debit it exactly once. A separate 256-slot retry ring without this debit would silently double the accepted deferred bytes.

Selection for one lane must be:

1. First, find a queued exact cancellation or unregister for the retry's allocation. It is an explicit revocation and may pass the barrier. `Unregister` and `CancelCursor` must themselves carry the admission-time allocation token, otherwise an old raw-id control message can revoke a successor.
2. If `yield_one_peer`, select the earliest eligible non-A authority of the **same scheduling priority**; skip every authority whose allocation is A. After exactly one such B has run, flip the retry entry to retry-first.
3. If there is no eligible B, or after that one B, select the retained retry directly. It executes the byte-identical event on the same allocation. Later A stays queued until the retry succeeds/terminally faults/revokes.
4. On a second retryable fault, retain the same owner again and reset `yield_one_peer`; on a terminal response clear it before ordinary queue selection resumes.

Normal Interactive/UserVisible priority over Background/Maintenance remains unchanged. The fairness promise is therefore exact for B at the retry's same lane/priority: with `A1`, later `A2`, and B admitted at that priority, the observed guest calls are `A1(fault), B, A1(retry), A2`. A continuous higher-priority lane can still preempt both A and B under the existing scheduler policy; this packet must not weaken that policy.

To make B or an explicit revocation that arrives *after* A1 eligible, extend `ShardLoop::can_accept_primed_frame` to return true for a live retry barrier, not just for an empty shard. `ShardExecutor::run` then transfers at most one retained ingress frame on that next worker opportunity. `pump_primed` consumes that primed frame before selection already. If it contains B/cancel/unregister, the barrier selects it; if it contains only later A, it cannot pass and the retry runs. This is one normal bounded frame admission, not a timer, a polling loop, or an unbounded queue bypass.

### Native law matrix

Use `MockGuestRuntime::script_turn` and its observed event trace in `shard/🦀️.rs` tests, plus the production `ShardExecutor` property path for the primed-frame case.

1. **Exact order and one-peer fairness.** In one interactive `Grant`, queue lifecycle A1, later A2, and B. Script A1 with the exact retryable `plugin.reactor-turn-deadline`, then B and A1 success, then A2 success. Assert the trace `A1,B,A1,A2`, the first A emits no effects/patch/outcome, and both A attempts receive byte-identical event encoding.
2. **Late B admission.** Admit A1; after its fault, put B in `ShardExecutor.ingress`. The retry-aware priming opportunity must admit B and run it before A retry. A frame containing only later A must instead produce `A1,A1,A2`.
3. **Same-raw replacement fence.** A1 faults; `try_register(A, fresh)` while A remains live returns the exact fresh `GuestInstance` and does not invoke either old/new guest. After explicit old unregister/drop and same-raw re-registration, old A retry and queued A2 are discarded by allocation mismatch; only a new envelope admitted after the new token reaches fresh.
4. **External-generation fence.** Queue/retry old `ActorId`, retire it, register `ActorId::next_generation`, then admit old and new envelopes. Old allocation has no guest call; new exact allocation has one.
5. **Revocation.** Queue A1/A2 then exact `CancelCursor` or `Unregister` after A1 faults. It consumes the retry credit exactly once; no second A1 guest call occurs. Unregister drops the original instance exactly once and stale A2 cannot touch a same-raw successor.
6. **Credit/capacity.** Fill each lane to 256 / 16 MiB including a retry. A plus-one frame is retained/rejected exactly as before. Retry→success/cancel/unregister frees exactly one item and its original byte credit; no double return after stale discard.
7. **Fault classification.** `DeadlineExceeded`, fuel, malformed/oversize fault bytes, nonretryable guest fault, and two lifecycle events must not create a retry barrier. A terminal fault never leaves hidden retry credit.

### Minimal edit inventory

- [`🖥️host/🧵️shard/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs): actor slot/token, retained-credit ring handoff, all authority variants and maps, retry-aware selection/admission, lifecycle-fault disposition, and direct laws.
- [`🖥️host/🧵️shard/🧵️executor/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs): return exact duplicate-registration owner across immediate and queued registration, allow one primed ingress frame for a retry barrier, and replace the same-raw suspend/resume test setup with explicit retirement/re-registration.
- [`🖥️host/🧵️shard/👶️child/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/👶️child/🦀️.rs): consume `try_register` result and terminally drop any rejected instance rather than assuming infallible insertion.

This is the native retained-input half only. It neither changes component WIT nor claims browser/async host retry ownership.
