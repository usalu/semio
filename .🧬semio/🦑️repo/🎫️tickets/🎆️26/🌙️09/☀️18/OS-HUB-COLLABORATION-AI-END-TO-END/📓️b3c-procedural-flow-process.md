# B3c — 🌀️procedural, 🌊️flow, 🏭️process: the full interaction bar

Slice B3c, session 5b (2026-09-20). Owner: worker B3c (Opus). Ticket
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END`.

Scope: outcome 1 depth for four artifacts —
🌀️procedural ×2 (`generation2d`, `generation3d`), 🌊️flow ×1 (`flow`, + its 9 extension plugins),
🏭️process ×1 (`process3d`, + its 4 material extensions).

The bar, per artifact, measured headless and never inferred:
**boot → example load → one real mutation observed in the document/ledger → undo → redo → zero console fault lines.**
For 🌊️flow the bar additionally requires evaluating one node graph that uses at least one node from
**three different extensions**.

## 1. Bar matrix (measured only)

| artifact | port | boot | example | mutation | observed | undo | redo | 0 faults | bar |
|---|---|---|---|---|---|---|---|---|---|
| 🌀️generation2d | 6021 | ✅ | ✅ | ✅ `create-widget` | ✅ artifact panel `slider_2` | ✅ | ✅ | ✅ 0 | **PASS** |
| 🌀️generation3d | 6018 | ✅ | ✅ | ✅ `move-widget` | ✅ graph pane text | ✅ | ✅ | ✅ 0 | **PASS** |
| 🌊️flow | 6016 / **6216** | ✅ | ✅ | ❌ `addWidget` refused by its retained factory | ❌ | ❌ | ❌ | **1** (was 2017, was 17) | **FAIL** — 2 of 3 root faults fixed (§5.1, §5.3 → **§FL1.1**); the third is §FL1.1.6 |
| 🏭️process3d | 6022 | ✅ | ✅ | ✅ `create-step` | ✅ ledger + edit count | ✅ | ✅ | ✅ 0 | **PASS** |

## 2. Inherited state

An earlier B3c worker (session 4, 2026-09-19 00:21–01:11) died without a report. What it left:

| artefact | state |
|---|---|
| `📜️b3c-activate.sh` / `📜️b3c-serve.sh` | usable, identical in shape to B3a's; reused unchanged |
| `🐍️b3c-interaction-probe.mjs` (252 lines, 00:42) | a PRIVATE fork of the pre-B3a2 probe — **not used**: it predates the four probe defects B3a2 fixed today, so any verdict from it would be unsound |
| `🐍️b3c-flow-probe.mjs`, `🐍️b3c-generation2d-probe.mjs`, `🐍️b3c-generation3d-probe.mjs`, `🐍️b3c-process3d-probe.mjs`, `🐍️b3c-extension-probe.mjs` | thin wrappers over that private fork, carrying guessed verb names (`addWidget`/`addGeneration`/`addStep`) |
| `🗑️generated/b3c-*` | **nothing** — no capture survived, so no measurement was inherited |

Nothing in the plugin tree is attributable to that worker: the modified files under
`✏️s/🔌️plugins/{🌀️procedural,🌊️flow,🏭️process}` are peers' in-flight work (V3b's mount/marker sweep,
the schema/fixture regeneration) and are left alone.

Staged guest components at slice start (S2's 07:00 staging, `🔌️plugin-modules/`):

| plugin | staged core.wasm | size | mtime |
|---|---|---|---|
| 🌀️procedural | `semio_s_plugin_procedural_component.core.wasm` | 70 451 570 B | 2026-09-19 18:01 |
| 🌊️flow | `semio_s_plugin_flow_component.core.wasm` | 104 171 469 B | 2026-09-20 07:43 |
| 🏭️process | `semio_s_plugin_process_component.core.wasm` | 107 361 719 B | 2026-09-19 18:01 |

All 9 flow extensions and all 4 process extensions are present as their own
`🔌️plugin-modules/` entries (`🏘️flow-extension-bim`, `🧊️flow-extension-brep`, `📚️flow-extension-dictionary`,
`🎨️flow-extension-draw`, `📃️flow-extension-list`, `🔀️flow-extension-logic`, `🧮️flow-extension-math`,
`🔤️flow-extension-primitive`, `📝️flow-extension-text`; `🏙️process-extension-concrete`,
`🔩️process-extension-metal`, `🤖️process-extension-robotic`, `🪓️process-extension-wood`).

## 3. Method

Identical to B3a2's (📓️b3a-block-gis.md §15–§18) and B3b's, so the verdicts are comparable:

* **activate once per plugin**, never `dev <variant>` (watch mode re-activates on every peer edit and
  takes the vite server down mid-probe) — `📜️b3c-activate.sh <variant> <port>`, detached, capture in
  `🗑️generated/b3c-<variant>-activate.txt`.
* **serve detached** with `📜️b3c-serve.sh <variant> <port>` on the port the `.vscode/launch.json` row
  names (`S_OS_PORT`): flow 6016, generation3d 6018, generation2d 6021, process3d 6022.
* **warm with `curl`** before the first `page.goto` (B3a2 §17.1: the first navigation against a fresh
  vite serve exceeds playwright's 30 s default under load; it is not a plugin fault).
* **probe with the SHARED `🐍️b3a-interaction-probe.mjs`**, unchanged, driven through the thin wrapper
  `🐍️b3c-bar-probe.mjs` (config from the environment so a verb can be re-picked without an edit).
  `SEMIO_PROBE_SETTLE_MS=60000`, chromium headless 1600×1000 `--use-angle=metal`.
* The bar is scored by the probe's own `summary.interactionBar`:
  `exampleRendered && mutated && undone && redone && faultLines === 0 && !error`.

### 3.1 What `activate-generation2d-react-dev` actually builds (worth knowing for the next slice)

The procedural `generation2d` playground session is **not** a single-plugin session: its
`@semio-tech/plugin-registry:session-generation2d` target pulls in `flow-plugin` and every
`flow-extension-*-rust` component as well, so one `activate-generation2d-react-dev` rebuilds and
re-materializes 🌊️flow + its 9 extensions before it touches 🌀️procedural. Measured from
`🗑️generated/b3c-generation2d-activate.txt` (start 06:05Z).

## 4. 🌀️procedural

### 4.1 `generation2d` CLEARS THE BAR

`http://127.0.0.1:6021/?plugin=generation2d`, serve pid 89798 detached
(`🗑️generated/b3c-generation2d-serve.txt`, `[fresh] 10 staged components match their sources and the
activation receipt`, `VITE ready in 31 706 ms`). Probe: the shared `🐍️b3a-interaction-probe.mjs`
through `🐍️b3c-bar-probe.mjs`, `B3C_ACTION=addWidget B3C_ARGS='{"kind":"inputSlider"}'`.
Capture `🗑️generated/b3a-generation2d/report.json` (copied to `🗑️generated/b3c-generation2d-report.json`)
+ `🗑️generated/b3a-generation2d-console.txt`.

