# Fix 2026-09-16 — concurrent six-pane boot: load timeout and close-cleanup fault

Two independent defects made the `♻️mit-bestand/🧺️demonstrator` page lose two or three of its six
`FrameworkOsShell` panes to "Plugin Recovery — This program crashed", while every pane booted ALONE
reaches READY in ~40 s.

- **(a)** the plugin program loader timed a phase nothing was reporting progress for, so the panes that
  lost the connection race died on an idle window they were never idle in;
- **(b)** the Rust close-cleanup pump called a ladder "stalled" after eight fruitless STEPS regardless of
  how little time had passed, and the reactor turn that carries every instance of an actor died with that
  one retired instance's verdict.

---

## 1. Findings

### 1a. The program loader's progress/timeout contract

| what | where |
| --- | --- |
| idle rule | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:1759` (`pluginLoadRemainingMs`, re-armed `setTimeout`) |
| thrown message | same file `:1764` — `timeout loading ${pluginId} after … with no progress for 30000 ms` |
| budgets | `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts:490` — `SHARD_LIVENESS_POLICY.pluginLoadIdleTimeoutMs = 30_000`, `pluginLoadCeilingMs = 300_000` (schema-owned, `🧬️schema/🔣️.json` + `🧫️fixtures/🔣️.json`) |
| the only progress stamps that existed | `🔌️PluginRuntime/🟦️.tsx` — three `notePluginLoadProgress(pluginId)` calls around `fetchDescriptorManifest` / `registerManifest` / `getShardClient` |

**What counts as "progress" is the finding.** The timed region is `loadPluginModule`, whose only `await`
is the small descriptor JSON (`fetchDescriptorManifest`, `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:117`) — it
does **not** cover `WebAssembly.compile`, which happens later, inside the shard worker, on
`registry.activate` → `loadActor` (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts:529`, the
`await import(moduleUrl)` at `:542`).

So the 30 s of "no progress" was spent **inside `fetch(🔣️.json)`**, and the cause is contention, not a
dead worker:

1. Six panes boot 1.5 s / 35 s apart. Each `ShellHost` calls `loadPluginModuleResilient` independently.
2. Each shard worker that activates an actor does `await import(moduleUrl)` — a ≈210 MB `wasm-dev`
   component — saturating the browser's ~6 connections per origin for minutes on a load-25-100 machine.
3. Every pane ALSO issued its own request for the SAME `🔣️.json`, five of them redundant, each taking
   another connection.
4. A pane whose descriptor request was queued behind those transfers reported nothing for 30 s and was
   failed as dead — `program load failed demonstrator … Error: timeout loading demonstrator after
   30002 ms with no progress for 30000 ms`.

The shard worker was in fact beating the whole time: `postBeat("module-fetch"/"module-ready"/
"actor-ready")` plus a 1 s `progress` ticker (`🏗️materialization/🟦️.ts:358,371`), folded by
`ShardClient.recordHeartbeat` → `noteLiveness` (`📮️shard-client/🟦️.ts:2404,2408`). That proof reached
the shard watchdog and **nothing else** — the plugin-load clock never saw it.

**Module sharing:** a `WebAssembly.Module` is NOT recompiled per pane. Each shard worker realm imports
`moduleUrl` once (ES module cache), so the five panes cost **one compile per shard** (2 shards), not five.
Cross-realm module sharing would need structured-clone hand-off between workers; not attempted here — the
saving is the remaining 1 compile, and the boot failure was never caused by the compile count.

### 1b. The Rust close-cleanup pump

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.

| what | where (pre-fix) |
| --- | --- |
| the verdict | `runtime_close_nonterminal_status` (`:34071`) |
| the credit | `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT: u8 = 8` (`:33608`) — a COUNT, with no time term at all |
| the message | `runtime_cleanup_fault` (`:32961`), fed `state.last_callback_elapsed_us` |
| the ladder | `RuntimeCloseCleanupJob::step` (`:33915`) and `RuntimeLiveCleanupPump::close_step` (`:33624`) |
| the emit site | `plugin_step_close_cleanup` (`:34483`) |
| the propagation | `⚛️reactor/🔄️turn/🦀️.rs:561` — `plugin_step_close_cleanup(runtime)?` |

