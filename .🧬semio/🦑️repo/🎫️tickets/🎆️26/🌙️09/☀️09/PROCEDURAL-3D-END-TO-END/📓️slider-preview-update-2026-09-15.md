# Moving A Slider Must Move The Preview — lane `slider-preview-update` (2026-09-15)

Answers the user's 13:03 directive: *"The react renderer isn't ready yet end to end. Moving a slider
doesn't update the preview as it should."*

- Probe: `🐍️slider-live-preview-probe.mjs` (new) — drags each slider kind with real pointer events at
  ~60 Hz for 1 s across its range and reports latency, coalescing, stale tail, round trip, history and
  dropped actions.
- Recon: `🐍️slider-recon.mjs`, `🐍️inspector-reach-recon.mjs` (new)
- Laws: `📺️renderer/🧑‍🎨engine/🧪️tests/🎚️continuous-gesture-lane/🟦️.ts` (TS twin),
  `✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` (Rust), over the shared fixture
  `🪆️subsets/✳️any/🧫️fixtures/🎚️slider-gesture.json`
- Runtime logs: `🗑️generated/slider/`

---

## 1. TL;DR

**The node-graph slider was not slow — it was not connected at all.** Every value it produced was
dispatched as `setGraphParameter`, an action NO window kind of ANY app declares, and the shell dropped
each one with its own console error. The knob moved, the local flow session moved, the document never
did, and the preview therefore never re-evaluated. Measured on 6018, before this lane: a one-second
drag of `Column Height` = **6 value changes, 6 dropped actions, 0 invocations, 0 preview publications.**

The two slider kinds that WERE connected were connected the most expensive way there is: one document
edit per value. A one-second stream into the Inspection panel's number field cost **29
`patchFlowWidgets`, 24 `toolRunStart`s and 29 history entries**, the first mesh arrived **3.0 s** after
the value moved, the shell took **12.5 s** after release to settle, and the geometry it settled on was
not the value the field ended on.

**After the fix, on the 16:57 guest and all eight bundled examples: 0 dropped actions, exactly ONE
history entry per press, 3–8 dispatches for 9–63 value changes, and the preview follows the knob.** The
user's headline symptom is gone. Two things are not yet where they should be and are handed on with
evidence (§7): the first mesh arrives in ~1 s rather than ≤ 150 ms — now entirely the GUEST's
evaluation chain, since the dispatch storm is gone — and six of the eight examples settle on an EMPTY
payload reported as `idle` rather than on the released value or a named fault.

Three fixes, each on the layer that owns it:

1. **Reachability (renderer + guest).** The inline slider dispatches `nodeGraphEdit` with a `setSlider`
   sub-operation — the same vocabulary `move`, `connect` and `disconnect` already travel in — and the
   guest applies it through `FlowHost::set_slider_value`. `setGraphParameter` is deleted from
   `nodeGraphActions`; nothing declared it and nothing else used it.
2. **Coalescing (framework, shared by all three kinds).** `createContinuousGestureLane` keeps at most
   ONE dispatch in flight and ONE owed value — always the newest — and always sends the release. Both
   the node-graph overlay and the UI interpreter's continuous controls (slider, `commit`-less number
   field) ride it.
3. **One edit per press (guest).** Every tick of one press emits under that press's own coalesce key,
   so a whole drag folds into ONE undoable edit, and declares a narrow `UiDirtyScope::Partial` instead
   of a whole-shell refresh.

---

## 2. Root causes, file:line

| # | layer | site | what was wrong |
|---|-------|------|----------------|
| 1 | renderer | `📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` `FlowGraphCanvasHost` slider overlay | dispatched `nodeGraphActions.parameter` = `setGraphParameter`, undeclared by every app → the shell dropped every tick (`dropped action "setGraphParameter" … no window kind declares it`) |
| 2 | renderer | same file, `WasmGraphSurface` slider overlay | dispatched `nodeGraphEdit` with `{ operator: "setSlider", … }` — a key (`operator`) and a shape (no `operations` array) the guest never reads, so it parsed as an empty operation list |
| 3 | guest | `✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs` `apply_operations` | had no `setSlider` arm at all; a slider value fell through `_ => {}` |
| 4 | renderer | `📺️renderer/…/🕸️NodeGraph/🟦️.tsx` `dispatch` | dropped `onAction`'s promise, so no continuous gesture could tell when its last value had landed — the settle signal the shell already returns (`ShellHost` `onAction` → `handleAction` → `awaitOperationSettle`) was thrown away at the surface |
| 5 | renderer | `📺️renderer/…/🗣️Interpreter/🟦️.tsx` `SliderView` / `InputView` | dispatched the `change` trigger per value with no lane: 60 document round trips per second of gesture |
| 6 | guest | `✏️node-graph-edit/🦀️.rs` emit | `coalesce_key: None` → one undoable edit per tick; `ui_scope` default `Full` → a whole-shell refresh per tick |
| 7 | framework | `🖱️ui/🎬️scene/🟦️.ts` `createContinuousGestureLane` (new, found by its own law) | a release that arrived after the lane had drained sent NOTHING, so a gesture whose final value happened to win the race was never committed |