```
SUMMARY {"ready":"generation2d","error":null,"exampleRendered":true,"actionCount":21,
         "mutated":true,"undone":true,"redone":true,"panelRoundTrip":true,
         "faultLines":0,"interactionBar":true}
```

| rung | witness |
|---|---|
| boots | `data-semio-os-ready="generation2d"`, windows `generation2d-main` + `generation2d-preview`, tabs `Flow`/`Preview`, 3 canvases, **zero window faults** |
| example loads | inspection panel `Schema: flow.host_snapshot` / `Widgets: 3` / `Show mode: preview`; artifact panel rows `slider`, `rect`, `preview`; `uiNodes ["generation2d.play.main\|3.0", "generation2d.play.preview\|"]` |
| rail | 21 action rows incl. `action.addWidget` (staged `kind` select, default `inputSlider`) |
| mutation | ledger row `create-widget index=3 input-slider id=slider_2 label="" valu…`, edits 0 → 1 |
| observed | the app's **artifact panel** gains a fourth row `slider_2` (`slider`, `rect`, `preview`, **`slider_2`**) — the app's own projection, not the ledger |
| undo | edits 1 → 0, panel back to three rows |
| redo | edits 0 → 1, panel back to four rows → `panelRoundTrip: true` |
| console | **0 fault lines** |

Verb choice note: `addGeneration` is declared on the builder
(`…/🌀️generation2d/…/✏️editor/🦀️.rs:1836`) but is **not** on the edit-mode rail — it belongs to the
`🧬️generate` mode — so the first run scored `clicked: "absent"` on it. That is a mode fact, not a
defect; `addWidget` is the edit-mode mutation and it is the one measured above.

### 4.2 `generation3d` CLEARS THE BAR

`http://127.0.0.1:6018/?plugin=generation3d`, serve pid 16430 detached
(`🗑️generated/b3c-generation3d-serve.txt`, `VITE ready in 5 310 ms`). No fresh activation was needed:
the `generation3d` playground session and the 🌀️procedural staged component were already on disk and
the serve's staleness report named only 🌊️flow (this slice's own edit) and unactivated peers, never
🌀️procedural itself.

```
SUMMARY {"ready":"generation3d","error":null,"exampleRendered":true,"actionCount":32,
         "mutated":true,"undone":true,"redone":true,"panelRoundTrip":false,
         "faultLines":0,"interactionBar":true}
```

| rung | witness |
|---|---|
| boots | `data-semio-os-ready="generation3d"`, windows `procedural-main` + `procedural-preview`, 3 canvases, zero window faults |
| example loads | example combobox `Hexagonal Mushroom Column`; graph pane 11 chars; artifact panel rows `Column Height Evaluated`, `Num Output`, `Profile Radius Evaluated`, `Side Count Evaluated`, … |
| rail | 32 action rows |
| mutation | `action.reorganize` → ledger `move-widget id=column-preview layout { x=180 y=-40 }`, edits 0 → 1, `renderChanged: true` |
| undo / redo | 1 → 0 (`action.undo`), 0 → 1 (`action.redo`) |
| console | **0 fault lines** |

`panelRoundTrip` is `false` and honest: the moved widget's coordinates are not printed in the artifact
panel's text, so there is no textual projection for the round-trip test to move — the same canvas case
B3a2 recorded for block3d/gis. The mutation itself is witnessed in the ledger AND in `renderChanged`.

