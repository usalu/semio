# Incremental Re-Evaluation Of A Moved Slider — lane `slider-latency-incremental-eval` (2026-09-15)

Picks up `📓️slider-preview-update-2026-09-15.md` §7 item 3: *"first mesh is ~1 s, not ≤ 150 ms … the
remaining latency is the GUEST's evaluation chain."* Owns the incremental-evaluation machinery —
dirty-set precision, the tessellation cache, progressive publication, and the per-stage attribution
that says where the second goes. Port 6025.

- Recon: `🐍️incremental-eval-recon.mjs` (new) — ONE slider step from rest, sampled every 50 ms with
  the per-mesh id + content digest, the chain census (`progress.nodesDone/nodesTotal`), the in-page
  `performInvocation` ledger and the renderer's `semio.hop.*` User Timing spans on one clock.
- Gate: `🐍️slider-live-preview-probe.mjs` (existing), `🐍️journey-probe.mjs` (existing).
- Laws: `🧵️preview-eval/🧪️tests/🔁️incremental/🦀️.rs` (new Rust lane `incremental-eval`) over
  `🧫️fixtures/🔁️incremental-eval.json` (new), TS twin in
  `📺️renderer/🧑‍🎨engine/🧪️tests/🧊️world3d-mesh-residency/🟦️.ts` over the extended
  `🌐️World3dHost/🧫️fixtures/🧊️mesh-residency.json`.
- Runtime logs: `🗑️generated/slider-latency/`

---

## 1. TL;DR

**The evaluation chain was never the second.** On the 17:10 guest, ONE slider step of the hex column
cost 2 685 ms to settle — and the guest's own dag walk inside it measured **2.3 ms**
(`worker.guest` on the `flowEvalTick` request span), with the brep round trip at 20.6 ms. The second
was spent almost entirely in **whole-shell UI refreshes that the chain itself asked for**: every hop
of the chain emitted the default `UiDirtyScope::Full`, and each such refresh re-rendered
`procedural.play.main` — the node-graph body, the most expensive body in the app — at a 183–266 ms
patch install. Three of them, 452 + 574 + 558 ms, for a walk that moved one number.

Worse than the cost: **the geometry the hop produced could not reach the screen until that refresh
finished.** The tick settled at +838 ms and the first mesh payload landed at +1 312 ms — 474 ms of
pure queueing behind a refresh of bodies that had not changed.

And what landed was a payload with the geometry MISSING. A budgeted walk emits no entry at all for a
node whose extension request is parked, so the preview lost the whole recomputing branch: three
meshes fell to one at +1 312 ms and the extruded solid was off screen for 1.4 s of a 2.7 s
re-evaluation. The knob moved and the model disappeared.

Two fixes, each on the layer that owns it:

1. **The chain hop declares what it actually invalidates.** `flowEvalTick`, `flowEvalResolve` and
   `flowTessellateResolve` now emit `UiDirtyScope::Partial` over the addressed preview body plus the
   two panels that read the chain's own progress back, and name the graph body ONLY when the hop
   moved the chrome the per-node census paints.
2. **A node whose answer is still crossing keeps the answer it had.** The session retains the last
   CONVERGED evaluation and paints the live walk over it, so a recomputing branch keeps its geometry
   until the new geometry arrives, and its mesh — keyed by an unchanged brep handle — is neither
   re-tessellated nor re-delivered.

What was ALREADY right, and is now stated as a law rather than assumed: the dirty set. Moving
`height` on the hex column dirties exactly `height → extrusion-axis → extrude`; `radius`, `sides` and
`profile` free-ride on the converged baseline, `profile@wire` keeps its content address, and exactly
one mesh is owed a `tessellate` round trip.

---

## 2. The chain, stage by stage

One slider step (`Column Height` 6 → 7), 6025, guest 17:10, load average 3.96. `t` is milliseconds
from the pointer press. Sources: `🗑️generated/slider-latency/recon-before/{ledger,spans,changes}.json`.

