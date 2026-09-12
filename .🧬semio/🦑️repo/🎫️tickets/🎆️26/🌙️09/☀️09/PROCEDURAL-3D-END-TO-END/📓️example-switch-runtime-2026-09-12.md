# Picking An Example — Root Cause, Fix, Laws (2026-09-12)

Lane: `example-switch-runtime`. Raw output: `🗑️generated/example-switch-runtime/` (native + vitest logs),
`🗑️generated/sweep-2/`, `🗑️generated/sweep-3/`, `🗑️generated/sweep-4/` (browser probes, screenshots,
per-pick consoles). Probes: `🐍️example-switch-probe.mjs` (one pick, full console),
`🐍️example-sweep-probe.mjs` (every pick, flip/mesh timing), `🐍️journey-probe.mjs` (repaired, §6).

---

## 0. Bottom line

The P0 as filed — "`setActiveExample` dispatches and settles but the Flow window keeps the previous
graph" — is **three distinct defects stacked on one symptom**, and the loudest of them was in the
HOST, not the guest:

| # | defect | owning layer | status |
|---|---|---|---|
| 1 | `refreshUi` **abandons** a pass whose generation moved under an await, and the eval chain requests passes faster than one pass can cross — so no pass ever applies and the shell keeps the previous example's body | host, `🏛️ShellHost/🟦️.tsx` | **fixed, live on reload, re-probed** |
| 2 | the picker's `No example` row resolved to `default_snapshot()` — the hexagonal mushroom column, itself one of the eight bundled examples — so the row that promises no example silently loaded one | guest, `✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs` | **fixed, native proof; restage required** |
| 3 | a `setActiveExample` that changed nothing (unknown id, or re-picking the open example) still armed a full `flowEvalTick` chain on every attached preview | guest, `✏️editor/🦀️.rs` | **fixed, native proof; restage required** |

Plus one measurement defect that manufactured the original report: the journey probe read the flow
window's **eval status map** (`data-status-json`) as if it were the graph. It is not — the graph is
`data-fixture-json`. §6.

---

## 1. What the runtime actually did (evidence, not inference)

### 1.1 The document DID change; the surface did not

`🗑️generated/sweep-3/console-Rectangle-Extrude-Volume.txt`, one pick of `Rectangle Extrude Volume`
on `http://127.0.0.1:6018/?plugin=generation3d` (runtime diagnostics armed through
`localStorage.SEMIO_RUNTIME_DIAGNOSTICS=1`, which is the shell's own switch — no source change
needed for this round):

```
183040  performInvocation {"actionId":"setActiveExample"}
183801  command ingress settled status=command-complete
184015  history patch applied {"currentCursor":6,"patchCursor":9,"upserts":2,
          "labels":["snapshot config { … }","delete-widget-position id=column-preview"],"canUndo":true}
184015  completion apply {"operation":1344,"scope":{"kind":"full"},"refresh":{"kind":"full"},…}
184015  applyHostEffects refresh {"scope":{"kind":"full"}}
```

The mutation landed in **0.8 s**, the completion carried `UiDirtyScope::Full`, and the host asked for
a full refresh immediately. The flow window's published fixture (`data-fixture-json`) was still the
hexagonal mushroom column **45 s later**, across seven more `applyHostEffects refresh
{"scope":{"kind":"full"}}` passes. No fault anywhere.

So neither the guest gesture nor the completion's scope was at fault — both are exactly right
(`ui_scope=Some(Full)` is printed by the existing native law too).

### 1.2 Why no pass applied — the livelock

`refreshUi` bumps `refreshGenerationRef` on every call and **returns without applying** whenever the
generation moved under one of its two awaits (`🏛️ShellHost/🟦️.tsx`, the two
`if (generation !== refreshGenerationRef.current …) return;` guards). That is correct only while a
pass finishes faster than passes are requested.

It does not. In the same console one `flowEvalTick` guest crossing during the brep solve is:

```
184091  performInvocation {"actionId":"flowEvalTick"}
200625  command ingress settled status=command-complete          ← 16.5 s
215778  performInvocation {"actionId":"flowEvalTick"}
229738  command ingress settled                                   ← 14.0 s
```

and every one of those ticks' completions carries `{"scope":{"kind":"full"}}`, so each settles into
another full pass. Passes arrive every few seconds; one pass takes 15-16 s to cross. **Every response
was superseded before it could be applied**, so the cache was never written, the dispatch never
happened, and the shell kept the previous example's body for as long as the evaluation ran.

