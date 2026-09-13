# 👁️ Viewer / Generate Status Parity and the Shared Settle Path — 2026-09-13

Lane **viewer-generate-status-parity** · ticket `26/09/09/PROCEDURAL-3D-END-TO-END` · servers
`http://127.0.0.1:6018` (React) and `http://127.0.0.1:6118` (wgpu).

Abbreviations: `T` = this ticket folder; `any/` =
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`;
`flow-host.rs` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`.

---

## 1. TL;DR

| | |
|---|---|
| **(a) status parity** | **Already true in the served guest, and now proven from the browser.** All three preview windows publish the identical `World3dComputeStatusV1` key set. The brief's premise — "the viewer reports `idle` with `null` progress fields" — was a misread of the journey probe's own console line, whose two `null`s are `fault` and `widgetIds`, not progress. §3. |
| **(a′) shared SETTLE path** | **Real defect, found and fixed.** `evaluate_tick` recorded the EVALUATION's `more` as the window's `unfinished` flag — and `!more` is exactly the branch that parks `tessellate`. A window still owing mesh round trips recorded itself FINISHED, so `window_tick_owed` went false for good and the terminal tick could only ever come from one tessellate answer arming the next. The first answer that came back `Ready` while another handle was untessellated stranded the surface at `inFlight: 1` forever. §4. |
| **(b) `No example`** | **Edit and generate were already correct** (measured: preview window stays mounted, 0 meshes, 0 instances, `idle`/`ratio 1`). **The viewer was not**: its config could not tell "never picked" from "picked `No example`", so the row resolved to the opened document — which in the playground IS a bundled example — and painted the hexagonal mushroom column. §5. |
| **(c) probe predicate** | The old predicate reached past the status contract into the FLOW window's per-node status map, which neither the viewer nor the generate preview has. Every `view:*` step and `generate-added` therefore burned its full 60 s budget while the surface had been `idle`/`ratio 1` the whole time. §6. |
| **(d) runtime proof** | §7. |
| **NOT claimed** | §9. |

---

## 2. What the brief asked vs. what was actually broken

The brief named four symptoms from `T/🗑️generated/s4-journey-1/`. Measured against the served guest,
they resolve into **one probe defect, one real guest defect in the settle path, and one real guest
defect in the viewer's picker** — plus one symptom that was an artefact of that run alone:

| brief symptom | verdict |
|---|---|
| (1) viewer `converged=false` although `meshes ≥ 1`, status `idle` with `null` progress | **probe predicate**; the status was complete (§3) |
| (2) `generate-added` `converged=false` with `meshes=1` | **probe predicate**; same shape (§3, §6) |
| (3) `view:No example` shows `meshes=3` | **real guest defect** (§5) |
| (4) `edit:No example` burns 60 s at `converged=false meshes=0` | **probe predicate** for the verdict; the 60 s itself was that run's mid-run plugin hot-swap (§8) |

And the coordinator's follow-up — the wgpu viewer stalling on the two boolean examples — is the
settle-path defect in §4, which is the one that actually mattered.

---

## 3. (a) The served status contract, read from the browser

`T/🐍️status-parity-probe.mjs` (new) reads the whole `data-status-json` off every World3d host in
each of the three windows. Run `T/🗑️generated/viewer-status/parity-1/`, 6018, staged wasm of 17:34.

Published key set, **identical in all three windows**:

```
["phase","phaseLabel","progress","cancellable","cancelAction","debug"]
```

(`generation3d-generate-preview` adds `hint` while it has nothing to show — a declared state of that
window, not a schema difference.)

Verbatim, one row per window:

```
edit    window:procedural-preview        {"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
  "progress":{"unitsDone":122,"unitsTotal":122,"facesDone":26,"facesTotal":26,"inFlight":0,
              "evalUnitsDone":0,"evalUnitsTotal":0,"ratio":1},
  "cancellable":false,"cancelAction":"cancelPreviewEval","debug":{"meshesLen":19231,"instancesLen":219}}

generate window:generation3d-generate-preview  … same keys … "debug":{"meshesLen":19231,"instancesLen":219}

viewer  window:procedural-view-preview   {"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
  "progress":{"unitsDone":41,"unitsTotal":41,"facesDone":7,"facesTotal":7,"inFlight":0,
              "evalUnitsDone":0,"evalUnitsTotal":0,"ratio":1},
  "cancellable":false,"cancelAction":"cancelPreviewEval","debug":{"meshesLen":32860,"instancesLen":211}}
```

