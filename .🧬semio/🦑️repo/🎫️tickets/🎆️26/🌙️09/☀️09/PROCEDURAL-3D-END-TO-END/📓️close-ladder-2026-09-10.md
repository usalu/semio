# 🚪️ Instance close ladder — generation3d / generation2d reach `Retired` (2026-09-10)

Lane: the instance close ladder (`VcsArtifactApp::close_step`, `PluginApp::close_step`, the
reactor-close / cold-pair / lifecycle drop authorities, `lease.is_retired()`) plus generation3d's
window-transient close ladder.

**Restage required: yes.** Every change is Rust inside the guest component (`🔌️plugin`, `🏪️store`,
`⚛️reactor`, both procedural editors), so the browser only sees it after
`nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`. This lane ran **no** `activate`
of its own — the flow-eval-tick lane's restage was in flight on the shared target.

## 1. Reproduction

Two independent native reproductions.

### 1.1 Reactor level — the new law

`✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs` (`[[test]] close_ladder`, registered in
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`):

1. installs the real bundle and opens the generation3d editor through `Event::InstanceOpen`, ACKs
   `Captured`;
2. loads the bundled `hexagonal-mushroom-column` DSL through `plugin_load_document_text` (paired with
   the booted instance's own `ops` text — see §6.1);
3. renders `procedural.play.preview` four times under a real `ViewModel`, interleaved with reactor
   turns, so the retained `FlowEvalSession`, the per-window transient partitions and the in-flight
   tessellation table are all live at close time;
4. publishes `Event::InstanceClose`, ACKs `Accepted`, then drives empty turns and requires a
   `Retired` receipt inside a bounded turn count, followed by the terminal ACK releasing the
   structural owner — with no aborted process.

The same law runs for generation2d (one ladder, two artifacts).

**Before the fix** (`🗑️generated/` was swept per this lane's brief; raw tails are in §7):

```
test generation2d_instance_close_reaches_retired ...
panicked at 🧪️tests/🚪️close-ladder/🦀️.rs:95:
  generation2d close ladder must reach Retired within 4096 reactor turns
panicked at ⚛️reactor/🚪️lifetime/🦀️.rs:482:
  runtime lifetimes require terminal exact ACK before teardown
panic in a destructor during cleanup … (signal: 6, SIGABRT)
```

generation3d, once a real document was loaded, died differently and earlier:

```
procedural close-ladder turn: Fault { code: "plugin.internal.zero-progress",
  message: "runtime close cleanup faulted for instance 7: the cleanup pump made no progress
            for its whole stall credit [zero-progress] (elapsed 22us, ceiling 8000us)" }
```

and, when the fault was observed from the other side first,

```
procedural close-ladder turn: Fault { code: "plugin.reactor-close-authority",
  message: "native close terminal unavailable" }
```

— exactly the message boot #6 saw on a `[handler/turn]`.

### 1.2 App level

`semio_framework_plugin::testkit::close_registered_fixture_app` on the registered generation3d editor
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs`,
`preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app`) aborted with
`registered fixture close blocked: transient read remains live`.

## 2. Where the ladder stalls — measured, not inferred

Instrumented with temporary `[DEBUG]` probes (all removed again; see §5) at each authority in turn.

### 2.1 `prepare_retired` → which lifecycle gate

```
[DEBUG] prepare_retired turn=4    phase=Accepted not-closing
[DEBUG] prepare_retired turn=8    phase=Closing owner-not-terminal
…
[DEBUG] prepare_retired turn=8192 phase=Closing owner-not-terminal
```

`GuestLifecycleCell::prepare_retired` reached `Phase::Closing` and then hung on
`NativeLifetimeOwner::terminal_is_empty`.

### 2.2 Which of the five terminal gates

```
[DEBUG] native terminal gate pending turn=1..128 gate=lease
[DEBUG] native terminal gate pending turn=256..4096 gate=reactor-close
```

The app-instance lease (`lease.is_retired()`) **cleared** around turn ~130 — the app close ladder was
never the blocker for a bare instance. The **reactor-close** gate
(`⚛️reactor/🚪️lifetime/🦀️.rs:349`, `super::reactor_close_complete(self.key)`) stayed pending forever.

### 2.3 Which reactor-close stage

```
[DEBUG] step_reactor_close turn=1024 ingress=true requests=false … task_cursor=0 …
[DEBUG] step_reactor_close turn=2048 ingress=true requests=true resumes=true task_cursor=0 …
[DEBUG] step_reactor_close turn=4096 ingress=true requests=true resumes=true task_cursor=0 …
```

**Stall point #1 — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1019`**
(`step_reactor_close`, task-cancellation stage). It looped on

```rust
} else if state.task_cursor < REACTOR_TASK_SLOTS {
    cancel_instance_tasks_step(state.instance, &mut state.task_cursor);   // return value DISCARDED
    false
}
```

`cancel_instance_tasks_step`'s **return value is the completion witness**, and the production
(`#[cfg(not(test))]`) body delegates to
`⚛️reactor/🧵️executor/🦀️.rs:227` (`LocalReactorExecutor::close_instance_step`), which answers
`ReactorTaskStep::Complete` **without advancing the cursor** the moment its sweep has nothing left to
visit. The cursor therefore never reaches `REACTOR_TASK_SLOTS`, `state.complete` is never set,
`reactor_close_complete` never returns `true`, no `Retired` receipt is ever published, and
`NativeLifecycleRegistry::drop` (`⚛️reactor/🚪️lifetime/🦀️.rs:482`) aborts the process at teardown.

The `#[cfg(test)]` body of `cancel_instance_tasks_step` advances the cursor itself, which is exactly
why `reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps` was green the
whole time and the defect was invisible to every in-crate law.

### 2.4 Stall point #2 — the structural livelock accountant vs. truthful handoff steps

With #1 fixed, generation2d retired in 2052 turns but generation3d — the instance that had actually
loaded a document — faulted with `plugin.internal.zero-progress`. Probing the app ladder:

```
[DEBUG] close stage … owned_stage=0 retained_empty=false          (×8, unchanged)
[DEBUG] cursor disposer phase=Displaced active=false items=1 bytes=4096   (×8)
[DEBUG] displaced owner zero progress
        owner=…::ArtifactStoreEnvelopeRetirement<Generation3dSnapshot, Generation3dMutation>   (×8)
```

**Stall point #2 — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:1352`**
(`ArtifactStoreEnvelopeRetirement::close_step`). Every handoff (mutation → `active`, edit → `active`,
change/checkpoint/alternative/message/conflict/metadata-string → `active`, initial snapshot →
`active`) and every phase advance returned
`SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }` — the *exact* signal the
runtime's structural livelock accountant (`runtime_close_nonterminal_status`,
`RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT = 8`) reads as "this ladder is livelocked". A displaced envelope
carrying a real document walks far more than eight consecutive handoff steps, so a truthful ladder was
killed as a livelocked one. `PluginInstanceCloseLease::is_retired`
(`🔌️plugin/🚪️lifetime/🦀️.rs:42`) then turns that status into `Err`, which
`NativeLifetimeOwner::terminal_is_empty` flattens to the fixed string `native close terminal
unavailable` — the message boot #6 reported.

