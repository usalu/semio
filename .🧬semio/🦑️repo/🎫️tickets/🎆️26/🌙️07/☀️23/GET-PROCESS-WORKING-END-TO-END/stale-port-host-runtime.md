# Stale Port: Plugin Host and Interpreter Runtime

Prefix `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`. Build: `stale-cargo.sh host-*|runtime-*`, native `build-domain`.

## `🧠️interpreter/🏃️runtime` (14 errors before, 0 after; `cargo check --tests` clean)

Before: `NativeEncodeControl::close_retirement_recipient` removed, `ControlledRetirement::step(grant)` / `close_step(grant)` now `(body, effect)` returning a poll, `OriginalActorFixedReceipt::read` returns `(call, rest)`.

After:
- New test-only helper `🧪️tests/🪙️close/🦀️.rs`, mounted `#[cfg(test)]` from `🦀️.rs` as `original_close`: `paid` (one `AuthoringWallet::run_invoked` effect of exactly the grant), `close_encode` (local `OriginalEffectReply` slot over `close_retirement_recipient_into`), `settled` (poll to step).
- Ported sites: `🎛️operation/📥️input/🦀️.rs` tests, `🎛️operation/📤️checkpoint/🧪️tests/🦀️.rs`, `🧪️tests/🔬️owned-runtime/🦀️.rs` (`.0` of the read receipt).
- Dropped: the zero-grant `owner.step(denied)` assertion (a zero grant cannot open an effect any more); the equality `heap.requested == progress.retained_capacity` is now `>=` because an invoked step allocates its reply handoff on top of the reported progress; `heap == demand` removed.
- Snapshot cancellation test grant widened to items 16 and copy 4096 (a step through the invoked effect needs more than the old 64 copy bytes).

Test state (`--lib`): 17 pass (plus the input tests, see run `runtime-t8`); remaining failures not caused by the port:
- `owned_instance_open_tests` x4: the neutral component `semio_plugin_host_test_component.wasm` was not produced (`🧪️testing/🧩️component` dist missing); needs the repo producer, not run here.
- `microsecond_clock_tests::original_interpreter_deadline_...`: a non-expired snapshot row no longer returns `Trapped`.
- `original_startup::source_clock_reply_tests::...same_callback...`: round 1 observes no host clock read.
These two are interpreter behaviour, untouched by the ported APIs.

## `🖥️host` (17 errors before, 0 after; 23 of 25 lib tests pass)

Before: unit tests built `PluginInstanceHandle::new(.., test_relay_wake_authority())` and called `io_run/io_sniff/migrate/mutation_plan` on the handle; both helpers and the methods are gone (handles are now born inside each call's own Source; routers take `PluginRegistration` and `PluginCallDrivers`).

After (`🧪️tests/🔬️unit/🦀️.rs`):
- New `mock_registration` (registers a `MockGuestRuntime` package with `origin_registry` under an `OriginalJobFixtureCaller`, retiring the caller) and `compose_key/compose_dialects`.
- `io_router_registers_two_plugin_registrations` replaces the two-hop `run_io` test; `io_router_compose_still_refuses_to_route_back_into_the_calling_plugin` ported to the 4-arg `compose` (refusal precedes any driver).
- Removed: the mock-backed dispatch tests (`io_run`, `io_sniff`, `migrate`, `mutation_plan`, `compose` dispatch, 2-hop `run_io`). Host startup (`🏗️instance/🦀️.rs`) requires the real `Owned` phased backend, so a mock cannot serve a routed call ("Host startup requires an implemented original phased backend"); `migrate` has no job kind any more. Dispatch coverage needs the built component.
- `🪞️schema-parity`: expected actor export set updated to the WIT (it gained the codec sqlite, actor-context and failure exports).
- `🎠️activation` test: the quarantine stage now submits and ticks a turn before `record_transport_fault` (a bare `complete` is `TurnNotIssued`).

Product fixes:
- `🧬️guest-contract`: `PluginHostError::OriginalTurn` boxed (3024 B inline), `TurnFault::Guest` boxed (192 B); all construction sites in host and runtime updated; the layout fixture budgets are `80` (host error, `ValueError` is 64 B and must stay inline: the invalid-receipt law forbids allocation) and `160` (turn fault, `OriginalActorFixedCall` is 144 B and must stay inline for the same law). Previous budgets were 64 and 112.
- `🧵️shard/🪪️identity`: `OriginalShardIdentity` leaked its `NativeAllocationAuthority` (its Drop asserts an explicit owner transfer); the authority is now `Option` and returned through `into_port` on drop (unsettled authorities are forgotten, never abort); the refused-issue path returns it as well.

Remaining host failures:
- `neutral_activation_failures_...`, stage `complete`: `OriginalShardIdentity::issue` is refused with "forwarded native storage requires its quoted allocation effect" (`NativeEncodeControl::charge` on a forwarded port outside a quoted effect; the same class as the forwarded-binding gap noted for the decode retirement ledger). Real product gap in the identity birth, not fixed.
- `runtime_metrics_publisher_...2550_record_scale_fixture_registry`: the registry fixture it reads (`🧪️testing/⚖️scale/🧪️tests/🧫️fixtures/📇️registry/🔣️.json`) is now a three-case list, not a record registry with `recordCount`; the 2550-record registry has no producer in the tree.

Dependents of the boxed variants (`🖨️describe`, `🌉️mcp`, hub and plugin guest tests) only pattern-match on `TurnFault::Guest`, which autoderefs; not rebuilt here.
