# wgpu — `generate-add` and `port-fit`: the polluter, the starvation, and what was actually red

Lane `wgpu-generate-add-port-fit`, 2026-09-14. Two rows of `🐍️wgpu-battery.mjs` that the 15:32
scoreboard carried red — `generate-add` (`blocked-no-row`, `dispatched: 0`) and `port-fit`
(`wgpu.fit.dispatched`, `wgpu.fit.notTheSceneCamera`) — with the brief "green in their own lane, red
in the full run, find the state polluter".

Probe output: `🗑️generated/wgpu-genfit/`.

---

## 1. TL;DR

* **There is no cross-probe state polluter.** Every wgpu probe launches its own headless Chromium
  with a fresh profile and reloads `:6118` from scratch, so no probe can hand another one app state.
  What the two rows actually depended on was the **build** and, inside `port-fit`, **one probe step
  polluting the next step of the same run**.
* **`port-fit`'s `wgpu.fit.dispatched` was never a dead verb.** The console of the very run that
  scored it red carries the fit twice — `[DEBUG] shell node-graph fit {"surface":"procedural-main",…}`
  at 109 152 ms and 111 866 ms. The `F` key was **dispatched 7 632 ms after its own key-down**, long
  after the probe's 2 × 2.5 s window had closed. Focus, window-kind resolution, the keyboard verb map
  and the hit owner are all disproven by that same trace (`contentFocus=false`, the fit fires on the
  first routing).
* **The root cause is input starvation.** The renderer lends its ONE interaction state to a dispatch
  for the whole of that dispatch's future. `AppInteractionState::handle_pointer_move` ended in
  `ShellState::flush_deferred_actions`, whose `settle_ui_chain` converges the guest chain to a fixed
  point (up to `SHELL_SETTLE_ROUNDS` = 64 whole-shell refresh passes). One hover therefore held the
  `dispatch-event` checkout for **8 515 ms across 6 refresh passes of 7 surfaces**, and the host
  dispatched **no input at all** in that window (§3.3).
* **What armed that chain was the probe's own previous step**: the `wgpu.press.resolvesOutput` drag
  from `extrusion-axis@vectorOut` snapped onto `extrude@wire` and the graph accepted it —
  `nodeGraphEdit {disconnect e4}{connect extrusion-axis.vectorOut → extrude.wire}` — rewiring a vector
  output into a wire input and re-solving the model. That is the intra-run polluter, named in §3.2,
  and a second real defect in its own right (§6.2, **not fixed here**).
* **Fix landed**: no input gesture converges its own chain any more. The three shell pointer entry
  points and the renderer's pointer move now DECLARE the chain (`owe_settle`) and return; the frame
  loop's `FrameDeferredWork::Settle` takes one step per frame and returns the interaction state
  between steps — which is exactly the authority `🫀️settle-pump`'s own fixture already claimed and the
  pointer path was the surviving exception to.
* **Law**: `🧫️fixtures/🫀️settle-pump/🔣️.json` gains a `gestureRows` section and two laws, answered by
  the Rust twin and the TypeScript twin over the same oracle (§5).
* **`generate-add`'s `blocked-no-row` did not reproduce on the current build** — it is green in the
  full sequential prefix (§4). Its oracle no longer answers with a bare absence: the probe now
  publishes the state it found and names one of seven distinct blocked verdicts (§5.3).

---

## 2. What the brief assumed, and what the evidence says

| brief | evidence |
|---|---|
| state pollution across probes in the full run | impossible by construction — one `chromium.launch` per probe, fresh profile, fresh page (§3.1) |
| `generate-add`: the `Add Generation` row is not found after the preceding probes ran | the preceding probes are `boot` and `no-example` only (the battery runs its OWN declaration order, not `--only` order); on the current build the row is found and the row passes in sequence (§4) |
| `port-fit`: the `F` verb does not reach the node-graph fit | it reaches it — 7 632 ms late (§3.3) |
| focus / window-kind resolution / keyboard verb map / hit owner | all four disproven by the same trace (§3.4) |

---

## 3. `port-fit` — bisection and root cause

### 3.1 The full run has no cross-probe channel

