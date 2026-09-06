# Reactor Exact Close-Lease Current Audit

## Current verdict — P0, not ready for a host capture wait

The current production `InstanceClose` pre-loop double-admits one native close. It therefore cannot safely be the close side of cold-pair capture or an eventual host lifecycle wait.

1. [`⚛️reactor/🦀️.rs:1304`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1304) captures a `PluginInstanceCloseLease`, and [`:1308`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1308) calls `lease.begin_close(runtime)`. The lease implementation moves the exact live app into `close_quarantine` in [`🚪️lifetime/🦀️.rs:15`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs:15).
2. [`⚛️reactor/🦀️.rs:1317`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1317) immediately calls `plugin_destroy_app`, which calls `plugin_begin_instance_close(..., None)` again in [`🔌️plugin/🦀️.rs:30501`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30501). That function rejects a populated quarantine before it can find an app in [`:30522`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30522), yielding `runtime close quarantine is saturated or collided`. No intervening cleanup can empty the quarantine between the sequential calls.

This is a deterministic close-path fault, not merely a duplicate cancellation request.

## Additional concrete ownership failures

### Foreign close is destructive before its authority is checked

`JOB_RENDER_BINDINGS.close_instance(instance)` at [`⚛️reactor/🦀️.rs:1306`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1306) and `lease.begin_close` at `:1308` both precede `NativeCloseKey::capture(request.lifetime, &lease)` at `:1310`. A malformed or old `ActorInstanceCloseRequest` can thus close the numerically selected live app and its render binding before the request lifetime/allocation mismatch is rejected. `NativeCloseKey::capture` is expressly side-effect-free before `begin_close`, as the source comment says at [`:1164-1170`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1164), so this ordering has no authority justification.

### The exact close lease is dropped and Retired can outrun native retirement

`InFlightClose` is `Copy` and keeps only request, generation and `NativeCloseKey` at [`⚛️reactor/🦀️.rs:1173-1178`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1173). The local lease is then dropped. Completion at [`:1340-1366`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1340) checks only reactor, patch and pending-patch close state; it never calls `PluginInstanceCloseLease::is_retired` ([`🚪️lifetime/🦀️.rs:27-48`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs:27)). It removes the lifetime slot and returns a `Retired` receipt before `plugin_step_close_cleanup` runs at [`:1369`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1369).

The retained lease has the only precise `RuntimeCloseWorkerState` identity and the only terminal validation that its cell, session, rejection, outcome, and pump are empty. A copied allocation address is not a replacement owner.

### Any post-begin reservation fault or ignored lifecycle ACK loses the protocol

After `begin_close`, each of reactor, patches, and pending patches can still return `Err` ([`⚛️reactor/🦀️.rs:1311-1316`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1311)). Before `InFlightClose::insert`, an error returns from `poll_kernel` while the runtime app has already left the live registry; no retained close cursor remains to prove/drain it or publish one outcome.

The pre-existing `GuestLifecycleCell` already models exact `Captured → Live → Accepted → Closing → Retired → Released`, holds its owner through ACK, and asserts on an early drop ([`⚛️reactor/🚪️lifetime/🦀️.rs:47-169`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:47)). Production does not mount it: opening records only bare `ActorInstanceLifetime` at [`⚛️reactor/🦀️.rs:1379-1402`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1379), never emits `Captured`, and ignores `InstanceLifecycleAck` at [`:1408-1411`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1408).

## Small coherent correction

Replace the bare `InstanceLifetimeSlot` plus Copy `InFlightClose` route with one fixed production lifecycle registry whose exact entry retains an owned `PluginInstanceCloseLease` through terminal receipt acknowledgement. This is a direct production use of the existing cell, not a second lifecycle protocol.