Why the message read `elapsed 0us, ceiling 8000us`:

1. **The two numbers in the message are unrelated to the verdict.** `elapsed` is the last turn's cost and
   `ceiling` is `INTERACTIVE_STEP_CEILING_US`; the verdict was made purely from a step COUNT. Eight steps
   that each observe an empty ladder cost microseconds, so the credit read exhausted at `elapsed 0us` —
   exactly the "judged exhausted before any time elapsed" symptom. **A count is not a duration.**
2. **Real structural progress was priced as zero progress.** `RuntimeLiveCleanupPump::close_step` mapped
   `JobPayloadCloseStep::Complete` / `WorkerJobCloseStep::Complete` — "this sub-owner is retired, move
   on" — to `Pending { released_items: 0, released_bytes: 0 }`, which `runtime_close_nonterminal_status`
   reads as a stalled step. Retiring the outcome, then the rejection, then the session each bought one
   unit of stall debt for work that actually happened.
3. **A step that recorded nothing at all bought stall debt too.** `progress.is_none()` was folded into
   `zero_progress`. `None` means the step never reached the ladder (cancelled, weak-upgrade miss) — an
   absence of evidence, and already terminal through `PriorOutcome`. The live-maintenance twin
   (`runtime_live_cleanup_nonterminal_status`, `:33804`) had always excluded `None`; the close twin was
   the outlier.
4. **The entry never left quarantine on a fault**, so the same dead lifetime would re-raise its verdict on
   every later cursor pass.
5. **The verdict killed the shared turn.** `plugin_step_close_cleanup(runtime)?` in the reactor turn made
   instance 5's close fault the WHOLE turn's fault. The host reported `shard N worker fault
   [handler/turn]`, `PluginRuntime: actor demonstrator#5 trapped`, and instance 6 — a sibling pane
   mid-boot in the same shard — went to Plugin Recovery with it.

The boot retry after (a) is what opened a close during activation in the first place, which is why the two
defects always appeared together.

---

## 2. Design

**(b) Rust.** The zero-progress verdict is now priced in both currencies it claims: `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT`
consecutive **observed** `Pending { 0, 0 }` steps AND `RUNTIME_CLOSE_STALL_CREDIT_US` (= `INTERACTIVE_STEP_CEILING_US`,
8 000 µs) of real elapsed time since the ladder last moved. `stall_since_us` carries the instant the run began;
`stall_credit_spent_us` carries what was actually spent, so the emitted message quotes a measured stall instead of an
unrelated turn cost. A cleanup with nothing left to retire completes: structural retirement of a sub-owner reports one
released item, and a step that read nothing clears no ladder and buys no debt. A faulted close leaves quarantine with
its verdict, and the reactor turn contains any fault whose code belongs to the cleanup taxonomy
(`runtime_cleanup_fault_is_instance_scoped`) instead of dying with it — anything else (a busy runtime authority, a
poisoned borrow) still ends the turn.

**(a) TypeScript.** The plugin-load liveness clock moves into its own React-free leaf,
`🔌️PluginRuntime/🫀️load-progress/🟦️.ts`, so its accounting is testable without a `Worker`. Three sources now feed it:

1. **fetch bytes** — `fetchDescriptorManifest` streams the body through `response.body.getReader()` and beats on the
   headers and every chunk (falling back to `response.text()` when there is no reader).
2. **shard beats** — `ShardClientOptions.onLiveness` fires for every proof of life this client folds in, and
   `buildShardClientOptions` routes it to `notePluginLoadProgressForInFlightV1`, which stamps every load currently in
   flight. A beat through a worker's `await import()` is the only proof a starved load has that the pipeline it is
   queued behind is alive. `pluginLoadCeilingMs` (300 s) still bounds the attempt, so forgiveness is not unbounded.
