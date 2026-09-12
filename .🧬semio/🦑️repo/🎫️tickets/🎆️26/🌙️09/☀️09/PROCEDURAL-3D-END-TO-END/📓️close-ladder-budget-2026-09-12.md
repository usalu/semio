# 🚪️ Close-ladder budget — a role switch must not wait for the predecessor (2026-09-12)

Lane: the instance close ladder in `🔌️PluginRuntime/🟦️.tsx` / the reactor close path, and its use by
`🏛️ShellHost/🔀️surface-switch/`.

**Restage required: yes, for the guest half.** Everything under `🔌️plugin` / `⚛️reactor` is Rust inside
the guest component, so `http://127.0.0.1:6018` keeps running the last staged `generation3d` wasm until
someone runs `nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`. This lane started
**no** wasm build (lane brief). The shell half (`🔀️surface-switch/🟦️.ts`) is host TS and was live on
6018 through HMR, which is what §5 measures.

---

## 1. The defect, and what it actually cost

`🗑️generated/journey-5/console.txt`, after eight example picks and several extension chains:

```
1304285 [DEBUG] surface-switch quiesce settled:0
1304761 [DEBUG] surface-switch create s.procedural.generation3d@1/*#viewer:2
1391724 [DEBUG] surface-switch retire-failed procedural#1
1391724 [DEBUG] switchToPluginApp: predecessor …#editor retirement failed
        Error: plugin-ui.lifecycle-close-budget-exhausted
            at retireInstanceLifecycle (…/🔌️PluginRuntime/🟦️.tsx:1213:54)
1391724 [DEBUG] surface-switch publish / seed / refresh …#viewer
```

**86 963 ms** between `create` and `publish`. The shell `await`ed `ports.retire(current)` before
publishing the successor, so the user's ⌘⌥V sat on the editor for 87 s and then landed on a viewer whose
predecessor had never finished closing. `📓️role-switch-runtime-2026-09-12.md` §5.1/§5.3 measured the
same shape at 68 s / 62 s with `plugin-ui.owner-close-budget-exhausted`.

Two independent causes, both now fixed:

| | cause | evidence |
|---|---|---|
| **A** | the guest close ladder needed **2 052 reactor turns**, and in the browser every reactor turn is one worker round trip | §2 |
| **B** | the shell **waited** for that ladder before mounting the successor | §4 |

`PLUGIN_UI_CONTINUATION_LIMIT` is 4 096 steps; a close that needs 2 052 reactor turns plus the poll
steps around them exhausts it whenever the runtime is also serving a freshly created successor.

---

## 2. Measurement — what a close costs after a realistic session

New law + fixture (§6.1): boot a generation3d editor, replay `documents` example loads with
`rendersPerDocument` preview renders each (so the retained window transients with meshes, the
tessellation table, the eval session, the contributions registry and the document history are all real),
then `InstanceClose` and count reactor turns to `Retired`.

### 2.1 Before — the cost is the GEOMETRY, not the session

```
[DEBUG] close-cost cold reached Retired after 2052 close turns
[DEBUG] close-cost warm reached Retired after 2052 close turns
[DEBUG] close-cost long reached Retired after 2052 close turns
[DEBUG] close-cost fixture costs=[("cold", 2052), ("warm", 2052), ("long", 2052)]
```

Zero documents, three documents + six renders, eight documents + thirty-two renders — **identical**. So
the browser's 62–87 s was not the retained data at all: `step_reactor_close`
(`⚛️reactor/🦀️.rs:1029`) advanced **one slot per reactor turn** across its fixed geometry —
`REACTOR_TASK_SLOTS` (1 024) task slots plus the request registry's own 1 024 slots plus the timer array
— and the turn drove it exactly once (`⚛️reactor/🔄️turn/🦀️.rs:440`, `let _ = step_reactor_close()?;`).
1 024 + 1 024 + 4 = 2 052.