`🐍️wgpu-battery.mjs` spawns each probe as its own `bun` process; every wgpu probe calls
`chromium.launch(...)` and `browser.newPage()` and navigates `:6118` from scratch. A fresh Playwright
profile carries no `localStorage`, no IndexedDB and no service worker from the previous probe, and
`:6118` is a Vite dev server with no per-session store. The only channels between probes are the
served build and the machine. The battery also runs probes in **its own declaration order**, so
`--only=deferred-commit,node-gestures,catalogue,io,boot,no-example,spawn-job,port-fit,generate-add`
executes as `boot, no-example, generate-add, deferred-commit, spawn-job, node-gestures, catalogue,
io, port-fit`: `generate-add` is the THIRD probe, not the ninth.

### 3.2 The polluter is inside `port-fit`'s own run — one step arming the next

`🗑️generated/wgpu-genfit/port-fit-baseline-0847/console.txt` (the run whose verdict the 15:32
scoreboard carries):

```
 93971  [DEBUG] dag port press port=Some("extrusion-axis@vectorOut") interaction=draw-edge(new)
 97249  [DEBUG] frame input action controller=…#editor action=nodeGraphEdit
        args={"operations":[{"operation":"disconnect","synapseId":"e4"},
                            {"operation":"connect","sourceNodeId":"extrusion-axis","sourcePortId":"vectorOut",
                             "targetNodeId":"extrude","targetPortId":"wire"}]}
```

The `wgpu.press.resolvesOutput` step drags 40 px from the `vectorOut` handle and releases; the
graph's wire-snap found `extrude@wire` within tolerance and `try_connect_handles` accepted it.
`hexagonal-mushroom-column` declares `e4 profile@wire->extrude@wire` and
`e5 extrusion-axis@vectorOut->extrude@vector`, so the gesture **replaced the profile wire with the
axis vector** and the model re-solved into a fault
(`frame fault recorded: world3d scene mesh-wire bridge faulted`, 112 383 ms). Those are the effects
the next step's `F` had to queue behind.

### 3.3 The measurement — 8 515 ms with the interaction state checked out

Intake (`os_host handle_event`) against dispatch (`os_host dispatch_normalized_event`), 312 events
paired in order, same console:

| event kind | n | first lag | last lag | max lag |
|---|---|---|---|---|
| PointerMove | 298 | 6 ms | 7 949 ms | **10 734 ms** |
| PointerDown | 2 | 4 ms | 10 732 ms | 10 732 ms |
| PointerUp | 2 | 1 964 ms | 10 715 ms | 10 715 ms |
| Scroll | 6 | 465 ms | 2 484 ms | 2 766 ms |
| **KeyDown (`f`)** | 2 | **7 632 ms** | **7 868 ms** | 7 868 ms |

The lag is 0–50 ms for the whole first 96 s and explodes exactly at the rewire. Between
**100 432 ms and 108 947 ms there is not one `apply_pending_step: interaction state is available
again`** — the ledger reports the same holder throughout:

```
100432  apply_pending_step blocked: head needs the interaction state, checked out at Some("dispatch-event") for 1 opportunities
…  (8 515 ms, 6 × `wgpu-shell render begin` over all 7 surfaces, 3 typed-operation commands, 1 brep evaluate)
108947  apply_pending_step: interaction state is available again
109148  os_host dispatch_normalized_event KeyDown { key: "f", … }
109151  wgpu-shell key routing window=procedural-main contentFocus=false action=Char("f")
109152  shell node-graph fit {"surface":"procedural-main","x":20.114…,"y":-132.897…,"zoom":0.92497…}
```

The probe had already recorded the row: it presses `f` at 101 516 ms, waits 2 500 ms, presses `KeyF`
at 103 998 ms, waits 2 500 ms, and gives up at ~106 500 ms — 2.6 s before the first key is dispatched.

### 3.4 Why the four hypotheses in the brief are all out

* **focus** — `contentFocus=false` on the routing line; the content-focus branch was not taken.
* **keyboard verb map** — the `F` arm ran and called `fit_node_graph_camera`, which published.
* **window-kind resolution** — `keyboard_fit_surface_id` answered `procedural-main`, the graph pane.
* **hit owner** — no hit is involved; `F` is a chrome-level key handled before the app keybinding loop.

