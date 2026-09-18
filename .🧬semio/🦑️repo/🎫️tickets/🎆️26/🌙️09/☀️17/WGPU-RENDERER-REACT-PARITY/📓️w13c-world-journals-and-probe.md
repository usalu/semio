# 🌍️🫧 W13c — the four world/dropdown journal rows, the ingress fold, and Wave 13's integration loop

Packet W13c of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Input: `📓️w12c-chords-and-camera-live.md`
§2–§4.1 and `🗑️generated/w12c-parity-run-19/{parity.md,steps.json}` (17 match / 20 differ).

Three jobs: (1) the four world/dropdown rows of run 19, each with its React-side root cause and a law;
(2) the Wave 13 integration loop — rebuild and probe after these fixes and again once W13a's and
W13b's reports land; (3) the ingress latency of §4.1.

---

## 1. The four rows — root cause → fix

Every "React does X" below is read from React's own source, not inferred, and every "wgpu did Y" is a
row of `🗑️generated/w12c-parity-run-19/steps.json`.

### 1.1 `orbit-drag` — React's extra `noteShellCommand` is the window activation on the press INTO the pane

**Measured.** React seq 43: `noteShellCommand`, `windowId: null`, origin `user`, between the step's
`interactionHover` and its `interactionSelect` — i.e. on the press itself.

**React's site.** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:310` —

```tsx
onPointerDownCapture={(event) => { if (!isSurfaceActiveBackgroundPointer(event)) onActivate?.(); }}
```

a CAPTURE-phase handler on the whole window element, so it fires for the window's chrome and for the
scene canvas filling its body alike, before any inner handler runs. It reaches `Mode`'s
`activateWindow` (`🎨️Canvas/🟦️.tsx:1446`), whose `onActiveWindowChange` is the single funnel
`🏛️ShellHost/🟦️.tsx:10389`'s `handleActiveWindowChange` notes `shell.windowActivate` from.

**Root cause.** The wgpu shell has ELEVEN sites that write `active_window_id` — dock tab select, dock
tab focus action, palette `window:` row, keybinding owner activation, session switch, boot, … — and
**not one of them is a press into a window BODY**. W10b had already built the note
(`arm_window_activation_note`); nothing ever changed the active window for it to witness.