1. At successful `InstanceOpen`, mint and retain the allocation-bound lease in the same fixed lifecycle entry, install it in `GuestLifecycleCell`, return `Captured`, and require the exact `InstanceLifecycleAck` before `Live`. Failed app/actor setup must remove the cell only while it remains `Opening` and ownerless.
2. On `InstanceClose`, first read the entry and call `validate_close(request)`. Capture the existing lease/key and verify `NativeCloseKey::capture(request.lifetime, lease)` **before** touching render bindings, runtime quarantine, or any close registry. Then preflight all four fixed slots: lifecycle/in-flight, reactor, patches, pending patches.
3. Only after a retained entry exists for that exact key, call `lease.begin_close(runtime)` once, record its checked generation with `record_close_admission`, reserve/activate each close participant, and emit the cell's `Accepted` receipt. Delete the `plugin_destroy_app` call from this path.
4. A close cell remains mounted on every fault after native admission. It drives both `lease.is_retired()` and the three reactor close-complete predicates on later turns. It releases reactor/patch/pending resources only after all are complete, then calls `prepare_retired`; this prevents `Retired` preceding native cleanup.
5. Route `InstanceLifecycleAck` to `stage_ack`, then use `release_owner_step` followed by `finish_turn`. The terminal owner drops only after the exact `Retired` ACK and successful clock verdict. No raw key, weak allocation address, or synthetic receipt may release it.

The lifecycle module's `GuestLifetimeOwner` sealing currently admits implementations only in `⚛️reactor/🚪️lifetime`. Define the small runtime lease owner there (or expose a narrowly sealed constructor there), with `terminal_is_empty` delegating to `lease.is_retired` and `release_terminal` consuming only the terminal lease. Do not store a generic `PluginInstanceCloseLease<PA>` in the current non-generic thread-local registry without making that lifecycle owner representation explicit; the current `poll_kernel` is generic ([`⚛️reactor/🦀️.rs:1284`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1284)).

## First executable laws

Add these to the existing reactor lifecycle corpus, using a real `PluginRuntime` rather than a key-only fixture:

1. An exact open → Captured ACK → exact close admits one native generation, has no second `plugin_begin_instance_close`, and produces `Accepted`, then `Retired` only after `PluginInstanceCloseLease::is_retired`.
2. A close naming the same numeric instance but foreign activation/guest lifetime leaves app, `JOB_RENDER_BINDINGS`, reactor slots, and close quarantine unchanged.
3. Inject each reserve/activate failure after preflight. The entry stays retained, runtime close drains, exact resources roll back or are retired once, and no other instance/lifetime is changed.
4. A premature/mismatched `Retired` ACK leaves the owner and all close slots mounted; the exact ACK plus a successful clock releases exactly once. A stale ACK after numeric slot reuse is rejected.
5. Saturate a colliding fixed close slot before admission: the candidate remains live and renderable, rather than being quarantined and then faulted.

No test/build was run for this audit.

## Native production-reducer fixture — no new `PluginApp` mock

The smallest reusable native factory already exists, but it is deliberately private to the
plugin-runtime contract test module:

- [`🔌️plugin/🦀️.rs:33329`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33329) starts
  `plugin_builder_contract_tests`.
- [`:34738-34744`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34738) defines the real
  enum-dispatched `TestRuntimeApps: PluginApp`; it carries the real `VcsArtifactApp<TestApp>`.
- [`:34795-34809`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34795) creates the
  `App` declaration through the normal `App::builder` path, and [`:35598-35600`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35598)
  builds the actual `Plugin<TestRuntimeApps>` with `Plugin::builder`, `document_app::<TestApp>`,
  mutation-roster registration, and `try_build`.

Put the first direct native reducer law **in this existing module**. It then needs neither a
visibility change nor a broad `PluginApp` mock:

```rust
let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
crate::plugin_runtime::install_plugin_bundle(
    &runtime,
    __semio_plugin_bundle().await.expect("real synthetic bundle"),
);

let open = semio_framework::kernel::Event::InstanceOpen {
    request: semio_framework::kernel::ActorInstanceOpenRequest {
        activation_generation: 1,
        instance_id: 41,
        request_sequence: 1,
    },
    app_id: semio_framework::kernel::AppInstanceId(TestApp::<false>::APP_ID.into()),
    actor: "native-fixture".into(),
    config: Vec::new(),
    assets: Vec::new(),
    capabilities: Vec::new(),
    quotas: semio_framework::kernel::QuotaSchema::default(),
};
let result = crate::reactor::poll_kernel(
    &runtime,
    vec![open],
    None,
    semio_framework::kernel::Budget {
        fuel: 1024,
        deadline_ms: 1_000,
        max_effects: 16,
        max_patch_bytes: 65_536,
        max_frames: 16,
    },
).await.expect("actual reducer open");
```

The proposed law must assert the `Captured` receipt exactly matches the source open request
(`actor_instance_captured_receipt_matches`), the real runtime contains the selected app and
actor, and a later exact `Captured` ACK—not the open call itself—makes it live. It must not
inspect or hand-construct a bare `NativeCloseKey`.

This is currently blocked only by the reducer's conditional compilation: as observed,
`poll_kernel` is still defined in cfg-gated `wit_bridge` at
[`⚛️reactor/🦀️.rs:1284`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1284)
and re-exported only for wasm at [`:1080-1085`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1080).
Moving the reducer out of that WIT-only module is therefore the sole prerequisite. Keep WIT
translation (`wit_event_to_kernel`) gated; the native law supplies the existing canonical
`semio_framework::kernel::Event` directly. That makes the native path the same reducer, not a
second interpreter or a WIT mock.

### Required companion rows after the successful open

1. `Captured` ACK is exact; foreign request sequence, activation generation, or guest lifetime
   leaves the live owner intact.
2. A close before the `Captured` ACK is rejected without changing the real runtime cell or render
   binding.
3. The ordinary exact open → ACK → close path observes one runtime close admission and only emits
   `Retired` after the retained native lease reports terminal emptiness.

These rows may remain in the same private contract module. They use a real registered bundle and
the production reducer; no separate fixture bundle, WIT envelope, or `PluginApp` implementation
is warranted.

## Setup must be one preflighted runtime transaction

There is a distinct owner leak in the present open sequence. Reactor inserts static metadata,
calls `plugin_create_app_with_id`, and only then calls `set_instance_actor`
([`⚛️reactor/🦀️.rs:1381-1401`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1381)).
`plugin_create_app_with_id` has already bound and installed an `Arc<RuntimeAppCell<PA>>`
([`🔌️plugin/🦀️.rs:29680-29712`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29680)), but
`set_instance_actor` can reject oversized actor bytes, a busy registry, or an occupied/colliding
actor slot ([`:29335-29348`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29335)).
The current error branch removes only static metadata/lifetime bookkeeping, leaving that real app
in `runtime.instances` without lifecycle ownership or an actor authority.

The new runtime entrypoint should therefore replace—not wrap sequentially—the pair of public
operations for the reactor path. A narrowly crate-private shape is sufficient:

```rust
pub(crate) async fn plugin_open_actor_instance<PA: PluginApp>(
    runtime: &PluginRuntime<PA>,
    open: ActorInstanceOpenRequest,
    app_id: &str,
    actor: String,
    lifecycle: GuestLifecycleCell<NativeLeaseOwner<PA>>,
) -> Result<ActorInstanceLifecycleReceipt, OpenRejected<PA>>
```

Before factory work, preflight exact identity/collision/capacity of **all** fixed owners:
runtime live instance, close quarantine, actor row, lifecycle row, reactor metadata row, and its
render/patch reservation rows. Construct `RuntimeActorAuthority` before any insert. After
`Plugin::create_app` and `bind_instance_id`, capture the allocation-bound lease, install its
`NativeLeaseOwner` in the admitted lifecycle cell, then commit every prepared row without a
fallible operation between commits. Returning an error before owner installation leaves an
`Opening` ownerless cell safe to drop; any error after it must return/retain the exact mounted
cell, never remove its row.

