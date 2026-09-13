# 🧾️ wgpu DEFERRED-ACTION COMMIT — a user's commit is not the frame's to throw away

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-deferred-action-commit**, 2026-09-13.
Closes `📓️wgpu-generation-publication-2026-09-13.md` §8 (`renameGeneration`'s COMMIT is dropped by the
frame's deferred-action queue) and, with it, the same queue's other traffic —
`📓️wgpu-retained-controls-wires-2026-09-13.md` §2's six retained control kinds.

Repo MCP was down the whole session (`repo -32602 invalid initialize params`,
`semio CONNECTION_CLOSED`); no ticket was opened, closed or reopened, `📓️status.md` and
`🎫️ticket.json` were not touched. No git-state-modifying command was run. The coordinator's serve on
`http://127.0.0.1:6118` was neither started nor stopped. Evidence under
`🗑️generated/wgpu-deferred/`; nothing under any `🗑️generated` folder this lane did not create was
removed.

---

## 1. TL;DR

| question | answer |
|---|---|
| Where did the commit die? | **Nowhere it could be seen.** It was not installed, not dispatched, and not even RETIRED. The frame candidate holding it was **dropped by Rust** — §3. |
| Root cause | `🧵️frame-job/🦀️.rs:357` (as shipped): `AppFrameTransactionStep::Superseded => { self.phase = ActiveFramePhase::Terminal; … }` replaces the live `FrameTransaction` and runs no close ladder. The transaction OWNED the action queue, and `app.input.take_action_step()` had already taken the action **out** of the bounded input authority — so nothing else owned it. §4. |
| Why the earlier hypothesis was wrong | §8 of the previous report named `AppFrameAfterChrome::close_step` (the cancel ladder). Instrumented, that line never fired: measured, the candidate is dropped, not closed. Both paths destroy a commit; only the drop was the one taken. §3. |
| Fix | The ledger is the **runtime's**, not the frame's. New `AppRuntime::frame_actions` (`🧊️renderer/🦀️.rs:10764`) outlives every candidate; `FrameTransaction`, `FrameBuildCursor`, `AppFrameAfterChrome` and `FrameFinishCursor` own no action queue at all, and the four `close_step` ladders that used to pop one are gone. §5. |
| Laws | **5 green Rust** (2 new + 3 pre-existing in the frame-deferred area) and **3 green TypeScript**, over one new language-neutral fixture with **9 rows** that keeps BOTH ownership models side by side. Each new law measured FAILING against the shipped model first — §6. |
| 6118 | `renameGeneration` — minted → installed → **admitted by the guest** → **guest ran the command** → the roster shows the new name, across 70 supersessions. A Form slider DRAG mints up to 7 `updateGenerationValues`, and the guest admits and runs them. The retained-controls lane's own probe now reports `updateGenerationValues: 5` where its report recorded zero. §7. |
| Not claimed | The generate preview's **mesh** changing. Two downstream faults that only became reachable once commits started landing, and which **quarantine the wgpu surface**. §8. |

---

## 2. The reported symptom

`📓️wgpu-generation-publication-2026-09-13.md` §8: the inline rename editor is opened, focused and
typed into; the commit reaches the dispatcher with the right arguments
(`frame input action … action=renameGeneration args={"id":"generation-1","value":…}`); and then
nothing. No `plugin_exchange actionId=renameGeneration`, no `gen3d command`, no fault, no error, for
769 console lines.

Reproduced unchanged against that lane's own last build
(`🗑️generated/wgpu-generation/final-1/console.txt`, its run at 21:06–21:08 on the 21:05 wasm, which
is the FIRST build carrying its two new `[DEBUG]` lines):

```
frame input action                                  1     ← minted
frame deferred install carries an action            0     ← never installed
frame deferred action retired without dispatch      0     ← never retired either
plugin_exchange actionId=renameGeneration           0
frame deferred action failed                        0
```

The action was neither installed nor retired. §8's hypothesis — the cancel ladder popping it as a
retirement unit — predicted the third line to be `1`. It is `0`. Something else destroyed it.

---

## 3. Hop by hop, measured

Every silent drop site was instrumented (one `[DEBUG] frame deferred drop site=<site> action=<name>`
per pop in `FrameBuildCursor::close_step`, `FrameFinishCursor::close_step`,
`AppFrameAfterChrome::close_step`, `FrameTransaction::close_step` and `FrameDeferredCursor::close_step`;
plus one line when a superseded transaction still owned work, and one for every `record_frame_fault`).
Built (`🗑️generated/wgpu-deferred/wasm-1.txt`, `Successfully ran target wasm`, dist 21:18) and run
(`🗑️generated/wgpu-deferred/measure-1`).

```
137934 [DEBUG] frame input action controller=s.procedural.generation3d@1/*#editor
               action=renameGeneration args={"id":"generation-1","value":"Generation 1BalconyStudy",…}
137935 [DEBUG] frame transaction superseded while still owning work
137965 [DEBUG] frame build admitted generation=Generation(1712)
```

**One millisecond.** And for the whole 152-second run:

| line | count |
|---|---|
| `frame deferred drop site=…` (all five ladders) | **0** |
| `frame deferred install carries an action` | **0** |
| `frame fault recorded` | **0** |
| `frame transaction superseded while still owning work` | **655** |

No close ladder ever ran on the transaction that held the commit. It was dropped.

---

## 4. Root cause

Two lines, in two files, that are only a defect TOGETHER.

**(a) The frame candidate owned the queue.** As shipped, `FrameTransaction` declared
`deferred_actions: FrameActionOwners` and handed it down the whole build —
`FrameTransaction` → `FrameBuildCursor` (`FrameBuildCursor::new`) → `AppFrameAfterChrome`
(`FrameBuildPhase::Complete`) → `FrameFinishCursor` (`FrameFinishPhase::Deferred`) — reaching
`AppRuntime::pending_frame_deferred`, the first owner that outlives a frame, only at
`FrameFinishPhase::Complete`. The window between is the entire rest of the build: chrome, input,
engine and world resources, the prepared packet.

**(b) A superseded candidate is dropped, not closed.**
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:357`
(shipped):

```rust
crate::AppFrameTransactionStep::Superseded => {
    self.phase = ActiveFramePhase::Terminal;
    ActiveFrameStep::Complete(None)
}
```

The borrow of `transaction` ends at the assignment and the `FrameTransaction` is destroyed by
`Drop` — `retire_active_phase`'s `ActiveFramePhase::Build(transaction) => transaction.close_step()`
(`:207`) is the CANCEL path and is not on this one.

Supersession is not an error condition. `FrameTransaction::step` answers `Superseded` from four
places (`🧊️renderer/🦀️.rs`: a moved operation/generation, a cancelled or deadline-exceeded context, a
stale presentation witness, a moved base witness, and the deliberate no-parking rule
`if !app.interaction_available()`), and the browser frame session cancels a build whose generation
moved. It happened **655 times in one 152-second session**. The retained rename's own dispatch
checkout is one of the things that moves the generation, so the gesture superseded the very frame
carrying its commit.

And the commit had nowhere else to be: `AppFrameTransactionPhase::InputEvents` reaches it through
`app.input.take_action_step()`, which **takes** it out of the bounded input authority. Once the
candidate died, the action existed in no owner at all — which is exactly why there was no fault, no
error, and no retirement line to find.

**Why every existing law was blind to it.** The ownership models are indistinguishable on an
uninterrupted frame, and every law drove exactly that frame: `frame_deferred_cursor_advances_one_owned_operation_in_order`
and `frame_deferred_cancel_retires_one_action_per_step` (`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`)
both start from a `FrameDeferredCursor` that already exists — i.e. after the install. Nothing tested
the hop into it. §6's fixture keeps both models side by side so the difference is what is scored.

---

## 5. The fix — one owner for a queue that must outlive the frame

A user's commit is SESSION state. The frame borrows it at install; it never owns it.

### 5.1 The ledger — `🧊️renderer/🦀️.rs:10764`

`AppRuntime` gains `frame_actions: FrameActionOwners`, declared beside `pending_frame_deferred`, the
other half of the same pipeline and the only other owner in the renderer that outlives a frame. The
type, the fixed FIFO capacity and the refusal-returns-the-action contract are unchanged.

* The two minting authorities push onto it: `AppFrameTransactionPhase::SceneCamera` (`:11413`) and
  `AppFrameTransactionPhase::InputEvents` (`:11479`), both through the `app` guard they already hold.
* `FrameFinishPhase::Complete` (`:13582`) is the ONLY reader: `std::mem::take(&mut self.frame_actions)`,
  all of it, once, into the `FrameDeferredCursor` it installs.
* `FrameTransaction`, `FrameBuildCursor`, `AppFrameAfterChrome` and `FrameFinishCursor` no longer
  declare `deferred_actions` at all, so the four `close_step` pops and the four `terminal_is_empty`
  clauses that used to discard a commit are deleted rather than silenced — and the two boundary
  faults that existed only to carry the queue across a hop (`"frame build lost deferred action
  owners"`, `"frame completion lost deferred action owners"`) are gone with them.
* The existing refusal `"frame completion found an unclosed deferred owner"` now **costs nothing**:
  the ledger stays on the runtime and the next completing frame installs it, where before the
  faulted candidate went on to discard what it was holding. The three `cursor.deferred_actions =
  Some(deferred_actions)` restore-on-fault dances in `FrameFinishPhase::Complete` are gone for the
  same reason.

`🧵️frame-job/🦀️.rs:360` keeps the plain drop and now says WHY it is sound: a transaction owns nothing
a user minted.

### 5.2 A fault a probe can read — `🧊️renderer/🦀️.rs`

Both fault publishers now emit one bounded `[DEBUG] frame fault recorded: …` line:
`RuntimeMailbox::record_frame_fault`, and the interaction-checkout stale notice that writes
`frame_fault` directly. Faults quarantine the surface and were previously visible ONLY as text
painted on the canvas — and on this target the canvas belongs to the frame worker
(`OffscreenCanvas` transferred), so `page.screenshot` returns black and no probe could see one.
§8 is measured entirely through these lines.

---

## 6. Laws

Shared oracle: `🧫️fixtures/🧾️frame-action-ledger/🔣️.json` — 9 rows over four operations (`mint`,
`supersede`, `retire`, `complete`, `dispatch`) and two declared owners, `runtime` (the rule) and
`candidate` (the shipped model the oracle exists to refuse). Four rows are matched pairs: the same
gesture under both owners, so the fixture states the DIFFERENCE rather than only the new answer.

### 6.1 Rust — `🧪️tests/🧾️frame-action-ledger/🦀️.rs` (+2)

`cargo test -p semio-framework-os-renderer-wgpu --lib -- frame_action_ledger frame_deferred` →
**5 passed; 0 failed** (2 new + the 3 pre-existing frame-deferred laws).

* `a_commit_outlives_every_frame_candidate_that_was_discarded_before_it_dispatched` — replays every
  row against the LIVE `FrameActionOwners` and the LIVE `FrameDeferredCursor`, with real
  `ActionDescriptor`s, asserting per step that a completion installs exactly when no owner is live
  and the ledger is not empty, and that each dispatch hands the shell the action the fixture names.
* `the_ledger_is_the_runtimes_and_no_frame_candidate_owns_one` — the structural half: the runtime
  declares the ledger, both authorities push onto it, only a completion takes it, no frame-candidate
  struct body contains `deferred_actions`, no `close_step` pops one, and the superseded arm is still
  a plain drop (which is what makes the ownership load-bearing).

**Measured failing first.** With the replay forced onto the candidate-owned model (`"runtime" =>
false`): **1 passed; 1 failed** —
`a-commit-minted-in-a-superseded-frame-still-dispatches step 2: the completion installs exactly when
no owner is live and the ledger is not empty`, because the supersede had already destroyed it.
The structural law's six predicates are each false against the shipped source (read from
`git show HEAD:` — read-only):

