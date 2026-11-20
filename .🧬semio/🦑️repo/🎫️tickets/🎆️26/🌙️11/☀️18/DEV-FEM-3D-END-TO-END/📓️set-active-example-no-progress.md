# Set Active Example Makes No Progress (2026-11-19, build 12:33)

The fem component wasm published at 12:33 includes the ingress return-table fix (`lend_step` reserves its whole claim ladder). The livelock is gone: commands seq 2, 4, 5 and 6 now complete in the browser.

The command `Set Active Example` (seq 7, kind 1, one page) never completes. After 48 s the UI shows the stall banner "has not answered “Set Active Example”".

## Browser trace (dev server 127.0.0.1:6087, `SEMIO_VITE_HMR=0`)

```
gate exchange.produced: seq 2 applied true frames 2 effects 0 events 0 ...
gate exchange.produced: seq 4 applied true frames 2 ...
gate exchange.produced: seq 5 applied true frames 2 ...
gate exchange.produced: seq 6 applied true frames 3 ...
gate exchange.produced: seq 7 applied false frames 0 effects 0 events 0 constructor false preparation false retirement false typed_pages 0
exchange_generic: seq 7 command decoded output false applied Some(false)   (repeats every turn)
more-work: command_ingress CommandPending(CommandPageCursor { owner: 1, generation: 1, command_index: 0, command_count: 1, instance: 1, seq: 7, kind: 1, page_index: 0, page_count: 1, item_count: 0, metadata: 0 })
```

Every turn, `plugin_exchange_turn` returns `applied false` with nothing produced, so `route_exchange_output` routes an empty output and the owner stays `Generic`. The ingress gates cycle normally: lend, resume, `settle_step` ready again after 2 → 3 row transitions.

## Code path

`⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs` `exchange_generic` → `plugin_exchange_turn` (semio_plugin_runtime_exchange) with a Decoded command and `OriginalCommandExchangeState` whose `applied` stays false.

## Findings

Status: root cause NOT yet proven. No fix was applied. Instrumentation is in place so the next component build names the stalled stage in one console line. Everything below was derived by reading the code and the live page, not from a reproduction.

### What is established