So `📓️viewer-status-parity-2026-09-12.md`'s claim holds against the SERVED contract, and the single
projection it introduced — `any/🧵️preview-eval/🦀️.rs` `preview_window_status_json` (one call site per
window: `any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:85`,
`any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:63`,
`any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:368`) — is what produces all three.
**No status-projection change was needed and none was made.**

---

## 4. (a′) The real defect: the settle path was not shared

### 4.1 Root cause

`any/🧵️preview-eval/🦀️.rs`, `evaluate_tick`:

```rust
} else if !more {
    extension_invocations.extend(preview_tessellate_invocations(window_id, window_kind_id, session, fixture, tolerance));
}
session.note_window_tick_outcome(window_id, more);      // ← the defect
```

`more` is the EVALUATION's own "there is more to compute". `!more` is literally the branch that parks
the `tessellate` invocations — so the tick that hands off to tessellation recorded
`unfinished = false`.

`flow-host.rs:3015` then reads:

```rust
pub fn window_tick_owed(&self, window_id: &str) -> bool {
    match self.window_tick_latches.get(&flow_eval_window_key(window_id)) {
        None => true,
        Some(latch) => latch.unfinished && !latch.armed && latch.in_flight == 0,
    }
}
```

With `unfinished` false the host refresh poll (`pending_effects`) can never arm another tick. The
chain's only remaining continuation was one tessellate answer arming the next, in
`preview_eval::resolve_tessellate`:

```rust
let armed = outcome.needs_another_round_trip() && session.arm_window_tick(&payload.window_id);
```

So the first answer that came back `Ready` while another handle was still untessellated left the
window with **no continuation at all** — `inFlight: 1`, `ratio < 1`, `cancellable: true`, no fault,
no alert, forever. That is exactly the wgpu viewer signature recorded in
`📓️wgpu-example-chain-2026-09-13.md` §6.1 (`sphere-box-fuse` `unitsDone 24/41 inFlight 1
ratio 0.5853658536585366`; `sphere-cut-with-torus` `inFlight 1 ratio 0.0`), deterministic across four
runs. React hid it because its refresh cadence is poll-rich; on wgpu the poll's effects are precisely
what a blocked `apply_pending_step` never applies (`…/matrix-2/sphere-box-fuse/viewer/console.txt`,
`apply_pending_step blocked: head needs the interaction state, checked out at "frame-deferred"`).

### 4.2 Fix, at the owning layer — `any/🧵️preview-eval/🦀️.rs`

Both halves are in the surface-neutral chain, so all three preview windows get them at once.

1. **The honest record.** New named rule, so a law can state it without a contributed registry:

```rust
pub fn tick_is_unfinished(more: bool, parked_extension_invocations: usize) -> bool {
    more || parked_extension_invocations > 0
}
…
session.note_window_tick_outcome(window_id, tick_is_unfinished(more, extension_invocations.len()));
```

2. **The terminal arm is the chain's own.** `resolve_tessellate` now asks the session the same
question the refresh poll asks, so convergence stops being a property of the shell's refresh cadence:

```rust
let discharged = session.settle_window_extension(&payload.window_id);
let owes_more = outcome.needs_another_round_trip() || session.window_tick_owed(&payload.window_id);
let armed = owes_more && session.arm_window_tick(&payload.window_id);
```

It is **latched**, not unconditional: `settle_window_extension` runs first and always materialises the
latch, so a stale or foreign answer, or an answer on a window whose own tick reported itself finished,
reads `unfinished == false` and arms nothing. The terminal tick itself parks nothing and computes
nothing more, records `unfinished = false`, and the chain stops — the editor's own observed
`flowEvalTick settled effects=0`.