```
frame_actions declared on the runtime : False      struct FrameBuildCursor   owns deferred_actions: True
authorities push onto the runtime     : False      struct AppFrameAfterChrome owns deferred_actions: True
struct FrameTransaction owns it       : True       struct FrameFinishCursor  owns deferred_actions: True
a close ladder pops a commit          : True
```

### 6.2 TypeScript twin — `🧪️tests/🧾️frame-action-ledger/🟦️.ts` (+3)

`bunx vitest run --config 🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts 🧪️tests/🧾️frame-action-ledger/🟦️.ts`
→ **3 passed**. Re-derives the ledger from the fixture's statement alone (a FIFO the runtime keeps, a
discarded candidate that cannot touch it, a completion that takes all of it and only when no owner is
draining), plus the same structural and superseded-arm assertions read off the Rust sources as text.
Registered in that config's `include`.

Measured failing first, the same way: **1 failed | 2 passed**,
`a-commit-minted-in-a-superseded-frame-still-dispatches step 2: installs: expected false to be true`.

### 6.3 Regressions

`cargo test -p semio-framework-os-renderer-wgpu --lib` has pre-existing reds outside this lane
(`native_binary_owns_exactly_one_entrypoint_driver`, `raster_upload_cache_…`,
`renderer_asset_probe_…` which SIGABRTs in `♾️infinite/🌍️world`, `caret_blink_…`, `dock_stack_…`,
`engine_canvas_slot_tables_…`, `node_graph_window_attaches_…`). None names a deferred action, a frame
transaction or a frame cursor; the `remaining-suite-reds` lane owns them. Full logs:
`🗑️generated/wgpu-deferred/test-full.txt`, `test-rest.txt`.