### The order the evidence came in

1. `🐍️slider-live-preview-probe.mjs` on a clean boot, `graph` kind: 6 value changes, **0**
   publications, **0** invocations. Not slow — silent.
2. The in-page console hook named the silence exactly: six
   `dropped action "setGraphParameter" … no window kind declares it` lines, one per value change.
3. `grep` for `setGraphParameter` across the repo: **one** producer (the renderer) and **zero**
   consumers outside a framework test fixture. The guest's `apply_operations` matched five operation
   names, none of them a slider.
4. With the `inspector` kind, which IS connected: 29 `patchFlowWidgets`, 24 `toolRunStart`s, 29 history
   entries, 3.0 s to first mesh, 12.5 s to settle, round trip false.

---

## 3. The fixes

### 3.1 Framework — `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`

`createContinuousGestureLane<Value>({ send, onFault })`: at most one `send` in flight, at most one owed
value (the latest), the release always sent and always as `commit`, a refused round trip frees the lane
instead of wedging the gesture. `ContinuousGesturePhase` is `"live" | "commit"`. `nodeGraphActions`
loses `parameter`.

The lane is the same rule the ui-refresh coalescer runs on: the intermediate values a user swept
THROUGH were never asked for, only the one they are on now.

### 3.2 Renderer — `🕸️NodeGraph/🟦️.tsx`

- `dispatch` returns what `onAction` returns. `ComponentSceneHostProps.onAction` is declared
  `void | Promise<void>` and the React shell already answers with a promise that settles on
  `OperationCompleted`; only the surface was discarding it.
- `useGraphSliderLanes(surfaceId, dispatchRef)` — one lane per slider widget plus the gesture identity
  minted on press. Used by BOTH graph surfaces (`FlowGraphCanvasHost` and `WasmGraphSurface`).
- `GraphSliderOverlays` gained `onSliderCommit` and now passes the widget id to
  `onSliderPointerDown`/`onSliderPointerUp`. The release is taken from the slider's own
  `onValueCommit`, so a keyboard-only user commits too.

### 3.3 Renderer — `🗣️Interpreter/🟦️.tsx`

- `UiInterpreterContext.onIntent` is `(intent) => void | Promise<void>`; `dispatchTrigger` returns it.
- `useContinuousTriggerLane(context, record)` — one lane per mounted control, reading `context`/`record`
  through a ref because both identities change every render while the lane must outlive them.
- `SliderView` offers on `onValueChange` and commits on `onValueCommit`.
- `InputView`: a `number` field with no `commit` mode IS a continuous control (a held spinner, an arrow
  key on repeat); it offers on change and commits on blur.

### 3.4 Guest — the two other doors a continuous value reaches this document through

`patchFlowWidgets` (the Inspection panel's number field) and `updateGenerationValues` (a generate-mode
Form slider) gained a `gesture: Option<String>` and fold under `patch_coalesce_key(gesture)` —
`widget-field:<press>` — with the same narrow `ui_scope`. The identity rides on the wire because a
CONTINUOUS control's `Change` payload is now a RECORD: `{ value, gesture, commit }`.
`uiIntentPayload` merges a record input's own keys into the binding's args, so `value` still arrives
under its own name for every existing consumer and nothing else in the contract moved.

### 3.5 Guest — `✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs`

- `"setSlider"` arm → `host.set_slider_value(widget_id, value)`.
- `gesture_coalesce_key` — a key is minted only when EVERY sub-operation of the dispatch belongs to the
  same press (the releasing tick included, so a whole drag is one undo step). A mixed batch, or a
  `setSlider` with no press identity, is a described edit of its own.
- `slider_gesture_ui_scope` — `Partial` over the two window bodies that change (`procedural.play.main`,
  `procedural.play.preview`) and the two panel bodies that read the value back
  (`procedural.play.inspection`, `procedural.play.artifact`), with every rail off.