### 4.3 A second, independent asymmetry — `any/👁️viewer/🦀️.rs` `pending_effects`

The viewer asked its re-arm gate about the wrong graph:

```rust
if windows.is_empty() || !preview_eval::may_rearm(&doc.snapshot.fixture) { return Vec::new(); }
```

`doc.snapshot` is the document this session OPENED; the chain TICKS the **viewed** document, which is
a different document the moment the navbar names an example (the editor has no such split and asked
about the only graph it has). Now resolved through the same `Generation3dViewedDocument::resolve` the
tick, the render and the interaction topology run, and the projection is retired explicitly.

---

## 5. (b) `No example`

### 5.1 Edit and generate were already correct — measured, not assumed

`parity-1`, step `edit:no-example` (a real pick over `Sphere Cut With Torus`):

```
windows=["procedural-preview"]
window:procedural-main       statusKeys=[]   (the graph really is empty)
window:procedural-preview    meshes=0 instances=0
  {"phase":"idle","progress":{…"unitsTotal":0,"facesTotal":0,"inFlight":0,"ratio":1},
   "cancellable":false,"debug":{"meshesLen":2,"instancesLen":2}}   ← "[]" is 2 bytes
  selection {"selectedIds":[],"hoverTarget":null,"gumballActive":false,…}
```

The preview window stays mounted and settles immediately as idle-empty. No change was needed.

### 5.2 The viewer was not

`parity-1`, step `view:no-example`, picked over `Sphere Box Fuse` while the opened document was
`Box Fillet Preview`:

```
window:procedural-view-preview  meshes=3 instances=3  unitsDone 44/44 facesDone 8/8 ratio 1
  debug {"meshesLen":3641,"instancesLen":695}
```

3641/695 are the **hexagonal mushroom column's** numbers — the app's default document. `No example`
showed an example.

Cause: `Generation3dViewConfig::active_example_id` was a bare `String`, so the empty id had to mean
both "never picked" (⇒ the opened document) and "picked `No example`". The row therefore resolved to
the opened document, and in the playground that document IS a bundled example. It is the same lie the
editor's own row carried until 2026-09-12.

Fix — the picker has **three** states, so the config leaf carries three:

| config | means | viewed |
|---|---|---|
| `None` | nothing picked yet (the ABSENCE of a dispatch) | the opened document |
| `Some("")` | the picker's `No example` row | the EMPTY document |
| `Some(id)` | that bundled example | that example |