Note the cost asymmetry that made this mis-scaling invisible: the **live** cleanup accountant grants
`RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT = 256`, the **close** accountant 8, while the close ladder is
the one that necessarily walks long handoff chains.

### 2.5 Stall point #3 — a pending close authority was a fault, not a wait

`RuntimeCloseCleanupJob::step` turned `PluginCloseStep::Blocked` and `PluginCloseStep::AwaitingInput`
into `StepOutcome::Fault`, which the pump promotes to `RuntimeCleanupFault::PriorOutcome` and
`is_retired()` reports as `native close terminal unavailable`. The **live** cleanup job for the same
two variants has always answered `StepOutcome::Yield`. So any authority that is merely not-yet-
releasable — a window-transient partition whose snapshot read is still live, a store waiting on a
returned reader — aborted the instance instead of being waited on.

The same asymmetry existed in `testkit::close_registered_fixture_app`, which `panic!`ed on `Blocked`.

## 3. Fixes

| # | file | change |
|---|---|---|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` | `ReactorCloseState` gains `tasks_complete`; the task stage now terminates on `cancel_instance_tasks_step`'s **own** witness instead of on a cursor the production executor stops advancing |
| 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | `ArtifactStoreEnvelopeRetirement::close_step` reports `released_items: 1` for every handoff and phase advance — one retained owner really did cross the close boundary, the same convention `ArtifactStoreCursorDisposer` already uses; documented on the impl |
| 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | the runtime close job answers `StepOutcome::Yield` for `Blocked`/`AwaitingInput` (mirroring the live cleanup job) and `runtime_close_nonterminal_status` maps a pending authority to `RuntimeCloseStatus::ExternalWait` **without** spending structural livelock credit; a rate-limited `[DEBUG] runtime close pending authority` line names the authority that is holding the close |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`testkit`) | `close_registered_fixture_app` keeps stepping on `Blocked`/`AwaitingInput` and reports the last pending authority in its terminal assertion instead of panicking on the first one |
| 5 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs` and the generation2d twin | `Generation{3,2}dInstanceOperationOwner::maintenance_step` no longer drives `FlowEvalSession::close_step` while the app is **live**: a session that has not begun closing owns nothing retirable, and `FlowEvalSession::close_step` answers `Blocked` until `begin_close`, so the owner reported `Blocked` on every idle maintenance turn and spent the live pump's zero-progress credit. It now answers `Complete` while live and `Pending{0,0}` for an empty grant |
| 6 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `preview_eval_exact_window_transient_…` scopes the two `WindowTransientSnapshot` reads it asserts on, so the test no longer holds live window-transient read leases across the close it then asserts on |
| 7 | `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧫️fixtures/🔣️.json` | `outbound` `cancelJob` gains its `requestId` field, in declaration order, matching `📮️shard-client/🟦️.ts:367` |