---

## 7. 6118 — what the runtime does now

Renderer wasm rebuilt from this lane's source (`🗑️generated/wgpu-deferred/wasm-3.txt`,
`Successfully ran target wasm`, dist 22:16). The generation3d guest was NOT restaged — only the
renderer changed.

### 7.1 `renameGeneration` (`🗑️generated/wgpu-deferred/final-1`, and fix-1, commit-1…5)

| hop | evidence | before |
|---|---|---|
| the commit leaves the input authority | `frame input action … action=renameGeneration` ×1 | ×1 (this always worked) |
| **a completing frame installs it** | `frame deferred install carries an action` ×1, 38 ms later | **×0** |
| **the guest admits it** | `plugin_exchange actionId=renameGeneration branch=catalog` ×1 | **×0** |
| **the guest runs the command** | `gen3d command action=renameGeneration` ×1 | **×0** |
| **the roster shows the new name** | the `…generation-1.rename` node's text goes `"Generation 1"` → `"Generation 1Balcony"` | **unchanged** |
| supersessions in that window | **70** | 72 |

Reproduced on five consecutive runs. (`"Generation 1Balcony"` rather than `"Balcony"` is the
pre-existing pair `📓️wgpu-generation-publication-2026-09-13.md` §8 already named: `Control+A` does not
clear a retained input, and `ui_event_from_key_action` has no mapping for `Space` — both lawed, both
out of scope here. What is scored is that the text CHANGED at all.)