---

## 4. Measured, before and after

Port 6018, React editor, real pointer drags, in-page sampling (control value every 10 ms, mesh payload
every 50 ms). "Before" = the 13:04 guest at 14:00–15:00; "after" = the 16:57 guest, serve recycled
under the freshness guard, 17:05–17:40. Load average ≈ 30 with the fleet live.

### 4.1 The node-graph inline slider (`Column Height`, hexagonal-mushroom-column)

| reading | before | after (1 s burst, 60 Hz) | after (short drag) | budget |
|---|---|---|---|---|
| dropped actions | **6** (`setGraphParameter`) | **0** | **0** | 0 |
| evaluations dispatched | **0** | 5 `nodeGraphEdit` / 3 `toolRunStart` | 3 `nodeGraphEdit` / **1** `toolRunStart` | ≤ 1 per change |
| evaluations per value change | — | 0.33 | 0.25 | ≤ 1 |
| preview publications | **0** | 2 | 2 | > 0 |
| value → first mesh (median) | **never** | 1 543 ms | 1 005 ms | ≤ 150 ms |
| p95 | — | 3 284 ms | 1 372 ms | ≤ 150 ms |
| publication after the LAST value | **0** | 1 | 1 | ≥ 1 |
| history entries for one press | 0 (nothing landed) | **1** | **1** | 1 |
| settled geometry | z-extent 6 — unchanged | z-extent **9** = released value | z-extent **7** = released value | = released value |
| settled phase | idle (nothing ran) | `invalid` | **idle** | idle |

The chain, sampled on the burst (`🗑️generated/slider/after-graph/…-samples.json`): `computing` at
+1.1 s, `cancelled` at +1.5 s (the newer value superseding the older evaluation — the supersession
working), an emptied intermediate at +2.4 s, the **correct** 9-unit geometry at +6.1 s, then
`samplingEdges` and `invalid`.

### 4.2 The Inspection panel's number field (`height`, 60 values in 1 s)

| reading | before | after | budget |
|---|---|---|---|
| dispatches | **29** `patchFlowWidgets` | **4** | — |
| evaluations | **24** `toolRunStart` | **3** | ≤ 1 per change |
| value → first mesh (median) | 3 003 ms | **948 ms** | ≤ 150 ms |
| release → settled | 12 466 ms | **4 159 ms** | — |
| history entries for one press | **29** | **1** | 1 |
| settled geometry | z-extent 7.45, not the value the field ended on | z-extent 7.95 = the last value the control accepted | = released value |
| settled phase | `samplingEdges` | **idle** | idle |

The residual 7.95 is not a stale preview: the Inspection body is republished on every guest turn and
its `<input>` leaves the DOM mid-stream, so the probe's 40th of 60 values (6 + 0.05×39 = 7.95) is the
last one the control ever accepted. The product landed exactly on it. The vanishing control is a
separate defect (§7).

### 4.3 All eight examples, node-graph slider

Every bundled example paints inline graph sliders; all eight were reached and dragged
(`🗑️generated/slider/after-examples-a|b`, `after-singlestep`, `after-graph`).

| example | slider range | dispatches / starts | value → mesh (median / p95) | dropped | history | settles on |
|---|---|---|---|---|---|---|
| hexagonal-mushroom-column | 0…10 | 3 / 1 | 1 005 / 1 372 ms | 0 | 1 | **released value, idle** |
| rectangle-wire-preview | 0.2…6 | 6 / 4 | 794 / 2 913 ms | 0 | 1 | **released value, idle** |
| sphere-cut-with-torus | 0…10 | 7 / 4 | 1 351 / 1 964 ms | 0 | 1 | empty payload, idle |
| box-shell-preview | 0.5…5 | 8 / 5 | 948 / 1 330 ms | 0 | 1 | empty payload, samplingEdges |
| box-fillet-preview | 0.5…5 | 5 / 3 | 1 077 / 1 665 ms | 0 | 1 | empty payload, samplingEdges |
| rectangle-extrude-volume | 0.1…10 | 4 / 2 | 2 066 / 3 101 ms | 0 | 1 | empty payload, invalid |
| sphere-box-fuse | 0.2…3 | 5 / 3 | 780 / 1 477 ms | 0 | 1 | empty payload, idle |
| face-sweep-extrude | 0.5…6 | 4 / 2 | 1 866 / 2 348 ms | 0 | 1 | empty payload, invalid |