### 3.1 What the fixes are *not*

No threshold was raised. `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT` stays 8; fix #2 makes the ladder tell the
truth instead of widening the tolerance for a lie.

## 4. Tests

| law | file | status |
|---|---|---|
| `generation3d_instance_close_reaches_retired` | `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs` | **new, green** |
| `generation2d_instance_close_reaches_retired` | same | **new, green** |
| `pending_close_authority_waits_without_consuming_structural_close_credit` | `🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` | **new, green** |
| `reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps` | `⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` | extended with the `tasks_complete` witness assertion, green |
| `reactor_native_lifecycle_retains_exact_close_until_ack` | `⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs` | green |
| `structural_zero_progress_exhausts_its_exact_close_credit` | close-budget | green |
| `preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` | generation3d unit | **red → green** |
| `ActorWorkerInboxInventory` (both laws) | `🎭️actor/📤️return/📨️response/🧪️tests/…/🟦️.ts` | **red → green** (2 passed) |

The `cfg(test)` reactor law cannot by construction catch stall #1 (its `cancel_instance_tasks_step`
body advances the cursor itself). The production regression law is `close_ladder`, which links a
non-test framework — that is why it lives in the plugin crate's public test surface.

## 5. Temporary instrumentation

Every probe added while bisecting was removed again: the `prepare_retired` / terminal-gate probes and
the `Phase: Debug` derive in `⚛️reactor/🚪️lifetime/🦀️.rs`, the `step_reactor_close` stage probe in
`⚛️reactor/🦀️.rs`, the 18 `close_stall_probe` sites, the `close_stall_report` method and the live-pump
probe in `🔌️plugin/🦀️.rs`, and the `ErasedSnapshotRetirement::debug_type` + cursor/displaced probes in
`🏪️store/🦀️.rs`. The only `[DEBUG]` line kept is the rate-limited `runtime close pending authority`
reporter introduced by fix #3, which is a designed diagnostic, not a bisecting aid.

## 6. Findings handed to other lanes

### 6.1 `plugin_load_document_text` with empty `ops` aborts the process

`store::replay_ops` (`🏪️store/🦀️.rs:11661`) drops its `initial_snapshot` un-retired on the error path,
so a rejected `ArtifactTextFiles { dsl, ops: String::new() }` aborts with

```
ordered-map root must be explicitly retired before drop
  at 🧰️framework/🔨️modules/📡️replication/🌱️value/🗂️ordered/🦀️.rs:81
  in os_store::replay_ops::{closure#0} ← parse_document_text ← load_document_text
```

This lane worked around it by pairing the example DSL with the booted instance's own `ops` text. The
error-path retirement in `replay_ops`/`parse_document_text` is the store lane's — this is the
`load_document_text` twin of the `load_document_pack` defect the flow-host lane fixed in its §4.

### 6.2 A close costs ~2 052 reactor turns

Both artifacts reach `Retired` in **2 052** close turns. `step_reactor_close` advances **one** unit per
reactor turn across three ~1 024-slot sweeps (requests, tasks, timers), and the app close ladder walks
`TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS = 1 088` cancellation slots plus five 64-slot
output cursors one item at a time. It is bounded and abort-free, but closing or replacing a document
in the browser costs thousands of turns. Batching those O(1) cursor sweeps under the existing 8 ms
budget is a separate interaction-friendliness change and was deliberately left out of this lane.

### 6.3 `generation_preview_is_one_app_transient_shared_by_two_generation_windows`

No longer a window-transient close red. Its close now succeeds; it fails earlier, in
`drive_preview_operation`, with `Generation3d preview operation did not finish` (30 s) — the same
typed-operation completion class as `refresh_pending_effects_arms_flow_eval_tick_chain`. Hot-path /
JSON-512 lanes.

## 7. Gates

### 7.1 `cargo check -p semio-s-plugin-procedural --keep-going` (native)