### 4.3 A FRAMEWORK defect this found: a long Actions rail clips its staged-argument form, with no scroll

The generation3d run above first failed on `addWidget` with
`filled: ["kind:TimeoutError: click: Timeout 8000ms exceeded."]` — the `kind` select never became
clickable, so the verb submitted with no staged argument and did not mutate. That looked like a
generation3d bug. It is not; it is framework chrome, and it is a real user-facing defect.

Measured with `🐍️b3c-staged-form-diagnose.mjs` (capture `🗑️generated/b3c-generation3d-staged-form.txt`
+ `.png`) — the control is mounted, visible, `pointer-events: auto`, and simply **off-screen**:

```
{"id":"kind","tag":"BUTTON","role":"combobox","box":{"x":146,"y":1015,"w":160,"h":22},
 "mounted":true,"pointerEvents":"auto","visibility":"visible","opacity":"1","atCentre":null}
{"id":"framework.window.proceduralMain.action.addWidget.execute","box":{"x":183,"y":993,"w":66,"h":22},
 "atCentre":null}
```

`y = 1015` in a **1000 px** viewport, and `atCentre: null` because `elementFromPoint` is being asked
about a point outside the viewport. The ancestor chain says why nothing can scroll it into view:

```
viewport 1000
… select-trigger  overflow-y: visible  top=1015
… (7 more)        overflow-y: visible  top=1014
DIV app-body      overflow-y: HIDDEN   h=942  scrollHeight=942  clientHeight=942
DIV layout        overflow-y: HIDDEN   h=1000 scrollHeight=1000 clientHeight=1000
```

`app-body` clips at 942 px and its `scrollHeight` equals its `clientHeight`, so the overflowing rail
content is **clipped, not scrollable** — Playwright's `scrollIntoViewIfNeeded` has nowhere to scroll
and times out, and so does a human's mouse wheel.

Proof that height is the whole story, not generation3d: the identical sequence at a **1400 px**
viewport reaches the same element at the same `top = 1015`, and it works —

```
{"viewportHeight":1400,"comboboxTop":1015,"clicked":"ok","options":["Neuron","Slider","Note","Preview"]}
```

So: **any app whose Actions rail is long enough pushes its expanded staged-argument form past the
window body, where it is unreachable at that window height.** generation3d has 32 action rows and hits
it; generation2d has 21 and does not, which is the entire difference between the two siblings. Not
fixed here on purpose: the rail is shared chrome that every other slice is measuring against right
now with `elementFromPoint`-based probes, and a layout change to it cannot be re-verified across the
fleet from this slice. Handed to the framework renderer owner with the two reproductions above.

Consequence for every slice using the shared probe: a `TimeoutError` on a staged arg at 1600×1000 is
**not** evidence that the app's verb is broken — re-run at a taller viewport before scoring it.

## 5. 🌊️flow

### 5.1 Root fault found live: flow's editor had NO `command_from_action`, so its evaluation chain was dead

First measured run on `http://127.0.0.1:6016/?plugin=flow` (serve pid 64179,
`[fresh] 10 staged components match their sources and the activation receipt`, `VITE ready in 7 390 ms`):

```
SUMMARY {"ready":"flow","error":null,"exampleRendered":true,"actionCount":27,
         "mutated":false,"undone":false,"redone":false,"panelRoundTrip":false,
         "faultLines":17,"interactionBar":false}
```

**All 17 fault lines are one cause.** Sixteen are this line, repeating from t+19 s to the end of the
run (`🗑️generated/b3a-flow-console.txt`):

```
error scheduled dispatch of "flowEvalTick" failed SemioFaultError:
  action 'flowEvalTick' is not a framework-reserved action
  (history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively
  through the typed command channel now (see `dispatch_typed_command`)
```

The seventeenth is the measured mutation itself, at t+31 s, and it names the same refusal:

```
warning input #9 addWidget refused: dispatch-failed (user window=flow-compiled-dag) —
  action 'addWidget' is not a framework-reserved action … dispatched exclusively through the typed
  command channel
```

So `mutated: false` in that run is **not** a second, independent defect: flow's published
`app.commands` carries exactly two ids (`flowEvalTick`, `flowEvalResolve` — read out of the staged
descriptor `🔌️plugin-modules/🌊️flow/🔣️.json`), so every OTHER id the rail dispatches — `addWidget`
included — misses the `isAppCommand` test in `makeEffectDispatchOne`, falls to `handleAction`, and is
refused by the same default `command_from_action`. One missing method took out both the evaluation
loop and every rail mutation.

The chain that produces it, read end to end:

1. flow arms a `flowEvalTick` whenever the main snapshot has uncomputed nodes
   (`…/🌊️flow/…/✏️editor/🦀️.rs:2413`) by emitting `Effect::dispatchAction`.
2. The host honours that effect through `scheduleDispatchAction`
   (`🏛️ShellHost/🟦️.tsx:5902` → `🛠️ShellHelpers/🟦️.tsx:961`), which calls the
   `makeEffectDispatchOne` closure.
3. That closure (`🛠️ShellHelpers/🟦️.tsx:886`) reaches `handleCommand` only for ids it can see in
   `baseSession.app.commands`; everything else goes to `handleAction`.