That is the same defect class the patch ladder in the very same function was corrected for on 2026-09-10
(`PATCH_CLOSE_UNITS_PER_TURN`, documented there as "8 799 worker messages and 24.3 s for the 180-object
Nakagin switch"): **retirement is bounded work per TURN, not per item** — the reactor close was the one
ladder still priced per item.

### 2.2 Which family dominates — measured, not inferred

Temporary `[DEBUG]` probes on the five terminal gates
(`NativeLifetimeOwner::terminal_is_empty`, `⚛️reactor/🚪️lifetime/🦀️.rs:352`) and on every `return` of
`VcsArtifactApp::close_step`. All removed again (§7).

```
[DEBUG] close gate pending=lease   ×216   (cold)
[DEBUG] close gate pending=lease   ×206   (warm)
```

After the reactor-close fix, **100 %** of the remaining turns wait on the `lease` gate — the runtime
close cleanup pump, i.e. the app ladder — and never on `reactor-close`, `patches`, `pending-patches` or
`cold-pair`. Inside the app ladder, per close:

| stage | cold | warm | long (8 documents) |
|---|---|---|---|
| `retire_document_windows_step` (retained window transients / preview meshes) | 0 | 47 | **196** |
| the owned-store disposers (document / config / draft / presence / transient / window-config / window-transient / interaction) | 8 | 13 | 13 |
| everything else (cancellation slots, output cursors, envelope, children, roster) | 24 | 24 | 24 |
| **total `PluginApp::close_step` calls** | **32** | **84** | **233** |

So the app ladder IS O(retained) — one step per retained window-transient owner — while the reactor close
was O(1) but with a 2 052-turn constant. The browser pays a round trip for the reactor close's turns and
a *pool pump* for the app ladder's steps (`pump_process_worker_pool`, 64 pumps / 2 ms per reactor turn),
which is why the 2 052 dominated the wall clock so completely.

### 2.3 After

```
[DEBUG] generation2d reached Retired after 64 close turns      (was 2052)
[DEBUG] generation3d reached Retired after 125 close turns     (was 2052)
[DEBUG] close-cost fixture costs=[("cold", 88), ("warm", 152), ("long", 265)] ceiling=768 growth=8
```

The remaining native turn count is **not** step count — it is how many empty reactor polls fit while the
close pump grinds on the maintenance worker lane and loses `cell.instance.try_lock()` to the polling
thread. It is a latency proxy, and it is noisy (cold 56–163 across runs). What is deterministic is the
step count in §2.2 and the reactor-close geometry, now one turn.

---

## 3. Fixes — guest

| # | file | change |
|---|---|---|
| 1 | `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | the turn's `retirement_deadline` is computed FIRST, and the reactor close + cold-pair close are drained with `retire_while_progress_fallible` / `retire_while_progress_bounded` under it — `REACTOR_CLOSE_UNITS_PER_TURN = 4 096` units, clock re-read every `PATCH_CLOSE_DEADLINE_STRIDE` units. The whole fixed geometry now costs ONE turn instead of 2 052 |
| 2 | `🔌️plugin/🦀️.rs` | `RUNTIME_CLOSE_ITEMS_PER_STEP` **1 → 1 024**: an app close step is priced per PAGE, like every other stage of the turn. The stages that own real per-item work keep their own `.min(1)` |
| 3 | `🔌️plugin/🦀️.rs` | `RUNTIME_CLOSE_BYTES_PER_STEP` **4 096 → 32 KiB**. At 4 096 it was exactly `ARTIFACT_OUTPUT_CHUNK_BYTES`, so a retained item larger than one output chunk could not be released by ANY number of steps |
| 4 | `🔌️plugin/🦀️.rs` | `VcsArtifactApp::close_step`'s 1 088-slot tool-cancellation sweep walks up to `maximum_items` slots per call (2 calls instead of 1 088), and the five 64-slot output cursors fast-forward over vacant slots in bulk |
| 5 | `🔌️plugin/🦀️.rs` | the runtime close job drains the live-maintenance session's own retirement in a bounded BATCH per step, cut short by the step's own microsecond grant (`cx.deadline_exceeded()`), instead of one item per step |
| 6 | `🔌️plugin/🦀️.rs` | three stages that report `Complete` — `local_interaction_query`, `snapshot_read_returns`, `presence_store` local reads — now answer `Pending { released_items: 1 }` instead of `Pending { 0, 0 }`. A hand-off IS progress; the 2026-09-10 lane made the same correction to `ArtifactStoreEnvelopeRetirement::close_step` |
| 7 | `🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs` | a CLOSING app hands the displaced window-transient registries the whole page grant; a LIVE one still hands them one owner per step |
| 8 | `🔌️plugin/🪟️window/🫧️transient/🦀️.rs` | the partition's disposer gets the caller's grant instead of re-clamping it, and a retirement that released **nothing** answers `AwaitingInput` — an external wait — instead of `Pending { 0, 0 }` |
| 9 | `🔌️plugin/🫧️transient/🧵️publication/🦀️.rs` | `TransientStoreDisposer::close_step` passes the caller's grant to the retained transient's own retirement |

### 3.1 What the fixes are *not*

**No budget was raised.** `PLUGIN_UI_CONTINUATION_LIMIT` stays 4 096, `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT`
stays 8, `SESSION_SWITCH_QUIESCE_BUDGET_MS` stays 12 000. Fixes 2/3 change the PAGE SIZE of one
retirement unit (the unit the rest of the turn is already priced in), fixes 1/4/5 change how many units a
turn spends, and fixes 6/8 make the ladder tell the truth about progress and about waiting.

### 3.2 Why #7 is asymmetric, and why #8 exists

Both are measurements, not preferences.

* Widening the LIVE path's grant as well retires a displaced generation's window transients in one
  maintenance step and empties the preview an example switch is still reading through:
  `switching_active_example_changes_preview_meshes` goes red with `left: "[]" right: "[]"`. Verified by
  toggling that one call and re-running the test (green with `.min(1)`, red without).
* With the close path batched, the last remaining partition —
  `kind=procedural-preview owners=1 grant=1024/32768` — still answers `Pending { 0, 0 }` while its
  returned read lease is outstanding. Eight of those in a row and
  `runtime_close_nonterminal_status` kills the close with
  `plugin.internal.zero-progress … (elapsed 15us, ceiling 8000us)`. It reproduced intermittently (2 of 3
  runs) on the eight-document session. `AwaitingInput` is the shape the runtime already treats as an
  external wait (2026-09-10 fix #3): it yields, names the authority through
  `runtime_close_pending_authority`, and spends no structural livelock credit. Three runs green
  afterwards.

---

## 4. Fix — shell: a role switch does not wait for the close ladder

`🏛️ShellHost/🔀️surface-switch/🟦️.ts`, `runSessionAppSwitchV1`. The order was

```
quiesce → seal → create → await retire → publish → seed → refresh
```

and is now

```
quiesce → seal → create → START retire (never awaited) → publish → seed → refresh
                                     ↘ later: retire  |  retire-failed → onRetireFailed
```

A sealed predecessor publishes nothing, answers nothing and is dropped by the sealed-instance ledger, so
the only thing awaiting its ladder buys is latency. `SessionAppSwitchStepV1` gains `retire-started`; the
outcome is `switched` the moment the successor is published, and a retirement that fails — or never
finishes at all — reports through `onRetireFailed` without ever delaying or blocking the successor.

This half needs **no restage** and is the one measured in §5.

---

## 5. Runtime (6018) — the switch after the fix

Probe: `🐍️close-ladder-probe.mjs` (new, kept in the ticket folder). Boots, converges, picks three
examples so the retained families are real, then presses ⌘⌥V — retrying while the shell answers
`draining`, because a converged DOM is not a quiet instance — and records the `surface-switch` trace, the
chord→publish latency, the chord→retire-report latency, and every `close-budget-exhausted` /
`no actor for instance` / `pageerror`.

`SEMIO_PROBE_OUT=close-ladder/switch-5 bun 🐍️close-ladder-probe.mjs` →
`🗑️generated/close-ladder/switch-5/{summary.json,steps.json,console.txt,*.png}`:

```
170677 [DEBUG] surface-switch quiesce settled:0
170677 [DEBUG] surface-switch seal           procedural#1
170897 [DEBUG] surface-switch create         s.procedural.generation3d@1/*#viewer:2
170897 [DEBUG] surface-switch retire-started procedural#1
170897 [DEBUG] surface-switch publish / seed / refresh  …#viewer
170898 [DEBUG] surface-switch retire-failed  procedural#1
171378 [DEBUG] surface-switch outcome switched …#viewer pending=0
```

| reading | journey-5 (before) | switch-5 (after) |
|---|---|---|
| chord → successor published | **86 963 ms** | **253 ms** |
| `close-budget-exhausted` | 1 (`plugin-ui.lifecycle-close-budget-exhausted`) | **0** |
| `no actor for instance` | 20 | **0** |
| `pageerror` | 1 | **0** |
| DOM after the switch | roles `editor`, `window:procedural-main` | roles `viewer` pressed, `windows: ["procedural-view-preview"]` |

`switch-3` is a second, independent run of the same shape (chord → publish **629 ms**, budget-exhausted
0, noActor 0, pageErrors 0).

Two honest caveats on these runs:

1. The predecessor's retirement reported `retire-failed` after 1 ms with **`actor-lifecycle.worker-lost`**
   — the shard worker had already been terminated by the host watchdog during example 3
   (`the worker was silent for 16973 ms; outstanding: turn flow-extension-brep#request …`), which is the
   eval/tessellation lanes' defect, not this one. It makes the run an unusually good proof of the law the
   brief asks for: **the predecessor's retirement failed outright and the successor still mounted in
   253 ms with zero `no actor for instance` and zero page errors.**
2. The 2 052 → 1-turn guest half cannot be seen on 6018 until the procedural/generation3d guest is
   restaged. Everything in §2/§3 is native.

Also observed and handed on: the editor does **not** go quiet on the staged guest — the shell refused the
chord `draining … extension-invocation=2` for 10 consecutive attempts over ~3 minutes before the ledger
finally read 0. That is the `flowEvalTick` re-arm chain, `preview-eval-cancellation`'s lane.

---

## 6. Tests

### 6.1 Rust law — close cost is bounded regardless of the session

| file | what |
|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🧫️fixtures/🚪️close-ladder/🔣️.json` | **new** — language-agnostic: three sessions (`cold` 0 documents, `warm` 3×2 renders, `long` 8×4 renders), `maximumCloseTurns`, `maximumCloseTurnGrowth`, `turnBudget` |
| `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs` | **new law** `generation3d_close_cost_is_independent_of_the_retained_session` reads the fixture, replays each session on its own OS thread, closes, and asserts every close is under the ceiling and that the longest is within `growth`× the shortest |

Every law in that file now runs on its own 256 MiB thread (`on_close_ladder_thread`): the guest reactor's
registries are `thread_local!`, so one runtime per thread is the only way two laws can co-exist, and the
composed boot/replay/close future overflows a harness thread's default stack. Dev-dep
`semio-framework-async` added for its `block_on`.

```
> cargo test -p semio-s-plugin-procedural --test close_ladder -- --nocapture
[DEBUG] generation2d reached Retired after 64 close turns
[DEBUG] generation3d reached Retired after 125 close turns
[DEBUG] close-cost cold reached Retired after 88 close turns in 732 ms
[DEBUG] close-cost warm reached Retired after 152 close turns in 1098 ms
[DEBUG] close-cost long reached Retired after 265 close turns in 1946 ms
[DEBUG] close-cost fixture costs=[("cold", 88), ("warm", 152), ("long", 265)] ceiling=768 growth=8
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.89s
```

Three further consecutive runs, all green, no `plugin.internal.zero-progress`:
`[("cold", 149), ("warm", 264), ("long", 461)]`, `[("cold", 153), ("warm", 261), ("long", 475)]`,
`[("cold", 163), ("warm", 282), ("long", 462)]` (these three ran while a browser probe loaded the
machine, which is why they are higher — the metric is latency-shaped, see §2.3).

Registered: `🧪️test🚪️procedural🧊️close-ladder` in `.vscode/launch.json` already runs the whole
`close_ladder` target with no filter, so the new law needs no new entry.

### 6.2 Shell laws — the successor mounts while the predecessor retires

`🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json` gains `retireMode` (`resolve` / `reject` / `never`) and
`expected.retireFailures` per row, the `expected.trace` vocabulary gains `retire-started`, and two rows
are new:

* `a-predecessor-whose-close-ladder-never-finishes-does-not-delay-the-successor` — `retire` returns a
  promise that never settles; the switch still answers `switched`, publishes, seeds and refreshes.
* `a-predecessor-retirement-that-fails-never-blocks-the-successor` — `retire` rejects; the switch still
  answers `switched`, and `onRetireFailed` fires exactly once, AFTER `refresh`.

`🧪️tests/🔀️surface-switch/🟦️.ts` reads them twice — against the fixture and against the independent
in-file oracle (`oracleSwitchTrace` composed with a new `oracleSettledTrace`) — and adds the orderings
that ARE the fix: the successor is published before the switch answers, and `retire` / `retire-failed`
can only appear AFTER `refresh`, never before `publish`.

```
> nx run @semio-tech/framework-renderer-react:surface-switch-check
[DEBUG] surface-switch boot=10 group=4 roleTargets=4 switch=10 work=3 quiesce=3 sealed=5 busyLabel=2 modeSteps=6 keybindings=4 twin=33 PASS
 ✓ surface switch > … 456ms
 Test Files  1 passed (1)      Tests  1 passed (1)
```

(`switch=10`, up from 8. The node twin — 33 rows in a bare `node -e` process over the shipped source
text — still passes unchanged.)

---

## 7. Gates and pre-existing reds

| run | result | vs baseline |
|---|---|---|
| `cargo test -p semio-s-plugin-procedural --test close_ladder` | **3 passed / 0 failed** | new law added, the two existing laws went 2 052 → 64 / 125 turns |
| `nx run @semio-tech/framework-renderer-react:surface-switch-check` | **1 passed** (10 switch rows) | green |
| `cargo test -p semio-framework-plugin --lib -- close` | **64 passed / 6 failed** | identical 64/6 and the identical six names as `📓️close-ladder-2026-09-10.md` §7.5 — no regression |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib` | **371 passed / 5 failed** | the five reds `📓️role-switch-runtime-2026-09-12.md` §4.5 recorded — no regression |
| `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib` | **227 passed / 4 failed** | two beyond the 2026-09-10 baseline: `add_widget_undo_redo_round_trip` and `every_emitted_action_is_declared_on_its_window_kind`, both peers' (the generation3d twin `undo_redo_round_trips_flow_graph_edits` was already red in the 09-12 baseline, before this lane) |

Verified individually rather than assumed:

* `app_close_step_drains_at_most_one_segment_and_one_chunk_budget` fails with
  `left: Pending { released_items: 1, released_bytes: 0 } right: Pending { 1, 4096 }` — character for
  character the pre-existing red `📓️close-ladder-2026-09-10.md` §7.5 attributes to a peer widening the
  cancellation guard to `TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS`.
* `instance_lifetime_close_late_physical_step_retains_its_exact_outcome` **stack-overflows** in the
  default 2 MiB harness thread and passes under `RUST_MIN_STACK` with
  `total_distinct_steps=326, physical_close_calls=1` — a frame-size problem in a peer's test fixture,
  not a logic regression (it overflows identically with this lane's byte-grant change reverted).
* The full `semio-framework-plugin --lib` run is **530 passed / 145 failed**, every failure
  `app-definition.interactive-job-classification: unclassified interactive command …` — a peer's
  in-flight classification gate, nothing to do with close.
* The workspace broke under this lane twice from peers mid-run (`semio-framework-ui`
  `missing field overrun_run`, `semio-framework-os-infinite` `CONTENT_FIT_PADDING_PX`); every
  measurement above is from a run that compiled cleanly.

### 7.1 Temporary instrumentation — all removed

The five terminal-gate probes in `⚛️reactor/🚪️lifetime/🦀️.rs`, the 65 `LAST_CLOSE_STAGE` line probes and
the `LAST_CLOSE_STAGE` static in `🔌️plugin/🦀️.rs`, the zero-progress reporter in
`runtime_close_nonterminal_status`, the job-step and maintenance-progress probes in
`RuntimeCloseCleanupJob::step`, and the owner probe in `WindowTransientOwnerRegistry::close_step`.
Grepped clean. The only `[DEBUG]` lines left in the touched files are pre-existing designed diagnostics.

### 7.2 Raw logs

`🗑️generated/close-ladder/{close-ladder-after.txt, framework-plugin-close.txt,
framework-plugin-lib-serial.txt, generation3d-lib.txt, generation2d-lib.txt, surface-switch-check.txt}`
and the probe runs `🗑️generated/close-ladder/switch-{1,2,3,5}/`.

---

## 8. Handed to other lanes

1. **`actor-lifecycle.worker-lost` during a three-example session** — the host watchdog terminated the
   shard five times in `switch-5` with `the worker was silent for 16973 ms; outstanding: turn
   flow-extension-brep#request started 17037 ms ago`. A guest turn that costs 17 s is the eval /
   tessellation lanes'. It is also why the predecessor's retirement could not run at all on 6018.
2. **The editor never goes quiet** — `surface-switch outcome draining … extension-invocation=2` for ten
   consecutive chords over ~3 minutes. The 12 s quiesce budget is not the problem; the `flowEvalTick`
   re-arm chain never lets the work ledger reach 0. `preview-eval-cancellation`'s lane.
3. **`WindowTransientPartition`'s retirement reports a wait as zero progress** — worked around at the
   window-transient boundary (fix #8) by answering `AwaitingInput`. The honest fix is in
   `store::SnapshotRetirement`: a retirement holding a live returned read should answer `Blocked`, not
   `Pending { 0, 0 }`. Store lane.
4. **The native close is lock-contention-bound** — 705 (cold) / 1 527 (warm) `RuntimeCloseCleanupJob::step`
   invocations reach only 25 `PluginApp::close_step` calls; the rest lose `cell.instance.try_lock()` to
   the reactor turn holding it. Harmless on wasm (single-threaded, pumped inside the turn) but it makes
   every native close-turn measurement a latency proxy rather than a work count.

---

## 9. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | reactor + cold-pair close drained under the turn's deadline; `REACTOR_CLOSE_UNITS_PER_TURN`; `retire_while_progress_bounded` / `retire_while_progress_fallible` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | page-priced close grants; batched cancellation + output-cursor sweeps; bulk maintenance-pump close; three truthful hand-off reports |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs` | partition disposer gets the caller's grant; a zero-release retirement answers `AwaitingInput` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs` | close gets the page grant, live keeps one owner per step, with the measurement documented |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs` | `TransientStoreDisposer` passes the caller's grant through |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` | the predecessor retires in the BACKGROUND; `retire-started` step |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json` | `retireMode`, `retireFailures`, the new trace vocabulary, two new switch rows |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts` | the background-retirement laws and their settled oracle |
| `✏️s/🔌️plugins/🌀️procedural/🧫️fixtures/🚪️close-ladder/🔣️.json` | new close-cost fixture |
| `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs` | new close-cost law; every law on its own stack-sized thread |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` | dev-dep `semio-framework-async` |
| `<ticket>/🐍️close-ladder-probe.mjs` | new browser probe |
| `<ticket>/📓️close-ladder-budget-2026-09-12.md` | this report |