3. **one descriptor request per (plugin, url)** — `sharedDescriptorManifestV1` collapses the six panes' identical
   `🔣️.json` requests into one, returning five connections to the module fetches. Keyed by plugin as well as url so the
   fetch's own per-caller identity check can never be bypassed.

---

## 3. Edits

### Rust

| file:line | change |
| --- | --- |
| `🔌️plugin/🦀️.rs:32970` | `runtime_cleanup_fault_is_instance_scoped` — new; the reactor turn's containment predicate |
| `🔌️plugin/🦀️.rs:32990` | `RuntimeCloseWorkerState` gains `stall_since_us` / `stall_credit_spent_us` |
| `🔌️plugin/🦀️.rs:33634` | `RUNTIME_CLOSE_STALL_CREDIT_US` — new, `= INTERACTIVE_STEP_CEILING_US` |
| `🔌️plugin/🦀️.rs:33637-33672` | `RuntimeLiveCleanupPump::close_step` reports a retired sub-owner as `Pending { 1, 0 }`, not `Pending { 0, 0 }` |
| `🔌️plugin/🦀️.rs:34116` | `runtime_close_nonterminal_status` → `runtime_close_stall_verdict`: `None` no longer buys debt; fault needs count AND elapsed; returns the credit spent |
| `🔌️plugin/🦀️.rs:34322` | call site stores the spent credit before recording the fault |
| `🔌️plugin/🦀️.rs:34512` | `plugin_step_close_cleanup` takes the faulted entry out of quarantine and quotes the stall credit for `ZeroProgress` |
| `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:563` | a close-cleanup verdict no longer aborts the turn every sibling instance shares |
| `🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:34` | fixture state gains the two new atoms |
| `🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs` | rewritten for the new verdict + 4 new laws |

### TypeScript

| file:line | change |
| --- | --- |
| `🔌️PluginRuntime/🫀️load-progress/🟦️.ts` | **new** — the whole load-liveness clock (deadline rule, progress ledger, in-flight roster, shared descriptor request) |
| `🔌️PluginRuntime/🟦️.tsx:489` | `buildShardClientOptions` gains `onLiveness` → `notePluginLoadProgressForInFlightV1` |
| `🔌️PluginRuntime/🟦️.tsx:510` | the inline progress ledger is replaced by an import + re-export of the leaf |
| `🔌️PluginRuntime/🟦️.tsx:2227` | `loadPluginModule`'s prologue runs under `withPluginLoadInFlightV1`, through `sharedDescriptorManifestV1`, with the fetch beating into the roster |
| `🛠️ShellHelpers/🟦️.tsx:198,1709` | imports the rule from the leaf instead of owning a copy |
| `🎭️actor/📮️shard-client/🟦️.ts:990,1058,1083,2412` | `ShardClientOptions.onLiveness`, fired from `noteLiveness` (the one funnel every beat path reaches) |
| `🎠️kernel/🟦️.ts:117` | `fetchDescriptorManifest` gains `onProgress`; `readDescriptorText` streams the body per chunk |
| `⚛️react/🧪️tests/🎚️config/🟦️.ts:73` | registers the new suite (an unregistered suite is a gate that reads green measuring nothing) |
| `🧑‍🎨engine/🧪️tests/🫀️plugin-load-progress/🟦️.ts` | **new** — 16 laws |

### Incidental repair

`🔌️PluginRuntime/🟦️.tsx` lines 1450 and 4058 carried **raw NUL bytes** inside template-literal composite keys
(`` `${actorId}\0${coalesceKey}` ``), which is why `grep` treats the file as binary and needs `-a`. An `awk` pass in
this session turned those NULs into line breaks, truncating both lines; both were restored byte-exact. Worth a
follow-up to replace the raw bytes with ` ` escapes.

---

## 4. Commands