### 7.2 A Form slider drag — `updateGenerationValues` (`🗑️generated/wgpu-deferred/commit-2…5`, `final-1`)

A drag on `generate.form.height.slider` commits every intermediate value, as
`📓️wgpu-retained-controls-wires-2026-09-13.md` §3.2's `commits_while_dragging` says it must:

```
frame input action … action=updateGenerationValues args={…,"questionId":"height","value":2.5,…}
frame input action … value=3.5
frame input action … value=4.5
frame deferred install carries an action
plugin_exchange actionId=updateGenerationValues branch=catalog        ×3
gen3d command action=updateGenerationValues before=1 after=1 ops=1    ×3
```

Best run: **7 minted, 7 admitted by the guest, 7 run** (`commit-3`); 0 `frame deferred action failed`.
Values that arrive while a previous owner is still draining wait on the ledger — which is the rule
working, and is why a run's admitted count can trail its minted count when the session ends first.

A **precondition** this lane had to fix in its own probe, not in the renderer: the retained hit
registry is drained one entry per frame-build boundary step and rebuilt by the chrome walk, so
consecutive pointer samples at the SAME point alternate `targets=40 hit=Some((Slider,…))` and
`targets=0 hit=None`. A blind `mouse.down()` lands on the empty half about half the time, no retained
press is routed, and a `PointerDown`-seeded gesture never seats its capture — `commit-1` measured
exactly that (`retained press … kind=Slider down=false` with no `down=true`, and zero commits).
`🐍️wgpu-deferred-commit-probe.mjs`'s `armHit` jiggles until the shell's own trace resolves the target,
then presses. This is a renderer-side defect in its own right; it is NOT fixed here.