1. `seq 7` is `AppCommand::Command` (index 1 in `📡️spr/🧵️channel/📨️app-command`), i.e. the manifest path. `advance_manifest_step` (`⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`) is the only place that can return `applied false` with an empty output while the command stays `Decoded`. An `Err` is impossible: it would be retained as `FailedDecoded` and logged as `seq 7 FAILED with ...` on the next turn. So some stage returns `Ok(None)` forever.
2. The page fires `setActiveExample` itself on boot (page default example, shell log `[DEBUG] command ingress fem action=setActiveExample ... command-pending`, then `[os-shell] command stalled: fem setActiveExample has held fem#1's lane for 5000 ms`). Every reload reproduces it, so a fresh load is a complete reproduction. Seq 2-6 are the boot commands before it.
3. The retained grant is not the limit: the console shows `maximum_items 8.3M, copy 530M, capacity 532M, fuel 49.9M`, `yield false` on every turn. Stages that gate on `paid()/grant` therefore pass; the blocker is a logical wait, not a quote refusal.
4. `setActiveExample` of fem3d is a declared typed command, one of the 34 retained tools (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../✏️editor/🗃️retained-commands`), publication lane `HostOnly`, handler `🎮️commands/📚️set-active-example` is synchronous and cannot be the blocker (it builds one `Effect::LoadDocument`). The 15 document tools publish to the Artifact lane, camera/result tools to WindowConfig. `HostOnly` is shared with framework undo/redo, so the lane alone is not unique.
5. Wrong turns ruled out: the catalogue route (`CATALOGUE_EXAMPLE_ACTION_ID`) is skipped because the registry has the verb; the effect pack of the `demo` example is 2.9 KB so no size cap applies; the `ENCODE` phase of the command producer asks 65,536 bytes of a 16 MiB capacity grant.
6. Console state at the stall (`more-work inputs`): `typed_runnable false, result_pages false, pending_effects false, displaced true`. No typed operation was ever mounted for seq 7 (otherwise `typed_runnable` would be true), so the stall is BEFORE the typed handoff in `dispatch_typed_command_inner`. `displaced true` is constant and comes from `plugin_drive_displaced_surfaces`, which reports pending for any instance whose `host_instance()` is lent out or whose surface registry holds a capture lease; it needs a look together with candidate (b).

### Candidates, in order of how well they fit

Pre-handoff gates that return `Ok(None)` without consuming any budget (all in the action path `handle_action_invocation` -> `dispatch_action` -> `dispatch_action_received`, `🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs`):

- (a) `host_events.resumes.in_flight()` or `host_events.delivering`: a task-resume or host-event delivery holds the shared typed source, every other lane waits (first line of `dispatch_action_received` and `dispatch_command`).
- (b) `OriginalSurfaceRefresh::advance` (`⚛️reactor/🪟️surfaces/native.rs`) returns `false` while `registry.cached_capture.is_some()`; `retire_displaced_step` waits on the same lease. The earlier `refresh:url#1 pending=[':fem3d-model', ':fem3d-results', ':framework.section.catalogue']` banner (seen on the old page after minutes) is the same refresh waiting. If a surface reader keeps the capture while the surfaces it reads wait for the example, the command and the render deadlock (empty windows until the example loads, example load waits for the surfaces).
- (c) `original_typing.pending()` / `prepare_action_typing_before` or the tool-tag capture (`prepare_action_tool_dispatch`, retained-tool scrub dispatch): `setActiveExample` is a retained tool id, so it takes the tag-capture ladder like every fem3d verb; the ladder returns `Ok(None)` while its machine is not ready.
- (d) `admit_typed_operation_slot` returning `None` (`latest_wins_order.allocation_admitted` false, or all 64 slots busy).
- (e) the `constructing` early return of `plugin_exchange` (`store_constructor_is_pending`), which would return an empty output before any command stage.

### Instrumentation added (temporary, `[DEBUG]`, wasm-only unless noted)

All new, each prints only when its text changes, so a stall prints once.

1. `⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`: `advance_manifest_step` is now a wrapper around `advance_manifest_step_inner` (the former body, unchanged). On `Ok(None)` it logs `[DEBUG] manifest pending: <receiving summary> | <typed_operation_diagnostic> | surfaces <capture/displaced state>`. The receiving summary (new `debug_summary`) carries `kind, phase, finished, metadata phase, projection, context_ready, refresh, refresh_ready, result, args, action id`. Decides between (b) and everything after the metadata stage.
2. `.../🛂️manifest/metadata.rs`: `debug_phase()` (wasm-only accessor).
3. `⚛️reactor/🪟️surfaces/native.rs`: `SurfaceContexts::debug_state()` (capture lease held, displaced count, retiring, slots in use). Decides (b).
4. `🧵️instance/🔌️plugin-app-impl/🦀️.rs`: `typed_operation_diagnostic()` now also prints `gates resumes=.. delivering=.. typing=.. emission_host=.. emission_input=.. typed_input=.. typed_receipt=.. typed_meta=.. typed_production=.. action_source=..` ahead of the existing `lw=.. ob=..` part. Decides (a), (c), (d). Not wasm-gated (pure formatting).
5. `⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs` (`exchange_generic`): when `plugin_exchange_turn` returns `applied false`, `debug_state("exchange.unapplied", ...)` logs `constructing` (`store_constructor_is_pending`) plus the same typed diagnostic. Decides (e) and covers stalls that never reach the manifest stage (the manifest line would then be missing).

How to read the next run: one `[DEBUG] gate exchange.unapplied:` line and, if the manifest stage was reached, one `[DEBUG] manifest pending:` line per distinct state. The existing probes `[DEBUG] manifest advance_step`, `manifest parse step`, `manifest admit_loan refused`, `metadata advance`, `displaced:` and `surface capture taken by reader` are already in the build and are printed on change too; their first lines are lost in the flood (the console keeps ~5 s), so filter them by name and reload, or run the page in a background tab with a name filter before the stall starts. Reading them from the 12:33 build was not possible: the page buffers only the newest few thousand lines and the actor logs come from a worker, so a main-thread hook cannot see them.

### Fix

None. A fix chosen without the stage name would be a guess. Expected shape per candidate: (a) release/complete the resume or delivery that holds `resumes.in_flight()` before the action lane; (b) never let the command-side `OriginalSurfaceRefresh` wait on a capture lease owned by a reader that depends on the command (take the refresh from the registry's last view when the lease is held); (c)/(d) quote the missing precondition into the grant ladder.

### Native test

No test was added: without the stage there is no failing assertion to write. Existing native coverage of the neighbouring code: `🧪️testing/🧪️artifact-app-laws/🪧️pane-smoke-law` (fem3d pane smoke `✏️s/.../✏️editor/🧪️tests/🪧️pane-smoke`) drives `setActiveExample` through `dispatch_original_fixture_command` with a maintenance step between pending turns, but it does not compile today (`owner.step` -> `step_into` drift, see `📓️fem3d-native-test-failures.md`) and it does not run the reactor `exchange_generic` loop, which is where the difference lies (the reactor does not run `maintenance_step` between the command's turns). Once the stage is known, the regression test belongs next to `⚛️reactor/🧪️tests/🚚️more-work-drive` with a fixture app whose `setActiveExample` is a retained tool, driven through `exchange_generic` for N turns.

### Verification and environment

- `cargo check -j 4` (CARGO_INCREMENTAL=0, CARGO_PROFILE_DEV_DEBUG=0, CARGO_TARGET_DIR=C:/b/sae-target, build dir `C:/b/sae-target/b`, new layout off, plain rustc): `semio-plugin-runtime-ops`, `semio-plugin-instance-app` and `semio-plugin-reactor-turn` pass natively (5 min + 50 s). This type-checks edits 3, 4 and the closure arguments of edit 5.
- `cargo check --target wasm32-wasip2 -p semio-plugin-runtime-ops` FAILS in a crate I did not touch: `semio-framework-pack-record-retained` (`close_step_unpaid` is not a member of trait `RetainedValueDriver`, a peer's in-flight change). So the wasm-only blocks (edits 1, 2 and the log call of 5) are NOT compiler-checked; they mirror existing `#[cfg(all(target_arch="wasm32",target_env="p2"))]` blocks in the same files (`#[cfg] if` as in `drive_self_waking_ready`, `thread_local!` with `const` init, `with_instances_turn_mut(..).await` as the other manifest stages). Re-run that check once the peer's crate compiles, before the 60-minute build.
- C: had 25 GB free after the checks; `C:/b/sae-target` holds 1.4 GB and can be deleted. No processes of mine remain.
- Browser: I reloaded the coordinator's `127.0.0.1:6087` tab twice (it now shows the stall banner again on a fresh load) and opened/closed one background tab; nothing else was touched.

### Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/metadata.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/native.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`
- Script input kept: `🗑️generated/sae-env.sh` (cargo env for the checks above).

## Static analysis (agent 2)

Read-only pass, no source touched. `P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Confidence: high that this is a real, reachable deadlock matching every console fact; medium that it is THE one in this build (not proven by a run, see the probe predictions at the end).

### Verdict per candidate

- (b) capture lease as the ONLY cause: no, it is a symptom. `OriginalSurfaceRefresh::advance` (`P/⚛️reactor/🪟️surfaces/native.rs:34`) does wait while `cached_capture.is_some()`, but the lease is only taken by `OriginalSurfaceViewRead::advance_selected` phase 0 (`P/⚛️reactor/🪟️surfaces/read.rs:62-64`) and only released by its `close_step` (`read.rs:115`). Its only drivers are `OriginalResponseActionSource::advance_cache` (`P/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/response.rs`), used by deferred responses (`P/⚛️reactor/🔄️turn/📥️receiving/🦀️.rs:61`) and, through `intent.rs`, by UI intents (`receiving/🦀️.rs:127`). Surface renders (`:fem3d-model`, `:fem3d-results`, `:framework.section.catalogue`) never touch it: they use `borrow_window_lease` (`🖥️surface-render/staged.rs:56`). The lease is released only after the response/intent finishes its dispatch (`receiving/🦀️.rs:51`, `close_intent_source` `receiving/🦀️.rs:108`), so it is held for as long as that dispatch returns `Ok(None)`. A reader that does not need the example does exist; what it needs is the same shared claim the command needs (below).
- Why `displaced` is always true: `plugin_drive_displaced_surfaces` (`P/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs:50-61`) is pending for an instance whose `host_instance()` is lent out or whose `retire_displaced_step` is false. `retire_displaced_step` (`native.rs:82-103`) returns false for a non-empty `displaced`/`retiring`/`retirement` as soon as `cached_capture.is_some()` (`native.rs:89`), otherwise it moves one paid step per turn. Constant `true` for 48 s with a huge grant therefore means a lease holder exists for the whole stall (an unreleased response/intent source) and a displaced projection waits behind it. It is polled only at `poll-kernel-turn/🦀️.rs:67`, AFTER the command stage; the command path never waits on it.
- (a) `resumes.in_flight()` / `delivering`: cannot persist at boot. `in_flight = started||closing` (`P/🎮️command/🎟️typed/🔁️resumes/🦀️.rs:71`); `started` is only set by `plan` (`:99`) with a non-empty queue (a task must have resolved with `TaskResolution::Command`; the boot page has none). `host_events.delivering` is set at `P/🧵️instance/📨️dispatch/📥️staging/📣️host-events/🦀️.rs:117-123` only with queued host events. Predicted `gates resumes=false delivering=false`.
- (c) typing/tool-tag ladder, (d) `admit_typed_operation_slot` (`P/🧵️instance/📄️docs/📥️staging/🧾️typed-operations/🦀️.rs:71`), (e) `store_constructor_is_pending`: all sit AFTER the claim below in the action path (`dispatch_action_received`, `P/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:158+`), and each would leave a pending flag or op visible (`typed_runnable` or the gates line). The console shows none; (e) would also have blocked seq 2-6.

### Most likely deadlock: the shared command-scope claim is pre-emptible and never held

One owner token `original_command_scope` (`P/🧵️instance/🦀️.rs:94`) is claimed by SEVEN lanes through `claim_scope` (`P/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs:9`), via `capture_original_host_event_route` or `claim_original_command_scope` (same file):

- manifest commands: `handle_command` (`P/🧵️instance/🔌️plugin-app-impl/🦀️.rs:516`, first statement)
- manifest actions: `handle_action_invocation` (`plugin-app-impl/🦀️.rs:491`) and `handle_action` (`:471`)
- deferred responses: `handle_response_action` (`:493`)
- UI intents: `handle_intent_frame` (`:532`)
- task resumes: maintenance branch (`:278`)

`claim_scope` semantics: (1) scope matches this caller (binding + decoder identity) returns true; (2) else if `shared_pending` returns false and keeps the scope; (3) else it OVERWRITES the scope with the new caller, charges the claim, and returns FALSE. The existing law pins this: `host-event-routing/🧪️tests/🔣️.json` `foreignIdle:false, rebound:true`. So a claim needs one more call by the same caller to become true, and nothing protects it in between: `shared_command_source_pending()` (`P/🧵️instance/📥️staging/📣️host-event-routing/🦀️.rs:3`) is only true after the owner began real work (route cache, tag capture, typing, typed command), which only happens AFTER its claim returned true.

Cycle (one reactor turn, order from `poll-kernel-turn/🦀️.rs`):
1. `drive_command_ingress` (`:48`): seq 7's manifest stage is past refresh (`refresh_ready`), calls `handle_action_invocation`/`handle_command`: scope is not mine, nothing pending, claim, scope := seq 7, return `Ok(None)`. `advance_manifest_step_inner` returns `Ok(None)`: the observed `applied false`, empty output, `CommandPending`.
2. `advance_original_receiving` (`:50`, `event-dispatch/🦀️.rs:245`): a second owner on the same instance, which took `cached_capture` while seq 7's refresh was between steps, runs `advance_intents`/`advance_responses`. Its source advance is done (it holds the lease), so it calls `handle_intent_frame`/`handle_response_action`; first statement is the claim: scope is seq 7, not pending, so it OVERWRITES scope := reader, returns false, `Ok(None)`.
3. Next turn: seq 7 finds scope = reader and claims again (false), then the reader claims again (false). Forever. A claim costs nothing the turn notices, `yield` stays false, nothing typed ever mounts (`typed_runnable false, result_pages false, pending_effects false`), and the reader never finishes, so `cached_capture` stays set and `displaced` stays true. That is every row of finding 6. Seq 2, 4, 5, 6 ran when no second owner was active in the turn (alone: claim, then match on the next call). Seq 7 is the first command that overlaps a window-mount intent or host response on boot, which is why a fresh reload always reproduces it.

The interleaving that makes (b) a symptom: seq 7 completed `OriginalSurfaceRefresh` first (it no longer needs the lease), then the reader took the lease between steps. The turn order is fixed (command ingress, then receiving), so the two lanes alternate perfectly and neither claim ever survives to its next call. The same ping-pong happens with the task-resume maintenance lane (`:278`) as second owner, without the lease/displaced signature.

### Clean fix at the source

Make the scope a held lease instead of a pre-emptible hint. In `host-event-routing/🦀️.rs`:

```rust
fn claim_scope(scope:&mut SourceScope,pending:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if scope.is_some_and(|held|held.owner(cx))  { return Ok(true) }   // mine
  if pending||scope.is_some()                 { return Ok(false) }  // someone else holds it: wait, never overwrite
  /* pay the claim once */ *scope=Some(Held::new(binding,decoder)); Ok(false)
}
```

plus an explicit `release_original_command_scope(cx)` (sets the scope to `None` when it matches the caller) called at each lane's terminal: when `handle_action`/`handle_action_invocation`/`handle_response_action`/`handle_command`/`handle_intent_frame` (`plugin-app-impl/🦀️.rs:471,491,493,516,532`) return `Some(result)` or `Err(_)`, and in every owner's close path (manifest `OriginalDecodedPluginCommand::close_step`, `RetainedIntents::close_intent_source`, the `RetainedResponse` finished branch, task-resume `Close`). A waiting lane then returns `Ok(false)` without overwriting, the holder matches on its next call and proceeds, and waiters are served in turn order. `original_terminal_is_empty` of the instance must also assert `original_command_scope.is_none()` so no stale holder survives an instance close. No timing tricks and no skipping of the response lane.

### Native test (deterministic)

Extend `P/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🧪️tests/🦀️.rs` (same `OriginalJobFixtureCaller`, two `OriginalJobDecodeOwner`s `a`/`b`, `scope=None`, `pending=false`), new law row `"fairRounds": 4` in `🧪️tests/🔣️.json`; test `two_idle_callers_never_starve_each_other`: for 8 rounds call `claim_scope(&mut scope,false,cx)` first as `a` then as `b` (the reactor order), record `(a,b)`, assert `granted.iter().any(|(a,b)| *a||*b)` within `fairRounds`. Today every round is `(false,false)` (fails). With the lease: round 1 `(false,false)`, round 2 `(true,false)`, `b` stays refused until `release_original_command_scope`, then `b` wins within two calls. A reactor-level regression belongs next to `⚛️reactor/🧪️tests/🚚️more-work-drive`: fixture app, a command past refresh plus one pending intent holding `cached_capture`, `exchange_generic` for 50 turns, assert the command completes and the lease is released.

### What the existing `[DEBUG]` probes print if this is right

- `[DEBUG] manifest pending: ...` once (state never changes): receiving summary with `refresh_ready true`, `context_ready true`, no result, metadata finished; app part `gates resumes=false delivering=false typing=false emission_host=false emission_input=false typed_input=false typed_receipt=false typed_meta=false typed_production=false action_source=false`; `surfaces capture true displaced N>=1 ...` (lease held by the OTHER lane).
- `[DEBUG] gate exchange.unapplied:` with `constructing false` and the same all-false gates.
- `[DEBUG] surface capture taken by reader 0x... selection Some(..)` once, with NO matching `surface capture released by reader close`; `[DEBUG] displaced: cached_capture Some(0x...) displaced N ...` once and constant.
- Absent: `manifest admit_loan refused`, `metadata advance` churn, any `FAILED`.
- Disproof: `capture false` in the `surfaces` part kills the capture half; with `refresh_ready false` in the summary the stall is inside `OriginalSurfaceRefresh` itself. Any `gates` field true means take that candidate (a/c/d) instead. A cheap extra probe (not added, tree is frozen): log inside `claim_scope` when the scope changes owner with the lane kind (command|intent|response|resume); two owners alternating on consecutive turns is the smoking gun.

## Fix: held command-scope lease

`P` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `D` = `P/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing`.

### Cause as verified in the code

`claim_scope` (`D/🦀️.rs`) returned true only for the matching holder. A non-owner with nothing pending OVERWROTE the scope with itself, paid the claim and returned false. The scope was never released. Two lanes in the fixed reactor order (command stage, then receiving stage) therefore replace each other's claim every turn, and neither ever sees a matching claim on its next call. The old law pinned exactly this (`foreignIdle:false, rebound:true`). The claim sites are `handle_action`, `handle_action_invocation`, `handle_response_action`, `handle_command`, `handle_command_frame`, `handle_intent_frame` and the task-resume branch of the maintenance turn. Whether this is THE stall of the 12:33 build still has to be confirmed by the next browser run (see "Not covered").

### Fix

- `D/🦀️.rs`: `original_command_scope` is now a `CommandLease` (`Lease<LeaseKey>`), a held lease with a fixed FIFO of 8 waiters (no allocation).
  - `arbitrate`: the holder gets true. A free lease goes to the head of the queue only. Every other key is queued once and never overwrites the holder. A grant or queueing commits only when the paid claim is affordable, and the granting turn itself answers false, as before.
  - `release` / `leave` drop the hold and the queue slot, `clear` resets, `is_idle` is true with no holder and no waiter.
  - `claim_scope` stays for the single-lane host-event scope (`original_host_event_scope`); it has no second claimant, and its old test could not run (the empty decoder has no receiving identity, so the first claim failed before this change).
- Release on every completion, error and close path:
  - `P/🧵️instance/🔌️plugin-app-impl/🦀️.rs`: the six `handle_*` lanes run their body in an async block and call `settle_command_lease`, which releases on `Ok(Some(_))` and `Err(_)` and keeps the lease on `Ok(None)`. The task-resume branch releases on error or once resumes are not in flight and no typed command is pending. A maintenance turn whose lane holds the lease while no resume is runnable and no typed command is pending releases it (a lane that claimed and then lost its work cannot hold forever).
  - Close: `maintenance_source_terminal_is_empty` is false while the decoder owns or awaits the lease. `maintenance_source_close_step` releases it (`command_lease_release_ready`: owner with nothing pending, or a waiter) before any paid close branch, with zero demand. A cancelled or closed owner thus always passes its decoder through that drain, which releases. `close_ladder_step` clears the lease once no shared source is pending.
  - `🔒️close-terminal/🦀️.rs`: both instance terminal-empty lists (the conjunction and `close_blockers_debug`) require `original_command_scope.is_idle()`.
- Fairness: arrival order is the queue order, a lane that completes and re-claims goes to the back, so with two idle callers in reactor order A then B each completes at least every third round (A rounds 2, 5, 8; B rounds 3, 6).

### Test law

- `D/🧪️tests/🔣️.json` is rewritten language-agnostically as op scripts (`claim`, `poor`, `holds`, `queued`, `leave`, `idle`): held until released, foreign idle never overwrites, FIFO grants only the head, pending shared source blocks the grant, unaffordable claim changes nothing, dropped waiter leaves no slot, plus `fairness` (`rounds 8`, `eachCompletesWithin 4`, order a then b).
- `D/🧪️tests/🦀️.rs` (the only runner, there is no TS one) runs every script against `Lease<char>` and adds: the regression "two idle callers in reactor order A then B for 8 rounds: each completes within 4 rounds and at least twice", "a held lease is never overwritten in 32 rounds", and "a full queue refuses further lanes without disturbing the holder". The core is generic over the key, so the law needs no decoder fixture.

### Verification

Environment as requested: CARGO_INCREMENTAL=0, CARGO_PROFILE_DEV_DEBUG=0, `-j 4`, CARGO_TARGET_DIR=C:/b/scope-target (deleted afterwards; C: kept at 30 GB free). Scripts kept: `🗑️generated/scope-run.sh`, `scope-run2.sh`.

- `cargo test -p semio-plugin-instance-dispatch host_event_routing`: 4 passed, 0 failed.
- `cargo check -p semio-plugin-instance-app` and `-p semio-plugin-reactor-turn`: pass (they compile the async-block wrappers and the lease type change).
- `cargo check --target wasm32-wasip2` on `P/⚛️reactor/🔄️turn/📦️packages/🦀️rust/Cargo.toml`: finished without errors, including the wasm-only `[DEBUG]` probes in `manifest/🦀️.rs`, `manifest/metadata.rs`, `command-ingress-turn/🦀️.rs`, `poll-kernel-turn/🦀️.rs` and the reconcile-drive file. No probe edits were needed.
- No process of mine remains.

### Not covered

- No reactor-level test (fixture app, command past refresh plus a pending intent holding `cached_capture`, `exchange_generic` for 50 turns). Only the unit law and the compile gates ran. Confirm in the browser with the existing `[DEBUG] manifest pending` / `gate exchange.unapplied` probes: seq 7 must complete.
- A holder whose lane returns `Ok(None)` forever while waiting for something that only a queued lane can produce would now wait instead of being overwritten (residual mutual-wait). The analysis found no such dependency for seq 7 (refresh is done before the claim).

### Changed files

- `P/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs`
- `P/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🧪️tests/🦀️.rs`
- `P/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🧪️tests/🔣️.json`
- `P/🧵️instance/🦀️.rs` (field type)
- `P/🧵️instance/📥️staging/🚪️ingress/🦀️.rs` and `P/🧵️instance/🧱️construction/🧺️sources/🦀️.rs` (default init)
- `P/🧵️instance/🔌️plugin-app-impl/🦀️.rs`
- `P/🧵️instance/🔌️plugin-app-impl/🔒️close-terminal/🦀️.rs`
- `P/🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs`
- Script inputs: `🗑️generated/scope-run.sh`, `🗑️generated/scope-run2.sh`

## Fix: record action before close

### Cause

- `OriginalEffectReply::scope` writes the action key into its resident slot (a derived record's `route`, the clone owner's `close_action`) when the scope is created. Only a deposited reply clears it (`Forward::write`). A producer that returned `Err` after the scope existed (every `?` after `scope(...)` in `close_reply`, the advance routes and `RetainedCloneOwner::close_into`) therefore left the key set although no reply and no pending child existed. The derive's direct take branch (`self.route=Some(key)` then `take(...)?`) had the same hole.
- Sequence for seq 7: the manifest command's execution failed (`exchange_generic`, `plugin_exchange_turn` Err, `retain_decoded_exchange_failure`), the record's failed action kept `route`. `retire_failed_step_on_original` then closed the owner: `begin_close_into` refused with "record action must settle before close" (derive `🦀️.rs` 457-458), and because that refusal was again a `?` after the owner's own scope, `close_action` stayed `Some`: every later poll hit "clone owner has unresolved close action" (cause 2), so the actor stayed in `source-not-ready`.
- The original failure of the command itself is not visible in the pasted log (it was masked by the close refusal). Not proven statically; see probes.

### Fix

- `🌱️value/🎟️control/📨️reply/🦀️.rs`: `OriginalEffectReplyScope` now holds `Option<OriginalEffectReply>` and implements `Drop`. A scope dropped without `forwarded()` (producer refused), not recovering, not unwinding and unrecorded frees the key it created. `forwarded()` takes the reply out, so pending returns keep the key; recovered keys and panics keep theirs for the recovery callback. This fixes the unsettled record `route`, the clone owner's `close_action` and every other scope user at once; the check itself is untouched.
- `🌱️value/✨️derive/🧬️retained-clone/🦀️.rs`: the take branch clears the `route` it set itself when `take` refuses.
- Probe (wasm only): `🔄️turn/⌨️command-ingress-turn/🦀️.rs` logs `[DEBUG] gate exchange.retire-refused: seq N record close refused after the original command failure: ...` once per distinct state via `debug_state`. With the fix the close succeeds and the existing `[DEBUG] failed-decoded state ... first_failure {..}` and `[DEBUG] exchange_generic: command seq 7 refused: {..}` lines now show the real command failure (pattern `command seq 7 refused` / `first_failure`).

### Tests

`🌱️value/🎟️control/📨️reply/🧪️tests/🦀️.rs` and `cases.json` + `schema.json` (new law keys `refusedStepClearsKey`, `recoveredKeyRetained`): a refused producer leaves no key and a fresh close scope can begin; a recovered key survives a refused recovery and an unwind. `cargo test` on the reply package: 5 passed. No derive-level test (a full record fixture with failing child was out of scope).

### Verification

CARGO_INCREMENTAL=0, CARGO_PROFILE_DEV_DEBUG=0, `-j 4`, CARGO_TARGET_DIR=C:/b/close-target. `cargo check --target wasm32-wasip2` on the reactor turn package: finished without errors (wasm probes included). `C:/b/close-target` deleted.

### Changed files

- `🧰️framework/🔨️modules/🌱️value/🎟️control/📨️reply/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🎟️control/📨️reply/🧪️tests/🦀️.rs`, `cases.json`, `schema.json`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`