`active_example_id: Option<String>` — mirroring the editor config's own `selected_generation_id:
Option<String>` — through all six schema surfaces (`🔣️.json` / `📜️.wit` / `🔗️.graphql` / `🛰️.proto` /
`🦀️.rs` / `🟦️.ts`), the `SetActiveExample` config leaf, the viewer command (which always writes
`Some`, because a dispatched empty id is always an explicit pick), and
`Generation3dViewedDocument::resolve`, which gained an `Empty` arm over
`empty_generation3d_snapshot()` and retires it like every other owned projection.

---

## 6. (c) The journey probe's convergence predicate

`T/🐍️journey-probe.mjs`. The old predicate:

```js
const nodesOk = nodes.length > 0 && nodes.every(…);            // the FLOW window's per-node map
const graphOk = …(main?.widgetIds ?? [])…;                      // the FLOW window's published graph
return Boolean(preview) && preview.phase === "idle" && preview.ratio === 1 && nodesOk && graphOk;
```

Neither the viewer nor generate mode has a `window:procedural-main`, so `nodes.length > 0` was false
for every one of those steps and they could never converge — regardless of the app. `edit:No example`
failed for the mirror-image reason: an empty graph has zero node statuses.

New predicate, contract-first and uniform:

```js
const settled = (h) => Boolean(h) && h.phase === "idle" && h.ratio === 1 && h.computing !== true && !h.fault;
```

— every attached preview window must be `settled`; a `No example` step must additionally leave no mesh
and no instance on ANY preview window; the picker must name what was picked; and the graph oracle
(which is what caught the stale-flow-window bug originally, so it stays) is asked **only where a flow
window exists**. `idle` + `ratio 1` is also the honest idle-empty state: `PreviewTessellateStatus::
ratio` returns 1 for a zero total with nothing in flight (`flow-host.rs:3829`).

---

## 7. (d) Runtime proof

Restage (mine, foreground, `T/🗑️generated/viewer-status/restage.txt`, `exit=0`) staged
`…/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/…core.wasm` at **18:20**. That staging root is
the ONE both servers read (`🧑‍💻dev/♻️activation/🟦️.ts:80 pluginModulesRoot`), so 6018 and 6118 pick it
up on reload; `activate-generation3d-wgpu-dev` was run too (`restage-wgpu.txt`, `exit=0`) and neither
server was started or stopped.

### 7.1 React journey, 6018 — `23/23 converged`

`cd T && SEMIO_PROBE_OUT=viewer-status/journey bun 🐍️journey-probe.mjs`
→ `T/🗑️generated/viewer-status/journey/` (`results.json`, `console.txt`, 23 screenshots).
`DONE steps 23 meshSteps 20`, **not-converged: 0 of 23**.

| step | conv | s | meshes | hosts |
|---|---|---|---|---|
| `boot` | ✔ | 10 | 3 | main + preview |
| `edit:No example` | ✔ | **3** | **0** | main + preview |
| `edit:Hexagonal Mushroom Column` | ✔ | 4 | 3 | main + preview |
| `edit:Rectangle Extrude Volume` | ✔ | 8 | 1 | main + preview |
| `edit:Sphere Cut With Torus` | ✔ | 7 | 1 | main + preview |
| `edit:Box Fillet Preview` | ✔ | 5 | 1 | main + preview |
| `edit:Sphere Box Fuse` | ✔ | 6 | 1 | main + preview |
| `edit:Face Sweep Extrude` | ✔ | 7 | 1 | main + preview |
| `edit:Rectangle Wire Preview` | ✔ | 3 | 1 | main + preview |
| `edit:Box Shell Preview` | ✔ | 5 | 1 | main + preview |
| `generate-mode` | ✔ | 3 | 0 | generate-preview |
| `generate-added` | ✔ | **3** | 1 | generate-preview |
| `back-to-edit` | ✔ | 3 | 1 | main + preview |
| `viewer-role` | ✔ | **4** | 1 | view-preview |
| `view:No example` | ✔ | **3** | **0** | view-preview |
| `view:Hexagonal Mushroom Column` | ✔ | 3 | 3 | view-preview |
| `view:Rectangle Extrude Volume` | ✔ | 3 | 1 | view-preview |
| `view:Sphere Cut With Torus` | ✔ | 4 | 1 | view-preview |
| `view:Box Fillet Preview` | ✔ | 3 | 1 | view-preview |
| `view:Sphere Box Fuse` | ✔ | 4 | 1 | view-preview |
| `view:Face Sweep Extrude` | ✔ | 4 | 1 | view-preview |
| `view:Rectangle Wire Preview` | ✔ | 3 | 1 | view-preview |
| `view:Box Shell Preview` | ✔ | 3 | 1 | view-preview |

Before (`s4-journey-1`): every `view:*` row and `generate-added` at **60 s / `converged=false`**,
`view:No example` at **meshes=3**, `edit:No example` at 60 s. After: the worst step is the 10 s boot.

### 7.2 wgpu, 6118 — the viewer lane goes `6/8 → 8/8`

`SEMIO_PROBE_LANES=viewer SEMIO_PROBE_BUDGET=150 bun 🐍️wgpu-example-matrix-probe.mjs`
→ `T/🗑️generated/viewer-status/wgpu-viewer-all/`, **`{"pass": 8, "total": 8}`**, 4.25–6.36 s per cell.

The two cells that were deterministically red across four runs and two builds:

| example | before (`wgpu-examples/matrix-2`) | after (`viewer-status/wgpu-viewer-all`) |
|---|---|---|
| `sphere-box-fuse` | `fail` · 168 s · `unitsDone 24/41 inFlight 1 ratio 0.5853658536585366` · `sceneInstances 0` | **`pass` · 5.27 s · `facesDone 7/7 ratio 1` · `instances 1 draws 1`** |
| `sphere-cut-with-torus` | `fail` · `inFlight 1 ratio 0.0` | **`pass` · 5.33 s · `facesDone 3/3 ratio 1` · `instances 1 draws 1`** |

A second, independent sample of just those two (`viewer-status/wgpu-booleans/`) also passed —
8.63 s and 6.44 s — so the result is not a single lucky run. No regression on the edit lane:
`viewer-status/wgpu-edit-booleans/` is `2/2 pass` at 27.4 s / 28.19 s, matching `matrix-2`'s own
27.81 s for the same cell.

### 7.3 Native laws

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- preview_eval set_active_example status_contract \
  flow_tessellate_resolve example_switch --test-threads=1
```
→ **53 passed; 0 failed** (`T/🗑️generated/viewer-status/run-preview-eval.txt`), including the five new
laws and every pre-existing tessellate-resolve, example-switch and status-contract law unchanged.
Re-run to green after the final edit; three intermediate attempts were blocked by a peer's in-flight
refactor of unrelated crates (`semio-framework-os-infinite` `MergeMode::Range` non-exhaustive,
`INTERACTION_TARGETS_OPEN` unresolved) — transient workspace churn, neither in nor reachable from any
file this lane touched, and gone by the fourth attempt.