This avoids trying to compensate a partially open app with a second close operation, which would
reintroduce the double-admission fault above. Native test rows should force respectively an
oversized actor and an actor-slot collision and assert: no live app, no close quarantine,
no lifecycle/metadata row, and no allocation leak. A successful row proves the ordinary bundle
factory plus every runtime row commit together.

### Generic-bound correction required before mounting the typed field

`PluginRuntime` is intentionally declared for `PA: PluginApp` only
([`🔌️plugin/🦀️.rs:29236-29250`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29236)).
`PluginInstanceCloseLease` is also declared at that bound, but its single current implementation,
including `is_retired`, requires `PA: PluginApp + 'static`
([`🚪️lifetime/🦀️.rs:5-27`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️.rs:5)).
Consequently a `PluginRuntime` field of
`GuestLifecycleCell<NativeLeaseOwner<PA>>` cannot satisfy the cell's sealed
`GuestLifetimeOwner` bound for every `PA` unless the whole runtime is unnecessarily narrowed.

Split the lease methods instead. Keep allocation identity, admitted generation, and `is_retired`
in `impl<PA: PluginApp>`; keep only `begin_close` in
`impl<PA: PluginApp + 'static>` if the close worker truly requires it. Then
`NativeLeaseOwner<PA>` can implement the sealed terminal trait at the existing runtime bound,
delegating `terminal_is_empty` to `is_retired`; the reactor's `poll_kernel` may retain its already
appropriate `'static` bound for actual close admission. This preserves unrelated non-static
runtime users and avoids a broad generic API change.

### The native success row must close what it opens

The test must not stop after asserting its successful `InstanceOpen`/`Captured` receipt.
`GuestLifecycleCell::Drop` deliberately asserts that a nonterminal owner was not silently
discarded ([`⚛️reactor/🚪️lifetime/🦀️.rs:167-170`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:167)).
The real-bundle native law therefore needs one complete tail: exact Captured ACK, exact close,
Accepted ACK, repeated ordinary poll turns until the retained native lease is empty, Retired ACK,
then a final successful turn that releases the runtime/lifecycle entries. This proves the initial
open and also prevents an intentionally loud terminal-owner assertion from masquerading as a
test-fixture failure.

## Current implementation delta — typed owner has landed, reducer migration remains in progress

The current source now places the right *kind* of state on `PluginRuntime`:
`guest_lifetimes: RefCell<NativeLifecycleRegistry<PA>>` at
[`🔌️plugin/🦀️.rs:29236-29258`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29236),
and `NativeLifetimeOwner` retains the close lease plus the three reactor/patch/pending close
participants at [`⚛️reactor/🚪️lifetime/🦀️.rs:174-258`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:174).
`plugin_open_actor_instance` now preflights actor/live/quarantine rows before it installs that
owner and commits app/actor rows ([`🔌️plugin/🦀️.rs:29674-29696`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29674)).
The semantic reducer is now also separable from WIT:
[`⚛️reactor/🔄️turn/🦀️.rs:160`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:160)
defines `poll_kernel`, and reactor re-exports it unconditionally at
[`⚛️reactor/🦀️.rs:1083-1085`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1083).

This is not yet coherent source: `turn` still owns and drives the old component-persistent
`INSTANCE_LIFETIMES` and `IN_FLIGHT_CLOSES` (`🔄️turn/🦀️.rs:12-110`) and still runs the old
double-close path / ignored ACK path (`:173-289`). Those two routes must not coexist. Replace
the pre-loop and old `Event::InstanceOpen`/`InstanceLifecycleAck` arms with calls into the one
runtime registry, then remove the static lifetime/in-flight types and their native-key helper.
Leaving either old close path reachable bypasses the retained cell and reintroduces the original
P0.

Correction to the preceding generic-bound concern: `PluginApp` itself is explicitly
`Send + 'static` ([`🔌️plugin/🦀️.rs:11754`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11754)).
Therefore every existing `PA: PluginApp` already satisfies the close lease implementation's
`'static` requirement; no generic bound split is needed. No native qualification is claimed for
this in-progress source.