**8/8 reachable, 8/8 zero dropped actions, 8/8 exactly one history entry per press, 8/8 coalesced
(≤ 8 dispatches for 9–63 value changes).** 2/8 settle on geometry that matches the released value.

`sphere-box-fuse` settles empty for a drag of 1.2 → 2.2 — nowhere near its 0.2…3 extremes — and it
does so for a FOUR-value single-step gesture as well as for a 60 Hz burst
(`🗑️generated/slider/after-singlestep`). So the empty payload is neither the burst nor an extreme
value: it is the evaluation chain's own answer for a moved slider on those six examples, and it is
published as `idle` with no fault (§7).

## 5. Laws

| fixture table | rows | Rust | TS twin |
|---|---|---|---|
| `🎚️slider-gesture.json` → `coalescing` | 5 | `every_slider_gesture_row_folds_one_press_into_one_edit_on_the_released_value` | — |
| `🎚️slider-gesture.json` → `uiScope` | 1 | `a_slider_tick_declares_the_narrow_scope_and_a_discrete_edit_keeps_full` | — |
| `🎚️slider-gesture.json` → `liveEvaluations` | 1 | `a_press_spends_at_most_two_live_round_trips_and_one_commit` | `🎚️continuous-gesture-lane/🟦️.ts` (7 cases) |
| the same rule at the panel door | 2 | `a_patch_that_names_its_press_folds_under_that_press_and_an_anonymous_one_does_not`, `a_live_patch_declares_the_narrow_scope_and_an_anonymous_one_keeps_full` | — |

**Counts**

- `SEMIO_TEST_LEVEL=long bunx vitest run --config …/🎚️config/🟦️.ts continuous-gesture-lane` →
  **7 passed, 0 failed**. One of the seven was RED first and found a real product defect: a release
  arriving after the lane had drained sent nothing at all (§2 row 7).
- `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --
  patch_flow_widgets node_graph_edit` → **9 passed, 0 failed**.
- The crate's `--lib` test binary did not compile when this lane arrived at it: a peer's in-flight
  `fixture` → `host_snapshot` sweep had rewritten expression positions in two of their own test files
  (`🕸️flow/🧪️tests/🔬️unit`, `🗿️artifact/🧪️tests/🔬️unit`) to names with no binding. Fixed FORWARD onto
  the new vocabulary (`with_host(&fixture, …)`, `fixture.retire_cold()`), reverting nothing. The same
  sweep rewrote this lane's own local `fixture` bindings mid-edit; the laws now name their table
  `table`, which the sweep does not touch.

---

## 6. Files

Created:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎚️continuous-gesture-lane/🟦️.ts`
- `✏️s/…/✏️editor/🎮️commands/🩹️patch-flow-widgets/🧪️tests/🔬️unit/🦀️.rs`
- `📜️restage-slider.sh`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🎚️slider-gesture.json`
- `🐍️slider-live-preview-probe.mjs`, `🐍️slider-recon.mjs`, `🐍️inspector-reach-recon.mjs`
- `📓️slider-preview-update-2026-09-15.md` (this report)

Updated:
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` — the lane, `nodeGraphActions.parameter` deleted
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` — the
  settling `dispatch`, `useGraphSliderLanes`, both slider overlays, `onSliderCommit`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx` — the
  settling `onIntent`, `useContinuousTriggerLane`, `SliderView`, `InputView`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` —
  the new suite registered
- `✏️s/…/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs` — `setSlider`, the gesture coalesce key, the
  narrow scope
