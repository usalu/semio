# Current Shard Lifecycle Retention Audit

Scope: read-only inspection of the current native shard lifecycle, executor, and activation callers. No build was run.

## Current positive boundary

`ShardLoop::consume_frame` now rejects a target that is not simultaneously present in `instances` and `allocations` before grant-budget or lane-map mutation. `JobStep` admission now occurs only after the selected allocation is current, so an unknown queued job faults before a guest call or command-turn mutation. These close the prior unknown-actor map-pollution and dispatch-time mutation findings.

The lifecycle retry remains in its original priority ring and retains the exact `ShardActorAllocation`, byte cost, lane, and authority. The existing peer barrier admits a different current allocation in that same ring before a yielded retry can run again. It is not a cross-priority fairness mechanism; that is a policy/test choice, not a demonstrated credit leak.

## Superseded immediate-branch P0 — shared activation now compensates ordinary failures

Both activation callers mint a Kernel actor before instantiating and registering the guest:

- [`activation.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs:118)
- [`runtime.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs:106)

This was true before the shared activation helper landed. Current
[`install_actor`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🦀️.rs:18)
checks the Kernel record/shard before instantiation and routes ordinary
instantiate/refusal/rejection failures through guest disposal then
`retire_failed_activation`. Root's five concrete shared-helper cases are green.
Do not retain the claim that those *ordinary returned branches* still leak.

The remaining P0 is cancellation/liveness, not branch duplication: an external
drop at an await inside `install_actor` can still strand its local guest or
Kernel actor. The coordinator/retained-slot design below supersedes the proposed
caller-local `PendingKernelGuestActivation` repair.

Required native rows:

1. instantiate fault leaves no actor, scheduler entry, or shard allocation;
2. successful instantiate plus missing shard drops the exact instance once and removes the actor;
3. queued registration refusal returns the exact instance to the retained activation owner, which closes it before deactivation;
4. cancellation while the registration reply is pending retains the same owner until guest close and Kernel deactivation both finish.

## P0 — `oneshot` receiver loss currently manufactures ownerless `Stopped`

[`ShardExecutor::register`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:345) maps `receive.await` failure to `RegistrationAdmission::Stopped`. That value carries no `GuestInstance`. This is not the normal closed/pool-shutdown path: those now correctly return `Refused::Stopped` with the exact instance before either direct or queued admission. It remains the unexpected channel-loss path, where the activation caller has no way to invoke the required async `GuestRuntime::drop_instance`.

`refuse_pending_registrations` can retain a receiver-less entry (`reply: None`) in the same fixed ring, and `take_unclaimed_registration` can return it. However, no current caller uses that method. Root's in-progress native/parallel per-shard asynchronous drop drive is the necessary completion, but it needs an explicit terminal contract: channel disappearance must leave the tuple in the executor-owned ring, never be reported to the caller as an ownerless terminal state.

Use a typed outcome such as `RegistrationAdmission::ExecutorRetainsForClose` only internally, or keep waiting until the executor has placed the raw guest in its async close drive. Public `register` must not expose a bare `Stopped` after a guest has been accepted into executor ownership.

Required law: forcibly drop the reply receiver after the executor accepts the tuple; assert one native/parallel shard tick invokes `drop_instance` exactly once, no actor remains registered, ring credit returns, and no user-facing result lacks an instance owner.

## Superseded P1 — terminal reply wake now occurs outside the executor mutex

Current
[`refuse_pending_registrations`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:558)
pops under `state`, releases that mutex, then sends the refusal; a failed send
returns the exact instance to a `reply: None` row. Root's inline-wake law is now
green. The old mutex-held-send claim is superseded. The separate receiver-loss
terminal-cleanup issue remains below.

## Bounded same-raw actor identity policy

No unbounded shard tombstone map is justified by the current Kernel contract. [`Kernel::activate`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4745) advances the per-package ordinal and [`cascade_remove`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:5075) does not reset it: ordinary activation/deactivation therefore does not reuse a raw `ActorId` during a Kernel lifetime. `ShardActorAllocation.registration` remains the exact private identity for queued work already admitted.

The clean fence is a consuming, crate-private `KernelActorRegistration` authority minted only for a live Kernel actor pinned to its selected shard. `ShardExecutor::register` consumes/validates it only after local registration succeeds; refusal leaves it with the retained activation owner for cleanup/retry. A restart must obtain an explicitly validated `next_generation` authority before admitting the replacement. `Kernel::activate` must use checked ordinal increment and fault before mutation on exhaustion.

The test seam is: after old actor deactivation, an old buffered Grant/Envelope cannot acquire a live registration authority and cannot reach a new guest; a restart token has a new validated identity; an already-admitted retry still rejects its old allocation nonce. This is bounded by the existing live Kernel actor table and avoids a lifetime-growing raw-ID registry.

## Qualification boundary

The current source establishes local ingress and retry retention only. It does not yet prove runtime cleanup of receiver-less registrations or compensation of the two activation callers. No native qualification is claimed here.

## Current-source delta after the initial read

Native and Parallel `tick_and_dispatch` now each call `take_unclaimed_registration` once per shard and await `GuestRuntime::drop_instance`. This is a real improvement: a dropped registration receiver no longer implies immediate raw-instance loss while the runtime continues ticking. The earlier report's statement that no caller used the method is superseded.

It does not make the public bare `RegistrationAdmission::Stopped` a complete terminal contract. If executor closure follows the receiver loss and no later kernel tick is driven, the owner remains in the executor ring with no independent close driver. The executor terminal-close path therefore needs a bounded, host-owned unclaimed-registration drain, rather than relying on a future scheduler turn that may never occur.

The activation compensation finding remains current in both callers: instantiate failure and the impossible-but-validated missing-shard branch return before cleanup. The source still uses an unchecked `*ordinal += 1` in both Kernel activation methods. A checked preflight is necessary for the no-reuse contract even though ordinary deactivation does not reset the per-package ordinal.

## P0 — a delayed raw transport frame can reach a same-id successor

This is a present, executable local-reuse path, not merely ordinal-wrap speculation. The lifecycle law [`retry_refuses_replacement_and_stale_queue_cannot_reach_same_id_successor`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🔁️lifecycle/🧪️tests/🦀️.rs:84) deliberately unregisters `ActorId(1)` and registers a fresh guest with the same raw id. `ShardActorAllocation.registration` changes, correctly preventing already-admitted retry owners from calling the successor.

However, [`ShardFrame`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:58) serializes only the raw actor id in `Grant` and `Envelope`; [`consume_frame`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1977) accepts any currently-present raw id. A delayed old Grant or Envelope after that re-registration is therefore applied to the fresh guest. The allocation nonce is currently a local queue-only guard.

Smallest bounded repair: make the wire target an exact `ShardActorKey { actor: ActorId, registration: NonZeroU64 }`. Every target-bearing frame (`Register`, `Unregister`, `Grant`, and standalone `Envelope`) carries or is wrapped by that key, and `consume_frame` checks exact equality with `allocations[actor]` before budget, lane, replay, or guest mutation. The current allocation table is the bounded live map; no tombstone is required.

The Kernel needs the current key as activation state: after executor registration succeeds, bind the returned allocation to the live actor before it can be scheduled, and have `TurnGrant`/both native dispatch loops stamp that key. Local reactivation binds a new key before the scheduler is set active. The direct process child must mint its own one-live-instance key and must not offer raw registration to external input. Make raw `ShardLoop::register` crate-private; production entry is a consuming, shard-pinned Kernel registration authority. Existing tests may use a test-only constructor.

Required law: retain a byte-identical old Grant/Envelope, unregister and re-register the same raw id, then inject the old bytes. It must fault/discard without budget/lane-map mutation or a successor guest call; a new-key frame must run. Add a checked ordinal exhaustion test for both `activate` and `activate_pinned`.

## Minimal end-to-end instance-identity repair

### The two identities are complementary

`ActorId`'s packed generation and a checked, per-package Kernel ordinal prevent a Kernel allocation from being silently reused. They do **not** distinguish two physical guest instances deliberately re-bound to the same still-live raw actor id. That rebind is already exercised by the lifecycle test above. The bounded per-instance transport nonce is therefore required at every wire admission; checked ordinal/generation is still required at Kernel allocation and restart.

Use a domain-neutral `ActorShardKeyV1` with these semantic fields:

| field | native packed form | web structured-clone form | invariant |
| --- | --- | --- | --- |
| `actor` | `ActorId` u64 | existing actor id string | exact target |
| `registration` | non-zero u64 | `bigint`, 1..=2^64-1 | exact physical guest incarnation |

The shared JSON fixture can represent both values canonically as decimal strings, while Rust pack codecs use `read_u64` plus non-zero validation and TypeScript converts `registration` to `bigint`. The schema belongs beside actor identity rather than in a renderer: `🧰️framework/🔨️modules/🎭️actor/🪪️shard-key/{🧬️schema.json,🧫️fixture/🔣️.json}`. It specifies the semantic state transition, not an invented JSON replacement for the native packed transport.

### Native ownership and wire path

1. In [`actor/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4701), add `transport_key: Option<ActorShardKeyV1>` to private `ActorMeta`. `Kernel::activate` and `activate_pinned` must register the Scheduler entry inactive; `ActorStatus::Activating` is not currently observed by Scheduler, which otherwise can drain a mailbox during guest instantiation and emit an unbound Grant.
2. `ShardExecutor::register` returns `RegistrationAdmission::Admitted(ShardActorAllocation)` rather than a bare acknowledgment. The host commits that exact actor/shard/registration once through `Kernel::bind_transport_key`. The commit verifies the actor is still Activating, its pinned shard equals the key's shard, and the key is not already bound; it then records the key and calls `Scheduler::set_active(actor, true)`. Abort/deactivate retains scheduler inactive.
3. The existing `ShardActorAllocation.registration` is the native registration value. It is already checked against wrap (`next_registration` becomes zero and future admission refuses); do not add another live map. `ShardLoop::register` accepts the host-provided key or returns it on refusal, and raw registration becomes crate-private/test-only. Rebind first makes the Kernel entry inactive, locally retires the old guest, obtains a new key, then commits only after the new guest is registered.
4. Add `key: ActorShardKeyV1` to all target-bearing native frames: `Register`, `Unregister`, `Grant`, and standalone `Envelope`. `Grant` must reject unless every envelope targets `key.actor`; `Envelope` must reject unless `envelope.to == key.actor`. Before `validate_frame`, `preflight_frame`, budget/lane writes, or guest dispatch, `ShardLoop` compares the key with the one live `allocations` entry. A stale `Unregister` must be inert/faulted, never remove a successor.
5. Add the key to [`TurnGrant`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:3959), so `Kernel::tick` can emit only committed keys. Both host bridges are exact producers: [`NativeKernelRuntime::tick_and_dispatch`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs:162) and [`ParallelRuntime::tick_and_dispatch`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs:161) stamp it into `ShardFrame::Grant`; their `unregister` methods stamp the same key. The process child and [`process-transport`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs:761) use the same decoder/key contract.

`Kernel::activate` and `activate_pinned` must return a typed exhaustion error after a checked ordinal preflight, not `ActorId` with release-mode wrap. `ActorId::next_generation` needs the same non-wrapping policy at its actual restart authority; its current wrapping helper is insufficient by itself.

### Existing web counterpart

The web bridge already carries a physical-incarnation value outside `ShardFrame`: `OutboundMessage::frame` has `activationGeneration`, and `ShardClient.envelope`/`grant` capture it from the current private activation lease at [`shard-client.ts`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:1879). It is monotonic and checked on activation.

But the generated worker's ordinary `case "frame"` at [`plugin packages TS`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:418) does not compare `actor.activationGeneration` with `msg.activationGeneration` before `interpretFrame` writes the granted-budget map or invokes `poll`. `turn` and `dispose` do compare it. Add the same guard before *any* frame interpretation; effect-result already uses `deliverEffectResult`'s exact generation check. No second TS frame-key field is needed: the existing outer field is its key carrier. Keep it semantically paired with native `registration` through the new fixture.

### Required cross-language laws

1. **pre-registration:** submit while Kernel actor is Activating; `tick` produces no grant and leaves mailbox bytes intact. After exact bind, one new-key grant runs once.
2. **delayed grant:** retain old packed native Grant, rebind the same raw actor, inject it; assert fault/rejection, zero successor calls, and unchanged `granted_budgets`/`actor_lanes`. The current-key grant runs once.
3. **delayed standalone envelope:** identical assertion, including no fallback-budget mutation.
4. **delayed unregister:** old key cannot destroy the new guest; current key closes exactly once.
5. **malformed/mismatched:** zero registration, zero registration, max u64, actor/key mismatch, and mixed-envelope Grant all fail before preflight allocation or guest mutation.
6. **Kernel exhaustion:** both `activate` methods and actual restart reject before `ActorMeta`, Scheduler, or ShardTable mutation.
7. **web stale frame:** activate A, retain an old outer frame, dispose/reactivate A, deliver the old message directly to the generated worker; it neither runs `poll` nor changes `grantedBudgets`; current generation succeeds. This specifically covers the worker-side omission rather than only `ShardClient`'s well-behaved producer.
8. **parity:** AJV validates the common `ActorShardKeyV1` corpus; Rust maps its decimal keys to packed `ActorId`/non-zero `u64`, and the TypeScript actor package maps them to actor string/`bigint`, yielding the same accept/reject and mutation counts.

The existing lifecycle fixture is useful for retry allocation ownership, but it cannot alone prove a transport frame was rejected. Keep its current retry laws and add the identity corpus as a sibling; do not weaken it into a generic actor-id test.

## Executor shutdown and cancelled activation: current bounded-owner seam

### A pool shutdown cannot be the cleanup executor

The reusable mechanism is the *shape* of the mounted relay reaper, not its
`GuestRelayMountedOwner` payload:

- [`GuestRelayMountedRegistry::detach`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:4176)
  changes an exact generation-qualified slot to `DetachedForReap` and launches one
  `Lane::Maintenance` retained driver.
- [`GuestRelayMountedReaper`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:4567)
  performs one close opportunity, round-robins, parks on an owner waker, and only
  uses its coalesced timer when no close wake can be registered.
- `WorkerPoolUse` is the existing shutdown fence: [`WorkerPool::shutdown`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1959)
  returns `Busy` while a use is retained, before closing maintenance hooks, callbacks,
  and worker admission.

Do **not** place a `GuestInstance` close future directly in
`GuestRelayPoolFuture` and let the pool stop. On pool shutdown or poison,
[`GuestRelayPoolFuture::fail`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:3013)
removes and drops its retained future; its own relay owners are designed for that
failure contract, but an arbitrary `GuestRuntime::drop_instance` future is not.
Likewise `WorkerMaintenanceRegistry::shutdown` prevents further hook requests.
There is intentionally no valid “run an async cleanup after its only pool stopped”
path.

The minimal coherent ordering is therefore:

1. `NativeKernelRuntime::new` and `ParallelRuntime::new` pre-acquire one
   `Arc<WorkerPoolUse>` before constructing a Kernel or any `ShardExecutor`. A closed
   or closing caller pool is a constructor refusal, not a partially live runtime.
2. The runtime-owned close cursor retains that use until every executor close cursor
   is terminal. Consequently an outside `WorkerPool::shutdown()` first returns
   `Busy { retained_uses: 1 }`; it cannot silently discard a close driver.
3. Only the terminal runtime close releases that use. A genuine pool poison/admission
   failure returns an owner-bearing close fault; it must not drop the cursor or claim
   shutdown completed.

This is bounded by the existing runtime's fixed shard vector and the existing
`SHARD_DEFERRED_ITEMS` rings. It introduces neither a global unbounded cleanup queue
nor a foreground `tick` dependency.

### Exact retained close cursor

`ShardExecutor` currently has the required raw owners but no terminal driver:

- `state.registrations` already retains a receiver-less
  `(ActorId, GuestInstance, None)` in the fixed owner ring
  ([`refuse_pending_registrations`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:563)).
- `state.drive` may itself own a `ShardLoop` plus an in-flight async registration or
  guest close; it cannot be discarded or replaced while pending.
- After a terminal handoff, the executor leaves both `state.shard` (with all normal
  instances) and receiver-less registrations intact. Dropping the outer `Arc` would
  destructure raw `GuestInstance`s without the required
  [`GuestRuntime::drop_instance`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:707)
  call.

Add an executor-owned, non-public `ShardExecutorCloseCursor` rather than an
independent registration queue. It retains the exact `Arc<ShardExecutor>`, the
runtime's existing `Arc<GuestRuntimes>`, its executor close generation, and at most
one checked-out owner. Its phases are:

```
SealIngress -> DriveInFlight -> DrainUnclaimed -> DrainLiveInstances -> Terminal
```

- `SealIngress` atomically fences new registration/frame admission and turns each
  still-replying registration into its current owner-bearing refusal. It must send
  replies after releasing `state`, as described in the earlier mutex finding.
- `DriveInFlight` never moves or drops `state.drive`; it asks the executor's retained
  single-flight driver for one bounded close/poll opportunity. A pending drive remains
  in that exact field and wakes the same close cursor. This preserves a guest close
  already suspended inside the drive.
- `DrainUnclaimed` removes one `reply: None` entry from the existing ring only after
  it is locally owned by the cursor, awaits `GuestRuntime::drop_instance`, and retains
  that same entry on cancellation/fault.
- `DrainLiveInstances` must call a new private `ShardLoop` close cursor, not drop the
  loop. `ShardLoop::unregister` is the proven semantic primitive
  ([`shard/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1489));
  the close cursor iterates one stable live allocation per opportunity, retires its
  replay/job/deferred owners, and awaits that unregister. It must additionally drain
  terminal/deferred frame owners before returning `Terminal`.
- Only the terminal transition removes the exact reaper hook/slot and releases the
  `WorkerPoolUse` clone. A stale generation, duplicate close, or late executor wake is
  inert.

The existing relay reaper is a suitable local implementation model: one fixed
runtime/executor close slot, `DetachedForReap`-equivalent ownership, a
generation-qualified waker, and one `Lane::Maintenance` opportunity at a time. Its
payload types must not be reused: they own worker-job sessions and relay output, not
guest instances or a `ShardLoop`.

### Kernel authority cannot be cleaned up by the executor alone

The executor can close guest memory, but it cannot remove the associated `Kernel`
actor. A cancelled activation makes this material. `install_actor` mints an actor
before both awaited `GuestRuntime::instantiate` and `ShardExecutor::register`, while
[`Kernel::deactivate`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:5097)
requires mutable Kernel ownership. The current `tick_and_dispatch` drains only one
unclaimed guest per shard and neither deactivates its actor nor runs if the host goes
idle.

Thus a `ShardExecutorCloseCursor` is necessary but insufficient for cancelled
activation. The smallest host boundary is a fixed, runtime-owned
`PendingKernelGuestActivation` slot created **before** `Kernel::activate` is
committed. It owns `{ actor, shard, instance: Option<GuestInstance>, phase }` and an
`Arc<WorkerPoolUse>`; the runtime's own retained driver, which alone has mutable
Kernel access, advances it:

```
KernelReserved -> Instantiating -> Registering -> Bound
                      |                |
                      +--> CloseGuest -> DeactivateKernel -> Terminal
```

An externally cancelled activation future detaches this exact slot to that driver;
it does not drop the local future's `GuestInstance` or return bare `Stopped`. The
driver first closes the guest if present, then calls `Kernel::deactivate`; a failure
or unavailable runtime yields an owner-bearing close cursor. Until `Bound`, Kernel
scheduling must stay inactive so an abandoned reservation cannot receive a Grant.
This also gives the executor reaper a single host-side route to report a
receiver-lost actor for deactivation, without making `Kernel` thread-safe or adding a
second cleanup queue.

An `async fn install_actor(&mut Kernel, …)` cannot itself provide this guarantee: a
caller can drop it at every await. The concrete unsafe cuts are after instantiate
returns and before/inside `shard.register().await`, during `runtime.drop_instance`
on a refusal, and during `retire_failed_activation(...).await`. A local Rust drop of
`GuestInstance` is not equivalent to the trait's required async release contract.

### Minimal native laws

1. Queue a registration, drop its receiver, then stop foreground ticks. The
   runtime-owned reaper closes that one guest and its Kernel actor exactly once;
   ring credit and runtime `WorkerPoolUse` are still retained until terminal close.
2. Hold a guest `drop_instance` future pending; request runtime close and call pool
   shutdown. It returns `Busy`, the cursor remains byte/instance-identical, and a
   wake completes close without a `tick` call. The next shutdown succeeds.
3. Start activation, suspend instantiate; drop the caller future. Release instantiate
   and assert no raw guest, scheduler entry, shard allocation, or retained pool use
   remains after the detached activation reaches terminal.
4. Suspend registration after guest creation, cancel the caller, then terminalize the
   executor. Assert `drop_instance` once before `Kernel::deactivate`, no Grant to the
   activating actor, and no replacement executor steals the old cursor.
5. Fill the fixed deferred registration capacity, cancel every receiver, and prove
   close releases one rotating owner per reaper opportunity; a live caller-owned
   registration is never reaped.
6. Force maintenance admission/pool poison before close. The public close operation
   returns the exact retained cursor and holds its pool use; it never reports success
   by dropping the instance.

No native qualification is claimed: the current source has tick-based best-effort
draining only, no runtime-held `WorkerPoolUse`, no executor close cursor, and no
cancellation-safe activation operation.

## Owned Kernel coordinator and one retained activation slot

### Decision

One pending-activation slot is sufficient **per current mutable façade**, but it
cannot be a detached guest-only reaper.  Both façades own `Kernel` by value and
expose it as a borrow:

- [`NativeKernelRuntime`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs:21)
  has `kernel()` and `kernel_mut()` beside its current `async activate`.
- [`ParallelRuntime`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs:18)
  has the same shape.

A future stored outside either façade cannot hold `&mut Kernel`; a foreground
caller may keep that borrow, and an idle/drop path would have no lawful way to
call `Kernel::deactivate`.  Retaining only `GuestInstance` cleanup therefore
still leaks the Kernel actor or races later raw mutation.

The coherent target is one shared, owned `KernelRuntimeCoordinator` in
`plugin/🖥️host/🎠️activation`, with `NativeKernelRuntime` and `ParallelRuntime`
as thin clients.  Its retained worker job owns the whole `KernelRuntimeCore`,
including the Kernel; it never stores a future borrowing it.  A bounded command
mailbox and the retained driver are the sole mutators.  This is the required
answer if a dropped caller must be cleaned up without a future foreground
`tick`/`activate` call.

The less invasive alternative is valid only as an explicitly caller-driven
API: a `&mut self` façade may retain one slot and require callers to invoke
`activation_step`/`close_step`.  It protects ownership across cancellation of
those *step futures*, but it cannot promise cleanup after the façade itself is
abandoned.  Do not represent it as automatic retirement.

### Current consumers that prevent a hidden coordinator

Raw Kernel mutation must be replaced, rather than leaving an escape around the
coordinator:

| Current consumer | Exact current use | Coordinator replacement |
| --- | --- | --- |
| [`🏃️run/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs:1737) | `kernel_mut().activate` for the direct `PluginInstanceHandle` model | a separately typed `begin_direct_handle_activation` command and its close ticket; it must not borrow the Kernel or pretend it registered a shard guest. |
| [WGPU renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:7125) | `set_capabilities`, `link_extension` | `SetCapabilities` and `LinkExtension` coordinator commands. |
| [WGPU renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:7162) | `deactivate` | `BeginActorClose` command returning an owner-bearing close ticket. |
| [WGPU renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:6399) | `actor_record`/metrics snapshots for replay qualification | clone-only `ActorRecord`/metrics query command; no borrowed `&Kernel`. |

The two process3d `kernel_mut()` hits are unrelated geometry-session kernels, not
these host façades.  The compiler should remove `kernel()`/`kernel_mut()` from
both runtime clients; keeping them makes the coordinator an optional bypass.

### Concrete API

The coordinator has a single activation owner because all current activation
entry points require `&mut self`; the public client preserves that serialized
contract.  A small fixed command ring (eight command owners is enough for one
begin, cancellation/close, replies, and a wake without turning an admission
burst into an unbounded queue) is owned by the coordinator, not by a caller.
The ninth command is returned unchanged in `CoordinatorCommandRejected`.

```rust
pub struct KernelRuntimeClient {
    coordinator: Arc<KernelRuntimeCoordinator>,
}

pub struct ActivationTicket(NonZeroU64);       // opaque exact slot generation
pub struct ActiveActorLease { /* actor + kernel/shard binding; non-forgeable */ }

pub struct ActivationRequest {
    pub package: PackageId,
    pub plugin_ordinal: u16,
    pub kind: ActorKind,
    pub lane: Lane,
    pub window: Option<WindowId>,
    pub event: ActivationEvent,
    pub compiled: CompiledHandle,
    pub capabilities: Vec<BrokerCapabilityGrant>,
    pub instantiate_budget: semio_framework::kernel::Budget,
}

pub enum ActivationBegin {
    Accepted(ActivationTicket),
    Busy { pending: ActivationTicket },
    Refused(ActivationRequest, RuntimeOpenRefusal),
}

pub enum ActivationProgress {
    Pending(ActivationTicket),
    Bound(ActiveActorLease),
    Closing(ActivationTicket),
    Fault(ActivationFault),
}

impl KernelRuntimeClient {
    pub fn begin_activation(&mut self, request: ActivationRequest) -> ActivationBegin;
    pub fn request_cancel(&mut self, ticket: ActivationTicket) -> Result<(), ActivationTicket>;
    pub async fn activation_step(&mut self, ticket: ActivationTicket) -> ActivationProgress;
    pub fn begin_rebind(&mut self, old: ActiveActorLease, request: ActivationRequest)
        -> ActivationBegin;
    pub async fn close_step(&mut self) -> RuntimeCloseProgress;
}
```

`CompiledHandle` is already `Clone` and owns its component/artifact
[`Arc`]s ([`host/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:518));
the request owns one clone.  `BrokerCapabilityGrant` is clonable and the
instantiation `semio_framework::kernel::Budget` is `Copy`.  This is expressly
that Kernel budget, not `semio_framework_actor::Budget`, which is a different
turn-wire type.  Thus no `compiled`, capability slice, or budget reference
survives the caller.

Internally:

```rust
struct KernelRuntimeCore {
    kernel: Kernel,
    runtime: Arc<GuestRuntimes>,
    shards: Vec<Arc<ShardExecutor>>,
    outcomes: Arc<OutcomeSink>,
    pool_use: Arc<WorkerPoolUse>,
    activation: Option<PendingActivation>,
    close: RuntimeCloseState,
}

struct PendingActivation {
    ticket: ActivationTicket,
    pool_use: Arc<WorkerPoolUse>,
    phase: PendingActivationPhase,
}

enum PendingActivationPhase {
    Reserving { request: ActivationRequest },
    Instantiating {
        reservation: KernelActivationReservation,
        future: Pin<Box<dyn Future<Output = Result<GuestInstance, PluginHostError>> + Send>>,
    },
    Registering {
        reservation: KernelActivationReservation,
        registration: ShardRegistrationTicket,
        future: Pin<Box<dyn Future<Output = RegistrationAdmission> + Send>>,
    },
    Binding { reservation: KernelActivationReservation, key: ShardActorKeyV1 },
    Cancelling(PendingActivationClose),
}
```

The instantiation future owns `Arc<GuestRuntimes>`, the `CompiledHandle`, cloned
capabilities, budget, and actor; the registration future owns its exact
`GuestInstance`, `Arc<ShardExecutor>`, and ticket.  Neither captures `&mut
Kernel`.  `activation_step` only polls those stored futures.  If its caller is
cancelled, the pinned future is still in `PendingActivation`; a ready result is
moved to the next phase synchronously before another await.

`Kernel` needs three narrow reservation operations, not reuse of
`Kernel::activate`/`deactivate`:

```rust
pub async fn reserve_activation(&mut self, request: KernelActivationRequest)
    -> Result<KernelActivationReservation, KernelActivationRefused>;
pub async fn bind_activation(&mut self, reservation: KernelActivationReservation,
    key: ShardActorKeyV1) -> Result<ActiveActorLease, KernelActivationBindFault>;
pub async fn abort_activation(&mut self, reservation: KernelActivationReservation)
    -> Result<(), KernelActivationAbortFault>;
```

The reservation is non-`Copy`, opaque outside the coordinator, owns the checked
actor/shard allocation, and is *not scheduler active*.  `bind_activation` is the
only transition that installs Scheduler admission after the exact shard
registration key is returned.  `abort_activation` is idempotent for that
reservation and removes only the unbound actor/pin; it is deliberately not the
current cascading `Kernel::deactivate`, whose partially awaited cascade is not a
retry cursor.  This fixes the current `Kernel::activate` ordering, which already
registers a scheduler actor before the host has registered its guest
([`actor/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4749)).

`ShardExecutor::register` must similarly become begin/poll/cancel by an opaque
`ShardRegistrationTicket`.  Cancelling it either returns the same unregistered
`GuestInstance` into `ClosingGuest`, or reports the exact admitted
`ShardActorKeyV1`; it may not convert a receiver loss to the current bare
`RegistrationAdmission::Stopped`, because the coordinator then still needs the
same actor's Kernel abort.  `ClosingGuest` owns an explicit retained guest-close
operation; raw `GuestInstance` drop is not a substitute for
`GuestRuntime::drop_instance`.

`begin_rebind` first changes the old `ActiveActorLease` to a closing state,
retires its exact shard key and Kernel active binding, then begins the new
reservation.  It must not activate a successor alongside the old binding.  A
cancel after `Bound` is ordinary `BeginActorClose`, not a stale activation
ticket; a stale ticket, duplicate cancel, or future ticket is inert/refused.

### Coordinator and pool lifetime

Open either implementation through the same constructor:

```rust
pub fn open(pool: Arc<WorkerPool>, runtime: Arc<GuestRuntimes>, config: RuntimeConfig)
    -> Result<KernelRuntimeClient, RuntimeOpenRefusal>;
```

It calls the already cross-target
[`WorkerPool::acquire_use`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1906)
(the wasm twin is at line 2519) before creating `Kernel` or any
`ShardExecutor`.  `NativeKernelRuntime::new` obtains its process pool through
`process_worker_pool` then delegates to `open`; `ParallelRuntime::new` delegates
with its caller-provided pool.  Both current constructors return `Self`; replace
them with this fallible open boundary rather than build a partially live runtime
on `Closing`/`Stopped`/use-counter overflow.

`WorkerPoolUse` is itself retained as `Arc<WorkerPoolUse>` in the core and every
pending activation/close cursor.  Cloning the Arc does not increase the pool's
use count; the final clone does.  Consequently `WorkerPool::shutdown` returns
its existing `Busy { retained_uses }` while the coordinator has any live core or
retained close, before maintenance and deferred wakes are stopped.  A driver
submission refusal remains a retained coordinator state and replies with its
exact command; it never drops the core or claims a close succeeded.

The coordinator's retained WorkerPool job owns `KernelRuntimeCore` for its whole
turn, polls one activation/close opportunity, and re-arms through its own
generation-qualified wake.  It is modelled on the mounted relay reaper's
ownership, not its relay payload.  A job/scheduling fault leaves the core in a
terminal handoff row with the `WorkerPoolUse`; only `resume` or an explicit
owner-bearing close failure can move it.  This is what makes host abandonment
safe without a foreground tick.

### Fixture traces

Add a sibling `retained-slot-v1` corpus under the existing
[`host/🎠activation/🧫️fixture`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🧫️fixture)
directory and exercise it through both native and WGPU coordinator clients:

1. `begin → instantiatePending → secondBegin`: second request returns the same
   pending ticket as `Busy`; its compiled/capability owners are returned, while
   the first request remains byte-identical.
2. `instantiatePending → cancel → instantiateReady → guestClosePending →
   guestClosed → kernelAbort`: no scheduler grant, no shard allocation, exactly
   one guest close and zero pool-use release before terminal.
3. `registerPending → cancel → executorAdmission`: exact registration ticket
   returns/adopts the guest once, then aborts the same Kernel reservation.  A
   receiver loss cannot strand the actor.
4. `registerAdmitted → bind`: current key binds once and only then a submitted
   envelope yields a Grant.  A cancellation requested before bind wins and
   prevents the bind.
5. `bind → staleCancel`: stale ticket cannot close the active lease; explicit
   `BeginActorClose(activeLease)` retires guest, shard key, and Kernel actor once.
6. `oldBound → beginRebind → oldClosePending → newInstantiate`: no two live
   keys for the old raw actor; cancellation of the replacement leaves the old
   lease retired, not silently restored.
7. `poolShutdown while instantiate/close pending`: reports `Busy`, the
   coordinator continues after its wake without a foreground tick, and shutdown
   succeeds only after all terminal rows release their single use.
8. `coordinator job refusal/poison`: command and core are returned in the fixed
   terminal handoff, resume drives the same ticket; no raw `GuestInstance` drop,
   no forgotten Kernel reservation, and no extra pool use.

The existing five-row activation fixture proves only immediate success/failure;
it does not cover caller cancellation, one-slot busy refusal, retained pool use,
or post-registration loss.  No runtime qualification is claimed.

## Native WIT UI patch bridge

### Concrete current loss

This is a normal-path P0, separate from the coordinator work. The canonical WIT
already declares surface-ref with instance and surface and all eleven variants:
upsert, set-component, set-layout, set-activity, set-children, set-style,
set-accessibility, set-bindings, set-menu, remove, and set-root
([schema](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:821>)).
The guest reactor already has the exact encoder at
([reactor encoder](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1525>)).
Comments in the native host claiming an unagreed path or node representation
are therefore stale.

The Wasmtime path drains emit_patch_sink into an unused local and returns the
default UiTurnPatches owner
([native host](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:2147>)).
It also ignores the returned WIT patches. The async path takes _patches and
then defaults the owner too
([async conversion](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime/🦀️.rs:284>),
([async caller](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime/🦀️.rs:396>)).
Consequently both a returned component patch and a host-async emit-patch vanish
before the existing renderer transport can receive them.

### Small shared conversion boundary

Add one private converter shared by the native and async host paths, adjacent
to their existing WIT conversions:

    wit_ui_output_to_kernel(returned, emitted, receipt, budget)
      -> Result<(UiTurnPatches, Option<ActorUiPatchReceipt>), TurnFault>

The converter accepts exactly zero or one total patch. Current WIT exposes a
list, so the immediate bridge must reject a returned count other than zero or
one before semantic conversion. The greenfield ABI should then make this
schema-level by replacing turn-result ui-patches with one optional ui-patch,
not by adding a second compatibility field. A WIT operation list still lifts
into host memory; immediately cap its operation count and all packed payload
bytes by budget.max_patch_bytes before typed placement. A post-lift Vec length
check is not a host-allocation admission proof for an untrusted component.

Replace both unbounded patch sink Vec values with Empty, One(wit-patch), or
Refused. The first emit-patch moves in one value. A second drops both source
values, records Refused, and makes poll return a typed fault. It may not select
the first patch or retain an unbounded sink. A returned patch plus an emitted
patch is also a typed multiple-output fault, even if byte-identical.

Decode all eleven variants in reverse:

- upsert is a packed UiNodeRecord;
- set-component, set-layout, set-style, and set-accessibility retain their id
  and use the matching exact typed payload;
- set-activity decodes the existing packed activity plus disabled wrapper;
- set-children builds bounded UiNodeChildren from the u64 identifiers;
- set-bindings and set-menu decode UiNodeBindings and Option<MenuRef>;
- remove and set-root become the corresponding UiNodeId operation.

Every packed field must pass store pack decoding and the exact serde type.
Trailing wire data, an unknown field, invalid SurfaceId, invalid revisions,
wrong activity wrapper, operation or byte limit, and UiPatchOps placement
failure are a turn fault. No skipped operation, default value, or zero surface
fallback is correct. Build the whole UiPatchOps first, then call the existing
one-patch UiTurnPatches admission
([kernel owner](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1760>)).

Do not mint a host receipt or compare it to a host actor id. The component
reactor owns receipt issuance: it takes one pending patch, gets its guest
lifetime receipt, stages it, and commits it only after output preparation
([guest turn](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:800>)).
The native host must only run ActorUiPatchReceipt pairing validation: zero
patches require no receipt and one patch requires one structurally valid
receipt. A random valid receipt grants no host authority; it merely cannot
retire a guest slot it does not name.

### Rejection has to return to the guest

By host conversion time, the component reactor may have committed its pending
receipt. Turning a valid output capacity failure into an ordinary TurnFault
pins that guest slot: ShardLoop publishes an ordinary fault without adding
PatchRejected ([fault branch](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1834>)).
The actor transport independently closes a refused patch owner and returns an
error ([transport bridge](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:180>));
that has the same liveness failure.

For a fully decoded patch with a valid pair, retain one exact
PendingPatchRejection containing receipt, surface, revision, and reason inside
the retained GuestInstance. The next granted call prepends its PatchRejected
event before external events; clear the slot only after its input conversion
accepted it. This covers fixed owner exhaustion, byte or operation refusal,
and actor transport refusal. It is one per actor because a successful poll
issues at most one patch.

Malformed or unpaired output has no trustworthy rejection reference. It must
take the existing typed guest-close cursor so guest pending state retires; raw
GuestInstance drop is not sufficient. This is distinct from ordinary renderer
rejection, which already has a valid published receipt and follows the normal
PatchRejected event path.

### Minimal laws

1. One native law passes a returned WIT patch containing every one of the
   eleven variants and a valid receipt through the shared inverse, then through
   the existing renderer apply and ACK path.
2. One emitted-only patch follows the same route. Returned plus emitted, two
   emitted calls, two returned patches, receipt without a patch, and patch
   without receipt each fault with no renderer publication and no sink growth.
3. Malformed or trailing pack bytes, a wrong typed payload, activity wrapper,
   bad surface, operation cap, and byte cap leave no UiTurnPatches owner and
   advance no renderer revision.
4. Saturate the 64 turn-owner and transport slots. A valid decoded patch
   retains and delivers exactly one rejection before a later ordinary event,
   then the same component can issue a replacement. A malformed result instead
   enters typed close and releases every guest and host owner.
5. The existing ignored process-shard component smoke is the closest actual
   harness ([process smoke](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️.rs:687>)).
   Its scale fixture currently has the obsolete poll shape and omits current
   turn-result fields, so it is not a bridge qualification. Update it to the
   current WIT and add one ui-bridge profile returning one valid packed
   UiNodeRecord Upsert with paired receipt. A real wasm32-wasip2 run must
   assert one single-claim actor transport token and the exact decoded patch.

## Current Kernel activation reservation audit

Static review only; this is not a native qualification or an assertion about
the unavailable Dt0329 capture.

### What the landed foundation already gets right

The reservation itself is non-Clone and its authority field is private
([reservation](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎠️activation/🦀️.rs:34>)).
The exact authority is additionally checked by pointer identity before either
bind or abort. Reservation overflow is checked before pin, scheduler, or actor
mutation, and actor collision is checked before those mutations as well
([reserve](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎠️activation/🦀️.rs:59>)).
The current native fixture exercises exhaustion, collision, wrong shard, stale
authority, inactive tick suppression, and immediate error cleanup. The
scheduler entry is deliberately inactive and the unbound-mailbox trace
deliberately permits bounded pre-bind buffering. Neither is a scheduling
bypass: Scheduler tick filters on active state.

ActorShardKey is currently a trusted host data value, not a wire capability. It
must stay inside the owned coordinator and ShardExecutor handoff; its public
fields do not establish physical admission. This does not resolve the separate
actor allocation and transport nonce frontier: a raw ActorId can still be
re-used after an aborted colliding per-package ordinal, so delayed wire frames
must carry the checked allocation identity before reaching a shard.

The immediate non-cancelled façade failures have the intended local owner
order: missing shard and instantiate failure abort the Kernel reservation;
registration refusal drops the returned instance before abort
([shared façade](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🦀️.rs:19>)).
The executor also closes a receiver-lost admitted registration rather than
leaving a live instance in the shard
([receiver-loss branch](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:643>)).

### P0 — cancellation can split a reservation from its owners

install_actor is a by-value async future. Dropping it at an await drops the only
KernelActivationReservation; it has no retained cursor or Drop path that can
mutate the borrowed Kernel.

- Cancellation during instantiate leaves the pinned, inactive scheduler row
  and Activating ActorMeta created by reserve.
- Cancellation while ShardExecutor registration awaits leaves the
  registration sender and guest in its fixed executor ring. The executor can
  close that guest when its reply receiver is gone, but it cannot abort the
  Kernel reservation.
- Cancellation after physical admission, while bind_activation awaits shard
  lookup or Scheduler activation, can leave the guest registered while the
  Kernel is still reserved, or after metadata has consumed the reservation but
  before Scheduler activation.
- Cancellation during abort is independently unsafe: it awaits scheduler
  unregister, then shard unpin, then ActorMeta removal
  ([abort](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎠️activation/🦀️.rs:103>)).
  A cancellation between calls loses the reservation and retains partial
  Kernel state.

The owned coordinator must retain complete state, not merely hide current APIs:

    Reserved(reservation)
    -> Instantiating(reservation, compiled, caps, budget)
    -> Registering(reservation, guest, registration-ticket)
    -> Binding(reservation, admitted-shard-allocation)
    -> Active(active-lease)

Every cancellation or fault moves into ClosingGuest when a guest exists, then
an owned resumable Aborting(reservation, phase) cursor. Its phases are
Scheduler removal, pin release, and metadata/link finalization. Each poll
returns the same cursor on yield or fault; it never drops an owner. A bind
rejection similarly retains physical allocation and reservation, unregisters
or closes the guest first, then runs the same abort cursor. Do not retain a
mutable Kernel borrow in a detached future: the future coordinator must own the
core and serialize these transitions.

### Current pre-bind transition holes

The planned guards for complete, suspend and resume, exclusive placement, and
extension linking are necessary. In current source, resume turns an Activating
reservation into Scheduler-active without a physical guest
([resume](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4868>));
complete can write a failure or quarantine state that bind then overwrites to
Active ([complete](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4813>),
([bind](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🎠️activation/🦀️.rs:88>)).
An exclusive request also changes Scheduler placement away from the immutable
reservation shard, so later bind necessarily rejects.

Two additional concrete guards are needed:

1. apply_scene_patch accepts an Activating actor and directly mutates its
   window SceneStore ([scene patch](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:4898>)).
   Require a bound active lease, so a host cannot render before the same guest
   is physically admitted.
2. set_capabilities updates the future parent capability set while a
   reservation is current
   ([capabilities](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️.rs:5029>)).
   Require the same bound state. Otherwise a pre-bind actor can become an
   extension-capability parent without an admitted runtime.

Until link rejects a current reservation at either endpoint, abort_activation
also needs the same final link scrubbing as cascade_remove; it presently
removes only scheduler, pin, and ActorMeta. The cleaner permanent rule is to
reject link, suspend-cascade, resume-cascade, set-capabilities, scene
publication, complete, exclusive request or release, deactivate, and kill for
every current reservation. Then abort cannot own a graph or active side effect.
The public coordinator should expose no raw Kernel mutator capable of bypassing
that rule.

### Required exact laws

1. A reserved actor receives bounded mailbox input and cannot grant, render,
   gain capabilities, link, suspend or resume, complete, or move exclusive;
   every denied call preserves the exact reservation and mailbox.
2. A faulted pre-bind completion cannot be rebound out of quarantine.
3. Cancel independently at instantiate pending, registration pending,
   admitted-before-bind, bind-after-metadata-before-Scheduler, and each of
   the three abort phases. Re-drive to terminal and assert zero ActorMeta,
   Scheduler row, ShardTable pin, shard allocation, guest instance, and
   pool-use leak.
4. A registration reply receiver loss, bind refusal, and coordinator job
   refusal preserve the same reservation and guest-close cursor for later
   bounded retry; no panic or raw instance drop.
5. Preserve the existing overflow and collision no-partial-mutation checks.
   After abort and reuse, an old allocation-stamped frame is rejected before
   it can name the successor.

### Correction — actual suspension points and executor wake

The broad bind and abort cancellation wording above is too strong for the
current source and is superseded by this finding. The awaited Kernel helpers
in the reservation path are syntactically async but have no current Pending
source: ActorKind tag, saturation sampling, ShardTable pin, unpin and lookup,
Mailbox construction, and Scheduler registration, unregistration, and active
flag mutation are all in-memory computations. In particular, bind's lookup
and active write, and abort's three mutations, complete in the same poll under
the current implementations. They are not independently observable
cancellation boundaries merely because their callers use await.

The real current cancellation P0 is install_actor's externally controlled
guest instantiate and its queued registration oneshot
([instantiate and register](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🦀️.rs:28>)).
On a pending registration, the executor retains the instance after receiver
loss, but NativeKernelRuntime and ParallelRuntime only discover and drop that
unclaimed instance from a later foreground tick
([native tick cleanup](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs:101>)).
They cannot retire the matching Kernel reservation. The coordinator needs a
retained installation owner for that real two-sided suspension; it need not
pretend that today's pure Kernel mutations are asynchronous close cursors.

The nonterminal registration acknowledgement is currently sent while
ShardExecutor state is locked: run installs a retained drive and polls it
under the same mutex
([current poll](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:643>)).
The drive may call oneshot send before its first real guest poll. A waker may
re-enter executor APIs, so moving that poll outside the mutex is the correct
repair.

The move must be a single-poller handoff, not a temporary loss of the drive:

1. Under state mutex, take exactly one drive into a local Polling owner and
   record its unchanged drive generation; leave an explicit polling state so
   register queues a successor rather than creating another drive.
2. Drop the mutex, poll that same pinned future once, then re-lock and restore
   either the exact pending drive or its returned ShardLoop. Assert no second
   drive was installed and preserve registrations appended during the poll.
3. Keep scheduled true until restoration. A synchronous registration waker may
   schedule, but cannot create a concurrent poller. The existing generation
   wake remains valid only for the same restored drive.
4. Apply terminal ingress state after restoration and outside callback
   delivery. A receiver-loss admission must still unregister or drop its exact
   guest, never overwrite a successor registration.

Add a normal-admission analogue of the existing terminal wake test: park a
registration, use a Wake probe whose wake calls state.try_lock and queues a
second registration, then drive the first acknowledgement. It must observe an
unlocked mutex, one allocation for each actor, no duplicate poll, and exact
first/second ownership. Also add a stale generation wake after the first drive
has completed; it may not poll or replace the successor drive.

### Shutdown prerequisite for the retained close coordinator

The coordinator must be opened with and retain one WorkerPoolUse until every
active guest and unclaimed registration has reached its terminal close. Pool
shutdown already returns Busy while that use exists and leaves the pool Open,
so the close work can still run
([pool lifecycle](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1905>)).
Releasing the use before close is terminal is unsafe: shutdown stops
maintenance/deferred wakes and joins workers, while the current façades only
post fire-and-forget ShardFrame Unregister and do not remove the corresponding
Kernel record ([facade unregister](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs:120>)).

Thus begin-close must first fence activation and ingress, retain its
Kernel/shards/runtime/pool-use, and drive exact unregister or guest-close
acknowledgements. Only after every Shard allocation and instance, queued
registration, executor drive or terminal handoff, Kernel actor/pin, and
coordinator command row is absent may it drop the last use. The host then
retries WorkerPool shutdown. The WGPU façade needs the same terminal predicate
through its caller-owned pool; neither target may use a foreground tick or a
raw runtime Drop as close completion.

## Public Space Collaboration and Administration — Command Authority Fence

### P0 — an author can append a directory event after durable demotion

The public `POST /directory/commands` route resolves the bearer and calls
`authorize_directory_command` before entering
`execute_directory_command_receipt_fenced`
([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4783>),
[pre-fence authorization](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4790>)).
The current fence is absent for every command except `RemoveMember`; that one
only locks the *target* member's binding
([selection](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4731>),
[execution](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4766>)).
It never revalidates or reauthorizes the actor while fenced.

This is authorization-significant because `DirectoryService::decide` is
explicitly structural only and trusts the caller's actor
([contract](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1605>)).
Its idempotent execution holds the service write lock, appends events, records
the receipt, and only then publishes
([pipeline](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1963>)); the
write lock cannot repair a stale authorization decision made before it.

Exact trace:

1. A is an `Author` of space S. A submits `RemoveMember { S, B }` (the same
   issue applies to rename, visibility, member upsert, invite, revoke, and
   document announce).
2. A passes the outside `get_role(S, A)` check.
3. C durably changes A to `Spectator` through the ordinary `UpsertMember`
   command path.
4. A acquires only B's membership gate and calls `execute_idempotent`; its
   already-authorized actor is trusted, so `MemberRemoved { S, B }` becomes a
   durable accepted receipt after A is no longer an author.

This is not a Shell optimistic-UI flaw. The Shell maps the seven frozen
`os.directory.*` actions into the canonical command wire
([mapping](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1091>)),
and folds a directory mutation only from an accepted receipt's persisted
events ([receipt handling](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1819>)).
The worker also suppresses a receipt after its captured session or worker epoch
has changed ([transport settle](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:3158>)).
The server is the authority gap.

### Smallest coherent repair

Keep `DirectoryService`'s deliberate caller-authorization contract. Put one
linearizable authorization fence in Hub instead.

1. Add a private `SocketBindingKeyV1::DirectorySpaceAuthority { space_id }`.
   It is not a document write lock. It serializes authority-changing directory
   commands and scoped/document grant admission for the same space. Existing
   `Membership { user_id, space_id }` already protects one member's grant
   admission, while `DocumentWrite(DocumentScope)` serializes actual document
   mutation submission; neither covers a whole space's directory authority.
2. Add the space key to `socket_record_bindings` for `Document` and
   `DirectoryScoped` audiences. This makes their current sorted/deduplicated
   `acquire_record` sequence ([implementation](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:820>))
   share the same fence as a role/revocation command. Global directory sessions
   have no space and do not take it.
3. Make a command binding builder return a sorted, deduplicated vector:
   `User(actor)`, `Session(actor session)`, `DirectorySpaceAuthority(S)`, the
   actor `Membership(A,S)`, and any affected target membership binding(s).
   `SocketBindingGatesV1` needs a vector acquisition method rather than nested
   calls. It must acquire in `SocketBindingKeyV1` order, just as record
   admission does, or a grant/command pair can deadlock.
4. `post_directory_commands` must retain exact bearer/session evidence,
   acquire that vector, then re-resolve the session and call
   `authorize_directory_command` **inside** the guards, immediately before it
   creates the durable claim and invokes `execute_idempotent`. `CreateSpace`
   takes only user/session evidence; the remaining commands derive S
   canonically from the command. An administrator remains authorized by the
   active admin principal, but still takes the space gate when the command can
   invalidate scoped connections.
5. After a newly accepted execution—not `Existing` receipt replay—derive all
   affected `MemberUpserted`, `MemberRemoved`, archive-demoted, delete, and
   relevant space events from the returned durable events and invalidate their
   exact membership bindings before releasing the fence. The current code
   invalidates only one `RemoveMember` target after every execution. Role
   downgrade otherwise leaves a prior scoped/document grant live until its
   next authority check. The existing grant path already revalidates under the
   membership gate ([issue](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2181>)),
   so this extends the same ownership rule instead of adding a second
   revocation protocol.

The event append and receipt completion remain wholly owned by
`DirectoryService::execute_idempotent`; Hub only owns admission, the held
authority locks, and post-append live-grant invalidation. Do not move
authorization into `decide`, which has callers such as pre-audited system
operations with a different trust boundary.

### Deterministic proof and existing execution seam

`TestLiveGate` already contains a target-membership pause used by
`scoped_directory_socket_removal_and_delivery_have_one_total_membership_order`
([test](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10528>)).
Add a distinct test-only checkpoint **after the first REST authorization and
before command-fence acquisition**, rather than reusing the target fence:

1. A's REST `RemoveMember(S,B)` reaches that checkpoint after the old
   authorization read.
2. C's normal REST `UpsertMember(S,A,Spectator)` runs to an accepted durable
   receipt under the new space fence.
3. Release A. Its inside-fence reauthorization must return `403`; the
   directory head and B membership must be unchanged from step 2, and no A
   completion receipt may exist.
4. In the converse ordering, hold A after its inside-fence reauthorization;
   C must wait. A may complete once; C then demotes A. The A scoped/document
   socket closes `4401`, and a post-demotion write/grant request is denied.
5. A role-downgrade case must prove the existing B scoped socket is invalidated
   and the observer receives exactly the persisted demotion event once. A
   member removal case must retain the same property for B.

The existing real SQLite process journey already exercises actual two-user
membership, a peer's live event, scoped-socket close `4401`, and post-removal
administration denial
([process journey](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11600>)).
Extend that fixture/process driver after the native interleaving law; it is the
right peer-visibility regression, but its present sequential remove does not
cover the TOCTOU. Registered direct commands (not run here) are:

```text
bun ./📜️script.ts space-journey-check --source
bun ./📜️script.ts space-journey-check --process
```

from [the Hub Rust package script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11805>).

### Related observation, deliberately not folded into this P0

`record_sync_session_open(...).await.ok()` can leave a live document handler
unrecorded and therefore unavailable to an administrative connection inventory
([call](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4305>)).
Its socket still has per-frame and tick revalidation, so this is an operational
revocation/observability P1—not a path to append a write after membership loss.
It should be a later fail-closed-or-retained-recording operation, not a reason
to dilute the command-fence repair.

### Addendum — Complete Membership Writer and Visibility Fence Census

The first packet correctly identified the REST command P0, but a
space-authority key only repairs it if all membership writers and all scoped
visibility/admission paths share its ordering. The current source has the
following production paths.

| Path | Current authority/serialization | Migration needed |
| --- | --- | --- |
| REST directory command | Resolves/authorizes once, then only `RemoveMember` locks target `Membership` ([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4783>), [fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4766>)). | **P0:** acquire and revalidate exact user/session/space authority before the idempotent claim and append. |
| Admin directory intents | `execute_admin_intent` uses `execute_directory_command_fenced` for existing spaces, but `CreateSpace` calls `execute_create_space_with_id` directly ([dispatch](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6355>)). The endpoint checks the principal before execution, not inside a command fence. | Existing-space admin commands must take the same space key and final-revalidate the exact admin session. `CreateSpace` has no pre-existing scope, but still needs a final user/session admin recheck before its direct append. |
| Invite redemption | Public route calls `DirectoryService::redeem_invite` directly, with no binding gate ([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5203>)); the service atomically changes membership under only its directory writer ([service](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2092>)). | **P0:** introduce verified capability-to-space preflight, then user/session/space fence and final reauthentication before `redeem_invite`. Invalidate the redeemed member binding from the returned event. |
| Self logout/admin session revocation | Self logout locks its exact `Session`; admin revocation locks `User`, then invalidates each session grant/plan ([self](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5833>), [admin](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6412>)). | No space-key write migration: session invalidation already removes all indexed pending/live grants and plans. Preserve these subset locks; never acquire a user or session *after* a space key. |
| Scoped/document socket grant, consume, live delivery | All static scope paths use `acquire_record`, then revalidate ([grant](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2181>), [consume](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3759>), [live](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1479>)). | Add the space key to the existing record-binding projection for `Document` and `DirectoryScoped`; these sites migrate automatically. |
| Global directory socket delivery | It acquires only the global record's user/session keys, then checks the individual message space and awaits the send ([send](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5589>), [visibility](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5493>)). | **P0 privacy race:** acquire a union of the record keys and `DirectorySpaceAuthority(message.scope)` for a scoped event/connection/presence/rebootstrap, revalidate and decide visibility inside it, and retain it until send finishes. A global socket remains live after leaving one space, but cannot send a frame whose membership check lost to that space's removal. |
| Document-open plans and execution-target reads | Each protected document-plan/target path already acquires the record before revalidation, e.g. issue at [2301](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2301>), exchange at [2418](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2418>), target at [2491](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2491>). | Automatic through the record-binding projection. They subsequently take `DocumentWrite` only after record guards, preserving one order. |
| Normal document commands/checkpoint publication | Document WebSocket admission is record guards then `DocumentWrite` ([command](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4007>)); both checkpoint paths are record guards then `DocumentWrite` ([publisher](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3267>), [HTTP](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3483>)). | Automatic through record binding. Do not add a second membership check after `DocumentWrite`; the held scope guard is the linearization point. |
| GIS inference/approval artifact commit | The Hub builds an `InferenceRouteContextV1` with only an `Arc<DocumentWrite>` ([context](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6966>)). Approval checks Author before `commit_approval` ([runtime](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1888>)), while the retained committer later acquires/retains the document-write authority. | **P0 ingress bypass:** every inference route must retain a lexical session+space authority guard for the entire runtime call; especially approval must hold it through commit. The committer stays auth-free and receives only the already-authorized document-write owner—do not duplicate Hub membership authority in WGPU's retained committer. |

#### Exact key order and ownership boundary

Add `DirectorySpaceAuthority { space_id }` to `SocketBindingKeyV1` between
`Session` and `Membership`. Its derived ordering then gives the required order:

```text
User → Session → DirectorySpaceAuthority → Membership → Share → DocumentWrite
```

`SocketBindingGatesV1` needs one `acquire_bindings(Vec<SocketBindingKeyV1>)`
that sorts/deduplicates **before** it awaits any `lock_owned`; it must not hold
the internal weak-gate map mutex while waiting. `acquire_record` and the new
command/message helpers are thin callers of that one primitive. The optional
`DocumentWrite` lock continues to be obtained after the returned authority
guards, not inserted into a reverse nested sequence.

This is deadlock-free for the observed production paths if these rules remain
true:

1. Directory command and invite paths acquire gates first, then enter
   `DirectoryService`'s writer. `DirectoryService` must never obtain a socket
   gate while it holds its writer; it only appends and broadcasts
   ([idempotent pipeline](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1966>)).
2. Socket/document/checkpoint paths acquire record/space keys first and only
   then `DocumentWrite`. The new inference wrapper follows that same order;
   the current bare inference `DocumentWrite` is the one order-breaking
   bypass.
3. Self logout's `Session`-only lock and admin bulk revocation's `User`-only
   lock are prefix subsets; neither waits for a later space key. They can stay
   as they are.
4. A post-append invalidation runs while the relevant space guard is still
   held. It never calls back into command execution. Waiting socket work wakes
   afterward and revalidates, rather than publishing from a stale observation.

For REST member commands, retain the `SessionCapability` already owned by
`AuthedUser` and compare session id, user id, authorization generation, and
expiry after the gates are acquired; then rerun the role/owner matrix. For
admin commands, `AdminPrincipalV1` currently keeps session identity and
generation but not the capability needed by `authenticate_session`
([principal](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1892>)).
Make the private request authority retain that capability, or add a private
server-side exact-principal revalidator; a check only at
`admin_intents`' pre-execution point ([6497](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6497>))
does not establish the fence's linearization point.

#### Invite-specific prerequisite

The invite token does not carry a public scope. It is unsafe to lock a
client-provided space, and taking the service writer before the new space gate
would invert the command order. Add an internal
`invite_redemption_scope(capability)` read which validates selector and secret
digest and returns only the stored space id or the existing generic denial.
Implement it in the `HubDirectory` contract and the dispatch plus all three
first-party backends:

- [directory trait and enum dispatch](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2512>)
- [SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:801>)
- [Postgres](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:765>)
- [Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:516>)

The route performs that bounded preflight, takes `User/Session/Space`,
reauthenticates the bearer and repeats the capability-to-space check, then
calls the existing atomic redemption. Its returned `InviteRedeemed` event
names the actual user and space, which is the sole input for post-append member
binding invalidation. The atomic backend claim remains the duplicate/revoke
authority; the fence supplies cross-path linearization only.

#### Current readers outside the static grant fence

`GET /directory/events` filters a durable batch without a binding gate
([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5486>));
the canonical event-page route revalidates its session but evaluates each
space visibility while building the page, without a per-space guard
([builder](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5330>)).
They are not membership writers and do not invalidate the command repair. They
must not be claimed automatically protected by it, however: a later read-hardening
slice needs a bounded final visibility witness for every included space, or a
per-message/page scope guard, before returning bytes. The global WebSocket
send is the immediate P0 because it already has a concrete check-to-await-send
window and is part of the live peer path.

#### Focused native/process laws

In addition to the REST A/C/B interleaving in the prior section, add:

1. An invite redemption paused after scope preflight: a normal author revokes
   the invite under the same space fence; resume must append neither
   `InviteRedeemed` nor membership. Reverse order yields exactly one event and
   grants only after its member invalidation/space fence release.
2. A global directory socket paused after it has the message but before its
   per-message scope admission. Removal wins; require no frame and the global
   socket stays usable for a different authorized space. Send-wins is allowed
   only when it completes before removal acquires the scope key.
3. An approval paused after route scope admission but before runtime author
   check: a demotion wins and requires a denied response, no committed WAL
   event, no checkpoint, and no inference approved ledger event. The inverse
   must choose one total order rather than running an unfenced commit.
4. Existing [space journey process driver](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11600>)
   should add role downgrade (not only removal): it must observe the event once,
   close or deny the downgraded author socket as appropriate, and show no Shell
   success without the accepted durable receipt.


## GIS Approval — Retained Committer Versus Session/Space Fence

### Current concrete race

The production-configured route is direct HTTP ([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7054>)); its context has only a raw DocumentWrite mutex
([construction](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6992>)).
approve_gis_map_job does one Author lookup
([call](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1968>),
[lookup](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🛂️authorization/🦀️.rs:10>)),
then awaits base materialization, durable approval-prepared, assembly,
journal, verifier, and publication. It owns no User → Session →
DirectorySpaceAuthority → Membership admission during that work.

The lower retained state proves why a lexical HTTP check is insufficient:
GisMapDocumentWriteAuthorityV1 retains only an Arc Mutex
([definition](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:115>)),
while Ready, Assembly, Journal, Verification, and Publishing retain it with
only actor/document facts
([states](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:240>)).
No state contains a user, session, authorization generation, Author role, or
space admission. Thus A can pass the author read; C can commit normal
UpsertMember(...Spectator) or RemoveMember; and A can still reach the sole WAL
event/checkpoint. DocumentWrite does not serialize directory membership.

Cancellation has a separate, source-proven liveness loss. The ledger inserts
a durable prepared outbox row before calling the committer
([write](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:492>)).
Normal cancellation refuses while that row is prepared
([guard](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:450>)).
The retained core law deliberately returns Storage, keeps DocumentWrite locked,
and succeeds only on a same-owner retry
([law](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2285>)).
If that HTTP future is dropped and A is revoked, the current source provides
neither a typed Pending owner nor a maintenance/cancel cursor: the prepared
row cannot be normally cancelled and the retained document write may block
the document.

RetainedGisMapApprovalCommitterV1.close has only a unit-law call in the
current source census, so this is not a claim that a deployed server currently
auto-publishes after revocation. It is unsafe as a future generic teardown
driver, though: close invokes verify on Verification
([close](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1272>)),
and close_turn cancels then may acknowledge a Journal owner
([journal](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1198>)),
without receiving authorization authority.

### Smallest coherent repair

Introduce a private, one-slot-per-document HubGisApprovalCursorV1 rather than
placing Hub types in the Store/DB committer. It owns exact session facts,
scope, required Author role, approval command/base/outbox identity, and the
ordered Hub admission. The route takes and final-revalidates:

    User → Session → DirectorySpaceAuthority → Membership → DocumentWrite

before the first lower advance. The lower committer stays session-free. Never
re-enter a Hub membership/space gate from a publisher holding DocumentWrite;
that reverses this order.

The cursor needs one-way Authorizing → DurablyDecided state. The boundary is a
verified CommittedInferenceWalWitnessV1, not a raw journal receipt. Before
that witness, HTTP drop, user cancel, or revocation drives an abort-only close
cursor: it must not acknowledge a new event, publish a checkpoint, or leave
the prepared row/write lease behind. After it, immutable decision provenance
is retained and recovery may finish checkpoint/ledger reconciliation after
session expiry or restart. A generic close therefore needs distinct
pre-witness abort and post-witness recovery modes.

### Required laws

1. Pause A after final scope revalidation but before the journal turn. Let C
   execute the real REST demote/remove. If C wins: A is denied, with no Event,
   checkpoint, approved row, prepared row, or retained DocumentWrite. If A
   wins: C waits; exactly one Event/checkpoint/approved row occurs before C's
   durable demotion.
2. Drop A before a verified witness. Drive the retained abort cursor and
   require the same no-event/no-prepared/no-write-lease terminal state.
3. Drop A after the verified witness but before publication. Restart/close
   recovery completes that exact checkpoint and reconciliation once, without a
   second event.
4. Hold a normal document command's DocumentWrite, start A after scope
   admission, then start C's membership writer. Completion must show the
   forward order above and no DocumentWrite → Space wait.

Use TestLiveGate in [Hub tests](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:530>)
for the A/C order, and extend the retained publisher-control test at
[runtime](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2231>)
for exact witness, checkpoint, outbox, and terminal-owner observations. The
current Hub fixture uses UnavailableGisMapApprovalCommitterV1, so route-only
laws cannot substitute for retained production-state laws.

### Admission-cursor cutover addendum

The proposed private `Arc` admission is coherent only as a capability kept in
one runtime-owned document slot, rather than as a cloned route value. It needs
to contain the exact `DocumentScope`, approval actor, session id, user id,
authorization generation, expiry, required `Author` role, immutable
job/proposal/mutation/base identity, and the `OwnedMutexGuard`s acquired in
the existing `SocketBindingKeyV1` order:

```
User -> Session -> Membership(user, space) -> DocumentWrite(scope)
```

That is the order induced by `SocketBindingKeyV1`'s `Ord` and the current
record-admission helper ([keys and acquisition](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:676>)). A fresh
`socket_session_binding`/Author comparison has to occur only *after* those
guards are held. The public HTTP future must never receive the `Arc`: it
inserts a private `HubAdmittedGisApprovalCursorV1` into a bounded per-document
runtime slot before its first lower await, and only gets a progress/receipt
view. That prevents a dropped, never-polled handler from silently dropping a
prepared approval or guards. A failed first scheduling attempt must reject
before `prepare_approval`; after slot insertion, a runtime-owned maintenance
drive owns every terminal path.

There are three materially different cancellation phases, not two:

1. **Before any journal receipt**: abort may cancel Assembly/Journal and
   release the admission, document write, and prepared outbox only after the
   lower close confirms no receipt. Revocation winning this phase returns
   denied/cancelled and must leave no WAL event or checkpoint.
2. **Receipt, but no verified witness**: this is already potentially
   irrevocable. `drive_turn` receives `AwaitingAck(exact_receipt)`, stores it,
   and immediately calls `owner.acknowledge()`
   ([actual sequence](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:842>)). Therefore a request cancellation here
   cannot claim the no-event result. It must retain the lower owner/receipt
   and drive the verifier. It may reach a rejected physical receipt, but it
   must not blindly use the generic `close_turn` acknowledgement path.
3. **Verified `CommittedInferenceWalWitnessV1`**: the durable decision is
   authoritative. Session/role guards may be released *at this point* if the
   cursor retains the immutable witness, exact DocumentWrite lease, stores,
   and publication/reconciliation cursor. A request cancellation now only
   detaches the response; it cannot cancel recovery/publication.

Releasing authorization at a raw WAL receipt is unsafe: receipt matching is
only made authoritative by the verifier's exact transaction/document/event
scan. Releasing at the verified witness is safe with respect to the original
user decision, but not a terminal handoff: the current publisher performs
reservation, both stages, both read-backs, and durable publish in one awaited
`publish_candidate` call
([orchestrator](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:435>)). The owner must retain that in-flight/retryable
publication state, or make its CAS reservation recoverable, through both
checkpoint publication and `ledger.reconcile_committed_approval`. The latter
is a distinct durable side effect and the current committer releases
DocumentWrite only after it ([verify tail](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1120>)). Do not release the
write lease after WAL receipt merely because public delivery rechecks
descriptor/current/snapshot: that revalidation
([publisher](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3359>)) is a stale-checkpoint fence, not user/session authorization.

The minimal adversarial laws are: (a) dropped before first slot poll makes no
outbox/WAL/checkpoint and frees all four gates; (b) drop after prepare but
before receipt drives a confirmed abort and frees the outbox/write lease; (c)
drop after `AwaitingAck` but before verify drives verification, never reports
cancelled, and either safely rejects physical state or reaches one witness;
and (d) revocation after witness but before CAS publish permits exactly one
system checkpoint plus exactly one reconciliation, with no second WAL event
and no retained document gate. Each must observe a competing normal document
write is unblocked only after the appropriate terminal phase.

### Cross-space invite revoke is an authorization bug

This is independently exploitable. The REST/admin authorization checks the
client-supplied `DirectoryCommand::RevokeInvite { space_id, .. }` space
([Hub matrix](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4714>)); the decider likewise only requires that space before calling
`revoke_invite_as(invite_id, ...)`
([decider](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1674>)). The backend API omits space entirely
([trait](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2518>)). SQLite, Postgres, and Neo4j then update/match by invite id alone
([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1509>),
[Postgres](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1572>),
[Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1377>)).

An Author in space X who knows an invite id for Y can submit
`RevokeInvite { space_id: X, invite_id: Y }` and revoke Y's invite. Id
entropy reduces discovery but does not supply authorization.

The narrow repair is a breaking internal contract change:

```
revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id)
```

Every backend transaction must atomically predicate both stored invite scope
and id (`WHERE id = ? AND space_id = ?`; Neo4j properties likewise) for the
update *and* accepted-status probe. A scope mismatch returns the same generic
not-found result as an absent invite, and audit records the authenticated
space. Do not prelookup then revoke: that reintroduces a TOCTOU relation
check. Update the default method, enum dispatch, all three implementations,
and the command call site together.

Add one shared backend law: create X/Y, issue Y's invite, authorize an X
Author through the real command route, attempt `{X, Y-invite}`, require a
generic denied/not-found result and an active Y invite that still redeems;
then authorize Y's Author, revoke it, and require redemption failure. This
is a security test, not a fixture-only selector.

### Current ingress-interface delta

The lower runtime now has a public
`GisMapApprovalIngressAuthorityV1` trait and retains its `Arc` in
`GisMapCommitIdentityV1` ([interface](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:117>),
[identity](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:242>)). Its checks bind scope/user/session/nonzero generation
to the server actor ([preflight](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:410>)), and the receipt holds the Arc through
successful reconciliation ([tail](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1111>)). This is useful operation
correlation, but cannot itself be the Hub authorization proof: it is a public
trait with a test implementation, and no production bin implementation or
held-gate construction is present in the current source census.

Keep the actual authorization capsule private to the Hub binary/runtime slot.
Pass the lower runtime only a bound identity witness; rename the public trait
accordingly if possible, so callers cannot mistake trait construction for
authority. The private capsule owns the ordered guards and revalidation; the
lower layer only verifies that the supplied actor/job operation matches it.
This avoids an impossible cross-crate `pub(crate)` seal while preserving the
real security boundary.

There is still no automatic drive for the existing retained document state.
On a publication `Storage` outcome, the source law explicitly requires a
same-owner retry while DocumentWrite remains held
([law](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2367>)). The proposed slot must therefore own a concrete scheduler
ticket/maintenance callback. Retaining only the `Arc` in the `documents` map
does not satisfy the dropped-request path: it preserves the lock and prepared
outbox but makes neither progress nor abort.


## Directory Command Authority Corpus and Global-Frame Ordering Review

### The three new authority laws exercise the intended real boundary

The first native law pauses after the existing initial REST authorization
([route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4723>)), then applies a real
owner demotion/removal or direct session revoke before resuming the command.
The unfenced current route goes directly from that pause to the durable
idempotent command pipeline, so its expected 401/403, unchanged head, retained
target role, and independently claimable request id are genuine intended-red
assertions—not a fixture-only simulation. The request ids used by all three
laws are exact 32-character lowercase hex values.

The second law is likewise a real ordering test once the planned fence calls
the two existing test pause phases correctly: `pause(..., false)` before
admission and `pause(..., true)` after all authority guards are acquired. It
then proves the owner demotion waits, the original RemoveMember gets one
accepted receipt and event, and the demotion appends second. It is expected to
time out on the current source because the route currently calls only the
`false` pause; that is a valid TDD red, not a test defect.

The third law correctly proves pending and live grant invalidation for the
affected `(user, space)` binding and leaves global/other-space/other-user rows
alone. Replaying the same receipt after minting a fresh grant is a useful
check that an idempotent receipt does not invalidate a newly issued authority.
It deliberately tests the ledger, not a socket send; it cannot replace the
global-frame privacy law below.

Two small corpus/test fixes are still needed before treating the neutral file
as exhaustive:

1. Its schema only sets array minima; duplicate or substituted case ids can
   remove `session-before-command` or `command-before-demotion` without the
   TypeScript oracle failing. Assert the exact four case-id set and the exact
   five binding-id set, once each, in `proveDirectoryCommandAuthorityV1`, and
   have the Rust tests select named rows rather than merely `appended == 0/1`.
2. The success law's `timeout(100ms, revoking)` is a timing assertion. The
   pre-fence `directory_command_attempted` signal already establishes that the
   second request reached its admission attempt. Add one post-admission
   semaphore (or reuse a named authority-acquired signal) before the owner
   waits for the held membership key; then assert the durable head remains
   unchanged until release. This removes scheduling-speed dependence without
   weakening the total-order proof.

The native selector registration is correctly split: only these three laws
run under the SQLite/Postgres/Neo4j headless `os-hub` group
([script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10031>)). No runtime success is claimed here.

### Minimal nonflaky global directory-frame privacy law

Global directory sockets are a distinct hole: their normal send path first
takes only User/Session record authority, then reads membership in
`socket_directory_message_visible`, and finally sends a frame
([path](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5533>)). A membership-changing command can currently land between that read and
the write. The fix needs a message-derived authority helper, e.g.
`socket_live_directory_frame_authority(state, record, live_id, message)`, that
extracts the message's space, obtains sorted
`User -> Session -> Membership(user, space)` guards, then revalidates the
session and membership while they are held. It must hold them through the
single WebSocket text send. There is no DocumentWrite in a directory-frame
delivery.

Use the existing `TestLiveGate` and `DirectoryService::publish`, not a second
`AnnounceDocument`: announcing the same descriptor can be idempotent. The
deterministic process law is:

1. Create owner A and member B in one private space, issue B a **global**
   `/directory/socket-grants` grant, open `/directory/socket/v1?since=head`,
   await `socket_directory_admitted`, then release
   `socket_directory_release`.
2. The handler currently subscribes only after that release. Wire the already
   present but unused `directory_subscribed` / `directory_release` test
   semaphores immediately after `DirectoryService::subscribe`; await the
   subscription signal before publishing. Without this, the pre-existing
   scoped-order test can lose its broadcast because `socket_directory_release`
   starts at zero.
3. For **removal wins**, enable the existing membership-removal gate and start
   A's real `POST /directory/commands` RemoveMember(B). Wait until it holds B's
   membership fence. Configure the send test pause before the frame helper's
   message-scope admission, call
   `directory_service.publish(DirectoryStreamMessage::Presence { space_id,
   document_id, actors: ... })`, await the sender pause, release sender then
   remover, and require B's next WebSocket item to be close 4401 with no text.
4. Re-add B, make a fresh grant/socket, and repeat with the pause **after**
   message-scope admission. Publish another Presence, wait for the pause, start
   the real removal command, prove it is waiting via the post-admission signal,
   release sender, require exactly that Presence text, then let removal finish
   and require close 4401.

The frame is a real service broadcast and contains membership-only telemetry;
the test should use `next_close_without_authority` in removal-wins mode so a
text frame is an immediate failure. This establishes one total order without
depending on a duplicate durable document event or a sleep.

### Correction: current command ordering and global-directory socket semantics

The preceding two recommendations are superseded by the current source.

1. `directory_command_authority_holds_admitted_command_until_receipt_before_demotion`
   does **not** use `timeout(100ms, revoking)`. After the original author has
   reached the fenced pause, it consumes the owner's pre-fence
   `directory_command_attempted`, starts the owner demotion, consumes the
   demotion's own pre-fence `directory_command_attempted`, and immediately
   asserts `!revoking.is_finished()` ([law](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11512>)). The held author fence makes that immediate
   assertion causal: the demoter cannot reach durable execution while the
   original command is paused. No additional timing assertion or semaphore is
   required for this law.
2. Removing a member from *one* space must **not** close their global
   `SocketAudienceV1::Directory` socket. Its grant deliberately binds only
   User and Session, not a single Membership
   ([binding construction](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:787>)); the periodic live check consequently remains active after a
   one-space removal ([session-only revalidation](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2210>)). `4401` remains correct only when that
   Session/User authority is revoked. A revoked-space frame must be skipped,
   not converted into a socket close.

The focused global-frame law should therefore use two private spaces, `A` and
`B`, and one still-active member/session `M` in both. Open a global directory
socket for `M` with `since = head`, wait for its existing
`socket_directory_admitted` point, release it, and add the already-declared
test-only subscription handshake immediately after
`DirectoryService::subscribe` ([handler](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5679>)). Waiting for that handshake prevents publication
before the `broadcast` receiver exists.

For the **removal-wins** trace, pause the `Presence(A)` send *before* the new
message-derived `Membership(M,A)` frame admission, use the real
`POST /directory/commands` RemoveMember command to remove `M` from `A`, wait
for its normal receipt, then release the paused send. The frame path must
revalidate under the same `(M,A)` key and return `SkipUnrelated`; it must
neither write a text frame nor close the global socket. Immediately publish a
distinct `Presence(B, "barrier-1")`. Because one receiver processes the
`DirectoryService` broadcast in publication order, the next WebSocket text
being exactly `Presence(B, "barrier-1")` is a deterministic no-leak proof:
an `A` frame would necessarily have preceded it. It is also a liveness proof
for the still-session-authorized global socket. Publish a second distinct
`Presence(B, "barrier-2")` and require it next to rule out a deferred close
after the first barrier, without a sleep or a close-frame expectation.

For the converse **delivery-wins** trace, restore `M` in `A` and use a fresh
global grant/socket. Pause only *after* the frame helper has acquired and
revalidated `Membership(M,A)`, publish `Presence(A, "admitted")`, then start
the same real removal. The second `directory_command_attempted` plus immediate
`!removal.is_finished()` is the correct ordering witness here: the command is
waiting on the admitted frame's exact membership key. Release the frame,
require `Presence(A, "admitted")`, await the removal receipt, and then require
`Presence(B, "barrier-3")` on the same socket. This proves the sole allowed
order without treating membership removal as session revocation.

The implementation seam is `send_socket_directory_message`
([current global branch](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5582>)): add a helper that derives the message space and holds sorted
`User -> Session -> Membership(M, message-space)` through session/member
revalidation and the one `send_directory_message` call. For global audience,
failed membership maps to `SkipUnrelated`; only failed User/Session/live-ledger
revalidation maps to `CloseUnauthorized`. Do not add every membership to the
global grant's static bindings: that would both change its intentionally
multi-space audience and make its binding set unbounded.

## Checkpoint Send Correction, Fresh Archived-Space Grants, and Approval Drop Cutoff

### The current checkpoint handler's `Send` failure is not a fence guard

The exact compiler receipt was `exact-cargo-laws-Znbtxy/00`. It identifies the
awaited VCS callback type at
[trusted catalog apply](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:321>), not an HTTP or directory mutex:

```rust
dyn Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>>
```

has no native `Send` bound. That future reaches
[checkpoint materialization](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🦀️.rs:500>) and then the explicit
`Pin<Box<dyn Future + Send>>` response at
[the handler](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3440>). The current checkpoint fences are instead
`tokio::sync::OwnedMutexGuard`s, acquired at
[3457–3464](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3457>) and
[3268–3271](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3268>), which are `Send`.

The appropriate narrow repair is the target-specific first-party
`ArtifactCodecApplyFuture`: native targets require `+ Send`, while `wasm32`
keeps the local future. WGPU has applied that boundary; this audit does not
claim its rerun passes. Do not weaken the handler's `+ Send` requirement or
move a synchronous directory guard across an await.

### Archive is readable; author invite redemption is the real lifecycle gap

The preceding archive-denial recommendation was wrong and is superseded.
`archive` is intentionally a readable spectator Space: `ArchiveSpace` first
emits spectator upserts for every author and then the archive event
([decider](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1610>)). A fresh
archive socket grant returning `Active { role: Some(Spectator) }` is therefore
correct. The existing membership-only `socket_session_binding` queries are not
by themselves an authorization bypass. Deletion is also currently projected
with deletion of Space memberships/invites/descriptors—for example SQLite
[729–737](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:729>) and
PostgreSQL [658–666](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:658>).

The concrete P0 is a pre-archive **author invite**. Invite redemption checks
only `space_exists` in the common preflight
([1036–1055](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1036>)); each
backend therefore accepts a still-valid `Author` invite after archive and
projects `InviteRedeemed { role: Author }`. That bypasses the normal
`UpsertMember` archive rejection at
[1647–1650](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1647>) and
recreates a writer in an otherwise read-only archive.

Change the backend-independent preflight input from `space_exists: bool` to a
transactionally read `SpaceInviteAdmissionV1 { Missing, Archive, Writable }`,
or equivalent `space_kind`. A pending author invite is denied for `Archive`;
a pending spectator invite remains redeemable. Fetch the Space `kind` under
the same claim transaction/serialization boundary as the invite in SQLite,
PostgreSQL, and Neo4j—not with a route pre-read. A deleted Space remains
`Missing`, so its now-deleted invite cannot redeem.

Add three schema-first vectors to
`🎟️invite-redemption-transaction-v1`, then run each actual backend
implementation:

1. Archive after an author invite but before redemption: no marker, no
   directory Event, no Author membership.
2. Archive after a spectator invite: exactly one spectator redemption and a
   fresh document socket grant/open plan is read-only.
3. Delete after either invite: fresh redemption and fresh socket grant deny,
   with no descriptor/plan/body.

The route law must verify the archived author's fresh grant now carries only
Spectator and cannot write, rather than expecting an archive grant denial.
This preserves the established mixed archive/read contract while blocking the
one remaining author-creation path.

### Retained GIS approval: current ingress fix and remaining pre-witness drop P0

The previous pointer-identity ingress deadlock is superseded by the current
source. `GisMapCommitIdentityV1` no longer contains ingress
([252–265](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:252>));
`commit_retained` clones ingress only locally for the lifetime of the request
future ([1145–1163](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1145>).
The exact retained-committer law at
[2406–2428](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2406>)
already demonstrates a post-WAL publication refusal releasing the first
ingress and a fresh ingress completing the same retained publication. This is
source evidence only, not a native Hub acceptance receipt.

There is nevertheless an owner/liveness P0 when an HTTP future is dropped
*before* the durable receipt:

1. `prepare_retained_document` obtains and parks the exact
   `GisMapDocumentWriteLeaseV1` before it returns
   ([632–740](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:632>).
   Dropping the request immediately after that await leaves `Ready` with a
   parked writer.
2. Dropping while `commit_retained` awaits preflight leaves `Ready` with
   `pending: Some(identity)`. A same request can currently re-enter this
   phase, but an abandoned request holds the writer indefinitely.
3. Dropping at the explicit `yield_now` between assembly/journal turns leaves
   the exact three Store owner and writer in `Assembly` or `Journal`.
   A future request cannot even resume that owner: prepare maps either state
   to `Conflict` ([679–691](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:679>).
   `close()` is the only current autonomous driver, and it is server shutdown,
   not request cancellation.

The lower owner already exposes the right no-drop resolution: `close_turn`
calls `cancel(); advance()` for Assembly and a Journal without a receipt
([1216–1260](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1216>),
and the Store commit transitions cancellation to a retained abort before a
real journal receipt. The Hub must put that cursor behind a per-document
runtime-owned maintenance/admission slot, rather than leave it owned by an
HTTP future:

- Before `AwaitingAck`, request cancellation marks the exact retained cursor
  cancelled and schedules bounded `cancel/advance` turns until all three Store
  owners are handed back. Only then drop `DocumentWrite` and report the
  proposal unapproved/cancelled.
- If cancellation races `AwaitingAck`, acknowledge the exact receipt and
  switch irreversibly to verification/publication/reconciliation. Do not
  cancel or replay the committed Event.
- After that cutoff, release User/Session/Space/Membership authority. Retain
  only the DB witness, exact Stores, DocumentWrite, and a maintenance ticket;
  a worker-owned turn, not an HTTP retry, must advance publication. A later
  response/delivery freshly revalidates the caller and can be denied after
  revocation without reversing system recovery.
- Runtime shutdown must retain and drain the same cursor; it cannot discard a
  journal owner merely because its initial request future vanished.

The planned outer approval wrapper must **not** lock `DocumentWrite` before
calling `commit_prepared_approval`. The committer itself needs to acquire the
raw gate at [707](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:707>)
and retain that lease through the cutoff; an outer lock would self-deadlock.
It should instead hold the sorted User, Session, Space and Membership gates,
then the runtime document gate, and hand the raw DocumentWrite `Arc` to the
admission cursor. The installed production route still calls
`approve_gis_map_job` with the plain context at
[6993–7001](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6993>);
the private guarded ingress is not yet mounted there.

Required deterministic laws, in addition to the existing post-WAL
publication-error/retry law:

1. Pause after each of `prepare`, preflight, one Assembly turn, and one
   Journal-no-receipt turn; drop the HTTP owner; drive the admission slot;
   require one cancellation/abort, all three original Stores and DocumentWrite
   released, no WAL Event, and a different next request admitted.
2. Pause exactly as journal returns `AwaitingAck`; cancel/drop; require exactly
   one committed Event and one eventual checkpoint/reconciliation under the
   maintenance owner, never an abort.
3. After the receipt but before publish, revoke the Session/member. Require
   background recovery to complete exactly once, a fresh caller response to be
   denied, and no retained User/Session/Space/Membership gate. This proves the
   durable cutoff rather than incorrectly coupling repair to a revoked request.

### Scoped `RevokeInvite` backend repair: current verdict

The new `HubDirectory::revoke_invite_as` contract correctly carries the
authorized Space down to each persistence operation
([trait](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2508>),
[closed-set dispatch](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:3265>),
[decider call](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1664>)).
Each backend now scopes both the mutation and the accepted-marker diagnosis:

- SQLite: [1498–1504](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1498>).
- PostgreSQL: [1567–1580](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1567>).
- Neo4j: [1371–1388](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1371>).

Consequently a foreign caller cannot turn a correctly authorized command for
Space A into a revoke of an invite belonging to Space B. The new real SQLite
HTTP law at [11566–11592](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11566>) is well-shaped: it first observes `404` and unchanged foreign
row, then allows the owning-space author to receive `202` and mutate that row.
It also correctly asserts no directory event is appended: capability
revocation is intentionally not event-sourced.

The source oracle has a useful independent SQLite assertion: it extracts the
current first-party update and executes it against a fresh Bun SQLite database
([script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10145>)).
This is neither a PostgreSQL nor Neo4j behavioral test. The native group
enables all three Cargo features, but this particular Hub law uses `test_state`
and therefore supplies SQLite behavior only. Keep the honest distinction.

Two bounded additions remove remaining coverage ambiguity:

1. Have the source gate inspect the PostgreSQL update/diagnostic query and the
   Neo4j `MATCH (SpaceInvite {spaceId, id})` plus diagnostic `MATCH`, requiring
   their same exact `space_id`/`spaceId` predicate. This is a fail-closed
   topology assertion until those server backends have live fixture facilities.
2. Add `foreign-accepted-invite` to `inviteScopes`: an invite already accepted
   in B, revoked through an authorized command for A, must still return `404`,
   must not expose its accepted state, and must not change B. The current
   scoped diagnostic should satisfy it, but the existing two rows exercise only
   a pending foreign invite.

No defect was found in the current scoped update/rollback order itself: each
backend begins its transaction, performs the exact-space write, diagnoses only
the same-space accepted marker, writes its audit row only after a successful
change, and commits last. This review did not run those paths.

### Invite redemption: capability-derived scope and archive state — P0

`invite_redemption_preflight` presently receives only `space_exists: bool`
([common decision](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1036>)).
Each concrete claim transaction consequently asks only whether the stored
invite's space exists: SQLite at
[1447–1450](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1447>),
PostgreSQL at
[1481–1489](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1481>),
and Neo4j at
[1283–1293](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1283>).

That is a direct policy bypass. `ArchiveSpace` changes the persisted kind to
`archive` ([SQLite projection](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:726>))
and the decider refuses an Author membership in that state
([1647–1654](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1647>)).
But a pending Author invite is not routed through `decide(UpsertMember)`;
`redeem_invite_atomic` emits `InviteRedeemed { role: Author }` directly after
the bool preflight. Thus an Author invite issued while writable can be
redeemed after archive and restore a writer. This is independent of the new
`RevokeInvite` repair.

#### Closed, backend-owned state

Replace the bool with a non-wire, closed common type:

```rust
pub(crate) enum InviteRedeemSpaceStateV1 {
    Missing,
    Writable,
    Archived,
}
```

Put the only raw-kind conversion next to the common preflight. `atelier` and
`studio` map to `Writable`; `archive` maps to `Archived`; no row maps to
`Missing`; any other stored kind is `DirectoryError::Backend`, not a writable
fallback. The conversion must receive the **stored `InviteRecord.space_id`**,
never a path/query/body scope.

Each existing atomic transaction already has its serialization point and
should replace its `EXISTS` lookup with `SELECT kind` in that same transaction:

| Backend | Existing fence | Required scoped lookup |
| --- | --- | --- |
| SQLite | `TransactionBehavior::Immediate` | `SELECT kind FROM hub_space WHERE id = ?1`, bound to `invite.space_id` |
| PostgreSQL | locked directory-event head plus `SpaceInvite … FOR UPDATE` | `SELECT kind FROM hub_space WHERE id = $1`, bound to `invite.space_id` and read before the claim update |
| Neo4j | `DirectoryCounter.claimNonce` update plus locked invite mutation | `MATCH (s:Space {id:$space_id}) RETURN s.kind AS kind`, bound to `invite.space_id` before the claim mutation |

Do not add a `space_id` parameter to `redeem_invite_atomic`; that would turn a
server-derived invariant back into caller authority. The event-head/counter
fences already serialize archive/delete projection with redemption, so the
state decision and the marker/event/membership write remain one transaction.

The common ordering should be:

1. Locate the selector, constant-time check the secret, and validate the
   `DirectoryActor`/user binding.
2. Reject `Missing` before returning a historical acceptance. This makes a
   deleted space fail closed even if a corrupt leftover invite row exists;
   normal delete projection already removes its rows
   ([SQLite](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:729>),
   [PostgreSQL](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:658>),
   [Neo4j](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:370>)).
3. Preserve marker-integrity verification and accepted-marker handling. An
   exact same-user acceptance on `Archived` returns the existing immutable
   event, writes no event/membership, and does not republish. The archive
   event has already demoted current Authors to Spectator; replay must not
   change that current role.
4. For an unaccepted record, retain the existing user/revoked/expired
   precedence, then reject `(Archived, Author)` as generic `Unauthorized`.
   `Archived + Spectator` may claim normally. Future/unrecognized role values
   remain impossible through the typed record decoder.

The deliberate `AlreadyCommitted` rule is therefore preserved precisely for
an archive, not broadened to deletion. It already validates the linked event
against the record and same user at
[1058–1082](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1058>).

#### Hub fence without client scope authority

`POST /directory/invites/{token}/redeem` currently authenticates once and
calls `DirectoryService::redeem_invite` with no Hub fence
([5245–5250](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5245>)).
It needs a server-only, capability-verified **scope hint** solely to select
the correct gates before the service writer:

```rust
pub struct InviteRedemptionScopeHintV1 { space_id: String }

impl DirectoryService {
    pub async fn invite_redemption_scope_hint(
        &self,
        capability: &InviteCapability,
        actor: &DirectoryActor,
        user_id: &str,
    ) -> DirectoryResult<InviteRedemptionScopeHintV1>;
}
```

Back it with one new closed `HubDirectory` lookup implemented by SQLite,
PostgreSQL, and Neo4j. It must locate the stored record by selector, do the
same constant-time secret and actor/user checks, and return only its immutable
stored `space_id`; it is neither an authorization result nor a redemption
lease. The HTTP route never serializes the hint and maps every invalid hint to
the same generic denial. The final `redeem_invite_atomic` repeats all
capability, user, role, state, marker and claim checks under its real
transaction, so revoke/archive/delete after the hint cannot be accepted on a
TOCTOU path.

After obtaining the hint, the route must acquire, in the existing sorted
`SocketBindingGatesV1::acquire_bindings` order
([844–852](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:844>),
the exact keys:

```text
User(user_id), Session(session_id),
DirectorySpaceAuthority { space_id: hint.space_id },
Membership { user_id, space_id: hint.space_id }
```

`DirectorySpaceAuthority` already exists at
[690–697](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:690>);
no new client-visible scope field is necessary. The Membership key is an
absence-or-presence serialization key, not a precondition: a first redemption
has no membership. Under User/Session gates, reauthenticate the exact original
session capability and compare user id, session id, expiry and authorization
generation with the captured `AuthedUser`, then call the service. Acquiring
these gates before the service's global writer matches the directory-command
order ([4797–4805](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4797>)
and avoids a reverse writer→gate cycle.

#### Required neutral and native matrix

The current transaction corpus has no archive state: its schema/type and
interpreter only model `spaceExists`
([schema](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🧬️.schema.json:61>),
[neutral model](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11118>)).
Extend it schema-first with an internal fixture `spaceState` of
`writable | archived | deleted` and an explicit final membership role of
`author | spectator | none`; it is not request input. Keep the hostile
client-space/client-role rows and additionally reject client attempts to
supply `spaceState`.

| Vector | Initial facts | Expected result and durable state |
| --- | --- | --- |
| `archived-pending-author-denied` | writable pending Author invite, archive before redeem | `unauthorized`; marker null, zero `InviteRedeemed`, zero new membership/publication; current invitee remains `none` |
| `archived-pending-spectator-commits` | writable pending Spectator invite, archive before redeem | one new `InviteRedeemed(Spectator)`, one Spectator membership/publication |
| `archived-accepted-same-is-readonly-replay` | same user accepted Author while writable, then archive demotes to Spectator | `already-committed` with original event id; no appended/published event and final current role remains Spectator |
| `archived-accepted-other-conflicts` | other user accepted before archive | `conflict`, no mutation or publication |
| `deleted-pending-denied` | pending invite then `SpaceDeleted` | `unauthorized`; no marker/event/membership/publication, and no lookup scope returned |
| `deleted-accepted-denied` | accepted invite then `SpaceDeleted` | `unauthorized`; historical directory event may remain, but no capability record, membership, response replay or publication remains |
| `archive-races-author-redeem` | pause author redeem after verified scope hint; archive through the production directory command; resume | `unauthorized`, zero redemption event; proves the final backend state check rather than trusting the hint |

Run every state-row through the neutral interpreter and the existing native
SQLite HTTP/Directory path. Add source topology assertions for exactly the
three `kind` queries/conversions until PostgreSQL and Neo4j fixtures are
available; the current all-feature native target alone is not a live
PostgreSQL/Neo4j acceptance receipt. The native archive rows must use actual
`ArchiveSpace` and `DeleteSpace` commands, rather than injecting `kind` or
deleting rows, so they prove the same projection and gate path used in
production.

The smallest executable native set is five laws, all through the current
`POST /directory/invites/{token}/redeem` route:

1. Issue an Author invite in a Studio, archive through the real directory
   command, then redeem: `401`, no `InviteRedeemed`, no membership and no
   publication.
2. Repeat with Spectator: `200`, exactly one Spectator event/membership and
   one publication.
3. Redeem an Author invite before archive, archive (which demotes the member),
   then replay the same token: `200` with the original event id, unchanged
   directory head/publication count, and current role still Spectator.
4. Issue, delete by the real command, then redeem: `401`, no event/membership
   and no scope hint. Repeat after a prior acceptance to prove delete never
   returns a historical acceptance through a retained invite row.
5. Add a test-only pause after the validated scope hint but **before** the
   sorted gates. Revoke the recipient session or archive/delete the hinted
   space via its real route, release the pause, and require `401` and no
   receipt/event. A sister pause after User/Session/Space/Membership gates and
   final reauthentication must make the competing archive/revoke wait, then
   let the already-admitted redemption commit once. This proves both the
   pre-fence revalidation and the total-order guarantee; it does not mistake a
   preliminary hint for a capability.

### Directory command native first-law stalls: corrected receipt history

`qFpFTU/00` was **not** a state-open timeout: it compiled, reached the first
HTTP law, and correctly exposed the pre-fence behavioral RED (`expected 403`,
`actual 202`). `9hMnsM/00` is the attempt that exceeded the 120-second
first-law budget; its retained authority-first sample is diagnostic evidence,
not proof of a scheduler defect. `a3SjWT` later ran the four authority laws
green. `hBARZM` subsequently passed five laws, including the seven
archive/deletion HTTP rows. The new `NcKpki` reproduction is the only observed
five-second `test_state()` open failure before the first scenario diagnostic.
No receipt currently supports relaxing that deadline or attributing either
timeout to the public directory fence.

Current source provides two bounded explanations which are materially weaker
than a scheduler bug:

- `test_state()` calls `Database::open_at` on the process singleton pool
  ([7824–7826](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7824>),
  [2757–2768](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:2757>)).
  Its first task is filesystem `BackendOpen` on `Lane::Io`
  ([8126–8133](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8126>)).
  That task recursively creates and synchronizes every ancestor of the unique
  temp root ([7162–7176](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7162>)); a hot I/O worker is consistent with
  slow directory durability, not a parked future without a driver.
- The current first law now uses `spawn_restartable_server` and
  `stop_recovery_server` for every denied row
  ([11579–11624](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11579>)).
  The former detached-server explanation is therefore superseded. The observed
  third `test_state()` open after two completed shutdowns remains unexplained;
  fixture cleanup is necessary hygiene but not a demonstrated cure.

The normal DB-I/O poll path closes the lost-wake window: it either observes a
terminal result while holding the slot or installs its waker under that same
slot lock ([4735–4774](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4735>)); a completing worker then takes and wakes that
waker after releasing the lock ([4018–4061](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4018>)). I found no contrary
source trace in `Database::open_at`.

#### Concrete cancellation and shutdown ownership gaps

The five-second timeout itself introduces a distinct real leak. `FsStorage::open`
registers a filesystem backend and obtains the copyable `DbIoBackendControl`
before awaiting `execute(BackendOpen)`
([storage.rs:8126](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8126)).
On timeout, `DbIoTaskOperation::Drop` cancels/enqueues the task
([4808–4828](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4808>),
but no `FsStorage` has been constructed. The only normal control-retirement
path is `FsStorage::Drop` ([8156–8160](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8156>));
the control is `Copy` and has no drop owner. Cancellation returns no
`DbStorageOpenRejected`, so it cannot retain/retry exact backend close. This
does not explain the third open *before that third timeout fires*, but it
contaminates retries and is a production cancellation leak.

There is a second, independent teardown observation. `Database::shutdown`
releases database authorities and its own `pool_use`, but does not close the
`storage` field ([8415–8487](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8415>)).
After the database is dropped, `FsStorage::Drop` only signals asynchronous
backend close; `stop_recovery_server` does not await that control's terminal
acknowledgement. The next fixture state can therefore race prior Closing
backends. This is not yet proof that it caused `NcKpki`, but it is the exact
unverified lifetime boundary shared by the completed first two rows.

The narrow storage laws are: (1) pause immediately after BackendOpen task
admission, cancel the public open future, then require the same backend slot,
credit and `WorkerPoolUse` to reach terminal before reuse; (2) run 65
filesystem `open_at → Database::shutdown → drop` cycles and require each
backend terminal/pool-use baseline before the next open. Do not mask either
with a longer test deadline. The first law needs a retained open cursor or
Drop-driven control retirement; the second needs shutdown to own/await the
storage terminal cursor (or expose one to its caller), not merely signal it.

### Global Directory Socket: Per-Message Membership Fence — P0

Global `SocketAudienceV1::Directory` records deliberately contain only `User` and `Session`: [`socket_record_bindings`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:788) adds `DirectorySpaceAuthority` and `Membership` only for Document/DirectoryScoped audiences. `send_socket_directory_message` therefore obtains only User/Session through its global delivery ([bin.rs:5639](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5639)), then `socket_directory_message_visible` reads U's current membership in A ([bin.rs:5539](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5539)), then writes the frame. `RemoveMember { A,U }` owns `DirectorySpaceAuthority(A)` and `Membership(U,A)` ([bin.rs:4718](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4718)) but does not conflict with that sender. It can commit between the role read and send, exposing one post-removal A payload. This is a real delivery-authorization race.

Do not invalidate/close a global record on an A Membership change: that would incorrectly close the same socket's still-authorized B stream. After completed removal, the current global decision correctly maps A to `SkipUnrelated`. Session revocation remains different and terminal: `delete_session_me` owns and invalidates `Session(id)` ([bin.rs:5883](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5883)), which is an indexed global-record binding and produces 4401.

Add a private `directory_stream_message_space` from typed server envelope fields only: `Event.event.space_id`, `Connection.connection.space_id`, `Presence.space_id`, and `RebootstrapRequired.control.scope.space_id`; heartbeat has none. For a global Session audience only, union `record.bindings()` with `DirectorySpaceAuthority { space_id }` and `Membership { user_id: record.subject.user_id, space_id }`. Do not lock an unrelated event's space for DirectoryScoped sockets; their record already owns their fixed scope. Do not use a websocket query/client frame as message scope.

Factor `socket_live_authority` into a private bindings-taking primitive plus its current record-only wrapper. Acquiring the union then calling today's helper self-deadlocks on the non-reentrant User/Session Tokio mutexes. The global sender must acquire the one sorted union, revalidate audience/session+live ledger, run message visibility, then retain it through the existing bounded two-second websocket send. Releasing after `get_role` recreates the leak; bounded send time is the necessary maximum delay of the conflicting membership writer. Apply the same union to [`send_socket_directory_rebootstrap`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5687): its selected scope governs role lookup, private control read and control send, so removal cannot interleave there either.

Keep `scoped_directory_socket_removal_and_delivery_have_one_total_membership_order` ([bin.rs:10554](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10554)) unchanged: an A-scoped lease correctly closes 4401 and cannot prove global semantics. Current global tests prove only admission/replay session revocation ([bin.rs:10631](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10631)) and basic delivery before session revocation ([bin.rs:10209](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10209)). Add separate schema-first `global-directory-socket-membership-order-v1` beside `scoped-socket-revocation-v1`, not a scoped fixture mutation. Neutral rows must express decision, close code, cursor advance, stream-live and a later-B witness: removal-A-wins => no A text/no 4401/no A cursor advance then B delivered; send-A-wins => exactly one A then removal, later A suppressed and B delivered; session-revoke-wins => no later text/4401; unavailable union/read => no text/1013.

The native law should use the real global grant/route, U member of A and B, and the real `POST /directory/commands` removal. Reuse `socket_membership_remove_*` ([bin.rs:4766](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4766)) and add global-send admitted/release semaphores **after union acquisition**. Removal-wins: hold writer A fence, release, verify no A frame, then B frame proves liveness. Send-wins: hold the union, prove removal waits, release, observe one A, complete removal, then B frame proves retained global liveness. Register the new law in [`SocketGrantCheckScript`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2977) and add a dedicated fixture interpreter next to [`proveScopedDirectorySocketRevocationFixture`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2936). Semaphores—not sleeps—are the ordering witnesses.

### Invite Redemption State Fence: Current Source Review

**Scope.** This is a read-only source audit of the newly landed
`InviteRedemptionSpaceStateV1` / capability-bound hint / HTTP fence path. No
focused cohort was run in this audit, so the current registered selectors are
source topology and fixture coverage, not a fresh runtime receipt.

#### What the landed boundary gets right

The hint is a private scheduling input rather than authority. Each backend
first requires that the selector names both a still-existing `Space` and the
authenticated `User`, then calls the shared verifier; the verifier compares
the exact `User` actor grammar, selector, and constant-time secret digest
before it can return its private space id
([shared verifier](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1052),
[SQLite query](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1437),
[PostgreSQL query](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1470),
[Neo4j query](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1272)).
The HTTP caller cannot provide a scope or role; it obtains that hint before
the sorted `User → Session → DirectorySpaceAuthority → Membership` union,
revalidates the exact original session while the union is held, and retains
all four guards through the service call and its newly-committed invalidation
([route](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5245)).
`Membership` is correctly only an absence-or-presence serialization key, so a
first recipient is not accidentally required to already be a member.

The shared preflight has the intended state ordering: missing/deleted space is
denied before replay; a complete acceptance marker returns
`AlreadyCommitted` before the archived-Author restriction; only a still
pending Author is denied by archive
([preflight](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1089)).
Thus an archived accepted Author receives the immutable historical event but
is not reprojected to Author. The replay event is checked against the exact
marker, invitation id, space, role, actor, user and recorded timestamp
([event verifier](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1112)).
Deleted-space scope lookup fails because every hint query requires the live
space; deleted projections remove invitation records as well
([SQLite deletion projection](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:733)).

The individual redemption transaction is coherent in all three adapters:
SQLite uses `BEGIN IMMEDIATE` and marker/event/projection/commit as one unit
([SQLite](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1446));
PostgreSQL holds the event head and invite row before state evaluation
([PostgreSQL](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1478));
Neo4j mutates the singleton counter and exact invite before reading state and
claiming ([Neo4j](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1281)).
`DirectoryService::redeem_invite` publishes only `NewlyCommitted`; the route
invalidates Membership/plan authority only in that branch. An idempotent replay
therefore cannot retire a fresh grant created after a prior acceptance
([service](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2136),
[route invalidation](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5259)).

The current seven concrete archive/delete rows and four after-hint ordering
rows cover the intended single-Hub HTTP policy, including accepted archive
replay, deleted denial, and the `admitted redemption → waiting archive`
ordering ([fixture](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json),
[HTTP law](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11708),
[after-hint law](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:11827)).

#### P0 — backend serialization does not cover an archive decision made by another Hub/service

The new four-key `SocketBindingGatesV1` union is process-local. It protects two
HTTP calls through one `HubState`, but it does not serialize two
`DirectoryService` instances over the same SQLite/PostgreSQL/Neo4j directory.
The backend redemption transaction is atomic by itself, yet archive is still a
read-then-later-append command:

1. Service B enters [`DirectoryService::execute`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1999), and
   `decide(ArchiveSpace)` reads the current member list and constructs the
   Author demotions before B has entered an adapter transaction
   ([decider](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1664)).
2. Service A, with a separate local writer/gate, runs `redeem_invite_atomic`
   for a pending Author invite. Its local state check sees `studio`, commits
   `InviteRedeemed(Author)`, and projects the new Author.
3. B then appends its stale precomputed demotions and `SpaceArchived`. It did
   not know about A's member, so the final projection is `Space.kind=archive`
   **plus A as Author**.

PostgreSQL's `hub_directory_event_head FOR UPDATE` and Neo4j's
`DirectoryCounter` mutation serialize the eventual appends, not the earlier
`decide` read; SQLite's `BEGIN IMMEDIATE` starts only in `append_events` after
the decision. Consequently none of those locks repairs the ordering above.
The same stale-decision family permits `UpsertMember(Author)` to decide under a
Studio then append after a concurrent archive. This violates the repository's
"archive ⇒ nobody-writes" invariant, rather than merely changing which HTTP
response wins.

The narrow durable repair is a backend-held **space-state revision fence** for
every space-state-dependent decision. `decide` returns its observed
`(space_id, state_revision)` with the events; each adapter conditionally
validates that revision under the same transaction which appends/projects the
events. Every event affecting `hub_space.kind` or membership, including
`InviteRedeemed`, advances it. A failed conditional append makes
`DirectoryService` discard the old decision and retry from `decide`; it must
not append a stale archive list or turn the mismatch into an accepted empty
receipt. PostgreSQL can compare a revision in the locked `hub_space` row after
the event-head lock; SQLite can compare inside its existing immediate
transaction; Neo4j can conditionally increment a `Space` revision in the
transaction that owns the `DirectoryCounter`. This is a single cross-adapter
contract, not a process-local lock or an invite-only workaround.

Add a real SQLite law with **two independently constructed
`DirectoryService`s over two connections to the same database**. Pause B after
the real `ArchiveSpace` decision has captured members but before its conditional
append; let A redeem a pending Author; release B. The terminal outcome may be
`archive → redemption denied` or `redemption → archive retries/demotes`, but
it must never leave an Author in an archive, and the final event order and
membership must agree with that outcome. Add the symmetric pending
`UpsertMember(Author)` versus archive trace. The existing
`directory_invite_redemption_admitted_fence_precedes_archive` law proves only
the in-process gate order, not this durable race. Mirror the law against live
PostgreSQL and Neo4j adapters when their facilities are available.

#### Bounded residual test and cancellation work

One small coverage gap remains even after the durable revision repair: after a
successful Author redemption, run the real `RemoveMember`, then replay the
same token. It must return the original event with no append/publication and
leave current membership `None`; the source already does this because only a
new commit calls `project`, but the existing archive replay row only proves the
Spectator-demotion variant.

For request cancellation, the current route has no spawned/detached redemption
owner: dropping the handler drops the pending service future and its local
writer guard. Before the transaction commits this is a normal transaction
abort; after a ready commit, the service synchronously publishes its
`NewlyCommitted` event before returning to the route, so a disconnected client
can lose a reply but cannot produce a half marker/event/membership state. The
post-commit `revalidate_directory_caller` may make that lost-reply case a 401
only if a writer bypasses the held Session gate. The production self-revoke
route does acquire that exact Session gate
([delete_session_me](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5902)); retain the rule that any future direct
session-revocation ingress must use the same key or the final revalidation is
not a response-only check. This is an integration constraint, not evidence of
a current public bypass.

### Archive Is a Compound Projection Event, Not a Stale Member Snapshot

#### Current projection/reducer census

`ArchiveSpace` currently reads `list_members`, emits one
`MemberUpserted(Spectator)` per then-current Author, and only then emits
`SpaceArchived` ([decider](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1664)). That member snapshot is the
cross-service race source.

There are exactly two shared read-model reducers that rely on those synthetic
demotions: the framework Rust fold at
[`directory/🦀️.rs:132`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs:132) (also used by Space and Home) and the TypeScript
fold at [`directory/🟦️.ts:165`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️.ts:165). Both only set
the space kind today. The Rust stream client only parses/sequences events, so
there is no third frontend reducer. `space.archived { spaceId }` already has
the required wire meaning; no schema field is needed.

The three durable projectors also only set the space kind: SQLite
[`project`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:706) line 726, PostgreSQL
[`project`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:655), and Neo4j
[`project`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:367). Those same methods run during
full rebuild ([SQLite](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2032), [PostgreSQL](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2211), [Neo4j](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2076)). The repair must be a
shared event semantic in all five reducers/projectors, not a live-Hub side
effect.

#### Recommended event-semantic repair

Make `ArchiveSpace` emit exactly one `SpaceArchived { space_id }` after
`require_space`: no `list_members` read and no generated MemberUpserted
demotions. Each reducer/projector folds that one event atomically by setting
the space kind to Archive, demoting every *current* Author in that space to
Spectator, and applying the event timestamp to the space.

For SQLite/PostgreSQL, update the known `hub_space` row and all matching
`hub_space_membership.role = 'author'` rows in the current append transaction.
For Neo4j, set the `:Space` and its `MEMBER_OF.role = 'author'` relationships
in the current `Txn`. The Rust and TypeScript shared folds need the equivalent
member map. Do not synthesize member events and do not run a post-append
demotion job.

An Author redeemed before that one archive event reaches projection is thereby
included in the archive transaction, regardless of what the archive decider
observed. Full rebuild produces the same all-Spectator Archive view.
`SpaceArchived` already invalidates broad `DirectorySpaceAuthority`, so
scoped grants/plans retire without redundant member events.

Regenerate the greenfield golden
[`events.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/⚡️events.json): it currently has two spectator
upserts immediately before the archive row. Remove those compatibility rows
and regenerate cursor/hash expectations together.

#### Final durable invariant, including system paths

Archive folding alone does not stop a stale `MemberUpserted(Author)` or
`InviteRedeemed(Author)` later in the log. Every projector must reject either
Author-bearing body if the current target kind is Archive, within the same
transaction that persists/projects the event. Spectator remains valid. This
is necessarily actor-neutral: System/seed paths can archive and demote, but
cannot add an Author later; the existing Archive creation path already emits
its owner as Spectator ([creation](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1739)). The special verified
invitation transaction needs the same guard even though ordinary
`append_events` rejects raw `InviteRedeemed`.

Require `SpaceArchived` to affect one current space. A zero-row archive update
after deletion must conflict, not persist a no-op event. SQL can condition an
Author membership upsert on a live `hub_space.kind <> 'archive'` row and
require one returned/affected row; Neo4j must require matching non-archive
`:Space` and a nonzero `MERGE` result. A rejection rolls back event, sequence
increment and earlier command events. Rebuild must reject an
Author-after-archive history as corruption rather than recreate writable
state.

#### Scope versus a full transaction-scoped backend decider

For Archive, this removes the unstable member snapshot. It deterministically
resolves concurrent Author work as either “before archive, then demoted” or
“after archive, refused,” and is the smallest correct cross-process repair for
`archive ⇒ no Authors`.

It does not make every existing command decision serializable:
`DirectoryService::execute` still decides before append
([pipeline](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1999)), so stale rename/visibility/archive-after-delete choices are a
separate general problem. A full backend command transaction or per-space
revision fence is appropriate only if that entire class must serialize; it
would have to carry authorization, idempotency and System pathways through all
three adapters. Do not require that broader refactor merely to repair archive
membership.

One narrow idempotency repair follows: `execute_idempotent` releases a receipt
after `decide` fails but propagates `append_events` failure while retaining the
claim ([lines 2020–2030](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2020)). A stale Author refusal would poison its request id. Return a typed,
rollback-proven `RejectedBeforeCommit` outcome and release the receipt only
for that outcome—never after an ambiguous I/O/commit result.

#### Exact two-service native law

Reuse the existing two-SQLite-connection pattern in
[`invite_redemption_sqlite_claim_is_exactly_once_across_concurrency_restart_and_rebuild`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4796), not a mock. Add a test-only post-`decide`,
pre-`append_events` semaphore in `DirectoryService`; no sleeps.

1. Pause B after `ArchiveSpace` decision. A atomically redeems a pending
   Author invite. Release B. Require durable order
   `InviteRedeemed(Author), SpaceArchived`, zero synthetic spectator archive
   upserts, Archive kind and no Authors. Reopen and rebuild must preserve it.
2. Pause A after `UpsertMember(Author)` decision for an existing user. B
   archives. Release A. Require a typed no-commit conflict, no extra
   event/member/Author, and—in the idempotent variant—a released rather than
   permanently Pending receipt. Rebuild retains the Archive view.

Mirror both rows against PostgreSQL and Neo4j where their native facilities
exist. The current in-process invitation gate law cannot prove either durable
cross-service ordering.

### Claimed Command Receipts Need a Rollback-Proven Append Refusal

`DirectoryService::execute_idempotent` releases a receipt after `decide` fails,
but propagates `append_events` failure with its receipt still Pending
([service](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2044)). A stale
Author command rejected by the archive projector therefore replays as the
current `Existing(Pending)` secret-undeliverable result instead of retrying its
decision, although it has appended no event.

`DirectoryError::Conflict` cannot safely trigger deletion. All three append
paths persist/project inside a transaction, then map commit errors to
`DirectoryError::Backend` ([SQLite](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1962),
[PostgreSQL](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2094),
[Neo4j](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1950)). A lost `COMMIT`
response may have committed. The current `?` path also only relies on
transaction Drop; PostgreSQL/Neo4j do not provide an awaited rollback proof on
that path.

Use a classified internal outcome, not a string match:

```rust
enum DirectoryAppendOutcomeV1 {
    Appended(Vec<DirectoryEvent>),
    RejectedBeforeCommit(DirectoryAppendRejectionV1),
}
```

Only the typed semantic projector guard (archived Author or missing target
space) may produce `RejectedBeforeCommit`. Each adapter must explicitly roll
back the same transaction and receive successful rollback confirmation before
returning it. A rollback error, driver error, or commit error stays the outer
`DirectoryError` and retains/reconciles the receipt. Apply the same explicit
rollback to the special verified invite transaction, so a rejected Author does
not retain its accepted marker. In rebuild, map that impossible event order to
a corrupt-log backend error.

`execute` maps the classified refusal to public Conflict. `execute_idempotent`
first releases its claim, then returns Conflict. Make that release claim-shaped:
match `actor_user_id`, `request_id`, **and `command_sha256`** while Pending.
The current SQLite/PostgreSQL/Neo4j releases match only actor/request
([SQLite](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1345), [PostgreSQL](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1281), [Neo4j](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1092)). Never
release after an ambiguous commit outcome.

#### Exact two-service retry law

The current `backendOrders` test uses two service writer locks but one
in-memory `HubDirectories` ([test](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4910), [fixture](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:3947)); it is not the two-connection backend proof.
Reuse the existing file-backed two-SQLite-connection setup:

1. pause A after `decide` for an **idempotent** existing-user
   `UpsertMember(Author)`;
2. B, on its own SQLite connection/service, commits `ArchiveSpace`; release A;
3. require typed no-commit refusal, unchanged event head, no Author, and no
   membership write;
4. arm A's decision fence again and submit the identical claim. It must reach
   `decide` (proving claim removal), then conflict under Archive. Existing
   Pending replay would bypass that fence.

Reopen/rebuild must retain Archive/no Authors. Add a separate commit-uncertain
injection negative control: it retains the receipt and never blindly reruns.

### Classified Append Audit — Current Archive Transaction Patch

Reviewed the current landed source, not the earlier design only. The new
`DirectoryAppendOutcomeV1` is a sound distinct command-lifecycle seam:

- [`HubDirectory::append_events`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2677)
  is now a one-way default adapter from required
  `append_decided_events`, and each concrete backend implements only the
  required classified method. `HubDirectories` forwards that classified call
  directly at [3512](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:3512).
  There is no default-method recursion or loss of the outcome before the
  idempotent service sees it.
- The SQLite `BEGIN IMMEDIATE` path
  ([1973](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1973)),
  PostgreSQL head-row update path
  ([2112](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2112)),
  and Neo4j counter transaction path
  ([1966](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1966))
  all read the projection state under their writer transaction, explicitly
  roll back before returning `RejectedBeforeCommit`, and leave driver/commit
  failures as outer errors. This is the right classification boundary.
- `project` repeats the guard for verified invite and rebuild paths, and the
  archive fold now demotes currently projected Authors atomically in all three
  stores. The normal invite preflight is also serialized through the same
  head/counter, so an Author redemption is either before archive and demoted,
  or denied after archive.

Two repairs remain required before claiming the receipt lifecycle is closed.

1. **Exact release must verify it actually released.** All three new release
   queries include the command hash, but discard their affected-row result:
   [SQLite](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1356),
   [PostgreSQL](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1293),
   [Neo4j](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1105).
   If a matching Pending record is unexpectedly absent (or is already
   completed), `execute_idempotent` currently returns the semantic conflict
   while silently retaining a possibly stale Pending owner. Require exactly
   one deleted row/node and return `Conflict` otherwise. Neo needs a
   transaction-scoped `DELETE ... RETURN count(r)` (with its result consumed
   and dropped before `commit`), not an auto-commit fire-and-forget `run`.
   The required law has two controls: wrong hash and Completed status both
   preserve the record **and** make release fail; the owned Pending hash
   deletes exactly one row and allows a fresh redecision.

2. **Rebuild must take the same physical head lock before snapshot/truncate.**
   The current rebuilds begin a transaction then count/truncate/replay without
   the ordinary append lock: SQLite
   [2011](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2011),
   PostgreSQL [2199](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2199),
   Neo4j [2067](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2067).
   Direct callers are allowed by the public trait, so a separate service can
   append while projections are being cleared. Start SQLite with `Immediate`,
   lock the PostgreSQL singleton head (`FOR UPDATE`) before the count, and take
   the Neo counter-node write lock before the count. Keep it through the
   success-only rebuild commit. Also map the defensive
   `DirectoryProjectionRejectionV1` during PG/Neo replay to `Backend` with the
   event sequence, as SQLite already does at
   [2047](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2047);
   a historical Author-after-Archive is corrupt history, not an ordinary
   retryable conflict.

`directory_projection_space_v1` currently omits `MemberRemoved`. If “missing
member target” includes every membership mutation, add it and a deleted-space
stale-removal law. If the intended contract deliberately permits a no-op
removal after deletion, document that narrow exception; it must not be
accidentally inferred from the Author-only archive rule.

#### Current physical SQLite law

The revised `invite_archive_projection_serializes_independent_service_decisions`
now correctly uses two `SqliteDirectory::connect` calls to the same unique
ticket-generated `directory.sqlite3` path
([test](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4908)). It drops both
services/backends before removing only its own database, WAL, SHM and empty
root; this replaces the prior in-memory false proof. Its semaphore is an
ordering boundary rather than a sleep. Keep this shape.

Extend that fixture/schema and its independent Bun oracle with the
idempotent stale-Author row: A claims and pauses after decision, B archives
through the second connection, then A returns `RejectedBeforeCommit` with an
unchanged head and released exact claim. Re-arm A's decision fence and retry
the same claim; it must enter `decide` again rather than return an existing
Pending receipt. Add the equivalent wrong-hash and non-Pending release
controls above. The existing archive/invite row proves durable ordering and
demotion, but not receipt reusability.

For the special invite path, the normal archive-Author case is denied before
the marker mutation under the same head lock. Its defensive `project` error
currently relies on transaction Drop in all three backends. That is not a
classified receipt outcome, but an explicit awaited rollback branch in the
PostgreSQL/Neo defensive path is still preferable before reporting marker
rollback as physically proved. The existing SQLite trigger law is useful but
does not qualify those remote adapters.

### P0 — Global Directory Frames Do Not Hold the Message-Space Authority

The global `/directory/socket/v1` grant deliberately has only its principal
keys (`User`, `Session`):
[`socket_record_bindings`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:799)
adds `DirectorySpaceAuthority` and `Membership` only for a document or
`DirectoryScoped` audience. That is right for the *long-lived ledger*. A
global stream must stay connected after removal from A and continue delivering
B; turning dynamic message bindings into record bindings would incorrectly
close the whole stream after its first A membership invalidation.

It leaves a concrete privacy race in the current per-message path.
[`send_socket_directory_message`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5669)
first calls [`socket_live_authority`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1521), which holds only the global record's
User/Session guards. It then calls
[`socket_directory_message_visible`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5569), which reads membership in the event's
space without `DirectorySpaceAuthority { A }` or `Membership { T, A }`, then
starts the bounded WebSocket send ([5711](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5711)).

An Author R removing target T from A obtains A's authority and T's membership
key through [`acquire_directory_command_fence`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4729), but it need not own T's User/Session
keys. Thus: (1) T's global socket observes membership in A; (2) R commits the
removal and invalidates T/A; (3) the already-authorized sender emits A's frame
after the removal. Because global records are intentionally not indexed by
membership, no live-lease notification closes it. The one-second tick cannot
repair this: a global `Directory` audience only revalidates its session
([`SocketSubjectV1::revalidate`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:726)). This is a P0 raw
event/connection/presence disclosure after removal.

#### Minimal non-reentrant union

Keep `SocketGrantRecordV1::bindings()` unchanged. Add a pure extractor:

```rust
fn directory_stream_message_space(message: &DirectoryStreamMessage) -> Option<&str> {
    match message {
        DirectoryStreamMessage::Event { event } => event.space_id.as_deref(),
        DirectoryStreamMessage::Connection { connection, .. } => Some(&connection.space_id),
        DirectoryStreamMessage::Presence { space_id, .. } => Some(space_id),
        DirectoryStreamMessage::RebootstrapRequired { control } => Some(&control.scope.space_id),
        DirectoryStreamMessage::Heartbeat { .. } => None,
    }
}
```

For a global session record only, copy `record.bindings()` and append the
message `DirectorySpaceAuthority` plus exact recipient `Membership`; leave
user-only events and Heartbeat as record-only. The existing
[`acquire_bindings`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:855) sorts/deduplicates into the compatible global order
User, Session, DirectorySpaceAuthority, Membership. It agrees with the REST
command union at
[`execute_directory_command_receipt_fenced`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4808), the system/admin command fence, and
invite redemption at [`post_redeem_invite`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5262).

Do **not** call current `socket_live_authority()` and then acquire the dynamic
keys: the first call owns User/Session and Tokio mutexes are non-reentrant.
Instead split it into one internal `socket_live_authority_with_bindings`:

1. acquire the complete sorted union once, with the existing two-second
   timeout;
2. under it, revalidate subject/audience, document-plan validity and exact
   live ledger id;
3. under the same guards, make the global visibility decision without another
   `revalidate` or gate acquisition;
4. retain the guards through the existing two-second `sender.send` future.

Keep the record-only wrapper for connection setup, tick, and lag close. For
global sends, a false membership decision is `SkipUnrelated`; inactive
session/live record remains 4401 and unavailability remains 1013. The
visibility helper should be a boolean/result separate from principal validity,
so removal from A suppresses A but does not convert it into a global close.

Holding the union until `send_directory_message` completes linearizes a
successful frame before a conflicting fenced mutation commits. If the timed
send errors, the current 1013 path remains. This does not assert an
application-level acknowledgement for a cancelled socket write.

#### Deterministic production-socket laws

Add a test-only **global** send latch entered only after the dynamic union,
revalidation, and `Deliver` decision, immediately before the send. Do not
reuse the scoped-only latch at
[`socket_scoped_send_mode`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:575); it cannot prove this global path.

Use recipient T in A and B, separate Author R in A, real
`/directory/socket/v1`, and the existing membership-removal gate:

1. **Removal wins:** pause production `RemoveMember(A,T)` after it owns its
   fence; publish A traffic. The sender waits for the union, observes removal,
   and skips A. Publish B and require B is the next frame and the same global
   socket remains live.
2. **Delivery wins:** pause a global A sender at the new post-union latch;
   begin removal and require it waits. Release send, require exactly that
   pre-removal A frame, complete removal, then require B on the same socket.
3. **Session distinction:** pause an admitted global send, begin
   `DELETE /sessions/me`, require it waits behind User/Session, release the
   send, then require 4401. Retain the converse existing replay law
   [`socket_directory_revoke_after_admission_suppresses_replay_without_deadlock`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10764).
4. Unit-test the extractor for scoped Event, user Event, Connection, Presence,
   RebootstrapRequired and Heartbeat, and assert the ledger record remains
   exactly User/Session after building the transient vector.

The static-scope proof at
[`scoped_directory_socket_removal_and_delivery_have_one_total_membership_order`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10690)
is useful but cannot cover this record-plus-message scope race.

### P0 — Admin Directory Mutations Lose Their Principal Fence

The normal `/directory/commands` route is correctly principal-fenced: it
passes User, Session and caller Membership into
[`acquire_directory_command_fence`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4808), then revalidates the exact bearer before
writing. The admin route has a different, concrete gap.

`admin_intents` authenticates its administrator, records an `accepted` audit
fact, then reauthenticates the header at
[`bin.rs:6577`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6577). After that point, the eight closed directory
intents—rename, visibility, archive, delete, upsert/remove member,
create/revoke invite—are converted by
[`admin_directory_command`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6112) and invoke
[`execute_directory_command_fenced`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6452). That call provides `Vec::new()`;
the generic helper therefore acquires only `DirectorySpaceAuthority` and, for
removal, the *target* Membership ([4729](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4729)). It never owns the administrator's
`User` or `Session` key. `CreateSpace` is worse: it calls
`execute_create_space_with_id` directly at
[`bin.rs:6436`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6436), with neither principal nor command fence.

An administrator's session can be revoked after the reauthentication and
before the generic space/target fence, then that already-revoked principal
appends a directory event. This is real for both session-revocation writers:
`DELETE /auth/sessions/me` locks Session only
([5913](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5913)), while an administrator's batch revocation locks User only
([6492](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6492)). Neither conflicts with the current admin command fence.

#### Narrow repair

Keep `execute_directory_command_fenced` principal-free: it also serves
trusted system actors, for which inventing a session fence would be wrong.
Factor its body into a private `execute_directory_command_under_fence` that
assumes a correctly acquired guard vector, then add an admin-only entry point:

```rust
async fn acquire_admin_directory_mutation_fence(
    state: &HubState,
    principal: &AdminPrincipalV1,
    command: Option<&DirectoryCommand>,
) -> Result<Vec<OwnedMutexGuard<()>>, AdminMutationAuthorityErrorV1>;
```

It builds `User(principal.user_id) + Session(principal.auth_session_id)`, and
when `command` is present delegates the same vector to
`acquire_directory_command_fence` so it gains the existing closed
space/target keys in **one** sorted acquisition. For CreateSpace it acquires
the two principal keys directly. With all keys held, require
`socket_session_binding(session, user, generation, None, now)` to return
`Active { role: None, expires_at_ms }` with the principal's exact expiry;
`Revoked`/`Expired` is a cancelled authority, and a backend/timeout is
unavailable. The principal was constructed only after the configured
provider/subject match at
[`authenticate_admin_principal`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2132); all three backends only mutate identity-bearing
session fields by expiry or revocation, so this id/user/generation/expiry
check is the narrow retained-principal revalidation without retaining the
bearer secret in an async operation.

Use that helper immediately before CreateSpace and before the mapped directory
command's durable call; retain its guards through event invalidation. Map a
revoked/expired result to a terminal `admin-authority-changed` cancellation
with zero directory events, and unavailability to a failed/unavailable
terminal. The prior `accepted` admin-audit fact may remain—it is not the
directory mutation—but it must receive that terminal fact.

Do not wrap all `execute_admin_intent` variants blindly. In particular,
`RevokeUserSessions` can target the caller and independently takes that target
User gate; an outer principal User guard would self-deadlock. Share/session
administration and asynchronous rebuild need their own scoped authority
decision. This repair is deliberately just CreateSpace plus the eight
space-command mutations that currently traverse the generic space/target
fence.

`DirectorySpaceRole` demotion is **not** an admin-principal revocation in the
present contract: `AdminPrincipalV1` is configured provider/subject authority
and `authorize_directory_command(..., true, ...)` bypasses space roles
([4669](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4669)). A test expecting a configured admin to become 403 after Author →
Spectator would specify a new revocable-admin model, not validate this fence.

#### Exact native route laws

Add a test-only hook after the existing header reauthentication but before the
new admin fence, and another after its exact revalidation but before the
directory service call. Exercise `/admin/api/intents`, not only the private
function.

1. **Session revoke before fence:** pause an admin Rename/Remove request at
   the first hook; call `DELETE /auth/sessions/me` with the same capability;
   release. Require a cancelled `admin-authority-changed` receipt, unchanged
   directory head and target projection, and no accepted directory event.
   Repeat for CreateSpace to cover its currently unfenced branch.
2. **Session revoke after fence:** pause after the new U/S+scope union is
   held; start `DELETE /auth/sessions/me` and require it is pending. Release
   the mutation, require its one event/terminal success, then require 204
   revocation. This proves the intended total order rather than a stale
   operation after a winning revoke.
3. **User gate:** with a second configured administrator, start
   `RevokeUserSessions` for the first admin while that first admin's mutation
   is at the post-fence hook. It must wait on User; after release it revokes
   and the audit/event order is exact. This complements the Session-only
   self-revoke row.
4. **Role semantics:** separately demote a configured admin from Author to
   Spectator before its fence and require the mutation still succeeds. That is
   the current deliberate global-admin contract; ordinary Author demotion is
   already covered by the existing command-authority fixture and must remain
   a 403/no-append row.

The current
[`scoped_directory_socket_admin_removal_uses_the_same_membership_fence`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10495)
proves only the removed target's static socket boundary. It does not exercise
the administrator's own User/Session authority across the final mutation
fence.

### P0 — GIS Approval Can Strand a Durable `prepared` Outbox Before It Has an Owner

This is a current production-flow ordering issue, not a problem with the new
Hub ingress guard. The approval route persists the ledger row first:
[`prepare_approval`](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2379).
Only later does `commit_prepared_approval` reject a base whose composed member
set is not exactly drawing/value
([2007–2010](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2007)),
and the retained committer itself calls `preflight(&request)?` before it
constructs `GisMapApprovalRequestOwnerV1`
([1468–1482](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1468)).

The request owner is the only path that schedules
`abandon_prepared_approval` on a pre-witness cancellation
([344–379](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:344),
[1201–1219](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1201)).
Thus either early refusal leaves the just-created row `phase='prepared'` with
no retained owner. The ledger prevents cancellation while such a row exists
([sqlite 448–471](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:448)),
and a same-body retry returns that same prepared row
([492–534](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:492))
instead of repairing it. A valid Map that is not the supported fixed-three
member shape reaches the pre-committer `CommitUnavailable` branch after
durable preparation.

Either run every pure composed-members, canonical-command, and committer
preflight check before `prepare_approval`, or create a request-local
**PreparedApprovalOwner** immediately after the ledger write. That owner
contains the exact reader, job, mutation, command/proposal hashes and ingress;
until the real WAL receipt cutover it must abandon or retain/retry
`abandon_prepared_approval` on every error/drop. Once `AwaitingAck` wins it
must be explicitly disarmed. A generic `Storage` error cannot be assumed
pre-witness because it is possible after durable receipt.

Add a deterministic route/runtime law with a server-materialized base that
passes ledger preparation but fails the composed-member/preflight path. Require
`proposal_state=Offered`, no pending outbox row (or an exact `abandoned` row
with empty command), successful cancel, and a same-owner retry that prepares
again. An injected fault immediately after `prepare_approval` needs the same
result. The existing abandonment law starts only after an owner exists.

### P0 — A Post-WAL Retry Can Become a Second Publisher

The ingress and document-writer ordering itself is sound: Hub acquires sorted
`User → Session → DirectorySpaceAuthority → Membership`
([7084–7098](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7084)),
then the runtime separately takes `DocumentWrite`; the production publisher
accepts that retained lease and does not lock it again
([3402–3412](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3402)).
That avoids the proposed self-deadlock.

It does not serialize **drivers** after the WAL receipt. A fresh same-identity
approval is deliberately accepted while the document is `Verification` or
`Publishing` ([runtime 756–767](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:756)).
Both `drive_turn` branches then return `Verify` ([987–997](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:987)).
More importantly, `verify` treats an already-`Publishing` state exactly like
the initial verification and sets `already_published = false`
([1407–1436](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1407)).
The dropped HTTP request's maintenance owner and an HTTP retry can therefore
both leave the document mutex and concurrently call
`publish_checkpoint`/`CheckpointPublicationOrchestrator`. The current
DocumentWrite lease is shared state, not a single-poller capability, so it
does not prevent this duplicate reserve/materialize/publish race.

Keep the useful “fresh request may join post-witness recovery” behavior, but
make it observational. `Verification`/`Publishing` need one retained
post-witness driver token bound to the request token/generation. Only that
token may invoke verifier/publisher/reconcile. On an HTTP drop it transfers to
the maintenance owner; a retry with matching identity receives a bounded
pending/join result (or waits on that driver's completion notification), never
another `Verify` turn. The driver owns the existing DocumentWrite lease until
`Published` or terminal close.

Add a gated publisher law: pause the abandoned owner after it enters
`Publishing`, issue a same-identity retry, then release. Assert one `reserve`,
one materialization/publish operation, one ledger `approved` event, one exact
WAL event, and release of the writer. Run the converse ordering as well: a
retry first acquires the post-witness driver and a dropped original joins
without a second attempt. The current retry test covers an eventual retry after
an injected failure, not overlapping publisher calls.

### P1 — Post-Witness Maintenance Reuses an Expired HTTP Deadline

After a receipt, abandoned maintenance intentionally drops ingress and
continues verification/publication. Its retry calls `verify` with
`identity.journal_now_ms + JOB_MAX_LIFETIME_MS` and the original
`journal_now_ms` ([1226–1251](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1226)).
The production publisher compares that absolute deadline against the real
system clock ([3316–3328](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3316)).
If publication first fails or the request is dropped after that original HTTP
deadline, every maintenance retry is already cancelled. The exponential loop
keeps the Stores and DocumentWrite lease, while a fresh retry only joins the
same stuck `Verification`/`Publishing` state.

At receipt cutover, mint a process-owned, bounded **maintenance attempt
deadline** from the current clock (with an injected clock in tests), distinct
from the caller's input deadline. Each retry needs its own bounded attempt
control; the retained receipt/driver survives it. Add a law that advances the
publication clock past the HTTP deadline before dropping the request, injects
one publisher failure, then proves a later attempt publishes, reconciles once,
and releases DocumentWrite. This is not covered by the fake publisher, which
ignores its deadline.

### Current Ingress/Teardown Acceptance Boundary

The new ingress unit law correctly proves the four outer guards remain held
and that it does **not** take DocumentWrite
([bin 8401–8429](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8401)).
The request moves its original ingress Arc into `live_ingress`, then removes
every retained identity copy at `AwaitingAck` ([1468–1491](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1468), so normal
post-receipt cancellation does not retain User/Session/Space/Membership.
`HubInferenceRuntimeV1::close` delegates to the retained committer before DB
shutdown ([1806–1809](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1806;
[bin 7541–7549](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7541)).

These are source-level observations only. There is no production Hub HTTP
cancellation/teardown law that pauses a request pre-witness and post-witness,
stops admission, invokes this exact close path, and proves respectively:
`abandoned`/stores+writer returned; or one durable witness → one checkpoint →
one outbox reconciliation. It should use the actual Hub publisher rather than
the test publisher.

### Global Directory Message Authority Law — Read-Only Test Review

The new four-row law is a useful, intentionally meaningful RED against the
current record-only global socket fence. Point 1 pauses before
`socket_live_authority`; point 2 pauses after the old visibility decision
([5677–5733](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5677)).
For `delivery-before-membership`, old code reaches point 2 after a successful
role read while holding only recipient User/Session. The owner can complete
the real `/directory/commands` `RemoveMember`, then the released sender emits
the stale A event. The law's pending-revoke assertion at
[`bin.rs:10831`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10831)
therefore fails for the actual missing message-space union. The session rows
correctly use the same recipient User/Session keys and distinguish 4401 from
one-space removal/B liveness.

Two narrow law repairs are still needed before it can qualify the future
union:

1. The test spawns revocation and immediately uses a 100 ms pending assertion.
   It has no proof that the production HTTP task reached the command/fence;
   an unscheduled task can vacuously look blocked. Consume the existing
   `directory_command_attempted` semaphore, emitted at the real pre-fence
   route hook ([4765–4775](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4765)),
   before the pending assertion. For a strict causal proof, add test-only
   `directory_command_fence_attempted` immediately before, and
   `directory_command_fence_acquired` immediately after,
   `acquire_directory_command_fence`. In the delivery-first row, seeing the
   attempt but not acquisition before releasing point 2 proves the exact
   message union—not merely timing. After release, wait for acquisition and
   completion. The revoke-first row remains correctly paused before the union,
   lets removal commit, then releases the sender for its fresh visibility read.
2. The six-message fixture currently has only the Bun/SQLite expression oracle
   in [`DirectoryMessageAuthorityCheckScript`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3006).
   That validates the corpus but calls no Rust selector; current production has
   no selector yet. When root introduces the union, make its pure
   `directory_message_space`/binding builder decode every fixture `messageJson`
   and assert the exact `None → [User,Session]` or
   `Some(space) → [User,Session,DirectorySpaceAuthority,Membership]` vector.
   This catches a future omission of Connection, Presence, or Rebootstrap while
   retaining the independent Bun oracle.

The test's B assertion is non-vacuous: if a stale A frame leaks after a
removal-wins row, `next_directory_message` consumes A first and its exact-B
matcher fails ([10844–10850](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10844)).
Each socket is dropped at the end of its vector; final
`stop_recovery_server` waits for all directory/database Arcs to retire before
the explicit database shutdown ([10546–10562](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10546)).
No teardown race is demonstrated by the current source.

### Global Directory Message Authority — Current Production Union Review

The current implementation closes the observed delivery-before-removal race.
`directory_stream_message_space` derives the scope from all five message
variants, and `directory_space_message_bindings` borrows (rather than indexes)
the exact `DirectorySpaceAuthority` and recipient `Membership` keys only for a
global, session-bound record ([5586–5610](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5586)).  Scoped records already permanently carry
their own scope keys; global records retain only User/Session in the ledger.
The six decoded wire rows prove both sides of that distinction, including
Connection, Presence and Rebootstrap ([10818–10851](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10818)).

`send_socket_directory_message` takes that complete sorted union once, then—
without dropping it—performs subject/plan/live validation, the current
membership-only visibility read, and the bounded WebSocket send
([1533–1546](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:1533),
[5716–5765](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5716)).
There is no nested `acquire_record` in this path.  The regular live tick still
uses only the permanent User/Session record keys, which is correct: it can
close a globally invalid Session but cannot decide the visibility of an
individual space message.  The delivery path is the scoped decision point.
`send_socket_directory_rebootstrap` uses the same transient space union before
its role lookup and private checkpoint control read ([5768–5795](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:5768)).

The four production-socket vectors now have a causal attempted-ingress signal,
not only a timing assertion.  In a sender-first vector the message owner holds
the recipient’s User/Session/space/member union through its send and the
pending membership/session revoke cannot complete; in a revoke-first vector
the subsequent send revalidates and either skips A while preserving B or
closes 4401 for a revoked session ([10855–10937](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10855)).
This is the correct distinction between one-space membership removal and
principal revocation.

**Current verdict:** no source-visible authorization bypass or lock recursion
remains in this union.  Holding the keys through a bounded `sender.send` can
make a concurrent mutation wait up to the existing two-second outbound I/O
budget; that is the necessary linearization interval for “delivery wins” and
is bounded fail-closed rather than a security defect.  Native receipt
`QZSA7l/00` subsequently qualified both global laws and all four real
WebSocket ordering rows (SHA
`3346ef7ba7acdbc251a3f9f6592506953eb271549781361c23ecbe451b94fb79`).
The earlier `KktvOY` failure was a fixture wire-type error—an ArtifactHash hex
string where the codec requires the byte array—not an authority failure.

### Approval Retention Follow-Up — Current Partial Repair

The current requester drop path has materially improved.  It now treats both
an absent document state (`Complete`) and an explicitly reverted pre-witness
state (`Aborted`) as an exact `abandon_prepared_approval` obligation while the
identity still retains ingress ([1289–1317](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1289)).
This closes the previously reported `owner.admit`-then-mount-failure gap:
`admit` still clears the separate prepared record, but a failed
`database.document`/mount that has not installed `documents[key]` now reaches
the identity-based abandon branch.  Same-identity joiners also use a `watch`
state epoch rather than `yield_now` spinning ([786–870](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:786)).

Two current P0/P1 issues remain until the in-progress driver patch lands:

1. **P0 — `close()` can still become a second post-WAL driver.**  The active
   HTTP request is not represented in `maintenance`.  If it is paused in
   `Verification` or `Publishing`, `close` sees no maintenance owner and calls
   `verify` itself ([1762–1851](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1762)).
   `verify` accepts both states and treats `Publishing` as not already
   published, so the original and close path can both call the publisher
   ([1462–1535](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1462)).
   The driver token must cover live request, abandoned maintenance, and close;
   close must wait for or take a deliberate transfer, never independently
   verify.  Add a paused-real-publisher active-request + `close()` law proving
   one publish/materialization/reconcile and one terminal writer release.
2. **P1 — maintenance still uses the original clock.**  Both abandoned
   maintenance and `close` call `verify(JOB_MAX_LIFETIME_MS,
   identity.journal_now_ms)` ([1318–1333](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1318),
   [1840–1850](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1840)).
   The publisher interprets that as an absolute `decision_now_ms + lifetime`;
   a delayed retry remains immediately expired.  Mint a fresh, injected
   process-clock deadline per retained attempt; preserve `journal_now_ms` only
   for ledger/WAL semantics.  The native law must advance past the HTTP
   deadline, inject one failure, then prove a later retained attempt finishes.

**Correction — the preceding P1 is retracted.**  The current production
`GisMapApprovalCheckpointPublisherV1Impl` deliberately ignores
`decision_now_ms` and derives its control deadline from
`SystemTime::now() + attempt_lifetime_ms` ([3414–3442](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3414)).
Thus abandoned maintenance’s fresh bounded lifetime is already process-clock
relative; the immutable journal timestamp is ledger/event provenance, not a
publisher expiry.  The post-WAL single-driver P0 above remains open.

**Follow-up — the current source now repairs that single-driver P0, pending
native qualification.** `state_has_active_request` recognizes a state lease
whose private request token has an additional active-request or maintenance
owner, and `close` waits on the state epoch before selecting any document
([1116–1128](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1116),
[1774–1786](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1774)).
The close path cannot acquire its separate close token until the active
document-write lease releases the same gate.  That removes the concrete
paused-live-request/close duplicate-`verify` trace.  Acceptance still needs
two physical laws: pause a live publisher then call close, and pause
abandoned maintenance then call close; in each, close stays pending, release
produces exactly one publisher/materializer/reconcile operation, and every
Store/DocumentWrite owner terminally returns.

### P0 — The Synchronous Preflight Owner Still Discards Its Outbox Outside Tokio

The new `commit` constructs `GisMapApprovalRequestOwnerV1` synchronously, then
returns a future.  Its Drop correctly schedules exact abandonment when a Tokio
runtime is current, but its `Handle::try_current()` error arm merely removes
the maintenance key ([373–399](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:373)).
The corresponding `spawn_abandoned_prepared` arm does the same
([440–459](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:440)).
Creating and dropping the returned future from a synchronous caller therefore
loses the exact prepared ledger row, rather than proving the advertised
future-never-polled cleanup.

This is not an argument to make Drop perform async I/O.  The ledger’s first
`abandon_prepared_approval` attempt is synchronous: make the no-runtime path
attempt that exact abandon immediately.  If it refuses or faults, retain the
complete owner in an explicit bounded process-held recovery cursor (or reject
construction before durable preparation); it must never erase the only
recovery key.  Add a no-current-runtime construction/drop law with a prepared
row, asserting it is abandoned or remains observably retained for recovery,
never stranded.  The existing Tokio task-drop law does not cover this branch.

**Current-source correction.** The direct no-runtime loss is now repaired:
the identity-bearing branch transfers the complete
`GisMapAbandonedRequestV1` into `parked_requests`; the pre-admission branch
first attempts synchronous exact abandonment and otherwise retains
`parked_prepared` ([377–535](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:377)).
The new native source law creates/drops an unpolled future on a thread without
a Tokio runtime and proves the prepared outbox is abandoned before ingress is
released ([3150–3285](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3150)).
The preceding direct-loss P0 is therefore retracted.

### P0 — Parking Repair Still Loses an Owner if Its Caller Is Cancelled

`drain_parked_requests` and `drain_parked_prepared` remove the only exact
cursor from their maps and then await their respective unbounded retry loops
([474–485](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:474),
[526–537](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:526)).
Both are called at the beginning of caller-owned `commit_retained` and
`close` futures ([1658–1665](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1658),
[1855–1861](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1855)).
If the ledger has a transient failure, either driver awaits its backoff sleep;
dropping the outer commit/close future then drops the local cursor.  Its
maintenance key is never removed (that happens only after the loop returns),
and no map retains the identity/ingress/request needed to try again.  This is
the same owner-loss class the parking maps were introduced to prevent.

Once a runtime is available, draining must transfer each map entry directly to
an independently owned task before any awaited retry.  The task alone owns
the cursor and removes its exact maintenance key only after terminal
abandonment.  Alternatively, a `ParkedDrainLease` must reinsert the exact
cursor in `Drop`; merely cloning a key is insufficient.  Do the same for
prepared and identity-bearing request maps.  Add a physical law that parks an
owner, starts a drain, pauses the ledger failure/backoff, drops both the
draining commit and close future, then drives a fresh close/retry to terminal.
It must prove one owner remains observable throughout, the exact maintenance
bit stays live until terminal, the outbox reaches abandoned (or receipt
recovery), and Store/DocumentWrite/ingress owners are returned exactly once.

`HashMap::with_capacity(GIS_MAP_COMMITTER_CAPACITY)` is allocation only, not a
limit ([421–427](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:421)).
In particular, multiple manually-driven same-document waiters can enter
`prepare_retained_document` before being moved and dropped outside a runtime;
they can grow `parked_requests` even though mounted documents cap at 64.  The
parking insertion needs the same explicit fixed-capacity refusal/retention
contract and a saturation law; it cannot rely on the map’s initial allocation.

### Admin Directory Principal Fence — Qualified Mapped Slice

The new mapped-command boundary is coherent and now native-qualified by
`KeP3lW/00` (SHA
`5e0c9cebcd0176638e1aa0fda242ff7e0a7ce4992cb9b9c335c778a63716d6a0`).
`acquire_admin_directory_authority` builds the principal `User` and `Session`
keys, adds the server-derived create target when applicable, then delegates the
closed command’s space/member additions to the one sorted gate helper before
checking configured subject, exact session id/generation/expiry, and a
role-free session binding ([6491–6514](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6491)).  The Create and all eight closed
directory commands retain those guards through `DirectoryService` append
([6522–6564](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6522)).
This does not confuse configured administrator authority with an ordinary
space role.

The five HTTP rows are causally useful: before-fence session revocation
returns a cancelled receipt with zero principal events; admitted command wins
only until it releases its exact User/Session guards; and the configured
administrator still succeeds after durable demotion to Spectator
([10867–10966](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10867)).
The self-user-revocation tail also proves the current mapped helper has not
introduced a nested User lock.

### P0 — The Remaining Admin Intents Can Still Mutate After Principal Revocation

The mapping deliberately stops before `IssueDocumentShare`,
`RevokeDocumentShare`, `RevokeUserSessions`, `KickConnection`, and
`RebuildDirectoryProjections`.  Those arms enter after only the unretained
`authenticate_admin_principal` reread in `admin_intents`; none retains the
administrator’s User+Session authority through its effect
([6655–6699](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6655),
[6566–6650](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6566)).

This is a concrete mutation gap, not merely a missing test.  In the share
branch, the document descriptor read and `issue_share_token_as` insert are
separate awaits.  A winning self-session revoke after the initial reread can
therefore still mint a share.  A winning standard space deletion can also
fall between them because this arm holds no `DirectorySpaceAuthority`; each
backend’s share insert is a bare `hub_share_grant` insert, with no descriptor
or space existence predicate (SQLite shown at
[821–833](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:821)).
The share revocation arm holds `Share` only, and session revocation holds the
target User only, so each can similarly persist/signal after the actor loses
its administrator session ([6581–6629](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6581)).

Use one closed **intent binding planner**, rather than a second ad-hoc lock
path:

```rust
fn admin_intent_bindings(
    principal: &AdminPrincipalV1,
    intent: &AdminIntentV1,
) -> Vec<SocketBindingKeyV1>;

async fn acquire_admin_intent_authority(
    state: &HubState,
    principal: &AdminPrincipalV1,
    intent: &AdminIntentV1,
) -> Result<AdminIntentAuthorityV1, FencedDirectoryCommandErrorV1>;
```

The planner always begins with principal `User` + `Session`; it adds exactly
one `DirectorySpaceAuthority` for an issue/revoke-share scope, `Share` for
share revocation, and target `User` for session revocation.  It must build the
**whole** vector before `acquire_bindings`, which sorts/deduplicates it; the
self-revoke case then owns one User guard rather than trying to take the same
non-reentrant mutex inside the action.  Create and mapped commands should use
this planner too, retaining the existing closed `DirectoryCommand` scope and
RemoveMember target additions.  The returned opaque authority is the sole
input to the short effect helpers; remove their current nested `gate` calls.
Immediately after acquisition, run the existing exact configured-subject,
session id/generation/expiry binding revalidation.  A refusal appends the
existing cancelled terminal, never invokes the effect.

For the short actions, retain that authority until: the share insert plus
returned one-shot token are fixed; the durable share revocation plus grant/plan
invalidation are complete; target-session durable revocation plus every
affected grant invalidation is complete; or the kick has been looked up and
signalled.  `DirectoryService::execute` holds its own writer only for command
append ([2053–2062](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2053));
the sorted Hub authority must remain the outer linearization fence, not be
acquired from inside that service.

`RebuildDirectoryProjections` is deliberately different.  All three backends
hold one write transaction while truncating/replaying the whole projection
(SQLite begins the immediate transaction at
[2034–2050](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2034)).
Holding the principal User/Session gates across that up-to-ten-second job would
make revocation wait behind the rebuild, which is not acceptable.  Do **not**
apply the short-action guard unchanged.  The correct next API is a resumable,
copy-on-write rebuild: a bounded worker creates/replays a generation-owned
staging projection without exposing it; every bounded commit/checkpoint
reacquires and revalidates the principal authority; only a final short,
guarded atomic generation flip publishes it.  A revocation between chunks
sets the owned operation control to cancelled and discards staging.  Until that
exists, either leave rebuild out of the shared planner with an explicit
non-claim, or reject it from the public admin route—never hold the session
gate for its full transaction.

Required physical matrix for that next slice:

1. **Session wins before issue-share admission:** cancelled receipt, no
   `hub_share_grant`, no share-issued audit, no plaintext capability response.
2. **Issue-share wins:** revocation demonstrably attempts but waits on the
   principal union; after release exactly one share/audit exists, then revoke
   completes.  A deleted/archive-space peer must use the real directory command
   path and cannot cause a stale-scope insert.
3. **Session wins before revoke-share / revoke-other-user / kick:** each has
   zero durable target mutation and zero kick notification.  In the converse,
   each action has exactly one effect before the queued revocation completes.
4. **Self `RevokeUserSessions`:** the deduplicated union completes—no nested
   mutex timeout—and the actor session is absent afterward.
5. **Rebuild:** revoke after a real staging checkpoint returns within its
   ordinary two-second command budget; it forces a cancelled terminal, no
   published generation switch, no partially visible projection, and a later
   fresh configured session can start a new operation.  Test explicit
   operation cancellation in the same window as well.

The current five-vector fixture is sufficient evidence for the completed
mapped scope only; it must gain separate closed intent/effect vectors rather
than pretending `rename` covers share, target-session, kick, or rebuild
semantics.

### GIS Approval Parking — Caller Cancellation Repair Landed; Runtime Destruction Still Loses the Sole Owner

The previous caller-cancellation trace is repaired in the current source.
`resume_parked_requests` and `resume_parked_prepared` now drain only into a
separately spawned owned Tokio task; the caller-owned `commit_retained` or
`close` future does not await the retry loop any more
([475–545](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:475)).
That supersedes the preceding **caller-drop while draining** P0, pending its
native qualification.

There remains a distinct P0 at runtime destruction.  Once a parked entry is
drained, that spawned future is its sole owner.  Tokio aborts such futures
when the current runtime shuts down; neither `GisMapAbandonedRequestV1` nor
`GisMapPreparedApprovalV1` has a drop guard that re-parks the exact owner.
The task then vanishes while its maintenance key is never removed
([448–490](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:448),
([532–545](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:532)).
The no-current-runtime fallback does not cover a runtime which existed for the
spawn and is subsequently destroyed.

The maintenance task needs either a drop guard which re-inserts its exact
cursor and preserves/re-wakes the corresponding maintenance key, or an owner
held by a process-lifetime maintenance coordinator which the Tokio task only
borrows one turn at a time.  Removal of that exact key remains terminal-only.
Add a temporary-runtime physical law: drop an unpolled pre-witness request,
make its driver pause, destroy that runtime, then resume under a fresh runtime.
It must observe the same request/ingress until one terminal abandonment,
return every Store/DocumentWrite owner once, and remove the key only then.

Also make `parked_requests` and `parked_prepared` materially bounded.
`HashMap::with_capacity` at construction is not an admission limit
([397–427](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:397)).
A full map must retain the just-dropped owner through a defined refusing or
overflow-retirement path; it must never silently discard it.  Add a
capacity-plus-one law with manually driven, no-runtime request drops.

### Database/FS Open — Exact Cancellation and Retirement Boundary for the Hub First-Open Investigation

`test_state_with_directory` opens a new filesystem `Database` before every
scenario ([8031–8033](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8031)).
The observed five-second first-`test_state()` timeout therefore remains a DB
opening investigation, not evidence against the directory authority fence.
The current source gives a concrete cancellation leak candidate, but does not
yet prove it caused that timeout.

The exact FS opening sequence is:

1. `FsStorage::open` validates the root, reserves rollback capacity, acquires
   a `WorkerPoolUse`, and registers a backend control
   ([8126–8131](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8126)).
2. It submits and awaits `DbIoTask::BackendOpen`; the worker invokes
   `durable_directory` only after exact root-control validation
   ([7722–7730](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7722)).
3. Only after that await resolves is an `FsStorage` value constructed.  Its
   `Drop` is the normal nonblocking `retire_db_io_backend` owner
   ([8132–8160](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8132)).
4. `Database::open_with` then separately runs retained capability probing,
   catalog read, empty-catalog bootstrap or catalog decode, version-graph
   setup, emit, and facade publication
   ([8074–8146](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8074)).

Dropping the outer future in stage 2 is unsafe today.  `DbIoTaskOperation::Drop`
only marks its task abandoned/cancelled and enqueues task cleanup
([4808–4828](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4808)).
Task cleanup requests a backend close only for a `BackendClose` task;
`backend_to_close` is assigned only at `DbIoTask::BackendClose` allocation
([3738–3809](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3738)).
There is no `FsStorage` yet to retire the registered control and no
`DbStorageOpenRejected` is returned to retain it.  The backend registry can
therefore keep its executor and `WorkerPoolUse` live after a cancelled opening
task.  SQLite has the same registered-before-`BackendOpen` shape
([720–728](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:720));
but Postgres, Neo4j, and Memory do **not**: each constructs its concrete
storage before awaiting `BackendOpen`, so its existing `Drop` retires the
control if that await faults or is cancelled.  The immediate repair scope is
therefore exactly FS and SQLite.

The smallest repair is not a new generic open framework: construct the FS or
SQLite storage immediately after successful registration, before awaiting
`BackendOpen`, and execute through that storage.  Its existing `Drop` then
marks the exact control for nonblocking retirement on either cancellation or
the error return.  The returned `DbStorageOpenRejected::Registered` still
retains the same control for explicit close-fault observation; it no longer
is the sole cleanup mechanism.  A dedicated opening owner is justified only
if constructor-specific fields cannot be safely initialized before opening.

The causal native law should pause precisely before `FsDbIoExecutor` calls
`durable_directory`, cancel the opening future after registration, release the
worker, and drive ordinary DB-I/O maintenance.  It must show (a) task and
backend registration both return to their baseline, (b) the pool can shut down
without a retained use, and (c) a second FS/database open succeeds.  Use the
existing storage ledger witness and controlled backend-operation pattern; do
not rely on a sleep or rerun.  A parallel stage table should record
`registered`, `backend-open-entered`, `backend-open-terminal`,
`backend-close-requested`, and `backend-retired`, so a future Hub timeout can
be attributed to one retained owner transition rather than to the directory
fixture.

**Additional P0 — ordinary FS/SQLite opening failure has the same unowned registered
backend.**  If `BackendOpen` itself faults, FS returns
`DbStorageOpenRejected::Registered { control }` ([8131–8135](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8131)).
Neither that type nor `DatabaseOpenAtRejected` implements `Drop`; their only
cleanup operation is caller-driven `retry_close`
([2730–2797](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2730),
([7960–8004](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7960)).
The actual Hub production connector simply propagates that owner through
`HubError` with `?` ([7381–7394](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7381)); an ordinary failed startup response can therefore be
dropped without ever requesting backend retirement.  `#[must_use]` is only a
lint and does not retain or drive the backend.

Constructing FS/SQLite storage before their open await fixes this error route
too: stack unwinding drops the storage and requests exact nonblocking
retirement before the rejection leaves the function.  A caller that does use
`retry_close` still owns the same control and can observe an explicit close
fault; automatic retirement must not invent a second control or double-close
it.  Add the parallel injected-`BackendOpen`-fault law: propagate then drop
the rejection, drive maintenance, observe the backend slot and `WorkerPoolUse`
return to baseline, and prove a second open is admitted.  This remains a
causal candidate for the Hub stall, not a diagnosis of it.

### FS/SQLite Opening Repair — Existing Physical Test Seams

The narrow construction repair has a usable test boundary already; it does
not need a new public rejection API or an alternate executor framework.

* The storage crate's existing test module provides the serial guard,
  `fixture_serial()` ([8370](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8370)), exact
  `ledger_witness()` ([9211](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:9211)),
  `drain_control_tasks(control)` ([9242](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:9242)),
  and `db_io_test_pool()` ([5003](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:5003)).
  The existing one-worker `Lane::Io` blocker plus channel acknowledgement at
  [9898–9912](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:9898)
  gives deterministic admission ordering without a timer.
* For each concrete constructor, occupy a fresh one-worker pool's `Lane::Io`,
  pin `FsStorage::open(pool, root)` or `SqliteStorage::open(pool, path)`, and
  poll it once with the same `Context::from_waker(Waker::noop())` pattern used
  by existing retained-owner laws.  The pending poll has registered the real
  control and queued the real `BackendOpen`; drop the future, release the
  blocker, drive maintenance, then require the exact original ledger witness
  and `pool.shutdown() == Ok(())`.  This tests cancellation after registration
  but before the concrete backend calls its open operation.  It fails before
  the FS/SQLite preconstruction repair because their backend's `WorkerPoolUse`
  remains live; it passes only if the existing `Storage::Drop` has requested
  terminal close.
* The actual FS error injection needs no fake executor: make the proposed root
  a pre-existing ordinary file, so `FsDbIoExecutor::BackendOpen` reaches
  `durable_directory` and faults ([7724–7729](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7724)).
  Before calling `DbStorageOpenRejected::retry_close`, inspect the exact
  private registry row: the returned control must already have
  `close_requested`.  Then drain the retained rejection to terminal, require
  baseline ledger/pool shutdown, and prove a fresh open.  That assertion
  distinguishes automatic concrete-storage retirement from the later explicit
  `retry_close`, which itself would otherwise mask the bug.
* SQLite likewise has a physical failure path; no injected executor is needed.
  Pass an existing directory as the database path.  The concrete
  `SqliteDbIoExecutor::BackendOpen` invokes `Connection::open(path)` inside its
  real `execute_step`, which rejects that type conflict
  ([274–289](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:274)).
  Reuse the same registry-before-`retry_close`, drain, baseline, and clean
  reopen assertions.  This is stronger than a test-only injection because it
  preserves the actual SQLite open route.

Make these schema-first rows in the already relevant
`storage/🧪️fixtures/🔐️backend-pool-use/{🧬️.schema.json,🔣️.json}` rather
than inventing a public fixture protocol: one `queued-open-future-drop` row
for FS and SQLite, and one `backend-open-fault-drop` row for each.  Each row
needs explicit `closeRequestedBeforeRetry`, `poolShutdownAfterDrain`, and
`ledgerBaseline` facts.  That fixture is already independently validated by
`WalWriterAuthorityCheckScript` and its exact native law list lives in
[`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:19).
Register the two native laws in that list and run the existing targeted Nx
entrypoint, `@semio-tech/framework-os-kernel:wal-writer-authority-native-check`
(the project invokes `bun ./📜️script.ts wal-writer-authority-check --native`).

The applicable local instruction scope is exactly repository
[`AGENTS.md`](/Users/ueli/Documents/semio/AGENTS.md),
[`framework/products/AGENTS.md`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/AGENTS.md),
and [`framework/products/os/AGENTS.md`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/AGENTS.md); no more-local DB instruction file exists.

### GIS Approval Runtime Shutdown — Drop Guards Repair the Original Loss, but Parking Is Not Yet Bounded

The latest `GisMapAbandonedRequestTaskV1` and
`GisMapPreparedApprovalTaskV1` changes repair the earlier runtime-destruction
owner-loss P0 in source.  Each task now owns the original cursor in an
`Option`; its `Drop` puts that cursor back into the matching parked map and
only `complete()` clears the owner and removes the maintenance key
([412–455](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:412)).
The new temporary-runtime laws then observe parking after shutdown and a
fresh-runtime `close()` drain ([3308–3365](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3308)).  That is the correct owner-preserving
shape; it is source evidence only until native qualification.

One P0 remains.  `parked_requests` and `parked_prepared` are `HashMap`s with
initial capacity only ([407–408](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:407),
[470–474](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:470)).
The fixture's `cleanupCapacity = 128` does not make either map bounded.
Moreover, `commit_retained` sets the request owner admitted immediately after
pure preflight, before `prepare_retained_document` ([1721–1732](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1721)).
Distinct public request IDs can therefore be dropped at the latter boundary
and accumulate maintenance cursors independently of the SQLite ledger's
`JOB_CAPACITY` check.  The current `entry(job_id).or_insert(owner)` also drops
the later cursor silently if an equal job ID is already parked
([426–431](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:426)).

Before qualification, either demonstrate that every cursor receives an
already-bounded durable ledger admission before it can be parked, or reserve a
fixed maintenance slot before `owner.admit` and return/retain an exact
capacity refusal.  The law must fill that real slot table through dropped
pre-witness requests, including runtime shutdown, then assert capacity+one
does not allocate or lose an owner.  It must also prove why same-job duplicate
cursor replacement is safe, or retain/reconcile the duplicate explicitly.

#### Correction — Bounded Cleanup Reservation Now Lands Before Owner Capture

The current implementation satisfies the requested bounded admission in
source.  `commit` first reserves `cleanup_jobs[job_id]`, caps distinct entries
at the schema `JOB_CAPACITY`, and returns `Capacity` before it constructs an
owner or clones ingress ([2080–2104](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2080)).
Every production-created prepared/request cursor consequently has a
reservation; the parked maps are now bounded subsets of those 128 keys even
though their `HashMap::with_capacity` allocation is only 64.  Cancellation
parks without decrementing; terminal task completion removes its maintenance
key then decrements the exact reservation; replacement of a same-job parked
cursor drops one redundant reference ([422–537](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:422)).
The new capacity law fills all 128 slots, observes the 129th real commit
return `Capacity` with ingress dropped and the prepared outbox unchanged, then
releases every reservation ([3336–3368](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3336)).  This closes the preceding unbounded-parking P0 in source;
native qualification remains pending.

One bounded P1 remains if `GisMapApprovalCommitterV1::commit` stays a broadly
reachable internal API: `park_*` coalesces only by job id, not by the full
immutable mutation/command/proposal/scope tuple.  The actual route is safe in
the reviewed path—it fetches the ledger identity, derives the frozen base,
server-stamps the command, and prepares the same ledger row before invoking
the committer ([2736–2748](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2736)).
Close the direct committer boundary to that route, or assert tuple equality on
same-job coalescing and add a two-caller same-job/different-tuple rejection
law.  That is hardening, not a demonstrated production-route bypass.

#### Correction — Direct Commit Admission Now Also Binds the Durable Tuple

The direct committer hardening has now landed.  Before cleanup-slot reservation
or owner construction, `commit` resolves `approval_recovery_by_mutation` and
requires the durable outbox job/mutation/command/proposal tuple plus accepted
scope and ingress User/Session/generation to match
([514–536](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:514),
[2110–2135](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2110)).
A same-job substituted tuple therefore returns `Conflict` before it can reserve,
park, replace a cursor, or abandon the legitimate row.  This closes the P1
above in source.  The missing qualification row is now precise: substitute one
validly shaped command/proposal tuple for an existing job and require conflict,
unchanged cleanup reservations/outbox, and released ingress.

### P0 — `close()` Can Report Terminal While an Unpolled Commit Still Owns Cleanup

The new cleanup reservation makes this trace explicit.  `commit` validates and
reserves `cleanup_jobs[job_id]` synchronously, then returns an unpolled future
holding `GisMapApprovalRequestOwnerV1` ([2110–2140](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2110)).
Before that future is polled, there is no document state and no maintenance
key.  `close()` resumes parked work, sees no document, and returns `Ok` when
the maintenance set is empty; it never inspects `cleanup_jobs`
([1968–2026](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1968)).

That permits this physical lifetime inversion:

1. A valid commit is created but not polled; its cleanup reservation and ingress
   owner exist.
2. `close()` returns success.
3. The caller drops the commit future or its runtime.  The owner subsequently
   spawns/re-parks abandonment work, after the caller is entitled to stop the
   runtime, database, and worker pool.

The Drop guards preserve the cursor, but they cannot make a previously
successful close become nonterminal.  This is an owner/quiescence P0, not an
allocation issue.

Make successful close conditional on all three domains being quiescent:
documents, maintenance keys, **and cleanup reservations**.  If a caller still
holds an unpolled future, close must remain pending (or return an explicit
nonterminal refusal); once that future is cancelled/dropped, its exact
abandonment driver must reach terminal, release ingress, remove the maintenance
key, decrement the reservation, and only then wake close.  The committer also
needs a Closing admission bit, or its outer Hub route must be proven quiesced,
so a fresh `commit` cannot reserve immediately after close observed all three
sets empty.

The wake order is part of the repair.  `release_cleanup_job` currently changes
the cleanup map without announcing `state_epoch` ([535–544](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:535)).
An earlier turn announcement can wake close while the reservation is still
present; it will recheck, sleep again, and then miss a later normal
`owner.complete()` decrement.  Publish a state change after releasing the
cleanup mutex whenever the close predicate may have changed.  The physical law
must intentionally park close on this predicate before it drops the request,
so a mere immediately-ready close cannot mask that lost wake.

#### Correction — No Proven Separate Lost-Wake in the Current Normal Path

The preceding wake paragraph overstates the current source.  In
`commit_retained`, the turn announcement and `owner.complete()`/cleanup
decrement are consecutive synchronous statements with no await or yield
between them ([1809–1839](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1809)); a woken close cannot run in that interval.
The task-complete paths decrement then announce.  Publishing after every
cleanup-predicate change remains a sensible defensive simplification if close
is extended, but it is not a separately demonstrated P0.  The confirmed P0 is
solely close's current omission of `cleanup_jobs` from its terminal predicate.

Required physical law: create a fully valid prepared commit future without
polling it; first-poll `close()` and require `Pending`; drop the future; drive
the exact abandoned row; require released ingress, empty documents/maintenance/
cleanup maps and durable outbox terminal; only then observe `close()` ready and
successful database/pool shutdown.  A concurrent fresh commit after close
begins must receive the closed/refused outcome and leave no reservation.

#### Correction to the preceding wake correction — Production Hub Is Multithreaded

The preceding "No Proven Separate Lost-Wake" correction is wrong.  The Hub
entrypoint is `#[tokio::main]` without a current-thread flavor
([7488](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7488)), so
production runs Tokio's default multi-thread runtime.  Another worker can poll
`close()` after an earlier epoch notification yet before a normal owner has
decremented `cleanup_jobs`: it observes a nonempty predicate, subscribes for
another epoch, and otherwise has no wake for that later decrement.  The lack
of an `.await` between the notifying statement and the decrement does not make
those operations atomic with respect to a different runtime worker.

The current source has the required remedy: `release_cleanup_job` mutates the
exact cleanup entry under its mutex, drops that mutex, then sends
`state_epoch` ([540–550](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:540)).
That is the correct ordering.  Do not move the send before the predicate
mutation or under the cleanup lock.  A native law should use a two-worker
barrier at the old signal/decrement boundary: make `close()` observe the old
nonempty cleanup predicate after the first epoch, release the decrement, and
require close to complete from the second post-mutation epoch.  This remains
source evidence until the scheduled law is run.

### FS/SQLite Opening-Retirement TDD — Causal Review

The newly registered real-storage laws are causally shaped and do not hide the
defect with a test-side close request.  In
[`db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:10693),
the fresh one-worker pool's actual `Lane::Io` is occupied before the opening
future is first-polled.  `submit_db_io_task` allocates and submits the real
`BackendOpen` synchronously before that future awaits its terminal result
([4549–4559](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4549));
the test therefore observes a pending real operation and an exact retained
pool use, drops only the opening future, checks `close_requested`, then releases
the blocker.  It never invokes `retry_close`, `retire_db_io_backend`, or a
synthetic executor.  The terminal drain checks both the backend registry and
task slots for that exact fresh pool before it permits a fresh physical reopen
and successful pool shutdown ([10667–10730](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:10667)).

The failure law is also physical.  FS receives an ordinary file where its root
must be a directory; SQLite receives a directory where `Connection::open` must
open a database file, with an independent `rusqlite` rejection check
([10736–10771](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:10736)).
It inspects the returned registered control before it drops the rejection and
again performs only generic mounted maintenance.  The current constructors
still await before constructing `FsStorage`/`SqliteStorage`
([8126–8134](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8126),
[`sqlite:721–728`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs:721)),
so both laws should RED at `closeRequestedBeforeRetry` before a ten-second
drain timeout.  That is a strong expected-red: `DbStorageOpenRejected::Registered`
has no Drop close protocol; only the existing concrete storage Drop can signal
this deferred close.  Constructing that exact storage immediately after
registration is therefore both the minimal repair and what these laws isolate.

No test flaw requiring a change was found.  In particular, the initial
`pool.shutdown() == Busy { retained_uses: 1 }` assertion is non-destructive:
the pool stays `Open` on Busy ([1961–1966](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1961)).
The four schema rows are present in the independently validated
`backend-pool-use` fixture; only source/oracle qualification is currently
known, not a native result.

### Unified Admin Short-Intent Authority — Current Source Review

The new short-intent boundary is coherent in source. Every closed, bounded
intent other than long-running `RebuildDirectoryProjections` maps through
`admin_intent_bindings` before an effect begins
([6492–6519](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6492)).
It always includes the configured principal's `User` and `Session` keys, then
adds the deterministic create-space target, exact space, removed membership,
share, or revocation target user as applicable. `SocketBindingKeyV1` has a
single derived total order and `acquire_bindings` sorts/deduplicates before
locking ([708–715](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:708),
[863–869](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:863)).
This is compatible with the public directory, invitation, socket, and global
delivery fences that use the same order; no reverse nested gate acquisition was
found.

`acquire_admin_intent_authority` holds that union while it validates the
configured provider/subject, exact durable session/user/generation/expiry, and
then through the actual short side effect ([6522–6540](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6522),
[6548–6666](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6548)).
It calls direct directory share/user methods rather than trying to reacquire
the already-held `Share` or `User` key, so self-session revocation does not
self-deadlock. The initial accepted audit row is intentionally outside the
short-effect union; the subsequent inside-fence revalidation makes the terminal
receipt `Cancelled` and prevents the guarded physical effect if the session
lost first. That is exactly what the new physical RED was intended to catch.

The eight short-action rows exercise both orders for issuance, share
revocation, user-session revocation, and kick; they pause at the actual
pre-fence/fenced boundary, assert every derived key remains locked for the
action-first case, inspect SQLite effects, and check that revoked-principal
cases have no effect or secret ([10882–10970](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10882)).
The five directory rows separately cover create/rename and confirm that
configured-admin status, rather than an incidental current space role, is the
principal capability ([10975–11073](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10975)). Root reported this native cohort GREEN (`0esTMv/00`, SHA `961b1ec3a6c48eccadfee304e3579fa13d02063bd1d2f8e1536951f7f39fa0e3`); this audit did not run it.

No concrete short-intent bypass or lock inversion is present in the reviewed
source. One P1 acceptance gap remains: the fixture asserts binding *counts*
for four short kinds and only runs create/rename among mapped directory
commands. Add a pure schema-backed binding matrix for all 14 `AdminIntentV1`
variants, asserting the exact sorted key vector (not merely its count): include
space-only directory edits, `RemoveSpaceMember`'s target membership,
`RevokeDocumentShare`'s scope+share pair, `RevokeUserSessions`' deduplicated
self-target, and the explicit `None` long-rebuild classification. This is
coverage hardening; match exhaustiveness already prevents a newly added intent
from silently bypassing the helper. It does not expand the short-matrix claim
to long rebuild, multiwriter share scope atomicity, uncertain HTTP/backend
commit, or physical WebSocket kick closure.

#### Correction — GIS Committer Close/Admission P0 Is Repaired in Current Source

The preceding GIS close P0 describes the superseded state. Current `close()`
sets its closing latch while holding `cleanup_jobs`; `reserve_cleanup_job`
holds that same mutex while it checks the latch, so a post-close commit cannot
introduce an unseen reservation ([502–537](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:502),
[1997–2055](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:1997)).
Successful close now requires all three domains to be empty: document owners,
maintenance keys, and cleanup reservations. `release_cleanup_job` emits its
epoch only after mutating and unlocking the cleanup map. The new native-shaped
law creates a valid unpolled commit, first-polls close to `Pending`, proves
late admission is `Unavailable`, drops the original owner, and requires exact
abandonment/outbox drain before terminal database and pool shutdown
([3742–3851](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3742)).

This closes the owner/quiescence and lost-wake defects in source. WGPU reports
its source oracle GREEN; neither that report nor this audit is a native runtime
qualification.

### Resumable Copy-on-Write Directory Projection Rebuild — Implementation Packet

#### Current Boundary and Why It Cannot Resume

There is no reusable Hub projection-generation or staging facility. The nearby
framework `ArtifactProjectionStamp` is a process-local artifact-result
freshness stamp, not a durable Hub-directory generation
([store source](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3486)).

Each backend takes its directory writer and deletes its live read model before
replaying the whole log in one transaction:

- SQLite starts `BEGIN IMMEDIATE`, snapshots credentials into temporary tables,
  deletes live projections, then replays 512-event pages
  ([2034–2148](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2034)).
- PostgreSQL locks `hub_directory_event_head ... FOR UPDATE`, deletes the same
  live tables, and replays under that transaction
  ([2206–2313](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2206)).
- Neo4j obtains the `DirectoryCounter` writer lock, deletes live nodes and
  relationships, then replays under one graph transaction
  ([2082–2150](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2082)).

`ProjectionRebuildControl` only has synchronous cancellation/progress methods
and the public operation is just an in-memory deadline/counter
([581–615](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:581),
[6425–6455](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6425)).
After fifteen seconds, `reconcile_stale_admin_acceptance` terminally labels an
unowned accepted operation interrupted ([6349–6375](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6349)).
That is right for a nonresumable task but cannot be the source of truth for a
durable rebuild cursor.

The old control deliberately proves writer exclusion through
`DirectoryRebuildWriterProbe` ([4964–5051](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:4964)).
That assertion must be retired: COW keeps the active generation continuously
readable/writable and reserves writer exclusion only for bounded start,
catch-up, and publish steps.

#### Required Durable Schema Split

Keep these as durable authority/ledger data, outside a projection generation:
`hub_user`, `hub_auth_session`, `hub_sync_session`, `hub_share_grant`,
`hub_space_invite`, audits/receipts, `hub_directory_event`,
`hub_artifact_authority_journal`, `hub_artifact_cas_ledger_journal`, and CAS
head/barrier/delete-lease tables. `hub_user` has auth-session foreign keys and
contains non-event credential/identity material; it must no longer be deleted
and recreated during a read-model rebuild. A replayed `UserCreated` instead
checks the durable identity's immutable event fields. `hub_space_invite` must
lose its current foreign key to generation-scoped `hub_space`; live
`SpaceDeleted` deletes its durable invite rows, while invitation scope
preflight joins the active generation's space. The current SQLite FK is at
[160–171](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:160).

Generation-scope all reconstructable state:
`hub_space`, `hub_space_membership`, `hub_document_descriptor`,
`hub_artifact_checkpoint`, `hub_artifact_checkpoint_private`,
`hub_artifact_retention`, `hub_artifact_cas_reservation`,
`hub_artifact_cas_reservation_object`, `hub_artifact_cas_reference`, and
`hub_artifact_cas_reference_object`. Add `projection_generation` to identity,
every FK, and every active/lineage/object lookup index. The current global
checkpoint `event_seq UNIQUE` and partial active index
([72–114](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:72))
must become generation-qualified; otherwise a candidate cannot contain the
same log projection as its active predecessor. CAS's existing `generation` is
the immutable ledger sequence, so the new column needs the distinct name
`projection_generation`.

Add two domain-owned records to every backend schema:

```
hub_directory_projection_generation(
  generation PK, state building|active|retired|discarding,
  materialized_event_seq, materialized_cas_generation, created_at_ms
)
hub_directory_projection_rebuild(
  operation_id PK, candidate_generation UNIQUE,
  source_head_seq, event_cursor_seq, cas_cursor_generation,
  phase event-replay|cas-replay|catch-up|published|cancelled|failed,
  cancel_requested, updated_at_ms
)
```

`hub_directory_projection_active(singleton PK, active_generation,
published_head_seq, published_cas_generation)` is the sole publication pointer
and starts at generation 1. The rebuild row, rather than
`hub_admin_operation_audit`, is resumable state; the latter remains an
append-only public audit/receipt referencing the same server-issued operation
ID.

For Neo4j use an exclusive `DirectoryProjectionActive {id:'singleton'}` node,
`DirectoryProjectionGeneration {generation}`, and
`DirectoryProjectionRebuild {operationId}` nodes. Give reconstructed Space,
DocumentDescriptor, ArtifactCheckpoint/Private/Retention, CAS nodes and
membership relationships a `projectionGeneration` property and make unique
keys generation-qualified (for example encoded
`projectionGeneration + ':' + scopeKey`). `User`, session, invite, share,
event, private authority, and ledger nodes remain unversioned.

#### Core API and Step Protocol

Replace the all-log `HubDirectory::rebuild_projections[_controlled]` contract
at [2691–2697](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2691)
with a durable-operation seam. The existing `RebuildDirectoryProjections`
intent can retain its client fields; its server-issued operation ID becomes the
rebuild key. Add schema-first Rust/TypeScript/JSON twins in
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️.rs,🟦️.ts,🔣️.json}`
for:

```
DirectoryProjectionRebuildPhaseV1
DirectoryProjectionRebuildProgressV1 {
  phase, candidate_generation, source_head_seq,
  materialized_event_seq, materialized_cas_generation, cancel_requested
}
```

Expose only these backend methods through `HubDirectory` and its
`HubDirectories` forwarding implementation:

```
start_projection_rebuild(operation_id, expected_head_seq) -> Progress
step_projection_rebuild(operation_id, PageBudget) -> Progress | Published
cancel_projection_rebuild(operation_id) -> Progress
```

`PageBudget` is an internal fixed value (for example 128 events or ledger
entries), never caller input. `start` takes the normal directory writer only
long enough to snapshot both heads and allocate one building generation.
`step` commits one candidate page plus persisted cursors in one short
transaction, never touching the active pointer. A dropped HTTP task or process
restart can reopen the same marker; it cannot replay from zero or expose a
partial generation.

Candidate replay needs two immutable sources: event pages through captured
`source_head_seq`, including every checkpoint's
`hub_artifact_authority_journal` payload, and CAS ledger pages through captured
CAS generation. The current rebuild already demonstrates both inputs
([2091–2150](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2091),
[2130–2160](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2130)).
Replaying events alone would create a plausible checkpoint view with stale CAS
reference/reservation protection.

The final `catch-up` step locks the existing writer primitive, reads both
current heads, and projects at most one new event/CAS page into the candidate.
It commits its cursors and yields if either tail remains. Only when both
cursors equal both heads *in that transaction* does it set the active pointer,
mark the candidate active, and retire the prior generation. This prevents an
event `H+1` or standalone CAS reserve between the initial snapshot and pointer
flip from disappearing. PostgreSQL already has the directory lock row at
[249–253](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:249)
and CAS head update at [407–409](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:407);
Neo4j has equivalent `DirectoryCounter` and `ArtifactCasLedgerHead` mutation
locks ([85–88](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:85)).
Finalize in the established publication order: directory head, CAS head, then
active-pointer row/node. CAS-only reservation paths take a shared
active-pointer read after their CAS head lock; final publication waits for
that reader, so a concurrent reserve cannot write the retired generation after
the flip.

All normal writers need `project_active(tx, event)`, not the unparameterized
`project`: read/lock the active pointer in the existing transaction and apply
to that generation. SQLite call sites include normal decided appends
([2005–2007](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2005),
verified checkpoint publication ([1809–1814](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1809),
and invite redemption ([1520–1525](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1520)),
with Postgres/Neo4j twins. Split `SpaceDeleted` into live durable capability
cleanup (share/invite deletion) and generation-local read-model deletion.
Historical replay performs only the latter. Candidate replay also must not
create/delete sessions, shares, or invitations. This removes the current
temporary credential snapshots rather than trying to preserve them across a
long transaction.

Every reader of generation-scoped tables must derive the active generation in
the same statement/transaction: space/list/member/role methods at SQLite
[943–1072](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:943),
descriptor/checkpoint/retention methods [1080–1155](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1080),
invite scope checks [1467–1514](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1467),
and CAS protection/sweep queries [1667–1958](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1667).
Mirror that census in Postgres and Neo4j. One old unqualified CAS query after
the flip could falsely collect a live blob.

Retired-generation deletion is a separate bounded GC phase, never part of
publication. Keep at most one building and one retired generation; a third
admission returns `Capacity` without changing the active pointer. A later
bounded GC deletes retired rows after active-pointer joins make them
unreachable to normal reads.

#### Authority and Publication Ownership

`execute_admin_intent` intentionally returns no short authority union for
`RebuildDirectoryProjections` ([6513–6515](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6513))
and runs an in-memory ten-second task ([6645–6664](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6645)).
Do not replace that with a long-held `User`/`Session` guard. Persist an exact
principal snapshot in the rebuild row (user ID, session ID, authorization
generation, expiry, configured-provider/subject digest) solely to revalidate
each fresh driver step.

Before every page, the Hub driver acquires a new short sorted `User` +
`Session` union and compares that snapshot using the existing
`socket_session_binding` seam. If revoked before a checkpoint, it writes a
cancel/discard state; active generation is unchanged. A revocation after that
check can consume at most one nonvisible page. Before final catch-up/publish,
the driver holds that same short union through the pointer flip. Therefore
revoke-first means no flip, while publish-first means a complete,
already-authorized generation is published and revocation follows normally.
Neither path holds authority for the whole replay.

`AdminOperationRuntime` may remain an optional wake/progress cache, but status,
cancel, restart, and terminal audit must read the durable rebuild marker.
`reconcile_stale_admin_acceptance` must join/resume a live marker rather than
write `interrupted-before-terminal`; cancellation writes the marker before the
public terminal audit. `cancel_admin_operation` must set its durable cancel
request even where no local runtime exists.

#### Exact Backend/Public Work Map

- **SQLite:** extend `SCHEMA` in
  [sqlite directory](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:37),
  parameterize `project`, `project_verified_checkpoint`, CAS helpers and all
  reader SQL with active/candidate generation. Use short `IMMEDIATE`
  transactions for start/page/catch-up; its connection mutex then covers one
  page, not the complete rebuild.
- **PostgreSQL:** extend its embedded schema, parameterize projection/CAS SQL
  and query predicates, and use `hub_directory_event_head FOR UPDATE` plus the
  CAS-head row during final catch-up. Existing destructive rebuild starts at
  [2206](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2206).
- **Neo4j:** add active/job/generation nodes and constraints beside graph
  schema; make Space/descriptor/checkpoint/CAS keys and membership relations
  generation-bearing; route every `MATCH (:Space ...)` and CAS-protection read
  through the active node. Use current counter/CAS-head mutations for bounded
  final serialization, never a long-lived graph transaction.
- **Hub/public schema:** update the three framework schema twins and
  `🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1`; add phase/cursor/generation
  fields to `AdminOperationProgressV1` so a restarted client can distinguish
  building, catch-up, cancelled, failed, and published. Change operation
  spawning/status/cancel/reconciliation at
  [6349–6756](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6349)
  into a bounded durable-step driver.

#### Acceptance Matrix

Add a schema-first `projection-rebuild-cow-v1` fixture under
`🌎️hub/📇️directory/🧫️fixtures` with a TypeScript oracle, registered through
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, then prove these physical laws:

1. A paused candidate page is invisible: active role/descriptor/checkpoint/CAS
   reads remain complete while a second connection appends a normal event.
2. Resume catches that post-snapshot event and post-snapshot CAS reservation;
   publication atomically exposes both and no active object loses CAS
   protection.
3. Drop/reopen after event replay and after CAS replay preserves the candidate
   generation and cursors; it neither restarts at zero nor flips early.
4. A missing private checkpoint-journal payload or malformed ledger payload
   fails only the candidate; active projection bytes, pointer, and durable
   capabilities are unchanged.
5. Revoke the initiating session at a page gate: the next checkpoint cancels
   and discards without a pointer flip or retained session lock. Race the final
   gate in both orders to prove revoke-or-publish linearization.
6. A live `SpaceDeleted` deletes durable shares/invites exactly once, while a
   candidate replay of that historical event cannot mutate those durable rows.
7. Fill one building plus one retired slot and require a third start to return
   capacity; bounded retired GC then permits the next start.
8. Run the contract against SQLite, PostgreSQL, and Neo4j. SQLite needs two
   connections to one generated file and page/final gates; Postgres/Neo4j need
   their concrete backend suites, not a memory-only substitute.

This is the minimum coherent replacement for the current full-lock rebuild. It
keeps normal writes available, makes cancellation/restart durable, gives
session revocation a bounded cutoff, and publishes only a generation caught up
through both authoritative logs. No runtime qualification was run in this
audit.

### Cold `HubState` Open Lost-Wake Audit (2026-09-07)

**Concrete cause found — not a pool-capacity or registered-backend-retirement
claim.** `run_socket_test` creates a Tokio *current-thread* runtime
([Hub bin:8021–8031](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8021));
there is no second Tokio worker that can incidentally repoll a task which was
never woken. `test_state_with_directory` first awaits
`Database::open_at` ([8045–8047](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8045)).
`open_at` performs a filesystem `BackendOpen`, then immediately enters
`Database::open_with`'s capability probe before catalog read/bootstrap
([DB engine:8016–8019](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8016),
[8074–8100](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8074)).

The `pf3kjS` failed root being present but empty is consistent with the first
filesystem task having run; it does not prove the catalog stage was reached.
The blocking filesystem `BackendOpen` publishes its terminal and takes the
waiter while holding the same task-slot mutex that public polling uses
([storage:3948–4065](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3948),
[4735–4765](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4735)).
That path has no check/register wake window. Its `finish()` also actively
drives terminal retirement, so the recent FS/SQLite opening-retirement repair
is a separate issue from this evidence.

The capability probe does have the window:

1. Its public `poll` takes `completion` under `completion`'s mutex
   ([engine:982–991](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:982)).
2. It then releases that mutex and separately stores `context.waker()` under
   `waker`'s mutex, immediately returning `Pending`
   ([992–993](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:992)).
3. A process I/O worker can run `publish_staged`/`complete` in between. It
   writes `completion`, then finds no registered waker to wake
   ([engine:403–406](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:403),
   [706–734](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:706)).
4. The one current-thread Tokio task stays asleep until the outer five-second
   timeout drops it. This is an exact lost wake, independent of worker
   throughput or a deadline choice.

The capability backend is deliberately scheduled onto `Lane::Io`
([422–438](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:422));
the pool is process-wide and its workers are native threads
([async:1757–1784](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:1757),
[2765–2770](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️.rs:2765)).
That cross-thread completion is precisely what makes the short registration
window reachable. It is not evidence of a scheduler deadlock.

#### Bounded Repair and Proof

Make capability public polling match the already-correct catalog variants:
`take completion` → install transient waker → **take completion again** → if
present, clear the transient waker, mark `resolved`, apply the same error
`abandoned` handling and `release_success`, and return `Ready`; otherwise
return `Pending`. No new queue, timer, or blocking wait is needed. The locks
are not held across one another in either the existing first check or
`complete`, so this preserves the present lock ordering.

Add the test-only controlled hook exactly after the first capability
completion check and before waker insertion, analogous to the existing
catalog-read hook at [engine:2080–2092](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:2080).
The deterministic unit law should use the existing controlled capability
probe ([10519–10618](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10519))
to publish the exact ready result in that hook, manually poll once, and require
`Ready` with the same storage pointer, zero dependence on a later wake, no
transient waker, and released admission/registry after result handback. A
second row publishes an error in the same window and proves the existing
abandoned/terminal-owner protocol still retains and drains its exact owner.

For scope: `DatabaseCatalogReadFuture` already rechecks after registration
([2073–2092](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:2073)),
as do bootstrap ([3446–3463](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:3446))
and create-catalog ([7248–7265](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7248)).
Do not broaden this fix to those paths or attribute the intermittent `test_state`
open to the independently repaired FS/SQLite registration lifetime.

#### Follow-up: Exact Engine Public-Future Boundaries

The other currently unprotected engine public future is **history**, not
compaction. Engine [9943–9953](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:9943)
is `HistoryFuture` (the `ArtifactHandle::history()` entry point is at
[10323–10330](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10323)).
It takes `completion`, then writes `waker`, and returns `Pending` without a
second completion check. Its actor worker publishes in
`ArtifactHistoryState::complete` ([9516–9526](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:9516)).
Thus the same cross-thread check/register lost wake is reachable for retained
history replay. The actual compaction entry path is an actor `AskFuture`
([8547–8554](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8547),
[10209–10220](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10209));
this audit found no separate compaction future at that location.

Apply exactly the same check/register/recheck construction to `HistoryFuture`.
A test-only hook between its first check and registration should synchronously
publish a controlled `Err(DbError::Closed)` outcome and require the *same*
manual public poll to return `Ready`, then drive the existing terminal cursor
to preserve all history reservation/work retirement obligations. This is a
small engine-future law, not an expansion of the Hub-open fix.

There is a separate, concrete capability **success-retirement** interleaving
even after the public recheck is added. `DatabaseCapabilityOpenState::complete`
writes `completion`, drops its mutex, then calls `wake_waiter`
([409–419](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:409)).
`DatabaseCapabilityOpenFuture::poll` may legally be spuriously polled in that
interval: it consumes the Ready result, but `release_success` declines because
the prior public waker is still retained in `roots_are_empty`
([841–850](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:841),
[736–751](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:736)).
The publisher then removes and wakes that obsolete waker; no owner necessarily
calls `release_success` again, leaving its admission slot and registry entry
live.

The narrow repair is to call `release_success()` once more after
`wake_waiter()` in capability `complete`. If the public result remains in
`completion`, roots are nonempty and this is a no-op; if the consumer won the
interleaving, the just-cleared waker lets the existing release procedure retire
the exact admission. It needs no change to the worker pool or queue protocol.
Prove it with a two-thread/barrier law: first manually poll Pending to install
W1; pause `complete` after storing the ready result but before taking W1;
manually poll the same future to Ready (spurious poll is permitted); release
the publisher; require `finished`, no registry generation, and reusable
admission. Exercise both a successful owner and an error owner whose existing
terminal/abandonment routing must remain intact.

### Capability Completion TDD Review (2026-09-07)

The new test-only hooks model both causal intervals precisely without a wall
clock: `controlled_publication_before_waker_hook` runs after the first public
completion check, and `controlled_completion_before_wake_hook` runs after the
publisher has stored completion but before it takes the existing waiter
([engine:346–358](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:346),
[409–425](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:409)).
The first law's before-registration rows force publication *only* in the
public check-to-registration window; the second has a real publisher thread
parked after publication, then polls the same public future before releasing
that thread ([11247–11335](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:11247)).
Those are deterministic causality tests rather than retries or timing tests.

The planned repair is sufficient and preferable to changing publisher
terminalization: every Ready path clears the transient waiter before
`release_success`; the post-registration completion read applies the identical
resolved/error-abandoned bookkeeping, clears that just-installed waiter, and
returns Ready. It covers both publication-before-registration and
consumer-before-publisher-wake. Clearing is safe: one `Future` has one mutable
poller; a Ready call itself owns the result and must not later wake that same
completed future. No counterexample requiring a queue, a deadline change, or a
publisher-side terminal call was found.

The fixture has the required three publication placements × success/fault and
the success/fault consumed-before-wake rows
([fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️fixtures/📬️capability-completion/🔣️.json)).
It checks first-poll readiness, wake count, exact returned storage pointer for
success, exact error for fault, fault abandonment bookkeeping, empty waiter,
and terminal-empty admission cleanup. One small strengthening is warranted:
after each row, explicitly assert that
`database_capability_open_registry()[state.slot]` is `None` (or run an exact
same-slot reuse check). `terminal_is_empty` proves the local admission and
roots but does not itself prove a registry removal bug could not strand the
generation. The schema's fixed lengths do not by themselves guarantee all
named Cartesian rows remain present; the TypeScript/source gate should retain
the existing eight exact names or validate the pair set, rather than relying
on count alone.

### Authenticated GIS Cold-Map Browser Acceptance Frontier (2026-09-07)

**Current source-law fact — not a native receipt.** The new GIS cold-map law
is source-present but has not run against a freshly materialized GIS WASM.
When run, it opens the genuine GIS component, applies the canonical Pack/SPR
pair, deliberately sends
`SurfaceVisible`, then checks a `tiled-map` `UiPatch`; its follow-up
`patchPositions` action yields a later scene/revision
([component law](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs:192),
[pair/action assertions](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs:254)).
The separate `browser-actor-gis-describe-check` target only derives the actor
and invokes `describe`; it explicitly has no Hub session, renderer, mutation,
or peer route ([target](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📋️project.json:476),
[implementation](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5495)).
Neither is evidence that a Map became visible in Shell.

There is a real renderer endpoint to exercise once the actor has emitted that
patch: `ShellHost` verifies the runtime/surface/generation/revision before it
ACKs a `browser-actor-ui-patch`, and the interpreter maps `"tiled-map"` to the
real `TiledMapHost` canvas rather than a placeholder. Thus the browser oracle
should assert both the accepted UiDocumentStore patch body and the mounted
`.semio-tiled-map-host` canvas; canvas pixels alone are not stable map-state
evidence.

#### Current Route and Runtime Preconditions

The nearest real two-browser harness is `@semio-tech/framework-os-dev`'s
`collab-e2e`: it starts a real SQLite Hub, two independent React `s` servers,
and two Playwright contexts ([Hub process](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2403),
[two user servers](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2528),
[two-context orchestration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2887)).
It is the right reusable launch base, through
`bun nx run @semio-tech/framework-os-dev:collab-e2e`, after a GIS-specific
variant adds the verified GIS artifact/bundle to its narrowly selected
prebuild inputs. The current list builds host defaults plus `writer`, not GIS
([prebuild selection](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2430)).

The Hub must start with SQLite and `native-artifact-execution`, a verified
catalog that contains the GIS binding, a configured canonical artifact
authority, and open-plan readiness. Otherwise it deliberately leaves
`inference_runtime` absent; with all inputs present it creates the retained
GIS committer and publisher ([Hub construction](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7550)).
The browser still needs a normal authenticated document opening relay carrying
`spaceId`, `documentId`, and schema before `ShellHost` can call its existing
`openDocument` path. That payload is already present on the actual Space
editor route: create, row-open, and open-with emit those three fields together
with the artifact reference
([create](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs:44),
[row open](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️open-artifact/🦀️.rs:23),
[open with](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️open-artifact-with/🦀️.rs:24)).
The WIT conversion preserves its complete `args`
([reactor](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1629));
`ShellHost` resolves the tuple then invokes `openDocument`
([handoff](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3797)).
The generic `AppCommand::OpenArtifact` helper does omit the tuple
([generic helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32069)),
but it is a separate synthetic app-channel route and must not be widened with
browser-chosen document identity. The existing collaboration assertion is
therefore stale about its asserted cause
([diagnostic](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2692)).

#### Three Concrete Blocking Gaps

1. **Initial cold render — source-qualified browser repair.** Root reports
   that the generic `SurfaceVisible`/reconcile/ACK/wake law is now green in
   the lightweight browser-worker test. The implementation sends the initial
   surface turn only after the terminal cold-pair receipt, then applies a
   bounded patch-feedback/wake loop
   ([worker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1397)).
   This is not a native GIS component or Shell receipt. The genuine GIS WASM
   still has not been freshly materialized and run in this path.

2. **Direct Space opening is present; its server-authorized outcome needs an
   end-to-end proof.** `documentId`/`spaceId`/schema from the effect are only
   routing hints. `openDocument` sends them into the Hub-backed worker, whose
   plan/lease response is the authority before target acquisition
   ([worker open](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4095)).
   Do not add a replacement tuple to `AppCommand::OpenArtifact`. Replace the
   stale collaboration diagnostic with a direct Space-command proof that the
   server admits the precise scope and that a foreign/malformed hint cannot
   attach a backbone. The same proof must resolve whether a create-and-open
   effect is deliberately sequenced after its artifact mutation reaches the
   Hub; source inspection alone does not establish that timing.

3. **No normal checkpoint-to-live-peer refresh.** A successfully approved GIS
   checkpoint is published by the retained publisher, but the document socket
   emits `RebootstrapRequired` only after broadcast lag
   ([socket loop](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4402)).
   The client correctly treats that frame as a scope-checked canonical reload
   ([worker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2758));
   ordinary `Commands` merely update the local frontier and do not feed the
   actor ([2799](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2799)).
   Therefore approved publication currently cannot refresh either already-open
   actor. Add a receipt/frontier-bound post-publication notifier that uses the
   same verified `RebootstrapRequired` control and existing canonical reload,
   emitted only after terminal checkpoint publication. It must emit no frame
   for cancellation or failed publication; it must never carry Pack/SPR bytes.

#### Minimal Browser Acceptance (One GIS Variant of `collab-e2e`)

1. Start the existing real Hub/two-Playwright-context harness with the
   verified GIS catalog/bundle and create/open one server-authorized GIS
   document in a shared space from both contexts. Require the same scope,
   descriptor digest and baseline frontier at both actor leases.
2. In both contexts, require the verified cold-pair load, `tiled-map`
   UiDocumentStore patch ACK, and a visible `TiledMapHost` canvas. Explicitly
   reject `renderer-unavailable`; on close, require child actor and byte
   capacity return to zero.
3. Invoke the registered `proposeBoundsRegion` action through the actual GIS
   surface. Observe the real `InferencePortPanel` progress/events, then cancel
   a pre-approval job and require no checkpoint notification or scene/frontier
   change. Run a second job to approval and require its retained receipt to
   reach published/applied.
4. Require both already-open sockets to receive the newly emitted,
   scope-valid rebootstrap control; both must install the exact published pair
   and render the changed `tiled-map` patch. Assert the new frontier and the
   semantic patch body, not pixels, in both contexts.

**Important semantic correction.** Today's server-stamped AI route permits
only `CreateRegion` with id `inference-<jobId>` and kind
`inference-bounds` ([route validation](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2526)).
It does **not** approve `patchPositions`. The GIS component's local
`patchPositions` law proves a distinct editor action, not the AI approval
result. Thus the honest initial acceptance asserts the approved region's
appearance in both Map scene payloads. A requirement that an AI approval move
positions needs an explicit new, server-stamped position proposal and its own
durable three-store/peer propagation proof; it must not be silently asserted
against the present bounds-region implementation.

### Shell Document Opening: First-Install Selection and Attempt Correlation (2026-09-07)

#### Current First-Install Blocker

The normal Shell cannot currently reach the already sound private first-install
path. `PersistenceBinding` names `requestedSurfaceId` and optional
`installedTarget` ([wire type](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:630)),
but the sole normal binding factory emits a stale `surface` member instead
([factory](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts:32)).
There is no production `installedTarget` producer in `ShellHost`; the only
current use is a scoped-presence test configuration
([test-only binding](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/👥️presence-scope/🌐️browser/🟦️.tsx:46)).

Consequently the worker's outer `openArtifact` condition rejects the real hub
open before it requests an open plan
([early rejection](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4069)).
That is a real production blocker, not an absence of a trusted descriptor in
the loaded renderer plugin. `LoadedProgramState` deliberately holds only
`handle` and `manifest` ([state](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:116));
a manifest must never be promoted into lease fields.

The lower algorithm already has the correct authority boundary. With a
requested surface but no installed target, it requests the server plan,
downloads the bounded manifest/component/descriptor, verifies their plan-bound
digests, privately mints `DocumentExecutionTargetLease`, then compares the
full receipt-free `lease.fields()` projection before accepting a socket grant
([first install](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1693)).
This is the sole source of a first execution-target lease. No Shell-derived
component, descriptor, length, hash, catalog row, or a fake `installedTarget`
is needed.

The bounded repair is therefore exactly:

1. Have the normal opening factory pass
   `requestedSurfaceId: canonicalSurfaceId(target.app.dialect, target.app.role)`
   in its hub binding, never `surface`.
2. Replace the outer `installedTarget === undefined` rejection with
   `requestedSurfaceId === undefined && installedTarget === undefined`.
   The existing `requestDocumentSocketAuthority` branch remains the only
   first-install/mint path and remains fail-closed for no selection.
3. Add a source/neutral worker law using a hub binding with only a valid
   `requestedSurfaceId`: it must make the exact open-plan, manifest, component,
   descriptor, and grant requests; mint no public raw-byte owner; and reach the
   same plan/socket relation. A no-selection row must produce the existing
   `installed-target-unavailable` result without issuing any asset request.
   This is a worker fixture law, not evidence of an authenticated real GIS
   browser session.

#### P0 Follow-on: Runtime Key Does Not Identify One Shell Opening

The just-corrected scope routing still does not make asynchronous lifecycle
messages owner-safe. `openDocumentSessionsRef` and `socketActorReadyRef` are
both keyed solely by `runtimeKey` ([maps](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1680)).
`socket-actor` and `socket-actor-failed` carry only scope/document/actor or
fault, and unconditionally settle and delete that key's current waiter
([consumer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1751)).

There is a concrete replacement trace:

1. Opening A installs entry and waiter `WA` for runtime `K` and posts `open`.
2. Opening B for the same `K` replaces both maps with B/`WB`.
3. A's queued old `socket-actor` can resolve `WB`; alternatively A's timeout
   unconditionally deletes `WB` in its `finally`
   ([A cleanup](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4110)).
4. A can then attach its old plugin/session or post a scoped directory open
   after B is current. A stale `close` likewise has no generation and closes
   whatever state currently occupies `K`.

The automatic space-index path duplicates exactly the unsafe entry/waiter/open
sequence ([index open](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6184));
repairing only `openDocument` would leave this second producer exploitable.

##### Minimal Wire Contract

Reuse the already bounded, non-secret `clientInstanceId` vocabulary rather
than inventing a second bearer-like identifier. It is already part of the
server-validated `DocumentOpenIntentV1`, capped at 128 UTF-8 bytes
([schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1071)),
and the worker already captures it as the exact plan/asset owner
([capture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1640)).
The Shell must generate a fresh UUID for every document-opening attempt before
the `open` post; the worker must use that supplied value rather than minting
one after it has lost the Shell correlation opportunity. It identifies one
client lifecycle only; it neither authenticates a client nor authorizes a Hub
operation.

Make it a required field for every **Shell document** request and its matching
response, while internal local-only worker users may carry no value:

```ts
type BackboneWorkerRequest =
  | ({ kind: "open"; clientInstanceId?: string } & ArtifactActorConfig)
  | { kind: "send"; documentId: string; spaceId?: string; clientInstanceId?: string; message: ArtifactActorMsg }
  | { kind: "close"; documentId: string; spaceId?: string; clientInstanceId?: string };

type ShellDocumentEntry = {
  session: ActiveSession; plugin: PluginWasmHandle; documentId: string;
  scope?: DocumentScope; clientInstanceId: string;
};
```

`clientInstanceId` must be exact UUID-shaped/within the existing 128-byte
schema bound in the TypeScript binary decoder before dispatch. In the Rust
binary counterpart it is an optional bounded `String` on
`BackboneWorkerRequest::{Open,Send,Close}` and on
`BackboneWorkerResponse::Event`; the local Rust artifact host need not treat it
as document authority. The wasm worker closes/sends only when its retained
owner's value equals the request value. `documentExecutionOwners` must retain
the value too: a same-key different value first dispatches a close stamped with
the old value, a same-key same-value duplicate is inert, and stale send/close
is inert. This covers both TypeScript and Rust selection rather than allowing
a stale close to cross a backend-owner handoff
([dispatcher](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:190),
[Rust framing](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:605)).

Every state-derived response must echo this value: `event`, bootstrap
progress/failure/rebootstrap, `socket-actor`, `socket-actor-failed`, and
execution-target status. The main-thread decoder must reject malformed values
for those document responses. `directory-scope-revoked` deliberately remains
scope-only: it is a server revocation of the current authority, not an old
opening completion.

The browser patch handoff needs an outer Backbone envelope value as well. Its
domain-neutral `BrowserActorUiPatchOfferV1` parser deliberately rejects
additional keys ([strict parser](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts:225)).
Do not pollute that WIT/receipt contract. Instead, strip/validate
`clientInstanceId` in `decodeBackboneWorkerResponse` before invoking the
existing patch-offer parser; reattach it only to the outer
`BackboneWorkerResponse`. The Shell echoes it in the patch result, and the
worker checks it against the current `ArtifactState` before calling
`settleUiPatch`. This prevents a queued old base-revision-zero patch from
becoming B's first UI store after B cleared the old store.

##### Exact Producer and Consumer Census

All these sites must migrate in one contract change:

| Boundary | Current site | Required owner operation |
|---|---|---|
| Normal Shell opening | [openDocument](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4069) | Mint the UUID before maps/routes, store it in entry+waiter+open request, and after every await prove the exact entry still owns `K` before directory-scope open, attach, or UI dispatch. Its `finally` deletes only its own waiter. |
| Automatic space index | [manual open](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6184) | Remove its bespoke map/waiter/post path by sharing normal opening admission, or mint/store/pass the same token and use identical post-await assertions. |
| Explicit close and send | [close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4126), [plugin relay](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2998), [heartbeat](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4952) | Stamp the retained entry token. After `ephemeralSnapshot` awaits, re-read that entry/token before posting. A cancellation from an old bootstrap must not close B. |
| Socket and ordinary events | [worker message handler](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1725) | Require `entry.clientInstanceId === message.clientInstanceId` before resolving/rejecting a waiter, changing sync/presence/bootstrap/status state, dispatching mutations, or accepting a patch. After `loadAppDocumentPack` or `applyMutations` awaits, recheck before publishing any result. |
| TS worker lifecycle | [dispatch owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:202), [state open](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4013), [session posts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2878), [status posts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:939) | Retain the request token in the owner map and state, stamp all state-derived frames, and match it before dispatching send/close or settling a patch result. |
| Native/wasm worker | [wire enum](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:605), [host handler](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🦀️.rs:39) | Carry the optional token through open/close/send and the spawned event-forwarder. A stale close/send must be discarded by its retained document entry, not merely route by `document_id`. |

The `clientInstanceId` only proves one Shell-local attempt. It cannot stop an
already admitted `loadAppDocumentPack` call from changing a reused plugin
instance if replacement happens *during* that non-cancellable guest await.
The above post-await check prevents stale Shell publication, but not that
in-guest mutation. Treat that as a separate bridge obligation: either await
terminal retirement of the old session before admitting a same-instance
replacement, or give `PluginWasmHandle.loadAppDocumentPack` a retained
operation/abort owner. Do not claim the wire token alone cancels guest work.

##### Required Executable Laws

Use one schema-first `document-opening-attempt-v1` corpus beside the existing
bootstrap/session fixtures
([bootstrap fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️artifact-bootstrap-owner-v1.json),
[session fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️browser-actor-session-v1.json)).
The worker and Shell source gate should run its same rows independently:

1. A opens, B replaces the same runtime, then old A success/failure/progress,
   event, UI-patch and `close` arrive. B's waiter/session/route/UI/store and
   worker state remain exact; no A message resolves or deletes B.
2. B success resolves exactly once; A's timeout `finally` cannot delete B's
   waiter. A's stale send and heartbeat are ignored, B's exact send succeeds.
3. Same-token duplicate open is inert; reused/malformed/absent token against a
   token-owned entry is denied without sending, closing, or allocating a
   successor. A matching close releases precisely A's route, directory scope,
   browser UI, waiter, and worker owner.
4. An old `browser-actor-ui-patch` with a valid inner receipt but A's outer
   token is ignored and receives no ACK; B's exact offer is ACKed once.
5. A replacement during `loadAppDocumentPack` records no stale Shell state
   publication; a separate bridge row documents whichever retained cancellation
   or serial retirement mechanism is chosen for the in-flight guest call.

These laws establish local lifecycle correlation only. They do not prove a
Hub-authenticated open, real GIS materialization, a visible map, or peer
delivery.

### Retained Short Administrator: Current Source Audit (2026-09-07)

This is source inspection only. The source gate is reported GREEN by the
owner, but no native execution result is inferred here.

#### Confirmed retained boundary

The handler appends an `accepted` fact, obtains one of the 64 owned permits,
installs `AdminOperationRuntime` in the process map, and moves the permit,
principal, request identity and one-shot sender into the future before
`AdminOperationTaskOwner::spawn`
([admission](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6736)).
Thus an HTTP future awaiting the receiver is not the task owner. Its dropped
receiver does not cancel the retained future. `AdminOperationCleanup` owns
both the runtime-map removal and permit release
([cleanup](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6457)),
and shutdown first closes admission, marks every retained runtime cancelled,
waits the fixed 11 seconds, then aborts and joins remainder
([owner shutdown](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6484)).

The current authority vector is held as `_authority` across each actual
directory/share writer await; it is dropped only when
`execute_admin_intent` returns. The terminal audit happens subsequently.
That is enough to fence the effect writer, but does not make the terminal
audit itself a transaction with the effect. Existing native source rows cover
dropped-request continuation, pre/post-admission cancellation and bounded
abort; they do not prove the remaining two boundaries below.

#### P0: cancellation has no single linearization point

`cancel_admin_operation` only stores `cancel_requested`; the executor first
loads that Boolean, later stores the independent `effect_started` Boolean,
then begins the effect
([cancel](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6386),
[admission](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6585)).
The exact interleaving is: runner observes `false`; cancellation stores
`true`; runner stores `effect_started=true` and commits. The cancellation was
accepted while the operation was still pre-effect, yet it cannot prevent the
effect. The current pause law does not force the check-to-store interval.

Replace the two cutoff Booleans with one atomic phase:

```rust
#[repr(u8)]
enum AdminEffectCutoverV1 { PreEffect, EffectAdmitted, Cancelled }
```

`cancel` performs `PreEffect -> Cancelled` by `compare_exchange`; the runner
performs `PreEffect -> EffectAdmitted` immediately before the writer. Only the
winner decides the outcome. A post-admission cancellation observes
`EffectAdmitted` and returns progress without claiming rollback. Add a paused
native row which stops the runner after its last authority/revocation check
but before this CAS, races the production cancel route, then proves exactly
one winner, zero/one writer accordingly, one terminal fact and full
slot/runtime release.

#### P0: bounded abort preserves uncertainty but lacks eventual reconciliation

`reconcile_stale_admin_acceptance` is intentionally a no-op
([function](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6371)).
An execution timeout, panic, terminal-audit failure, or forced shutdown abort
therefore drops `AdminOperationCleanup` (removing the process runtime and
releasing its permit) while leaving only the durable `accepted` audit fact.
Later idempotent requests return that `Accepted` receipt; no process owner,
backend query, or restart driver can determine whether the effect committed.
This is correctly non-fictional—no fabricated `cancelled`/`failed` record—but
it is not eventual reconciliation.

The narrow repair is a durable, private `AdminOperationEffectReceiptV1` keyed
by `(operation_id, intent_digest)`. Every short writer must atomically create
or complete that receipt in the same backend transaction as its actual effect;
the terminal-audit/restart driver queries it and appends a terminal fact only
when it proves the outcome. No receipt leaves the public audit projection. An
unknown record remains `Accepted` and may not be replayed. The current
per-request `correlation_id` is unique
([principal construction](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:2153))
and appears in the share auth audit, but that audit has no universal exact
operation receipt/query contract for all commands; it is not a substitute.

Required law: pause a real writer after the backend has committed its effect
but before `append_admin_operation_audit(terminal)`, force task abort/restart,
and prove the reconciler emits exactly one factual terminal without replay.
Repeat with an ambiguous/absent receipt and require `Accepted`, zero replay,
and bounded capacity reuse.

#### P1: dropped one-shot secrets are not wiped

Invite and share plaintext is exposed as `String` inside public
`AdminIntentResultV1` before the terminal audit
([schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:794),
[invite](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6633),
[share](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6649)).
The task deliberately does not store that type in the audit/runtime, and a
failed `oneshot::Sender::send` drops the undelivered value. Ordinary `String`
drop frees rather than wipes its heap bytes, however. The existing source gate
only establishes non-persistence; it does not observe erasure.

Keep an internal zeroizing encoded-secret owner through effect and terminal
audit, construct `AdminIntentResultV1` only after terminal persistence and
only if the one-shot receiver is still live, then explicitly wipe the
undelivered owner on closed receiver or audit failure. The public response may
remain the current schema; the private holder must never enter
`AdminOperationRuntime`, `AdminOperationAuditRecord`, or a retry/status
projection. Add a test-only wipe observer for both invite and share covering
(a) dropped receiver after effect, (b) terminal-audit failure, and (c) retry;
all must expose zero secret result and observe the exact candidate wiped.

### Shell Requested-Surface First Open: Follow-Up (2026-09-07)

The current scope helper now returns the protocol's actual
`requestedSurfaceId`, not the obsolete `surface` property
([helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts:40)).
The bare-space route also supplies the `spaceSession` and host plugin
explicitly to the normal opening function after a possible app switch
([route](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3981)).
That removes the ambient-space inheritance shown by the prior audit.

The installed-target absence is not a descriptor-provenance hole. At the
lower admission, `requestDocumentSocketAuthority` requests an authenticated
open plan and, where there is no existing React lease, installs the
server-verified target before using `lease.fields()` as the actual private
authority. It is therefore correct for a fresh Hub binding to carry only the
requested surface. The outer `openArtifact` precondition must reject a Hub
binding only when **both** `requestedSurfaceId` and `installedTarget` are
missing. No Shell-loaded-plugin descriptor is an acceptable substitute.

The owner reports a test-first RED followed by a selected source test GREEN:
requested-surface-only opening performs the five bounded HTTP stages
(`open-plan`, manifest, component, descriptor, socket grants), selects one
socket actor, and does not take the local socket failure path. This is a
bounded mocked server/socket source test, not native GIS, a Hub process, or a
real browser map acceptance result. The attempt-correlation follow-on above
remains required: the scoped first-open repair does not make a same-runtime
replacement safe.

### Plugin Registry Taxonomy Check: Rule Drift vs Tree Debt (2026-09-07)

Source-only audit of the current registry checker and a bounded Writer/GIS/Space
sample. The parent reports `check-generated` GREEN and a single full
`plugin-registry:check` run with 4,187 findings; this audit did not rerun that
expensive check.

The authoritative taxonomy is
[taxonomy.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:4).
Its current shape is `artifact -> 🏅️standards -> 🪆️subsets`; a subset owns
`🧬️schema`, `🚪️io`, and `📚️examples`, while an artifact owns only
`🏅️standards` ([canonical directory sets](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:17784)).
The registry checker instead uses the retained legacy `artifactComponentDirs`
set and requires direct artifact schema, io, examples, and an `⚙️engine`
directory ([checker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1206)).
That is checker-contract drift: Writer, GIS, and Space place their schemas,
ports, examples, viewers, and editors beneath standards/subsets as the
canonical taxonomy requires. Artifact-local `⚙️engine` is not a current
taxonomy requirement.

The checker must derive all required-owner checks from the `newArtifact*`,
`standard*`, and `subset*` sets, rather than legacy `artifact*` sets. The
narrow replacement contract is:

- require `🏅️standards` at an artifact, `🪆️subsets` at a standard, and
  `🧬️schema`/`🚪️io`/`📚️examples` at each applicable subset;
- remove the direct artifact `⚙️engine` requirement; and
- retain rejection of children not in the canonical owner set.

The module-path findings contain a second proven checker false-positive class.
The custom scanner in
[registry script](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1444)
only parses the package root's `🦀️.rs`; it does not follow a module's own
`#[path]` descendants. For example, Writer's config root mounts
`🧬️schema/🧬️mutations/🦀️.rs`
([config](/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:106))
and its presence root mounts another mutations leaf
([presence](/Users/ueli/Documents/semio/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:87)).
GIS config does the same for diff/mutations
([GIS config](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:126)).
Use the repository's recursive `inspectRustModuleGraph`
([discovery](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:7056))
over the exact package context. A nested reachable leaf is valid; a missing
target or a component file unreachable from every package graph remains a real
blocking tree error.

`🎚️options` is genuine taxonomy debt, not checker drift: the canonical window
child is `☑️options` ([taxonomy](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:18250)).
The bounded samples contain 8 Writer, 13 GIS, and 6 Space old-name path
occurrences (versus 1, 0, and 2 canonical-name occurrences respectively).
Likewise `⚙️config` is not canonical; `🎚️config` is. These must be corrected
in the plugin trees, not hidden by a checker relaxation. Counts are raw path
occurrences, not a claim about independent logical windows.

Minimal registry test-first matrix for the parent-owned checker repair:

1. A nested artifact/standard/subset with only subset schema/io/examples is
   accepted; a direct artifact has no engine requirement.
2. Missing each subset-owned schema, io, or examples yields an exact finding.
3. A package root -> component root -> nested `#[path]` leaf is accepted;
   a missing nested target and an unreachable leaf both fail.
4. `☑️options` is accepted; `🎚️options` is rejected.

This separates rule drift from actual tree migration without rewriting plugin
trees. The parent separately reports first-open source tests GREEN5, including
four hostile-entry cases; that remains mocked browser/socket evidence only,
not native GIS or end-to-end acceptance.

### Plugin Registry Facet Contract: Schema, I/O, And Contribution Authority (2026-09-07)

The current taxonomy does **not** require six schema formats at every schema
owner. It partitions them by the normative leaf:

| Facet kind | Normative leaf | Required formats |
| --- | --- | --- |
| `🧬️data` | `🔣️.json` | JSON Schema, Rust, TypeScript, GraphQL, Protobuf |
| `📜️interface` | `📜️.wit` | WIT only |

This is expressed in
[taxonomy.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:17630)
and implemented by the reusable `schemaFacetFormatEntries` resolver
([discovery](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:2682)).
It selects from the present normative leaf and defaults an otherwise
unclassified facet to `🧬️data`, so a missing JSON normative leaf remains a
failure rather than a way to evade completeness. Surface
`🎚️config/🧬️schema`, `👥️presence/🧬️schema`, and `🫧️transient/🧬️schema`
are explicitly JSON-normative
([taxonomy](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:17825)).

The registry's `TAXONOMY_SCHEMA_FILENAMES` global set and its artifact/app
loops instead require every schema format, including WIT
([checker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1133),
[app loop](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1574)).
That is a proven false-positive rule. The sampled Writer editor config and
presence, both GIS editor config/presence pairs, and Space/Home editor
config/presence pairs each have exactly the five data leaves and no WIT leaf.
They are valid data facets. Use `schemaFacetFormatEntries` for every existing
facet path; do not use its six-format superset.

This does not waive actual schema debt. Space's owned
`🪐️space/.../✳️any/🧬️schema` has only Rust and TypeScript at its direct
root: with no WIT it defaults to data and lacks the normative JSON, GraphQL,
and Protobuf leaves. That remains a real finding. A checker must therefore
separate an owned-but-incomplete schema from a contribution rather than using
absence as permission.

For I/O, the authoritative grammar permits both forms beneath a subset-owned
`🚪️io`:

- native, bidirectional semantic collections directly below I/O:
  `📸️snapshot`, `🔺️diff`, `💡️inferences`, and `🧬️mutations`, each with
  `📝️text`/`💾️binary` representations; and
- foreign directions only: `📥️import/🧩️deserializers/🗿️artifacts` and
  `📤️export/🧵️serializers/🗿️artifacts`.

The native/direct rule is explicit in the discovery contract
([discovery](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:1025))
and its `artifactFacetPathIsDeclared` tests cover direct I/O mutations and
inferences. Writer is a positive production example: its subset I/O contains
all four native collection roots plus foreign import/export. GIS Map and
Terrain, and Space/Home, are positive foreign-direction examples. The registry
only admits direction roots and labels Writer's direct native roots undeclared
([checker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1366));
this is another rule-drift failure. Reuse the shared facet-path grammar for
allowed descendants and preserve rejection of an undeclared direct I/O child.

There is one separate, unresolved taxonomy/tree mismatch that must not be
hidden by the same repair. Current foreign codec targets are physically
versioned and subset-qualified, e.g. Writer's PDF deserializer lives at
`.../🗿️artifacts/📖️pdf/🔖️1.4/🧱️base/🦀️.rs`, whereas the current shared
I/O grammar reaches a wildcard target below `.../🗿️artifacts/<artifact>/`
and stops there. The registry's direct-leaf expectation is therefore
incompatible with real Writer/GIS/Space paths, but the present taxonomy does
not yet declare those deeper standard/subset segments. Keep this as an
explicit taxonomy-contract decision: do not silently recursively bless all
arbitrary descendants in the checker. A later taxonomy update should name the
version/subset target grammar; then the checker can verify exact target leaf
placement. The immediate registry repair can still preserve wrong-direction,
wrong-codec-folder, and missing reachable-module failures.

Contribution authority must be explicit. The root policy already provides the
correct bounded model in
[policyContributedSurfaceTargetBreaches](/Users/ueli/Documents/semio/📜️script.ts:27528):
index subset suffixes whose *other* plugin has `🧬️schema`, then require the
contributor's builder `.depends_on(owner)` (with the metadata parity rule).
For registry validation, construct the same suffix index from discovered
plugin roots and pass the matching discovered entry's `dependsOn` into
`validateTaxonomyTree`. Missing schema is exempt only when all are true:

1. the local subset is otherwise a contributed-surface shape;
2. exactly one other plugin owns the identical artifact/standard/subset
   suffix with its schema; and
3. the local entry declares that owner in `dependsOn`.

No owner, a same-plugin missing schema, an ambiguous owner, or absent
dependency is a failure. `consumes` is not sufficient authority: it is a
topic relation, not a runtime actor dependency.

Minimal failure-preserving checker cases:

1. JSON-normative config/presence with the five data leaves succeeds without
   WIT; an interface schema with only WIT succeeds; missing JSON on a data
   facet and missing WIT on an interface facet fail.
2. A Writer-like `🚪️io/📸️snapshot/📝️text` and an external
   import/deserializer root succeed; a made-up direct I/O collection fails.
3. An owned subset lacking schema fails even if it has a viewer/editor.
4. A mirrored subset surface without its own schema succeeds only with one
   matching external schema owner and the matching `dependsOn`; remove either
   condition and require a finding. Add two candidate owners for the same
   suffix and require an ambiguity finding.

These cases let the parent repair registry-only rule drift without turning a
missing owning schema or arbitrary I/O nesting into a tolerated tree state.

## Recursive Rust Taxonomy Mount Validation (2026-09-07, Source Audit)

The new `validateRustTaxonomyMounts` has the right authority boundary for the
first slice. It resolves the canonical package manifest at
`📦️packages/🦀️rust/Cargo.toml`, invokes the shared strict Cargo-aware graph,
and only accepts contexts attributed to that exact manifest
([validator](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1195)).
That correctly rejects a neighbouring/unrelated manifest as authority for a
taxonomy leaf. The seven-case fixture also has useful positive coverage for
nested paths, same-line `#[path]`, inline module base inheritance, and comment
decoys, as well as negative nested-target and unmounted-component cases
([fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧫️fixtures/🕸️rust-taxonomy-mounts/🔣️.json)).
This is source-only review; the parent-owned compiler receipt was still
running when inspected.

### P0: The current component inventory is broader than the selected Cargo target

`walkPluginTree` feeds every Rust leaf named `🦀️.rs` (and every example Rust
leaf) anywhere below a plugin into `componentFiles`
([inventory](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1450)).
It does not exclude `🧪️tests`, `🧫️fixtures`, or examples. The validator then
requires every one of those leaves to be reachable from the package's `[lib]`
root. That is a false unmounted finding for integration-test and fixture code,
not plugin-tree debt.

There is an independent form of the same discrepancy: the shared Rust parser
records conditional modules, but both its graph traversal and the new
validator treat them as required. Thus a `#[cfg(test)] #[path = "missing.rs"]
mod probe;` succeeds under the fixture's direct `rustc --crate-type lib`
oracle but receives a `missing module target` finding. Real plugin packages
already use `#[cfg(test)]` test modules, so this is not hypothetical.

The immediate library-target rule should inventory only production taxonomy
component leaves and ignore conditional-only module edges for the same cfg
selection used by the oracle. Do **not** call a `[[test]]` integration root
unmounted under that rule. The follow-on needs a Cargo target matrix that
separately discovers each declared `[[test]]`, example, and bin root and
validates leaves assigned to that target. It is not evidence that existing
plugin trees are incomplete.

Add three fixture rows before treating the rule as complete:

1. `#[cfg(test)]` plus a deliberately absent nested target: `rustc lib`
   succeeds and no missing-target finding occurs.
2. A Rust leaf under `🧫️fixtures`/`🧪️tests` that is unreachable from `[lib]`:
   it is not reported by the library taxonomy rule.
3. A declared `[[test]]` root whose leaf is reachable only from that target:
   the present rule must classify it as out of scope (until target-matrix
   support), never as an unmounted taxonomy component.

### P1: Dep-info membership currently uses a suffix substring test

`RustTaxonomyMountsCheckScript` considers a source mounted when the dep-info
text contains `path + ":"` or `path + " "`
([oracle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:3022)).
This has no path-token boundary at the beginning: candidate `a/🦀️.rs` can
match prerequisite `nested/a/🦀️.rs`. It also does not decode depfile escaped
spaces or establish the separator convention emitted by `rustc` on Windows.
The result is a potentially false mounted verdict in the independent compiler
oracle.

Parse the `.d` prerequisite list into escaped tokens, normalize only the
relative path representation used for the materialized fixture, and compare
with `Set.has` exact path equality. Add an intentionally nested suffix-collision
case and a space-bearing path case (or reject spaces in the fixture schema if
they are outside the intended taxonomy grammar). A Windows run should be a
required oracle matrix entry before treating direct `rustc`/dep-info as
cross-platform qualified; this audit did not execute one.

### P1: Bounded process cancellation needs an owned outcome

The compiler probe has a 30-second `kill`, but it does not retain a timeout
classification; the killed child falls into the generic nonzero compiler
failure path. There is likewise no checker cancellation signal threaded into
the spawned child. This is bounded in wall time, but not diagnostically or
cooperatively owned. Keep the current small process scope, but record an
explicit `timedOut` outcome, await both output streams after termination, and
remove the materialized temporary fixture in a `finally`. If the surrounding
check framework exposes cancellation, bridge it to the same child termination
path. No production plugin-tree change is implicated.

### Inventory and Cargo authority limits

The exact `Cargo.toml` is a sound source of library-root authority only when
the graph parser accepts its `[lib]` root. A source file's physical location is
not a Cargo target declaration. The validator should continue to fail a
missing canonical manifest or an invalid selected manifest, but should not use
the mere presence of another `Cargo.toml` as a source root. The current
unrelated-manifest fixture correctly exercises that distinction. Future
target-matrix support must use declared Cargo target roots, not recursive
filename discovery.

### Recommended coherent Cargo-target matrix

A filename-based test/fixture exception would be weaker than necessary. The
small complete structural rule is to obtain logical roots solely from the
already selected canonical manifest, then assess every source only against
those roots. Extend the shared strict Cargo facts to expose:

```text
lib | bin | test | bench | example, target id, manifest-relative Rust path
```

`[[example]]` belongs in this list as well as the requested bin/test/bench
tables: the registry inventories example Rust leaves today. Keep the actual
`Cargo.toml` as `manifestPath`; do not create an unrelated synthetic manifest
that could accidentally confer ownership. A target context needs an additional
`targetId` in its deduplication identity so independently declared targets are
not collapsed merely because they share a manifest.

For every array-table target, require exactly one explicit relative `.rs`
`path`, normalize it from the manifest directory, and reject absolute,
drive-qualified, backslash, duplicate, or plugin-root-escaping paths. The
library retains its existing explicit path/default-root rule. A declared target
without a representable exact path must make this strict structural manifest
invalid rather than be silently omitted. The graph can expose this with an
opt-in `manifestTargetKinds` option; existing consumers keep their library-only
semantics and the registry selects all five kinds.

Conditional edges must neither create reachability nor generate an
unconditional missing-target finding. This checker has no target/cfg feature
matrix, so conditional compilation remains the compiler gate's responsibility.
The direct compiler reference likewise needs one invocation per logical root
(or must explicitly limit its equivalence assertion to the library); its
current single `--crate-type lib` invocation cannot prove a test/bin/bench
matrix.

Required matrix rows are: an exact declared test root with a nested member;
declared bin, bench, and example roots; a `cfg(test)` absent module with no
structural finding; a target path escaping the plugin root; and a sibling
Cargo manifest which owns no target. These establish manifest authority,
target reachability, and cfg scope without waiving an actual leaf based on its
directory name.

## Retained GIS Approval Native-Law Setup Audit (2026-09-07, Source Only)

The ledger fixture's `outbox` is intentionally a generic ledger envelope:
its proposal is `ledger-only-proposal` and its command is an older opaque
hexadecimal value. It is appropriate for isolated SQLite ledger/idempotency
coverage, but it is not a valid `RetainedGisMapApprovalCommitterV1` request.
The committer's preflight now rederives the fixed-three parent, its inverse,
the exact child order, actor, document key, proposal hash, mutation id, and
canonical command envelope
([preflight](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:746)).

### Immediate stale setups

Four laws still copy `ledger_fixture().outbox` at the time of review:

| Law | Current meaningful cutpoint | Required setup correction |
|---|---|---|
| `gis_map_approval_fails_closed_without_a_composition_transaction_and_never_auto_applies` | Unavailable committer port | Use a valid canonical tuple so `CommitUnavailable` is attributable only to no committer, even though the unavailable port does not itself preflight. |
| `gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | First publisher attempt deliberately returns `Storage` after the committed event/actor frontier | Must use the canonical tuple; otherwise the new preflight returns `Conflict`, masking the asserted publication-retry cutpoint. |
| `gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer` | Valid owner reaches preflight/assembly/journal; only the deliberately substituted child pair reaches `Rejected` | Rebuild its primary tuple. Keep invalid children as the one intentional preflight rejection, but do not make proposal/command stale as a second accidental cause. Capacity, no-runtime, parked, and post-witness subcases all require the valid tuple. |
| `gis_map_terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission` | A valid unpolled owner is retained, then a later valid request observes `Unavailable` after close fencing | Rebuild its tuple; otherwise `Conflict` can mask the terminal admission fence. |

The public route tests do not share this defect: routes derive their approval
command through `HubInferenceRuntimeV1::server_stamped_command`, and the two
fail-closed route cases intentionally stop before a retained committer exists.
The source-only `proposal`/privacy laws also do not enter the retained
committer. That distinction prevents a broad fixture rewrite.

### One shared test-only tuple builder

Retained-law setup should have a helper which accepts the *same* actual
`InferenceMapBaseV1`, accepted job id, identity, scope, and chosen `now_ms` as
the request under test. It must:

1. run `deterministic_map_inference(base, job_id)` and
   `create_region_group_work`;
2. serialize `work.parent` and `work.parent_inverse` with the production
   `os_pack::json` normalizer;
3. derive proposal hash and `approval_mutation_id` from those bytes and the
   actual accepted job;
4. use `approval_actor(identity)` and `document_key(scope)`;
5. encode `CanonicalInferenceCommandPartsV1` with the GIS schema, exact
   parent/inverse bytes, no dependencies, and the explicit HLC timestamp; and
6. return the command/hash and ordered
   `[drawing_child.child_id, value_child.child_id]`.

Prepare the ledger with this helper's proposal/command before constructing the
committer request. Preserve the generic ledger fixture and its standalone
contract: its `outbox` should not be repurposed to mimic a real Map command.
The neutral proposal fixture can still assert the independent expected parent
and inverse for its sample job; the real retained setup must not hand-copy its
serialized command.

### Concrete trait-boundary P0: ingress scope is validated too late

`validate_prepared_request` verifies outbox/job/hash and the ingress
user/session/generation, but omits
`request.ingress.scope() == request.scope`
([validation](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:519)).
`preflight` catches the mismatch later, after `commit` has reserved a cleanup
slot and created `GisMapApprovalRequestOwnerV1`.

On that preflight error, `owner.admit` has not run; its `Drop` therefore treats
the request as a prepared cursor and schedules
`abandon_prepared_approval` using the otherwise valid legitimate tuple
([owner drop](/Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:390)).
The public `commit_prepared_approval` helper performs the scope comparison
first, but the public committer trait is also directly callable. A matching
user/session/generation with a foreign ingress scope can therefore cause
unwanted abandonment of a legitimate prepared outbox through this lower API.

Add scope equality to the synchronous prepared validation, before cleanup
reservation/owner construction. The exact native law should submit a
foreign-scope ingress with a valid canonical tuple, require `Conflict`, an
empty `cleanup_jobs`, ingress release, and an unchanged prepared row. It must
not accept `Rejected` after a cleanup task, because that tests the unsafe
later cutpoint rather than the required pre-admission refusal.

The scope comparison alone is not sufficient. Actor syntax, children,
base/frontier, deadline, proposal rederivation, and canonical-command matching
are also pure `preflight` checks currently performed only after reservation
and owner construction. Any one can trigger the same prepared-owner Drop
abandonment. The bounded repair is to run full `preflight(&request)` in
`commit`, after ledger tuple lookup but before `reserve_cleanup_job`, then pass
the computed `(identity, snapshot)` into the retained future. Invalid direct
requests then have no owner to abandon a valid existing outbox. The
abandoned-owner law should use a valid tuple for every unpolled cancellation
case; invalid children become a distinct immediate-refusal law requiring no
cleanup reservation and no outbox mutation.

### Concrete trait-boundary P0: accepted base identity is fetched but not bound

The same `validate_prepared_request` obtains the accepted
`InferenceIdentityV1` from the ledger yet compares only its principal and
scope strings. It does not bind `descriptor_digest`, `head_ordinal`,
`head_edit_id`, `last_commit_seq`, `chain_hash`, or `input_hash` to
`request.base`/`request.base_frontier`. `preflight` only checks that the two
request frontier references agree with one another; it does not compare them
to the accepted identity. The document mount checks the actor's publication
frontier *against the caller-supplied frontier* and initializes its owned
three Stores from the caller-supplied pack. Thus this lower public trait has
no independent accepted-base fence.

The HTTP route currently runs `compare_frozen` before it prepares/calls the
committer, but direct trait users need the same lower bound. In synchronous
admission compare all accepted base evidence to the request: descriptor,
document id, ordinal, edit id, commit sequence, chain hash, and SHA-256 of the
base pack. This comparison uses already-recovered ledger identity and does not
need a catalog dependency. Add separate substituted-pack, descriptor, ordinal,
edit-id, commit-sequence, and chain-hash cases; every case requires conflict,
zero cleanup reservation, ingress handback, and unchanged outbox. The success
case should assert the rederived parent, both ordered children, and inverse
are exactly the accepted identity's base-derived operation.

## Warm GIS Native Retry Cargo Fingerprint Audit (2026-09-07)

### Evidence: this is Cargo invalidation, not a per-law target/profile reset

The two requested retained-GIS native captures use byte-identical build
receipts:

| Capture | Build receipt time | Cargo command | Target directory | Result |
|---|---:|---|---|---|
| `sQEoyS/00` | 19:32:17 | `cargo test --manifest-path Cargo.toml -p semio-hub --lib --no-default-features --features sqlite,native-artifact-execution --no-run --message-format=json` | shared `…/🗑️generated/reactor-lifecycle-native-target` | finished in 18m57s |
| `f14A8Q/00` | 19:53:28 | exactly the same | exactly the same | finished in 12m11s |

The gate deliberately obtains `CARGO_TARGET_DIR` from the configured ticket
environment and preserves it for both compilation and law execution
([runner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2058)).
It does not clean the target, change manifest/cwd, add `--target-dir`, or set
`CARGO_INCREMENTAL=0`. The GIS gate supplies only `RUST_MIN_STACK` to its
build stage ([stage environment](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:82)); its stored stdio fingerprint has the same test
profile, `full-artifact-catalog` feature set, `-Z threads=8` rustflags, and
compile kind. There is no capture evidence for diagnostic-path, cwd, profile,
or rustflags churn.

Cargo's JSON messages instead mark `semio-s-plugin-stdio` `fresh: false` in
both builds. The first build recompiles only five local crates:

```
semio-framework-plugin -> semio-s-plugin-stdio -> semio-s-plugin-gis
                                                   -> semio-s-plugin-vcs -> semio-hub
```

The second recompiles fifteen crates, beginning with changed framework inputs
(`os-kernel`, UI, core, graph, schema, plugin, compiler, and infinite) before
stdio and its GIS/Hub dependents. The Hub executable fingerprint also changes
between the captures (`58e702…` to `d409df…`). That is direct evidence of
changed Rust inputs across the retry interval, not a warm invocation merely
re-running a cached executable.

Cargo's ordinary JSON record does not retain its *dirty path/reason*, so these
captures cannot honestly identify one edited file as the trigger. A future
single diagnostic retry can set Cargo's fingerprint trace logging and retain
only the resulting build stderr in its ticket capture; do not infer a specific
file from current shared-worktree mtimes.

### Why this invalidation is expensive

The dependency is explicit and feature-forced, not pulled by the native law
selector. Hub's `native-artifact-execution` enables stdio's
`full-artifact-catalog` and GIS ([Hub manifest](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:30)). GIS independently requests the same
stdio feature ([GIS manifest](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:73)).

`full-artifact-catalog` mounts the entire stdio plugin root rather than a
GIS-only leaf ([stdio root](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:127)). Its current Cargo dep-info lists **4,565**
stdio-repository files (4,584 in-repository inputs total; the dep-info is
7,143,582 bytes), overwhelmingly taxonomy artifact schemas/mutations. The
same crate also directly depends on the framework crates that appeared stale
in `f14A8Q` ([stdio manifest](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:29)). Consequently either a mounted stdio
artifact input or a recompiled upstream ABI forces a very large codegen/const
evaluation unit to rebuild. That behavior is legitimate under the current
closed component/catalog contract.

### Bounded recommendation

Do **not** replace the shared `reactor-lifecycle-native-target` with a new
target per retry: it is already correctly shared, and doing so would make the
observed cost worse. First add only an opt-in, non-secret build-identity/
fingerprint-reason capture to `runExactCargoLaws` (selected Cargo/Rust env
keys, manifest args, target path, and Cargo's dirty trace). A next failed warm
retry will then attribute the invalidation without altering source or caches.

There is no safe runner-only performance repair. The only structural route is
a separately compiled, semantically verified GIS build dependency whose public
surface contains exactly the stdio types/codecs GIS needs, while the full
catalog remains the sole component/catalog producer. That is a new component
closure/policy decision and requires byte/descriptor equivalence evidence; it
must not be introduced merely to hide a real full-catalog input change.

## Plugin Registry Full-Check Measurement (2026-09-07)

### One isolated registered check

The one authorized full check was dispatched through the root wrapper:

```text
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=true \
NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/registry-check-audit/nx-workspace-data \
NX_CACHE_DIRECTORY=<ticket>/🗑️generated/registry-check-audit/nx-cache \
bun ./📜️script.ts nx run @semio-tech/plugin-registry:check --skip-nx-cache --output-style=stream
```

Its complete captured output is
[`registry-check.log`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/registry-check-audit/registry-check.log).
The target failed in taxonomy validation with **2,696** findings. It did not
short-circuit on stale generated catalog bytes. The active taxonomy area is
`clean`, so the validator reports these as hard failures.

### Representative remaining findings

| Plugin | Total | Taxonomy-rule drift | Target-matrix false positives | Exact tree debt |
|---|---:|---:|---:|---:|
| `✒️writer` | 14 | 9 | 3 | 2 |
| `🌍️gis` | 33 | 18 | 7 | 8 |
| `🪐️space` | 27 | 18 | 4 | 5 |

The three totals partition all 74 sample findings. The result is not evidence
that Writer, GIS, or Space needs every reported directory created.

#### Proven validator drift

1. **Artifact-root completeness is checked at the wrong owner.** The validator
   requires `🧬️schema`, `🚪️io`, `⚙️engine`, and `📚️examples` directly below
   every `🗿️artifacts/<artifact>` root
   ([validator](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1229)).
   The authoritative taxonomy says the opposite: an artifact owns standards;
   a subset owns schema, I/O, and examples; behavior is outside the artifact
   ([taxonomy](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:4)).
   Thus the 20 sample `artifact ... is missing ...` rows are checker drift.
   The stale explicit engine and artifact-example checks at
   [lines 1350–1357](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1350)
   must move to the correct owner or disappear; creating those directories at
   artifact root would contradict the taxonomy.

2. **`🔨️modules` is optional but required by the implementation.** The
   taxonomy explicitly says the W1 module child is optional and must not cause
   a completeness failure ([taxonomy rationale](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:11)).
   `validateTaxonomyTree` nevertheless loops over all `pluginChildDirs`, not
   `pluginRequiredChildDirs`, and requires a Rust leaf
   ([lines 1513–1520](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1513)).
   The three Writer/GIS/Space module rows are therefore drift.

3. **The Cargo reachability pass only models the library root while classifying
   every `🦀️.rs` test/example leaf as a component.**
   `validateRustTaxonomyMounts` creates one graph rooted at the library
   manifest ([lines 1196–1208](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1196)), while the recursive walk indiscriminately
   adds `🦀️.rs` leaves to `componentFiles`
   ([lines 1466–1487](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1466)).
   This produces 14 Writer/GIS/Space unreachable rows. GIS makes the matrix
   defect directly observable: `native_codecs` and `component_cold_map_patch`
   are real `[[test]]` targets, not lib modules
   ([GIS manifest](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:62)).
   The Writer/Space test/example leaves are similarly not proof of a missing
   lib mount. The pending manifest-target matrix must classify each leaf by an
   exact declared target or a test/example role before reporting it as an
   unmounted production component.

4. **Surface config/presence assumes a complete top-level schema-format set
   whenever the lane exists.** The sample has 20 rows, including intentional
   `📌️.empty.md` config/presence lanes and editor lanes missing only WIT. The
   validator's unconditional `TAXONOMY_SCHEMA_FILENAMES` loop
   ([lines 1525–1558](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1525))
   does not consult a facet's declared collection/format ownership. An empty
   lane must remain valid; a nonempty lane should be checked against its
   declared data/interface formats, not synthesized into every format.

#### Exact tree debt, not validator relaxation

- `🎚️options` is a real obsolete window child: the taxonomy requires
  `☑️options` ([vocabulary](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:18250)).
  Writer has one, GIS four, and Space two such rows.
- `⚙️config` is explicitly a legacy surface/window name; the validator's
  required rename to `🎚️config` is consistent with its taxonomy policy
  ([legacy check](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1561)).
  GIS has four and Space two rows.
- Each sampled plugin declares the required root `🎮️commands` directory but
  retains only `📌️.empty.md`, not the required root leaf. These three rows are
  genuine required-contract debt. Space additionally lacks the required
  `🎮️commands` child in its `🪐️space` editor's `✏️edit` mode; that is the
  remaining one mode-row debt.

### Minimal next checker slice

Keep the just-qualified recursive library mount check. Next change only the
owner predicates: artifact checks descend through
`artifact → standard → subset`; `pluginRequiredChildDirs` controls root
completeness; empty lanes do not infer absent schema; and Cargo target facts
separate lib/bin/test/bench/example roots from non-production fixtures. Keep
the `🎚️options`/`⚙️config` and command-root/mode findings hard. This removes
false noise without suppressing a missing owned schema or a real vocabulary
violation.

## Capability-Open Cancellation Close Ownership Audit (2026-09-07)

The current native failure at the one-grant assertion has two separate causes.
The observed count drop is not, by itself, evidence that one public
`close_step` released two owners: the test admits real automatic cleanup, then
measures only around a competing manual step.  `cancel()` reaches
`stage_terminal`, which schedules cleanup; the test then drops its future and
loops on the terminal handle without controlling that submitted work
([current law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:11423),
[scheduling path](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:700)).
Several background driver grants may therefore retire roots between its two
snapshots.

The existing test-only `ControlledCapabilitySubmitQueue` and
`controlled_submit_hook` are sufficient to make that law deterministic
([queue](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10548)).
Install the hook before cancelling/inducing stale state, execute at most one
captured job per iteration, and compare the owner count immediately before and
after *that job*.  Separately compare immediately before/after each terminal
`close_step`; reset the baseline after every exact grant; cap the whole trace
at 64.  Preserve the storage pointer until drain and assert final empty roots,
no admission, and an absent exact registry slot.  This validates the claimed
per-grant bound without changing production terminal semantics.

There is nevertheless a separate source-visible P0 that the controlled test
must not mask.  `drive_one` clears `scheduled` before it starts non-poll phase
work ([line 549](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:549)).
During `Handoff` and `RetainWork`, it temporarily owns `work` in a stack local
outside every state root ([lines 559–575](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:559)).
Meanwhile public `close_step` treats only `scheduled` or `polling` as an
active driver ([lines 761–790](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:761)); after it drains visible roots it can release
the admission and registry slot ([lines 829–835](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:829)).
`terminal_checked_out` is only a terminal-handle exclusivity bit
([lines 1188–1196](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1188)); it does not fence the running driver.

The adverse trace is: a worker clears `scheduled`, passes its cancellation
check, and takes `work`; another thread drops the public future (which marks
abandoned/cancelled and may terminalize because it observes neither scheduled
nor polling, [lines 1018–1033](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1018)); the terminal caller can then drain the
state roots and admission while the worker still carries the exact work owner.
The worker can subsequently restore `poll_work` after the slot has been made
reusable.  Submission failure/terminal-job paths make this reachable even when
the cancellation's successor cannot stay queued.  The same shape applies to
the `RetainWork` transfer.

The narrow repair is a state-level `driving`/RAII drive lease acquired before
clearing `scheduled` and released only after all phase mutation and successor
submission are complete.  It is not a new owner or queue.  `close_step` must
set cancellation but return `Blocked` while `driving`; future `cancel`/`Drop`
must mark cancellation only while `driving` and leave the active driver to its
next phase boundary; terminal `resume` must likewise reject a live drive.
Do not add this gate to normal successful result retirement unless the driver
also performs a final release hand-off: a future can consume the published
success as soon as `complete` wakes it.  The critical invariant is that no
terminal admission release observes an untracked stack-local work owner.

Add one targeted controlled interleaving law in addition to the corrected
per-grant law: pause a drive after it claims `work` but before the destination
root is installed, drop/cancel the future, and call public terminal
`close_step`.  It must be `Blocked`, retain the same storage pointer and
generation/registry entry, then converge under one controlled job at a time
to exactly one final admission release.  Repeat at the `RetainWork` transfer.
That proves the actual driver/terminal exclusion rather than merely quiescing
the prior flaky accounting assertion.

### Exact Active/Queued Lease Revision (2026-09-07)

The later controlled native failure is the expected concrete form of the
stack-local-owner defect: a paused `Handoff` drive reports `Progress`, while
the admission is already gone although the exact storage owner has not been
lost.  A boolean `driving` check is insufficient: a successor requested from
within the active turn can be synchronously invoked by a controlled submit
hook, and a load before `close_step`/`resume` is a TOCTOU check rather than
authority.

Use one state atomic as a lease, with `ACTIVE = 1`, `QUEUED = 2`, and
`CLOSED = 4`:

| State | Meaning | Permitted owner movement |
|---|---|---|
| `0` | idle | a public close/resume/finalizer may CAS to `ACTIVE`; a drive request may CAS to `QUEUED` |
| `QUEUED` | one exact submitted/captured job owns the next turn | only that job CASes to `ACTIVE` |
| `ACTIVE` | one driver or public terminal operation owns all root transfers | no competing root transfer |
| `ACTIVE|QUEUED` | a successor was requested during that operation | RAII exit converts it to `QUEUED` and dispatches exactly once |
| `CLOSED` | final admission/registry retirement is linearized | no later request can re-open or enqueue |

`request_drive` needs a CAS loop, not a bare `fetch_or(QUEUED)`: it first
observes `CLOSED`, then either coalesces an already queued request, changes
`ACTIVE` to `ACTIVE|QUEUED`, or changes idle to queued and submits.  The
worker must CAS `QUEUED -> ACTIVE` before looking at a generation or moving a
root.  Its RAII guard performs the only `ACTIVE -> idle/queued` transition
after every phase has restored/dropped its local owner.  This includes the
currently separate retry callback: it must claim idle-to-active **before**
`retry_job.take()`, and leave the retry root installed if busy.  Otherwise the
callback recreates the same unprotected local-`Job` gap.

The guard's `submit_exact` transfer is critical.  It first changes
`ACTIVE|QUEUED` to `QUEUED`, disarms itself, then calls the pool without a
mutex held.  A synchronously reentrant test submitter can therefore run the
new job and claim `QUEUED -> ACTIVE`; it cannot disappear.  If pool submission
returns the exact job, `handle_submit_refusal` must claim `QUEUED -> ACTIVE`,
install that exact job in `retry_job` or `terminal_job`, and only then release
its active lease.  This makes the new one-shot `Shutdown` refusal seam an
accurate production-path test rather than synthetic state mutation.

`cancel` and future `Drop` should set their flags and call `request_drive`; if
the lease is active they must not call `stage_terminal` or move roots.  Public
`close_step`, `DatabaseCapabilityOpenTerminalHandle::resume`, and terminal
result `take`/`resume`/`close_step` must each first CAS idle-to-active and return
their owner unchanged/`Blocked` on every other live state.  A close that finds
queued or active first records cancellation; the authoritative worker then
performs terminalization.  This covers the `work`, `poll_work`,
`staged_result`, `terminal_work`, `terminal_result`, `completion`, retry job,
and terminal job take sites.  `DrainWork` does not export a root from its
mutex, but it still requires the same active exclusion.

Completion needs a final, distinct handshake.  A consumer can take a
published completion while the producer's driver remains active (the
publication-before-wake test intentionally demonstrates that interleaving).
`take_completion` should clear its transient waker, set
`completion_consumed`, and:

1. if it can claim idle-to-active, run the guarded finalizer itself;
2. otherwise leave the flag for the active guard; and
3. never call `release_success` directly while another active lease exists.

At guard exit, if completion was consumed and all roots are empty, CAS either
`ACTIVE` or `ACTIVE|QUEUED` to `CLOSED` before dropping admission or erasing
the exact registry slot.  The `ACTIVE|QUEUED` case deliberately discards that
pending ordinary drive: terminal success is definitive.  If roots are not
empty (for example an error with a retained terminal job), do not close; pass
the queued successor or terminal cursor forward.  The same closed transition
is required for a final successful public `close_step`.  `finished` alone is
not a linearization point: a concurrent `request_drive` can otherwise set
queued in the gap between its check and `release_success`, leaving an orphan
job after admission release.

`completion_consumed` must mean only a result actually returned by
`DatabaseCapabilityOpenFuture::take_completion`, not a completion moved by
future `Drop` into `terminal_completion`.  Drop has deliberately abandoned
the caller and needs the public terminal cursor to retire it; it must mark
cancellation and request a governed turn, not be mistaken for a successful
finalizer during the brief interval between its two mutex writes.  Both
terminal resume routes must clear this consumption flag before exposing a new
future.

#### Required controlled laws

1. Pause after `Handoff::work.take`, request cancellation by future drop, and
   inject the real one-shot exact-job `Shutdown` refusal.  While paused,
   close is blocked, terminal resume returns the exact terminal cursor, and
   storage pointer/admission/slot remain unchanged.  On release, bounded
   controlled cleanup reaches empty once.
2. Repeat at `RetainWork`, and pause immediately after `poll_work.take` but
   before `poll`; the existing poll-body barrier alone is too late.
3. Make a submit hook invoke the newly submitted job synchronously.  Assert
   an `ACTIVE|QUEUED` successor executes once, and a refusal stores the exact
   returned `Box<Job>` rather than losing it.
4. Hold a publisher in its before-wake hook, consume completion on another
   thread, and prove that admission/registry survive until the active guard's
   closed finalizer; then assert an attempted post-completion `request_drive`
   cannot enqueue a job.
5. For the stale-worker test, do not call `drive_one` directly.  While the
   admission is still current, install `ControlledCapabilitySubmitQueue`, call
   normal `schedule`, pop the actual captured job, then change the admission
   generation and invoke that job.  This exercises the real queued-to-active
   gate.  Restore the fixture generation only after it has terminalized, then
   drain captured cleanup jobs one at a time.

These transitions are confined to the existing capability state and its
existing exact job/cursor roots; they do not introduce another queue or relax
the one-grant assertion.

### Live First Lease Repair Review (2026-09-07, source only)

The landed capability state implements the required ownership boundary:
`request_drive` coalesces via the active/queued lease, `drive_one` claims
queued-to-active before a phase checkout, retry claims idle-to-active before
`retry_job.take`, and every direct terminal result movement now claims the
same lease.  The public close path claims only idle-to-active and otherwise
returns `Blocked`; the paused `Handoff` law now uses an actual captured drive,
future drop, and the common submission-refusal handler.  This removes the
previous source-visible early-admission-release trace.

The active guard also handles the important completion race correctly in its
normal path: a ready consumer sets `completion_consumed`; an idle consumer can
perform a guarded final retirement itself, while an active producer's guard
does the closed transition after all roots are empty.  `CLOSED` prevents a
late ordinary schedule request from creating an orphan after the registry
slot is erased.  The guard transfers active-to-queued before its submit call,
so the controlled submitter may invoke the successor synchronously without
losing it; the refusal handler reclaims queued-to-active before it parks the
returned exact job.

The terminal-resume reset is now corrected: it clears
`completion_consumed` once for both its terminal-job and common successful
branches; terminal-result resume already did the same.  The remaining
coverage follow-up is that the older post-ready cancellation/stale test still writes `lease = QUEUED`
   and calls `poll_backend_once` directly
   ([engine](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:11769)).
   It is not a valid lease test: no submitted job owns that queued bit, and a
   cancellation during the direct poll observes queued rather than
   active-queued.  Change it to capture a normal `schedule` job and invoke
   it, just as the stale law now does; the cancel/stale hook then runs inside
   the actual active driver.  Also add the asserted `terminal.resume()`
   refusal while the transfer hook is paused.  These are coverage gaps, not a
   new production ownership escape.

Add one reentrancy law: capture the initial job, invoke it manually, invoke
its Handoff successor synchronously from the second submission-hook call, and
capture the third.  The inline successor must claim queued-to-active, perform
one Poll, and leave exactly one later phase successor.  This proves an
`ACTIVE|QUEUED` handoff cannot lose work when submission is synchronously
reentrant.

The existing `consumed-before-wake` retirement rows exercise `complete` from a
separate thread while the lease is idle, so their expected immediate registry
retirement is correct but they do not prove the new active-guard rule.  Add a
separate actual-driver law: stage a real result at Publish, run the captured
Publish job, block its before-wake hook, consume the completion on another
thread, and assert the admission and exact registry row still exist.  Release
the hook, join the active driver, then require `CLOSED`, one release, and no
queued successor.  That is the direct proof of delayed final retirement.

### Capability Paused-Owner and Successor Rows — Current Source Review (2026-09-07, unrun)

The current three-row `driveOwnership` law now has the right physical pause
locations.  Its hooks fire immediately after the authoritative local checkout
at `Handoff`, `RetainWork`, and (importantly) after `poll_work.take()` but
before the first backend poll.  The test runs the normally captured initial
job on a separate thread, drops the future while that exact root is
stack-local, checks that public resume returns the unchanged terminal cursor,
then requires `close_step` to remain `Blocked`, the admission/registry entry
to remain exact, and the storage weak pointer to retain the original address.
The one-shot `Shutdown` refusal is routed through the ordinary refusal handler,
not a synthetic lease write.

The new `leaseCompletion` active-publisher row also correctly uses a normal
captured sequence through `Publish`.  It pauses only after completion is
installed and before its wake, consumes the public result while the producer
still owns `ACTIVE`, requests two successors, and releases the producer.  The
assertions require the admission to survive consumption until the guard exits,
then require `CLOSED`, an empty registry slot, and no late submission.  The
synchronous row invokes every reentrant job outside the submitter mutex and
therefore exercises `ACTIVE|QUEUED` rather than a fake direct phase call.

No target capability row still invokes a controlled job while holding its
queue mutex.  The formerly hazardous chained form has been split into a local
`job` followed by `job()` in the new rows.  The fixture values also require all
three paused rows to block public close and retain the same storage identity.
This is source-only review; native session `54312` remains the qualification
boundary.

There are still five older controlled test calls in this same engine which
retain a queue mutex through arbitrary job execution:

- `database_catalog_bootstrap_handoff_interruption...` at engine lines 10996
  and 11007;
- the bootstrap result-drop path at line 11352; and
- rejected catalog-read close at lines 12100 and 12111.

They are not part of the new capability proof, but each can deadlock if its
governed job synchronously resubmits through the same controlled queue.  The
minimal test-only correction is the exact new-row pattern: perform `pop()` in a
short block, drop the mutex guard, then invoke the local `Job`.

### Separate Current P0 — Catalog-Read Has No Drive Lease

`DatabaseCatalogReadState` still has only independent `scheduled` and
`polling` booleans.  `drive_one` clears `scheduled` before it transfers an
owner at Handoff, RetainWork, or Poll (engine lines 1839, 1850, 1860, and
1908).  In that interval a future drop marks cancellation and schedules a
cleanup job, and a second worker can clear that second `scheduled` bit and
advance terminal cleanup while the first worker still owns `work` on its
stack.  Public `close_step` then observes neither `scheduled` nor `polling`,
consumes terminal roots, and can erase the admission and exact registry slot at
lines 2028-2079 before the original worker restores its local work.  This is
the same two-driver/stack-owner class that the capability lease just repaired;
it is not a failure of the new capability implementation.

The bounded repair is to give CatalogRead the same single active/queued/closed
transfer authority (or an equivalent RAII lease) before it clears
`scheduled`; terminal close, drop/cancel, retry callback, and both terminal
resume routes must not move a root unless they own that authority.  Add
controlled pauses immediately after Handoff, RetainWork, and Poll checkout;
while paused, drop plus public close must be `Blocked`, preserve the exact
storage/admission/registry identity, and converge only after the original
driver returns.  This remains a separately scoped P0; do not widen the current
capability patch to rewrite other engine families without a dedicated slice.

### Catalog-Bootstrap Comparison — No Matching Stack-Owner Finding

CatalogBootstrap does not presently share that specific public-close trace.
Its `drive_one` CASes `Queued -> Driving` before clearing `scheduled`, retains
`Driving` through `drive_claimed`, and only releases it after every phase root
movement.  `DatabaseCatalogBootstrapTerminalHandle::close_step` returns
`Blocked` while `Driving` or `Retry`; concurrent `schedule` merely records
`wake_requested`.  Its retry callback likewise holds `Retry` while its exact
job is checked out and restores it on a failed transition.  Therefore a public
close cannot consume a root held by a bootstrap driver stack.

`DatabaseCatalogBootstrapResult::into_parts` can call `release_success` while
the publishing driver has not yet returned, but by then `publish_one` has
already moved storage/key/actual into the result and left no physical root on
the driver stack.  `finished` makes a late wake inert after the driver releases
`Driving`.  I found no concrete owner resurrection from that ordering.  A
small focused law can freeze this fact: pause just after Publish has installed
the public result, consume `into_parts` concurrently, release the driver, and
assert no successor is submitted after registry removal.  It is coverage, not
a current Bootstrap P0.

### Catalog-Read Lease Repair Plan (2026-09-07, source plan only)

`DatabaseCatalogReadState` has the same *kind* of physical roots as
CapabilityOpen — work/poll-work, a staged result, terminal work/result/
completion, retry and terminal jobs, a waker, and an admission/registry slot —
but it is not interchangeable state. The read result additionally owns the
exact `DatabaseCatalogRootKey` and a protocol-facing
`Result<Option<(DbIoPages, EpochFence)>, DbError>`. Returned pages are a
legitimate caller-owned result, not a cleanup root that a generic finalizer may
discard. Its admission has scalar catalog-read credit rather than capability
item/byte credit, and its current retry callback has no retry-generation
fence. Those differences make generic CapabilityOpen state/RAII reuse unsafe.

Use a private, separate `DatabaseCatalogReadLease` with the same three bits
(`ACTIVE`, `QUEUED`, `CLOSED`) and the same mechanical rules, but with
CatalogRead-specific root movement and errors. A generic trait-based lease
would need type-specific `roots_are_empty`, abandoned-completion handoff,
exact registry retirement, submit/refusal, and retry invalidation from Drop;
that hides the protocol-page boundary behind callbacks without removing a
mutex or allocation. A monomorphized abstraction does not improve the single
atomic fast path, while a trait-object makes the critical Drop path less
auditable. At most, share bit constants or a tiny CAS helper; retain distinct,
explicit lease types and state-specific Drop code.

Required CatalogRead conversion:

1. Add `lease: AtomicU8`, `completion_consumed: AtomicBool`, and a bounded
   retry-generation/armed pair. `request_drive` CASes idle to queued or active
   to active-queued, refuses closed, and submits only for idle-to-queued.
   `drive_one` claims queued-to-active before clearing the old scheduled
   diagnostic. `scheduled` must be removed as an ownership authority (or be a
   derived diagnostic only), never again a public-close admission predicate.
2. Its guard owns every stack checkout: Handoff work, RetainWork
   work-or-poll-work, Poll poll-work, Drain/Release work, staged-result
   transfer, and Publish. On normal exit it dispatches exactly one deferred
   successor; on a terminal consumed completion it atomically closes and
   discards a pending ordinary successor before dropping admission/registry.
   On refusal, it claims queued-to-active before parking the exact returned job
   in retry or terminal state.
3. `cancel` and Future Drop only mark abandoned/cancelled and request a
   governed turn while a drive is live. They do not move completion, work, or
   a job from an active caller. The guard moves an abandoned public completion
   to `terminal_completion` after its local work is restored. Public close,
   terminal-handle resume, terminal-result resume, and terminal-result close
   all require idle-to-active before a root checkout.
4. Add a `take_completion` helper analogous to CapabilityOpen: clear the
   transient waker, set `completion_consumed`, and either acquire idle
   finalization or leave finalization to the active guard. This specifically
   handles consumption after `publish_public_completion` stored completion but
   before that publisher has taken its waker. Reset consumption before either
   terminal resume re-publishes a completion. Include
   `terminal_result_checked_out` in the root predicate used by a finalizer.
5. Retry claims idle-to-active before `retry_job.take`; close invalidates an
   armed callback by retry generation while retaining/dropping its exact job.
   A callback that cannot claim leaves the job installed. Existing catalog
   admission-generation checks at drive entry/post-poll and the 64 KiB per-read
   root limit (4 MiB across 64 admitted reads) remain unchanged.

No public signature needs to change. The internal initialization site is
`DatabaseCatalogReadFuture::try_prepare_with_use`; callers remain
`Database::open_catalog_read_retained` and engine open/read paths around 8222
and 8319. The storage/key/root tuple must remain exact: no generic finalizer
may close `DbIoPages` transferred by `DatabaseCatalogReadResult::into_parts` or
the terminal-result cursor.

#### Test-first seam and laws

Add a CatalogRead-specific schema/fixture sibling to
`engine/🧪️fixtures/📬️capability-completion`, rather than mixing a protocol
result into the capability corpus. It needs `Handoff`, `RetainWork`, and
`Poll` paused-owner rows plus active-publication. Add one-shot transfer hooks
immediately after the checkouts at engine lines 1850, 1860, and 1908, and a
completion-before-wake hook in `publish_public_completion`, distinct from the
existing public check/register/recheck hook.

For every paused row, `try_prepare(..., false)` captures the ordinary scheduled
job; the actual worker pauses after checkout; future drop plus repeated public
close must be blocked; terminal resume returns the same cursor; exact
storage/key/admission/registry survive; then release converges in a bounded
controlled drain with one final admission release. For Poll, setup moves work
to `poll_work` and pauses before backend polling, not inside the fake backend.

For active publication, `controlled_catalog_read_probe(Ready)` supplies the
real protocol-shaped result. Drive normal captured jobs through Publish, pause
after completion installation, consume the public future, request two
successors, then release. Require admission/registry while active, one closed
retirement after release, zero late submit, and exact storage/key/epoch/pages
returned. Repeat with an error result. Existing probe starts at Poll, so it is
not evidence for the three checkout rows.

All controlled jobs must be popped into a local before invocation. The older
mutex-held calls at 10996, 11007, 11352, 12100, and 12111 require that same
test-only correction before any synchronous-submit hook reaches them.

### Descriptor Emitter Budget Semantics (2026-09-07, source-only)

The shared process contract is unambiguous: `BUILD_BUDGET_MS` defaults to zero
and `runCmdInternal` passes zero as `spawnSync({ timeout: 0 })`; its documented
meaning is no timeout. The descriptor emitter must preserve that value rather
than turn it into one millisecond.

The current extraction now implements the correct three-way behavior:

| budget | guard | child budget |
| --- | --- | --- |
| `0` | never expires by elapsed time | `0` (unlimited) |
| positive integer | expires only when elapsed is strictly greater | `max(1, budget - elapsed)` |
| negative integer | fails before the first checkpoint/child | irrelevant; helper's `1` is unreachable |

`remainingDescriptorEmissionBudgetMs` returns zero for unlimited, and
`emissionGuard` skips the elapsed-time comparison only for zero. Both child
boundaries use that helper: `ensureBuiltBin` for Cargo and `runCmdStatus` for
the descriptor binary. Thus the default `buildBudgetMs() === 0` is propagated
through both child calls, while a positive residual never accidentally becomes
unlimited. The new fixture's exact-limit row (`elapsed === budget`) correctly
permits launch with a one-millisecond minimum; its negative row fails in the
guard before either child can begin.

All immediate production callers of `describePluginComponent` and
`describeExtensionComponent` use their default control; they therefore retain
the same zero/unlimited build contract. No caller currently passes a positive
control deadline which would require a call-site adjustment. The independent
FreshComponent path must not be changed by this repair: its `remainingMs()`
contract deliberately requires a finite `1..86400000` process budget and is
not DescriptorEmissionControl.

The neutral AJV/Decimal/fake-clock test is a valid no-Cargo regression for
guard arithmetic and no-half-publication behavior. It intentionally throws at
the `emit` checkpoint, so it does not itself launch a child or measure native
emitter behavior; its child-propagation claim is source-bound to the two helper
call sites above, not runtime-qualified. That is appropriate for this isolated
regression, but should be documented as such.

One narrow hardening remains if `DescriptorEmissionControlV1` is treated as a
general programmatic boundary: validate the supplied budget once before the
first guard. Preserve integer `0` as unlimited and negative integers as
already-expired, but reject `NaN`, infinities, unsafe integers, and fractions.
Without that check, `NaN` makes both `elapsed > budget` and `Math.max(1,
budget - elapsed)` non-authoritative and can reach the child runner as an
ambiguous timeout. This is separate from the fixed zero regression. Add
neutral rows for a non-finite/non-integral invalid control only if that control
is public to untrusted callers; do not relabel the intentional `-1` expiry as
an invalid-budget row.

### Capability Completion Finalizer Check-to-Release Race (2026-09-07, source-only P0)

The newly added `controlled_lease_release_hook` is at the exact dangerous
window, but current production code still strands an admitted capability under
that interleaving. `DatabaseCapabilityOpenLease::Drop` tests
`completion_consumed && roots_are_empty()` before the hook, then clears only
`ACTIVE` at `engine/🦀️.rs:396-420`. `DatabaseCapabilityOpenFuture::take_completion`
sets `completion_consumed` and calls `release_success` at `:992-1001`, but
`release_success` currently returns when it cannot claim `lease == 0`
(`:961-966`).

Exact success (and equivalently delivered-error) trace:

1. The real `Publish` worker owns `ACTIVE`, installs/wakes completion, sets
   `Terminal`, and reaches Lease Drop. At the first finalizer predicate the
   completion is still unconsumed, so it is false.
2. The test hook pauses immediately after that predicate. The public future
   polls, takes the exact completion and transient waker, sets
   `completion_consumed = true`, and finds `ACTIVE`, so its current
   `release_success` returns without recording a successor.
3. The worker resumes, does `ACTIVE -> 0`; no queued bit exists, so it submits
   nothing. There is now no public Future, retry, terminal cursor, or driver
   job capable of retiring the still-registered admission. This is a real
   permanent retained-use/registry leak, not a scheduling fairness issue.

The existing `consumed-after-finalizer-check` row cannot certify this today.
It expects one retirement submission, but the current `release_success`
performs no `request_drive`; `Publish` itself schedules no successor after
`complete`. A passing prior active-publisher row only covers consumption
*before* the guard's initial predicate, where the guard directly retires.

The proposed direction is sound with one necessary refusal rule:

* When `release_success` cannot claim idle, it must call `request_drive`.
  While the producer is active this atomically produces `ACTIVE|QUEUED`; the
  producer Drop sees `QUEUED` and dispatches the one governed successor.
  If the consumer wins after the producer's final recheck but before
  `fetch_and(!ACTIVE)`, this same queued handoff covers it.
* Before releasing `ACTIVE`, the producer guard should recheck
  `completion_consumed && roots_are_empty()` and retire under its exact active
  lease. That is the cheapest path for the pause window and drops an ordinary
  queued intent rather than submitting needless work. `CLOSED` must remain
  absorbing in `request_drive`.
* A finalizer successor can itself be refused by the pool. Current
  `handle_submit_refusal` converts every terminal refusal to a new
  `terminal_job` plus fault completion (`:564-578`). That is invalid once the
  only public completion was already consumed: it manufactures an unclaimable
  fault/root and strands again. Under the exact queued refusal lease, if
  `completion_consumed && roots_are_empty()`, retire directly; only a
  non-finalized root may enter retry/terminal-job handling.

Required deterministic native law (separate from the existing before-wake
row): drive a normal captured job to the real `Publish` phase, pause through a
test-only hook **after** its initial finalizer predicate and **before** the
active-bit release, poll the real public future to `Ready`, then release and
join the worker. Assert exact result/storage identity, eventual `CLOSED`, no
registry entry/admission, and no more than one governed retirement callback.
Repeat with the controlled one-shot `Shutdown` refusal applied to that
retirement callback: it must still reach `CLOSED` with no `terminal_job`, no
replacement fault completion, and no retained root. Pop every captured job
into a local before invoking it; the hook must use bounded channels and join
the worker before terminal assertions. This directly covers the production
interleaving without adding numeric-budget scope.

#### Coalescing Delta Review

The landed `release_success` fallback now calls `request_drive` after an
idle-lease claim fails. That closes the demonstrated accepted-submission
trace: the paused producer sees `ACTIVE|QUEUED`, its Drop submits exactly one
drive job, and the governed `Terminal` turn claims queued-to-active then
retires because the consumed completion has no roots. A concurrent `CLOSED`
state is safe: `request_drive` treats it as absorbing. A pre-existing queued
turn is also safe because it is already the terminalization opportunity.

One P0 remains on the new successor's pool-refusal path. The successor reaches
`submit_exact`, but current `handle_submit_refusal` always turns a `Shutdown`
or exhausted retry into `terminal_job + fault completion`. In the exact
already-consumed/empty-root case, no public future or terminal cursor remains
to collect those newly created roots, and `_lease` then drops active to idle.
Therefore the accepted-submit coalescing change is necessary but not alone a
terminal proof. Make `handle_submit_refusal` retire directly under its queued
lease when `completion_consumed && roots_are_empty()` *before* it stores a
retry/terminal job or calls `complete`; cover it with the one-shot-Shutdown
variant above. The same special case safely absorbs saturated/contended final
retirement and avoids creating a pointless timer after the public result has
already transferred.

### Required Plugin Root and Descriptor Budget Source Handoff (2026-09-07, source-only)

The required-root repair is aligned with the authoritative taxonomy, not a
tree-specific waiver. `🔣️taxonomy.json:18326-18332` declares two permitted
plugin children (`🎮️commands`, optional `🔨️modules`) and exactly one required
child (`🎮️commands`). `validatePluginContractRoot` at registry
`📜️script.ts:1222-1238` now reads only `pluginRequiredChildDirs`, while still
requiring the root `🦀️.rs` and rejecting a nested `🔌️plugin` contract. The
seven closed fixture rows and SQLite required-minus-present oracle cover:
required-only, absent/marker/component optional modules, root/command absence,
marker-only command, and nested-contract. That is the correct current
contract; no registry source asserts that optional modules must exist.

Two deliberately out-of-scope shape edges remain inherited from the old helper:
the direct helper tests existence rather than regular-file type, and treats any
filesystem entry named `🔌️plugin` as a redundant nested contract. The outer
taxonomy/file walk is where source-file shape is checked. They are not a
regression in this required-only repair. If root validation is later promoted
to a standalone security boundary, add directory/symlink and leaf-as-directory
fixture rows with a matching `lstat` contract; do not fold that policy into
this semantic required-child fix.

The descriptor budget source repair is also coherent. At descriptor
`📜️script.ts:166-173`, zero is preserved as unlimited both in
`remainingDescriptorEmissionBudgetMs` and the elapsed-time guard; nonzero
budgets retain the existing strict `elapsed > budget` expiration. The caller
uses the helper for both bounded children at `:232` (Cargo emitter build) and
`:234` (descriptor emitter). Thus `0` cannot become a one-millisecond child,
while a positive at-limit budget retains a one-millisecond child minimum after
its allowed checkpoint. The five-row AJV/Decimal/fake-clock law correctly
proves zero/before/at/after/negative guard arithmetic and owner-pair
nonpublication. It intentionally aborts at `emit`, before either child; the
child propagation conclusion is source evidence, not a native emission claim.

No immediate `emitOwnerDescriptorPairV1` caller supplies a positive deadline;
default callers retain the repository `buildBudgetMs() === 0` unlimited
contract. A future public/untrusted `DescriptorEmissionControlV1` should
validate `deadlineMs` once for finite safe integers (preserving `0` unlimited
and negative already-expired), because current direct programmatic input could
pass `NaN`/fractional/infinite values. That optional hardening is distinct from
the source-qualified five-row repair.

### Retained Administrator Effect Receipts and Reconciliation (2026-09-07, source-only P0)

The core receipt design is sound in source. Each backend stores the complete
`(operation_id, intent_digest, committed_at, outcome_code, optional event
range)` identity and verifies an existing row rather than silently accepting a
different identity: SQLite `directory/🪶️sqlite/🦀️.rs:468-492`, Postgres
`directory/🐘️postgres/🦀️.rs:375-418`, and Neo4j
`directory/🌐️neo4j/🦀️.rs:57-96`. Durable share/invite/session changes insert
their receipt before the same transaction's commit (for example SQLite share
issue `:909-928`, share revoke `:949-965`), and the event path persists,
projects, inserts its receipt, then commits under the writer transaction
(`:2180-2198`; equivalent paths exist in Postgres and Neo4j). The receipt
query requires both exact operation and digest. The retained HTTP task
reauthenticates the captured principal, holds the sorted authority union
through effect execution, persists a terminal only after its execution result,
and discards one-shot secrets after terminal storage or a dead receiver.

However, two inner timeouts create a factual terminal contradiction:

* `RevokeDocumentShare` wraps its same-transaction writer in two-second
  `tokio::time::timeout` at Hub `🚀bin.rs:6762-6775`.
* `RevokeUserSessions` does the same at `:6777-6786`.

If the database commit wins physically while timeout selection wins the
caller-visible poll, the effect receipt and durable mutation exist, but these
branches turn `Elapsed` into `phase = failed`. The retained task then appends a
failed terminal at `:6923-6926`. Reconciliation deliberately returns early
when **any** non-accepted terminal exists (`:6391-6394`), so the exact receipt
can never repair that false failure; a same request-id retry returns the false
terminal rather than reconciling the factual effect. For share revocation,
the same timeout can also skip in-memory Share binding/open-plan invalidation.
This is an actual ambiguous-commit P0, not a speculative backend difference:
all three backends commit effect and receipt together, so they can all expose
the identical acknowledgement race.

Smallest correct boundary:

1. Do not map timeout, commit acknowledgement error, or task abort after a
   durable writer has begun to a failed terminal. Remove the two nested
   two-second wrappers; the existing process-owned outer deadline already
   retains `accepted` on a dropped/late execution and permits exact receipt
   reconciliation.
2. For every durable writer `Err`, emit failed only for a backend result that
   explicitly proves rollback/pre-commit semantic rejection. Otherwise retain
   accepted and query the exact `(operation_id, digest)` receipt. Exact receipt
   means factual success/reconciliation; absent receipt means uncertainty, not
   invented failure. A typed `Applied | RejectedBeforeCommit | Uncertain`
   writer outcome is preferable to treating generic `DirectoryError` as proof
   of no commit.
3. When reconciliation observes `share-revoked`, re-apply the idempotent
   ephemeral Share binding/open-plan invalidation before or with terminal
   publication. Session rows already invalidate their session bindings before
   their subsequent async live-session lookup, but the same receipt-driven
   recovery principle should cover any later externally visible cleanup.

Required acceptance law: pause each direct durable writer immediately **after
the transaction commits its effect receipt** but before the method returns;
make the handler deadline/ack path win. Assert mutation plus exact receipt,
no failed/cancelled terminal, accepted retained until reconciliation, exactly
one redacted succeeded terminal after restart/retry, and no second effect.
For share revocation, also assert the prior in-memory Share grant and document
open plan are invalidated by recovery. Run the same protocol law against live
Postgres and Neo4j once those backends are provisioned; current source/query
parity is not a live backend receipt.

P1 consistency note: Postgres has both `PRIMARY KEY(operation_id,
intent_digest)` and `UNIQUE(operation_id)`, but its insert names only the
composite conflict target. A substituted digest for the same operation thus
surfaces as a backend unique-violation rather than the uniform explicit
`DirectoryError::Conflict` produced by SQLite/Neo4j. The transaction rolls
back, so this is not an authority bypass; make the conflict target or mapped
error explicit and add a backend parity row when tightening diagnostics.

### Capability Refused-Finalizer and Catalog-Read Repair Packet (2026-09-07, source-only)

#### Capability refusal finalizer: qualified source closure

The landed narrow branch in `db/⚙️engine/🦀️.rs:564-578` is the correct repair
for the previously demonstrated finalizer-refusal leak. `submit_exact` only
calls `handle_submit_refusal` after the exact `Queued` job has been rejected.
The handler first CASes `Queued -> Active` through `try_lease`, so neither a
second driver nor a public `close_step` can concurrently own that submitted
job. If the public future has already transferred its completion and every
root is empty, the branch drops that exact rejected job and invokes
`lease.retire()` before it can create a retry timer, fault completion, or
terminal job. Retirement makes the same lease state `Closed`, releases the
admission, and removes only the matching registry generation. A late
`request_drive` treats `Closed` as absorbing (`:459-475`).

This is reachable on the actual successor path: `release_success` loses its
idle claim while a producer lease is active, sets `Active|Queued`, the producer
Drop submits the governed finalizer successor, and a one-shot pool `Shutdown`
returns that exact `Job` into this branch. Dropping the job before retirement
is correct: the exclusive active lease and the handler retain the state while
the incoming job is destroyed; `lease.retire()` then stores `Closed` and
releases admission/registry. The source has no unclaimable root after the
direct retirement. The reported native `PzDib7` RED followed by `CMISVx/00`
GREEN17 is therefore an appropriate behavioral
qualification for this narrow branch; no remaining refusal-path P0 was found
in it.

The law should permanently retain its two direct assertions in addition to
terminal empty: after the controlled `Err(job)` it must observe that the
one-shot refusal was consumed and that both `terminal_job` and `completion`
are empty. Those distinguish direct retirement from an accidental fault-root
handoff. Contended/saturated final retirement has the same already-consumed,
root-empty predicate and is deliberately safe to retire rather than arm a
pointless retry.

#### CatalogRead: current P0 and exact test boundary

`DatabaseCatalogReadState` has not received the capability's active/queued
lease. It still clears only `scheduled` at the top of `drive_one`
(`db/⚙️engine/🦀️.rs:1850-1856`) before it takes a root. This creates two
concrete public-close races:

1. `Handoff` takes `work` into a stack local at `:1865-1871` before it stores
   `poll_work`; and
2. `RetainWork` takes `work` or `poll_work` into a stack local at
   `:1875-1882` before it stores `terminal_work`.

At either pause, `scheduled == false` and `polling == false`. A concurrent
`close_step` sees no state-held work at `:2035-2100`, can consume the remaining
roots and admission, and remove the registry slot. The paused driver can then
store its exact work back into an old, closed state. Polling is also unsafe at
the narrower interval after `poll_work.take()` and before `polling = true`
(`:1909-1914`); once `polling` is set, close correctly returns `Blocked`, but
the current pre-flag interval has no such protection. These are ownership
defects, not merely test scheduling concerns.

There is a separate current liveness P0. Both `schedule` and `drive_one` stage
cancel/stale terminalization without considering whether a terminal error is
already retained (`:1767-1784`, `:1855-1862`). `stage_terminal` sets phase to
`RetainWork` and enqueues cleanup. The next cleanup job re-enters `drive_one`,
sees the same `cancelled`/stale condition while phase is still nonterminal,
stages again, and returns before the `RetainWork -> DrainWork -> ReleaseWork ->
Publish` match can consume anything. This is an infinite governed cleanup
loop. Capability Open's exact guard is the local model: only stage stale or
cancel when `terminal_error.is_none()`; after the first terminal decision,
continue the already-selected cleanup phase. The same predicate is required in
the retry callback before it replaces a cleanup turn with a new terminal job.

The existing `database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key`
law cannot prove that behavior. `controlled_catalog_read_probe` deliberately
moves its only work from `state.work` into `state.poll_work`
(`:10796-10808`), but the law later asserts that `state.work` still contains
the storage pointer (`:12166-12184`). It must assert the unique owner across
`work | poll_work | terminal_work`, and execute only a bounded local popped
job per turn. It currently neither pauses a real stack-local transfer nor
proves convergence through the terminal phase.

Smallest coherent repair is a private CatalogRead driver lease, following the
capability `Idle | Queued | Active | Closed` state rather than sharing the
type prematurely. CatalogRead has a different result owner
`Result<Option<(DbIoPages, EpochFence)>, DbError>` plus a
`DatabaseCatalogRootKey`, retry job, and public terminal-result resume path;
making the capability type generic would obscure these roots. The CatalogRead
lease must cover entry-to-return of every `drive_one`, move `Queued -> Active`
before any root take, defer requests as `Active|Queued`, make lease Drop
dispatch exactly one successor, and make `Closed` absorbing. `close_step`,
future Drop/cancel, retry callbacks, and successful public result release all
need the same CAS authority. Admission/registry release is legal only under
that lease after `roots_are_empty`, including the checked-out terminal-result
bit.

Required sibling fixture/laws, with controlled jobs always popped under no
queue mutex and invoked after the lock is released:

* Three paused-transfer rows: Handoff, RetainWork, and Poll-before-`polling`
  pause. While each is stopped, repeated public close/cancel returns Blocked,
  preserves the exact storage/key/work identity and admission/registry
  generation, and produces no successor. Release the pause and drive at most
  64 governed turns to exact terminal empty.
* Eight real pipeline completion rows: `DbIoPages` and inner backend `Err`,
  each with synchronous, before-wake, after-finalizer-check, and refused
  finalizer-successor timing. They must prove one exact result/error delivery,
  no dropped pages/key/fence, at most one successor, and `Closed` only after
  handback. A backend error is not interchangeable with a driver panic: it
  must pass through `DatabaseCatalogReadResult.root` unchanged.
* A safety-critical retry/resume law is still required beyond those eight:
  force a real Contended/Saturated retry job, take the abandoned terminal
  cursor, close then resume its exact stored work/result, and fire the old
  retry callback afterward. The old generation must be inert; the new future
  must own exactly the original `DbIoPages`/key/fence or backend error and
  converge in at most 64 turns. This covers both stale retry ABA and a public
  result-resume path that the completion rows do not exercise.
* A checked-out terminal-result row must block final admission release until
  `take`, `close_step`, or Drop returns the checkout bit. This prevents a
  direct result handoff from being miscounted as an internally retired root.

No CatalogRead production change or native qualification was performed by this
review.

#### GIS law-3 pure-admission conflict: current-source audit

The latest native receipt `MXcv1Z/00` passed laws 0–2, then law 3 returned
`Conflict` before document state admission. Its bounded test output reports
`state=missing`, an empty publisher order, and a still-`prepared` SQLite
outbox. That correctly limits the active failure to
`validate_prepared_request` or `RetainedGisMapApprovalCommitterV1::preflight`:
neither cleanup reservation nor mount, WAL, Store assembly, verification, or
publisher code is implicated.

I checked the law-3 tuple directly against all current early predicates:

* `canonical_identity_and_base` updates `identity.input_hash` to the exact
  encoded Map-pack digest *before* `ledger.accept`; SQLite persists that
  supplied identity. The immutable base-digest comparison therefore agrees.
* The fixture `chainHash` is 64 zeroes and the base uses
  `ArtifactHash([0; 32])`; document, ordinal, empty genesis edit, and commit
  sequence are copied from the same identity.
* `canonical_approval` and `preflight` both derive the same deterministic
  Map snapshot, fixed-three work, parent proposal/inverse JSON, mutation hash,
  scope document key, actor, schemas, and canonical command. The tracked
  ingress copies the same scope/user/session/generation, while the law's
  `60_000 - 1_004` deadline is within the 120-second bound.

Accordingly, no remaining mismatched value is inferable from source alone;
weakening any of the 18 prepared-row or nine pure-preflight predicates would
mask an unobserved defect. The newly landed `cfg(test)` named-predicate output
at [validation](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:522>) and
[preflight](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:769>) is
the necessary next native discriminator. A tiny test-local assertion that
builds the exact law-3 request and invokes those two private gates immediately
before `commit_prepared_approval` would pin the same boundary before any
mount/publisher path; it should not replace the native law. The relevant
fixture builders are [canonical base](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2909>),
[canonical command](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2930>),
and [law 3](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3317>).

**Superseding source finding.** The named-predicate follow-up found the
concrete mismatch: `ArtifactHash::parse_hex` is a document-blob parser and
rejects the all-zero genesis value, even though that value is a valid
frontier-chain identity. The fixture's `chainHash` is exactly 64 zeroes, so
validation returned `Conflict` before the equality check. The narrow repair is
an exact lowercase 32-byte *frontier* parser that admits zero, used in both
prepared admission and recovery reconstruction; it must retain the existing
full chain equality check. This supersedes the preceding inability to identify
a source mismatch. WGPU reports the temporary diagnostics removed and a new
native rerun launched; that is not yet a native result.

#### Retained administrator factual receipt follow-up

The current short-admin path no longer has a source-visible route from a
committed durable receipt to a `failed` terminal. Each durable writer inserts
the exact operation/digest/outcome receipt in its mutation transaction:
SQLite at [receipt insert](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:468>),
PostgreSQL at [operation-id conflict fence](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:375>),
and Neo4j at [unique operation node](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:57>).
After effect admission, durable-writer errors become private `uncertain`; the
retained task sends HTTP 503 without a terminal record
([execution](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6694>),
[response cutoff](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6940>)).
Reconciliation promotes only an accepted record with the exact receipt and
expected outcome, invalidates the relevant share or user binding/open plan
before it writes `succeeded`, and otherwise leaves the accepted fact unchanged
([reconciler](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6391>)).

No current retained secret path reaches an effect receipt, audit row, or
status response. A live one-shot receiver alone obtains the private invite or
share string; a closed receiver drops it, while the public one-display result
also zeroizes its moved string on Drop
([private owner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6501>),
[public result](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:804>)).
The unavoidable serialized HTTP body is the one deliberate delivery, not a
durable replay source.

There is one real **P1 liveness gap**, not a committed-effect safety bypass:
an accepted operation whose durable call yields a generic error with no
receipt becomes `uncertain`, releases its runtime and permit, and returns 503.
Thereafter the same POST and GET status path reread the accepted row and absent
receipt, then return its `Accepted` receipt; they neither re-drive nor
terminalize it, while cancel has no live runtime and returns conflict. This is
intentionally safe under an
ambiguous commit acknowledgement, but it makes a proved rollback/uncommitted
writer failure permanently non-progressing too.

The bounded repair is a typed writer result that distinguishes only an
explicitly confirmed `RejectedBeforeCommit`/rollback from `Indeterminate`.
The former may append a factual failed or cancelled terminal; the latter must
remain accepted and reconcile only from the same-transaction receipt. Required
laws: (1) typed rollback proof gives one terminal and no receipt/effect;
(2) post-commit lost acknowledgement gives 503 then receipt-backed succeeded
with the effect exactly once; and (3) a generic no-receipt error remains
accepted rather than fabricating failure. No native/backend-runtime claim is
made by this source review.

#### CatalogRead fixture/source-gate review

The new `catalog-read-ownership` corpus is a useful exact TDD boundary. Its
schema rejects additional fields, fixes three transfer phases and the eight
stage/outcome completion cases, while the check script asserts their exact
ordered cross product at OS Rust script `:22-69`. The Bun SQLite model is an
independent, deliberately abstract state oracle for the fixture values; it
does **not** certify Rust behavior. The reported AJV1/SQLite3+8 source gate is
therefore accurately source-only. The registered native target lists both new
laws and the seven older CatalogRead laws (`:71-98`), but no native result is
available yet.

The new Rust laws are bounded: pipeline jobs use limits of 32/64, page close
uses 128, every queued job is popped into a local before invocation, and both
thread pauses use bounded five-second channels followed by `join`. There is no
remaining queue-mutex-held callback in the new two laws. The transfer law
uses the actual `DatabaseCatalogReadWork` owner, records its `Arc<DbBackend>`
identity while it is stack-local, and verifies matching admission registry
identity before release. The completion law constructs an actual
`DatabaseCatalogReadResult`, verifies the same storage pointer, root key,
page operation/bytes/epoch, and separately exercises the inner backend-error
variant. The one-shot refusal takes the exact returned `Job` through the
production `submit_exact` error route, so it is not a synthetic terminal-job
field write.

Two small additions are required before the fixture is a complete repair
qualification:

1. The paused-transfer rows drop the future (which does call cancellation),
   but do not first invoke `probe.cancel()` while the original future remains
   live. Add one explicit cancel-before-drop assertion in each row: it must
   retain the same stack-local owner/admission and create no successor. This
   distinguishes public cancellation from the abandoned-terminal handoff.
2. Add a `recovery` fixture group with these bounded source-independent cases:
   
   * `retry-old-generation-inert-after-resume`: capture retry callback C1
     after a real Contended/Saturated submission; abandon and resume its exact
     terminal cursor; force the resumed job to contend and capture C2; invoke
     C1, then C2. C1 must not take/reorder C2's job or mutate its phase. This
     requires a CatalogRead retry generation as in Capability Open and a
     test-only captured-callback seam, rather than relying on timer luck.
   * `abandoned-publication-resume-{pages,backend-fault}`: abandon before
     publication, then prove `terminal_completion -> TerminalHandle::resume`
     returns exactly the storage/key/pages+fence or inner backend error once.
     This is distinct from terminal-result checkout.
   * `terminal-result-checkout-and-resume-{pages,backend-fault}`: cancel/stale
     after a ready poll so `RetainResult` owns the result. While the shallow
     terminal result ticket is checked out, outer `close_step` must be
     `Blocked`; ticket Drop permits another take; its `resume` returns the
     exact result once, after which admission/registry can become empty.

Those cases are the smallest coverage for retry ABA, both existing result
handoff routes (`terminal_completion` and `terminal_result`), and the public
checked-out-root barrier. They should remain separate from the eight
completion timing rows, which correctly focus only on active finalization.

#### CatalogRead recovery implementation guidance

The proposed interception boundaries are correct. Move the controlled submit
hook to `submit_exact`, not merely `submit_drive_job`: terminal resume and
retry both call `submit_exact` with an already-owned exact job. Add a
test-only retry-callback capture at the `callback_at` boundary so the recovery
law can invoke C1 and C2 deterministically; a controlled worker queue alone
cannot prove that a delayed timer callback is generation-safe.

The actual old-callback trace is:

1. submission S1 returns Contended/Saturated and retains J1, then captures C1;
2. the public future is dropped; it marks abandoned/cancelled but must not
   enqueue a competing cleanup driver while J1 or a terminal job is the exact
   retained root;
3. the terminal cursor resumes J1, invalidating C1's retry generation before
   the submit; S2 returns Contended/Saturated, retaining J2 and C2;
4. C1 runs late and must be a no-op; C2 alone submits J2 and yields the exact
   storage/root output.

The cancellation rule needs one nuance: while an idle lease sees a retained
`retry_job`, `terminal_job`, or armed retry callback, it should not submit a
cleanup driver competing for that root. It must still record the terminal
decision (or, for a live waiter, publish its `Closed` completion) without
moving/dropping the exact job. The existing callback or public terminal cursor
then owns the one allowed transfer. Merely returning after setting
`cancelled` leaves a still-live caller pending forever, whereas eagerly
`stage_terminal + schedule_cleanup` races the callback and defeats the
one-root rule.

Do not copy Capability's terminal resume decision wholesale. A CatalogRead
work future is non-repollable after `Poll::Ready` or a caught panic:
`poll_backend_once` restores `poll_work` before recording the staged result or
panic, and `DatabaseCatalogReadWork::poll` would return `Pending` forever once
its `future` has been closed. In particular, a terminal handle arriving after
a ready poll can observe both `staged_result` and a spent `poll_work`. Current
`TerminalHandle::resume` prioritizes work/poll-work over that staged result;
that would re-poll a completed future rather than return the exact result.

Safe resume phases are consequently closed:

* A retry may resume only an exact job whose retry generation is made newer
  than every captured callback before submission.
* A work retry may resume only a live `work`/`poll_work` whose underlying
  future remains present and for which there is no `staged_result`,
  `terminal_result`, or `terminal_completion`. It restores the matching
  Handoff/Poll phase under the same lease.
* If a result is staged, or terminal work is spent/panicked, `resume` must
  return its cursor unchanged. `close_step` must first dispose the spent work
  and materialize `terminal_result`; only the shallow result ticket may then
  `resume` it into a new public future. A terminal completion error can be
  handed back only after no work root remains. Never set Poll for a spent work
  just because a `terminal_work` slot is nonempty.

This preserves `DbIoPages`, `EpochFence`, storage/key provenance, prevents a
ready/panicked future from becoming a non-waking Pending poll, and makes the
result-ticket checked-out barrier meaningful.

### Retained Administrator No-Receipt Outcome Plan (2026-09-07, read-only)

The present `DirectoryResult<T>` collapses three materially different facts:
semantic refusal before commit, a transaction known to have rolled back, and a
commit/transport result the caller cannot know. That creates the remaining
`Accepted` liveness P1. The accepted audit record keeps only an intent digest
and public metadata, not a replayable canonical intent
([audit row](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:333>),
[fact construction](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6324>)).
It deliberately cannot recreate a share or invite capability. An absent
receipt must therefore never trigger automatic effect replay.

Add one closed, repo-owned internal result at the `HubDirectory` admin-effect
boundary, instead of making the HTTP layer classify `DirectoryError`:

```rust
enum AdminEffectCommitV1<T> {
    Applied(T),
    RejectedBeforeCommit,
    Indeterminate,
}
```

`RejectedBeforeCommit` contains no raw driver string and maps to the stable
terminal code `admin-effect-rejected-before-commit`. `Indeterminate` contains
no result and maps only to the existing private uncertain/HTTP-503 path. This
makes all callers choose a factual branch; no `Err(_) => failed` fallback
remains.

The seven logical effect paths that need this result are:

| Logical path | Current entry | Lower atomic writer |
| --- | --- | --- |
| create space | [`execute_create_space_with_id_and_admin_effect`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2228>) | event page + receipt |
| ordinary directory event command | [`execute_with_admin_effect`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2206>) | event page + receipt |
| issue document share | [`issue_share_token_as_with_admin_effect`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2575>) | share + audit + receipt |
| revoke document share | [`revoke_share_token_as_with_admin_effect`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2587>) | revoke + audit + receipt |
| revoke user sessions | [`revoke_auth_sessions_for_user_with_admin_effect`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2700>) | session rows + audit + receipt |
| issue space invite | `CreateInvite` in [`decide`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1759>) | invite + audit + receipt |
| revoke space invite | `RevokeInvite` in [`decide`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:1770>) | revoke + audit + receipt |

The invite paths must not disappear inside the existing
`decide(..., Some(effect)) -> DirectoryResult<Decision>` conversion. Add an
admin-effect decision helper (or make the effect decision return the closed
outcome) so a typed invite result reaches `execute_with_admin_effect`. The
lower [`append_decided_events_with_admin_effect`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:2840>)
is the common implementation detail for the first two paths, not an eighth
public effect contract.

Pure preflight may return `RejectedBeforeCommit` only for a deterministic
domain result (`NotFound`, `Conflict`, `Unauthorized`, bounded input). A
`DirectoryError::Backend` from a read is `Indeterminate`. Once an explicit
transaction exists, every error *before any call to `commit`* goes through a
backend-owned `rollback_proved` helper: only a successful explicit rollback
returns `RejectedBeforeCommit`; rollback failure returns `Indeterminate`.
Every commit call, transport/begin/statement/stream/receipt-insert error for
which rollback is not confirmed is `Indeterminate`. A commit error is never
rollback proof.

#### Exact driver boundaries

All six lower backend writers must implement that state machine, while the two
DirectoryService paths forward it.

1. **SQLite** uses one `rusqlite::Transaction`: session revocation begins at
   [`:577`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:577>),
   share issue/revoke at [`:913`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:913>)
   and [`:954`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:954>),
   invite issue/revoke at [`:1595`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1595>)
   and [`:1696`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:1696>),
   and event append at [`:2184`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2184>).
   The current projection refusal explicitly calls `rollback` and is the model
   for proof-bearing refusal. Each early `?`, changed-row semantic refusal,
   and persist/project/audit/receipt error needs the same explicit branch;
   RAII Drop is cleanup, not a terminal fact. `tx.commit()` error remains
   `Indeterminate`.
2. **PostgreSQL** begins `sqlx` transactions in its session helper
   [`:609`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:609>),
   direct methods [`:881`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:881>),
   [`:933`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:933>),
   [`:1657`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1657>),
   [`:1802`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:1802>),
   and page append [`:2393`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2393>).
   A semantic zero-row result must `rollback().await`, and only its success is
   conclusive. `commit().await` error, pool/begin error, socket error,
   cancelled query, and rollback error stay `Indeterminate`: a PostgreSQL
   client disconnect cannot prove whether the server committed.
3. **Neo4j** starts `Txn` for sessions [`:278`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:278>),
   share methods [`:621`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:621>)
   and [`:677`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:677>),
   invite methods [`:1462`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1462>)
   and [`:1625`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1625>),
   and page append [`:2277`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2277>).
   Every `RowStream` must be exhausted or explicitly dropped before rollback
   or commit. The rejected `revoke_invite` branch currently retains its
   `accepted` stream through its immediate return
   ([`:1637-1639`](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:1637>));
   a rollback-proof helper must drop it before `txn.rollback().await`. Failed
   `execute`/`next`/`run`/`commit`, transaction-start error, and failed
   rollback stay `Indeterminate`.

The helper inserts the existing full
`(operation_id, intent_digest, outcome, event range)` receipt only as the last
write before commit; it must never append a receipt after an effect commit.
Preserve SQLite/PostgreSQL's operation-id uniqueness fence and Neo4j's
equivalent node identity plus full row-equality check. A same-row receipt does
not authorize a fresh share/invite: an `Indeterminate` retry rereads the
receipt and never re-enters the writer.

#### HTTP, cancellation, and bounded liveness

[`execute_admin_intent`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6720>)
maps `Applied` to the current success flow, `RejectedBeforeCommit` to a failed
terminal through `append_admin_terminal_with_retry`, and `Indeterminate` to
the private uncertainty response. The effect-admission CAS remains the
cancellation cutover:

* cancellation before `admit_effect` writes the existing cancelled terminal
  and opens no transaction;
* cancellation or HTTP receiver drop after admission never changes a typed
  driver outcome, never manufactures `Cancelled`, and never returns an issue
  token;
* `Applied` with a dropped receiver preserves effect receipt and terminal but
  zeroizes the ephemeral share/invite capability; replay/reconciliation never
  reconstructs plaintext;
* `RejectedBeforeCommit` yields one failed terminal, no effect receipt and no
  mutation; the same idempotency request replays Failed rather than rerunning;
* `Indeterminate` returns 503 initially and later exposes only Accepted until
  the exact receipt proves success. It must not become Failed because time
  elapsed.

This closes the **proved** no-effect dead end, but it cannot make a post-commit
or transport ambiguity eventually decidable. A durable retry would require a
new canonical command outbox and reauthorization after recovery; today's audit
metadata intentionally lacks both the full intent and secret capability. The
same availability limit applies if a proven-rejection terminal cannot be
persisted by `append_admin_terminal_with_retry`: after process loss there is
no durable proof from which to recover it. That remains an explicit P1; it is
not a reason to invent a failed terminal.

#### Closed fixture and independent oracle

Extend the existing [retained-short-admin fixture](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin/🔣️.json>) and AJV/Bun-SQLite oracle [`proveRetainedShortAdmin`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3237>) with a fixed `admin-effect-commit-outcome/v1` group. Its `method` enum is the seven paths above; `driverOutcome` is exactly `applied | rejected-before-commit | indeterminate-no-receipt | indeterminate-after-commit`. Do not permit open string values.

For each of the seven methods, use these three fixed rows:

1. confirmed rollback: HTTP 200/Failed, one failed terminal, zero effect rows, receipt, or token delivery, and replay remains Failed;
2. no-receipt indeterminate: first HTTP 503, zero terminal, later POST/GET Accepted, cancel Conflict, and no second writer entry;
3. lost commit acknowledgement: first HTTP 503, exactly one effect plus a matching receipt, no initial terminal, restart reconciliation produces one Succeeded terminal and zero secret replay.

Add five cross-cutting rows: pre-admission cancellation; cancellation after admission followed by proven rollback (Failed, never Cancelled); receipt operation/digest substitution (Conflict, zero new mutation); same-receipt second writer attempt (rollback, no second mutation); and semantic refusal whose rollback call fails (Indeterminate, not Failed).

The independent Bun SQLite model must represent effect, receipt, and terminal as one transaction and test both sides of a commit-ack error—receipt absent and receipt present—while asserting that both first report Indeterminate. It is an oracle for the language-neutral state table, not proof that PostgreSQL or Neo4j ran. The Rust native law should use the existing live retained-admin HTTP harness for the SQLite physical rows: pause after admission, inject one backend-owned precommit refusal/rollback and one postcommit lost-ack result, then inspect through a second SQLite connection before and after restart. Feature-gated PostgreSQL/Neo laws need their actual drivers; source checks alone cannot qualify their rollback behavior. Every law needs a bounded wait, task/slot drain, exact audit count, and a separate secret-wipe assertion for issue-share and issue-invite.

### CatalogRead Lease Final Source Audit And History Handoff (2026-09-07, read-only)

The new CatalogRead lease has no additional source-proved owner escape in the
completion/cancel/public-close/retry paths I inspected. This conclusion is
limited to the current implementation and the reported native receipts
`6NMGK8` (CatalogRead ten laws) and `PlQU4K` (Capability seventeen-law
regression), which I did not execute myself.

The key current linearizations are sound:

* [`DatabaseCatalogReadLease::Drop`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1774>) keeps the active
  lease until it has moved an abandoned completion to the terminal slot,
  observed retained retry/terminal roots, and either atomically leaves the
  queued successor or submits exactly one after clearing `ACTIVE`.
* [`release_success`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:2321>) does not treat an active publisher as a
  terminal fact. Its failed idle-lease claim requests a `QUEUED` successor;
  the active lease's final drop either dispatches it or, after a real submit
  refusal, [`handle_submit_refusal`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1909>) drops that exact
  successor and retires only when the completion was consumed and every root
  is empty. That is the needed resolution of the post-finalizer-check race.
* Cancellation intentionally leaves an already-retained retry/terminal job as
  the only successor root ([`cancel`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1861>));
  the retry callback observes its generation and cancellation before it can
  submit. [`TerminalHandle::resume`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:2503>) increments that generation before
  transferring the exact retained job. A stale callback therefore cannot take
  the replacement root.
* A ready or panicking backend work item is explicitly non-pollable
  ([`DatabaseCatalogReadWork::poll`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1642>)); the terminal resume
  gate rejects staged/terminal result ownership before it considers a live
  work root. Public result resumption remains a shallow handback through
  [`DatabaseCatalogReadTerminalResult::resume`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:2567>), not a second backend
  read. I found no source trace that turns an oversized result or a caught
  poll panic into a reusable work future.

The neutral corpus is materially aligned with those facts: three paused owner
transfers, eight completion/finalizer rows, and eight retry/terminal-result
rows ([fixture](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️fixtures/📖️catalog-read-ownership/🔣️.json>)).
No new CatalogRead change is recommended from this audit. A direct
panic-resume row would be additive coverage, not evidence of a current defect.

#### Concrete HistoryFuture Check/Register Lost-Wake P0

`HistoryFuture::poll` has the same check/register handoff that CatalogRead and
Capability just repaired, but it lacks their second completion read. At
[`engine.rs:10349`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10349>) it:

1. takes `completion` and sees `None`;
2. then stores the caller waker at [`:10355`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10355>); and
3. returns `Pending` without rereading `completion`.

Meanwhile [`ArtifactHistoryState::complete`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:9880>) writes the completion before
looking for a waker. If it runs between (1) and (2), it sees no waker and
returns. The caller then installs a waker after the only wake opportunity and
waits forever despite an exact result being retained. This is a real liveness
fault, independent of `register_artifact_history`: admission generations are
monotonic and the eight-slot registry cannot overflow while its corresponding
eight admission claims remain live.

Minimal repair: extract a private `take_completion` helper that takes the
completion **and clears the transient waker**. Call it before registration,
store the waker, then call it again before `Pending`; on either `Ready`, set
`resolved` and invoke `finish_if_terminal_empty`. Clearing the just-stored
waker on the second-read `Ready` avoids retaining a stale executor wake. This
is the same ordering already used by CatalogRead
[`DatabaseCatalogReadFuture::poll`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:2447>).

The smallest bounded exact-runtime law should use an actual `ArtifactHandle`
and `HistoryFuture`, not a synthetic `Future` implementation. Add one
test-only one-shot hook immediately after the first empty completion check.
Its body calls the real `terminalize_unhanded_request(Err(DbError::Closed),
HistoryProgress::Cancelled)`, which transfers the real request and reservation
into normal terminal cleanup and calls `complete` while no waker exists. Poll
with a counting waker and require, in that same poll, `Ready(Err(Closed))`,
zero wake calls, no retained `state.waker`, and bounded terminal close to
registry/admission absence. That tests the exact check→complete→register
interleaving plus real reservation retirement; it does not claim a full
history replay or bypass the actor owner.

#### History Completion TDD Review (Source Only)

Root's landed six-row test is a meaningful RED for the above defect; its
capture and cleanup do not mask the gap. The new private
[`HistoryFuture::prepare`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10193>) is behavior-preserving for production:
[`submit`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10184>) immediately schedules the same prepared state only when
`work` exists; construction/rejection states remain unscheduled exactly as
before.

For success rows, the test starts a real `Database`/document, applies two real
operations, then schedules the actual `ArtifactAuthority::history_retained`
actor turn. The one-shot completion capture at
[`engine.rs:14919`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:14919>) intercepts only the final
`complete` handoff: the captured `HistoryView` still owns the actual entry
vector, operation ids, admission generation/slot, and state `Arc`. For the
cancelled rows, `cancel` executes the real pre-handoff request-to-terminal-work
and reservation-close path before the same capture. Neither branch fabricates
a result future.

The `before-registration` row installs publication through the hook located
exactly between the first completion check and waker store
([`engine.rs:10379`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:10379>)). On current production code its first poll is
`Pending`, so `first_ready=true` fails independently of the fallback second
poll. The fallback exists only to recover the retained owner for bounded
teardown; it cannot turn that failed `first_ready` observation into success.
The same row also expects `waiter_empty=true`; current code leaves the waker
installed after its second poll consumes completion, providing a second
independent RED witness. The `after-pending` rows separately require exactly
one real wake.

Cleanup is owner-preserving rather than permissive: success closes the actual
`HistoryView` until every replay owner retires; cancellation drives the public
terminal cursor; both then assert the original admission slot is released and
`history_terminal(generation)` is absent. The process-owned ticket directory
is correctly used for the physical database. The AJV/Bun-SQLite six-row oracle
and registered native target are coherent. This review is source-only; the
new native law has not been run. The only required production change remains
the check→register→recheck plus waker-clear described above.

### Genuine GIS Cold-Map and Trusted Materializer Audit (2026-09-07, Source Only)

The two exact GIS cold-map laws are well formed as a *component-runtime*
boundary, but neither is a browser/Shell claim. The native target produces one
fresh `semio:gis` component, stages its descriptor from the same verified
snapshot, and invokes the two laws only with that staged component SHA and
descriptor SHA
([GIS script](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:331>)).
`produceFreshComponentV1` captures the component, extracted core and descriptor
before staging, verifies the descriptor's embedded component/core hashes, then
stages both immutable copies before lending the component
([producer](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:496>)).

The actual Rust law then reads and SHA-256 verifies that staged component;
opens the real `semio:gis` component; requires and ACKs its exact captured
lifetime ([fixture](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs:48>));
streams a real GIS Pack+SPR pair and checks the terminal `Applied` receipt's
lifetime, transfer generation, frontier and aggregate digest
([`:136`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs:136>));
retains/ACKs the real UI patch through the shard owner; and submits addressed
`patchPositions`, requiring a higher-revision `tiled-map@1` scene containing
only the changed marker
([`:173`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs:173>),
[` :214`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🌉️component-cold-map-patch/🦀️.rs:214>)).
The stale-lifetime companion demands `cold-pair.not-live` before a patch.
This is the correct first expensive component boundary. I did **not** execute
the two Cargo laws and found no native receipt in this audit.

#### P0: Current-Publication Paths Bypassed the Exact Cold Proof

`TrustedStdioGisBundleCheckScript --native` correctly materializes a fixed
generation, invokes `proveTrustedGisColdMapComponentV1` against that very
generation, then starts the candidate that can publish current
([Hub script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9264>)).
The proof rereads both literal GIS files from the retained generation and
compares their length/SHA-256 to the same receipt before and after the exact
two laws ([`:8110`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8110>)).

However, `DevScript` creates an absent current generation and immediately
candidate-publishes at
[` :8998`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8998>),
and `TrustedStdioGisBootstrapScript` does the same at
[` :9303`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9303>),
without that proof. This is a concrete pre-publication ordering gap, not a
generation-fence flaw: the latter fixes the component, descriptor and closed
actor closure and verifies final path identities
([`:7677`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7677>)).

The smallest repair is one private shared operation inside
`validateAndPublishTrustedStdioGisCandidate`, before staging or launching the
candidate: materialize → prove exact retained GIS component/descriptor →
candidate validate/publish. It must take an explicit caller-owned validation
target/artifact root; it must not select an unrelated target implicitly, and
its failure must leave the current pointer untouched. The existing
schema-first bootstrap corpus
([schema](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🧬️.schema.json>),
[oracle](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7407>))
is the right TDD home: require a bounded trace `materialized →
cold-map-proved(exact generation) → candidate-validated → current-published`,
with omitted, late and wrong-generation proof rows across native, Dev and
bootstrap callers. A prior persisted current remains outside this *new
publication* fence unless a proof record/revalidation policy is added
explicitly.

#### Worker/Shell Handoff Is Wired, But Not Yet Genuinely Composed

The worker creates its exact `VerifiedColdDocumentPair` from the current
execution-target lease and canonical bootstrap pair
([`backbone-worker.ts:2661`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2661>)).
After the child captures and ACKs its exact lifecycle receipt, it transfers
each page, requires `Applied`, processes the returned patch, then drives
bounded `surface-visible`/`wake` turns
([`:1336`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1336>),
[` :1388`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1388>)).
The reactor invokes the real `plugin_load_document_pack` only after terminal
cold ingress and a fresh live-lifetime check
([reactor](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:321>)).
The Shell accepts only a scope/client/surface/session-bound patch offer,
applies it to its retained `UiDocumentStore`, sends the exact ACK/rejection,
and renders that store through the normal `InterpretedUiNode` route
([Shell ingress](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1729>),
[`store route`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:7334>)).

The remaining boundary is genuine composition. The existing
`browser-actor-gis-describe-check --native` builds a separate component and
proves only JCO child `describe`; it explicitly makes no cold-pair, renderer,
map-mutation or collaboration claim. The worker cold-pair test uses a
controlled child. Thus no registered command presently proves the *same
materialized generation* through Wasmtime, browser worker and Shell.

The lowest existing runtime command after the order repair is:

```sh
bun ./📜️script.ts nx run @semio-tech/gis-plugin:component-cold-map-patch-native-check --skip-nx-cache
```

The materializer-level command is:

```sh
bun ./📜️script.ts nx run os-hub:trusted-stdio-gis-bundle-check --skip-nx-cache -- --native
```

It is the only current command that feeds the exact cold laws from one
retained generation before candidate publication. A final dedicated native
browser target is still necessary: consume that target's retained component,
descriptor and closed actor rather than build a second component; issue one
authenticated Session/bootstrap; transfer the canonical pair through the real
worker; and require a visible `TiledMapHost` plus its exact patch ACK. Only
then is the separate simultaneous-two-peer mutation/refetch acceptance ready.

#### Current Publication-Fence Status (Source Only)

The preceding P0 is now repaired in the current tree, but has no native
receipt in this audit. `validateAndPublishTrustedStdioGisCandidate` calls
`proveTrustedGisColdMapComponentV1` before either candidate staging or Hub
startup ([Hub script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8630>)).
`proveTrustedGisPublicationFixture` also mutates the source five ways—missing,
late, substituted-generation, detached, and swallowed proof—and requires the
same shared candidate function for Dev, bootstrap, native, and rotation
callers ([`:7450`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7450>)).
This closes the source ordering gap for a new `current` publication. It still
executes the component, not the retained `closed-actor.mjs`, and therefore is
not a browser-worker or Shell acceptance receipt.

### Composed Trusted GIS Two-Peer Acceptance Packet (2026-09-07, Source Only)

There is useful real infrastructure, but no existing target composes it. The
closest browser worker law is explicitly controlled: its final diagnostic says
`scene=controlled simultaneous=0` after it manually constructs a
`UiDocumentStore` node ([worker test](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4770>)).
It is evidence for scoped rebootstrap owner retirement, not for a GIS guest,
two live browser peers, or a rendered map. Conversely,
`browser-actor-gis-describe-check --native` builds a separate fresh GIS
component and an independent closed actor, and states that it has no Hub,
renderer, Map-mutation, or collaboration claim
([Hub script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5746>)).

The permanent next target should be a Hub-owned process target—e.g.
`trusted-gis-map-collaboration-process-check`—registered beside
`trusted-stdio-gis-bundle-check` in the Hub `📜️script.ts` and corresponding
launch configuration. It must consume, not rebuild, one
`TrustedBootstrapMaterializationV1` and its selected GIS actor closure. The
existing `runCollabE2eVerify` supplies the safe browser mechanics (two
independent Playwright contexts, separate Shell servers, network/WebSocket
diagnostics, and unconditional browser/server teardown)
([dev harness](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:2891>)).
It must not be reused as the acceptance target: it prebuilds the legacy plugin
set and starts its own ordinary Hub, so it does not bind the trusted current
generation or the retained GIS actor bytes.

Required ordering and assertions:

1. Materialize once in a ticket-owned data root; run the mandatory retained
   component proof; stage/publish that exact generation; keep the immutable
   receipt until teardown. Start the actual `os-hub` binary against that same
   root, with two distinct authenticated `react-relay` profiles. A relay stays
   host-owned; no bearer credential may cross into the worker or page.
2. Create one real shared space/document through the production directory
   command route. Its descriptor, target and `browserActor` fields must equal
   the selected Map target in that retained bundle. Publish an initial
   canonical GIS checkpoint through the normal checkpoint/publication route,
   with its fixed parent/drawing/value identity. Direct `ArtifactStore` writes,
   in-memory Hub fixture state, and a static dev-plugin asset are invalid
   substitutes.
3. Open the document in two independent Shell contexts. Each must obtain the
   production open plan, target assets, socket grant and Session; the worker
   must fetch the selected `browser-actor` asset, verify its
   generation-bound SHA/length, load it in the real dedicated child, and make
   `describe` equal the retained descriptor. The worker's actual activation
   path already enforces these conditions
   ([lease activation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:859>)).
4. Require the actual terminal Pack+SPR receipt, real guest
   `plugin_load_document_pack`, Shell `UiDocumentStore` patch ACK, and a DOM
   `TiledMapHost` scene in *both* pages. The child-side generic transfer and
   the Shell's scope/client/surface/session-bound patch ingress already form
   the intended path; the test may only observe them, not inject an artificial
   UI patch ([Shell ingress](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1729>)).
5. Submit a real inference job as the authorized peer, first exercise a
   bounded cancellation on a separate job, then approve a proposed
   server-stamped `CreateRegion`. A second Author may submit its *own* job;
   it must be denied read/cancel/approval of peer A's private job while
   remaining connected. Await the durable WAL/checkpoint path and assert an
   identical later frontier/region scene in both contexts. No controlled
   `patchPositions`, direct `UiDocumentStore.applyPatch`, or manually injected
   rebootstrap frame qualifies.
6. The subsequent live-socket `RebootstrapRequired` must cause both workers to
   discard their old pair and reconnect with fresh plan/grant authority. This
   is actual code, rather than a manual reopen: `requireArtifactRebootstrap`
   aborts the pair and closes the socket
   ([worker](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2594>));
   `connectHub` then re-enters the authenticated plan/grant path
   ([`:2219`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2219>));
   and the Shell restores the discarded session only after the fresh
   `snapshotReplaced` path succeeds
   ([Shell](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1876>)).
   Assert fresh pair/receipt generations, not merely a status banner.
7. Teardown must close both pages, workers, relays and Hub; inspect the worker
   capacity trace for zero actors/bytes and the Hub committer/recovery close
   for no retained document owner. Zeroize relay capability values in the
   harness output.

The minimum prerequisites before spending the full build/browser budget are:

- a qualified materializer native result containing the shared component proof;
- a genuine *retained-generation* closed-actor child proof (the present
  describe target cannot stand in because it rebuilds independently);
- WGPU's fixed-three inference path through actual WAL/checkpoint/reconcile;
  the initial document generation `0` must remain accepted at SQLite
  reconciliation. WGPU has now removed its remaining zero-sentinel at
  [`reconcile_committed_approval`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:657>)
  while retaining the upper bound and active/exact witness match. That is
  source-only pending its native law; it must not regress as this composition
  target is added.

Only after those are qualified should the new process target be invoked. No
native component, browser, Shell, inference, or two-peer acceptance result is
claimed in this report.

### Retained-Generation Closed-Actor Chromium Proof (2026-09-07, Design Only)

The smallest next proof belongs in the Hub trusted-bundle path, not in a new
component build. Extract the Chromium body of
`BrowserActorGisDescribeCheckScript` into a private shared helper such as:

```ts
type ClosedActorChromiumProbeV1 = Readonly<{
  generationId: string;
  component: Readonly<{ sha256: string; byteLength: number }>;
  descriptor: Readonly<{ sha256: string; byteLength: number; bytes: Uint8Array }>;
  actor: DocumentClosedBrowserActorV1 & Readonly<{
    path: "packages/gis/browser/closed-actor.mjs";
    byteLength: number;
    bytes: Uint8Array;
  }>;
}>;

async function proveClosedActorDescribeInChromiumV1(
  repoRoot: string,
  evidence: ClosedActorChromiumProbeV1,
  control: Readonly<{ artifactRoot: string; check(): void }>,
): Promise<void>;
```

The helper receives already-read, private actor and descriptor bytes—not paths
or a component-build request. It must first check the actor SHA/length and
descriptor SHA/length, require the actor's
`sourceComponentSha256 === component.sha256` and
`sourceDescriptorByteSha256 === descriptor.sha256`, and keep the schema,
codegen policy, policy digest and sorted import interfaces in the identity
input. It starts a private Vite endpoint only for an exact-length local actor
body and its deliberate stalled-body row; the page imports the static child
module, reserves one child, verifies WebCrypto SHA after fetch, transfers it
to `load`, invokes `['describe', 'describe']`, and calls
`verifyBrowserActorDescribeV1` against the supplied descriptor. This reuses
the current child proof's meaningful transfer, stall, source-detach,
result-transfer and zero-capacity checks without granting Hub/session/Shell
authority ([existing mechanics](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5746>)).

`trusted-gis-bundle-check --browser` should then do exactly this after
`materializeTrustedStdioGisBundle` and the retained component cold proof:

1. Decode the immutable `TrustedBootstrapMaterializationV1.bundlePath` with
   `trustedBootstrapReadCurrentBundle`; reconstruct the two generation
   receipts; run `trustedBootstrapVerifyGeneration` before reading leaf bytes.
2. Select only package `gis`, validate its browser actor with
   `trustedBootstrapBrowserActorV1` against that package's component and
   descriptor source digests, require the fixed actor path, and boundedly read
   `packages/gis/browser/closed-actor.mjs` and `descriptor.semio` from the
   receipt's own generation directory.
3. Invoke the shared Chromium helper, then zero both leaf buffers in a
   `finally` and run `trustedBootstrapVerifyGeneration` again. A changed,
   linked, non-regular, oversized or mismatching actor/descriptor therefore
   fails before publication and cannot be substituted between the child proof
   and its generation fence.

The current materializer already supplies every required persisted field:
`TrustedBootstrapMaterializationV1` supplies `generationId`, bundle digest and
bundle path; the GIS package record supplies component/descriptor digests and
the closed actor's path, byte length, actor digest, source component/descriptor
digests, policy digest and import interfaces
([materializer](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7979>),
[generation fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7811>)).
No receipt type needs widening and the helper must not accept caller-selected
paths.

### Correction: selected-current browser evidence

The implementation decision is now deliberately sequential: `--browser` runs
only after `--native` has published the exact candidate into a ticket-owned,
isolated `dataRoot`. It reads that selected current once, preserves the full
actor metadata and validates the same generation before and after Chromium.
Browser failure does not erase the valid native-qualified current, and no
browser-qualified-current invariant is claimed. This is evidence about the
already-selected generation, not a second publication policy.

Cancellation/teardown must remain bounded: `check()` occurs before/after every
awaited Vite, Chromium, page and child operation; the helper keeps one
`AbortController` for the deliberately stalled response, closes the child in
its page-level `finally`, then closes Chromium and Vite in outer `finally`
blocks. It wipes served actor, fetched source, guest return, descriptor and
any error-path temporary buffers. The Vite listener is loopback-only, has no
Hub route or credential, and its temporary root is ticket-owned. A failure
must leave the materialized generation/current pointer untouched.

Keep the current generic
[`browser-actor-describe`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🧪️fixtures/🔣️.json)
fixture as the shared descriptor-normalization and transport oracle. Add a
small `browserActorProof` section to the existing schema-first
[`trusted-stdio-gis-bootstrap`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🧬️.schema.json)
fixture for the trusted boundary: exact, actor-replaced, descriptor-replaced,
wrong-generation, cancelled-before-load, and late-after-publication. Its
required trace is `generation-verified → actor-leaf-verified →
chromium-describe → generation-reverified → candidate-validated →
current-published`; every non-exact row retains the previous current
generation. This is the narrowest language-neutral TDD that joins bytes to
their immutable generation while keeping the existing child fixture reusable.

The new `--browser` mode proves only retained-generation JCO child `describe`.
It remains explicitly short of a Session, real Pack+SPR cold ingress, Shell
rendering, inference or two-peer collaboration; those stay in the later
process composition target above.

### Retained browser helper review

The landed helper has the intended selected-current binding. It compares the
published pointer with the materialization receipt, builds the complete two
package receipt map, and calls `trustedBootstrapVerifyGeneration` both before
and after Chromium. That verifier reparses the full closed-actor metadata
(schema, codegen policy, policy digest, imports, source component and
descriptor hashes) and rehashes every regular leaf. The child helper separately
checks the actor SHA/length, descriptor SHA, transferred source detachment,
result detachment and zero terminal child capacity
([retained helper](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8356>),
[generation fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7908>)).
No source-visible identity bypass was found.

The remaining hardening item is not a demonstrated leak: teardown awaits
`browser.close()` before `vite.close()` without a distinct close deadline, and
the six neutral rows exercise metadata substitutions rather than a live stalled
response teardown. The existing child proof checks that the stalled child
returns actor/byte capacity to zero; a later runtime row can additionally
assert that every owned stalled Vite response has been closed before the
server's final close. It must not change the evidence-only/current-publication
policy above.

## Administrator Effect Outcomes — Current Source Audit (2026-09-07)

The live `AdminEffectCommitV1` boundary correctly distinguishes the three
facts required for a short durable administrator writer.  SQLite,
PostgreSQL, and Neo4j map a semantic preflight failure to
`RejectedBeforeCommit`; after a transaction has begun, they return that result
only after their respective `rollback` has itself succeeded.  Failure to
acquire/begin, a backend preflight failure, any failed rollback, and every
commit acknowledgement failure remain `Indeterminate`.  Each event page,
share/invite mutation, and session-revocation mutation inserts the exact
`(operation_id, intent_digest)` receipt before that same transaction commits
([common contract](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🦀️.rs:370>),
[SQLite event writer](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:2304>),
[PostgreSQL event writer](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🐘️postgres/🦀️.rs:2498>),
[Neo4j event writer](</Users/ueli/Documents/semio/🌎️hub/📇️directory/🌐️neo4j/🦀️.rs:2407>)).

The Hub preserves the outcome: `RejectedBeforeCommit` may form a terminal
rejection, whereas `Indeterminate` returns private `uncertain` and a 503,
without a public token, failed terminal, or retry of the mutation
([dispatch](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6757>),
[retained task](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6930>)).
The one-shot response owner zeroizes an undelivered invite/share secret; the
effect receipt and later audit response contain no capability plaintext
([secret owner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6512>)).
An exact committed receipt is therefore not hidden by a later audit-write
failure: reconciliation validates its outcome vocabulary, reapplies revoke
invalidations, and appends a redacted `succeeded` fact
([reconciliation](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6391>)).

### Resolved source issue: HTTP timeout no longer drops the admitted writer

The earlier outer-timeout P0 is **superseded by the current source**, not by a
runtime receipt. The retained task now awaits `execute_admin_intent` directly;
only the request waits on the response one-shot with `timeout_at`
([retained owner](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6969>),
[HTTP wait](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7017>)).
Consequently a 503 caused by the lexical request deadline drops only its
receiver. The process-owned task continues to own the admitted
`User`/`Session`/scope guards, operation permit, and effect future until it
gets an outcome. This is the required authority lifetime split; it should have
a physical paused-after-admission/request-timeout/competing-scope law before
being treated as runtime-qualified.

`AdminOperationTaskOwner::shutdown` still requests cooperative cancellation,
then boundedly aborts any task which does not exit
([shutdown](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6644>)).
That is only safe as process teardown once the server is no longer admitting
new work; it must not be reused as an ordinary per-request deadline.

### Remaining P1: receipt-free accepted operations have no autonomous liveness

There is one concrete, intentionally safe but operationally unresolved path.
After the accepted fact is persisted, a task timeout or `Indeterminate` writer
result sends 503 and retains no terminal fact. Reconciliation only advances an
accepted fact when the exact receipt is present; without one it returns the
accepted row unchanged. The same request id likewise joins the old receipt
instead of driving the effect again
([reconciliation](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6391>),
[request join](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6900>)).
That is correct non-replay safety, but after restart an accepted/no-receipt
operation can remain externally `Accepted` forever with no process-owned
resolver or typed terminal distinction.

The bounded repair is not to invent a rollback or append an `Indeterminate`
terminal after one miss: an ambiguous remote commit can become visible later.
Retain the accepted identity and add a nonterminal, durable recovery/visibility
checkpoint which reports `Indeterminate` while an owned resolver continues
querying the exact receipt. It must never transition to `Failed`/`Cancelled`
or rerun the writer; only a rollback-proven refusal may terminalize. Add
physical backend rows for:

- exact receipt after an acknowledgement loss → redacted `Succeeded`;
- failed commit acknowledgement or rollback failure → `Indeterminate`, no
  second effect on the same request id;
- confirmed rollback before commit → `RejectedBeforeCommit` terminal; and
- restart with no receipt → `Indeterminate`, never a fabricated rejection.

## GIS Publisher Genesis Fence — Focused Source Review (2026-09-07)

`GisMapApprovalCheckpointPublisherV1Impl::current_matches_base` is
fail-closed in its current call path. It first binds
`base.document_id` to `request.scope.document_id`; a missing active checkpoint
then admits only the exact genesis tuple `{ ordinal: 0, edit_id: "",
commit_seq: 0, chain_hash: 0x00…00 }`. With an existing checkpoint it requires
both an exact `DocumentScope` and the complete `ArtifactFrontier`
([predicate](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3427>)).
The caller independently verifies the current document descriptor, has a
strictly idempotent already-published comparison over scope, descriptor,
frontier and both blob hashes/lengths, and derives the post frontier from the
scoped actor snapshot ([publication fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3416>)). I found no source-visible genesis or
publication-authority bypass.

The focused native law covers the four non-document genesis fields, but not
the predicate's first document-id guard nor any negative `Some` case
([current law](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8823>)). Add
`None + wrong document_id`, `Some + wrong scope`, and a `Some` row for every
frontier field mismatch (including document id). This is a coverage P1 only;
it does not require changing the predicate. A differing existing checkpoint
descriptor is not itself a bypass: the proposed request must still match the
current document descriptor, and an intentional same-base replacement after a
descriptor rotation remains possible.

## GIS Map Collaboration Contract — Ordinary Creation And Undo Route (2026-09-07)

[gis-map-collaboration](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🤝️gis-map-collaboration/🔣️.json>) correctly makes this an ordinary-authority journey: a directory command creates the document, a normal checkpoint-publication creates the initial pair, two distinct authenticated Authors open it, and only then an owner-scoped CreateRegion/DeleteRegion approval and ordinary undo occur. Its verifier is deliberately a schema/source contract, not an observed trace ([script](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7528>)); it must not be presented as collaboration qualification.

### Reusable, public initial-checkpoint path

There is already one first-party process helper that exercises the correct public authority path without an ArtifactStore write or test-state mutation:

1. [commitCheckpointPublicationProcessMutation](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1207>) posts canonical announce-document, obtains the real open-plan and one-use socket grant, opens /socket/v1, sends SocketHello, then submits one normal Commands WireMutationEnvelope and requires both Persisted and Applied: Accepted at exact frontier head-edit ordinal 1 / commit sequence 1 ([envelope and acknowledgement](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1274>)). The REST handler independently requires a revalidated Author inside the sorted directory fence before DirectoryService.execute_idempotent ([command route](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4908>), [author rule](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:4757>)).
2. [proveCheckpointPublicationMcpProcess](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1471>) uploads the exact bounded Pack/SPR to the authenticated Hub blob endpoint and POSTs the canonical checkpoint-publications command, then verifies idempotent replay. The route accepts only an authenticated Author, checks descriptor/current/frontier under the exact scope's record and DocumentWrite fences, then delegates materialization/publication ([handler](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3625>), [publication fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3510>)).

Home should reuse that helper and its existing checkpoint-publication-process-fixture generator as the production-route base: it already proves an authenticated document, a nonempty durable frontier, a normal Hub blob upload, and a normal initial checkpoint. It is not, by itself, the requested genuine GIS interaction: the helper uses ticket-generated trusted GIS Pack/SPR plus a fixture db.pathmap.v1 diff/inverse, and its own terminal text expressly says “no GIS actor execution, inference, rendering, or commit claim” ([fixture boundary](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1135>), [declared nonclaim](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11481>)). It is therefore the correct authorization/checkpoint substrate, not evidence of a mounted GIS-created scene.

### Concrete missing link: mounted Shell undo cannot yet produce a GIS document command

The Shell has a real ordinary undo affordance. The history button calls onAction with action undo ([Shell](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6347>)), which calls plugin.handleAction, applies its history patch, and processes requested host effects ([action funnel](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4226>)). Separately, the real Shell-to-worker relay already turns plugin BackboneMessage mutations into ArtifactActorMsg localMutations ([relay](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:3010>)); the worker preserves owned pending batches/outbox and sends them as ordinary Hub Commands after a verified Session ([relay](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2341>), [acknowledgement and rollback](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2390>)).

But the closed GIS child is currently only invoked for describe and reactor.poll lifecycle/cold-pair/UI-patch calls ([child boundary](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:881>), [cold install](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1322>), [render loop](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1432>)). No actor action/command invocation exists which can turn mounted GIS undo into a localMutations envelope. Thus neither the initial visible CreateRegion nor a Shell undo can yet traverse the closed actor to the existing normal Hub command transport. This is the concrete production gap; it is not solved by the direct process helper or a UI-patch-only response.

The smallest coherent follow-on is one scoped document-action bridge at the existing private browser-actor reservation/worker boundary, not an ArtifactStore or page capability:

- Require the live DocumentExecutionTargetLease, browser grant/session, ActorInstanceLifetime, current cold-pair frontier, and current scope/client/surface owner before the child accepts the action.
- Give the GIS actor exactly an action/invocation input for the declared undo action. It must yield an actor-owned, bounded canonical inverse envelope (for this contract DeleteRegion of the admitted CreateRegion), tied to the original mutation/invocation and base frontier; do not let the page manufacture the inverse.
- Feed that envelope through the existing Shell BackboneMessage mutations → worker localMutations → queueOutbox/relayMutationsToHub path. Existing acknowledgement handling then owns rejection rollback, and the existing remoteMutations/ApplyEnvelopes route re-renders both peers.

The smallest genuine acceptance composes, rather than replaces, the existing process helper: (a) run its author command/open/grant/socket/checkpoint sequence to establish the same trusted current; (b) open two real authenticated Shell sessions and require both Sessions and mounted closed GIS child scenes before mutation; (c) have A submit its own server-stamped CreateRegion, while B is separately able to submit its own job but is denied read/cancel/approve of A's job; (d) invoke the visible Shell Undo control only; (e) observe a second accepted ordinary Commands envelope equal to the actor-issued DeleteRegion inverse, two changed equal frontiers/scenes with the region absent, and then checkpoint/rebootstrap if the contract requires a later public checkpoint. Include stale lifetime, Viewer/Spectator, and stale-frontier rejection rows. This specifically avoids all listed forbidden substitutes in the contract.

## Closed GIS Guest Action → Ordinary Commands / Undo — Source Plan (2026-09-07)

### Current contract and the exact gap

The closed component needs no new WIT export to receive a normal command. The
sole guest world exports `reactor`, `jobs`, `checkpoint`, and `describe`, and
`reactor.poll` already owns a bounded `command-page` alongside events and the
cold pair ([world](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1368>), [poll](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1203>)).  Its cursor already binds
`owner`, `generation`, instance, sequence, page index/count and metadata
([cursor](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1093>)).  The wasm P2/JSPI macro maps that async
export to the same reactor implementation as native; `jobs` and `checkpoint`
are not an alternate action route ([guest exports](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33273>)).

The private browser child currently sends `null` for every command page: open,
cold load, render, wake, and patch acknowledgement all call only
`reactor.poll` lifecycle forms ([open](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1343>), [cold load](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1404>), [render](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1441>)).
`reconcileUiPatches` admits only UI-patch traffic; it does not demultiplex
`turn-result.effects` ([reconciler](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1490>)).

On the other side, the closed UI is rendered with the global Shell callbacks,
and `onIntentStable` first turns its semantic intent into a normal local
`ActionDescriptor` ([retained UI owner](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1683>), [global action funnel](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:4557>), [closed renderer](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:7362>)).
Consequently it calls the ordinary Shell plugin handle, not the mounted closed
GIS guest. That is the missing production binding.

Raw WIT `ui-intent` must **not** be substituted for the command page. Although
the reactor has that event, `VcsArtifactApp::handle_intent_frame` currently
rejects it with `interactive-job.intent-raw-owner-required`
([event](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:692>), [rejection](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25423>)).

### Existing typed route, and what it currently loses

The right inner request is the existing packed `AppCommand::Command` carrying a
validated `ManifestActionInvocation`. `plugin_exchange` routes it through
`handle_action_invocation`/the declared action registry, then emits a
correlated `AppFrame::Invocation` ([dispatch](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32795>), [native public equivalent](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31553>)).
The exact page encoder already exists in both runtimes; it bounds one command
to 64 byte pages and carries the full cursor ([TS encoder](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:113>), [cross-page oracle](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:5168>)).

The action result is currently unusable for collaboration. `InvocationResult`
does contain the actual `KernelMutation`s and exact `UndoGroup`
([types](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:745>), [result construction](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21276>)), but
`AppFrame::Invocation` serializes only output, diagnostics, UI scope, history
patch, and messages ([frame](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1646>), [current producer](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32829>)).
The existing React adapter therefore explicitly returns an empty mutation and
inverse group even after a successful invocation
([adapter](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:1392>)).

`AppFrame::DocumentChanged` is a codec-only temptation, not a live result
transport: it has causal envelopes and round-trip coverage, but this command
path does not publish it ([frame](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1654>), [codec law](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2769>)).
`PureCommand`'s `Emit` is also wrong: it returns forward operation packs but
not the mutation identity, dependencies, inverse, or undo group
([emit producer](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33045>)).

The reactor already delivers any non-patch `AppFrame` as a shell-directed
effect ([route](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:861>), [frame demux](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:384>)).
The shared TS `decodeAppFrame` and `shellFrameBytes` helpers already decode
that exact transport ([codec](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2351>), [effect helper](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:78>)); the browser worker simply has not
used them for a command owner yet.

### Smallest coherent binding

1. Add one schema-first, private `browser-actor-action-v1` request/result pair
   beside the existing patch handoff. The request carries only the retained
   `scope`, `clientInstanceId`, verified surface id, activation generation,
   guest instance/lifetime, current UI revision, and an action descriptor.
   The Shell must select this callback only when rendering a retained browser
   actor store, rather than sending that store's actions through
   `onActionStable`. The worker—not the page—derives the plugin/package, app,
   window kind/instance and actor from the verified lease fields
   ([lease fields](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1335>)). It rejects a supplied controller/action address that
   disagrees with that descriptor/guest surface.

2. `DocumentBrowserActorReservation` owns at most one action cursor. Before
   each await, page, continuation poll, and result publication, it invokes its
   existing lease/socket/grant/current checks; close aborts and wipes the
   exact pending page/result. It mints the command cursor from its private
   generation and lifetime, sends `reactor.poll([], commandPage, null, budget)`,
   then drives only the same cursor through `pending`/`complete`/`fault`.
   A duplicate request may re-observe its owned terminal result; a different
   request while live is refused. This preserves the reservation's current
   replacement fence rather than adding an unowned worker-global promise.

3. Extend the correlated `AppFrame::Invocation` protocol with two bounded Pack
   fields: `mutations: Vec<KernelMutation>` and `inverseGroup: UndoGroup`.
   Encode/decode them using the existing `ToValue`/`FromValue` model and update
   the Rust serializer, TS `AppFrameValue` twin, `decodeAppFrame`, React/WGPU
   adapters, and its fixture corpus together. It is a shared channel contract
   change, not a browser-only JSON side channel. A correlated invocation reply
   then gives the child the actual forward diff, inverse, dependencies,
   invocation id, and history group; no consumer reconstructs them from an
   `Emit` op pack.

4. The worker decodes only a shell effect addressed to the exact guest
   instance and exact command sequence. A shared typed adapter converts the
   decoded `KernelMutation` into the existing `MutationEnvelope`, fixing the
   Hub document id from the retained lease and validating schema, diff,
   inverse, dependencies, author and base version. It then calls the current
   `handleLocalMsg({kind: "localMutations"})` path. That path already retains
   an outbox/pending batch, refuses a viewer before enqueue, submits normal
   `Commands`, and applies ordinary ACK rollback
   ([local ingress](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:4135>), [Hub relay](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2347>), [wire envelope](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:1941>)). The Shell result may say only `queued` after this admission; it must not
   present durable success before the existing command acknowledgement.

5. Do not use the existing `ArtifactCommand` history-only arm for this route:
   it is intentionally limited to six history verbs and replies `Done`
   ([restriction](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32873>)). Use the same packed `Command`/manifest-action route for a
   declared GIS action such as `patchPositions`, which already has command
   coverage ([GIS command registry](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:220>), [declared-action law](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1271>)).

### Undo boundary and required laws

The existing local `undo` action is not a collaborative compensating command:
it changes the guest history but returns no `KernelMutation`
([law](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37440>)). Thus the new bridge must carry its exact
`historyPatch`/`UndoGroup` for local UI, but must **not** mint a fake Hub
inverse envelope when `mutations` is empty. Peer-visible undo remains a
separate canonical compensating-mutation design; it can use a guest-issued
inverse only when that invocation truly returned a forward causal mutation.

The first executable acceptance should include:

- a Rust/TS Pack corpus for good, malformed, oversized, duplicate, wrong
  sequence/lifetime/generation/surface/revision action requests and a
  `Invocation` frame with mutations/inverse;
- one real GIS `patchPositions` command-page law which produces exactly one
  mutation with its true inverse group (the framework analogue is
  `operation_command_emits_kernel_op_with_true_inverse` in
  [plugin tests](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:38054>));
- a browser-worker hostile law which replaces the socket or lease after each
  page/result await and proves zero `localMutations`, wiped transferred bytes,
  and no retained action owner; and
- an end-to-end normal-worker law proving one accepted action reaches exactly
  one existing `Commands` batch, while Viewer and Hub rejection preserve the
  current outbox/rollback semantics. A later genuine two-peer GIS proof can
  build on that; none of these source observations is such a runtime proof.

## Flow Browser WASM Close Trap — 2026-09-08

### Runtime evidence

`semio-framework-os-flow-core:test-browser` currently has a genuine browser
asset failure. Running the existing JS test file directly with Bun reproduces
the registered failure: the default generated loader accepts
`flow_core_bg.wasm`, `catalogueJson` and a 32-request burst succeed, then
`integrated.features.lifetime.close()` traps at
`exports.flow_bridge_send(...)` with `RuntimeError: Unreachable code should
not be executed` ([existing integrated row](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js:40>)).

This is not a missing browser clock or generated-import failure:

- The default initializer dynamically imports the generated `flow_core.js`,
  calls its default `initialize`, and no custom imports are permitted
  ([loader](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js:18>)). Generated initialization invokes `__wbindgen_start`; the Rust start
  installs and checks the browser monotonic `performance.now` source
  ([clock](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/⏱️clock/🦀️.rs:21>), [bridge clock gate](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:5508>)).
- In a fresh process, the consumed module accepts a valid open frame and
  `flow_bridge_allocate` remains callable after the session is open. An
  empty, freshly initialized bridge reaches `begin_close` and
  `terminal_is_empty() == 1` without a trap. The same consumed module traps
  both on the session Close control and on `flow_bridge_begin_close` after
  that open. The failure is therefore in the session teardown branch, before
  the JS drain loop; it is not caused by `Date.now`, `performance.now`, the
  eight-millisecond deadline, or an uninstalled import.

The actual generated `flow_core_bg.wasm` is dated `2026-09-06T13:32:04Z`,
while current [bridge teardown source](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:285>) is newer
(`2026-09-07T16:55:56Z`). Consequently the live trap proves the currently
consumed artifact is broken, but it does **not** prove the current Rust source
will produce the same trap after a real WASM rebuild.

### Current source teardown gap

Independently of artifact freshness, the source lacks a retained session-domain
close contract. `FlowDomain` offers only `bind_session` and `start_feature`
([trait](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:92>)).
`close_session` immediately removes the `Session` resource and drops its only
`Rc<RefCell<D>>` domain owner ([close](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:559>)). But
`FlowDomainAdapter::bind_session` always creates a `FlowRetainedVcs`
([binding](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:655>)), whose documented lifecycle requires
`begin_close` followed by bounded `close_retired_step` until
`terminal_is_empty` ([VCS close cursor](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:1688>)).

This is a concrete ownership violation even if the stale artifact's precise
panic site cannot yet be symbolized: the bridge cannot prove the adapter, VCS,
or host are terminal before releasing the session domain. The failure scope
matches the runtime discriminator above (empty bridge clean; one opened
session traps).

### Smallest fail-closed repair and qualification

1. Rebuild the generated binding through the existing `wasm` router
   ([producer](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts:53>)) before interpreting a source repair as a browser fix. Do not catch and
hide `RuntimeError` in `transfer`; it already releases the copied request in
its `finally`, but a terminal WASM panic must reject/close the host rather than
look like a successful close.
2. Add a close lifecycle to `FlowDomain`, e.g. `begin_close`, bounded
   `close_step(AbiWorkBudget)`, and `terminal_is_empty`. Keep the `Session`
   resource live in a `Closing` state; on bridge close, mark/cancel it, then
   drive the domain cursor from normal `poll` turns. Only after the domain
   reports terminal-empty may `resources.close(handle)` decrement
   `active_resources`. `FlowDomainAdapter` must first begin/drive its
   `FlowRetainedVcs` and consuming `FlowHostRetirement` close cursors. This preserves the bridge's
   bounded JS drain rather than dropping retained values synchronously.
3. Extend the existing integrated `test-browser` row—not a mock—with two
   explicit assertions: actual initialized WASM with one opened session can
   close without a WebAssembly trap, and `terminalIsEmpty()` becomes true after
   the bounded drain. Keep `test-browser-clock` as the separate proof that
   `performance.now` installation remains active. A native bridge law should
   separately open one `FlowDomainAdapter`, call `begin_close`, repeatedly
   poll under positive bounded credit, and prove the adapter/VCS cursor reaches
   terminal-empty before the session handle is released.

The browser startup JSON currently validates loader/import-selection cases,
not this live retained-session close. The registered `test-browser` target is
therefore the right executable qualification after regeneration; no native or
browser proof is claimed for a rebuilt artifact yet.

### Settled implementation map: retained Flow session close

There are exactly two `FlowDomain` implementations in the current repository:
the production [FlowDomainAdapter](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:655>) and
the protocol-law-only `MockDomain` ([test implementation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:725>)). There are no other
production callers or downstream implementations. This permits a required,
not silently-defaulted, lifecycle extension:

```rust
trait FlowDomain {
    fn bind_session(&mut self, session: AbiHandle);
    fn start_feature(/* existing */) -> Result<Box<dyn FlowFeature>, FlowFailure>;
    fn begin_close(&mut self);
    fn close_step(&mut self, budget: AbiWorkBudget) -> Result<bool, FlowFailure>;
    fn terminal_is_empty(&self) -> bool;
}
```

`MockDomain` can explicitly implement the trivial `begin_close`/complete
case; it must not inherit a default that lets a future retained domain bypass
the invariant. `FlowDomainAdapter` is the one nontrivial implementation. Its
`close_step` should translate the existing ABI budget to the already-used
`FlowVcsGrant` exactly as `FlowVcsFeature::grant` does
([translation](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:710>)): one item/control/output/event, `bytes =
budget.byte_credit`, `fuel = 1`, `interrupted = budget.cancelled ||
budget.interrupted`, and a deadline no wider than the VCS eight-millisecond
limit. It must drive `FlowRetainedVcs::begin_close`/`close_retired_step`
([VCS contract](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:1687>)) and
the consuming `FlowHostRetirement::new(host)`/`close_step` cursor
([host contract](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2110>)) to
their own terminal predicates before returning true. `vcs` may be set to
`None` only after its exact terminal predicate; the retirement remains retained
until its own `terminal_nonopaque_is_empty` witness is true.
`FlowSurface` is copied state rather than a retained owner, so it may be
cleared during adapter close; it must not be used as a shortcut around either
cursor.

The bridge cannot invoke adapter close as soon as it receives Close. A live
Flow VCS feature retains a VCS operation whose own close cursor must run first
([operation close](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:1571>)); attempting
VCS terminal retirement before that operation releases its credit is correctly
`ClosePending` ([guard](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:1694>)). The
minimal safe bridge state is therefore:

1. Extend `FlowSession` with `closing`/`domain_close_started`, retain it in
   `FlowResource::Session`, and add a bounded `closing_sessions:
   VecDeque<AbiHandle>` to `FlowBridge`
   ([resource owner](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:207>), [bridge state](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:248>)).
   Close marks the exact session closed, cancels its child operations, and
   enqueues the handle; it does **not** call `resources.close`, decrement
   `active_resources`, or remove the handle from `sessions`.
2. Continue the existing operation work queue first. Its `session_closed`
   branch already drives each child feature/page cursor to terminal and then
   retires the operation ([current close branch](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:455>)).
   A session-close turn may begin the domain cursor only after no exact
   `RequestEntry`/operation still names that session. This check is bounded by
   the fixed request table; no new unbounded tracking is needed.
3. Once child operations have retired, one session-close turn calls
   `domain.begin_close()` exactly once, then one `domain.close_step(budget)`.
   `false` requeues the same session handle. `true` must be followed by
   `domain.terminal_is_empty()`; only then may the bridge remove the resource,
   remove its `sessions` entry, and decrement `active_resources` once. A
   domain error remains retained/retryable in the closing session; it must not
   drop the `Rc` or report bridge terminal-empty.
4. `begin_close` must snapshot/copy the current session handles rather than
   `mem::take` the owner list ([current eager removal](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:285>)). It asks the same
   close transition of every still-open session. `accept_control(Close)` on an
   already-closing session returns `Closed`; after terminal removal it returns
   the usual stale/unknown handle. This preserves the current duplicate-control
   property without minting a second cursor.

`poll` currently only advances `work` ([poll](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:631>)). It must select an operation
turn first and then one `closing_sessions` turn when no child operation is
queued, so close never races a VCS operation for ownership. Outbound messages
remain first priority exactly as today; their acknowledged event owners still
block bridge terminal-empty. This is a bounded state-machine extension, not a
second background closer. The two existing immediate assertions must become
bounded polls: protocol `stale_duplicate_controls_and_idempotent_close`
([current assertion](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:832>) and the
production adapter law ([current assertion](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:5824>)).

### Fixture and acceptance rows

Add one new neutral contract beside, rather than overload, loader-only
`browser-startup`: `🧪️fixtures/🧹️session-close/{🧬️.schema.json,🔣️.json}`.
Both the Rust law and the browser test should validate it (Rust's native JSON
codec; browser AJV). Minimum closed rows:

- idle opened session: Close refuses new requests, has one retained session
  before the first close poll, and eventually reaches one terminal release;
- active VCS checkpoint/page: Close cancels the feature and drains the feature
  before beginning VCS/domain retirement; no operation credit/domain owner is
  dropped early;
- duplicate/stale Close: no second `begin_close`, no second resource release;
- zero-credit, interrupted, and expired close polls: retain the same exact
  session/domain, then a valid fresh budget resumes it; and
- queued terminal event/page acknowledgement: no `terminal_is_empty` until
  the exact message owner is accepted, followed by bounded close.

Use the existing schema-validated `test-browser` actual-WASM row for the
runtime version of idle and active close, and retain `test-browser-clock` for
the independent real-clock assertion. The production native law at
`component.rs:5736` is the exact non-mock place for VCS/page cancellation,
while `protocol.rs` owns the generic mock-domain ordering/duplicate/zero-budget
matrix. A regenerated browser artifact remains required before either browser
law can qualify this repair.

### Correction and current Flow host retirement review

The earlier reference to `host.rs:2469` was wrong: that type is the distinct
`FlowEvalSession`. The real host owner is the consuming
[`FlowHostRetirement::new`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2110>) and its strict
[`Drop`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2245>) invariant. The landed source now makes the
correct structural move: `FlowDomainHost::{Open(FlowHost),
Closing(FlowHostRetirement), Closed}` owns the *same* moved host, and closes
the VCS before calling
[`FlowHostRetirement::close_page(1, budget.byte_credit)`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2171>). It neither constructs another host nor fabricates a
`StepContext`; this is the safe minimum seam. The original native wrapper
remains a compatibility wrapper around `close_page(1, 4096)` and still applies
its real `StepContext::should_yield` fence.

The exact close order is sound in the current source: the bridge retains the
session while matching operation entries exist; only then does the adapter
move `Open(host)` into `Closing(FlowHostRetirement::new(host))`, drain the
already-begun `FlowRetainedVcs`, and finally advance the host cursor. A
`FlowDomainHost` dereference after `Closing` deliberately panics, but no live
feature can reach it under that order: session close first drains every exact
child operation. This relies on keeping that child-operation scan before
`domain.begin_close`.

Two concrete defects were identified in the current close implementation:

1. **Superseded source fix, pending qualification — retained outbound frame
   had been discarded at global close.**
   [`flow_bridge_begin_close`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:5631>) had dropped `RETAINED`. That value is an exact
   `AbiMessage` previously popped from `FlowBridge.outbound`; dropping it does
   not remove the corresponding `EventEntry` or decrement `event_count`.
   The bridge can then never reach terminal-empty and the peer cannot ACK the
   vanished frame. The minimal repair is to leave `RETAINED` intact across
   `begin_close`—the existing terminal predicate already fences it, and the
   next poll delivers the same frame for normal acknowledgement. A less direct
   alternative is an infallible exact `push_front` restoration into the bridge
   before close. Current source removes the premature `RETAINED.take()`;
   qualification remains the component law that forces a retained event by polling with
   an undersized output buffer, call `begin_close`, poll it at exact size,
   ACK it, then prove bounded terminal closure.
2. **Superseded source fix, pending qualification — a feature close fault had
   lost its only scheduled operation.**
   [`FlowBridge::advance`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:423>) removes the operation handle from `work`, then the
   session-closed branch applies `feature.close_step(budget)?`. On `Err`, the
   early return bypasses the later requeue, while its resource and
   `RequestEntry` remain live. The retained session close turn subsequently
   observes that request forever and cannot begin domain retirement. Requeue
   the exact operation before returning a typed close fault (or retain a
   dedicated fault cursor); never remove its root on that error path. The
   The current `advance_operation` extraction requeues the exact live handle
   after an error (guarded against duplicate queue entries), so the original
   owner-loss trace is no longer present in source. Its new one-time failing
   `close_step` row needs to prove the
   request remains schedulable and that retry/terminal rejection has one exact
   release, not a stranded session.

`close_page` honors the ABI's actual byte credit for Flow-retirement, neural,
and snapshot subcursors; `maximum_items == 0` or `maximum_bytes == 0` makes no
progress. The remaining deeper qualification gap is real but separate:
[`DagHostRetirement::close_step`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2614>) has no item/byte grant and is invoked first by
`FlowHostRetirement::close_page`. It performs one structural unit at a time,
but cannot evidence byte-credit accounting. Treat it as a follow-on P1: give
that cursor the same `(maximum_items, maximum_bytes)` contract before claiming
end-to-end byte-accounted Flow host retirement. It does not justify replacing
the present owner-preserving adapter seam with a fresh host or synthetic job
authority.

There is one additional current P1 liveness boundary in the host adapter.
`close_page` returns only `bool`, but its real inner failures set the retained
state's `faulted` bit (for example an error from `FlowRetirement::close_step`
or `FlowStore::close_owned_step`) and then return `false` forever. The adapter
cannot distinguish that state from ordinary pending work, so a browser close
turn can reschedule forever rather than report a retained close failure. Make
the new in-crate page seam return a private typed
`Result<Pending|Complete, FlowHostRetirementFault>` (or expose an equally
private checked fault accessor and translate it to `FlowFailure`). Preserve
the existing public `StepContext -> bool` wrapper for renderer callers. Since
the only direct page caller is the in-crate component, `close_page` should be
`pub(crate)` rather than a new external retirement-driving surface.

No current source build or regenerated browser artifact was run in this audit.

## Invocation V14 Propagation Audit (2026-09-08, source-only)

The V14 channel shape is internally aligned at the declaration layer. The shared
pin is 14 in [`🔖️channel-version.json`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/🔖️channel-version.json>), Rust
[`CHANNEL_VERSION`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:24>), and TypeScript
[`APP_CHANNEL_VERSION`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2650>). The two trailing frame fields are present in the Rust and TypeScript
frame encoders/decoders. `InvocationResult`, `KernelMutation`,
`InverseMutation`, and `UndoGroup` all derive the expected camel-case `ToValue`
shape; the new TS projection uses the same `invocationId`, `baseVersion`,
`targetMutation`, `inverseDiff`, and `physical_ms` spelling. No field-name or
Rust/TS `ToValue` parity defect was found in the inspected source.

The current Rust producer also carries both values from the exact result at
[`plugin.rs:32831`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32831>), and the UI-only invocation intentionally supplies the
empty pair at [`plugin.rs:32305`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32305>). The native WGPU bridge has now been
updated to decode and return the same paired fields at
[`ProgramBridge:136`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:136>), and React/WGPU TypeScript source both use the shared strict projection. Those source changes are not a runtime qualification.

### P0: the shipped WGPU worker still drops the V14 result

The checked-in browser worker is behind its source. The source
[`plugin-bridge.ts`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:175>) now calls
`decodeInvocationResultPacks` and returns its nonempty result. In contrast, its
served bundle's [`decodeInvocationPayloads`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js:23159>)
omits both fields and [`performInvocation`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js:23188>)
hard-codes an empty mutation/inverse response.

This is a real browser consumer: Trunk copies the file in
[`🌐️.html:17`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html:17>), and browser boot constructs a module worker from it at
[`🚀️boot.js:815`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js:815>) and
[`979`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js:979>). Regenerate the worker after
the source boundary settles, then require the existing
[`check-frame-worker`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:611>)
gate before claiming browser propagation. Its byte-for-byte renderer comparison
already rejects stale generated output at
[`script.ts:242`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:242>).

The minimal acceptance law is one nonempty, canonical result pair through the
actual WGPU frame worker, then its JS bridge and the wasm-side `InvocationResult`
parse. It must compare mutation ids, inverse targets, group identity, and a
nonempty `memberEdits` row—not merely assert that an empty pair remains empty.
The existing WGPU package test projection row at
[`package-integration.ts:86`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration.ts:86>) is the closest first-party
seam, but currently uses an empty V14 pair.

### P1: the transport grants only decode-side bounds today

`INVOCATION_RESULT_PACK_MAXIMUM_BYTES` is correctly defined as one command
budget (256 KiB) and checked for both fields on decoding. However, generic
Rust [`encode_app_frame`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2345>) and TypeScript
[`encodeAppFrame`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2327>) serialize arbitrary-length values; the present tests encode an
oversized frame and prove only that the later decoder rejects it. Rust likewise
checks the resulting vector only after its general `read_bytes` call at
[`channel.rs:1840`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:1840>).

For an ordinary command, the plugin has already dispatched its state change and
sets `mutated = true` before it serializes the new fields at
[`plugin.rs:32828`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32828>). Therefore a post-publication decode refusal is not a safe
backpressure policy: it can leave a changed guest with no usable authoritative
result at the host. Make the producer's result-projection boundary construct
both packed values, validate each length, and validate their pair relationship
*before* it marks the command published or emits any invocation side effects.
If either value is over budget, the command needs its retained typed fault/retry
path rather than one half of a result. A generic checked frame constructor or
`try_encode_app_frame` should be the only normal transport encoder so another
producer cannot recreate this bypass.

Also make the exported TS
[`decodeInvocationResultPacks`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:1696>) reject either `ArrayLike` length before constructing a
new `Uint8Array`. Its known channel callers arrive through the bounded frame
decoder, but the public helper itself presently accepts directly constructed
unbounded inputs. This is a boundary hardening requirement, not evidence that
the inspected `AppChannelClient` path bypasses the decoder.

Required neutral rows: each individual pack at `limit` and `limit + 1`; empty
pair; each asymmetric pair; malformed exact field; and a result-projection
refusal proving no invocation frame, effect/event publication, or host-visible
partial result. The existing channel fixture and
[`return-content` schema](</Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📐️fixture-schema/🔣️.json:13>) are the shared locations for the
binary vector and result fixture, respectively.

### Authority and correlation scope

The new TS projection rejects unknown fields, unsafe integers, malformed byte
vectors, asymmetric pairs, and nonempty group content with an empty group
identity. It does not prove the `UndoGroup` is a signed persistence receipt,
nor does it currently cross-check that its `mutations` / `inverseMutations`
refer to every returned `KernelMutation`. No inspected consumer treats that
projection alone as permission to commit or undo, so this is not a demonstrated
authority escalation. If a later Shell/React consumer uses it as such, it must
establish the relation against the mounted action/result authority rather than
trust a self-contained pair.

Normal React and WGPU source calls receive `AppChannelClient.command` replies,
which filter explicit `in_reply_to` values to the submitted sequence at
[`appChannelFrameBelongsTo`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2691>) and
[`pumpOutcomes`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:2812>). Thus no current normal-path wrong-sequence result
substitution was found. The extension-completion adapter separately feeds
locally collected shell frames into its private projection; retain its captured
activation assertion and add a hostile foreign-reply row if that collection is
ever widened beyond one settled turn.

No build, generator, native execution, or browser run was performed for this
audit.

## Flow Browser Multi-Session Ownership Audit (2026-09-08)

> **Superseded source boundary.** The singleton-host defect below was the
> evidence that motivated the current `FlowBrowserRuntime` landing. Its
> current-source audit and remaining terminal-receipt gaps are recorded in the
> following section; this historical reproduction is retained for causal
> traceability only.

### P0: one `FlowSession.free()` closes the singleton bridge for every sibling

The production renderer deliberately caches one Flow module promise in
[`WasmSessionLoader`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:110>): it initializes the generated core and browser entry once, then returns a new
[`FlowSession`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:112>) for every caller. The browser module sets one
module-global [`defaultHost`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js:7>), and each
`FlowSession` opens its feature handle on exactly that host
([`flow-browser.js:39`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js:39>)). This is a real multi-window route:
every Flow-capable `NodeGraphHost` mounts a `FlowGraphCanvasHost`
([`NodeGraph:1147`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:1147>)), which independently calls `createFlowSession` on mount and `free()` on
unmount ([`NodeGraph:2059`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:2059>)).

The close implementation is host-global, not session-local:
`createFlowFeatures` emits the session handle *and then awaits*
[`host.close()`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js:183>). That invokes `flow_bridge_begin_close`, marks the shared state closing, and rejects every later `start` with “Flow host is closed”
([`flow-host.js:126`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js:126>)). `defaultHost` and the loader promise are never reset after that terminal
state. Thus unmounting A invalidates live B and also makes all later Flow graph
mounts create a session whose initial `open` is already refused.

A read-only Bun controlled bridge reproduced the concrete sequence against the
canonical browser source: initialize once, open A and B, close A, then invoke
B. B rejected with the exact `Flow host is closed` result. The same controlled
bridge also observed two direct `createFlowBrowserFeatures({source: sameExports})`
calls emit the two open frames with the same request id, `["1", "1"]`. This was
not a Wasm/native qualification; it is a narrow JavaScript runtime proof using
a test-shaped in-memory bridge.

There is a second source-visible owner leak that the shared-runtime repair must
remove rather than mask. Browser [`init`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js:31>) calls `createFlowBrowserFeatures`, which itself opens a session before returning
([`flow-host.js:183`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js:183>)); `init` retains only its host and
discards that first session features object. The first renderer session is
therefore not the first guest session, and no owner can issue the discarded
handle's close control. With a correct per-session close, that orphan would
also prevent clean runtime retirement.

The duplicate factory identities are independently unsafe. A direct factory
always constructs a fresh host with `nextRequest = 1`, even when passed the
same already-instantiated exports ([`flow-browser.js:9`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js:9>)). The generated initializer itself returns its module-global
Wasm exports after the first call ([`flow_core.js:2428`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.js:2428>)). Two independently-created hosts therefore have neither a host epoch on the
wire nor disjoint request ids; concurrent polls may steal replies or each
claim the same reply. The normal renderer loader avoids this only incidentally
by caching one promise, not by an owned browser-runtime protocol.

### Clean replacement boundary

Replace the browser global/factory split with one explicit private
`FlowBrowserRuntime` owner. It owns exactly one initialized Wasm exports value,
one `FlowHost`, the active-session handle map, and a `Closing|Closed` runtime
state. Its API should be structurally equivalent to:

```js
const runtime = await createFlowBrowserRuntime({ source, ...options });
const session = await runtime.openSession();
await session.close();                 // exact session handle only
await runtime.close();                 // sole flow_bridge_begin_close owner
```

`openSession()` calls `createFlowFeatures(host)` once and immediately registers
the exact `{slot,generation}` before exposing the session. `session.close()` is
idempotent and: cancels/retires only tasks registered to that session, sends
one exact `closeHandle`, then removes that session from the runtime map. It
must never call `host.close`. Only the shell/runtime root calls
`runtime.close()`: it fences new opens, drives all admitted session closures,
then invokes the single bridge terminal close. Since the generated core keeps
one Wasm instance for its module lifetime, a terminal browser runtime is not
reopenable; the renderer's runtime promise should be process/page lifetime and
the browser worker/page owns its sole terminal close.

Make the existing `default` initializer create/store that runtime without
opening a feature. `WasmSessionLoader` should cache `Promise<FlowBrowserRuntime>`
and request `runtime.openSession()`, not cache the module and call a public
constructor. Retire the public arbitrary-host factory shape or make it return
the runtime rather than an opened feature/host pair; no second host may wrap
already-live exports. This is a protocol replacement, not a reference-counting
compatibility wrapper.

### Required acceptance packet

Add a schema-first `semio.flow.browser-runtime-lifetime/v1` corpus adjacent to
the existing [`session-close`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️fixtures/🧹️session-close/🧬️.schema.json>) fixture, and run it through the current
[`flow-host.test.js`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js:15>) mock bridge plus AJV. The minimal rows are:

1. open A and B, close A, then complete a B command; exactly one A close
   control, zero bridge-close calls, and B's request id remains live;
2. double-close A; exactly one handle close and no runtime-map underflow;
3. start A, cancel its React owner before its `open` reply, then open/use B;
   A's late handle closes itself and does not terminalize B;
4. runtime close with A and B active; new open is refused, every exact handle
   is closed once, and exactly one `flow_bridge_begin_close` reaches terminal
   empty;
5. attempt a second independent runtime over already-live exports; reject
   before any duplicate request frame (or prove an explicit new Wasm instance
   with a distinct epoch—never silently accept the shared-instance case).

The React target should additionally mount two actual `FlowGraphCanvasHost`
instances using the existing renderer package test seam at
[`index.test.ts:2825`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts:2825>), unmount one, and prove the other still completes a mocked Flow task
before the runtime owner closes. That test must not merely mock two unrelated
sessions; it must share the runtime test double and observe that `free` never
calls its global bridge close.

No Cargo, source edit, generator, or browser/Wasm artifact run was performed
for this audit. The controlled bridge result is limited to the canonical
JavaScript host protocol above.

## Flow Per-Session Terminal Receipt Audit (2026-09-08)

### Current source verdict

The landing design is the smaller correct protocol shape: retain
`AbiControl::Close { handle }` as the session capability, and add the distinct
non-replaceable `sessionTerminal` event (2657), rather than creating a second
close request operation. A close request would need a new request root,
cancellation rules, and an ordinary reply lifetime only to prove the same
fact. The existing open request already supplies a stable provenance pair.

Current source records that pair on the session at
[`protocol.rs:216`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:216>) and, after every child request has retired and
the `FlowDomain` reports terminal, emits the exact old `{slot,generation}`
handle with the original open request/generation at
[`protocol.rs:603`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:603>). The schema now names the same
2657 event and exact handle body at
[`schema.json:196`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/🔣️.json:196>) and
[`schema.json:249`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/🔣️.json:249>). This is sufficient to prevent a new
session that reuses a slot with a newer generation from satisfying A's close.

The browser side also has the necessary ACK ordering: it resolves and removes
the exact session only in the `transferControl` commit after the event ACK has
been accepted by the guest
([`flow-host.js:63`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js:63>)). A rejected ACK leaves the decoded event in
`state.blocked`; the normal pump retries it without releasing the session.
The native resource may close after event insertion: the outstanding
`EventEntry` plus the browser's exact session owner are then the two retained
receipt owners. It need not keep an already-terminal domain merely until a
browser ACK arrives.

This is source-only. The committed component/Wasm artifact has not been
rebuilt or browser-qualified in this audit.

### Required native publication behavior

The native retry behavior is essential, not an optional transport nicety.
An event acknowledgement's low slot is `origin % 64`, because the event id is
`origin ^ (sequence << 32)` and `request_slot` uses the low bits
([`protocol.rs:639`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:639>),
[`protocol.rs:723`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:723>)). Thus an unacknowledged later operation whose request id is
congruent to A's original open request blocks A's terminal event slot. A full
outbound/event ring does likewise. The current session transition correctly
maps `Busy|LimitExceeded` to `Ok(false)` at
[`protocol.rs:627`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:627>), so `advance` retains/requeues the exact
session instead of sending `Err` to the component boundary. That distinction
matters because `flow_bridge_poll` maps any bridge `Err` to `-1`
([`component.rs:5606`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:5606>)), which the browser treats as a
host-wide failure.

Keep this transactional ordering in the final code:

1. wait for all child operation roots; start/drive domain close once;
2. require `terminal_is_empty`; preflight one outbound entry and the event
   slot before mutating the event ledger;
3. on slot/ring pressure return Pending while retaining the session work item;
4. after the event is inserted, retire the native session resource and leave
   the `EventEntry` until its exact ACK.

`push_event` now preflights outbound capacity before modifying its ledger
([`protocol.rs:639`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:639>)); preserve that order. Do not turn expected slot/ring pressure into
`Busy` from `advance_session_close`, since that reintroduces an unrelated B
session failure.

### Remaining concrete gaps before qualification

1. **Current protocol laws still assume no session receipt.**
   `stale_duplicate_controls_and_idempotent_close_do_not_leak` expects terminal
   empty after three polls without ACKing 2657
   ([`protocol.rs:912`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:912>)). The longer close law similarly ACKs only the child
   operation terminal and then expects bridge terminal-empty
   ([`protocol.rs:925`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs:925>)). Both need to observe, decode, and ACK the
   `FLOW_EVENT_SESSION_TERMINAL` separately. Otherwise they either fail after
   the semantic change or accidentally mask an event-ledger leak.

2. **A rejected close-control write currently leaves a permanently rejected,
   still-closing session.** `sessionLifetime.close` sets `owner.closing` and
   caches a promise before `transfer(encodeClose(handle))`; on transfer failure
   it rejects but neither restores a retryable input root nor retires the owner
   ([`flow-host.js:168`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js:168>)). The pump then continues
   forever because it sees a closing session, while `FlowBrowserRuntime.close`
   rejects before its sole global close. Retain an exact pending-close control
   frame/phase until send succeeds, or fail the *whole* host only after a
   terminal bridge proof; do not reject and orphan A while B remains live.
   The former is the coherent per-session policy.

3. **The neutral browser-runtime fixture declares but does not exercise its
   late-open and ACK-pressure facts.** It validates the strings `lateOpen` and
   `exact-acknowledged-terminal`, but the current mock test only closes A after
   both open replies and always accepts the terminal ACK
   ([`flow-host.test.js:24`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js:24>)). Add paused/open-reply and
   rejected-ACK rows rather than treating schema constants as execution.

4. **Generated declaration/output must be synchronized with the new source.**
   The source declaration generator now describes `FlowBrowserRuntime`
   ([`script.ts:47`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📜️script.ts:47>)), but the checked-in
   `flow-browser.d.ts` still declares `createFlowBrowserFeatures`, a default
   initializer, and a public zero-argument `FlowSession`
   ([`flow-browser.d.ts:31`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📝️flow-browser.d.ts:31>)). Regenerate/check this
   output before renderer type qualification; the current
   [`WasmSessionLoader`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx:105>) already follows the
   intended runtime API, but package declarations do not prove it yet.

### Minimal exact law matrix

Use the existing `browser-runtime` fixture and `flow-host.test.js` mock for
wire/control behavior, then one real `FlowDomainAdapter` law in
[`component.rs`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:5693>) for native ownership:

| Row | Required observation |
| --- | --- |
| A close / B work | A emits one 2657 with A's exact open origin and handle; ACK it; B completes a real operation; `flow_bridge_begin_close` remains uncalled. |
| Close before open reply | UI asks A to close while its open reply is paused; the late exact handle is registered, one close control is sent, A's 2657 ACK resolves `free`, and B remains usable. |
| Duplicate close | Same `Promise`, one close control, one 2657, one ACK; no double map removal. |
| Event-slot collision | Hold an unacknowledged event with `origin % 64 == A.open_request % 64`; domain A becomes terminal but remains retained/Pending. ACK the blocker, then receive 2657 exactly once. |
| ACK/control refusal | Reject an event ACK once and a close-control send once. The retained message/control is retried; no owner is released before the successful ACK/control. |
| Reopen while A closes | Open C while A is Pending publication; C has a distinct `{slot,generation}` and completes work. |
| Runtime close | Fence new opens; install close owners for each already-admitted session, receive/ACK all session receipts, then call the one global bridge close and prove `terminal_is_empty`. |

The component law must drive actual `FlowDomainAdapter` close work, not only
the JavaScript mock. It should retain an event/ACK ledger assertion: before the
2657 ACK, native `event_count` is live and browser A is unresolved; after that
exact ACK, A is absent while B is still addressable. No Cargo, browser, or Wasm
artifact execution was performed for this report update.

## Trusted Stdio/GIS Publication And App-Channel Drift Audit (2026-09-08)

### Observed boundary

This is a source-only audit. The retained producer capture
`🗑️generated/trusted-stdio-gis-browser/fresh-process-jaRPzf` was still compiling
`semio-s-plugin-stdio` when inspected; it contains neither a completed
materialization receipt nor a current-pointer publication. Nothing below claims
a producer, native, process, or browser acceptance result.

The existing output fence is substantial, but it is an **artifact-tree** fence,
not a common source/ABI-generation fence:

- [`materializeTrustedStdioGisBundle`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8132>) captures the two native-codec fact files once, then invokes
  `produceFreshComponentV1` for stdio and GIS serially. It does not snapshot
  their transitive Rust/WIT/host inputs or retain one common protocol digest.
- Each [`produceFreshComponentV1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:496>) result is internally bound:
  the descriptor must name the observed component and core SHA-256s before it
  is accepted. The catalog generation subsequently includes both package byte
  digests. A leaf-byte substitution cannot be relabelled as the same generation.
- [`trustedBootstrapVerifyGeneration`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7963>) closes the staged tree to the exact bundle,
  component, descriptor, and GIS closed-actor files; it rejects links and
  checks opened-file identity before/after reading.
- Native cold-map proof checks the GIS component and descriptor around its
  native call ([`script.ts:8333`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8333>)); browser proof rechecks the exact current receipt/tree
  before and after the child ([`script.ts:8412`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8412>)). Neither proves a stdio/GIS common ABI, nor includes the
  independently generated WGPU frame worker.

Consequently a shared protocol source changing between the stdio and GIS Cargo
invocations can produce a content-addressed, individually valid **mixed-epoch
pair**. The catalog deliberately gives that pair a new byte generation; it has
no semantic equality that says both components implement the same app channel.
Building `os-hub` before this materialization in the native/process/browser
gate adds the same unbound epoch between Hub binary and candidate artifacts.

### Current V14 correction and remaining P0

The prior report's shipped-worker omission is superseded. Current
[`frame-worker.js`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker.js:22189>) decodes the paired
Invocation result packs and forwards them at
[`frame-worker.js:23279`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker.js:23279>). Its separate generated-byte check
([`script.ts:242`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:242>)) only proves that file against the source
available to that check. It is not included in a trusted-catalog receipt,
generation, plan, lease, actor boot claim, or action admission.

There is currently no `appChannelVersion`, `protocolVersion`, or
`executionProtocol` field in the strict directory plan/lease/browser-actor
schemas ([`schema.ts:1086`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1086>),
[`schema.ts:1139`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1139>),
[`browser-actor.ts:13`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts:13>)). This permits a retained component produced for
channel 13 to be paired with the separately regenerated channel-14 worker
until some later decode fails. That is a fail-open action-admission boundary.

The smallest strict repair is the Home-owned required
`executionProtocol.appChannelVersion` (or a canonical ABI digest if the version
is insufficient) emitted in **each compiled guest's** descriptor, required
equal across the fixed stdio/GIS closure, included in the canonical profile
encoding/generation, propagated unchanged into plan, lease, and actor boot, and
compared to the worker's compiled claim before any action/result ingress.
Missing or unsupported claims must refuse.

The claim must be compiled-byte truth. The Rust descriptor emitter executes the
raw built component's `describe()` first and only patches byte-hash fields
([`describe.rs:426`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs:426>)). A host-side current `14` stamped after that call
could falsely relabel an already-built channel-13 wasm. If guest `describe()`
cannot provide the fact, derive it from a canonical ABI/WIT projection of the
same component bytes; the current producer retains only export names, not a
structural ABI digest.

Required cross-language rows are: V14/V14 accepted; V13 component/V14 worker
refused before action; GIS/stdio claim mismatch refused during materialization;
missing claim refused; and a descriptor field changed outside the component
proof refused. These are protocol-admission tests, not browser-render success
claims.

### Publication TOCTOU (P0)

[`stageTrustedBootstrapCandidateCurrent`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8477>) verifies and copies a generation to candidate storage.
[`validateAndPublishTrustedStdioGisCandidate`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8849>) proves that candidate, then calls
[`publishTrustedBootstrapCurrent`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8748>) with only the original receipt. The latter atomically renames a
three-field pointer, but does not revalidate the original receipt tree and has
no expected-current compare or publisher serialization.

Thus a source generation leaf changed after candidate copying/proof and before
the pointer rename can become current even though the candidate was the object
actually tested. Browser mode will discover the mismatch only after pointer
publication; native/dev/bootstrap do not add a post-candidate source fence.
Two concurrent candidates can likewise both qualify and last-writer-wins,
rolling current back from B to A after A's obsolete precheck.

Before pointer mutation, a consuming publisher proof must:

1. reread the receipt's canonical bundle and rerun
   `trustedBootstrapVerifyGeneration` on the original generation path;
2. compare the current pointer token observed before qualification with the
   one observed under publication exclusion; and
3. make `publishTrustedBootstrapCurrent` consume that proof rather than a
   naked materialization record.

Candidate A's changed leaf must refuse without moving current. Candidate A
paused after qualification, B published, then A resumed must refuse rather
than overwrite B. Those two rows belong beside the existing trusted generation
stage fixture.

### Cross-process publisher exclusion

There is no safe reusable TypeScript lock in the Hub script. The repository's
mkdir parity lock ([`dev script.ts:3973`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:3973>)) is a polling build mutex; a process crash leaves its
directory forever and it has no owner/recovery protocol. Do not repurpose it,
use a PID lock, or delete a lock by age. A precompare followed by `renameSync`
without exclusion is also not compare-and-swap.

The existing suitable primitive is private Rust
[`ArtifactCasFileFence`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:676>) and its
[`acquire_file_fence`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:864>) owner: it rejects links and validates the opened leaf, holds a
file handle with nonblocking `flock` on Unix and `LockFileEx` on Windows, and
releases through RAII/OS process exit. Generalize or expose this *OS-handle
owner* behind a narrowly scoped trusted-catalog publisher operation; do not
attempt to recreate it in TS or via a shell command.

The critical section is intentionally brief: snapshot current token before
qualification; acquire `trusted-catalog/publisher.lock`; reread token and
reject change; run the receipt-tree reverify; write/rename/fsync pointer; drop
the handle. An OS-released advisory lock supplies crash recovery without unsafe
stale-age deletion. It also needs no lock across Cargo, Hub startup, or browser
work.

## Native Trusted-Catalog Publisher Capability (2026-09-08)

### Existing authority and missing entrypoint

No existing Hub entrypoint can perform this safely.

- [`TrustedCatalogLoader::load_current`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:458>) necessarily obtains
  `trusted-catalog/current.json` before choosing a generation. It is therefore
  not usable to validate a first candidate or to repair a bad pointer.
- The opened-root authority already has the necessary safe route: a server-owned
  data-root handle can open a named generation descriptor-relatively without
  reading current ([`opened-root.rs:44`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:44>),
  [`opened-root.rs:58`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:58>)). It retains opened regular-file handles and
  bounds every read ([`opened-root.rs:69`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:69>)).
- [`os-hub`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7736>) has no command dispatch: normal `main` creates the normal
  configuration, calls `configured_artifact_authority`, which calls
  `load_current`, and then opens DB/directory/listeners. It cannot serve this
  pre-current operation.
- FD 3 is a genuinely cross-platform inherited transport today, but its current
  semantics are only the strict credential bootstrap handshake
  ([`local-bootstrap.rs:282`](</Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🦀️.rs:282>),
  [`local-bootstrap.rs:878`](</Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🦀️.rs:878>)). Reusing that schema would incorrectly couple catalog
  publication to identity/session issuance. There is no existing publish
  request, HTTP route, or CLI flag.

### Minimal capability shape

Add one early, exact binary mode, before `HubMode`, auth, database, directory,
or listener setup:

```rust
os-hub --trusted-catalog-publish-v1
```

It should reject every extra argument and require `native-artifact-execution`.
It reads one bounded request and writes one bounded terminal reply over an
inherited FD 3. Reuse only the existing cross-platform *file/async-frame
plumbing* from `local-bootstrap`; define a different domain and strict schema,
for example `semio.hub.trusted-catalog-publisher/v1`. Do not make stdin a
second authority path and do not extend the local credential bootstrap protocol.
The TypeScript wrapper can spawn the already-built Hub binary exactly as
`startLocalHub` already supplies its fourth stdio pipe
([`script.ts:780`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:780>)), but without starting a Hub or issuing a credential.

The request contains only a canonical bounded correlation id and a candidate
receipt:

```text
{ schema, kind:"publish", requestId,
  expectedCurrentSha256: null | lowercase-32-byte-sha256,
  receipt:{ profileId, generationId, bundleSha256 } }
```

It has no `bundlePath`, source path, data root, raw current pointer, or
permission selector. The TypeScript producer captures the SHA-256 of the exact
canonical current-pointer bytes (or `null` for absent) *before* candidate
qualification. `OS_HUB_DATA` names the server-owned root. Under the publisher
fence, Rust safely opens and canonicality-checks current, hashes those exact
opened bytes, and compares this expected value before it can write. The
terminal reply echoes only `{schema,kind:"published"|"refused",requestId,receipt}`
and a bounded non-sensitive code. It never returns component bytes, descriptor
bytes, native bindings, or a filesystem path.

Expose a non-fixture Loader entry, not `load_fixture`, conceptually:

```rust
pub struct TrustedCatalogCandidateReceiptV1 { /* private validated fields */ }

impl TrustedCatalogLoader {
    pub async fn load_candidate(
        data_root: &Path,
        receipt: &TrustedCatalogCandidateReceiptV1,
        providers: &dyn NativeCodecProviderSourceV1,
        context: &OperationContext<'_>,
    ) -> Result<VerifiedTrustedCatalog, AuthorityError>;
}
```

It must open `TrustedCatalogDataRoot`, open exactly
`trusted-catalog/generations/{receipt.generationId}`, open only
`trusted-catalog.json` relative to that directory, compare its bounded SHA-256
with `receipt.bundleSha256`, then invoke the same `load_selected` path and
require `verified.generation_id() == receipt.generationId`. It must never read
`current.json`. The receipt parser validates lowercase 32-byte digests and the
bounded profile identity before it opens anything.

This is a full verifier, not merely a hash checker: `load_selected` proves the
descriptor/component/actor closure and native-provider bindings. It also
registers the verified codec assembly at the end
([`trusted-catalog.rs:660`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:660>)). Therefore the publisher command must be an
ephemeral one-shot process that exits immediately after its response; it must
not be added to a live Hub or used as a repeatable in-process probe with no
codec-registration owner.

### Publication transaction

Inside the command, retain a private `PreparedTrustedCatalogPublicationV1`:

1. receive the caller's pre-qualification `expectedCurrentSha256`; it is a
   stale-write guard, not authority to select a path or current receipt;
2. run `load_candidate` outside the publisher fence to reject bad candidates
   without blocking another publisher;
3. take the generalized OS-handle publisher fence for the fixed
   `trusted-catalog/publisher.lock` leaf;
4. safely open, canonicality-check, and hash `current.json` (or prove its
   absence), then refuse unless it equals `expectedCurrentSha256`;
5. run `load_candidate` again while holding the fence, then atomically write,
   rename, and fsync the pointer; and
6. drop the pointer temporary, the `VerifiedTrustedCatalog`, and the file-fence
   owner before exiting.

The second load is required. The pre-fence verification establishes candidate
quality; the fenced load is the final exact-tree proof immediately before
pointer mutation. Candidate staging must remain append-only after its atomic
generation-directory rename; all trusted writers must use this publisher for
the pointer. An advisory lock cannot protect against an unrelated same-user
writer that intentionally mutates supposedly immutable generation files.

`expectedCurrentSha256` detects the ordinary stale A/B race, but its three-field
pointer source has an ABA limit: A → B → A recreates A's byte hash, allowing a
very old A-preparation to pass. If the product permits republishing/rolling
back to an earlier exact generation, strict publication-instance ordering also
needs a monotonic immutable `publicationRevision` carried in the pointer (and
therefore covered by `expectedCurrentSha256`), or an explicit
no-republish/no-rollback rule. A content hash alone proves content equality,
not that no intervening publication occurred.

Use a canonical **decimal `u64` string**, not a JSON number: `"1"` through
`"18446744073709551615"`, ASCII digits only, no sign, whitespace, leading zero,
or zero. Rust parses it with checked `u64`; TypeScript parses only via `BigInt`.
This avoids JSON/`Number` silently collapsing adjacent revisions above
`2^53 - 1`. Under the fence, decode the existing pointer, checked-increment the
revision, write that exact canonical field with the new candidate receipt, then
hash the complete canonical pointer. Initial publication writes revision `"1"`.
A→B→A now produces A revision `"3"`, not the stale A revision `"1"`, so the
old expected SHA refuses. Since this is greenfield, update the strict pointer
reader, all fixtures, source guards, and the candidate-private pointer writer
at once; do not accept three-field legacy pointers.

The current validation stage also writes `candidate-data/trusted-catalog/current.json`
directly so the isolated candidate Hub can start
([`script.ts:8578`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8578>)). Do not leave that write behind a generic
`publishTrustedBootstrapCurrent(dataRoot, receipt)` API which a later live
caller can accidentally reuse. Either route even the private candidate through
the same native publisher capability, or make it a separate
`installPrivateValidationCandidatePointer` operation which accepts an opaque,
fresh validation-root owner (not `string dataRoot`), requires absent current,
does its own full generation verification, and is uncallable for live
publication. The latter is cheaper because candidate-data is private and fresh;
the former gives one uniform implementation. In both cases only the native
publisher capability may replace an existing live `current.json`.

Generalize `ArtifactCasFileFence` as an owner restricted to an already-opened,
server-owned directory and one fixed leaf name. Do not export a raw arbitrary
path file-lock API. It needs the CAS implementation's no-link/opened-leaf
validation, Unix `flock`, Windows `LockFileEx`, cancellation checkpoints while
waiting, and RAII/OS-exit release. The publisher's lock is held only across
steps 3--5; not Cargo, candidate Hub startup, browser work, or the initial
candidate load.

### Required native fixture and laws

Extend the existing language-neutral publication fixture at
[`trusted-catalog/🧪fixtures/📤publication`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔣️.json>) rather than encoding
interleavings in script-only assertions. Add a `publisher-v1` section with
canonical `previous`, candidate A/B receipts, and these fixed rows:

| Row | Pause / action | Required terminal fact |
| --- | --- | --- |
| no-current | no pointer exists; supply A | `load_candidate` succeeds before any current read; pointer becomes A only after full verification |
| substituted-leaf | pause A after its first verified load; replace a descriptor/component/actor/bundle leaf; resume | second fenced load refuses; previous pointer bytes remain exact |
| stale-A-after-B | pause A after capturing P0; B fully publishes from P0; resume A | A returns `current-changed`; pointer remains B |
| holder-exit | process A owns `publisher.lock` and terminates before pointer write; start B | B obtains an OS-released fence without stale-file deletion and publishes exactly once |
| malformed-current | `current.json` is noncanonical or linked | refuse; leave it byte-identical; do not select a generation |

The first four need two levels of proof: a neutral/native library law using an
actual `TrustedCatalogLoader::load_candidate` with a provider fixture, and a
process law that starts two `os-hub --trusted-catalog-publish-v1` children over
FD 3. The crash row must be a real child exit while its handle is held; testing
only Rust `Drop` cannot prove OS handle release. A test-only control checkpoint
inside the private publisher service may pause the first/second verification,
but no pause field belongs to the production wire request.

### Handle-relative pointer replacement is a separate required primitive

The preceding `ArtifactCasFileFence` recommendation needs one important
qualification: the existing CAS implementation is reusable for the *advisory
lock semantics*, but not unchanged for the pointer write. Its `FsArtifactChunkCasStorage`
stores a `PathBuf`, opens leaves by pathname, and uses pathname
`std::fs::rename` on Unix / `MoveFileExW` on Windows
([`chunk-cas.rs:864`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:864>),
[`chunk-cas.rs:910`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:910>)).
That permits a trusted-catalog parent replacement between the path checks and
the rename. It must not become the publisher's final mutation primitive.

The loader already has the right ownership base. `TrustedCatalogDataRoot`
retains an opened server-owned directory `File`, and all catalog reads descend
from it descriptor-relatively ([`opened-root.rs:40`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:40>)).
On Unix this is already `openat(..., O_NOFOLLOW)`; on Windows it is rooted
`NtCreateFile` with `OBJ_DONT_REPARSE` ([`opened-root.rs:135`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:135>),
[`opened-root.rs:231`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:231>)).
Extend that private platform module, rather than exposing path operations:

1. Add a non-cloneable `TrustedCatalogPublicationRoot` obtained only from the
   already-opened data root and retaining the opened `trusted-catalog`
   directory handle.
2. Relative to that handle, create/lock the fixed `publisher.lock`, read the
   fixed `current.json`, create one unpredictable `current.tmp.<nonce>` with
   create-exclusive/no-follow semantics, write and sync the canonical pointer,
   then replace the fixed leaf by handle. Unix needs `openat`, `flock`,
   `renameat`, and `fsync` of the opened directory. Windows needs the same
   rooted `NtCreateFile` create mode already used by the loader plus a scoped
   `SetFileInformationByHandle(FileRenameInfo[Ex])` whose `RootDirectory` is
   that exact handle; its source is the opened temporary file, not a path.
3. Keep the fence and directory `File`s until the pointer byte readback through
   the same handle equals the just-written canonical bytes. Do not return
   `published` until the platform's supported data/metadata flush boundary has
   succeeded. If a Windows directory-flush guarantee cannot be established by
   the new low-level helper, classify its post-crash reply as indeterminate;
   do not claim a durable `published` acknowledgement merely from
   `MoveFileExW`.

This preserves the loader's no-link/parent-replacement guarantee during the
entire CAS comparison and replacement. A path-based temporary created beneath
an earlier validated directory is not an equivalent substitute. The platform
surface should stay `pub(super)` under `trusted-catalog/opened-root`; no Bun
FFI, shell lock, arbitrary path parameter, or stale-lock deletion belongs in
it. Add two process rows to the publisher fixture in addition to the previous
five: replace the path spelling of `trusted-catalog` while A holds its opened
publication root (A either commits to its original handle or refuses, but never
writes the replacement), and exit A after the temporary sync/before rename
(B observes the prior pointer and obtains the released lock). A separate
platform test must prove a reparse/symlink current, temporary, or lock leaf is
refused.

### Native Flow 54786 → 54926 fingerprint audit (2026-09-08)

The corrected comparison is two runs of the **same** registered
`flow-session-close-native-target`, not Flow versus the WASI release producer.
The target was preserved. Its first stdio output linked around 01:55, while the
second invocation began recompiling `semio-s-plugin-stdio` around 02:00.

This was not attributable to the two Flow test-import repairs alone. A direct
Flow source/test change should recompile the Flow unit but not its dependencies.
The retained target timestamps show the second invocation rebuilt the full
upstream chain immediately before stdio: `semio-framework-os-kernel`, UI,
framework core, graph, schema, plugin, and infinite all acquired new
fingerprint/dep-info records at 02:00, and the stdio unit then acquired a new
metadata output. `semio-s-plugin-stdio` directly depends on that framework
chain and Flow explicitly enables its `full-artifact-catalog` feature
([`flow Cargo.toml:34`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml:34>)).
Thus current evidence supports a genuine upstream fingerprint invalidation
between 54786 and 54926; it does not support target/profile/cwd churn or a
generated timestamp-only cause.

Cargo overwrote the old fingerprint JSON and the captures do not retain
`CARGO_LOG=cargo::core::compiler::fingerprint=trace`, so the exact changed path
cannot be proven retrospectively. In particular, Home's concurrent v14
framework edits are a plausible cause, not an established attribution. The
only bounded diagnostic improvement is to have the registered native runner
retain Cargo's fingerprint-dirty trace for the next warm retry, together with
the existing target/profile/features. Do not change targets or attempt to
share the Flow host-debug `cdylib+rlib` output with the independent wasm32
wasm-release producer; those are distinct Cargo units and cannot be reused.

### Next Executable MCP-to-Durable-GIS Collaboration Slice (2026-09-08, Source Audit Only)

The smallest credible next harness extends the existing real checkpoint
publication process. It is already a registered launch target:
[os-hub checkpoint-publication process check](</Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc:5851>).
The process starts an isolated Hub with verified GIS authority, creates the
document through the ordinary directory and socket route, publishes Pack and
SPR through the public checkpoint route, then starts an FD3-authenticated MCP
child ([Hub script:1473](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1473>),
[Hub script:1209](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1209>),
[Hub script:1490](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1490>),
[Hub script:1559](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1559>)).
It presently verifies scoped resource bytes, hashes, frontier, and cross-space
denial, but expressly makes no inference, actor execution, rendering, or
commit claim ([Hub script:11558](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:11558>)).
The following packet is an implementation recommendation, not run evidence.

1. Replace the current proposal process branch that only prints DESIGNED NOT
   RUN ([Hub script:9271](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9271>)).
   Reuse the checkpoint-publication setup and its ticket-owned artifact root;
   do not build a second component or write directly to a Store.
2. Start two ordinary local profiles A and B plus the existing administrator
   observer. A creates the private space, then promotes B to Author using B's
   server-returned identity through the ordinary upsert-member command. The
   contract requires two distinct Authors and permits B to submit B's own job
   ([collaboration fixture](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🤝️gis-map-collaboration/🔣️.json:20>)).
3. Deliver independent FD3 envelopes to two MCP workspace children. Each calls
   artifact_open before inference. Generalize the existing bounded
   activeMcpDocumentConnections and waitForMcpDocumentConnection helpers
   ([Hub script:1315](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1315>))
   to require two distinct sync-session ids and two expected users. Retain a
   real grant selector digest for each child through the existing waiter
   ([Hub script:1349](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:1349>)).
   This establishes two concurrent production document sockets rather than
   sequential resource reads.
4. Both children read the initial scoped checkpoint resource. Compare their
   catalog generation, component, descriptor and closed-actor identities,
   document, surface, checkpoint id, frontier, and Pack/SPR hashes and
   lengths. After Home's descriptor protocol field lands, include the
   guest-described app-channel version. It must be taken from the exact
   descriptor, plan, lease, and compiled component metadata, never host
   stamped onto older bytes.
5. A calls existing inference submit, events, and approve with the exact
   offered hash. The MCP handle is session-owned
   ([MCP inference:1226](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1226>)),
   forwards through the authenticated Hub binding
   ([workspace:1990](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1990>),
   [workspace:2045](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:2045>)),
   and Hub reconstructs CreateRegion plus inverse before its retained
   fixed-three-store and WAL commit
   ([inference runtime:2589](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2589>)).
6. Give B A's literal job handle for events, cancel, and approve. Each must be
   PERMISSION_DENIED without a page, cursor, proposal, or receipt. Separately
   B's own job must be admitted. Require A's offered page to carry the
   server-stamped CreateRegion proposal/inverse and A's approval receipt to
   report applied true.
7. Require each existing MCP binding to consume actual rebootstrap fanout. The
   Hub sends it only after checkpoint apply
   ([Hub binary:3500](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3500>));
   the binding invalidates precisely on that control and refreshes via the
   ordinary stream loop
   ([remote binding:355](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:355>),
   [remote binding:863](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:863>)).
   Wait for a new selector or session receipt per child, then re-read both
   resources. The later tuples must equal each other, differ from initial
   checkpoint/frontier/pair identity, and bind to the approval's public
   checkpoint/frontier.
8. Close both children; require both connections vanish, bounded output stays
   credential-free, and selected catalog generation is unchanged.

This is the narrowest real MCP to authenticated Hub to durable approval to
second-peer acceptance. Current ingress remains bounded and author-fenced:
request bodies cap at 1024 bytes ([Hub binary:7540](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7540>)),
and submit/approval take the document gate plus live-author revalidation
([inference runtime:2450](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2450>),
[inference runtime:2590](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2590>)).

#### Honest boundary after the packet

Submission runs inference inline before its HTTP response
([inference runtime:2450](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2450>)).
Thus a sequential MCP client cannot prove cancellation of a genuinely
in-flight normal job: an offer normally exists before submit returns. A future
process cancellation law needs a production bounded service yield/checkpoint,
not a sleep or fake job.

MCP history undo still routes a local ActionAdapter token
([MCP root:459](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs:459>)).
An approved GIS job returns a job, mutation, and proposal receipt, not a token
or Hub ordinary-document undo command. Invocation v14 propagation is useful
infrastructure, but not that durable undo bridge. This process packet must not
call history undo and misrepresent local history as undo of the WAL-backed
region.

MCP resource equality also is not a mounted Shell scene. The retained GIS
cold-map check keeps browser acceptance separate
([Hub script:8393](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8393>)).
The Sol-owned two-host/reactor work must load each refreshed pair through
plugin_load_document_pack, require exact surface/session receipts, and then
prove equal visible scenes plus ordinary durable undo. Controlled actors,
direct UI patches, and sequential peers are not substitutes.

### Compiled Protocol Materialization and Final Generation Fence (2026-09-08, Source Audit Only)

This review inspected the current Hub producer and Rust loader source only. The
reported publication source gate is useful evidence for the TypeScript fixture,
but is not a native loader or publication result.

#### What is correctly fail-closed

The materializer obtains app-channel protocol from the actual emitted Pack
descriptor, after checking the descriptor bytes against the fresh producer
receipt; it does not stamp a host-side version onto the component
([Hub materializer](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8262>),
[descriptor projection](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7166>)).
The resulting protocol is framed into the profile generation for both selected
packages and the target, so a descriptor protocol substitution changes the
generation identity ([profile framing](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7205>)).

Before a generation is staged or published, the final fence requires exactly
the GIS and Stdio closure, regular non-link directories, bounded file reads,
digest and length equality, descriptor-to-receipt protocol equality, and a
post-read identity pass ([final fence](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7997>)).
The publisher invokes that fence after re-reading its bundle and before
replacing current ([publisher](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8859>)).
This prevents a changed component, descriptor, actor, or bundle from being
accepted through this path; it does not yet solve concurrent-current CAS or
handle-relative pointer replacement, which is correctly a separate publisher
boundary.

The Rust loader independently requires descriptor protocol equality with the
bundle record and the live Rust channel version
([catalog validation](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1116>)),
then projects the decoded descriptor protocol into the open package
([selection](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:635>)).
Home's TypeScript and Rust plan and lease parsers currently require the same
v14 app-channel value and include it in field equality
([TS plan](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1245>),
[TS lease equality](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1498>),
[Rust plan](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1700>)).
Thus an actually compiled old component cannot be relabelled v14 by catalog
metadata: production loader admission refuses it.

#### Current actionable omissions

1. The candidate Hub acceptance proof compares the entire parsed plan package
   to an expected object which omits executionProtocol
   ([candidate plan oracle](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8919>)).
   With Home's strict parser, this makes the first real candidate plan reject
   even when the server correctly returns v14. Add the exact compiled
   selected.executionProtocol object to the expected package and an altered
   protocol denial row. This is an availability and proof-completeness defect,
   not a path to live publication.

2. The cold-map proof only rereads GIS component and descriptor digests before
   and after Cargo, and passes hashes by environment
   ([cold proof](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8441>)).
   It does not call the full generation verifier or compare the descriptor's
   decoded protocol with the bundle receipt. Final publication later does that,
   so this is not a live-current acceptance bypass. It can, however, execute a
   costly cold proof against a bundle whose protocol field is inconsistent with
   its otherwise hash-correct descriptor. Capture one immutable verified
   generation snapshot before each consumer: bundle bytes and digest, both
   component, descriptor, and actor receipts, decoded protocol, and final file
   identities. Cold map, candidate copy, and browser proof must consume that
   same snapshot and re-fence it after their await. Do not reopen and
   reconstruct semantic fields separately between sequential Stdio and GIS
   builds.

3. Existing generation-stage vectors cover missing, unsupported, receipt, and
   staged descriptor protocol changes, but do not prove the materializer's
   sequential compiled-descriptor capture. Add a source fixture where a fresh
   receipt matches emitted descriptor bytes carrying v13 or missing protocol:
   materialization must refuse before profile encoding or actor derivation.
   Add the positive vector asserting compiled descriptor protocol equals bundle
   protocol equals plan package equals lease fields equals actor boot claim.

4. FreshComponentReceiptV1 carries package/version and output digests, but no
   source epoch, and the materializer compiles Stdio then GIS serially
   ([receipt](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:47>),
   [sequential build](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8248>)).
   A shared-worktree edit between them can therefore produce a byte-self-
   consistent mixed epoch if both descriptors still claim v14. Capture an
   immutable bounded source manifest before the first compile, recheck it
   after each compile and before staging, and include its digest in the
   generation receipt/framing; or compile from a source-owned immutable
   snapshot. Output hashes alone do not establish one compile epoch.

#### Minimal native fence laws using the real opened-root loader

Do not promote the Node-only publication fixture into the sole publication
oracle. The native loader already owns the necessary no-link, bounded-open
primitive: it opens current and generation relative to a retained data-root
handle ([loader](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:457>),
[opened data root](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:44>)).
It has physical linked-root, leaf, and opened-handle substitution laws already
([link law](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:2167>),
[retained-handle law](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:2218>)).

Add a test-only helper that lays the existing prepared FixtureDirectory below
one real data-root path as trusted-catalog/generations/G and writes a canonical
current pointer for its actual profile generation. Use FixtureProviderSource
and TestControl with TrustedCatalogLoader.load_current before and after each
publication attempt; do not mock loader or providers.

The smallest laws are:

1. Pause after publisher qualification, replace a descriptor, component, or
   actor leaf by a regular-file rename, then resume. Publication must refuse;
   the old current bytes and the prior successful loaded catalog remain exact,
   and the provider observes no replacement bytes.
2. Prepare A and B from the same opened current P0. Publish B under its real
   CAS and fence, then let A resume with expected P0. A must return
   CurrentChanged, B remains the only loadable current, and neither candidate
   registers a codec on refusal. The eventual revision field must make
   P0-A-B-A fail too.
3. Open the original server-owned data root, replace its pathname with a
   distinct data root before final replacement, then resume. The publisher
   must only affect the original opened root or refuse; loader access through
   the replacement pathname must never see the candidate. This pins the
   planned handle-relative writer to the same authority model as reader opens.

Cancellation or crash coverage should assert that no temporary pointer is
accepted and that a fresh loader can still load exactly the old or the fully
new canonical current. It must not require a fabricated current pointer or a
mock-only filesystem result.

### Opened-Root Publication Owner and File Fence (2026-09-08, Source Audit Only)

The new `TrustedCatalogPublicationOwner` retains both the already-opened
`trusted-catalog` directory and a `FileFence`.  Its reads, generation opens,
temporary-leaf creation, replacement, and cleanup are descriptor-relative.
The path-replacement row is therefore meaningful source coverage: after the
catalog pathname is moved and a foreign replacement directory is installed,
the retained owner changes only the original directory.

`acquire_publication` checks its operation context before each lock attempt
and again after acquisition.  The post-acquisition checkpoint is important:
cancel/error drops the just-created `FileFence`, rather than returning a
detached lock.  `replace_current` writes and synchronizes a uniquely named
temporary regular file before replacement, and its failure cleanup removes
only an inode still identical to the retained file.  These are sound
source-level ownership properties.

The Unix fence uses `flock(LOCK_EX | LOCK_NB)` and `LOCK_UN`; the Windows
fence uses the first byte with `LOCKFILE_EXCLUSIVE_LOCK |
LOCKFILE_FAIL_IMMEDIATELY` and the corresponding `UnlockFileEx`.  The local
`OVERLAPPED` representation has the required pointer-sized internal fields
and 32-bit offset fields.  Windows replacement uses `FileRenameInfo` with a
directory root handle and a byte `FileNameLength` of 24 for `current.json`;
the source layout, flags, and handle-relative destination are consistent with
that ABI.  `NtCreateFile` also requests a non-reparse regular leaf/directory.
This is source analysis only: the new tests have not been run and there is no
native Windows ABI/runtime receipt.

Two bounded gaps remain before calling the publication lock/CAS proof
complete:

1. `trusted_publication_owner_lock_drop_and_exact_replacement_match_fixture`
   tests a second handle in the *same process*.  It does not demonstrate
   separate-process arbitration, which is the essential `flock`/`LockFileEx`
   contract and particularly important for the Windows FFI path.  Add a tiny
   child helper/process law: parent holds the real `.publication.lock`, child
   receives Busy; parent drops it, child acquires it; a child exit/drop then
   permits a fresh owner.  It must use the real rooted `open_lock`, not a
   synthetic mutex.
2. Unix `replace_current` compares the temporary leaf inode and then calls
   `renameat` by its name.  A writer able to mutate the retained directory
   namespace can replace that temporary name between the comparison and
   rename.  It can also unlink/recreate `.publication.lock`, causing two
   publishers to hold locks on different inodes.  This is not an escape from
   the opened root; it is a same-directory-writer threat.  If that directory
   is exclusively server-owned, make that namespace ownership an explicit
   boundary.  If another mutator is in scope, a file lock alone cannot supply
   mutual exclusion; source-file identity must be made atomic with publish or
   the publication directory needs a stronger exclusive owner.  Windows
   source-handle rename avoids the temporary-name swap, but lock-leaf
   replacement remains the same namespace-authority concern.

The existing three rows establish cancellation-before-replace, collision
preservation, rooted parent-path replacement, and Unix symlink rejection.
They should gain a real Windows reparse-point lock-leaf refusal row, not only
the Unix symlink branch.

#### Required CAS and durability contract for the upcoming loader integration

The owner itself intentionally does not choose the current generation.  The
publisher must therefore, while holding this exact owner:

1. reopen the candidate generation through `owner.open_generation`, perform
   the full trusted loader verification there, and bind the verified receipt,
   descriptor/protocol identity, component bytes, and candidate pointer bytes
   to that handle;
2. reopen and strictly parse `current.json` through `owner.open_current`, then
   compare both the request's `expectedCurrentSha256` and a monotonic
   `publicationRevision`; revision is needed to reject A→B→A even when the
   old SHA reappears;
3. only then replace the pointer.  A pre-lock read, path-based candidate
   reopen, or an expected SHA without revision is not a CAS proof;
4. treat `TrustedPublicationSync::Unconfirmed` as indeterminate durability:
   do not return a successful durable publication receipt and do not blindly
   repeat the effect.  Reconciliation must use a fresh opened-root loader to
   establish exactly old-or-new current after crash/restart.

Private candidate staging must either use this owner with a separate staging
pointer or be kept structurally unable to call the live-current replacement
API.  The latter prevents a future staging caller from bypassing the live CAS
fence.  Native proof should reuse real `TrustedCatalogLoader` fixtures and
provider control to cover A/B stale-CAS, A→B→A, generation-leaf replacement,
cancel-before-replace, and retained-root path replacement; the current Node
fixture oracle is useful schema/source coverage but cannot certify these
filesystem semantics.

### Flow Browser Runtime: Two Mounted Sessions (2026-09-08, Controlled-Bridge Source Audit)

The new runtime boundary is materially better than the former per-feature
global close.  `createFlowBrowserRuntime` owns one `createFlowHost` and a
private set of `FlowSession` objects.  A session's `free`/`close` reaches only
its `features.lifetime.close`; only `FlowBrowserRuntime.close` invokes
`flow_bridge_begin_close`.  The bridge's exact `2657` terminal event binds
the session handle to its original open request/generation, retains it until
the acknowledgement is accepted, and removes only that session.  Rejected
close controls and rejected receipt acknowledgements remain queued for a
later pump rather than fabricating retirement.

The React rows exercise the important visible projection of that contract:
two `FlowGraphCanvasHost`s use the one loader runtime; unmounting A closes
slot 1 while a command continues on B's slot 2; a delayed A open resolves
into A-only retirement while B remains command-capable; and explicit runtime
close is the sole global-close call.  The fixture also pins same-promise
duplicate close, duplicate-export owner refusal, and session terminal
backpressure.  This is useful controlled serialized-bridge/React evidence
only.  It is not a rebuilt Wasm, browser, GPU, or WebGPU receipt.

One concrete admission-failure hole remains.  `openSession` inserts its JS
session into the runtime set before `createFlowFeatures(host)` resolves, but
`FlowSession.close` releases it only in the success continuation after
`features.lifetime.close`.  If the initial native `open` reply is an error,
`#ready` rejects before a native lifetime owner exists; `#release` is never
called.  The permanently retained failed JS session then makes
`runtime.close()` reject its `Promise.all` before `host.close()` can begin.
This has no native session to wait for, so it must not be treated like a
failed terminal close of an admitted session.

The smallest repair is an explicit pre-admission state in `FlowSession`:
release the runtime-set entry exactly once when open rejects before
`sessionLifetime` was constructed; retain it on any post-admission close
failure until a terminal receipt or host-wide terminal fault decides it.  Add
one fixture/controlled bridge row where A's `open` errors, A is closed (or
runtime-close starts), B opens and completes, and global close occurs once
with no retained A.  Keep the existing delayed-success row: it proves the
opposite, namely that cancellation before a *late successful* open does wait
for A's real terminal receipt.

### GIS Inference Cancellation and Durable Guest Undo (2026-09-08, Source Audit Only)

#### Current observable boundary

The GIS algorithm already has a real bounded semantic checkpoint.  The Hub
constructs its request in [`HubInferenceRuntimeV1::infer`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2097>) and passes
`InferenceOperationControlV1::checkpoint` through the controlled GIS call.
[`infer_gis_map_controlled`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:247>) checks work `0`, checks again
after decoding the snapshot, and delegates every nested GIS value to
[`visit_lon_lat_pairs`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs:29>).
That visitor invokes the callback before each object/array/value traversal.
The control's acquire-load observes `cancel()`'s release-store, deadline, and
work cap ([`InferenceOperationControlV1`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🦀️.rs:42>)).  Thus the next visited value is the first
genuine computation cut point at which a running GIS job can stop.

That cut point is not presently reachable from the real MCP journey.  The
Hub's [`submit_gis_map_job`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2450>) accepts and starts the ledger claim, then calls
the synchronous `runtime.infer` directly before it returns a receipt.  It has
no `.await` between that call and computation completion.  The document gate
is deliberately released while computing, so the route *would* allow a
separate cancel request; however a current-thread Tokio executor cannot poll
that request while this CPU call occupies its only worker.

The MCP side makes the absence deterministic rather than merely
scheduler-dependent: [`HeadlessWorkspace::submit_gis_map_inference_job`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1990>) and its remote driver block until the
HTTP submit returns, while [`StdioTransport::serve`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️.rs:85>) dispatches one input line synchronously before
reading the next.  The source explicitly describes `notifications/cancelled`
as a request-level no-op and the job route as the only durable cancellation
([`mcp inference`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:960>)).  Therefore a normal stdio
client cannot obtain a job handle and send `inference_cancel` until the
inline submission already reached offered/failed/cancelled.  The bounded
process-local `INFERENCE_INFLIGHT` token is only an interruption of that
local wait; it cannot establish durable mid-run cancellation.

#### Smallest retained execution packet

No public receipt-state addition is required: the existing job schema already
has `accepted`/`running` and the existing `events` page already carries the
bounded progress cursor.  The required schema-first addition is a
language-neutral `gis-map-running-job/v1` fixture that describes the retained
owner and its terminal laws, not a client-supplied execution authority.

After the existing under-gate sequence `accept` → `retain_operation` →
`start` succeeds, construct and admit one runtime-owned
`RunningGisMapJobV1`.  It owns exactly: the accepted identity and claim epoch,
the frozen `InferenceMapBaseV1`, receipt/job id, `Arc<InferenceOperationControlV1>`,
ledger/runtime arcs, and owned directory/rebootstrap/document-gate references
needed for the existing terminal revalidation.  It must be admitted before
the submit handler returns the existing owner receipt showing `running`.
The HTTP/MCP request owns no terminal result after that point.

The retained owner runs only the current pure `runtime.infer` body on a
bounded blocking/CPU worker, retaining its exact control.  Its callback keeps
the current monotonic `ledger.progress(job, owner, run_epoch, completed,
total, now)` rule.  The async owner then reacquires the existing document gate
and performs the current live-author plus frozen-base recheck before
`ledger.succeed`; on cancellation/error it performs the current
`ledger.cancel`/`fail`; only after that terminal transition does it release
the operation slot.  This preserves the current no-write-during-computation
property and makes a second stdio line or concurrent HTTP request observable
at the real next-value checkpoint.  Merely wrapping the call in
`spawn_blocking(...).await` inside the request handler is insufficient: a
cancelled/disconnected handler would then lose the terminal ledger owner.

The terminal owner must distinguish these four existing-state cases:

1. Cancel before the worker starts: `checkpoint(0)` observes it and terminalizes
   cancelled without an offer.
2. Cancel after a paused real visitor checkpoint: the next visitor callback
   observes it; no proposal is stored.
3. Cancel after computation but before the under-gate success transition:
   `ledger.succeed` already rejects `cancel_requested` and terminalizes
   cancelled ([`succeed`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:287>)).
4. Cancel after `offered`: retain current behavior, which removes the private
   offer unless a prepared approval outbox prevents it; it must never attempt
   to undo a committed approval.

The smallest physical acceptance law should run the actual Hub route on a
`new_current_thread` runtime with a test-only checkpoint gate around the
existing GIS callback (not a replacement inference implementation).  After
the real `running` event/progress is persisted, issue its real authenticated
`POST …/jobs/{id}/cancel`, release the checkpoint, and require a cancelled
job, no proposal/outbox/WAL event, zero retained operation owner, and a
subsequent submission capacity reuse.  Companion fixture rows cover the four
cut points above and verify the exact run epoch and progress cursor.  This is
the missing runtime proof; the native GIS codec's current controlled-callback
rows prove callback semantics only, not a concurrent route or stdio journey.

#### Durable undo is not connected to a GIS approval

`ActionAdapter::history_undo` resolves an in-process `undo_` handle containing
only `(instance, txn_id)` members and fans `TransactionUndo` over its local
channel ([`UndoRecord`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:332>),
[`history_undo`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:868>)).  The real
`PluginArtifactChannel` has a `TransactionUndo` frame mapping, but its
[`persistent_command_completion_port_ready`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:910>) currently returns `false`; an unprepared real
channel therefore refuses the command as not wired.  Its passing adapter test
uses `MockArtifactChannel`, so it is not a durable guest-history receipt.

More importantly, GIS `inference_approve` bypasses `ActionAdapter` entirely.
It calls the Hub approval route and receives a `{jobId, mutationId,
commandHash, proposalHash, applied}` receipt
([`inference_approve_handler`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1373>)); the Hub rebuilds and
stamps the inverse server-side ([`server_stamped_command`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2133>)) and the retained
committer persists its own WAL witness.  No ActionAdapter undo record is
minted for that receipt, and no Hub GIS undo route exists.  Enabling the local
command-completion port alone would therefore still not undo a durable GIS
approval.

The narrow durable connection is a new Hub-owned, witness-derived
`GisMapApprovalUndoTargetV1`: `{scope, originalJobId, mutationId,
commandHash, proposalHash, committedWitnessDigest, beforeFrontier,
afterFrontier, ownerUser/session/generation}`.  It is created only after the
sole committed WAL witness and is never supplied by the client.  A new
authenticated `POST …/inference/gis-map/jobs/{job}/undo` accepts an exact
target id, expected current frontier, and idempotency key—not inverse bytes.
Under the same `DocumentWrite` authority as approval, it revalidates current
user/session/membership and document frontier, reloads the original witness,
rebuilds its inverse from the canonical command, and appends a new inverse
event through the same retained committer.  It returns a durable undo receipt
only after that event/checkpoint witness; stale/foreign/non-tail targets have
zero new WAL events.

At MCP, evolve `UndoRecord.members` to a closed tagged union of the existing
local guest transaction member and a `HubGisMapApproval` member.  The latter
is dispatched through an explicit workspace `HistoryUndoPort`, not coerced
into a guessed plugin instance/transaction id.  A receipt token remains a
session-bound capability locator; the Hub witness and fresh authority remain
the durable authority.  The existing local completion-port work is a separate
prerequisite for normal guest `action_invoke` history, not a substitute for
this Hub undo route.

Required acceptance is one real Hub+SQLite/restart law: submit and approve,
restart the runtime/ledger, undo using the returned durable target, and prove
one inverse WAL/checkpoint event plus the peer-visible frontier.  In the same
fixture, foreign user/session, a peer edit that changes the tail, and a
duplicate idempotency request must respectively deny/conflict/replay with no
second inverse event.  A small adapter-only law may verify tagged local versus
remote dispatch, but it cannot certify durable undo.

### Trusted Catalog Publisher Current-Selection Review (2026-09-08, Source Audit Only)

[`TrustedCatalogPublisher::publish_current`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:236>) performs its expected-current read only after it
holds the opened-root publication fence.  It requires the
`expectedCurrentSha256` member to be present even when its typed value is
`null`, canonical-decodes a present current pointer, and compares the raw
canonical pointer hash before it opens the requested generation.  A normal
publisher cannot ABA through this token: `publicationRevision` is part of the
canonical pointer bytes and the publisher increments it under the same fence,
so a P0 → P1 → same-generation P2 pointer has a different SHA.  Revision zero
is refused and `checked_add` refuses `u64::MAX`; the current-pointer and
receipt schemas permit the corresponding canonical 20-digit decimal range.

The candidate sequence is also correctly split for its stated purpose:
publication invokes the private pure
[`verify_selected`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:608>) rather than
[`load_selected`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:599>).  Consequently this path does not begin an artifact
assembly or register document codecs.  Before pointer replacement it rereads
every selected component, descriptor, and applicable browser-actor leaf from
the retained generation handle, checks length and digest against the original
verified bundle record, and then rereads/re-hashes the bundle.  With the
documented cooperating-writer ownership of that directory, this is a
meaningful final full-selection recheck; it is not a claim against arbitrary
same-UID directory mutation.

One concrete fail-closed issue remains:

1. [`replace_current`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:106>) returns `TrustedPublicationSync::Unconfirmed` after a
   visible rename whose directory synchronization could not be confirmed.
   `publish_current` converts that state into `Ok(Vec<u8>)` containing an
   ordinary receipt with `outcome: "replaced-unconfirmed"`
   ([lines 281–283](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:281>)).  The raw result type does not force a caller to inspect
   `outcome`; a future transport can therefore parse any successful return as
   publication success and activate/announce a not-yet-durable current.  This
   contradicts the required indeterminate/reconciliation boundary.

   Keep the visible replacement, but make the Rust result a closed typed
   `Durable(receipt) | Unconfirmed { request, observedCurrent }` outcome (or
   return a dedicated `AuthorityError::PublicationIndeterminate` after
   retaining sufficient reconciliation identity).  Only `Durable` may cross
   the command transport as a success receipt or trigger catalog activation.
   `Unconfirmed` must make a fresh opened-root `load_current` reconciliation
   decide old-versus-new before retry/activation.  Add a native fixture row
   that forces `sync_publication` unconfirmed and proves no codec registration,
   no activation callback, and no success wire response before reconciliation.

No current source defect was found in the strict `Option` presence check,
normal publisher ABA prevention, revision exhaustion, or selected-leaf
verification.  Native publication/CAS tests and binary transport wiring remain
unrun/out of scope for this review.

### Sequential Stdio→GIS Source Epoch (2026-09-08, Read-Only Implementation Packet)

#### Current materializer is fresh per package, not one compiler epoch

[`materializeTrustedStdioGisBundle`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8254>) currently loops `stdio` and `gis`, creates an empty
`build-<nonce>/<plugin>-target` for each, and calls
[`produceFreshComponentV1`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:496>) once per package.  That producer
requires an empty target root and sets `CARGO_INCREMENTAL=0`; its receipt binds
only staged component/core/descriptor/WIT bytes, not any source input digest.
The two components can therefore be byte-verified independently while having
been compiled from different dirty-workspace states.

This is more subtle than a simple stdio-output reuse.  The direct stdio
component leg uses its default `plugin-root` feature, whereas GIS declares
stdio with `default-features = false, features = ["full-artifact-catalog"]`
([GIS manifest](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:58>)).  With the present separate fresh targets, the GIS leg recompiles that
different stdio dependency unit; it does not consume the first target's stdio
component.  Replacing the two target roots with a shared Cargo target would
weaken the existing empty-target/fresh-output property and risks feature-unit
coupling.  Preserve the targets and bind their two actual build legs to one
source receipt instead.

#### Reusable facilities and their limits

* [`semanticOwnedInputFileSnapshot`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:4829>) is the right physical-file primitive.  It rejects unsafe
  coordinates and symlinked ancestry, opens read-only/no-follow, checks the
  opened handle against name and ancestor identities before/after reading, and
  returns a SHA-256, mode, size, and exact bytes.  A source-epoch wrapper must
  immediately zero those returned bytes and retain only the tuple.
* [`inspectRustModuleGraph`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:7056>) already follows manifest-declared lib roots and
  mounted `mod`/`#[path]` edges.  This is essential here: both plugin root
  files mount large artifact trees outside `📦️packages/🦀️rust`, so a package
  directory walk would omit genuine compiler inputs.
* The existing metadata consumers use `cargo metadata --no-deps` only (for
  example the dev capability lint).  That output is not a transitive closure
  and cannot be reused as the epoch authority.  `FreshComponentReceiptV1` and
  `freshRun` similarly record compiled outputs/process diagnostics, not source
  provenance.

The static graph is a bounded *preflight scope*, not an authoritative Cargo
feature evaluator: it deliberately does not resolve `cfg` selections,
`include_*`, build-script output, or target-specific resolution.  The epoch
must validate it against compiler-produced dep-info after each real leg rather
than treating a directory or manifest-name match as an exact closure.

#### Minimal source-owned receipt and procedure

Put the new owner beside `produceFreshComponentV1`, not in Hub:
`🧰️framework/…/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` owns the
exact `cargo rustc` argv and is reusable by every fresh component producer.
Hub merely requests one immutable two-leg receipt before it begins
materialization.

```ts
type FreshSourceEpochV1 = Readonly<{
  schema: "semio.plugin.fresh-source-epoch/v1";
  target: "wasm32-wasip2";
  profile: "wasm-release";
  legs: readonly [
    { package: "semio-s-plugin-stdio"; cargoArgsSha256: string; resolvedPackagesSha256: string },
    { package: "semio-s-plugin-gis"; cargoArgsSha256: string; resolvedPackagesSha256: string },
  ];
  workspace: { cargoToml: SourceFileV1; cargoLock: SourceFileV1; cargoConfig: SourceFileV1 | null };
  files: readonly SourceFileV1[]; // bytewise-sorted path, no bytes retained
  toolchain: { cargoVersion: string; rustcVersion: string; allowedBuildEnvSha256: string };
  digest: string;
}>;
type SourceFileV1 = Readonly<{ path: string; sha256: string; byteLength: number; mode: number }>;
```

`cargoArgsSha256` must encode the actual root-cdylib commands currently used
by the producer, including package, target, profile, crate type and enforced
environment—not a host-provided label.  `allowedBuildEnvSha256` is a closed
projection of Cargo/Rust inputs that can affect a build (`CARGO_*` target and
flags, `RUSTC*`, `RUSTFLAGS`/encoded flags and the isolated target path).  The
repo `.cargo/config.toml`, root `Cargo.toml`, and root `Cargo.lock` are source
inputs too: the config supplies the WASI rustflags and wrapper behavior, and
the lock anchors third-party package checksums.  Do not silently inherit an
unrecorded wrapper/flag/config change.

For each leg, run read-only full `cargo metadata --offline --locked
--format-version=1 --filter-platform wasm32-wasip2` at the existing workspace
root to map Cargo package IDs to their `manifest_path` and `source` records;
do **not** use `--no-deps`.  Metadata alone has no package-selection flag that
mirrors `cargo rustc -p` and a virtual-workspace invocation may unify unrelated
default features.  Select the leg instead with the matching, read-only root
command `cargo tree --offline --locked -p <actual-package> --target
wasm32-wasip2 --edges normal,build --prefix none --format {p}`.  Intersect
that exact package-ID tree with metadata's package map, retaining only local
(`source == null`) manifest paths under the repository.  The leg receipt
records the tree package IDs, feature-bearing actual build argv, and the
metadata map digest.  It must not hash or copy registry, vendor, target, or
generated trees: third-party resolution is represented by the root lock digest
plus resolved package IDs/checksums.

This avoids a synthetic probe manifest.  Such a probe would need a separate
lock file and could resolve a different external graph from the workspace
`cargo rustc`; it is not a sound shortcut for this receipt.  The tree is still
only the package selection boundary; compiler dep-info below remains the
authority for the exact local file leaves and build-script inputs.

`cargo tree`'s printable package projection is not by itself an unforgeable
Cargo package ID.  The adapter must map a tree row only when it identifies one
metadata package record; duplicate local name/version/source candidates are a
hard refusal.  It must additionally parse the existing producer-owned Cargo
JSON `compiler-artifact` records from each successful `freshRun` and require
their local package IDs to be a subset of that selected leg before accepting
dep-info.  This turns an ambiguous textual tree row into fail-closed
preflight, never a guessed path binding.

Build the static local preflight scope from those selected manifests plus all
repo Rust files through `inspectRustModuleGraph`; include each selected lib,
proc-macro and build-script root, its mounted modules, and explicit
`include_*`/WIT/non-Rust build inputs that the source scanner can prove.  Take
the no-follow tuple snapshot once before stdio.  Then:

1. build direct stdio in its existing empty private target and recheck every
   tuple;
2. build GIS in its separate empty private target and recheck the same tuple;
3. parse the `.d` dep-info files emitted in **each** private target.  Every
   repo-relative source leaf actually reported by the compiler, including
   build-script inputs, must already be in the preflight receipt with an exact
   tuple.  A newly discovered local leaf, a mismatched tuple, an unsafe path,
   or an unparseable/over-limit dep-info file rejects both staged components;
4. only then permit the existing descriptor/component/browser-actor receipts
   to enter `trustedBootstrapVerifyGeneration` and the later catalog
   publisher.  Extend `FreshComponentReceiptV1` with the same mandatory
   `sourceEpochDigest`; do not host-stamp it in Hub.

The digest is a length-prefixed binary encoding of schema, target, profile,
ordered leg commands/resolved package records, toolchain/environment records,
and byte-sorted source tuples.  JSON stringify or a package-name-only hash is
not sufficient.  The final bundle must require equal epoch digest on stdio and
GIS as well as the existing component/descriptor/protocol identities.

This provides a fail-closed cooperative-workspace qualification: an ordinary
source edit visible before, between, or after legs invalidates the candidate;
it neither locks nor copies collaborators' files.  No lock-free user-space
scheme can prove an instantaneous whole-tree snapshot against an adversarial
writer that changes a file A→B→A while rustc reads it.  Do not claim that
property.  The final compiled component/descriptor/actor byte checks remain
the publication authority; this receipt adds bounded source-drift detection.

#### Bounds, cancellation, and laws

Use no-follow capture only for paths relative to the repository and reject
source files above 4 MiB, more than 32,768 local files, more than 128 MiB
aggregate, dep-info above 8 MiB per target, or paths outside the selected
closure.  These limits prevent a manifest or macro path from turning a fresh
component receipt into a target/vendor copy.  Check cancellation between
metadata nodes, snapshots, each dep-info record and both legs; on any failure
zero transient read buffers and remove only the current ticket-owned target,
stage, probe, and diagnostic owners.  Never delete or write workspace source
or use Git/worktrees/locks.

Add `🧪️fixtures/🧾️fresh-source-epoch/{🧬️.schema.json,🔣️.json}` next to the
existing `🧊️fresh-staging` and `🧵️fresh-process` fixtures, with a source-only
`testFreshComponentSourceEpochV1` called from the existing trusted bootstrap
fixture runner.  Its closed rows should prove:

1. the two real feature/argv legs share one ordered epoch and component
   receipt digest;
2. a changed Rust/module/manifest/lock/config file before stdio, between legs,
   or after GIS rejects before generation publication;
3. a mounted `#[path]` leaf and an actual dep-info-only local leaf are both
   required; a target/registry/vendor entry is never copied into the receipt;
4. symlink/name replacement, oversized file/dep-info, feature/target/env
   substitution, cancellation, and unequal leg epoch digests all retain no
   staged generation/current pointer;
5. descriptor/component receipt equality without the matching epoch digest is
   refused.

The expensive native producer law can reuse the real two components and its
two isolated targets: retain the final dep-info list as ticket evidence and
require both staged receipts to carry the single epoch digest before the
existing generation/actor proof.  That would be genuine compiler evidence;
the language-neutral fixture is only the resolver/capture oracle.  No build
was run for this audit.

### Native Trusted-Catalog Publisher One-Shot (Source Audit, 2026-09-08)

#### Current reusable authority and the boundary to preserve

The native publisher is already the correct mutation owner:
[`TrustedCatalogPublisher::publish_current`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:242>) accepts only the existing closed,
4,096-byte JSON command, opens the configured data root through
[`TrustedCatalogDataRoot::open_server_owned`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:45>), acquires the
handle-owned publication fence, checks `expectedCurrentSha256` *under that
fence*, fully verifies the selected candidate, rechecks every selected leaf,
and only then replaces `current.json`.  Its `TrustedCatalogPublicationOutcome`
is already correctly split into `Durable(Vec<u8>)` and
`Unconfirmed(Vec<u8>)` at
[`trusted-catalog/🦀️.rs:235`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:235>).  The command and receipt are JSON, not Pack:
the command schema is
[`📬️command.schema.json`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📬️command.schema.json:3>).
Do not introduce a second framed/Pack representation merely for the binary
route.

The normal Hub entrypoint is unsuitable: it parses bind/mode, opens inherited
development bootstrap, loads the current catalog through
`configured_artifact_authority`, opens DB/directory/CAS, and starts
maintenance before the listener at
[`🚀️bin.rs:7736`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7736>).  In particular,
`configured_artifact_authority` calls `TrustedCatalogLoader::load_current`
([`🚀️bin.rs:438`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:438>), whose
`load_selected` branch registers document codecs.  That is catalog activation,
not a publication preflight.  `publish_current` uses the verifier-only
`TrustedCatalogLoader::verify_selected` instead, so it does not register a
codec or activate a server.  `NativeCodecProviderSetV1::linked()` is also the
right provider closure: its construction is a fixed compiled-in inventory and
does not invoke or publish factories
([`📇️native-openable-provider/🦀️.rs:21`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:21>)).

#### Minimal command route

Split the current `main` into an argument dispatch plus the existing server
boot body.  The only accepted one-shot argv is exactly:

```
os-hub trusted-catalog publish
```

Reject any extra/substituted argument before parsing `OS_HUB_MODE`, a bind,
bootstrap transport, DB configuration, or a catalog.  Under
`native-artifact-execution`, dispatch to a private
`run_trusted_catalog_publish_once`; without that feature, refuse on stderr
before consuming a command.  That helper should:

1. require `OS_HUB_DATA` to be present, nonempty, absolute and free of
   `..`/drive-relative components; it must not use the server default or
   `canonicalize` it.  `open_server_owned` already walks the supplied root
   descriptor-relative and no-follow on Unix and Windows
   ([`opened-root/🦀️.rs:245`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:245>),
   [`opened-root/🦀️.rs:465`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:465>)); a
   path-based existence check or a fallback data root would reintroduce a
   different authority.
2. read exactly one EOF-terminated command from stdin with a `4_097` byte
   sentinel, refuse empty/overlong input before provider construction, and
   pass its bytes unchanged to `publish_current`.  The live TypeScript caller
   must write one canonical schema-v1 JSON value and close stdin.  A bounded
   read deadline is useful for a hostile caller that never closes stdin, but
   it must occur before the catalog root/fence is acquired.
3. use a private silent `AuthorityOperationControl`: wall-clock `now_ms`, a
   fixed finite one-shot deadline, `AuthorityLimits::maximum()`, and a no-op
   `report`.  Do **not** reuse `StartupCatalogControl`, whose `report` writes
   catalog progress to stderr at [`🚀️bin.rs:214`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:214>), and do not start a Tokio task.
   Parent cancellation kills this one owned child; the OS closes its open
   fence.  There is no detached operation to resume.
4. construct only `NativeCodecProviderSetV1::linked()` and call
   `TrustedCatalogPublisher::publish_current(data_root, stdin_bytes, &providers,
   &context)`.  It must not call `configured_artifact_authority`,
   `connect_db`, `connect_directory`, `connect_artifact_cas`, bootstrap, or
   any Router constructor.
5. write and flush one bounded receipt JSON byte sequence to stdout, and write
   no progress, diagnostics, or `println!` data there.  Invalid command,
   stale token, verifier failure, timeout, and I/O error emit no stdout and
   terminate nonzero; bounded diagnostic text is stderr-only.  Add a receipt
   JSON schema/parser alongside the existing command schema so the TS caller
   validates exact `schema`, request/profile/generation/bundle identity,
   revision, `currentSha256`, and outcome before treating any bytes as a
   receipt.

This retains the existing server-owned provider and root authority, adds no
client-supplied path/provider selector, and avoids both server and codec
activation.

#### Durable, unconfirmed, and cancellation cutover

There is one important integration constraint with the proposed nonzero
`Unconfirmed` exit.  On Unix, `replace_current` synchronizes the directory and
returns `Durable`; on Windows it deliberately returns `Unconfirmed` after a
visible replacement because directory synchronization is unavailable
([`opened-root/🦀️.rs:348`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:348>),
[`opened-root/🦀️.rs:605`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:605>)).  Therefore the command has already
mutated `current.json` when it returns `Unconfirmed`; a nonzero exit cannot
mean “no mutation.”

If the binary uses a designated nonzero exit for `Unconfirmed`, it must first
write **and flush** the exact typed receipt, let `publish_current` return (so
the opened-root owner and fence are dropped), and only then leave with that
status.  Do not call `process::exit` while a publication owner is live.  More
importantly, the TS caller cannot use a generic “nonzero means throw/discard
stdout” process helper: it must collect a bounded stdout independently of
status and parse the receipt on the designated status.  Otherwise it cannot
distinguish a visible-but-unconfirmed replacement from a process killed after
rename and before reply.

For a killed/timed-out/EPIPE/malformed/over-limit/missing receipt, classify
the operation as **Indeterminate**, retain no claim of durable publication,
and do not blindly submit the original command again.  It may have renamed
the pointer after the command’s last pre-rename checkpoint; a retry with the
old expected token rightly conflicts.  A later verified Hub boot or a future
explicit verifier-only reconciliation command may settle it.  A raw TS
path-read may diagnose the pointer under the documented cooperating-writer
boundary, but is not a durable confirmation.  This is the safe post-rename
cutover; it needs no rollback and must not erase the already-published
candidate.

The existing [`runExactCargoLawProcess`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2016>) cannot be the live
caller because it fixes stdin to `ignore`.  Add a narrow Hub-script process
owner next to [`hubBinaryPath`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:753>): spawn that exact built binary with the literal argv,
`stdio: ["pipe", "pipe", "pipe"]`, an explicit absolute `OS_HUB_DATA`, a
one-shot deadline/cancellation owner, and separate bounded stdout/stderr
collectors.  It must `stdin.end(commandBytes)`, zero the local command/output
buffers after parsing, and validate one complete receipt rather than treating
all nonzero exits as equivalent.  This is a new transport owner, not a reason
to loosen the generic Cargo-law runner.

#### Existing physical lock evidence and remaining one-shot laws

The reusable cross-process facility now exists in
[`trusted_publication_owner_process_crash_releases_exact_lock`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:649>): it recursively starts the
same test executable in a holder mode, acquires the actual
`TrustedCatalogDataRoot` fence, proves a separate process is contended, kills
the holder, and then reacquires the same fence with the original pointer
unchanged.  It exercises `flock`/`LockFileEx`, not a mock or stale-lock age
heuristic.  It qualifies OS lock release, but not the command’s stdin/stdout
or its post-rename result classification.

Add a dedicated native binary law using the built `os-hub` process and the
existing publication fixture, with these bounded rows:

* malformed, empty, 4,097-byte and trailing-non-JSON stdin produce no stdout,
  no current pointer and no codec registration/server listener;
* valid initial CAS and stale expected token prove the route uses the supplied
  `OS_HUB_DATA`, exact compiled provider closure, and one clean durable receipt
  (stdout within the receipt ceiling, stderr bounded);
* cancellation before lock/verification leaves current unchanged and a fresh
  process acquires the real lock; cancellation after a pause immediately after
  replace is **Indeterminate**, not “failed/no mutation,” and a second command
  cannot bypass the current-token CAS;
* platform-unconfirmed replacement returns the exact `replaced-unconfirmed`
  receipt and TS preserves it for reconciliation even with its designated
  nonzero status; an invalid/missing response is never reported durable.

The opened-root crash law is the correct lower lock oracle for the second row;
the new binary law is needed to bind it to actual CLI routing and stdout
discipline.  No Cargo build or source modification was performed for this
audit.

### Implemented One-Shot Publisher and Pointer Closure Audit (Source Only, 2026-09-08)

#### Result

The current implementation satisfies the intended one-shot ownership cutover.
[`📤️command/🦀️.rs`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs:7>) admits only the literal two-argument verb, and
[`🚀️bin.rs`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7737>) dispatches it before normal Hub boot.  The route reads stdin before constructing
the data root, provider closure, operation context, or publication fence.  Its
4,097-byte sentinel correctly distinguishes a complete 4,096-byte command
from an over-bound stream; the five-second reader timeout therefore cannot
leave a catalog/lock owner behind.  A reader thread may remain blocked until
process exit after that timeout, but it owns only stdin and a dropped one-shot
sender; it has neither data-root nor publication authority.  This is not a
durable-owner leak.

[`TrustedCatalogPublisher::publish_current`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:242>) returns its typed outcome only after
`replace_current` has completed.  Consequently the CLI writes/flushed its
receipt only after the publisher-local opened-root/fence owner has been
dropped.  Its raw JSON receipt is capped at 4,095 bytes before the route adds
the newline, so the exact transport maximum is 4,096 bytes.  A durable result
returns success; a visible-but-unsynchronized result writes that exact receipt
then returns an error.  The latter is deliberately a post-rename condition,
not a rollback signal.

The live TypeScript owner now makes the required transport distinction at
[`publishTrustedBootstrapCurrent`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:8997>): it rejects a failed/signal/overflow or any exit outside
`0|1` *before* parsing a receipt; accepts `replaced-unconfirmed` only with
status `1`; and accepts `durable` only with status `0`.  This repairs the
earlier receipt-prefix classification flaw: a killed child that happened to
write a valid-looking `replaced-unconfirmed` prefix is now indeterminate, not
a trusted unconfirmed result.  The `close` callback clears its timer and closes
both evidence descriptors, which also waits for the child stdio streams; stdout
is retained to 4,096 bytes and stderr to 65,536 bytes.  The child has no
expected descendants, so direct process termination is sufficient for this
private one-shot binary rather than a general process-tree facility.

The current-token snapshot remains correctly before the costly cold proof at
[`📜️script.ts:9137`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:9137>), while the native publisher rereads it under the OS fence.  Candidate staging uses a separate
`validation/.../candidate-data` root; no TypeScript code writes the live
`dataRoot/trusted-catalog/current.json` after the split.  The final parent
read after a durable receipt verifies the exact returned pointer digest, so a
concurrent later publication cannot silently activate the candidate plan.

No current implementation defect was found in stdin lifetime, post-rename
return ordering, child reaping, bounded output, or the repaired status/receipt
classification.  This remains a source review: it makes no claim that the
new native command or its subprocess route has executed.

#### Remaining Pointer-Shape Closure

Two actual test-generated catalog pointers outside the trusted-catalog module
still omit the now-required canonical `publicationRevision`:

| Priority | Writer | Effect | Repair |
| --- | --- | --- | --- |
| P1 | [`🚀️bin.rs:8193`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8193>) `native_openable_stdio_bundle` | The generated current pointer is passed into `configured_artifact_authority`; `TrustedCatalogCurrentPointer::decode` now rejects it before its intended native-openable assertion. | Add `"publicationRevision":"1"` in canonical serde field order. |
| P1 | [`🚀️bin.rs:8615`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8615>) `checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog` | Its copied process data root will fail a real selected-current load before the checkpoint fixture can test its intended behavior. | Add the same canonical revision field. |

[`🚀️bin.rs:8070`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8070>) intentionally writes `{}` solely to prove that an existing malformed
pointer still forces the no-provider fail-closed branch.  It is not a valid
pointer writer and should remain malformed.  The opened-root neutral corpus’s
three-field `current` object is currently unused by any pointer decoder or
writer (the Rust law reads only `relativePaths`); it is schema terminology
drift, not an executable bypass.  Update that fixture/schema to include
`publicationRevision` when aligning the closed current-pointer representation,
but do not mistake it for another runtime writer.

#### Minimum TS-Only Subprocess Laws

Keep these as a private process-transport helper test: production still pins
the executable path to the compiled Hub binary, while the test supplies
`process.execPath` plus a tiny temporary JavaScript child through the *same*
`spawn`/stdio/close owner.  This qualifies TypeScript transport/error handling
without asserting that a Node fixture is the native publisher.

1. Exact canonical durable receipt + exit `0` returns only after `close`, with
   bounded stdout/stderr evidence.
2. Exact canonical `replaced-unconfirmed` receipt + exit `1` yields only
   `TrustedBootstrapPublicationUnconfirmed`; the same receipt + exit `0` or
   `2` is indeterminate.
3. A valid unconfirmed prefix followed by stdout overrun, signal, or injected
   stdin/spawn failure is indeterminate.  This pins the repaired ordering.
4. Empty/malformed/noncanonical/identity-mismatched receipt at exit `0`, and
   durable receipt at exit `1`, are indeterminate.
5. A child that leaves stdin unread or never exits is killed by the existing
   owner deadline; after `close`, no stdout receipt is accepted and a
   subsequent fixture child can run.  This proves timeout/reap behavior
   without a Cargo build.

Pair these controlled process laws with the existing native opened-root
cross-process crash/release law at
[`opened-root/🦀️.rs:649`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:649>).  A later actual `os-hub` subprocess law remains necessary to
qualify argument routing, `OS_HUB_DATA`, compiled providers, and post-rename
native output; the TS fixture must not claim those native facts.

### MCP Inference Cancellation: Retained `Running` Worker Packet (Source Only, 2026-09-08)

#### Current P0 and exact source trace

The public protocol is already capable of returning an early, owner-private
`running` receipt.  Both Hub and MCP declare `accepted`, `running`,
`succeeded`, `failed`, and `cancelled`; neither receipt adds private bytes.
There is no schema change required just to expose `Running`.

The problem is implementation ownership, not wire vocabulary:

1. [`submit_gis_map_job`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2450>) accepts and claims under its document gate at lines 2457–2480, but calls the synchronous GIS executor inline at lines 2491–2519.  Its HTTP response therefore cannot return until the inference is terminal.
2. The MCP handler only mints its job handle *after* that blocking call returns at [`inference_submit_handler:1285–1308`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:1285>).  `inference_cancel` consequently has no handle while a submit is computing.  The workspace explicitly describes the call as inline at [`workspace:1987–2001`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs:1987>), and the remote driver blocks its private runtime on the HTTP response at [`remote:1018–1025`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs:1018>).  This is the direct MCP cancellation P0.
3. [`HubInferenceRuntimeV1`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2016>) retains only a `job_id → control` map.  `close` at lines 2042–2045 drains only the approval committer, not an inference task.  It cannot prove that compute has stopped before the ledger/database is released.
4. There is an independent concrete duplicate-controller race today.  `retain_operation` unconditionally overwrites `operations[job_id]` at lines 2066–2073.  A second same-idempotency submission can then observe `start == None` and call `release_operation` at lines 2482–2484, deleting the first worker's control.  A subsequent cancel no longer reaches that first worker.  A retained implementation must replace this map rather than wrap it with a second unsynchronised task map.

The actual GIS call is appropriate to place behind an owned blocking join, but
not to detach.  [`infer_gis_map_controlled`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:247>) checks at admission, after decode, during every bounded geometry value, and before return (lines 253–277).  Hub forwards each callback through
`InferenceOperationControlV1::checkpoint` at
[`runtime:2112–2119`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2112>).
Thus cancellation is observable at genuine GIS checkpoints, but only if the
same `Arc<InferenceOperationControlV1>` remains owned until the actual
`spawn_blocking` join completes.  `Notify` in the control only wakes async
waiters; it does not preempt synchronous GIS code.

#### Minimal retained-runtime seam

Keep the existing HTTP routes and `InferenceJobReceiptDtoV1` unchanged.  Add
a private job supervisor to `HubInferenceRuntimeV1`, replacing `operations`:

```rust
struct InferenceRunOwnerV1 {
    key: InferenceRunKeyV1,       // job_id + immutable identity digest + run_epoch
    identity: InferenceIdentityV1,
    scope: DocumentScope,
    base: InferenceMapBaseV1,     // zeroizes with the owner after terminal handling
    claim: InferenceRunClaimV1,
    control: Arc<InferenceOperationControlV1>,
    gate: Arc<tokio::sync::Mutex<()>>,
    directory: Arc<HubDirectories>,
    rebootstrap: Arc<VerifiedRebootstrapSource>,
    slot: tokio::sync::OwnedSemaphorePermit,
    // The supervisor, not the request stack, owns both join handles.
}
struct InferenceRunSupervisorV1 {
    closing: bool,
    runs: BTreeMap<InferenceRunKeyV1, InferenceRunTaskV1>,
    slots: Arc<tokio::sync::Semaphore>, // exactly OPERATION_CAPACITY
}
```

`InferenceRunTaskV1` must retain the outer async `JoinHandle` and the exact
`InferenceRunOwnerV1`; the outer task awaits its own `spawn_blocking` handle.
Dropping either handle is prohibited because Tokio detaches a dropped join
handle.  A completion notification may mark the exact key finished, but only
the supervisor's `reap_finished` / `close` removes and joins that exact key.
It must compare `job_id`, identity digest, and `run_epoch` before removal, so
an old completion cannot remove a later recovery claim.

The narrow route transition is:

1. Retain today's initial authenticated-session, descriptor, active-pair,
   frozen-identity and Author checks under the existing document gate
   ([`runtime:2451–2476`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2451>)).  Do **not** retain a session/membership mutex across compute; revocation and cancel must acquire the gate.
2. Admit one supervisor reservation keyed by the deterministic accepted job
   identity before returning any receipt.  It has to distinguish `First`,
   `SameRunning`, and `Closing/Capacity`; a duplicate must observe the
   existing owner/page, never replace its controller.  The reservation is a
   `Drop` guard until the exact run is installed.
3. In the same initial gate turn, call the existing durable
   `ledger.accept`, then `ledger.start`; convert `First + Some(claim)` to an
   installed owner before returning.  A duplicate `None` claim releases only
   its *own* reservation and returns `owner_page_receipt`.  If admission or
   install fails after `accept`, terminalize the exact accepted/run epoch
   before releasing the reservation; never return `Running` for an unowned
   ledger row.  The supervisor's keyed first-admission cell needs a bounded
   completion/waiter result so a simultaneous duplicate neither starts a
   second run nor spuriously gets `Capacity` while the first is writing its
   receipt.
4. The installed outer task invokes `spawn_blocking` with only the frozen
   base, identity, claim, control and ledger/progress callback.  It owns and
   awaits that join.  After it joins, it reacquires the per-document gate,
   performs the existing fresh Author check and `map_base`/`compare_frozen`
   recheck ([`runtime:2499–2506`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2499>)), and only then calls `succeed` for its exact
   `run_epoch`.  A cancelled, revoked, stale, expired or late result is
   terminalized/wiped and cannot offer a proposal.  The request token and
   lexical `AuthSessionRecord` are not retained; the immutable identity plus
   fresh directory revalidation are the authority.
5. `cancel_gis_map_job` keeps its current authenticated owner recheck, then
   signals the supervisor's exact owner and durably calls `request_cancel`.
   The ledger's immediate cancellation is allowed, but it does **not** release
   the run owner/slot until the blocking join has observed that it cannot
   publish.  Foreign user, session, scope, or authorization generation never
   finds/signal-matches an owner.

The separate `document_write` authority remains approval-only.  It must not
be captured by inference compute; retaining it would self-deadlock the
approval path and turn a bounded GIS calculation into a document-write lock.

#### Clock and claim requirement

The worker cannot reuse `InferenceRouteContextV1::now_ms`: it is a single
submit timestamp, currently used for every progress and terminal write at
[`runtime:2491–2519`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🏃️runtime/🦀️.rs:2491>).  A retained job lasts up to 120 seconds, whereas a ledger
claim lasts 30 seconds ([`schema:18–19`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🧬️schema/🦀️.rs:18>).
Moreover, `ledger.progress` currently validates the lease but does not renew
it ([`sqlite:327–358`](</Users/ueli/Documents/semio/🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:327>).  After expiry a retry can claim a new epoch while a live old
worker still holds the prior control.

Add a process-owned clock injected into the runtime for native tests, and a
ledger `renew_progress`/`heartbeat` transaction that, after exact
`job_id + identity + run_epoch + running + not-cancelled` validation, advances
`lease_expires_at` to `min(now + CLAIM_LEASE_MAX_MS, job.expires_at)` together
with an optional bounded progress append.  Call it at every actual GIS
checkpoint.  Require controlled GIS to checkpoint before the claim deadline;
if no heartbeat can be made before expiry, terminalize the exact epoch rather
than let a runner continue with a stealable lease.  A terminal `succeed` must
also reject an expired lease, not merely a changed epoch, unless it atomically
renews/consumes that lease in the same transaction.

#### Close, cancellation and error boundary

`HubInferenceRuntimeV1::close` must first set `supervisor.closing` under the
same mutex used by admission, signal every exact control, and then join every
outer worker (which joins its blocking child) before invoking
`committer.close`.  It must not copy `AdminOperationTaskOwner`'s abort
fallback at [`🚀️bin.rs:6625–6667`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6625>): aborting an outer task detaches its
`spawn_blocking` child and can drop the only durable finalizer.  There is no
safe `abort` substitute here.  A non-terminating worker must keep the runtime
closed and its owner retained; it is a shutdown error, not permission to drop
DB/directory owners.

The normal main path already awaits inference close before artifact maintenance
at [`🚀️bin.rs:7906–7916`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7906>).  Extend the test shutdown helper
[`stop_recovery_server:10972–10988`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:10972>) to call the same runtime close before
dropping state and asserting database uniqueness.  That makes retained-worker
leaks physical rather than merely task-map assertions.

#### Schema-first fixture and executable laws

Keep the existing ledger-only corpus
[`🗺️gis-inference-job-v1`](</Users/ueli/Documents/semio/🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🧬️.schema.json>) focused on durable transitions.  Add a sibling closed fixture,
`🌎️hub/🧪️fixtures/🗺️gis-inference-retained-runtime-v1/{🧬️.schema.json,🔣️.json}`,
covering these bounded rows:

| Row | Required physical assertions |
| --- | --- |
| `accepted-running-before-release` | Production POST returns `running`, `proposalState:none`, cursor after `accepted,running`; the actual worker is paused at a controlled GIS checkpoint and the MCP handler can mint a handle. |
| `owner-cancel-during-checkpoint` | A production cancel route from the same authenticated session writes `cancel-requested/cancelled`, signals the exact control, then released GIS work cannot offer result/proposal; exact owner, slot, task and private bytes retire. |
| `revocation-before-terminal-recheck` | Pause after claim, demote/revoke through the real directory command path, resume work, then fresh terminal Author recheck refuses with zero `succeeded`/outbox rows. |
| `same-request-one-owner` | Two same request/id/identity submits yield one job/epoch/worker; the second cannot overwrite or remove the first controller; cancel reaches the first. |
| `late-result-after-cancel` | Cancel terminalizes first, then a released worker returns a normal GIS result; epoch/state fence prevents `succeed` and wipes both private byte buffers. |
| `claim-heartbeat-no-steal` | Fake runtime clock crosses an old 30-second claim while paused/released checkpoints renew it; an old epoch cannot publish after a later claim. |
| `admission-or-spawn-refusal` | No `Running` receipt, worker, slot or silently stranded accepted row remains after the exact reservation/installation failure. |
| `runtime-close-drains-worker` | Pause worker, call real runtime close, prove new submit is refused, cancel signal reaches GIS, both joins finish before committer/database teardown, and no task abort occurs. |
| `foreign-cancel-inert` | Different user/session/generation/scope receives only denial and cannot signal the controller or change its ledger event stream. |

Use the existing real Hub GIS fixture and route harness at
[`🚀️bin.rs:8927–9050`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8927>) for the native law.  It already builds a trusted GIS profile,
announces a real Map descriptor and publishes a real checkpoint; it is the
right parent for an `InferenceRuntimeTestGate` wired at the Hub callback
boundary, not a fake GIS implementation.  The controlled GIS codec's existing
literal interruption law at
[`native-codecs tests:34–98`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧪️tests/🦀️.rs:34>) remains the independent
third-party/native algorithm oracle.  Add a focused MCP integration law only
after the Hub law: submit must first receive the real `running` receipt and
mint its handle, then a second ordinary `inference_cancel` call uses that
handle.  It must not claim a real GIS/MCP cancellation proof from a scripted
transport response alone.

No source modification or Cargo execution was performed for this packet.

### Windows Opened-Root Current Publication: Rooted Replacement, Durable Outcome, And Safe Reconciliation (2026-09-08)

#### Verdict

The present Windows replacement is correctly *descriptor rooted* and should
remain the only replacement primitive.  It is not a durable-publication
primitive yet.  Keeping its result as `Unconfirmed` is therefore correct;
neither `MoveFileExW` nor an unqualified `FlushFileBuffers` call justifies
changing it to `Durable`.

There is, however, a safe zero-touch way to make a visibly replaced selection
usable: a separate `visible-verified` outcome may be returned only after a
new, side-effect-free, opened-root read of `current.json` verifies the entire
selected closure.  It means “this process can now start and will reverify at
startup”, never “the replacement was crash durable”.  A later crash may leave
the old pointer, the new pointer, or no valid pointer; ordinary startup must
accept only whichever exact current pointer `load_current` can fully verify.

#### Current source evidence

* [`opened-root:63–75`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:63>) acquires a real retained
  `FileFence` on a `.publication.lock` opened beneath the already-opened
  `trusted-catalog` directory.  The existing Windows implementation uses a
  one-byte exclusive, fail-immediately `LockFileEx`; Windows releases such a
  lock on process termination, though availability may lag.  The existing
  process-kill law deliberately waits for the next acquisition
  ([`opened-root:648–703`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:648>)).  This is a sound
  cooperating-writer fence, not a stale-lock-age scheme.
* The temporary pointer is create-new, written and file-synced before the
  irreversible name replacement ([`opened-root:106–124`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:106>)).
  The Windows walk uses `NtCreateFile` relative to each retained parent with
  `OBJ_DONT_REPARSE` and `FILE_OPEN_REPARSE_POINT`, rejects a reparse or type
  mismatch, and gives an owned temporary `GENERIC_WRITE | DELETE`
  ([`opened-root:491–565`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:491>)).
* [`opened-root:580–607`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:580>) calls `SetFileInformationByHandle(...,
  FileRenameInfo, ...)` on that exact temporary handle.  Its `flags: 1` is
  the legacy `ReplaceIfExists = TRUE`; `RootDirectory` is the retained
  catalog directory and the fixed relative UTF-16 `current.json` has the
  correct 24-byte non-NUL name length.  Microsoft explicitly documents both
  `RootDirectory` resolution for a relative rename and that a terminator is
  unnecessary in [`FILE_RENAME_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_rename_info).
  Therefore no path-based replacement is necessary or desirable.
* The source correctly returns `Unconfirmed` for Windows, and the current
  physical owner law asserts exactly that
  ([`opened-root:707–734`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:707>)).
  The existing chunk-CAS `MoveFileExW` helper is unsuitable: it reconstructs
  two pathname strings and its Windows parent-sync is a no-op
  ([`chunk-cas:841–864`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:841>)).  Do not reuse it.

#### What Windows actually supports here

`SetFileInformationByHandle` accepts `FileRenameInfo` with an appropriate
file handle and is supported from Vista.  The present source has the important
source-handle `DELETE` access and broad delete sharing.  The relevant API does
not promise uniform behaviour/durability across every underlying driver;
Microsoft expressly qualifies such behaviour by OS and driver in
[`SetFileInformationByHandle`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileinformationbyhandle).

`FlushFileBuffers` is usable only on a handle with `GENERIC_WRITE`; the
temporary/current file already has that right.  A second flush *after* its
rename is technically feasible without admin or a pathname and can be treated
as best effort.  It is **not** evidence that the parent-directory entry is
crash durable.  The official contract says it flushes buffered information
for the specified file; it says flushing an entire volume requires an
administrator volume handle
([`FlushFileBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers)).
Opening a second rooted directory handle with `GENERIC_WRITE` would not repair
that evidentiary gap, would introduce ACL failure, and would still not provide
a documented directory-rename durability contract.  Do not use a volume
handle, `ReplaceFileW`, `MoveFileExW`, `FileRenameInfoEx`, or an unsafe
stale-lock deletion as a workaround.  `FileRenameInfoEx` is neither needed
for exact replacement nor a cross-version durability feature.

The lock itself is correctly compatible with the source: `LockFileEx` needs
read or write access, supports the immediate exclusive request used here, and
the OS releases a process's outstanding file locks on termination
([`LockFileEx`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex)).
That proves fencing/recovery, not media persistence.

#### Required post-replace boundary

Do **not** make `sync_publication` fallible after `SetFileInformationByHandle`
without first splitting the pre- and post-rename states.  In the current
wrapper, any `Err` triggers `remove_owned`
([`opened-root:114–124`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🦀️.rs:114>)); after a successful
rename, that exact handle names live `current.json`, so treating a later flush
failure as ordinary error would delete the published pointer.  This is the
main concrete ordering hazard for the Windows repair.

The minimal API separates the irreversible replace result from the
caller-visible receipt, for example:

```rust
enum TrustedPublicationSync {
    Durable,
    Unconfirmed,
}
enum TrustedCatalogPublicationOutcome {
    Durable,
    VisibleVerified,
    Unconfirmed,
}
```

`replace_current` owns cleanup only until rename success.  Once rename
succeeds, every later failure (post-rename flush, re-open/verification,
receipt delivery, cancellation) leaves the current pointer in place and is
reported as an `Unconfirmed` outcome; it never rolls the name back or deletes
it.  A post-rename `FlushFileBuffers` may be attempted, but its success still
maps to `Unconfirmed` on Windows.  Only the independently successful
verifier below upgrades that caller-visible result to `VisibleVerified`.

Before allowing a live TS caller to start Hub after that result, run a new
private **verifier-only** pass while the same `TrustedCatalogPublicationOwner`
still retains the fence:

1. reopen `current.json` with `owner.open_current()`, read it boundedly and
   require byte-for-byte equality with the just-produced canonical pointer;
2. decode its generation/profile/bundle digest/revision; open that exact
   generation from the same owner, hash the bundle and rerun
   `TrustedCatalogLoader::verify_selected` plus the existing final selected
   component/descriptor/browser-actor leaf loop;
3. return `VisibleVerified` only if all of that succeeds.  It must not call
   `load_selected`, `register_document_codecs_in_assembly`, or otherwise
   install a codec.  The publication path already demonstrates the safe
   non-registering verifier at [`trusted-catalog:266–283`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:266>).

The consumer may then start Hub, whose independent startup path reloads and
registers only a fully verified current selection through
[`TrustedCatalogLoader::load_current`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:570>) and
[`configured_artifact_authority`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:438>).  If a process or power failure leaves a
truncated pointer, substituted leaf, invalid reparse path, missing generation,
or incompatible provider, that fresh load fails closed and activation does
not occur.  This is automatic safe recovery, not a retroactive `Durable`
receipt.

#### Windows qualification laws

Add Windows-only physical rows alongside the existing opened-root fixture;
retain the Unix durable assertion unchanged.

| Law | Required assertion |
| --- | --- |
| `windows-rooted-replace-visible-verified` | Exact `FileRenameInfo` replacement is reopened only through the retained owner; bytes, canonical pointer hash/revision, bundle and every selected leaf verify; outcome is `VisibleVerified`, never `Durable`; no codec registry is touched by verification. |
| `windows-post-rename-sync-failure-keeps-current` | A test-only post-rename sync result failure/uncertainty leaves the exact new `current.json` readable and returns unconfirmed; it never enters `remove_owned` or reports an ordinary pre-publication error. |
| `windows-current-substitution-refuses-startup` | After visible replacement, replace the pointer, generation leaf, or provider binding through the hostile opened-root fixture; the verifier and then `configured_artifact_authority` refuse with no catalog/codec activation. |
| `windows-publisher-crash-releases-fence` | Preserve the existing child-kill lock law, then acquire the same rooted fence and verify whichever exact pointer remains.  This must not assert a particular new/old result after kill. |
| `windows-sharing-denial-preserves-old-current` | A conflicting external non-delete-sharing handle makes rooted rename fail before the irreversible transition; old current stays byte-identical and the temporary is removed. |

Run these on a native Windows worker against the supported local filesystem.
A force-killed process validates lock release and recovery classification; it
does **not** simulate power loss or prove physical directory persistence.
That final property requires an external Windows/filesystem qualification
(NTFS/ReFS and any supported network provider under an actual power/fault
model).  Until such evidence exists, no code path may label the Windows
publication `Durable`.

No source, Cargo, or native test was run for this audit.

## DAG and Flow Host Retirement Cursor Audit (2026-09-08, source-only)

The new DAG cursor is structurally preferable to a recursive destructor:
[`DagPayloadRetirement`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2569>) owns a `LinkedList` of exact
payload roots, and [`FlowHostRetirement`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2177>) retains that DAG cursor until its
terminal witness before dropping it.  The current Flow constructor also
correctly transfers the Flow-level `ghost_node` into the DAG cursor rather
than dropping it directly
([`host:2142`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2142>)).  No recursive normal-path stack walk was found in
that payload path.

### P0: bounded positive credits cannot close the authored DAG fixture

This is a source-visible contradiction between the new native test and the
cursor contract.  `credit_backing` correctly does **not** claim a partial
release: it returns `Blocked` while a contiguous `Vec` allocation exceeds the
given byte grant
([`dag:2643`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2643>)).  `Text` becomes a `Vec<u8>` and does the same
([`dag:2666`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2666>)).

But the new fixture makes a 1,600-character `高` note—4,800 UTF-8 bytes—and
its law requires every positive credit in `{1, 64, 4096}` to progress;
[`close_dag`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🦀️.rs:95>) panics on `Blocked`, while the row invokes all three
values at [`tests:122`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🦀️.rs:122>).  Hence this native law cannot pass as
written.  More importantly, normal component tests and the old
`FlowHostRetirement::close_step` wrapper use a 4,096-byte page
([`component:5651`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs:5651>), so the production close can retain this exact
host forever when clients continue to offer ordinary pages.

There are two valid contracts; choose one explicitly before a runtime claim:

1. Make close data page-owned at admission (or transfer a fixed-size page
   cursor) so every positive permitted ABI page can retire physical storage;
   a contiguous allocation may not report partial bytes it still owns.
2. Preserve whole-allocation admission, let `Blocked` be a retained result,
   and require the scheduler to issue a later grant at least the exact backing
   capacity.  The fixture must then assert bounded retention and a successful
   large grant, not progress at `1/64/4096`.

The first choice matches the existing small-page law and interactive close
semantics; the second is simpler but needs a documented maximum allocation and
an actual scheduler escalation path.  Merely changing the test to tolerate
`Blocked` leaves close liveness unresolved.

### P0: terminal path releases uncharged backing allocations

When a `Vec` does fit the grant, the payload arms pop one value and requeue the
same allocation with `remaining_backing_bytes: 0`; its final destructor frees
the vector capacity while returning `released_bytes: 0` (for example
[`DslValues`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:2687>), with identical arms for ports,
fixture nodes/edges, event vectors, ids, and optional ids).  This is not a
false partial credit; it is a hidden **uncredited** physical drop.  The
existing fixture gets its expected count from text bytes and does not contain
a high-capacity vector, so it cannot detect the omission.

The same boundary exists for host maps/sets.  `DagHostRetirement` removes one
entry at a time but never retires `HashMap`/`HashSet` bucket allocations
([`dag:3264`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3264>)).  Its terminal `ManuallyDrop::drop` then frees
those empty tables at once ([`dag:3343`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3343>)).  The graph engine has the same
entry-by-entry drop pattern for empty B-tree/vector backing
([`board:645`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs:645>)).  Thus `DagHostRetirement::Complete` is correctly a
nonopaque owner witness, but it is not evidence that every backing allocation
was released within a reported caller byte grant.

Require a private test-only physical-backing census and add these rows:

- one high-capacity `Vec<u8>` and one `Vec<DagNodeSpec>` with small length:
  no positive `released_bytes` before the actual allocation is relinquished;
  the eventual charge is exact;
- one capacity-heavy host map/set: completion cannot hide a table drop after a
  smaller byte grant; and
- a normal Flow component close at a 4,096-byte credit with an over-page DAG
  payload, followed by cancellation/valid retry, proving the exact cursor
  remains owned until its eventual terminal result.

### P0: vector icon scenes escape the close owner permanently

For a vector icon, `IconPaintCache::close_page` reserves an opaque token,
moves the `Scene` into the global registry, clears the local slot, and permits
the DAG host to become terminal
([`directed:914`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:914>)).  The recipient is not a retained
retirement cursor: `OpaqueSceneRetirementRegistry` only reserves and publishes
into `ManuallyDrop<Scene>` slots; it has no release/advance operation, and its
`next` counter only increases until a process-lifetime capacity of 1,024
([`canvas:615`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs:615>)).  Although `Scene` already has a one-command
retirement primitive, no code drives it here.

Consequently a Flow close can report its nonopaque terminal condition while a
potentially large scene survives permanently in a global `ManuallyDrop` slot;
after 1,024 such closes new icon retirement faults.  This is a real ownership
transfer, not a double-free, but it lacks an eventual terminal receipt and is
outside the caller's byte budget.  Either make that registry a bounded,
driven owner whose exact token is included in the Flow close terminal fence,
or explicitly define it as a separately admitted process-lifetime resource
and exclude it from any claim that host close reclaims the icon scene.  A
vector-icon law must retain the exact token through cancellation, prove host
close cannot claim full terminal before it drains, and prove released slots can
be reused rather than saturating monotonically.

### Drop/error ordering

The current cursor does retain the DAG owner on ordinary `Blocked` and exposes
`Failed` without issuing a Flow terminal result
([`host:2183`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2183>)); no ordinary early terminal receipt was found.  `ManuallyDrop`
is released only after the cursor's terminal predicate.  However, both DAG
payload and host `Drop` implementations intentionally leave the inner
`ManuallyDrop` un-dropped while unwinding
([`dag:3036`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3036>)).  That prevents an accidental
recursive destructor, but a caught panic leaks the exact state.  The existing
nonterminal-drop test only proves the panic, not a retained failed owner or a
process-fatal boundary.  Add a catch-unwind law that either proves a retained
failure cursor can resume/abort exactly once, or treats the panic as a
nonrecoverable process boundary; it must not silently count this as successful
retirement.

No Cargo or native test was run for this audit.  The newly authored Flow
retirement source oracle is explicitly source-only; the positive-grant native
row above is expected to expose the stated contradiction until repaired.

## Fresh Source Epoch and Cargo Input Evidence (2026-09-08, source-only)

The new epoch owner is a sound **captured-file stability** primitive, not yet a
qualified compiler-input proof.  Its boundaries are useful: it orders the
compiler legs, snapshots each regular no-follow file with a bounded,
checkpointed descriptor read, checks its identity and every ancestor after
read, then re-reads every captured plan file at every `start`, `complete`, and
`finish` ([`describe script:146`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:146>)).  Thus a compiler report cannot hide drift of a file already in `plan.files`: `verify()` scans the full captured set, not just the reported subset.  Cancellation during an owned file read zeroes the temporary buffer and closes its descriptor
([`semantic snapshot:4829`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:4829>)).

The prior serializer bug was real: repository `canonicalJson` sorted
id-object arrays, erasing the semantically ordered stdio→GIS leg sequence.
The current local `freshSourceOrderedJson` preserves arrays while sorting
object keys ([`describe script:100`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:100>)).  The new independent framed oracle must retain the reversed-leg rejection.  This is a source change under test; no compiled-source-epoch or Cargo qualification has been run here.

### Remaining P0: `complete` has no consumer-owned compiler evidence

`FreshSourceEpochOwnerV1.complete(id, compilerInputs)` accepts a public
caller-provided string array and only proves each supplied coordinate belongs
to the captured set ([`describe script:211`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:211>)).  It cannot establish that the list is the complete local input set actually used by Cargo/rustc/build scripts.  `produceFreshComponentV1` still invokes generic `freshRun` directly and has no epoch owner or dep-info/build-output parser
([`describe script:656`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:656>)).  Do not expose a better public string-list API: make completion evidence an opaque private result returned by the same child owner that spawned, drained, and reaped Cargo.

Static lexical mounted-module discovery is only a conservative preflight
superset.  In particular, `inspectRustModuleGraph` roots only a manifest
library path ([`discovery:7063`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:7063>)); it is not an exact resolver for a selected bin/test/bench/custom-build target, and requiring equality with its results would reject valid cfg-dependent builds.  The older `rustSourceFiles` walker is still reusable for a bounded conservative source closure, including literal `include*!`, but not as the completion oracle
([`library mjs:57`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:57>)).

### Smallest sound private integration

Introduce a private `runFreshCargoEpochLeg` used by `produceFreshComponentV1`,
not by arbitrary callers:

1. Derive a closed leg specification (exact package ID, manifest, target kind,
   target triple, profile, features and generated Cargo argv) from owned
   `cargo metadata --format-version=1 --locked` under the same environment.
   Bind actual `cargo -Vv`/`rustc -vV` output and normalized relevant
   environment values before capture; declared strings alone are not toolchain
   identity.
2. Use metadata's selected target `src_path` and each reachable local
   (`source == null`) package/custom-build root to make a no-follow,
   bounded *preflight* superset: workspace and package manifests, lock/config
   inputs, target root, literal modules/assets, and build-script source
   closure.  This must select Cargo targets from metadata, not default every
   manifest to `src/lib.rs`.
3. Run Cargo once in a fresh private `CARGO_TARGET_DIR`, with
   `--message-format=json`, retaining only bounded raw JSONL plus the exact
   selected compilation records.  Cargo's `build-script-executed` JSON exposes
   package identity and `out_dir`, but not the `rerun-if-*` declarations;
   derive the corresponding fresh target `build/<unit>/output` only from that
   owned `out_dir`, parse its bounded raw build output, and retain a digest of
   the raw report.  The official Cargo docs confirm both this JSON limitation
   and that build-script output is cached, so a shared target is not an
   admissible evidence source ([external tools](https://doc.rust-lang.org/stable/cargo/reference/external-tools.html),
   [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)).
4. Parse the exact selected local dep-info files from the isolated artifact
   paths, after Cargo exit and child reaping.  All actual repo-local Rust and
   literal include inputs must already have a captured snapshot; foreign
   registry/sysroot paths are resolver/toolchain facts, not repo source
   coordinates.  Unknown, malformed, duplicate, over-limit, or wrong-package
   records refuse qualification.  This follows the repository's existing
   isolated `rustc --emit=dep-info` oracle pattern
   ([`caching script:477`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts:477>)).
5. A build script's `rerun-if-changed=directory` needs a new source-record
   variant, not a fake file entry: retain the directory identity and an ordered
   no-follow recursive manifest of regular children, with bounded depth/count/
   bytes and membership/ancestor rechecks at every epoch transition.  An empty
   directory still needs its identity so an added child invalidates the epoch.
   `rerun-if-env-changed` values join the owned environment projection.  A
   script with no rerun declarations has Cargo's whole-package fallback, so
   either capture its bounded package-tree manifest or refuse this strict
   qualification; it cannot be modeled as zero inputs.

Raw Cargo artifacts and `rerun-if` output alone do **not** prove every
filesystem read a general build script performed: scripts are arbitrary host
processes.  To make the preceding trust claim sound without cross-platform
syscall tracing, qualification must additionally constrain local build scripts
to read only their captured package tree plus explicitly declared, captured
external roots; otherwise reject the candidate.  Generated `OUT_DIR` inputs
are derived evidence: retain their exact selected hashes/report under the
fresh target and bind them to the final component receipt, rather than calling
them source files.

For sequential stdio then GIS, retain one epoch owner across both generated
legs.  If any capture changes during or after stdio, do not use that stdio
output for GIS: abort and rebuild both legs under a new epoch.  The final
component/descriptor byte checks remain a distinct compiled-output proof.

### Required laws before a qualified source-epoch claim

- reverse the ordered stdio/GIS legs while preserving object keys: epoch bytes
  and digest differ; the current independent framed oracle should be the
  authority;
- actual dep-info discovers a local `mod`/`include_bytes!` omitted by
  preflight: completion refuses; a cfg-only static overapproximation is
  permitted and does not cause refusal;
- `rerun-if-changed` file changes during the qualifying leg, and directory
  add/remove/same-byte replacement/ancestor substitution each refuse; an
  unchanged empty directory accepts;
- no `rerun-if` declaration refuses absent a bounded package-tree manifest;
  unknown/out-of-root declaration and changed `rerun-if-env-changed` both
  refuse;
- cancel during a chunked capture and while Cargo runs reaps the child, zeros
  owned buffers, aborts the epoch, and leaves no component receipt; malformed
  JSON/build-output/dep-info and a wrong target/package record refuse;
- a source change between stdio completion and GIS start forces a complete
  two-leg restart, never a mixed-generation pair.

No Cargo or native test was run for this audit.  The current fixture proves
owner transitions and file stability only; it deliberately does not yet prove
Cargo resolver, dep-info, build-script, or directory-membership completeness.

## Stdio Full-Catalog Native Compile Cost (2026-09-08, read-only live sample)

The observed compiler is the exact active process, not a reconstructed build:
PID 12020 is compiling `semio_s_plugin_stdio` as both `cdylib` and `rlib`, with
`full-artifact-catalog`, against the isolated
`reactor-lifecycle-native-target` ([5-second macOS sample, retained under this
ticket](🗑️generated/stdio-rustc-12020-sample.txt)).  At capture it had run for
25 minutes, used a 14.0 GiB physical footprint (16.9 GiB peak), and all 2,863
samples reached LLVM code generation; the active worker was in
`LLVMRustWriteOutputFile`/AArch64 machine-function optimisation.  This is real
backend work, not a Cargo wait, source scan, macro parse, or a blocked file
lock.  The sampled process had no `RUSTC_WRAPPER`, uses the ticket-local
incremental directory, and has `-Z threads=8`; it is not evidence that a
shared target/cache was reused or corrupted.

The source gives a concrete explanation for the unusually large codegen unit:

- `full-artifact-catalog` mounts all 36 artifact roots and the full plugin
  root ([`stdio crate:127`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:127>)).
- That root closes a single `StdioApps` enum with 176 concrete
  `VcsArtifactApp<EditorApp<_>|ViewerApp<_>>` variants
  ([`stdio plugin:11`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🦀️.rs:11>)); the
  plugin builder itself registers 192 editor/viewer surfaces across the existing
  media/geometry/data/document groups ([`stdio plugin:254`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🦀️.rs:254>)).
- `PluginApp` has 78 dispatched methods ([`plugin trait:11781`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11781>)).  The current
  `dyn_enum_close!` expansion therefore creates a very large set of
  type-specialised forwarding/match bodies over that one closed fleet; this is
  coupled with the underlying generic `VcsArtifactApp` instances, rather than
  merely the 36 JSON `include_str!` definitions.

The immediate Hub dependency makes this cost unavoidable for the current
native target: `native-artifact-execution` forwards
`semio-s-plugin-stdio/full-artifact-catalog`
([`Hub Cargo:30`](</Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:30>)).  Yet the
native provider's actual contract is catalog-only: it needs the full schema
catalog and the 26 native codec factory receipts
([`registry:1118`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:1118>)), not an
instantiated 176-surface guest `Plugin<StdioApps>` runtime.

### Recommended small architectural split

Add a **`native-codec-catalog`** Stdio feature/surface, then make Hub's native
artifact execution depend on it rather than `full-artifact-catalog`.  It is
not a reduced catalog: it must mount all 36 definition/schema roots and all 26
native codec factories/receipts, retaining the existing complete-bijection
law.  It deliberately excludes only component-root concerns that a headless
native catalog provider never uses: `plugin()`/`Plugin<StdioApps>`,
`plugin_exports!`, and the editor/viewer module and 176-variant dispatch
fleet.  The actual WASI component keeps `plugin-root → full-artifact-catalog`
unchanged, so guest runtime semantics and the component artifact catalog stay
complete.

The necessary seam is narrow but must be explicit:

1. Move the component package identity parser out of `crate::plugin`, so
   `NativeCodecFactoryReceipt::instantiate` does not pull the full guest
   assembly merely to validate `semio:stdio`.
2. Split registry assembly into a catalog-only exact projection and a
   component-only surface projection.  `native_codec_factory_receipts` must
   validate the former against all 36 source definitions/26 factory bindings,
   without calling `crate::plugin()?.manifest`.
3. Bind that catalog projection to the decoded, compiled full-component
   descriptor in the existing trusted-catalog verification path.  A native
   receipt may therefore never silently describe a different 36/26 set than
   the genuine component; it simply no longer builds all app implementations
   to obtain the comparison.
4. Change only Hub's headless forwarding feature.  GIS/component producer and
   the `plugin-root` component keep their present full feature.  Do not use a
   partial “GIS-only” codec list: the current native provider contract is a
   complete 26-receipt catalog.

Add a schema-first `stdio-native-codec-catalog/v1` fixture that names the 36
definition identities, all 26 factory receipt tuples, and the compiled
descriptor projection digest.  Its native law must instantiate all 26 codecs,
compare exact artifact/schema/hash/extension tuples against the full component
descriptor projection, and reject a missing/extra factory or an attempted
`StdioApps`/guest-export link in the native-catalog build.  Its component law
must prove `plugin-root` still builds the same complete catalog and every
existing app ID.  A before/after isolated target timing/peak-footprint
measurement is a performance qualification, not a substitute for the
catalog-equivalence laws.

Nested enum sharding alone is a possible later LLVM-unit optimisation, but it
still monomorphizes every app and preserves the Hub's unnecessary guest-app
dependency.  It is not the smallest correction to this exact native build
edge.  No source, Cargo command, configuration, cache, or running process was
altered for this audit.

### Catalog split seam correction (2026-09-08, source-only)

The preceding recommendation needs one important feature-graph correction:
changing Hub's direct forwarding alone cannot remove the app fleet.  Both GIS
and VCS transitively request `stdio/full-artifact-catalog`
([`GIS Cargo:81`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:81>) and
[`VCS Cargo:36`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml:36>)).
Those uses are schema/snapshot/codec imports under `stdio::artifacts`; this
audit found no GIS or VCS call to `stdio::plugin()` or `StdioApps`.  Cargo
feature unification means Hub cannot undo either transitive request.

The clean minimum is therefore a semantic split inside Stdio, not a
Hub-specific escape hatch:

1. Keep `full-artifact-catalog` as the complete 36-root taxonomy and 26-codec
   surface used by native dependents, but make it catalog-only.
2. Add private/component feature `component-app-assembly =
   ["full-artifact-catalog"]`, and make `plugin-root =
   ["component-app-assembly"]`.  Gate the `plugin` source module containing
   `StdioApps` and the 176-variant `dyn_enum_close!` on
   `component-app-assembly`; retain `plugin_exports!` exclusively on
   `plugin-root`.
3. Extract `component_package_id` from
   [`stdio plugin:191`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🦀️.rs:191>) into a catalog-owned,
   pure Cargo-contract reader.  This is the only `crate::plugin` dependency
   of receipt instantiation ([`registry:889`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:889>)).
4. Replace the other guest dependency in
   `native_codec_factory_receipts`—`crate::plugin()?.manifest` at
   [`registry:1136`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:1136>)—with a catalog-owned
   ordered projection derived from the same 36 `artifact_assemblies()` and 26
   factories.  The current all-36 assembly and runtime-26 checks at
   [`registry:1129`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:1129>) must remain; the
   component-only `crate::plugin().is_ok()` test at `registry:1310` belongs
   behind `component-app-assembly`, not in the headless catalog law.

`manifest/🦀️.rs` is not a hidden app dependency: it merely forwards
`artifact_definitions()` and `format_descriptors()` and has no external
consumer found by the source census.  It can be catalog-gated or folded into
the registry projection.  The relevant factory functions already only depend
on the 26 artifact modules, their snapshot/mutation types, and protocol
`include_bytes!` hashes ([`registry:915`](</Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:915>)); no guest export or app
factory is needed to instantiate one.

There is a correctness requirement beyond merely removing that dependency.
The Hub's current `NativeCodecProviderSourceV1::preview` hands the decoded
descriptor to providers, but the Stdio branch ignores it
([`native provider:45`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:45>),
[`native provider:51`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:51>)).
The trusted loader currently proves only that every *declared descriptor kind*
is found in the bundle's native-codec rows
([`trusted catalog:1260`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1260>)); it does not prove the inverse.
This is harmless only because the old receipt constructor rebuilt the full
plugin manifest.  A headless replacement must make the relation bidirectional
and exact: sort and compare all 26 `(kind, schema)` rows of decoded
`descriptor.manifest.artifact_kinds` with the catalog projection before
returning any native binding.  The existing receipt/provider checks must also
retain exact package id/version, unique `factory_id`, unique
`descriptor_codec_id`, unique `(artifact_kind,schema)`, nonzero protocol hash,
and factory output `schema`/`extension`/`pack_schema_hash`
([`native provider:131`](</Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:131>)).

`PackageDescriptor` has no field for the ten definition-only roots: its
manifest exposes runtime `artifact_kinds`, while the complete `ArtifactDefinitionRegistry`
is an in-process plugin runtime owner.  Thus exact 36-root coupling cannot be
truthfully claimed from current compiled descriptor bytes.  To bind the
headless full catalog to the genuine component rather than only prove it from
shared Rust source, add one descriptor-owned, canonical
`stdioNativeCodecCatalogV1` projection or its domain-separated digest.  It
must include ordered 36 definition identities plus the exact 26 receipt tuples
(`factoryId`, descriptor codec id, runtime capability id, kind, schema,
extension, protocol SHA-256), package identity/version, and no factory
function pointers.  The component-app assembly emits the same projection; the
headless provider recomputes it and the loader compares it with the decoded
descriptor before registration.  The component's existing byte/describe proof
then binds both projections to the actual compiled component.

Required fixture/law rows are: a valid equal projection; each missing/extra
or reordered definition; each missing/extra/duplicated codec; altered
factory/descriptor-capability/kind/schema/extension/hash; a component
descriptor omitting one otherwise-valid native kind; a catalog descriptor
digest differing from the compiled component; and a catalog-only target which
can instantiate all 26 but cannot name `StdioApps`, `plugin()`, or guest export
symbols.  A `plugin-root` target must still prove all 36 definitions, the same
26 projection digest, and its existing full app roster.  These are
source-design requirements; no compilation or runtime qualification was run.