| t | stage | cost | who owns it |
|---|---|---|---|
| 25 | `nodeGraphEdit` (live tick) dispatched → settled at 81 | 56 ms | slider-preview-update ✅ |
| 78 | `refresh` **partial** — the edit's own UI refresh, 2 windows + 2 panels | **405 ms** | the edit's scope, already narrowed |
| 140 | `patch.install 1:procedural-main` inside it | 280 ms | flow-surface-followup |
| 134 | `toolRunStart` dispatched — **queued behind that refresh**, crosses at 585 | **451 ms** | the serialized Interactive lane |
| 589 | `flowEvalTick` ×2 dispatched (edit preview + generate preview) | 214 / 249 ms | this lane |
| — | the dag walk itself (`worker.guest`, `request`) | **2.3 ms** | — |
| — | the brep `evaluate` round trip (`worker.guest procedural#1`) | 20.6 ms | — |
| 736 | `refresh` **full** — asked for by the tick's settle | **452 ms** | **this lane** |
| 880 | `patch.install 1:procedural-main` inside it | 184 ms | — |
| 1 189 | `refresh` **full** again | **574 ms** | **this lane** |
| 1 297 | `mesh.decode` — the FIRST mesh reaches the surface | — | — |
| 1 312 | first published payload: **1 mesh** (`eval-profile@wire#0`), nodes 5/7 | — | — |
| 1 394 | `patch.install 1:procedural-main` | 266 ms | — |
| 1 763 | `refresh` **full** again | **558 ms** | **this lane** |
| 1 834 | 2 meshes — the vector marker returns, nodes 6/7 | — | — |
| 2 685 | settled `idle`, **2 meshes** — the extruded solid never returned | — | slider-reevaluation-correctness |

**Attribution of the 2 685 ms:** 1 584 ms in three chain-driven FULL refreshes, 451 ms of the
evaluation hop queued behind the edit's own refresh, 463 ms in the two `flowEvalTick` round trips,
~23 ms of actual evaluation and kernel work.

The gesture reading from the same stage, for the record (`🗑️generated/slider-latency/before-graph`):
a 1 s 60 Hz drag = 9 value changes, 0 dropped actions, 5 `nodeGraphEdit` / 3 `toolRunStart` /
5 `flowEvalTick`, first-mesh median **1 511 ms**, p95 2 460 ms — and the "first mesh" it timed was the
payload going from 3 meshes to 1, i.e. the geometry LEAVING the screen.

---

## 3. The fixes

### 3.1 The chain hop's UI scope — `🧵️preview-eval/🦀️.rs`, `✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`, `👁️viewer/🦀️.rs`

`preview_eval::chain_ui_scope(preview_body, graph_body, panel_bodies, census_moved)` is the
surface-neutral rule; each surface binds its own body keys.

- **Always** the addressed preview body (`procedural.play.preview` /
  `procedural.play.generate-preview` / `procedural.view.preview`) — that is what carries the geometry.
- **Always** the panels that read the chain's own progress back: `procedural.play.artifact` and the
  framework tool-run panel.
- **The graph body only when the chrome the census paints moved.** `FlowEvalTickOutcome` gained
  `census_moved`, a digest taken before and after the walk over `census_chrome_marks` — the census
  reduced to what the graph body actually paints DIFFERENTLY: the chrome being on at all, and which
  nodes carry a fault. Deliberately not the whole census: every hop of a live evaluation reshuffles
  `computing`/`queued` among nodes that are all already busy, and the first cut of this fix (digesting
  the census verbatim) still paid a 188–265 ms `patch.install` of `procedural.play.main` four times
  per evaluation — 974 ms of a 2 529 ms re-evaluation, spent in front of the very geometry whose
  progress it was reporting. **A progress indicator may not outrank the thing whose progress it
  reports.** The chrome is painted when it appears, when it clears, and whenever a fault lands or
  changes; the per-hop advance the user watches is the preview's own status pill, republished every
  hop.
