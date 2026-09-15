# 🕹️ The wgpu selection round trip — where it broke (lane `wgpu-selection-roundtrip`, 2026-09-15)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`. Target: the coordinator's wgpu serve,
`http://127.0.0.1:6118/?plugin=generation3d` (`&role=viewer`). Red #1 of
`📓️wgpu-mesh-oracle-2026-09-14.md` §5.1 — *a click on geometry never selects on 7 of 8 examples in
BOTH lanes* — is closed.

Repo MCP was down all session (`repo CONNECTION_CLOSED`, `semio CONNECTION_CLOSED`); no ticket was
opened, closed or reopened. No `git commit/stash/checkout/worktree`. Nothing under `🗑️generated`
that this lane did not create was touched. No peer process was killed.

---

## 1. TL;DR

**The seam is one line of the guest's publication and one missing fold in the browser wgpu bridge.**

A framework-reserved interaction verb is ADMITTED by the dispatch turn with an empty
`InvocationResult` and an `Effect::SpawnJob`; the real answer — `output.interactionView` plus the
app-declared refresh scope — is published by the job's COMPLETION turn as

```rust
frames.push(protocol::AppFrame::Invocation {
    in_reply_to: 0,                                   // 🔥 answers no command sequence
    output: encode_wire_serialized(&result.output),   //    interactionView
    ui_scope: encode_wire_serialized(&result.ui_scope),
    …
});
```
`💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `plugin_complete_reserved_spawned_job`.

`AppChannelClient` correlates every reply by `in_reply_to`, and sequences start at **1**
(`AppChannelRequestSequence::nextSequence` → `++this.sequence`), so `in_reply_to: 0` names **no
caller**. The channel's own pump classifies such a frame as operation PROGRESS and hands it to
`onOperationProgress` (`💻️os/🟦️.ts:3628`) — a lane the **React** runtime subscribes to and the **wgpu**
bridge subscribes to **nowhere** (`grep -n "onOperationProgress" 🐚️plugin-bridge/🟦️.ts` → 0 hits).

React never lost it for a second reason: its `deliverJobCompletionTurn` parks a job turn's host
effects in the LEFTOVER lane untouched, and `invocationFromFrames` peels them back with
`leftoverShellInvocationFrames`. The wgpu bridge did the opposite — `drainSpawnedJobs` converted the
job's host effects into correlated FRAMES (`shellFrameBytes`) and `stashLeftoverHostEffects`
explicitly dropped every shell frame from the leftover — so the settled answer existed on a lane with
no reader at either end and was destroyed by the channel's correlation.

Consequence, measured verbatim on 6118 (`📓️wgpu-mesh-oracle-2026-09-14.md`'s own evidence folder,
`🗑️generated/wgpu-oracle/world3d-editor/box-shell-preview/console.txt`):

```
75967 [DEBUG] wgpu-bridge spawn-job done instance=1 job=530 kind=framework.reserved.tool … outcome=ok
76138 [DEBUG] wgpu-bridge spawn-job settled instance=1 job=530 turns=3 status=idle frames=2
76138 [DEBUG] wgpu-shell dispatch action=interactionSelect scope=none
76141 [DEBUG] wgpu-shell refresh scope=none rendered=0        ← nothing re-rendered, ever
```

`rendered=0` is the whole defect: the guest's interaction store DID take the pick — the same console
line 75969 prints the framework's own loss detector with `readback=graph=node:shell@solid` — but no
surface was ever asked to re-render, so `World3dScene.selection_json` was never rebuilt and stayed
`{"ids":[]}` (`selectionLaneBefore: 172, selectionLaneAfter: 172`, byte-identical across the whole
run) for every hop of every example.

**The fix is on the lane, not on the app**: an `AppFrame` that answers no sequence now stays in the
leftover lane, where `wgpuInvocationFromFrames` folds it exactly as `PluginRuntime` folds its own.
With that, `interactionSelect` answers the app-declared refresh scope, the shell refreshes, the guest
republishes `selection_json` from `interaction.selection("graph")`, and the World3d surface publishes
the pick. Measured live on 6118 the moment the fixed bundle went up (§5.1): the same dispatch line
goes from `scope=none rendered=0` to `scope=full rendered=3` on all eight examples, and every example
whose pick actually reached geometry published `['fuse@solid#0']` / `['brep_bool_cut_5@solid#0']` —
exactly the ids the native law in §4.2 predicts.

---

## 2. What was ruled OUT, with evidence

`📓️wgpu-mesh-oracle-2026-09-14.md` §5.1 had already ruled out granularity pruning, topology
membership and the payload marks logic. This lane rules out four more, by reading the shipped code
rather than by guessing:

| candidate | verdict | evidence |
|---|---|---|
| the wgpu shell dispatches a different `windowId`/domain/granularity/target shape than React | **no difference** | wgpu stages `{domainId, targets: "[{granularity,id}]", merge, method}` (`♾️infinite/🌍️world/🦀️.rs`, `WorldFlatActionKind::Select`); React's `world3dSelectionActionArgs` builds `{domainId, targets: JSON.stringify(targets), merge, method:"pick"}` — the same four args, neither carries a `windowId`, and the runtime line on 6118 reads `args={"domainId":"graph","targets":"[{\"granularity\":\"object\",\"id\":\"shell@solid\"}]","merge":"replace","method":"pick"}`, which is the React shape verbatim |
| the guest's `interaction.selection("graph")` never took the pick | **it took it** | the guest's own loss detector printed `dispatched=graph=object:shell@solid;vortex=object:shell@solid validated=graph=node:shell@solid readback=graph=node:shell@solid;vortex=object:shell@solid` — the READBACK names the id |
| the wgpu shell keeps its own leftover selection overlay that needs the React lane's `leftoverWorldOverlayIdsInDocumentV1` law | **there is no wgpu overlay** | `grep -n "interactionView\|InteractionView\|leftoverWorld" 🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` → **0 hits**. The wgpu World3d surface paints from the guest's published `selection_json` alone, so it needs no overlay and no prune twin; the React overlay exists because that pane paints from HOST state (`worldSurfaceSelectionDomV1(selection, interaction)`) |
| the NATIVE wgpu shell has the same drop | **it does not, for a different reason** | `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `invocation_from_frames` also matches `in_reply_to == seq` and so also never folds the settled answer — but its `ui_scope` default is `UiDirtyScope::default()` = `Full`, so the native shell refreshes anyway and the selection appears. Named here because it is a masked instance of the same seam, not fixed by this lane (§7) |