### 7.3 The retained-controls lane's own probe, re-run (`🗑️generated/wgpu-deferred/controls-1`)

`bun 🐍️wgpu-control-commit-probe.mjs`, unmodified:

```
"row": 6, "resolved": { "kind": "Slider", "controlId": "generate.form.height.slider" },
"dispatched": [ 5 × frame input action … action=updateGenerationValues … ]
"counts": { "updateGenerationValues": 5, "addGeneration": 2, "panicked": 0, "surface fault": 0 }
```

`frame deferred install carries an action` ×2 in the same console. That probe's own lane reported
**zero** dispatched form commits (`📓️wgpu-retained-controls-wires-2026-09-13.md` §1: "the Form has no
controls to press until `addGeneration` actually creates a generation"). Its screenshot is 5854 bytes
— black, i.e. the canvas is still the frame worker's and the surface did NOT quarantine in that run.

---

## 8. What is NOT claimed

**8.1 The generate preview's MESH changing.** Not shown, and the obvious witnesses do not work:

* the world3d census is the wrong instrument — a height change re-tessellates the SAME three meshes,
  so `instances=1 lines=1 state-meshes=3` is identical before and after by construction;
* a pane screenshot is worse than useless here. `page.screenshot` on this target returns the UI
  thread's canvas, and the real one belongs to the frame worker (`OffscreenCanvas` transferred), so a
  pane shot is empty. The 98 %-of-bytes-differ reading `commit-3`…`final-1` produced was the
  **fault overlay appearing**, not a mesh — confirmed by opening both images
  (`🗑️generated/wgpu-deferred/commit-5/preview-{before,after}.png`: the same empty-pane cross, one
  dark teal and one dark maroon). It is recorded in the verdicts as `previewPixels` with its
  byte-identical idle control, and it is not evidence of anything about geometry.

What IS shown is upstream of the mesh and downstream of this lane's fix: the guest admits and runs
`updateGenerationValues`, and re-publishes every surface afterwards
(`renderSurface surface=generation3d-generate-preview … rev=17`, up from `rev=9`). The Form's own
retained slider knob moves to the committed position, which the fault-overlay screenshot happens to
show directly.

**8.2 Two downstream faults that this fix made REACHABLE, and that quarantine the surface.** Before
this lane no retained control's commit ever crossed into the guest on wgpu, so nothing downstream of
one had ever run. Both are now measured, both are outside this lane's owning layer, and both are new
work:

* `world3d scene mesh-wire bridge faulted` — `🧊️renderer/🦀️.rs:11631`,
  `AppFrameTransactionPhase::World3dSnapshot`'s `step_world3d_scene_bridge` answering `Fault` while
  the preview rebuilds after a committed value change. Measured once in `final-1` at 105372 ms. The
  same subsystem panicked on teardown in `commit-4`
  (`World3dState reached Drop before retained dynamic owners reached terminal empty`,
  `♾️infinite/🌍️world/🦀️.rs:1664`) — that file is being edited by another lane right now.
* `runtime interaction checkout at dispatch-event outlived its bounded step after 241 apply
  opportunities` — `📮️runtime-mailbox-core/🦀️.rs:123`, `INTERACTION_CHECKOUT_CREDITS = 240`. The
  credit counts **apply opportunities**, not time or frames, and the mailbox is pumped from every
  `ActiveFrameBuild::advance` — thousands of opportunities a second. A `dispatch-event` checkout that
  awaits a guest round-trip therefore burns the whole credit: the blocked episodes around the drag
  measure 365 ms and 485 ms, and the one that quarantined the surface began with
  `plugin_exchange actionId=flowEvalTick branch=command-invocation` inside the checkout
  (`commit-5`, 114843 ms). The shell RECOVERS — it re-rendered every surface and reached
  `wgpu-shell ui chain settled` four seconds later — but the fault is sticky (first fault wins) and
  the surface stays quarantined.

Reproduction of the quarantine is gesture-dependent, not universal: 4/4 runs of this lane's own probe
(whose `armHit` loop plus 6-step drag is a burst), 0/1 runs of the retained-controls lane's slower
probe, 0/5 runs without a seated drag. A run that quarantines is identifiable without reading the
canvas — its `final.png` jumps from ~5.8 KB (black, worker-owned) to 55–78 KB (the UI-thread fault
overlay).

**Next step for whoever picks this up:** the credit's UNIT is the defect, not its size — a bound
counted in mailbox polls cannot express "a checkout may not outlive its bounded step" when the poller
runs at frame-build speed. `🐍️wgpu-deferred-commit-probe.mjs` reproduces it, and §5.2's
`frame fault recorded` line names which fault fired without opening a screenshot.

**8.3 Also not claimed.** A `NumberStepper`, `Ring`, `Toggle` or `IconSelect` commit — generation3d's
generate-mode windows publish `stack`, `text`, `field`, `slider`, `input` and `componentScene` and no
other control kind, so there was none to press. The `Slider` is the kind that was exercised. Nothing
about the NATIVE (`not(target_arch = "wasm32")`) path: the ledger is shared, but only the browser
frame session was run. Nothing about React, which does not use this producer.

---

## 9. Peer breakage fixed forward (noted, not silently absorbed)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:4` — a new
`impl FlowArtifact` block (`retire_cold`, added minutes earlier by the lane that owns that file) was
not importing `FlowArtifact`, so `semio-framework-artifact-flow-flow` failed to compile
(`error[E0425]: cannot find type FlowArtifact in this scope`) and with it every wasm build in the
repo. One name added to the existing `use crate::{…}` list; nothing else in that file was touched.

---

## 10. Method, evidence, files

Probe (ticket root): `🐍️wgpu-deferred-commit-probe.mjs` — new. Boots 6118 into generate mode, presses
the retained `Add Generation` row at its own published rect, then drives the two commits that cross
this queue and reports, from the console alone, the four hops each (`frame input action` →
`frame deferred install carries an action` → `plugin_exchange actionId=…` → `gen3d command action=…`)
plus every `frame fault recorded`, the published node text, and a preview pane pixel diff against a
byte-identical idle control. `SEMIO_RUNTIME_DIAGNOSTICS` is armed so the guest's own traces are
readable (the door `📓️wgpu-generation-publication-2026-09-13.md` §5.3 opened). Also re-run unmodified:
`🐍️wgpu-generation-publication-probe.mjs` (`SEMIO_PROBE_JOURNEY=1`) and
`🐍️wgpu-control-commit-probe.mjs`.

Evidence: `🗑️generated/wgpu-deferred/{measure-1,fix-1,commit-1…commit-5,final-1,controls-1}`,
`wasm-{1,2,3}.txt`, `check-{1,2}.txt`, `test-{1,full,rest,shipped-model}.txt`.

Changed:

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — the ledger, the two authorities, the install, four owners stripped of it, two fault diagnostics
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs` — the superseded arm's rationale
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs` — the browser runtime's ledger
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧾️frame-action-ledger/🔣️.json` — new
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧾️frame-action-ledger/🦀️.rs` — new
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧾️frame-action-ledger/🟦️.ts` — new
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` — one `include` entry
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` — peer, §9
* `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️wgpu-deferred-commit-probe.mjs` — new