- **Never `None`.** The host polls `pending_effects` once per `refreshUi`; a hop that asked for no
  refresh would be the last hop a settled chain could take. The same reason
  `owe_attached_previews_carrying` carries a run start rather than trusting a poll.

All three chain routes travel through `flow_eval_tick::evaluate`, including a declined inline
continuation, so there is one scope decision and no route can drift from it.

### 3.2 The painted evaluation — `🌊️flow/🖥️host/🦀️.rs`

`FlowEvalSession` gained two retained texts beside `eval_json`:

- `converged_eval_json` — the last evaluation that CONVERGED, advanced with the incremental baseline
  under the same `remaining.is_empty()` condition and never apart from it.
- `painted_eval_json` — what a preview PAINTS: the live walk's own answer, with every CURRENT widget
  the live walk has not answered yet filled in from `converged_eval_json`
  (`merge_unanswered_eval_entries`). Identical to `eval_json` the moment the walk converges.

Absence is the whole rule: a budgeted walk writes an entry for every node it resolved and none at all
for a node whose extension request is parked, so *"missing from the live answer, present in the
converged one, still declared by the document"* is exactly *"this node's new answer is still
crossing"*. A node the walk answered — **including one it answered with an `error`** — keeps its live
answer, so this can never paint over a real failure; and the fallback is filtered through the host's
CURRENT widget roster, so switching example replaces the graph instead of painting the previous
example's leftovers over the new one.

`flow_eval_publication_for` publishes the painted text. `eval_json` stays the live walk's own answer,
because that is what the next walk and every convergence law read.

### 3.3 The mesh the preview keeps — `🧵️preview-eval/🦀️.rs`

`preview_tessellate_invocations` now retains against the PAINTED evaluation.
`FlowEvalSession::retain_preview_meshes` drops every mesh whose handle is not in the set it is given,
and a node that is merely recomputing has no handle in the live answer at all — so retaining against
the live answer threw away the very mesh the preview was still showing, and the geometry could not
come back without a round trip it had already paid. A stale handle that already holds a mesh is
skipped by `pending_preview_tessellate_handles`, so nothing is re-tessellated for it.

### 3.4 The dirty set, stated rather than assumed — `🌊️flow/🖥️host/🦀️.rs`

`FlowHost::dirty_widget_ids_since(&previous_host)` — the incremental contract as a number a law can
read, with no contributed operator table and no dispatch: one `compute_dirty_set` over the same two
`TreeSnapshot`s `evaluate_step` itself diffs, so the law states what the evaluator does rather than a
parallel rule that could drift from it.

### 3.5 One blocker fixed forward — `🐚️Shell/🟦️.tsx`, `🎯️targets/⚛️react/🟦️.tsx`

The TS twin could not run: `🧪️tests/🧊️world3d-mesh-residency` imported `🌐️World3dHost`, which imports
`🛠️ShellHelpers`, which imports `🐚️Shell`, whose module body called `shellLabel(...)` at TOP LEVEL for
`UI_INSPECTOR_MIXED_PLACEHOLDER` — reading a ShellHelpers binding ShellHelpers had not reached yet
(`ReferenceError: Cannot access '__vite_ssr_import_18__' before initialization`, 0 tests run). The
file's own header comment already names this hazard. It is now `uiInspectorMixedPlaceholder()`: a
function has no load-time order to get wrong, and the label is locale-resolved when it is read, which
is what a relabelled shell needs anyway. Two barrel lines followed. Both files were untouched for
over 90 minutes.

---

## 4. Laws

### 4.1 Rust — lane `incremental-eval`, over `🧫️fixtures/🔁️incremental-eval.json`

`✏️s/…/🧵️preview-eval/🧪️tests/🔁️incremental/🦀️.rs`. Driven on the REAL hex column
(`📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/…/🗣️.dsl.semio`) with the packaged `brep`/`math`
extensions installed, so the dirty set, the handles and the meshes are the kernel's own answers.