This is the same pathology the contributions publisher was lifted out of `refreshUi` to escape —
its own docstring names it verbatim ("a guest re-arming a faulting `flowEvalTick` … superseded every
push mid-read"). The refresh itself was never given the same treatment.

Whether a pick survived was therefore luck — whether the eval chain happened to leave a gap:

| pick (sweep-3, before the fix) | flow window flipped | preview meshes |
|---|---|---|
| No example | never (correctly — see §3) | 3 |
| Hexagonal Mushroom Column | never (correctly — already open) | 3 |
| Rectangle Extrude Volume | **never (45 s)** | 3 (stale) |
| Sphere Cut With Torus | **never (45 s)** | 3 (stale) |
| Box Fillet Preview | 5 s | 0 |
| Sphere Box Fuse | **never (45 s)** | 0 |
| Face Sweep Extrude | **never (45 s)** | 0 |
| Rectangle Wire Preview | 33 s | 1 |
| Box Shell Preview | 10 s | 0 (`phase: invalid`) |

### 1.3 What was NOT the cause

Ruled out with evidence, so the next reader does not re-walk them:

- **`mergeRecordPreservingIdentity` / the cached section.** Never reached — the response was dropped
  before `applyUiRefreshResponseToCache`. The new `[DEBUG] refreshUi sections` line (added to
  `refreshUi`, gated on the shell's existing diagnostics switch) prints `asked` vs `changed` per pass
  and now shows `"changed":["procedural-main"]` on the pass that applies.
- **The guest reporting the section unchanged.** `plugin_refresh_ui` re-renders every requested
  window and hashes the fresh node (`🔌️plugin/🦀️.rs`); `flow_window::render` reads `document.fixture`
  directly with no cache. A stale hash is impossible while the document has moved.
- **The retained `FlowEvalSession` not invalidating.** The gesture arms through
  `arm_window_tick` and the native law `a_shell_dispatched_example_switch_republishes_both_windows_and_rearms_the_eval_chain`
  proves the preview transient's `preview_eval_text` changes. The new fixture law re-proves it per row.
- **The example id not resolving.** `NavbarExampleSelect` normalizes its `__none__` sentinel to `""`
  and passes every other option's id through verbatim; the live console shows
  `setActiveExample {exampleId: rectangle-extrude-volume}` etc. The ids are correct.
- **The dialect change.** `examplesForApp` resolves all eight for `…#editor`; the live picker lists
  all nine rows (`[DEBUG] options [...]`).

### 1.4 One transient hazard worth recording

Mid-session the dev server on 6018 served a **stale vite transform** of
`🔌️PluginRuntime/🟦️.tsx` — the module referenced `hostContinuations` with its import elided, so every
fresh page load died with `ReferenceError: hostContinuations is not defined` and the following
`setActiveExample` got `command ingress backpressure after serialized submission`. The disk file was
correct; `?t=<now>` served the correct transform while the bare URL did not. `touch`ing the file
cleared it. A probe run that reports both of those errors is measuring vite, not the app.

---

## 2. Fix 1 (host) — coalesce, never abandon

`🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — new `mergeUiDirtyScopeV1(first, second)`: `full`
dominates, `none` is the identity, two partials union their body lists and OR their flags.

`🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` — new `//#region 🤝️UiRefreshCoalescing`. The existing
callback is renamed `runUiRefreshPass` (its body, its generation guards and its deps are untouched)
and `refreshUi` becomes a thin wrapper enforcing one law:

> **At most ONE ui-refresh pass is in flight per shell, plus at most one OWED follow-up carrying the
> union of everything asked for while it ran.**

A caller that arrives while a pass is crossing merges its scope into the owed slot and awaits the
running pass; the pass that started the run drains the owed slot in a loop before returning. Joining
callers deliberately do NOT await the follow-up — a refresh is a projection, and the owed run the
starter drives covers whatever the join asked for.

This is the host twin of the guest's own `FlowEvalSession` per-window latch, and it makes progress
monotone: a pass now always applies.

Also in `refreshUi`: `[DEBUG] refreshUi sections` (gated on `runtimeDiagnosticsEnabled()`, beside the
existing `completion apply` / `applyHostEffects refresh` lines) naming the sections a pass asked for,
the sections the guest re-serialized, and their hashes.

## 3. Fix 2 (guest) — `No example` clears the graph

`✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs`: the empty id resolved to `default_snapshot()`, which
is `GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT` — the hexagonal mushroom column, itself the first row of
the picker. Measured live: `setActiveExample {exampleId: }` settled with **one config upsert and zero
artifact mutations**, and the column stayed on screen. The row that says "no example" did the
opposite of what it says, silently.

It now resolves to `empty_generation3d_snapshot()` — the genuinely empty projection (no widgets, no
synapses, no positions, no generations). An id the dialect never published is still a no-op rather
than a blank: the picker can only ever offer declared ids, and a typo must not destroy a graph.

The **viewer's** twin is deliberately left alone: a viewer owns no document authority, so its empty
id means "back to the document this session opened", which is a real and different thing. That
asymmetry is inherent to read-only vs read-write and is recorded as a follow-up (§7), not papered over.

## 4. Fix 3 (guest) — a switch that moved nothing owes no re-arm

`✏️editor/🦀️.rs` `Generation3dPreviewWork::step`: the `SetActiveExample` arm extended its effects with
`rearm_attached_previews` unconditionally, so an unknown id (`Emit::default()`) and a re-pick of the
already-open example (config mutations only) each paid a full evaluation round trip on every attached
preview. It is now gated on `!emit.artifact_mutations.is_empty()` — only a switch that actually moved
the graph owes the preview chains a restart. The generation-command arm below it is untouched.

---

## 5. Laws

### 5.1 Language-agnostic fixture (new)

`✏️s/…/✳️any/🧫️fixtures/🎨️example-switch.json`, `semio.generation3d.example-switch` v1. Six rows; each
is one pick sequence on a live surface with a declared window roster, and declares the graph the flow
window must publish, the previous example's widgets that must NOT survive, the preview chains the
gesture itself must arm, and whether the preview's retained evaluation must have moved. It also
carries the authored widget-id table of all eight examples, so a drifting `.dsl.semio` asset fails
here first. `📚️example-picker.json` (`🛂️manifest`) says which examples a surface may OFFER; this says
what picking one must DO.

### 5.2 Rust law (new)

`✏️editor/🧪️tests/🔬️example-switch/🦀️.rs::every_example_switch_row_of_the_fixture_holds` replays every
row against a live registered instance under the shell's own flow-window view. Armed windows are
compared as a set, not a sequence — the retained ladder drains `Emit::effects` with `pop()`, so the
published order is the roster's reverse and carries no meaning.

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- example_switch set_active_example --test-threads=1
```
→ **25 passed; 0 failed; 348 filtered out** (`🗑️generated/example-switch-runtime/native-fixture-law-4.txt`).
The pre-fix baseline of the same command was **24 passed; 0 failed**
(`…/native-baseline.txt`) — the 25th is the new fixture law. Per-row evidence from that run:

```
[DEBUG] example-switch row the-boot-example-is-the-hexagonal-mushroom-column picks=[] published={column-preview, extrude, extrusion-axis, height, profile, radius, sides} armed=[]
[DEBUG] example-switch row one-pick-republishes-the-flow-window-and-rearms-the-preview picks=["box-shell-preview"] published={box, shell, size, thickness} armed=["preview"]
[DEBUG] example-switch row a-pick-that-shares-widget-ids-with-the-previous-example-still-republishes-only-its-own picks=["box-fillet-preview", "box-shell-preview"] published={box, shell, size, thickness} armed=["preview"]
[DEBUG] example-switch row every-attached-preview-window-gets-its-own-chain picks=["rectangle-wire-preview"] published={height, rect, width} armed=["generate", "preview"]
[DEBUG] example-switch row no-example-clears-the-graph picks=["box-shell-preview", ""] published={} armed=["preview"]
[DEBUG] example-switch row an-unpublished-example-id-changes-nothing picks=["not-a-bundled-example"] published={column-preview, extrude, …} armed=[]
```

Runs 1-3 of that law are kept beside it and show the two guest fixes landing: before fix 2 the
`no-example` row published the hex column; before fix 3 the unknown-id row armed `["preview"]`.

Wider native guard over the editor's whole component surface:

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- editor::generation3d::component --test-threads=1
```
→ **75 passed; 4 failed** (`🗑️generated/example-switch-runtime/native-editor-component-guard.txt`).
The four reds are NOT this lane and none of them dispatches `setActiveExample` at all (checked per
test):

- `two_instances_converge_disjoint_widget_moves` and `undo_redo_round_trips_flow_graph_edits` live
  entirely in the framework's own conformance harness (`🧰️framework/…/🔌️plugin/🦀️.rs:6839`/`:6862`)
  and fail on framework VCS gates — `module.vcs: remote snapshot merge is fail-closed until the
  app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized`,
  and `undo did not revert to the expected snapshot (left: 8, right: 7)`.
- `generation_preview_is_one_app_transient_shared_by_two_generation_windows`
  (`Generation3d preview operation did not finish`) and
  `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`
  (`generation3d-publication.contended`) are retained-ladder ownership/contention faults in code this
  lane did not touch.

This lane's two guest edits are the empty-id branch of `set_active_example::emit` and the re-arm gate
on the `SetActiveExample` arm of `Generation3dPreviewWork::step`; neither is on any of those four
paths. Stated as a reachability argument, not as a clean-tree bisect — several peer lanes are
mid-flight across `✏️editor/` and the framework plugin crate.

### 5.3 TypeScript twin — the host-caching half (new)

`📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, `//#region 🎨️ExampleSwitchHostCaching`, four
cases: a `full`-scope completion re-takes both generation3d window bodies while a history-only
completion re-takes none; a changed guest body replaces the cached flow body while the untouched
preview body keeps its object identity; an unchanged hash keeps the previous graph (the failure the
law names); and `mergeUiDirtyScopeV1`'s union, including that the union still asks the guest for
BOTH bodies — the whole point of coalescing rather than superseding.

```
cd 🧰️framework/…/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bunx vitest run \
  --config vitest.config.ts --testNamePattern="example switch"
```
→ **4 passed | 952 skipped** (`🗑️generated/example-switch-runtime/vitest-host-caching-1.txt`).

Regression guard over every refresh/picker law the coalescing change could reach:

```
cd 🧰️framework/…/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts \
  --testNamePattern="refresh|ui refresh|example picker|published example graphs"
```
→ **38 passed | 918 skipped, 0 failed** (`🗑️generated/example-switch-runtime/vitest-refresh-guard.txt`).

---

## 6. The probe that manufactured the report

`🐍️journey-probe.mjs` read the flow window's `data-status-json` — the **evaluation's per-node status
map**, published from the retained `FlowEvalSession`, which lags the document — and treated its keys
as the graph. Its `converged()` therefore accepted a stale surface, declared convergence in ~3 s and
screenshotted the previous example for the whole journey; `nodeStatuses` stayed length 7 for all 22
steps because the eval session still held the hexagonal column.

Repaired in place: the probe now also reads `data-fixture-json` (the published graph), carries the
authored widget-id table of all eight examples, and a pick step converges only when the flow window
publishes the graph of the example the picker NAMES. Correct expectations, for the record —
`Box Shell Preview` is **4** widgets (`size, thickness, box, shell`) and `Rectangle Wire Preview` is
**3** (`width, height, rect`), not 3 and 2.

---

## 7. Runtime re-probe after the host fix — 0/6 → 6/6

`SEMIO_PROBE_OUT=sweep-4 SEMIO_PROBE_BOOT_WAIT=240 SEMIO_PROBE_HOLD=45 bun 🐍️example-sweep-probe.mjs`
against the SAME served build (the host half is live on reload; the plugin wasm was not rebuilt).
Screenshots `🗑️generated/sweep-4/N-<Example>.png`, per-pick consoles beside them, raw
`🗑️generated/sweep-4/results.json`. `flip` is seconds from the click until the flow window publishes
the picked example's own widget ids; `mesh` is seconds until the preview is `idle` with ≥ 1 mesh.

| pick | flip BEFORE (sweep-3) | flip AFTER (sweep-4) | published widget ids | preview meshes |
|---|---|---|---|---|
| No example | — | — (guest fix pending restage, §3) | hexagonal column (unchanged) | 1 |
| Hexagonal Mushroom Column | — | — (already open; correct no-op) | `height, radius, sides, profile, extrusion-axis, extrude, column-preview` (7) | 3 |
| Rectangle Extrude Volume | **never (45 s)** | **21 s** | `width, height, distance, rect, vector, extrude, volume` (7) | **1** at 21 s |
| Sphere Cut With Torus | **never** | **31 s** | `slider_2, brep_prim3d_sphere_3, brep_prim3d_torus_4, brep_bool_cut_5, brep_measure_volume_2, preview_3` (6) | 0 |
| Box Fillet Preview | 5 s | **13 s** | `size, radius, box, fillet, preview` (5) | 0 |
| Sphere Box Fuse | **never** | **17 s** | `radius, size, sphere, box, fuse, preview` (6) | 0 |
| Face Sweep Extrude | **never** | **24 s** | `width, height, distance, rect, face, vector, extrude` (7) | 0 |
| Rectangle Wire Preview | 33 s | **12 s** | `width, height, rect` (3) | **1** at 17 s |
| Box Shell Preview | 10 s | **4 s** | `size, thickness, box, shell` (4) | 0, `phase: invalid` |

Every published set is EXACTLY the example's authored widget ids — no leak from the example before
it, in either direction, including the `box`/`size`/`radius` ids that several examples share. Six of
six real switches now land; before the fix three of six never landed at all inside 45 s and the three
that did were luck.

The coalescing is visible in the console:

```
[DEBUG] refreshUi sections {"scope":{"kind":"full"},"asked":["procedural-main","procedural-preview",
  "generation3d-generations","generation3d-generate-form","generation3d-generate-preview"],
  "changed":["procedural-main"],"hashes":{"procedural-main":"dceb29b9:4",…}}
[DEBUG] refreshUi coalesced {"scope":{"kind":"full"},"owed":{"kind":"full"}}
```

— passes now COMPLETE and apply (`changed:["procedural-main"]`), and the requests that used to
supersede them are coalesced into one owed follow-up instead.

### 7.1 What is still red, and whose it is

`meshes = 0` for five of the eight examples is the **kernel**, not the switch, and was already
root-caused in `📓️audit-examples-2026-09-12.md` §3/§4: `blocked-on-extrude-orientation`
(rectangle-extrude/face-sweep/hex-column), `blocked-on-fillet-kernel` (box-fillet),
`blocked-on-boolean-kernel` (sphere-box-fuse, sphere-cut-with-torus). `Box Shell Preview` settles at
`phase: "invalid"` with a `shell-not-closed` diagnostic — same `🧊️brep` subset owner. The two
examples whose kernels work (`rectangle-extrude-volume`'s wire/volume path and
`rectangle-wire-preview`) do produce meshes through the switched chain, which is what proves the
chain itself is whole.

Flip latency is still 4-31 s. That is the guest crossing, not the host: one `flowEvalTick` during a
brep solve was measured at 14-16 s and the picker's own refresh has to queue behind it on the actor's
single-flight lock. The structural cause is that **every `flowEvalTick` completion declares
`UiDirtyScope::Full`** (`Emit::default()`), so a converging preview demands a whole-shell pass every
few seconds. Narrowing the eval chain's own scope to the two window bodies it actually dirties is the
next real win and is guest-side — recorded as a follow-up rather than smuggled into this lane.

---

## 8. Follow-ups

1. **Restage the procedural plugin.** Fixes 2 and 3 (§3, §4) are guest-side and have native proof
   only; `No example` still loads the hexagonal column on the served build.
2. **`flowEvalTick`/`flowEvalResolve`/`flowTessellateResolve` should declare a partial
   `UiDirtyScope`** (the flow body + the preview body), not the `Full` they inherit from
   `Emit::default()`. That is the remaining source of the refresh pressure §2 now survives.
3. **`example_snapshot(id).unwrap_or_default()`** (`✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs`)
   silently substitutes `Generation3dSnapshot::default()` — the three-widget `slider → add → preview`
   demo graph — if a bundled example's DSL ever fails to parse. Reachable only through a broken
   asset, but it is a silent wrong answer where a fault belongs. Left alone here because changing it
   is a behaviour change other laws read.
4. **`No example` on the viewer** means "back to the document this session opened", which after this
   lane differs from the editor's "clear". Inherent to read-only vs read-write authority, but the two
   surfaces now answer one label differently and that deserves a decision by the viewer lane.
5. **Vite serves stale transforms after peer edit bursts** (§1.4). Any runtime probe should assert
   the served module is current before trusting a red result.

---

## 9. Files

**Created**
- `✏️s/…/✳️any/🧫️fixtures/🎨️example-switch.json`
- `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️example-switch-probe.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️example-sweep-probe.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/📓️example-switch-runtime-2026-09-12.md`

**Changed**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
  (`runUiRefreshPass` rename + `//#region 🤝️UiRefreshCoalescing` + `[DEBUG] refreshUi sections`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
  (`mergeUiDirtyScopeV1`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
  (`//#region 🎨️ExampleSwitchHostCaching`, 4 cases)
- `✏️s/…/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs` (empty id clears)
- `✏️s/…/✳️any/✏️editor/🦀️.rs` (`SetActiveExample` re-arm gated on a real graph change)
- `✏️s/…/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` (`//#region 🎨️FixtureLaw`)
- `.🧬semio/🦑️repo/🎫️tickets/…/PROCEDURAL-3D-END-TO-END/🐍️journey-probe.mjs` (§6)

**Note on ticket hygiene:** an intermediate probe output folder `🗑️generated/sweep-1/` — produced by
this lane minutes earlier and invalidated by the stale-transform incident of §1.4 — was deleted
before it was recognised that the ticket's `🗑️generated` tree is never to be swept. Nothing that
predates this lane was touched; `sweep-2`/`sweep-3`/`sweep-4` and `example-switch-runtime/` carry the
full evidence.