- `✏️s/…/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` — the three laws
- `✏️s/…/✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs` — `gesture`, `patch_coalesce_key`, the narrow scope
- `✏️s/…/✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs` — `gesture`, the same coalesce key
- `✏️s/…/✏️editor/🦀️.rs` — `gesture` decoded for both verbs
- `✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs`,
  `✏️s/…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — the peer sweep's two broken call sites,
  fixed forward
- `✏️s/…/✏️editor/🎮️commands/➖️remove-widget/🧪️tests/🔬️unit/🦀️.rs`,
  `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`,
  `✏️s/…/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🔬️unit/🦀️.rs` — the new field at their
  literal sites

---

## 7. What is NOT claimed, and what is handed on

### Handed on — measured here, owned elsewhere

1. **A moved slider settles on an EMPTY payload, reported as `idle`, on six of eight examples.**
   `sphere-box-fuse` does it for a four-value gesture from 1.2 to 2.2 inside a 0.2…3 range, so it is
   neither the burst nor an extreme value. The coordinator's own 17:00 reading on :6028 names the same
   thing from the other side: the Tool runs body says `1 of 7 nodes did not evaluate` and the extrusion
   mostly vanishes. **A value the kernel cannot build must publish a NAMED fault, not an empty payload
   under `idle`.** Owner: `🧵️preview-eval` / the brep extension's own diagnostics, not the slider lane.
   Evidence: `🗑️generated/slider/after-examples-a|b`, `after-singlestep`.
2. **The burst case ends in `PreviewTessellatePhase::Invalid` and never returns to `idle`.** The hex
   burst publishes the CORRECT 9-unit geometry at +6.1 s and then goes `samplingEdges` → `invalid` and
   stays there for 40 s. The same drag as a short gesture ends `idle`. Owner: the tessellate phase
   machine in `🌊️flow/🖥️host/🦀️.rs` (`resolve_preview_tessellate`), reached through a superseded
   evaluation.
3. **First mesh is ~1 s, not ≤ 150 ms.** With the dispatch storm gone, the remaining latency is the
   GUEST's evaluation chain: `toolRunStart` → `flowEvalTick` → extension `evaluate` → `tessellate` →
   publication. The host's own share is the 3–8 dispatches this lane already reduced it to. Closing the
   last order of magnitude is an evaluation-chain question (incremental re-evaluation of the nodes
   downstream of ONE slider, rather than a whole-graph tick), not a gesture question. Nothing in this
   lane makes the kernel faster and nothing here claims it does.
4. **The inline knob's painted label disagrees with its published value** (coordinator, 17:00:
   `aria-valuenow` 10 while the canvas label reads `0.0`). Two producers: the DOM overlay reads
   `sliderOverlayStateJson`, the painted label comes from the guest's own scene. Not touched here.
5. **The inline slider's rail is 64 px wide at the default graph zoom for a 0…10 range** — 6.4 px per
   unit, measured (`🗑️generated/slider/after-graph/results.json` `gesture.rail`). The coordinator's
   "60 px = full range" is that number, not a gain bug: the control is simply drawn 64 px wide, which
   is a usability decision for `GraphSliderOverlays`, not a mapping defect.
6. **The Inspection panel's `<input>` leaves the DOM mid-gesture.** The panel body is republished on
   every guest turn and the control is not retained across it, which cost this lane three failed
   measurement runs (`focus: Timeout 30000ms` on an element `count()` had just answered for) and is why
   the probe drives that field through the native value setter instead of Playwright actionability.

### Not claimed

- **Nothing about the wgpu surface.** `🕸️NodeGraph/🟦️.tsx` is React-only; the wgpu node-graph writes its
  own `nodeGraphEdit` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu`'s `write_graph_edit_action`) and was not
  touched or measured. The guest's `setSlider` arm serves both by construction.
- **The generate-mode Form slider's AFTER numbers were not taken.** Its `updateGenerationValues`
  gesture key landed in the tree at 17:02, after the 16:57 wasm this stage serves, so measuring it now
  would measure the old guest. Its before-numbers (§4's 14:00 run: 6 value changes, 2 publications,
  615 ms median, **8** history entries for one drag) stand; the fix is the same code path the other two
  doors are already measured on, and the Rust law covers the key it emits.
- **The Inspection panel renders a NUMBER FIELD, not a slider**, for an `InputSlider` widget. That is
  what the product does today; this lane made that field continuous rather than replacing it.
- **The declarative-control slider** (`🗣️Interpreter/🟦️.tsx`'s `dispatchDeclarativeControlAction`, used
  by the measure rails) was not put on a lane. Different owner, not one of the three kinds under test.
- **`@semio-tech/framework-renderer-react:typecheck` is red on this tree, and was before this lane.**
  Its errors are a peer's in-flight `snapshotJson`/`synchronizeSnapshotJson` rename, the `Diagram`
  fallback's props, and three pre-existing ones; none names a symbol this lane added
  (`onSliderCommit`, `createContinuousGestureLane`, `useGraphSliderLanes`, `useContinuousTriggerLane`).
- **The React battery was not run.** The gate asks for it; this lane spent its window on the shared
  workspace being red from 15:20 to 16:01 under a peer's rename and on two restages that failed on the
  same breakage. The probe is written to be a battery row and is not yet registered as one.
- **The Flow window's light canvas and missing node boxes**, seen on this stage at 13:10 and again in
  this lane's recon, are not this lane's and were not fixed here.