---

## 3. The fix, file by file

### 3.1 One lane rule, both renderer targets

`🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` — `leftoverShellInvocationFrames(leftover, decodeAppFrame)`
is now the ONE law, in the module whose own doc already promised it (`WIRE_SEND_MESSAGE_ROUTED_TARGETS`
names it as "`leftoverShellInvocationFrames` for a frame that lands on a later turn than the call it
answers"). The decoder is injected, the way `decodeWirePatchOps` already injects `decodePackValue`,
because the `AppFrame` codec lives one layer above this module.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — its
local copy is deleted and replaced by a three-line wrapper over the shared law; its
`applyInvocationFrame` now guards mutations/inverse-group per field like every other carrier, so a
completion frame that carries none cannot blank an admission that did.

### 3.2 The wgpu bridge: an uncorrelated frame belongs in the leftover lane

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`

- `shellFrameAnswersACaller(payload)` (new, exported) — `false` for an `AppFrame` whose
  `in_reply_to` is `0`; undecodable bytes answer `true`, so only a positively-uncorrelated frame is
  rerouted.
- `stashLeftoverHostEffects` keeps such an effect in `pendingTurnEffects` instead of dropping it
  with every other shell frame.
- The four places that split a turn's host effects into frames/leftover
  (`drainSpawnedJobs`, `runQueuedTurnSerialized`, `drainTypedOperations`, the extension-answer pump)
  no longer push an uncorrelated frame onto the correlated lane.
- `wgpuInvocationFromFrames(frames, leftover)` (new, exported) is now the ONE place this target
  decides what a dispatch answered: it folds the frames, then folds every leftover shell
  `Invocation`, per field. `performInvocation` is three lines over it.

No Rust changed, and **no renderer wasm rebuild was needed**: the bridge is bundled into
`🎞️frame-worker/🤖️generated/🟨️.js` by
`nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker`, which the 6118 serve mounts
straight off disk (`wgpuBrowserMounts`, `/🎞️frame-worker.js` → `…/🎞️frame-worker/🤖️generated`).
Verified served, not just built:

```
curl -s http://127.0.0.1:6118/🎞️frame-worker.js/🟨️.js → 200, 1 168 021 B
grep -c shellFrameAnswersACaller → 6
```

---

## 4. Laws

### 4.1 TS — the host fold, over a shared fixture

Fixture: `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json` (new) — the
admission frame, the settled frame, and what a correct fold must answer.
Law: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🕹️wgpu-selection-roundtrip/🟦️.ts`
(new, registered in `🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`).

```
bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🧪️tests/🕹️wgpu-selection-roundtrip/🟦️.ts
 Test Files  1 passed (1)
      Tests  4 passed (4)
```

**Failing-first, measured.** With `shellFrameAnswersACaller` reduced to `return true` and the
leftover fold removed — the exact pre-fix shape — the same law answers:

```
 × routes the admission onto the frame lane and the settled answer onto the leftover lane
     AssertionError: expected true to be false
 × folds the settled answer's interaction view and refresh scope out of the leftover lane
     AssertionError: expected [] to deeply equal [ 'shell@solid' ]
 × keeps an admission that carried its own answer when a completion frame carries none
     AssertionError: expected 'full' to be 'none'
      Tests  3 failed | 1 passed (4)
```

The third case in the suite is the counter-model kept permanently: folding the FRAME lane alone
answers `uiScope.kind = "none"` and no `selectedIds` — the shell's `scope=none rendered=0`.

### 4.2 Rust — the app half, fixture-driven over all 8 examples

`✏️s/…/🧊️generation3d/…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — `assert_selection_round_trip`, scored by
the eight existing `delivery_*` tests on the SAME delivered session, driven by each fixture's own
`preview.node`/`preview.channel` (no new fixture field, no fudge factor). Four laws per example:
a click on the example's own target publishes that target in `World3dScene.selection_json`; the
viewer window publishes the same for the same pick; selecting every published target publishes every
published instance and never fewer than the single pick did (shift-click ADDS); an empty selection
publishes none (Escape / empty-space click).

`RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry delivery_ -- --nocapture`
→ **8 passed, 0 failed**, measured `[SELECTION]` rows:

| example | picked target | topology ids | click → ids | shift-add → ids | clear → ids | viewer click → ids |
|---|---|---|---|---|---|---|
| rectangle-wire-preview | `rect@wire` | 1 | `["rect@wire#0"]` | `["rect@wire#0"]` | `[]` | `["rect@wire#0"]` |
| rectangle-extrude-volume | `extrude@solid` | 1 | `["extrude@solid#0"]` | `["extrude@solid#0"]` | `[]` | `["extrude@solid#0"]` |
| face-sweep-extrude | `extrude@solid` | 1 | `["extrude@solid#0"]` | `["extrude@solid#0"]` | `[]` | `["extrude@solid#0"]` |
| hexagonal-mushroom-column | `extrude@solid` | 3 | `["extrude@solid#0"]` | `["profile@wire#0","extrusion-axis@vectorOut#0","extrude@solid#0"]` | `[]` | `["extrude@solid#0"]` |
| box-shell-preview | `shell@solid` | 1 | `["shell@solid#0"]` | `["shell@solid#0"]` | `[]` | `["shell@solid#0"]` |
| box-fillet-preview | `fillet@solid` | 1 | `["fillet@solid#0"]` | `["fillet@solid#0"]` | `[]` | `["fillet@solid#0"]` |
| sphere-box-fuse | `fuse@solid` | 1 | `["fuse@solid#0"]` | `["fuse@solid#0"]` | `[]` | `["fuse@solid#0"]` |
| sphere-cut-with-torus | `brep_bool_cut_5@solid` | 1 | `["brep_bool_cut_5@solid#0"]` | `["brep_bool_cut_5@solid#0"]` | `[]` | `["brep_bool_cut_5@solid#0"]` |

Only `hexagonal-mushroom-column` has more than one geometry-bearing channel, so it is the only
example on which "shift-click ADDS" can grow the set — and it does, 1 → 3.

### 4.3 Rust — the example-switch prune, re-run not re-written

`cargo test -p semio-framework-plugin --lib interaction_selection_laws` → **3 passed, 0 failed**
(`a_document_change_prunes_the_vortex_mirror_and_the_leftover_cover_with_the_selection` and
siblings). That law is lane `selection-prune-interact`'s, landed 2026-09-14; it is renderer-agnostic
and this lane only confirms it still holds.

---

## 5. Battery

### 5.1 The seam, measured the moment the fixed bundle went up

The bundle was republished at **10:04**. A PEER's `--only=world3d-editor,world3d-viewer` run was in
flight across that instant, and its own console is the cleanest A/B this lane has, on one build, one
probe and one machine — `🗑️generated/wgpu-verify/` (that lane's root, read only, never written to):

| lane / window | `wgpu-shell dispatch action=interactionSelect scope=` |
|---|---|
| editor lane, examples 1-6 (started 09:52, pre-bundle) | `scope=none` × 24 |
| editor lane, examples 7-8 (ran past 10:04) | `scope=full` × 7 |
| viewer lane, all 8 examples (started 10:08) | `scope=full` × 32 |

and the per-example publication on the viewer lane, where the pick reached geometry at all:

```
sphere-box-fuse        h3 selection lane 168 → 192   published selected ['fuse@solid#0']
sphere-cut-with-torus  h3 selection lane 168 → 214   published selected ['brep_bool_cut_5@solid#0']
```

Those are exactly the ids §4.2's native law predicts for those two examples. Before this fix the
same lane was `168 → 168`, `selected []`, on all eight (`📓️wgpu-mesh-oracle-2026-09-14.md` §5.1).

### 5.2 This lane's own run

`SEMIO_BATTERY_ROOT=wgpu-select bun 🐍️wgpu-battery.mjs --only=world3d-editor,world3d-viewer`
against `http://127.0.0.1:6118/?plugin=generation3d`, 11:48-12:20, **1 914 s, 0 page errors**,
scoreboard `🗑️generated/wgpu-select/scoreboard.json`. 31 of 31 `interactionSelect` dispatches traced
`scope=partial windows=[procedural.play.main,procedural.play.preview,procedural.play.generate-preview]
panels=[…]`, **0 traced `scope=none`** — the app-declared scope a peer landed mid-lane (§7), reaching
the shell through this fix.

| lane | steps | seconds |
|---|---|---|
| world3d-editor | 69/115 | 964 |
| world3d-viewer | 64/115 | 950 |

**The lanes are NOT green, and the reason is not the selection seam.** In 13 of the 16 example rows
the renderer's World3d authority reported `objects=0` at the very first hop — the guest had published
its meshes (`dumpMeshStats` answers the committed role map for 8 of 8 in both lanes, and the
`publishes exactly its committed meshes` step is green 16/16), and the renderer's own mesh STORE had
them (`state-meshes=3`), but its DRAWS never rebuilt: 98 consecutive frame lines of
`state-draws=0 state-instances=0 state-meshes=3 apply=false apply-page=0/0` for
`hexagonal-mushroom-column`, which only reaches `state-draws=1 state-instances=1` at hop 8. A pick
with nothing on screen to hit dispatches `targets: "[]"`, which is a correct empty selection — and the
guest's own loss detector stays silent, because no id was ever carried.

In **every** row where geometry did reach the renderer, the round trip works:

| lane | example | `objects` at h1 | h3 published selection | h4 published | h4 authority |
|---|---|---|---|---|---|
| editor | box-shell-preview | 1 | `['shell@solid#0']` | `['shell@solid#0']` | 1 |
| editor | sphere-cut-with-torus | 1 | `['brep_bool_cut_5@solid#0']` | `['brep_bool_cut_5@solid#0']` | 1 |
| viewer | box-shell-preview | 1 | `['shell@solid#0']` | `['shell@solid#0']` | 1 |
| editor / viewer | the other 13 rows | **0** | `[]` | `[]` | 0 |

3 of 3, against 0 of 16 before. `h3 the selection reaches the published document` — the step that was
red on 7 of 8 in BOTH lanes and is the one this lane owns — is green on every row whose pick had a
body to land on, and `h4 shift-clicking adds to the selection rather than replacing it` goes
`before: 0 → after: 1` on `box-shell-preview` in both roles.

One residual latency, named rather than papered over: at h3 the renderer's INTERACTION authority census
still reports `selected: 0` while `dumpMeshStats` already reports `selected: ['shell@solid#0']` for the
same surface at the same instant; the census catches up by h4. So `h3 clicking the body selects that
target` (which reads the census) stays red while `h3 the selection reaches the published document`
(which reads the publication) is green. That is one frame-build of `sync_world3d_state` lag inside the
renderer, not a lost selection.

---

## 6. Files

**Changed (the fix):**
- `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` — `leftoverShellInvocationFrames`, the shared lane law.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` — `shellFrameAnswersACaller`, `wgpuInvocationFromFrames`, the four frames/leftover splits, `AppFrameInvocationPayload`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` — its local peel replaced by the shared law; per-field mutations/inverse-group fold.

**Changed (the laws):**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — `assert_selection_round_trip`, `editor_selection_ids`, `viewer_selection_ids`, `selection_json_ids`, four new `DeliveryRun` lanes and the `[SELECTION]` line.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` — the new suite registered.

**New:**
- `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🕹️wgpu-selection-roundtrip/🟦️.ts`

**Rebuilt (no Rust):**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js` — `nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker`.

**Ticket:**
- `📓️wgpu-selection-roundtrip-2026-09-15.md` (this report), `📜️run-wgpu-select.sh` (the gated runner),
  `🗑️generated/wgpu-select/` (this lane's evidence root).

---

## 7. What is NOT claimed

- **The NATIVE wgpu shell's `invocation_from_frames` was not changed.** It matches `in_reply_to == seq`
  and so also never folds the reserved verb's settled answer; it is masked because its `ui_scope`
  default is `Full`, so it refreshes everything regardless. That is a real (if invisible) instance of
  the same seam, on a target this ticket's gate does not exercise, and closing it needs a native run
  to prove — not done here.
- **The refresh scope this lane delivers is the APP's, and the app's scope is not this lane's.**
  When the seam was opened at 10:04 generation3d declared no `ArtifactApp::interaction_scope`, so the
  framework's widest answer arrived and the shell traced `scope=full rendered=3`. A peer landed
  `✏️editor/🧪️tests/🕹️interaction-scope/` and restaged during this lane's own wait, and the same
  dispatch now traces `scope=partial windows=[procedural.play.main,procedural.play.preview,
  procedural.play.generate-preview] panels=[…]`. Both are this fix working; neither narrowing is mine.
- **No host-side selection overlay was added to the wgpu shell**, because it keeps none: it paints
  from the guest's published `selection_json` alone (§2). The React `leftoverWorldOverlayIdsInDocumentV1`
  law therefore has no wgpu twin to give, and none was invented.
- **The renderer wasm was not rebuilt and the guest was not restaged.** Neither was needed; the
  measurements below therefore ride whatever renderer/guest build the coordinator's 6118 serve was
  carrying, which this lane did not pin.
- **`🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts` is red and is not this lane's.** It is a SOURCE SCAN over
  `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` counting `terminalCursorRelease()` occurrences and finds 3 where the
  fixture declares 2; this lane never edited that file. `🧪️tests/🧩️package-integration/🟦️.ts`'s 6 reds are
  `ReferenceError: Bun is not defined` — an artefact of running vitest under node instead of the
  project's own bun script.
- **`semio-framework-os-renderer-wgpu`'s `async_boundary_tests` fail 4 of 16** before any vitest runs
  (`native_binary_owns_exactly_one_entrypoint_driver` and three siblings). They fail identically at
  HEAD and were recorded as pre-existing by `📓️wgpu-hit-registry-drain-2026-09-14.md` §7.
- **The `boot camera frames the committed bounding box` and `box-fillet-preview` centre-hover reds of
  `📓️wgpu-mesh-oracle-2026-09-14.md` §5.2/§5.3 are not this lane's** and were not touched.
- **`world3d-editor`/`world3d-viewer` are NOT green, and this lane does not claim them.** 13 of 16
  example rows never got a DRAW into the renderer (§5.2), so 13 of 16 picks had nothing to hit. This
  lane did not diagnose that and does not own it: the mesh store holds the geometry
  (`state-meshes=3`) while the draw rebuild sits at `apply=false apply-page=0/0`, which is the
  `wgpu-edit-convergence-perf` surface.
- **Whether this fix makes that starvation WORSE is an open question this lane could not answer
  honestly.** Before it, `interactionHover`/`interactionSelect` refreshed nothing at all
  (`scope=none rendered=0`); after it each one re-renders the preview body, which restages the
  World3d scene bridge. The `objects=0` rows appear in runs on both sides of the app-declared
  narrowing a peer landed (`scope=full` at 10:08 → 2 of 8 rows had draws; `scope=partial` at 11:12 →
  0 of 8; `scope=partial` at 11:48 → 2 of 8 editor, 1 of 8 viewer), so the narrowing did not move it
  — but settling this needs an A/B against a deliberately un-fixed bundle on the shared 6118, which
  would corrupt whatever peer run is in flight. It was not done, for the same reason
  `📓️wgpu-hit-registry-drain-2026-09-14.md` §7 refused the same experiment.
- **The machine was loaded throughout.** Eight React vite serves, two peer batteries and a peer's
  `prune-incremental` loop were running during the measurement window, and the standing wgpu probe
  gate held this lane's own run off 6118 for 100 minutes. No number here was taken on a quiet box.
- **The gate script had to be repaired before it could clear.** A plain
  `pgrep -f 'wgpu-batter[y]|wgpu-.*-pro[b]e'` also matches every peer agent's own gate SHELL, whose
  command line quotes that pattern: four of them were waiting on each other at 11:47 with no probe
  running at all. `📜️run-wgpu-select.sh` anchors on `^bun ` instead. Nothing was killed to clear it.
