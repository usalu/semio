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
[`GuestRelayPoolFuture::fail`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:3013)
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
  ([`refuse_pending_registrations`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs:563)).
- `state.drive` may itself own a `ShardLoop` plus an in-flight async registration or
  guest close; it cannot be discarded or replaced while pending.
- After a terminal handoff, the executor leaves both `state.shard` (with all normal
  instances) and receiver-less registrations intact. Dropping the outer `Arc` would
  destructure raw `GuestInstance`s without the required
  [`GuestRuntime::drop_instance`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:707)
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
  ([`shard/🦀️.rs`](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:1489));
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