| law | what it states | measured |
|---|---|---|
| `an_edit_dirties_its_own_branch_and_leaves_every_other_node_to_free_ride` | `height` 6 → 7 dirties `["height","extrusion-axis","extrude"]` and `radius` 0.5 → 0.7 dirties `["radius","profile","extrude"]`; the clean nodes free-ride; every unchanged branch keeps its content address and every dirty one mints a new one | `[STATS] edit height 6 -> 7 dirty=["height", "extrusion-axis", "extrude"]`, `[STATS] edit radius 0.5 -> 0.7 dirty=["radius", "profile", "extrude"]` |
| `a_burst_of_values_costs_one_dirty_branch_per_value_and_never_a_growing_one` | 5 successive `height` values, baseline carried forward each time | `6.5 / 7 / 7.5 / 8 / 8.5` each `dirty=["height", "extrusion-axis", "extrude"]` |
| `only_a_changed_handle_is_owed_a_tessellate_round_trip` | a `height` edit re-tessellates at most ONE mesh, and the unchanged handle's mesh is byte-identical in `positions` and `indices` | `[STATS] height edit retessellations=1 budget=1` |
| `an_unanswered_node_keeps_its_converged_answer_and_an_answered_one_never_does` | the painted merge fills an outstanding node, never an answered one (error included), never a node the document does not declare, and never drops a live entry | green |
| `a_chain_hop_names_the_preview_and_names_the_graph_only_when_the_census_moved` | the hop's scope is `Partial`, always names the preview body and the two panels, names the graph body exactly when `censusMoved`, and turns every rail off | `censusMoved=false windows=["procedural.play.preview"]`, `censusMoved=true windows=["procedural.play.preview", "procedural.play.main"]`, `panels=["procedural.play.artifact", "framework.panel.toolRun"]` |
| `the_chrome_digest_moves_on_a_fault_or_a_chrome_edge_and_never_on_a_reshuffle` | 2 census pairs that paint the same chrome digest the same; 5 that must differ (chrome on, chrome off, a fault landing, a fault's message changing, a block landing) | `[STATS] chrome marks same=2 different=5` |

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly \
  --test incremental-eval -- --nocapture --test-threads=1
→ test result: ok. 6 passed; 0 failed
```

### 4.2 TypeScript twin — the host delivery

`📺️renderer/🧑‍🎨engine/🧪️tests/🧊️world3d-mesh-residency/🟦️.ts` over the extended neutral fixture
`🌐️World3dHost/🧫️fixtures/🧊️mesh-residency.json`, two new cases in the generation3d payload's own
mesh-id vocabulary:

- *"one slider edit rebuilds only the branch downstream of the knob"* — three meshes, the profile
  wire byte-identical, the axis and the solid moved → `reusedIds: ["eval-profile@wire#0"]`, `built: 2`.
  This is the delivery half of §3.3: the guest's tessellation cache is only worth anything if the host
  can see that the unchanged entry did not move, and it keys that on the element TEXT.
- *"a mid-gesture payload that omits a still-computing branch takes it off screen"* — the
  before-behaviour, stated so it cannot come back unnoticed: a payload that drops two of three meshes
  retires them, and the geometry is gone until the answer lands.

```
SEMIO_TEST_LEVEL=long bunx vitest run --config …/🎚️config/🟦️.ts world3d-mesh-residency
→ Test Files 1 passed (1), Tests 19 passed (19)
```

That lane answered **0 tests** before §3.5: it could not import at all.

---

## 5. Measured, after

_TO BE FILLED_

---

## 6. Files

Created:
- `✏️s/…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔁️incremental-eval.json`
- `✏️s/…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔁️incremental/🦀️.rs`
- `🐍️incremental-eval-recon.mjs`, `📜️restage-incremental.sh`
- `📓️slider-latency-incremental-eval-2026-09-15.md` (this report)

Updated:
- `✏️s/…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` — `chain_ui_scope`,
  `FlowEvalTickOutcome::census_moved`, `node_census_digest`, mesh retention against the painted eval
- `✏️s/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` — `preview_body_key`, the editor's
  `chain_ui_scope`, the scope on every `Emit` the three chain routes return
- `✏️s/…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs`,
  `✏️s/…/✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs` — the scope on the session-free `handle`
- `✏️s/…/👁️viewer/🦀️.rs` — `generation3d_view_chain_ui_scope`, the scope on the viewer's chain work
- `✏️s/…/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml` — the `incremental-eval` test lane
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `painted_eval_json`,
  `converged_eval_json`, `FlowEvalSession::repaint`, `merge_unanswered_eval_entries`,
  `FlowHost::dirty_widget_ids_since`, the publication reading the painted text
- `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🧊️mesh-residency.json` — the
  two slider-chain delivery cases
- `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx`,
  `🧰️framework/…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx` — `uiInspectorMixedPlaceholder()` (§3.5)

---

## 7. What is NOT claimed, and what is handed on

### Handed on — measured here, owned elsewhere

1. **The 451 ms the evaluation hop spends QUEUED behind the edit's own UI refresh.** The
   `nodeGraphEdit` that carries a slider value already declares the narrow
   `slider_gesture_ui_scope` (slider-preview-update), and that partial refresh still costs 405 ms —
   280 ms of it a single `patch.install` of `procedural.play.main`. The command lane is serialized per
   instance, so the `toolRunStart` the same edit armed cannot cross until that refresh has drained.
   Nothing this lane did changes it, and nothing this lane did could: the knob itself is painted from
   the graph body, so that body legitimately has to be republished. **The cost is the flow body's own
   render/patch size, not the scope.** Owner: `flow-surface-followup` (§9's 88 KB/23 KB once-per-attach
   payloads is the same body).
2. **The extruded solid does not come back on a settled single-step evaluation.** The recon's settled
   state is `idle` with two meshes and no `eval-extrude@solid#0` — the same defect the coordinator
   read as `1 of 7 nodes did not evaluate` and `📓️slider-preview-update` §7 item 1 read as an empty
   payload under `idle`. The painted evaluation of §3.2 keeps the LAST GOOD solid on screen while the
   new one is crossing; it deliberately does **not** invent one for a node the chain answered with
   nothing, because that is a fault that must be named. Owner: lane
   `slider-reevaluation-correctness`.
3. **Two `flowEvalTick` hops per arm.** Both attached preview windows (`procedural-preview` and the
   generate mode's `generation3d-generate-preview`) are ticked on every arm, even when the generate
   preview is not on screen and returns immediately for want of a selected generation. Each costs a
   real round trip (214 and 249 ms on the measured step). Whether an unmounted/invisible preview window
   should be in `attached_preview_windows` at all is a windowing question, not an evaluation one.

### Not claimed

- **Nothing about the wgpu surface.** wgpu is ON HOLD; the guest halves (`chain_ui_scope`, the painted
  evaluation) are surface-neutral and serve it by construction, but nothing here was measured on it.
- **No claim that the ≤ 150 ms budget is met.** See §5 for the measured after-numbers and exactly
  which terms remain.
- **`deliver only changed meshes` is a CONTENT-HASH property here, not a delta protocol.** The guest
  still serializes the whole `meshesJson` array per publication; what §3.2/§3.3 buy is that an
  unchanged mesh's element text is byte-identical, which is the key the host's residency
  (`advanceWorldMeshResidency`) reuses its `BufferGeometry` on — proved by the TS twin. A wire format
  that ships only the changed entries was not built.
- **The dirty set was already precise.** This lane did not make it so; it made it a law
  (`dirty_widget_ids_since` + the `incremental-eval` lane) after measuring that the profile branch was
  already free-riding. The report says so rather than claiming a fix.
- **No guest-side span vocabulary was added.** The per-stage attribution in §2 is built from the
  renderer's existing `semio.hop.*` spans (including `worker.guest` / `worker.turn`, which ARE the
  guest's own wall time, handed back on the reply) plus the chain census the preview already
  publishes. A separate in-guest span ring would have measured the same 2.3 ms from the inside.