```
bun "T/🔍️preview-status-contract.ts"
```
→ `generation3d preview-status surfaces=editor:procedural-preview generate:generation3d-generate-preview
viewer:procedural-view-preview states=idle,computing,faulted-evaluate,faulted-unaddressable-kernel,cancelled
cancelAction=cancelPreviewEval` / `OK` — now including the settle-path model.

`cargo check -p semio-s-artifact-procedural-generation3d --lib` and the same with
`--features component-app-assembly --profile test`: clean, **2 warnings, both pre-existing and in
files this lane did not touch** (`unused extern crate` in the crate root, `retained_allocated_bytes`
never used in `🧬️mutations/💾️binary`).

---

## 7.4 Laws added, with counts

| law | where | drives |
|---|---|---|
| `a_tick_that_parked_extension_work_is_never_recorded_finished` | `any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` | `settlePath.unfinishedRows` — 4 rows |
| `the_last_tessellate_answer_arms_the_terminal_tick_itself` | same | `settlePath.latchSequence` — 4 steps, each asserting `armed`/`inFlight`/`tickOwed` |
| `an_answer_on_a_finished_window_arms_nothing` | same | the latch's negative half |
| `the_no_example_row_clears_the_viewed_document_and_never_picking_does_not` | `any/👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs` | `🎨️example-switch.json` `viewer.states` — 3 states |
| `SettleLatch` model + settle-path replay | `any/🧪️tests/🔬️status-contract/🟦️.ts` | the same two fixture tables, independently modelled in TypeScript |

Fixture rows added: `🛑️preview-cancel.json` gained 3 `laws` entries and a `settlePath` section
(4 + 4 rows + the terminal-tick descriptor); `🎨️example-switch.json` gained the `viewer` section
(3 states) and the `viewPreview` window kind.

---

## 8. Why `s4-journey-1` is not a clean baseline

That run was contaminated mid-flight. At `t=60467` its console records a full plugin hot-swap
(`hot-swap flow-extension-brep` … `hot-swap procedural`), which revoked the app actor
(`PluginRuntime: turn failed for actor procedural#1 Error: actor-activation.revoked`, `t=60753`) and
left the page with `hosts=[]`, `windows=[]` until a new instance was created at `t≈83.6 s`. That is
what produced the empty `edit:No example` row and its 60 s, and it is a peer restaging while the
journey ran — not app behaviour. The per-step seconds in §7 come from a clean run.

---

## 9. What is NOT claimed

1. **No status-projection change was made, and none was needed.** §3 verifies the existing one; the
   lane's code change to the status path is zero. If a reviewer expected the viewer's `phase`/
   `progress` to be *added*, they were already there in the served wasm.