```
cargo check --target wasm32-wasip2 -p semio-framework-plugin                      # clean
cargo test  -p semio-framework-plugin --lib -- instance_lifetime_close runtime_cleanup_fault_vector runtime_close_budget
SEMIO_TEST_LEVEL=long      bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts plugin-load-progress
SEMIO_TEST_LEVEL=long      bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts window-fault engine-contract router-plugin-faults ShellHelpers
SEMIO_TEST_LEVEL=exhaustive bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts PluginRuntime
bunx tsc --noEmit                                                                 # in ⚛️react/📦️packages/🟦️typescript
bun nx run @semio-tech/demonstrator-plugin:materialize-dev                         # 1m37s, staged
bun nx run @semio-tech/mit-bestand-demonstrator:activate-dev                       # 6m28s, 28 components
```

Logs under `🗑️generated/close-fault-*-2026-09-16.txt`.

## 5. Before / after

| gate | before | after |
| --- | --- | --- |
| `runtime_close_budget_tests` (native) | 6 passed | **10 passed** (4 new laws) |
| close-path native selection (`instance_lifetime_close` + fault vector + close budget) | 20 passed | **24 passed, 0 failed** |
| `🫀️plugin-load-progress` (vitest) | did not exist | **16 passed** |
| `🩺️window-fault`, `🧯️router-plugin-faults`, `🛠️ShellHelpers` suites | pass | pass (unchanged) |
| `bunx tsc --noEmit` on the react package | 459 pre-existing errors | **459** — none in any touched file |
| `cargo check --target wasm32-wasip2 -p semio-framework-plugin` | clean | clean |

**Pre-existing failures, NOT from this work** (peers are mid-refactor in the same files — `🔌️plugin/🦀️.rs`
was already `MM` at session start):

- `component::app::app_builder_tests::*` (7) — `try_build_definition` panics through `build_definition`
  (`🦀️.rs:5671`); and every `*_dispatch_*` builder-contract test fails with
  `interactive-job.missing-factory: typed command 'amendLabel' has no exact controller/owner/factory/tool/schema proof`.
  A peer's factory-proof migration.
- The full `cargo test -p semio-framework-plugin --lib` run aborts (SIGABRT) in
  `NativeLifecycleRegistry::drop` — "runtime lifetimes require terminal exact ACK before teardown". The test
  that aborts **differs between runs** and each passes in isolation: load-dependent, pre-existing.
- `🔬️engine-contract` 9 failures (ink canvas host, tutorial tracks, catalog kinds, projection pane styling,
  `generation3d-generate-preview` action scoping) — all UI/descriptor work owned by peers.
- `🔌️PluginRuntime` in-source: `host effect address decoding` / `leftover brush guest hover retain` — a
  different one fails per run, neither touches load progress.

## 6. What the coordinator should verify in the browser

1. Boot `♻️mit-bestand/🧺️demonstrator` with all six panes on the loaded machine.
2. **`timeout loading demonstrator after … with no progress for 30000 ms` must not appear at all.** A slow
   pane should now sit in `installing` for as long as the shard keeps beating, up to the 300 s ceiling.
3. **`plugin.internal.zero-progress` must not appear with `elapsed 0us`.** If a genuine stall ever occurs the
   message now quotes a measured credit ≥ 8000 µs.
4. If a close does fault, confirm it no longer reads as `shard N worker fault [handler/turn]` and no longer
   trails `PluginRuntime: actor demonstrator#N trapped` — expect instead the contained line
   `[DEBUG] close cleanup fault contained to its own retired lifetime: plugin.internal.… ` and the SIBLING
   panes staying live.
5. In the Network panel, confirm exactly **one** request for `🔌️plugin-modules/🎪️demonstrator/🔣️.json`, not six.
6. Expected outcome: **six of six panes READY**. If a pane still fails, capture the console and say which of
   (2)/(3)/(4) it matched — they are now three distinct, separable signatures.
