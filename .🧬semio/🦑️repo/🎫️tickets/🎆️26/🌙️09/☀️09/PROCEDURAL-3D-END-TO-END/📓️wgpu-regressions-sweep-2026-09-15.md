# 🧹️ wgpu regressions sweep — lane `wgpu-regressions-sweep` (2026-09-15) · ⏸️ **PAUSED**

> ⏸️ **This lane was stopped mid-flight at 13:05 by a user directive relayed through the coordinator:
> the wgpu renderer is ON HOLD until the React renderer is butter smooth end to end.** No battery was
> run by this lane. Everything below is either a Rust/TS law that was RUN, or a `[DEBUG]` trace read
> off a console on 6118 — nothing is inferred. §7 says exactly what is unfinished.
>
> Every crate this lane touched compiles, native and `wasm32-unknown-unknown` (§6), and the renderer
> wasm serving 6118 was rebuilt from these sources at **13:00** (`dist/wasm-dev`, 81 993 064 B).

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`. Target: the coordinator's wgpu serve,
`http://127.0.0.1:6118/?plugin=generation3d`. Repo MCP was down all session
(`repo CONNECTION_CLOSED`, `semio CONNECTION_CLOSED`); no ticket was opened, closed or reopened. No
`git commit/stash/checkout/worktree`. Nothing under `🗑️generated` that this lane did not create was
touched. No peer process was killed.

---

## 1. TL;DR — five reds, three root causes, all three found

| ask | verdict |
|---|---|
| `chrome` 4/5 — the world3d cancel control is never declared | **root cause found and fixed**: the wgpu bridge addressed EVERY re-armed host effect as an app-owned COMMAND, so the framework-reserved `toolRunStart` was refused by the guest's own ownership gate and the preview evaluation's `ToolRunView` never reached the surface (§2) |
| `boot` 2/3 — `framework.panel.toolRun … retained document ingress reached its terminal fault` | **narrowed to one arm, witness landed, not closed**: the retained paint frame faults in `synchronize-node`; which of that walk's 34 `let … else` arms fires is now stamped and printed, but no run on the new build was captured (§3) |
| `no-example` 0/2 | **the battery row was wrong, not the product**: `ArtifactApp::initial_snapshot` opens the hex-column starter by declaration, and a peer's own shipped law says so verbatim. Row rewritten (§4) |
| `spawn-job` 7/8 (`s1_hover`) and `world3d-editor`/`world3d-viewer` hover+select | **downstream of the world-surface delivery**, not their own defects (§5.3) |
| `node-gestures` 5/8 (wire connect/cut) | **not this lane's** — `apply_node_graph_screen_pointer` answers zero edits; owner named in §7 |
| the `AppPresentCursor` frame-gate stall | **the `Engine` holder is measured and fixed, and a watchdog now bounds the whole ladder** (§5) |
| the native `invocation_from_frames` seam | **closed, with a Rust law over the shared fixture** (§2.2) |

The single highest-value reading of the session is in §5.1: the peer-landed
`[DEBUG] engine realize stalled arm=…` witness finally fired on 6118 and named
**`metrics-invalidation-scan`** — the arm `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §6.2 could not
choose by inspection and `📓️wgpu-host-settle-pump-2026-09-14.md` §7.2 guessed WRONG about.

---

## 2. `chrome` — the cancel affordance the wgpu host could never offer

### 2.1 The measurement, not the guess

`🗑️generated/wgpu-verify/chrome/console.txt`, the 11:10 run, 1 059 272 B:

```
grep -o 'cancellable[^,}]*' console.txt | sort | uniq -c
    206  cancellable\":false
      2  cancellable\\\":false