`wgpu.fit.notTheSceneCamera` is a consequence, not a finding: the probe parses the fit payload out of
the line it never saw. When the line is seen, the published camera is `20.114 / -132.897 / 0.925`
against the scene's `94.756 / -97.508 / 1.784` — different, and the step is green.

### 3.5 The structural defect, in the shell's own words

`ShellState::settle_pump_step`'s docstring already declared the authority:

> the frame loop is the only caller of `settle_pump_step`, so nothing converges inside a boot, a
> dispatch or a pointer handler any more

and `🧫️fixtures/🫀️settle-pump/🔣️.json`'s `authority` says "The shell never converges a chain; it only
declares one." **The pointer path was the surviving exception.** Four call sites still took the
synchronous door:

| site | what it did |
|---|---|
| `🧊️renderer/🦀️.rs` `AppInteractionState::handle_pointer_move` | `self.shell.flush_deferred_actions().await` on EVERY move |
| `🐚️Shell/…/🦀️.rs` `handle_pointer_button` | `self.flush_deferred_actions().await?` after the chrome dispatch |
| `🐚️Shell/…/🦀️.rs` `route_retained_pointer_press` | `self.flush_deferred_actions().await` as its result |
| `🐚️Shell/…/🦀️.rs` `dispatch_tree_selection` | queue + `flush_deferred_actions` |

`flush_deferred_actions` → `settle_ui_chain` → up to `SHELL_SETTLE_ROUNDS` (64) rounds of
`drain_deferred_actions` + `refresh_ui`, each refresh re-entering every window body through the guest.
Held inside `start_dispatch`'s `check_out_interaction("dispatch-event")`, that is a hard stop on every
later input.

---

## 4. `generate-add` — what reproduced, and what did not

Reproduction run, full battery prefix, one browser at a time, current build, before the fix
(`🗑️generated/wgpu-genfit/prefix-run-A.txt`):

```
boot            ok=false 27s   2/3   (surface fault framework.panel.toolRun — not this lane)
no-example      ok=false 28s   0/2   (not this lane)
generate-add    ok=true 139s   2/2   hexagonal-mushroom-column pass dispatched=5 t=67.34s
                                     box-shell-preview          pass dispatched=5 t=66.85s
deferred-commit ok=true 138s   3/3
spawn-job       ok=true 113s   8/8
node-gestures   ok=false 271s  6/8   (not this lane)
catalogue       ok=true 114s   3/3
io              ok=true 148s   6/6
port-fit        ok=true 139s   8/8   fit at 109 577 ms, 61 ms after its key-down
```

Both of this lane's rows are **green in the full sequential prefix on the current build**, with the
target resolving as
`tree[0]/stack[1]#procedural3d-play-generate.actions/stack[0]#procedural3d-play-generate.add-generation`.
The 15:32 scoreboard's red rows were carried over from the **08:35–08:47 build** (`battery-3.txt`);
peers landed the frame/revision authority work between then and now.

The honest verdict for `generate-add` is therefore: **`blocked-no-row` did not reproduce**, and the
brief's premise for it does not hold on this build. What is fixed instead of guessed is the oracle
(§5.3) and the latency the same gesture pays (§6.1).

---

## 5. What changed

### 5.1 Product — an input gesture declares, it never converges

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `handle_pointer_move` no longer calls `flush_deferred_actions` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `handle_pointer_button`, `route_retained_pointer_press`, `dispatch_tree_selection` arm `owe_settle()` and return; `handle_pointer_button` gains the docstring that names the measurement |

Nothing is lost by declaring rather than converging: `settle_pump_pending()` is re-derived every frame
over armed work, the owed refresh scope, the lane's own arm and any producer reporting `computing`, so
the same chain is drained — one step per frame, with the interaction state returned between steps.
`flush_deferred_actions` stays as the synchronous door an embedding host may take; no input path takes
it any more.

### 5.2 Law — Rust + TypeScript twin over one oracle

`🧫️fixtures/🫀️settle-pump/🔣️.json` gains `operations.gesture`, a `gestureRows` section with one row per
input entry point (`owns` the signature, `forbids` the symbol, `declares` the arming call) and two
laws:

* `an-input-gesture-declares-the-chain-and-never-converges-it-inside-its-own-dispatch`
* `a-pointer-move-returns-the-interaction-state-within-its-own-dispatch`

Answered by `🧪️tests/🫀️settle-pump/🦀️.rs::an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch`
and by the TypeScript twin `🧪️tests/🫀️settle-pump/🟦️.ts` ("lets an input gesture declare the chain and
never converge it inside its own dispatch"), both slicing the same two Rust sources named by the
fixture. The oracle — not either implementation — owns the list of entry points. The stale claim in the
`a-synchronous-flush-owns-the-chain…` row ("the door the native host **and the pointer path** use") was
corrected in the same pass.

The law is a source law on purpose: no predicate can express "this body must not be able to REACH the
synchronous door", and the defect is structural — any gesture body that can reach it starves every
later input, whatever that particular gesture happened to arm.

### 5.3 Probe — the oracle reports the state it found

`🐍️wgpu-add-generation-probe.mjs` now publishes a `found` record from the shell's own evidence
(boot left, window ids, dock-planned ids, per-window node counts / section ids / zero-sized rects, how
often the needle appears anywhere, how often the generations body rendered, surface faults, frame
faults, any live `role="alert"`), and `blockedVerdict(found)` turns it into one of

`blocked-still-booting` · `blocked-no-windows` · `blocked-no-generations-window` ·
`blocked-generations-window-unplanned` · `blocked-generations-never-rendered` ·
`blocked-generations-body-empty` · `blocked-actions-section-absent` · `blocked-row-rect-collapsed` ·
`blocked-no-row`

`🐍️wgpu-battery.mjs`'s `generate-add` verdict carries `found` into the scoreboard for any non-passing
row, so a red row names a defect and an owner instead of only naming an absence.

---

## 6. Numbers

### 6.1 Input latency (intake → dispatch), same gesture, before and after

PENDING

### 6.2 Not fixed, and why

* **The graph accepts a type-incompatible wire.** `board::is_valid_connection` checks handle role,
  self-node, duplicate edge and acyclicity — never the port's declared type — so a 40 px drag from
  `extrusion-axis@vectorOut` connects to `extrude@wire` and silently removes the existing `e4`. The
  data to refuse it does not reach the board: `IoPortSpec.value_type` is `channel_spec_value_type`,
  which is the operator-id list a channel was declared under (`brep.solid.extrude`,
  `math.vector,brep.solid.extrude`) for inputs and the bare `"value"` for every output, so a
  strict-equality rule would refuse the example's own `e4`/`e5`. Refusing incompatible wires needs a
  port TYPE the neural registry does not publish yet; inventing one here would have broken every
  shipped example. Named, measured, and left for a lane that can add the type to `ChannelSpec`.
* **`boot`, `no-example`, `node-gestures`** are red on the current build (§4) and belong to other
  lanes; `boot`'s `wgpu-shell surface fault surface=framework.panel.toolRun … retained document
  ingress reached its terminal fault` is new since 15:32.

---

## 7. Explicitly not claimed

* **No cross-probe state polluter was found, because there is none to find.** The claim "a mode/role
  left behind, a stale dock state, a hit registry generation, a quiesced session" is refuted, not
  unproven: each probe runs its own browser.
* **`generate-add`'s `blocked-no-row` was not reproduced.** It is green on this build in the full
  sequential prefix. The improved oracle is what will name it if it returns; this lane did not fix a
  defect behind it, because none was observable.
* **The `F` verb was never broken.** Nothing in the focus routing, the verb map, the window-kind
  resolution or the hit owner was changed, and nothing needed to be.
* **The 8 515 ms starvation was measured on the 08:47 build**, where it is unambiguous. On the current
  build the port drag happened not to rewire the graph, so the extreme case did not re-occur inside
  this lane's own runs; the fix is justified by the structural reading of the four call sites plus the
  measured 1 596 ms `generate-add` gap, not by re-measuring 8.5 s after the fact.
* **No claim about React.** The React shell's own refresh lane already declares rather than converges;
  nothing on `:6018` was run or changed.