2. **Selection clearing on `No example` is proven structurally, not by a click.** The viewed document
   drives `interaction_topology`, and the framework prunes selection against it, so an empty viewed
   document has nothing to select; the probe records an empty `data-selection-json` on every window in
   every step, but no step SELECTS something first and then picks `No example`. A dedicated
   select-then-clear probe would close that.
3. **`generate-added` is proven on React only.** On 6118 the `Add Generation` row still does not
   dispatch (`📓️wgpu-example-chain-2026-09-13.md` §6.2, owned by the `wgpu-input-hit-runtime` lane),
   so generate-mode convergence after `addGeneration` cannot be measured there at all.
4. **The wgpu host's blocked `apply_pending_step` is untouched.** This lane made the guest's settle
   path independent of the refresh poll, which is why the viewer now converges there; it did NOT fix
   whatever keeps `apply_pending_step blocked: head needs the interaction state, checked out at
   "frame-deferred"` spinning. Any other guest that still relies on `pending_effects` to continue its
   work has the same exposure.
5. **`tick_is_unfinished` is law-tested as a rule, not through a contributed registry.**
   `preview_tessellate_invocations` needs a resolvable geometry extension, which a bare `--lib` binary
   does not have, so no unit law drives `evaluate_tick` all the way to a parked tessellation. The
   end-to-end evidence for that half is §7.2.
6. **The full crate suite was not re-run by this lane.** A peer's `gates-green` lane was running
   `cargo test … --features component-app-assembly --lib` on this exact crate while this lane
   finished, and running a second copy would have starved both. The targeted `--test-threads=1` run in
   §7.3 covers every test in and around the changed code.
7. **Not measured:** locale switching of the new fixture prose, the release profile, and any surface
   other than generation3d.
8. **Fixture formatting is preserved, deliberately.** An intermediate edit re-emitted
   `🎨️example-switch.json` through a JSON pretty-printer and expanded every inline array — a 259-line
   whole-file reformat that would have collided with any peer holding that file. It was restored to
   its authored layout and the `viewer` section re-applied as a surgical insertion; the diff is now
   11 lines.

---

## 10. Files

Modified
- `✏️s/…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` — `tick_is_unfinished`, the honest `note_window_tick_outcome`, the terminal arm in `resolve_tessellate`
- `…/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` — 3 new laws, fixture-driven
- `…/✳️any/👁️viewer/🦀️.rs` — `Generation3dViewedDocument::Empty`, three-state `resolve`, `pending_effects` asks the VIEWED document
- `…/✳️any/👁️viewer/🎚️config/🦀️.rs` — `active_example_id: Option<String>`
- `…/✳️any/👁️viewer/🎚️config/🧬️schema/{🔣️.json,📜️.wit,🔗️.graphql,🛰️.proto,🦀️.rs,🟦️.ts}`
- `…/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/🎨️set-active-example/{🦀️.rs,🧬️schema/🔣️.json}`
- `…/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs`
- `…/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs` — the `No example` law
- `…/✳️any/🧫️fixtures/🛑️preview-cancel.json` — 3 laws + `settlePath`
- `…/✳️any/🧫️fixtures/🎨️example-switch.json` — the `viewer` section + `viewPreview` window kind
- `…/✳️any/🧪️tests/🔬️status-contract/🟦️.ts` — the `SettleLatch` twin
- `<ticket>/🐍️journey-probe.mjs` — contract-first convergence predicate
- `<ticket>/🔍️preview-status-contract.ts` — import repointed to the renamed `🟦️.ts`

Added
- `<ticket>/🐍️status-parity-probe.mjs`
- `<ticket>/📓️viewer-generate-status-parity-2026-09-13.md` (this file)

Generated (under `<ticket>/🗑️generated/viewer-status/`)
- `parity-1/`, `journey/`, `wgpu-booleans/`, `wgpu-viewer-all/`, `wgpu-edit-booleans/`,
  `run-preview-eval.txt`, `restage.txt`, `restage-wgpu.txt`