grep -c 'cancellable":true' console.txt → 0
```

**`cancellable: true` appears zero times in the whole run**, so this is not the "short window" the
settle-pump lane's §6.5 wondered about — the affordance is never declared at all. React, on the SAME
staged guest, publishes it at boot (`🗑️generated/react-verify/cancel-preview/run.txt`, line 1, t=5 266 ms):

```json
"preview": { "computing": true, "phase": "computing", "cancellable": true,
             "cancelAction": "toolRunAbort", "cancelArgs": { "runId": "1", "generation": 0 } },
"cancelButton": { "action": "toolRunAbort", "label": "Cancel", "tag": "BUTTON" }
```

The guest decides this in one line — `preview_progress_status_json_for`,
`🧊️generation3d/…/🧵️preview-eval/🦀️.rs`:

```rust
let abortable = run.filter(|run| preview_eval_run_is_abortable(run) && address.is_ok() && !chain.settled());
```

`run` is `doc.tool_run()`. Same guest code on both renderers, so a `cancellable` that is true on one
and false on the other means the two hosts leave the guest's tool-run ledger in different states.

### 2.2 The cause: one re-arm rule, two spellings

The wgpu console's FIRST tool-run line, 2 107 ms, is a warning no React console carries:

```
[DEBUG] contributions rearm failed toolRunStart command 'toolRunStart' is not owned by app s.procedural.generation3d@1/*#editor
```

That string is the guest's own ownership gate (`dispatch_command`, `💻️os/🔨️modules/🔌️plugin/🦀️.rs:24950`).
It fires because the wgpu bridge addressed the re-arm as an app command, unconditionally:

```ts
// 🐚️plugin-bridge/🟦️.ts, before
await performInvocation(requireChannel(instanceId), instanceId,
  { address: { owner: { app: { pluginId, appId } }, commandId: dispatch.action }, arguments: dispatch.args ?? {} }, slimView);
```

React has the rule, and only React had it (`🛠️ShellHelpers/🟦️.tsx`, `makeEffectDispatchOne`): an id the
app DECLARES as a command re-enters the typed command channel; everything else — every
framework-reserved verb, every window action — re-enters the scoped ACTION channel.

**Fix — one law, both hosts.** `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` now owns
`hostEffectInvocationV1(scope, appCommandIds, action, args)` and `appCommandIdsV1(manifest, appId)`.
React's `encodeEffectActionInvocation`/`encodeEffectCommandInvocation` are three lines over it (their
signatures and behaviour unchanged, by construction); the wgpu bridge calls it with
`wgpuEffectDispatchScope(pluginId, appId, slimView)` and logs the channel it chose
(`[DEBUG] contributions rearm … channel=action|command`).

Fixture: `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔁️host-effect-invocation/🔣️.json` (new).
Law: `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔁️host-effect-invocation/🟦️.ts` (new), registered as
`@semio-tech/framework-rs:test-host-effect-invocation`.

```
bunx nx run @semio-tech/framework-rs:test-host-effect-invocation
 5 pass · 0 fail · 10 expect() calls · Ran 5 tests across 1 file
```

Served, not just built: `nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker`, then
`curl -s http://127.0.0.1:6118/🎞️frame-worker.js/🟨️.js | grep -c wgpuEffectDispatchScope → 2`.

### 2.3 The native twin of the selection-roundtrip seam — closed

`📓️wgpu-selection-roundtrip-2026-09-15.md` §7 handed over the native `invocation_from_frames`: it folds
only `in_reply_to == seq`, so a framework-reserved verb's settled answer (published with
`in_reply_to: 0` by `plugin_complete_reserved_spawned_job`) is dropped, masked on that target because
its `ui_scope` default is `Full` — a mask the scope decode that landed with this ticket removed.

`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` now folds BOTH, per field, in frame order:
`UNCORRELATED_REPLY_SEQUENCE = 0`, every field guarded on non-empty bytes, and `saw_invocation` set by
the CORRELATED frame alone — so an uncorrelated frame is additive and can never satisfy "the plugin
answered this sequence". This target has no leftover lane (`KernelClient::exchange_commands` already
unpacks every `Effect::SendMessage{target: Shell{..}}` onto `ExchangeOutcome::frames`), so the rule the
browser bridge spells as `leftoverShellInvocationFrames` is spelled here as one guard.

Law: `🧱️elements/🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs` (new), over the **same**
fixture the TypeScript law reads — `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json`.

```
cargo test -p semio-framework-os-renderer-wgpu --lib reserved_verb_answer
test program_bridge::wasm_program_exchange::reserved_verb_answer_tests::an_uncorrelated_answer_never_satisfies_the_sequence_the_caller_waits_on ... ok
test program_bridge::wasm_program_exchange::reserved_verb_answer_tests::folding_the_admission_alone_reproduces_the_defect_this_law_closes ... ok
test program_bridge::wasm_program_exchange::reserved_verb_answer_tests::a_reserved_verbs_settled_answer_reaches_the_native_shell ... ok
test result: ok. 3 passed; 0 failed
```

The middle law is the before-state by construction: folding the admission alone answers
`uiScope.kind = "none"` and no `interactionView`, which is the defect verbatim.

---

## 3. `boot` — the toolRun panel's terminal paint fault, narrowed to one walk

`🗑️generated/wgpu-verify/boot/console.txt`, lines 10 449–10 706:

```
10449 [DEBUG] ui-doc ingress window=framework.panel.toolRun generation=7 nodes=17
10705 [DEBUG] ui-doc paint fault window=framework.panel.toolRun phase=Some("synchronize-node") nodes=Some(true)
10706 [DEBUG] wgpu-shell surface fault surface=framework.panel.toolRun body=framework.body.toolRun detail=retained document ingress reached its terminal fault
```

`synchronize-node` is `RetainedInteractiveSyncStep::Fault` out of
`sync_interactive_state_node_step` (`🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`) — a function with **34 `let …
else` arms that all answer the same bare `Fault`**. The paint frame collapses all 34 into one phase
name, and the shell collapses THAT into `retained document ingress reached its terminal fault`: three
names for one event, none of which says which arm fired.

**Landed, and it is a witness rather than a fix**: all 34 exits now go through one
`retained_sync_fault(cursor, line!())` that stamps the source line onto the cursor;
`UiEngine::paint_frame_sync_fault_line(window_id)` exposes it and the interpreter prints it —

```
[DEBUG] ui-doc paint fault window=… phase=… sync-line=NNNN nodes=…
```

Ruled out by reading while adding it, so the next run does not have to: the capacities are not it
(`RETAINED_SYNC_COLLECTION_ITEMS = 256`, `RETAINED_SYNC_OUTPUTS = 512`, `RETAINED_SYNC_DEPTH = 64`,
`RETAINED_SYNC_KEY_BYTES = 256`, against a 17-node panel); and `tree.accepted_layout(id)` cannot be
`None` where `tree.node(id)` just matched, because both are the same `arena.get(id)`. The two arms
left standing are the `child_scan` exhaustion pair — `SelectScan` and `TreeApplyScan` both answer
`Fault` the step AFTER `child_scan` runs off the sibling list, i.e. a declared row with no retained
child. **No run on the new build was captured, so this is narrowed, not closed.**

---

## 4. `no-example` — the row was wrong, and a peer's own law says so

The row demanded that a boot with no `?example=` paint NOTHING. The app declares the opposite:

```rust
fn initial_snapshot() -> Generation3dSnapshot { crate::standards::v1::subsets::any::schema::default_snapshot() }
// default_snapshot() = Generation3dSnapshot::parse_dsl(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT)
```

and the schema module says it in as many words beside `default_generation3d_snapshot`: *"NOT empty …
this is the DEFAULT document, not the empty one — the name it carried until ticket
26/09/09/PROCEDURAL-3D-END-TO-END said otherwise and made every 'empty document' law read against a
populated graph."* The genuinely empty projection is `empty_generation3d_snapshot`, which no boot path
opens.

A peer's shipped law states the same thing and is green on this tree:

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib set_active_example_artifact_gesture_fits
test editor::generation3d::component::fold_contract::set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example ... ok
```
> `assert!(generation3d_fixture_operations(&boot.fixture, &hex.fixture).is_empty(),
>   "the boot document IS the hex-column fixture, …")`

And the measured boot agrees: `🗑️generated/wgpu-verify/no-example/(none)/edit/console.txt` carries **no
`setActiveExample` at all** and paints `profile@wire` / `extrusion-axis@vectorOut` / `extrude@solid`
with a hexagonal profile (`edgePositions` = six segments at radius 0.5).

`🐍️wgpu-battery.mjs`'s `no-example` row now asserts the declared behaviour — *no example opens the
app's own starter document* — with the full derivation in its docstring. **Not re-run** (§7).

---

## 5. The frame gate — the `Engine` holder, measured at last

### 5.1 It is `metrics-invalidation-scan`, and it costs 256 blocked frames

The two `[DEBUG]` lines `📓️wgpu-host-settle-pump-2026-09-14.md` §7.2 added were never in a served
build. This lane built them, and they fired on the first boot
(`🗑️generated/wgpu-verify/examples/box-fillet-preview/edit/console.txt`):

```
4127 [DEBUG] engine realize stalled arm=metrics-invalidation-scan steps=64  scan=Some(148)
4127 [DEBUG] engine realize stalled arm=metrics-invalidation-scan steps=128 scan=Some(212)
4139 [DEBUG] os_host frame gate blocked=true pending=true phase=Some(Engine) … stall-steps=177 generation=Generation(5)
```

Identically on `sphere-cut-with-torus/edit`. That settles §7.2's open question — and settles it against
its own guess, which named `slot-retirement` as "the likelier holder".

The mechanism is arithmetic, not a race. `EngineCanvasPresenter::realize_step` advanced the scan **one
fixed slot per present step** and answered `Ok(false)`, and `ENGINE_SURFACE_CAPACITY` is **256** — so
one surface resize buys 256 consecutive `Ok(false)`s, during which `AppPresenter::admit_next_frame`
builds no frame at all and therefore the frame transaction, the runtime mailbox pump, the world
snapshot apply, the retained document ingress and the accessibility projection all stop. The scan's
entire per-slot work is one `candidate.begin_close()`.

**Fix**: `realize_step` drains the WHOLE scan inside one answer
(`for _ in 0..=ENGINE_SURFACE_CAPACITY { if self.invalidate_primary_metrics_step() { break } }`), so
the arm costs one blocked frame instead of 256.

Laws: `🧱️elements/⚙️EngineCanvas/🧪️tests/🩺️metrics-invalidation-drain/🦀️.rs` (new) — one behavioural
(the scan visits every fixed slot exactly once and leaves the presenter terminal-empty) and one source
scan (the arm drains against the fixed ceiling and no single-slot advance remains, because the arm
needs a live `GpuContext` no unit test can mint).

```
cargo test -p semio-framework-os-renderer-wgpu --lib metrics_invalidation_drain
test engine_canvas::metrics_invalidation_drain_tests::the_primary_metrics_invalidation_scan_terminates_within_the_fixed_slot_table ... ok
test engine_canvas::metrics_invalidation_drain_tests::realize_step_drains_the_whole_metrics_invalidation_scan_in_one_answer ... ok
test result: ok. 2 passed; 0 failed
```

### 5.2 A watchdog so the ladder can never silently stop again

`AppPresenter` now carries `AppPresentStallWatch`. Every `present_step` compares the cursor's progress
signature to the last one; a signature that repeats `APP_PRESENT_STALL_STEPS = 4 096` times is named
once and the presentation is forced to `AppPresentPhase::Aborted` with
`retained_fault = "presentation stalled: …"`, which is the one action that reopens the gate without
touching a GPU owner. `presentation_gate_shape` now prints the retained fault's TEXT (it printed a
bare `true` before) and the live stall count.

**The first signature this lane shipped was wrong, and the browser said so within the hour** — worth
recording, because it is the whole reason the watchdog is now written the way it is:

```
4420 [DEBUG] os_host present stalled phase=Render engine=1 upload=1 gpu-cursor=true retained-fault=None
```

on `box-fillet-preview/edit` AND `sphere-cut-with-torus/edit`, i.e. on every example boot.
`AppPresentPhase::Render` holds ONE `gpu_cursor` for a whole composite pass, so from outside the
cursor looks frozen for as many steps as the scene has commands, and a ceiling read against the outer
shape alone aborts a perfectly healthy present. The signature now carries the GPU cursor's own
`(phase, command, glass command, blur mip)` via the new
`PreparedGpuPresentCursor::progress()` (`🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`), and the fixture pins that
false positive as a case of its own.

Fixture: `🧑‍🎨engine/🧫️fixtures/🐕️present-stall-watch/🔣️.json` (new, 5 cases incl. the false positive).
Law: `🧑‍🎨engine/🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs` (new).

```
cargo test -p semio-framework-os-renderer-wgpu --lib present_stall_watch
test present_stall_watch_tests::a_cursor_that_moves_after_the_ceiling_rearms_the_watchdog ... ok
test present_stall_watch_tests::the_presentation_watchdog_names_a_frozen_cursor_once_and_a_healthy_one_never ... ok
test result: ok. 2 passed; 0 failed
```

### 5.3 The world-surface delivery the coordinator handed over — where it actually stops

`📓️interaction-scope-narrowing-2026-09-15.md` §8 hands over `world3d-editor` 62/115 and
`world3d-viewer` 63/115 with `objects=0 draws=0` before any interaction verb. Read off
`🗑️generated/wgpu-verify/boot/console.txt`, the surface's own census across one boot:

| t (ms) | state-meshes | apply | rebuild | state-draws |
|---|---|---|---|---|
| 3 375 | 0 | false | false | 0 |
| 5 482 | 1 | true (page 0/1) | false | 0 |
| 12 314 | 3 | true (page 0/2) | false | 0 |
| **15 288** | 3 | false | **true** | 0 |
| 17 047 → 56 981 | 3 | false | false | **0, for the rest of the run** |

So the meshes DO arrive and the apply DOES complete; the draw rebuild it begins is begun at 15 288 and
is gone by 17 047 having published nothing. `step_world3d_draw_rebuild` has exactly two ways to end
without publishing — `Stale` (the host closes it and nothing ever begins another) and `Fault` — and
neither was traceable. Both now print one line naming the step and the full census
(`[DEBUG] world3d draw rebuild surface=… step=Stale|Fault …`), and a completing apply prints
`[DEBUG] world3d delivery applied surface=… …`; the `Stale` arm keeps the exact source shape
`🧾️frame-action-ledger`'s own law scans for (re-verified, 3/3 green).

`spawn-job`'s single red is downstream of this and not its own defect:
`🗑️generated/wgpu-verify/spawn-job/results.json` reads `s1_hover {pumps:0, actions:0, hover:null}`
while `s2_click`…`s5_marquee` all read `{pumps:2, actions:1}` — a click publishes `interactionSelect`
whether or not it hits anything, a hover publishes only when the ray finds a triangle, and with
`draws=0` there are no triangles. The same reading explains `world3d-editor`'s
`h1 hovering the centre {"hover":null}` on 7 of 8 examples and every selection assertion below it.

**One hypothesis was tested and REFUTED, and it belongs on the record**: that
`sync_world3d_scene_fit` (`700854a9ab`, 09:46 today, the fit lane) strands the delivery's own rebuild
by advancing `interaction_revision` between the rebuild's `begin` and the apply's completion. A law
was written for it over the real scene-bridge fixture —
`a_first_framing_never_strands_the_deliverys_own_draw_rebuild`,
`♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` — and it **passes on the unmodified tree**
(`cargo test -p semio-framework-os-infinite --lib a_first_framing_never_strands` → 1 passed). The law
is kept: it pins the property the browser needs, and it is the first thing to re-check when the trace
above lands.

---

## 6. Compile state — everything this lane touched, green

Run foreground at 13:0x, after the directive to pause:

```
cargo check -p semio-framework-os-infinite       --lib --tests                      → Finished dev profile
cargo check -p semio-framework-ui                --lib --tests                      → Finished dev profile
cargo check -p semio-framework-os-renderer-wgpu  --lib --tests                      → Finished dev profile (43.36s)
cargo check -p semio-framework-os-renderer-wgpu  --lib --target wasm32-unknown-unknown → Finished dev profile (55.13s)
nx run @semio-tech/framework-renderer-wgpu:wasm  → Finished dev profile in 1m 44s, Published 2 renderer files (13:00)
nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker → Successfully ran (12:12)
```

Files this lane created or changed:

* `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` — `hostEffectInvocationV1`, `appCommandIdsV1`, `HostEffectDispatchScope`
* `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🔁️host-effect-invocation/🔣️.json` *(new)*
* `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔁️host-effect-invocation/🟦️.ts` *(new)*
* `🧰️framework/📦️packages/🦀️rust/📜️script.ts`, `📋️project.json` — `test-host-effect-invocation`
* `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` — `retained_sync_fault`, `fault_line`
* `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` — `paint_frame_sync_fault_line`
* `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs` — `PreparedGpuPresentCursor::progress`
* `…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `sync-line=` on the paint-fault line
* `…/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — the scan drain + test mod
* `…/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🩺️metrics-invalidation-drain/🦀️.rs` *(new)*
* `…/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — the uncorrelated fold + test include
* `…/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs` *(new)*
* `…/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — both encoders over the shared law
* `…/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` — `wgpuEffectDispatchScope`, the re-arm rule
* `…/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — the watchdog, the gate shape, the world3d traces
* `…/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` — the wasm32 `AppPresenter` initializer
* `…/🧑‍🎨engine/🧫️fixtures/🐕️present-stall-watch/🔣️.json`, `…/🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs` *(new)*
* `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` — `a_first_framing_never_strands_the_deliverys_own_draw_rebuild`
* `<ticket>/🐍️wgpu-battery.mjs` — the `no-example` row

---

## 7. ⏸️ What is NOT done, and what is NOT claimed

1. **No battery was run by this lane, at all.** The lane was paused before its own measurement window
   opened; the port gate was held by peer runs (`--only=world3d-editor,world3d-viewer`, then
   `--only=frame-loop,examples`) for the whole session. **Every row verdict quoted above is from the
   11:10–11:28 shared scoreboard, i.e. from BEFORE every fix in this report.** There is no after.
2. **Therefore no fix in §2, §4 or §5 has a runtime after-reading.** Each has a law that passes and a
   before-reading that is verbatim from a console; none has been shown to move a battery row.
3. **`boot` is narrowed, not closed** (§3). The `sync-line=` witness is in the served build and has
   never been read.
4. **The world-surface delivery is instrumented, not fixed** (§5.3). The `Stale`/`Fault`/`applied`
   traces are in the served build and have never been read. The refuted fit-lane hypothesis is
   recorded so the next lane does not spend the hour again.
5. **`node-gestures` 5/8 is not this lane's and was not diagnosed beyond one reading**:
   `🗑️generated/wgpu-verify/node-gestures/verdict.json` has `node-graph screen gesture: 0`,
   `action=nodeGraphEdit: 0`, `dag edge connected/removed: 0` against `wgpu node-graph hit: 21` and
   `handle_event PointerDown: 9` — so the gesture reaches the surface and
   `apply_node_graph_screen_pointer` answers an EMPTY `outcome.edits`. Owner: whoever landed
   `700854a9ab`'s *"infinite world directed DAG wire editing with unit conformance suites"*.
6. **`chrome` may need more than §2.** The re-arm fix removes a refused continuation that React never
   had; whether the guest's `ToolRunView` then reaches `Starting/Running/Paused` at the instants the
   surface renders is unmeasured.
7. **The watchdog cannot rescue a stalled `Aborted`.** Forcing a non-`Aborted` phase to abort reopens
   the gate; an `Aborted` cursor whose own close never terminates is named once and then still blocks,
   because the alternatives all mean dropping a live GPU owner. `close_active_candidate_step` and
   `EngineGpuCandidate::close_step` are bounded by construction (read while adding the watchdog), so
   this is believed unreachable — believed, not measured.
8. **`APP_PRESENT_STALL_STEPS = 4 096` is a judgement, not a measurement.** It is three orders of
   magnitude above the longest healthy non-advancing run this ladder is known to have, and §5.2 is the
   record of the first version getting the SIGNATURE wrong rather than the ceiling.
9. **The React half of §2 is unchanged by construction, not by test.** `encodeEffect*Invocation` now
   delegate to the shared law with the same inputs and outputs; no React battery was run to confirm
   it, and one should be before this is trusted.
10. **`cargo test -p semio-framework-os-renderer-wgpu --lib` does not complete on this tree.**
    `async_boundary_tests` aborts the process with `panic in a destructor during cleanup`
    (`WorldAssetFetchOwner::drop`) — pre-existing, recorded by `📓️wgpu-hit-registry-drain-2026-09-14.md`
    §7 and `📓️wgpu-selection-roundtrip-2026-09-15.md` §7. With `--skip async_boundary_tests` the run
    reaches four more reds before a second abort (`caret_blink_toggles_and_rearms_on_fire`,
    `dock_stack_content_fills_full_bounds_through_one_silhouette_clip`,
    `resize_hits_win_over_later_scroll_region`,
    `engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack`, plus
    `a_board_window_paints_its_tool_run_trace_lane_and_echoes_the_cursor`,
    `node_graph_window_attaches_the_flow_engine_and_paints_a_non_empty_draw_list` and
    `world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches`). None is in this lane's
    diff and none was reverted to prove it — the honest statement is "not this lane's symbols", not
    "pre-existing".
11. **The `@semio-tech/framework-renderer-react:typecheck` target is red repo-wide** (`bun:sqlite`,
    `Bun` globals, `ImportMeta.dir`, …). This lane's symbols appear nowhere in its error list; the
    `🐚️plugin-bridge` errors it does list are all `requestedEffects possibly undefined` /
    `FaultScope.req` / a missing `effects` field, none of which this lane wrote.
12. **No screenshot is evidence** — the canvas is an `OffscreenCanvas` owned by the frame Worker and
    captures blank. Everything above is a `[DEBUG]` trace, a probe JSON or a test run.

---

## 8. If and when this lane resumes

In order, and the first two need nothing but a run:

1. Run `SEMIO_BATTERY_ROOT=wgpu-sweep bun 🐍️wgpu-battery.mjs` on the 13:00 build and read the three new
   traces: `world3d draw rebuild … step=Stale|Fault`, `world3d delivery applied`, and
   `ui-doc paint fault … sync-line=`. Those three lines decide §5.3 and §3 outright.
2. Confirm `contributions rearm failed toolRunStart` is gone and `cancellable":true` appears — that is
   §2's whole after-reading.
3. Only then: `node-gestures` (§7.5), and the `world3d-editor` camera-framing reds, which belong to
   `📓️boot-camera-framing-2026-09-15.md`'s lane — its §3.2 wgpu column is a NATIVE law and its own §6
   says the wgpu reading was never taken.