```
    Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 2m 10s
```

0 errors.

### 7.2 `close_ladder`

```
running 2 tests
test generation2d_instance_close_reaches_retired ... [DEBUG] generation2d reached Retired after 2052 close turns
test generation3d_instance_close_reaches_retired ... [DEBUG] generation3d reached Retired after 2052 close turns
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 67.18s
```

### 7.3 generation3d suite

| run | result |
|---|---|
| lane hand-over baseline (`📓️flow-host-ownership-2026-09-10.md` §6.1) | 315 passed / 5 failed (320) |
| **after this lane** | **326 passed / 4 failed** (330) |

The four reds, none this lane's and all previously attributed:

| test | owner |
|---|---|
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` — now `Generation3d preview operation did not finish` | hot-path / JSON-512 (see §6.3) |
| `refresh_pending_effects_arms_flow_eval_tick_chain` — `typed operation did not retire within 30 seconds` | hot-path |
| `two_instances_converge_disjoint_widget_moves` — `module.vcs … remote snapshot merge is fail-closed` | excluded by this lane's brief |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` — `left: Fault, right: Ready` | envelope-load lane |

### 7.4 generation2d suite

**228 passed / 2 failed** (230) — the identical two the flow-host lane measured
(`two_instances_converge_disjoint_widget_moves`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`), unchanged.

### 7.5 `semio-framework-plugin --lib -- close`

**64 passed / 6 failed.** All four of this lane's target laws are green (§4). The six reds are the
concurrent fixture-separation wave's, not this lane's:

- `app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal` and
  `app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty` are already
  recorded as pre-existing peer reds in `📓️envelope-load-2026-09-10.md` §5.2.
- `app_close_step_drains_at_most_one_segment_and_one_chunk_budget` sets
  `close_cancellation_cursor = TOOL_CANCELLATION_SLOTS` and expects the segment stage next, but a peer
  widened the ladder's guard to `< TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS`, so 64
  cancellation steps now run first — exactly the observed `Pending { 1, 0 }`.
- `instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot` drives the
  `#[cfg(test)]` helper `reactor::cancel_instance_tasks`, which this lane did not touch;
  `step_reactor_close` is not on its path.
- `local_interaction_registered_query_channel_continuation_ack_and_close` (`presence.is_empty()`) and
  `peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority` (`stale roster
  outcome`) are presence/roster authorities this lane never touched.

The workspace broke under this lane three times from peers mid-run
(`flow_extension_plugin_id` removed, `PreviewTessellatePhase::Faulted` non-exhaustive,
`interactive-job.catalog-incomplete` on generation2d's command catalog); every measurement above is
from a run that compiled cleanly.

### 7.6 `cargo check -p semio-s-plugin-procedural --keep-going --target wasm32-wasip2 --profile wasm-dev`

With `CARGO_PROFILE_WASM_DEV_DEBUG=false`:

```
    Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `wasm-dev` profile [unoptimized] target(s) in 1m 46s
```

0 errors (the only warnings are a peer's `unused_qualifications` / `unused extern crate` sweep in the
two artifact crates).

### 7.7 Raw logs

`🗑️generated/close-ladder-pass.txt`, `close-suites.txt`, `close-framework-plugin.txt`,
`close-wasm-check.txt`, `close-inbox-inventory.txt`, `close-repro-before.txt`.

### 7.8 `launch.json`

`🧪️test🚪️procedural🧊️close-ladder` registered in `.vscode/🧩️launch.seed.jsonc` and
`.vscode/launch.json`, next to `🧪️test⏱️procedural🧊️boot-deadline`.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` | `ReactorCloseState::tasks_complete`; the task stage terminates on the sweep's own witness |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | `ArtifactStoreEnvelopeRetirement::close_step` reports one released owner per handoff/phase advance, with the convention documented on the impl |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | runtime close job yields on `Blocked`/`AwaitingInput`; `runtime_close_nonterminal_status` → `ExternalWait` without spending livelock credit; `runtime_close_pending_authority` diagnostic; `testkit::close_registered_fixture_app` keeps stepping |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` | new law `pending_close_authority_waits_without_consuming_structural_close_credit` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` | the bounded reactor-close law asserts the task sweep's own completion witness |
| `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧫️fixtures/🔣️.json` | `outbound` `cancelJob` gains `requestId` |
| `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs` | new — the instance close law for both procedural editors |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` | `[[test]] close_ladder`; dev-dep `serde_json` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | `Generation3dInstanceOperationOwner::maintenance_step` is a no-op while live |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | the generation2d twin |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | the window-transient law scopes its two snapshot read leases |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | `🧪️test🚪️procedural🧊️close-ladder` entry |