4. `handleAction` in the guest bridges through `ArtifactApp::command_from_action`. The framework's
   **default** implementation of that method refuses every id
   (`🧰️framework/…/🔌️plugin/🦀️.rs:11960`, `:31981`, `:32374`).
5. **The flow editor implemented no `command_from_action` at all** — `grep -rn 'fn command_from_action'
   ✏️s/🔌️plugins/🌊️flow/` returned **0 hits**, while 72 files elsewhere in `✏️s/🔌️plugins/` implement it,
   including `generation2d`, `generation3d` and `process3d`. So every hop of flow's evaluation chain was
   refused at the first step and the node graph never evaluated.

This is why the S1 matrix could only ever call 🌊️flow `staged` and G11 §C row 211 reads `staged`: the
app boots and paints, but its one live loop was cut at the host↔guest seam. It is invisible to
`cargo check` (the trait method has a default) and to every `--lib` test (nothing calls the action
channel) — a live-only fault of exactly the class the memory topic
*Editor Runtime Dispatch Chain, Live-Only Faults* names.

**Fix landed** (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2361+`):
`FlowPlayApp::command_from_action` mapping all 34 declared ids onto `FlowCommand`, mirroring
`Generation2dPlayApp::command_from_action` (same `str_arg`/`f64_arg`/`u64_arg` helpers, same
camel/snake key tolerance, same terminal refusal arm for an unknown id). Two shapes differ from the
sibling and are handled here: `nodeGraphEdit`/`spotlightCommit` carry a typed
`Vec<FlowNodeGraphEditOp>` (decoded per row through the ops' own `FromValue` derive by a generic
`graph_operations::<T>` helper, because the two modules declare **distinct** enums of that name), and
flow's `FlowEvalTick` is field-less while generation2d's carries `window_id`/`window_kind_id`.

`cargo check -p semio-s-artifact-flow-flow`: **Finished, 0 errors** (12 warnings, all pre-existing
unused imports).

### 5.2 The 9 `E0509`s V3b handed this slice — fixed

`cargo check -p semio-s-artifact-flow-flow --tests` was red with 9 × `E0509: cannot move out of type
FlowWorkingScene, which implements the Drop trait` — a peer gave `FlowWorkingScene` a drop witness and
three test bodies still moved fields out of it. Fixed at the three sites:

| file | was | now |
|---|---|---|
| `…/🧵️retained/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:43,68` | `FlowWorkingScene { widgets: …, ..Default::default() }` | explicit `synapses: Vec::new(), layout: Default::default()` — functional-update syntax moves the remaining fields out of a `Drop` value |
| `…/🧵️retained/🗿️artifact/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:100` | same | same |
| `…/🧵️retained/🗿️artifact/📬️preparation/🧪️tests/🔬️unit/🦀️.rs:13` | `flow_content_child_handle_and_cache(scene.widgets, scene.synapses, scene.layout)` | `.clone()` on each — the owner must stay whole until its own drop |

`cargo check -p semio-s-artifact-flow-flow --tests`: **Finished, 0 errors** (28 warnings).
Two `[DEBUG]` `eprintln!`s left behind in those test bodies were removed at the same time (preamble rule 10).

### 5.3 The fix is PROVEN live — and it uncovers the next fault, one layer down

`📜️b3c-activate.sh flow 6016` finally got the fleet mutex at 16:56 and ran clean:
`Activated flow react dev: 10 completed components (changed)`, `Successfully ran target
activate-flow-react-dev … and 33 tasks it depends on`, `exit=0` (`🗑️generated/b3c-flow-activate.txt`).
Serve restarted on the new guest (pid 30008, `[fresh] 10 staged components match their sources and
the activation receipt`, `VITE ready in 6 165 ms`), curl-warmed, re-probed with the same command.

**The §5.1 fault class is gone, measured:** `grep -c 'not a framework-reserved action'` over the new
console capture returns **0**, where the pre-fix run had 17. Boot, example, app panels, history and
the Actions rail all complete with **0 faults** through t+25 s, and the rail carries 27 rows with
`action.addWidget` present. `command_from_action` is doing its job.

**What it uncovered:** the very first `addWidget` dispatch — the one that used to be refused outright —
now reaches the guest, and the evaluation chain it wakes never converges:

```
SUMMARY {"ready":null,"error":"flow","exampleRendered":true,"actionCount":0,
         "mutated":false,"undone":false,"redone":false,"panelRoundTrip":false,
         "faultLines":2017,"interactionBar":false}
```

| step | t | faults |
|---|---|---|
| boot / example / open-app-panels / open-history / open-actions | 15.2 s → 25.1 s | **0** |
| `invoke-action` (`addWidget`) | 50.5 s | **2017**, all from t+47.8 s |
| undo / redo | 85.8 s / 121.2 s | 0 (the shell is already down) |

2015 of the 2017 are one line, repeating thousands of times in ~3 s:

```
error scheduled dispatch of "flowEvalTick" failed
  SemioFaultError: transient read registry is busy or exhausted
```

So the re-armed `flowEvalTick` chain spins without making progress, exhausts the transient read
registry, and takes the shell down (`data-semio-os-error="flow"`, `ready: null`, rail gone →
`actionCount: 0`). `addWidget` itself leaves the ledger and the edit count at 0.

This is a **second, independent defect**, not a regression of the fix: before the fix the chain died
at hop 1 with a refusal and could never spin; the fix restored the loop and the loop is broken. The
shape to look at first is the payload asymmetry this slice already documented — flow's
`FlowEvalTick` is **field-less** (`…/🎮️commands/⏱️flow-eval-tick/🦀️.rs:37`) while generation2d's carries
`window_id` + `window_kind_id`, and generation2d's eval chain is the one that converges. A tick that
cannot tell which window/target it is evaluating has nothing to advance, so `eval_tick_effect()`
re-arms on an unchanged state forever. Flow's `FlowEvalResolve` is likewise missing the
`window_id`/`window_kind_id`/`extension_id`/`ok`/`fault_*` fields its generation2d twin carries.

Not attempted here: giving those two payloads their window/target fields is a wire-format change to
`app_commands!` rows (`#[dsl(key = "flow-eval-tick")]`), which needs the schema/fixture regeneration
the plugin owns and at least one more mutex-serialised wasm build. Recorded with the exact
reproduction instead:

```
zsh "…/📜️b3c-serve.sh" flow 6016 &                       # the activated guest is already staged
SEMIO_PROBE_SETTLE_MS=60000 B3C_VARIANT=flow B3C_PORT=6016 B3C_ACTION=addWidget \
  B3C_ARGS='{"kind":"inputSlider"}' bun 🐍️b3c-bar-probe.mjs
```

🌊️flow therefore does **not** clear the bar, and its extra clause (a graph evaluating nodes from three
extensions) cannot be measured until the tick converges. Its level moves from G11 §C's `staged` to
**boots + example loads + rail, mutation still red** — with the first of its two blockers removed at
the root and the second named.

---

## FL1

Worker FL1, session 5c (2026-09-20). Slice: 🌊️flow eval-tick convergence (§5.3) + the framework
Actions-rail clipping defect (§4.3) + the 4 🏭️process `describe` targets (§6.2).

### FL1.0 Status

| item | landed | proven by |
|---|---|---|
| 1 flow eval tick/resolve carry their window; re-arm goes through the session latch | ✅ source + wire + 4 native laws | `cargo check --tests` 0 errors; `4 passed` new laws; OpBinary wire law `1 passed` |
| 1b ONE flow activation + live re-measure | ✅ **done** — the spin is gone: **2017 fault lines → 1**, shell stays up, rail back | `🗑️generated/fl1-flow-activate.txt` (`exit=0`), `fl1-flow-bar-probe.txt`, `fl1-flow-console.txt` |
| 2 Actions rail reaches its staged-argument form | ✅ structural fix + vitest law; live DOM improved and measured | `8 passed` incl. the 32-row law; `🗑️generated/fl1-generation3d-rail-{before,panels,after2,after3}.txt` |
| 2b shared-probe staged-arg fill | ✅ on 🌊️flow (`kind=inputSlider`) · ❌ on generation3d, for a **different, named** cause (§FL1.3.4) | `🗑️generated/fl1-flow-bar-probe.txt`, `fl1-generation3d-staged-arg-probe{,-after}.txt` |
| 3 the 4 🏭️process `describe` targets | ❌ not started — same mutex queue | — |

## FL1.1 — 🌊️flow's eval chain: the tick is ADDRESSED now, and it cannot spin

### FL1.1.1 What the spin actually was

§5.3 named the payload asymmetry as the shape to look at. It is the cause, and the mechanism is
exact. Three facts compose it:

1. `FlowEvalTick`/`FlowEvalResolve` were **field-less** (tick) / window-less (resolve), so no hop could
   say which window it advanced.
2. `FlowEvalSession` already owns a complete per-window tick latch in the SHARED crate
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:3496-3640`: `arm_window_tick`,
   `window_tick_owed`, `arm_owed_window_tick`, `begin_window_tick`, `note_window_tick_outcome`,
   `note_window_extensions_in_flight`, `settle_window_extension`, `abandon_window_tick`,
   `retain_window_tick_latches`) — the machinery generation2d's converging chain runs on. **Flow called
   none of it.** Its only gate was `FlowEvalSession::sync`'s process-wide `tick_scheduled` flag.
3. `FlowPlayApp::pending_effects` built a **throwaway `FlowEvalSession::new()` on every call**
   (`…/✏️editor/🦀️.rs:2507`, pre-fix). A fresh session has `tick_scheduled == false` and an empty latch
   map **by construction**, so the gate was open on every single host refresh poll.

So the loop was: refresh → `pending_effects` arms a hop on a fresh session → the hop's retained tool
operation takes a transient read lease → the lease registry refuses
(`🧰️framework/…/🏪️store/🦀️.rs:4873` `transient read registry is busy or exhausted`) → the failed
dispatch is activity → refresh. 2015 identical lines in ~3 s, and the shell with it. The re-arm never
needed to make progress to keep arming, because nothing it could have advanced was being consulted.

### FL1.1.2 Landed

| file | change |
|---|---|
| `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | `FlowEvalTick { window_id, window_kind_id }`; `window_args`/`eval_tick_effect(window_id, window_kind_id)`; ONE hop request id (`107`) replacing three (105/106/107) minted by three files for one chain; `may_rearm` (the twin of generation2d's, over `unserved_flow_operator_kinds`); `tick_result` now `begin_window_tick` → `note_window_tick_outcome` → one of THREE mutually exclusive continuations |
| `…/🎮️commands/🏁️flow-eval-resolve/🦀️.rs` | `FlowEvalResolve { window_id, node_hash, output_json }`; folds through `resolve_preview_eval` + `seed_node_cache`, `abandon_window_tick` on a fold it cannot seed, `settle_window_extension` + `arm_owed_window_tick` for the re-arm |
| `…/🎮️commands/🧮️evaluate/🦀️.rs` | `evaluate_result(…, window_id, window_kind_id)`: `sync` answers "are there uncomputed nodes", the **latch** answers "does anything already owe a hop" — both required |
| `…/✏️editor/🦀️.rs:2506+` | `pending_effects` reads the **retained** session through `owner.with_mut::<FlowInstanceOperationOwner,_>` + `with_session`, arms per ATTACHED `flow-main` window instance from `view.window_instances`, and drops detached windows' latches |
| `…/✏️editor/🦀️.rs` `flow_scalar_command_view` | the two payloads' binary field views (see FL1.1.3) |
| `…/✏️editor/🦀️.rs` `command_from_action` | both hops decode `windowId`/`windowKindId` (camel + snake), defaulting to `flow-main` |
| `…/🧫️fixtures/📡️host-wire/🔣️.json` | regenerated rows for both, `wireBytes` 4 → **30** and 22 → **35**, `symbols` 0/1 → 2/2 |

The three continuations `tick_result` now picks between, one and only one per hop:

* parked extension answers → `note_window_extensions_in_flight`, **no effect** (the answers own it, and
  `settle_window_extension` hands the arm to the one that lands last);
* unfinished work on a **servable** graph → `arm_window_tick` → exactly one `eval_tick_effect`;
* unfinished work no installed extension can serve → `abandon_window_tick`, **no effect** — the next hop
  would park the identical request and fault at the host's own cadence. With EX1's 25 ABI-stale
  extension crates unrebuilt this is the live case, and it is the arm that used to run forever.

### FL1.1.3 The wire is binary, and it has exactly three slots

The retained route's witness is `store::os_pack::ScalarRecordView`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/🦀️.rs:16`): `[Option<ScalarRecordField>; 3]`,
of which at most two may be `Text` (`texts: [Option<usize>; 2]`). That is the whole budget, so:

* `FlowEvalTick` → `[Text(window_id), Text(window_kind_id), None]`
* `FlowEvalResolve` → `[Text(window_id), U64(node_hash), Text(output_json)]`

and flow's answer deliberately does **not** carry generation2d's `extension_id`/`ok`/`fault_code`/
`fault_message` — they do not fit, and generation2d affords them because its hops ride a tool-run job,
not this scalar route. A faulted answer stays bounded here anyway: it folds as an envelope the session
cannot seed, which `abandon_window_tick`s the window instead of re-parking the request. Recorded as an
honest gap: flow will not SURFACE an extension's evaluate fault the way generation2d does.

`wireBytes` were derived from the encoding, not guessed (base 4 = format+ordinal+symbolCount+fieldCount;
a symbol costs `1 + len`; a symbol-referencing field costs 3; a u64 field costs `2 + varint`), and the
real `protocol::OpBinary::encode_op` agreed on the first run — **30** and **35** exactly.

### FL1.1.4 Measured

```
cargo check -p semio-s-artifact-flow-flow --tests   → Finished, 0 errors (28 warnings, pre-existing)
cargo test  -p semio-s-artifact-flow-flow --lib -- editor::flow::commands::flow_eval_tick
  a_hop_discharges_only_the_window_its_payload_names        ... ok
  parked_extension_answers_own_the_continuation             ... ok
  a_tick_chain_settles_within_a_bounded_number_of_turns     ... ok
  a_settled_window_owes_no_hop_on_an_unchanged_snapshot     ... ok
  test result: ok. 4 passed
cargo test  -p semio-s-artifact-flow-flow --lib -- host_wire_witness   → ok. 1 passed
```

The four laws are in `…/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs`. The bounded-turn one drives the
real chain (`evaluate_result` then `tick_result` per emitted effect) and asserts it reaches a hop that
arms nothing inside 64 turns, and that no hop ever arms more than one successor. The settled-window one
is the refresh poll asked 16 times with no edit in between and requires zero hops — the exact shape that
minted 2015.

Run in `🗑️generated/fl1-flow-native-tests.txt`. **Honest**: `cargo test --lib` over the whole crate is
`170 passed; 76 failed` — every failure is `artifact store reached Drop without its exact terminal-empty
shallow-shell witness` (`🏪️store/🦀️.rs:18419`) in `add_widget`/`disconnect`/`reorganize`/`node_graph_edit`/…,
files this slice never touched. That is the known store-drop-witness debt, not this change.

Two pre-existing `[DEBUG]` `eprintln!`s were removed from tests this slice edited (preamble rule 10):
`…/🧪️tests/🔬️interactive-job/🦀️.rs` and `…/🧵️retained/🔎️wire/🧪️tests/🔎️wire/🦀️.rs`.

### FL1.1.5 PROVEN LIVE — the spin is gone

`📜️fl1-activate.sh flow 6016` was queued into the fleet wasm mutex at 17:24 (ticket
`20260920172433-43184-fl1`), took the lock at 17:44 and ran clean: `Activated flow react dev:
10 completed components (changed)`, all nine extensions named individually
(`Activated extension flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}`),
`Successfully ran target activate-flow-react-dev … and 33 tasks it depends on`, `exit=0`, 8 m 40 s
(`🗑️generated/fl1-flow-activate.txt`). That one activation also discharges EX1 §2.4's ABI rebuild for
the flow half of the 25 `extension_exports!` crates.

Served detached on a port this slice owns (`📜️fl1-serve.sh flow 6216`,
`[fresh] 10 staged components match their sources and the activation receipt`,
`🗑️generated/fl1-flow-serve.txt`), curl-warmed, probed with the SHARED probe, same command as §5.3:

```
SUMMARY {"ready":"flow","error":null,"exampleRendered":true,"actionCount":27,
         "mutated":false,"undone":false,"redone":false,"panelRoundTrip":false,
         "faultLines":1,"interactionBar":false}
```

| rung | §5.3 (pre-FL1) | now |
|---|---|---|
| fault lines | **2017** (2015 × `transient read registry is busy or exhausted`) | **1** |
| `transient read registry is busy or exhausted` | 2015 | **0** (`grep -c`) |
| `not a framework-reserved action` | 0 (§5.1 already fixed) | **0** |
| shell after `invoke-action` | DOWN — `data-semio-os-error="flow"`, `ready: null` | **UP** — `ready: "flow"`, `error: null` |
| Actions rail after `invoke-action` | gone, `actionCount: 0` | **27 rows** |
| staged `kind` argument | never reached | **`filled: ["kind=inputSlider"]`** (the FL1.3 rail fix) |
| boot / example / panels | ok | ok — artifact panel lists `slider (slider) inputSlider`, `add (math.add) neuron`, `preview (preview) outputPreview` |

**The §5.3 defect is closed at the root**: the re-armed tick addresses its own window, the latch admits
one hop at a time, and an unservable graph abandons instead of re-arming. Nothing spins, nothing
exhausts the read-lease registry, and the shell survives the mutation attempt it used to die on.

### FL1.1.6 What is left on 🌊️flow — a THIRD, unrelated blocker

`mutated` is still `false`, and the one remaining fault line says exactly why:

```
warning input #9 addWidget refused: dispatch-failed (user window=flow-compiled-dag) —
  typed-operation failed: retained command work refused the command before any capacity was measured
```

That message is raised by `ArtifactRetainedCommandJob`'s **preflight**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:538`) when
`work.extent(command, snapshot, interaction, context)` answers `None` — i.e. `FlowChildGroupWork` refuses
`addWidget` before any capacity is measured, for the dispatch that arrives under the **`flow-compiled-dag`**
window (the probe's active window; see the memory topic *Panel Actions Dispatch In Active Window*). It is
neither §5.1's missing bridge nor §5.3's spin: the command now decodes, reaches its own retained factory,
and is refused there. Untouched by this slice — `addWidget` and `FlowChildGroupWork` are not files FL1
edited — and it is the next thing to chase for flow's bar, together with the window the verb lands in.

The bar's flow clause (a graph evaluating nodes from three extensions) is therefore still **not**
measured; the chain that must carry it is alive for the first time.

## FL1.2 — the report's §4.3 diagnosis is wrong, measured

§4.3 concluded: "`app-body` clips at 942 px and its `scrollHeight` equals its `clientHeight`, so the
overflowing rail content is **clipped, not scrollable**". Re-measured at HEAD on :6018, 1600×1000,
headless chromium `--use-angle=metal`, walking the FULL ancestor chain of the staged control
(`🐍️fl1-rail-diagnose.mjs`, capture `🗑️generated/fl1-generation3d-rail-before.txt`):

```
tree                     overflow-y hidden  oh 960  sh 960  ch 960
window-action-pane       overflow-y visible oh 960  sh 960  ch 960
window-engagement-body   overflow-y auto    oh 901  sh 974  ch 901   scrollable TRUE
window-engagement-overlay  position absolute  max-height calc(100% - 6.4px)
window-body              overflow-y hidden  oh 907
… app-body               overflow-y hidden  oh 942  sh 942  ch 942
```

`app-body` is **not** the box that should scroll and never was — the Actions pane has its own scroller
two levels above it, and §4.3's dump elided it ("(7 more) overflow-y: visible"). With no app panels
raised the form is reachable at 1000 px today: `scrollIntoViewIfNeeded` → `ok`, a real un-forced
`click` → `ok`, and the combobox opens `["Neuron","Slider","Note","Preview"]`.

**Consequence for the fleet**: §4.3's advice ("a `TimeoutError` on a staged arg at 1600×1000 is not
evidence the verb is broken — re-run at a taller viewport") should not be used to excuse a timeout. The
real gate is narrower and is named below.

## FL1.3 — the real rail defect, and the fix

### FL1.3.1 Reproduction

The shared bar probe raises `framework.panel.inspection`, `framework.panel.artifact` and
`framework.panel.history` before it ever presses the rail. Mirroring that state
(`FL1_PANELS=… 🐍️fl1-rail-diagnose.mjs`, `🗑️generated/fl1-generation3d-rail-panels.txt`):

```
tree                    overflow-y hidden  oh 960  sh 960  ch 960   scrollable false
window-action-pane      overflow-y visible oh 960  sh 960  ch 960   scrollable false
window-engagement-body  overflow-y auto    oh 901  sh 998  ch 901   scrollable true
control action.addWidget.arg.kind  y = 1038   inViewport false
scrollIntoViewIfNeeded ok · click TimeoutError · options []
```

The pane body can scroll 97 px; the control needs to travel ~130. `[data-slot="tree"]` carries
`overflow-hidden` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4018`), so the rows past the band
are clipped by the tree rather than handed to the one scroller above it. **A rail long enough to exceed
its pane hides its own last rows.** generation3d has 32 rows and does; generation2d has 21 and does not
— which is the entire difference between the siblings, as §4.3 said, but for this reason, not app-body's.

### FL1.3.2 Fix

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`,
`WindowActionPane` — TS only, no guest rebuild, every app with a rail:

```tsx
<div data-slot="window-action-pane" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-y-auto overscroll-contain">
  <Tree className="shrink-0" … />
```

Both halves are load-bearing and the middle state was measured, not reasoned: with `flex-1 min-h-0` alone
the tree is shrunk to the band **too** and clips exactly the rows the change is trying to reach — pane
862, tree `offsetHeight 862` with `scrollHeight 960`, `scrollable: false` on every ancestor
(`🗑️generated/fl1-generation3d-rail-after.txt`). `shrink-0` keeps the tree's intrinsic height so the
pane has something to scroll. A pane body with no definite height is unchanged: `flex-1` has nothing to
shrink against.

### FL1.3.3 Measured after

`🗑️generated/fl1-generation3d-rail-after2.txt` / `-after3.txt`, same probe, same panel state, live vite
transform (the edit was confirmed present in the served host by the DOM change itself):

| | before | after |
|---|---|---|
| `window-action-pane` | `visible`, oh 960, sh 960, **not scrollable** | `auto`, oh 862, sh 960, **scrollable** |
| `tree` | oh 960, clipping the overflow | oh 960, intrinsic, inside the scroller |
| control after `scrollIntoViewIfNeeded` | y 1038, `inViewport: false` | y **941**, bottom 964, `inViewport: **true**`, `paneScrollTop 98 / range 98` |

The staged-argument form is now inside the viewport after a scroll the user (wheel), the keyboard (focus)
and a probe can all perform. Vitest law: `a 32-row rail scrolls its own band and keeps the staged-argument
form inside it` in `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — builds a 32-row rail whose
last row carries a `kind` arg, expands it, and asserts the pane declares `overflow-y-auto` + `min-h-0` +
`flex-1` and that both the control and the Execute button render **inside** that scroller. jsdom lays
nothing out, so the law pins the structure and the browser numbers above pin the behaviour. The whole
`window action panel` suite: **8 passed** (`🗑️generated/fl1-rail-vitest.txt`).

### FL1.3.4 What is STILL red, and why it is a different defect

The shared bar probe on generation3d is unchanged after the fix:
`filled: ["kind:TimeoutError: click: Timeout 8000ms exceeded."]`, `mutated: false`,
`interactionBar: false`, `faultLines: 0` (`🗑️generated/fl1-generation3d-staged-arg-probe-after.txt`).

The reason is now visible and is NOT the scroll. At the scrolled position the control is in the viewport,
and `document.elementFromPoint` at its centre answers

```
atCentre: DIV#(no id)[tree-gutter]
```

— the tree's **gutter** is painted over the property control's hit point, so a hit-tested click lands on
the gutter. Measured in the preview window's narrower rail (`framework.window.proceduralPreview.…`);
`Execute` sits at y 1017, also below the fold. Named here and not fixed: it is a `Tree` property-row
layout question (gutter vs. control hit area) in shared chrome, and the scroll fix above is the part this
slice could prove end to end. The next worker on it should start from
`🗑️generated/fl1-generation3d-rail-after3.txt`'s `afterScroll` block.

## FL1.4 Files changed

Rust (🌊️flow): `…/✏️editor/🦀️.rs`, `…/🎮️commands/{⏱️flow-eval-tick,🏁️flow-eval-resolve,🧮️evaluate,▶️run-extension-action}/🦀️.rs`,
`…/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🦀️.rs` (new), `…/🎮️commands/🧮️evaluate/🧪️tests/🔬️unit/🦀️.rs`,
`…/🧪️tests/{🔬️unit,🔬️interactive-job}/🦀️.rs`, `…/🧵️retained/🔎️wire/🧪️tests/🔎️wire/🦀️.rs`,
`…/🧫️fixtures/📡️host-wire/🔣️.json`.
TS (framework): `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`,
`…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`.
Ticket: `📜️fl1-activate.sh`, `📜️fl1-serve.sh`, `🐍️fl1-rail-diagnose.mjs`, captures `🗑️generated/fl1-*`.

Live server left running for the next worker: 🌊️flow react dev on **:6216**, detached (`📜️fl1-serve.sh flow 6216`,
log `🗑️generated/fl1-flow-serve.txt`) on the FL1 guest. Kill it by the pid in that log if it is in the way.