**Fix.**
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9776` `ShellState::activate_window_under_pointer(x, y)` — resolves
  the press against `dock_window_plan`, this frame's own solved window bodies (the same rects the dock
  paints and hit-tests), skips the empty-id row W12c §4.3 recorded and refuses while a modal layer is
  up (React's menus and dialogs are portaled OUTSIDE the window element, so no capture handler of
  theirs fires).
- `🧊️renderer/🦀️.rs:14718` — called at the head of `handle_pointer_button`'s press, BEFORE the
  chrome/surface routing, which is where a capture-phase handler sits.

### 1.2 `pan-drag` and `context-menu` — React selects on a NON-primary press

**Measured.** React journals ONE `interactionSelect` for `orbit-drag` (button 0) and TWO for both
`pan-drag` (button 1, seq 48/49) and `context-menu` (button 2, seq 62/63). wgpu journalled none for
the latter two.

**React's route.** Not a handler of its own: `🌐️World3dHost/🟦️.tsx:6884`'s `handlePointerDown` returns
immediately for `event.button !== 0`. What publishes is the `<Canvas>`'s
`onPointerMissed={handleEmptyClick}` (`:7345`), which R3F raises for a `pointerdown` that resolved no
event-handling object — for ANY button — and again on the click. `handleEmptyClick` (`:7068`)
publishes `interactionSelect` over the hovered target, or an empty selection when there is none. A
PRIMARY press arms the marquee, and `wasMarqueeDragRef` swallows the first of the two calls; a middle
or right press arms nothing, so both land. That is exactly the 1-vs-2 shape measured.

**Root cause, two halves.**
1. The world lane retired a middle press as a gesture-less intent and answered a right press with the
   click/drag discrimination only.
2. For the right button the press never even ARRIVED: W10a's `🧊️renderer/🦀️.rs` routed a plain
   secondary press over a world pane to the shell's `open_context_menu` and **returned**.

**Fix.**
- `♾️infinite/🌍️world/🦀️.rs:6681` `plan_world3d_non_primary_press_pick` — the same
  `WorldRayPickPurpose::Instance` cursor a primary click runs, whose miss arm publishes the empty
  selection React's own miss publishes. `None` outside the select lane (component mode, brush,
  vertex granularity own their own press verbs — React's `handleEmptyClick` returns early under
  `paintMode` for the same reason).
- `🌍️world/🦀️.rs:5886` (right press) and `:5899` (middle press) mint it instead of retiring.
- `🧊️renderer/🦀️.rs:14795` — the plain-secondary arm no longer returns: the shell menu opens **and**
  the press falls through into the world claim, which is the order the DOM delivers them.

ONE selection, not two: a second selection of the same target is a duplicate, not parity, and the
probe's verdict is set-based. Recorded in the fixture's `why`.

### 1.3 `example-picker-open` — the stray `interactionHover`

**Measured.** wgpu seq 52: `interactionHover` with `targets: "[]"` — a hover CLEAR — in a step that
only clicks the navbar's example button. React journals nothing.

**Root cause.** The clear was owed ten steps earlier. W12a made a chrome-owned pointer hand the world
lane one `PointerLeave`, but only on the next MOVE. The journey's `context-menu` step (23) opened a
modal menu and then ran six keyboard chords; the first pointer move after it is `example-picker-open`
(33), so the clear landed there. React has no such delay: its overlays take pointer events the instant
they mount, so r3f raises `onPointerOut` in the step that opened the layer — which is why React's
`context-menu` row ends with an `interactionHover` and its `example-picker-open` row is empty.

**Fix.** `🧊️renderer/🦀️.rs:12444` — in the `World3dAuthority` phase, while
`ShellState::pointer_input_is_modal()` holds (W12a's own predicate: menu, dialog, dropdown, tour), a
surface that still publishes a hover is handed exactly one `WorldInteractionIntent::pointer_leave` and
nothing else. Idempotent by construction: the clear retires the published hover, so the predicate
answers `false` on every later frame the layer stays open.

### 1.4 `dismiss-tour` / `panel-artifact` — wgpu's BOOT announcements landed inside the first two steps

**Measured.** wgpu's whole `registerBrushMesh` run dispatched from t=15.3 s to t=20.5 s
(`🗑️generated/w12c-parity-run-19/wgpu/console.txt`), while the probe's first user press (the tour
dismiss) was at t=14.5 s. React's run is over before the probe's boot read. The rows are the same
boot announcement on both renderers; only their timing differs.

**Root cause, measured not inferred.** The GLB became resident at t=12.2 s (`asset ready kind=Glb
/mesh/🧊️hexagonal-cut-concrete-forest-left.glb`), the first announce action was produced at t=13.9 s
(`frame deferred install carries an action`) and dispatched at t=15.3 s — the boot frame cadence is
~0.7 s per frame there, and W12b's back-pressure law is deliberately ONE page per surface per frame.
React's `BrushMeshRegistrar` mounts off `useLoader`'s promise during the first render instead.

**Fix (probe, renderer-neutral).** `🐍️parity-interact-probe.mjs`'s `boot()` now means READY **and**
QUIET: the beacon, a settled control census, and two consecutive seconds in which the action ledger's
newest `inputSeq` did not move. Both renderers are held to the same gate; React's ledger is already
quiet, so its boot is unchanged. Every step reads its own cursor at its start, so a row that lands
inside this wait is attributed to the boot it belongs to and to no step at all. This is the brief's
"at least before the first user step", reached without delaying `data-semio-os-ready` itself — the
beacon is raised by the worker's `booted` message, which also wires input, and gating it on asset
quiescence would block the page's input and every other probe and e2e runner that waits on it.

---

## 2. §4.1 — the ingress fold

**What §4.1 actually measures, re-measured.** The DOM→worker hop is NOT the latency: in run 19 the
eight synthesized moves of `orbit-drag` arrive at `os_host handle_event` at t=79784…79918 ms and are
dispatched at t=79789…79953 ms — a 5–35 ms lag. The TS transport already keeps one sample per pointer
identity per rAF (`🚚️browser-frame-transport/🟦️.ts`'s `enqueueReplaceable`) and the host's own
`EventQueue` coalesces into a `CoalesceSlot`.

**Where it really is.** The retained world authority. The same console line shows `queued=10` in front
of `puzzle3d-main-perspective` — the two hover moves, the press, all eight drag moves and the release,
each answered with its own full pick/plan turn, and the first two hover picks alone cost ~100 ms each
(`pending=1035`). React's own host has the matching rung and says so at
`🌐️World3dHost/🟦️.tsx:6300`: its hover dispatcher "keeps at most one outstanding and coalesces the
rest onto the latest target".

**Fix.** `♾️infinite/🌍️world/🦀️.rs:2954` `WorldInteractionIntentQueue::coalesce_tail_pointer_move` +
`:5626` `world3d_pointer_move_folds`, taken by `enqueue_world3d_event` (`:5638`). A fresh pointer MOVE
folds onto the one already waiting at the queue's tail: `dx`/`dy` accumulate, `x`/`y` take the newest
sample, and the `down`/button/modifier state must match.

**Why it cannot change gesture classification** — the one thing the brief forbids:

| term | why the fold is exact |
| --- | --- |
| `dx`/`dy` | both camera operations integrate their delta linearly — `OrbitController::pan` moves `target` along an orientation-only basis, `orbit` adds straight onto yaw/pitch (`🖱️ui/🎬️scene/📐️math/🦀️.rs:391`, `:399`) — so the summed delta steers the same camera |
| `x`/`y` | every pick reads the point, and the newest point is the one React would have raycast |
| the FRONT | never folded into: a live cursor may already be half-stepping it (`len < 2` refuses) |
| a lasso marquee | refused — the gesture IS its own path (`marquee_is_crossing_from_path`, the painted outline) |
| a live paint stroke | refused — it raycasts once per move, so a folded move would leave a gap in the run |
| a modifier / button / `down` change | refused — that is a different gesture, not the same one sampled again |

Measured by law rather than argued: the same eight-move middle drag is driven twice, folded and (on a
lasso pane, where the fold is refused by contract) whole, and the two journals, their order and the
settled camera are identical while the queue depth differs by exactly seven.

---

## 3. Laws

| law | file |
| --- | --- |
| `a_non_primary_press_publishes_the_selection_react_publishes_for_any_button` | `♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` |
| `a_non_primary_press_publishes_nothing_outside_the_select_lane` | idem |
| `a_burst_of_pointer_moves_folds_onto_the_latest_without_moving_the_gesture` | idem |
| `a_fold_never_touches_the_front_or_crosses_a_gesture_boundary` | idem |
| `a_press_inside_a_window_body_activates_that_window_and_notes_it` | `🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs` |
| `a_modal_layer_or_an_unnamed_dock_row_activates_no_window` | idem |
| `a_plain_secondary_press_over_a_pane_opens_the_menu_and_still_presses_the_surface` (source anchor) | idem |
| `a_modal_layer_opening_clears_every_published_hover_in_that_frame` (source anchor + oracle) | idem |

Oracle: `🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json` — `pan-drag` and `context-menu` gain the
selection with React's own reason, `orbit-drag`'s `why` names the window-activation note, and two new
steps carry the packet's own contracts: `modal-open` and `coalesced-drag`.

Updated to the new contract: `🌍️world/🧪️tests/🔬️unit/🦀️.rs`'s
`an_intent_aimed_outside_the_surface_retires_instead_of_faulting_the_frame` — a middle press no longer
retires silently, it publishes the selection; its subject (a gesture-less button never faults the
frame) is unchanged and still asserted.

---

## 4. Gates

(filled in below by the runs)

---

## 5. Integration loop — probe runs and tallies

(filled in below by the runs)
