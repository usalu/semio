# 🌍️ W10a — World/engine-canvas pointer, wheel, pick and context-menu journal parity

The wgpu World3d pane now journals the same action ids, args and ORDER React does for hover, pick,
orbit/pan/zoom and right-click, and the chrome ledger records them with the provenance React's own
input ledger records. Everything below is measured against the reference's per-step journals in
`🗑️generated/parity-run-2/steps.json` (`orbit-drag`, `pan-drag`, `zoom-wheel`, `pick-instance`,
`context-menu`, `context-menu-dismiss`), read through `📓️w5b`'s probe.

**Live confirmation is W9c's next parity probe run.** Everything here is verified by native `cargo
check` + unit laws that drive the real retained authority; no wasm was rebuilt and no page was driven
by this packet.

---

## 1. What React actually does (the reference, re-derived from source)

Two mechanisms in `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`'s `WorldOrbitGated` explain every row of the
reference journal, and neither had a twin in the wgpu target.

| reference | where | contract |
| --- | --- | --- |
| mouse map | `resolveWorldOrbitMouseButtonsIdle` :3282 | `LEFT: null`, `MIDDLE: PAN`, `RIGHT: null`; `resolveWorldOrbitRightMouseAction` :3299 makes Shift+right pan and Alt+right orbit |
| camera report | `WorldOrbitGated.onEnd` :3598 → `reportCamera` :3563 | fires on **every** pointer-up — three's `OrbitControls.onPointerUp` dispatches `_endEvent` unconditionally (`node_modules/three/examples/jsm/controls/OrbitControls.js:1515`), whatever the button and whether or not the camera moved |
| debounce | `World3dHost/🟦️.tsx:5843` `dispatchWorldCameraDebounced` | trailing `CAMERA_SYNC_DEBOUNCE_MS`, so one gesture (or one wheel burst) publishes **one** `setCamera` |
| navigation note | `onEnd`'s `classifyWorldNavigationGestures` :3605 → `World3dHost/🟦️.tsx:7391` | only when a nav state actually STARTED (`_startEvent`) **and** the before/after diff classifies non-empty; un-debounced, so it lands **before** the camera report |
| menu | `World3dHost/🟦️.tsx:7293` `onContextMenu` → `openSurfaceContextMenu` | opens the shell menu with `world3dContextMenuSurfaceV1`'s `hits`/`selection`; **dispatches no verb** — `world3dContextMenuSurfaceV1`'s own docstring (:1910) records that `contextMenuAt` "no longer exists in any world-3d app … dispatching it anyway only produced an `undeclaredActionDiagnostic` drop on every right-click" |
| hover | r3f `onPointerOver`/`onPointerOut` via `dispatchInstanceHover` :6304 | publishes on a CHANGE of target, not per move |

That map reads the reference journal exactly: `orbit-drag` (left) gets a `setCamera` but **no**
`noteWorldNavigation` because the left button drives no camera; `pan-drag`/`zoom-wheel` get both;
`pick-instance`/`context-menu` get a `setCamera` with no camera movement at all.

---

## 2. Per-step root cause → fix

All file paths below are absolute-from-repo-root.

### 2.1 `orbit-drag`, `pan-drag`, `zoom-wheel` — one `setCamera` per pointer MOVE, never debounced

**Root cause.** `WorldInteractionAuthority::step`'s `PointerMove`/`PointerDrag` arms called
`plan_world3d_drag`, whose plan was a `WorldFlatActionKind::Camera` that RESERVED and PUBLISHED a
`setCamera` on every step (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`, old
`Camera` arm of `publish_world3d_plan_step`). The probe's drag is eight interpolated moves, so one
pan published eight reports where React publishes one. A wheel burst published one per tick.

**Fix.** Split the camera into "apply locally, report once at the settle":

- `WorldFlatActionKind::CameraLocal` (`🌍️world/🦀️.rs:2948`) moves the orbit and publishes nothing;
  `plan_world3d_drag` :6518 and `plan_world3d_wheel` :6492 now mint it.
- `WorldCameraSync` (`🌍️world/🦀️.rs:1871`, region `🧭️WorldCameraSync`) carries the debt: `owed`,
  `navigating`, the `start` snapshot, `hover_owed`, and the last pointer point `at`. The full
  reference contract, including three's unconditional `_endEvent`, is documented on that struct.
- `plan_world3d_camera_settle` (`🌍️world/🦀️.rs:6446`) builds the settle plan — `noteWorldNavigation`
  first, then `setCamera` — and `WorldInteractionAuthority::step`'s drained-queue arm
  (`🌍️world/🦀️.rs:5836`) runs it. **A drained intent queue is the trailing debounce**: it is exactly
  when React's timer would fire, and a burst coalesces by construction, with no timer in a lane that
  owns no clock.
- `world3d_interaction_front_generation` (`🌍️world/🦀️.rs:5560`) now answers the last minted
  generation while a settle is owed — without that the frame driver
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12396`)
  skips a surface whose queue has drained and the settle would never be stepped.

**React ref.** `dispatchWorldCameraDebounced` (`World3dHost/🟦️.tsx:5843`).

### 2.2 `noteWorldNavigation` did not exist in the target at all

**Root cause.** Zero occurrences of the verb anywhere in `♾️infinite` or `📺️renderer`.

**Fix.** `classify_world_navigation_gestures` (`🌍️world/🦀️.rs:1966`) is a verbatim port of React's
`classifyWorldNavigationGestures` — same thresholds (`panRatio 0.02`, `zoomRatio 0.03`,
`orbitRadians 0.05`, `🌍️world/🦀️.rs:1929`), same orthographic/perspective zoom branch, same
`pan, zoom, orbit` push order (`WORLD_NAVIGATION_GESTURE_IDS`). `WorldFlatActionKind::Navigation`
(`🌍️world/🦀️.rs:6665`) publishes `{windowId, gestures: [...]}`, the shape
`ShellHost/🟦️.tsx:6443` intercepts. The snapshot is captured on the FIRST navigation step of a
gesture (the `CameraLocal` arm, `🌍️world/🦀️.rs:6639`), which is React's `onStart`.

**React ref.** `WorldOrbitGated.onEnd` :3605, `World3dHost/🟦️.tsx:7391`,
`ShellHelpers/🟦️.tsx:242` (`NOTE_WORLD_NAVIGATION_ACTION_ID`).

### 2.3 `pick-instance` / `context-menu` — a gesture that moved no camera reported none

**Root cause.** The production authority had **no** release-time camera sync at all. (A `#[cfg(test)]`
mirror, `handle_world3d_pointer_button`, did have one — dead legacy that only the unit tests drove.)

**Fix.** Every pointer RELEASE arms the debt, at the head of the intent handling
(`🌍️world/🦀️.rs:5855`), which is three's unconditional `_endEvent`. The `cfg(test)` mirror's
right-button arm was brought to the same law (`🌍️world/🦀️.rs:11775`).

### 2.4 `context-menu` — the target dispatched a dead verb and opened no menu

**Two root causes, both fixed.**

1. `WorldContextMenuCursor` published `worldContextMenuAt` — a verb with **exactly one mention in the
   whole repository** (its own emission site) and no consumer, i.e. the twin React deleted. The
   cursor, its `WorldContextTargetKind`, the `WorldInteractionActive::ContextMenu` arm and the
   `WorldFlatActionKind::ContextMenu` plan arm are all gone; the right RELEASE now only closes the
   click/drag discrimination (`🌍️world/🦀️.rs:5825`, with the whole reasoning in place).
2. The shell's own `open_context_menu` — whose `resolve_context_menu_surface`
   (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10801`) **already** fills a World3d surface's `hits`/`selection`
   from `world3d_context_menu_surface` (W2b's work) — was never reached: a `HitKind::World3d` target
   is not chrome, so `handle_pointer_button`'s down path handed the press to the surface and returned.
   The renderer now routes a PLAIN secondary press over a world pane to the shell
   (`🧊️renderer/🦀️.rs:14728`), and keeps Shift/Alt/Meta+right on the surface where React's
   `resolveWorldOrbitRightMouseAction` puts pan and orbit.

**React ref.** `world3dContextMenuSurfaceV1` (`World3dHost/🟦️.tsx:1910`), `onContextMenu` :7293.

### 2.5 `context-menu-dismiss` — Escape closed the menu and swallowed `engagementAbort`

**Root cause.** `ShellState::handle_keyboard_async`'s context-menu arm returned `Ok(())` on
`ContextMenuKeyOutcome::CloseMenu`, so the app-declared keybinding ladder below it never ran.
`engagementAbort` is an app action bound to Escape in the plugin manifests
(`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`); React's menu is a DOM popover, so the same keydown closes it
**and** reaches `handleAppKeydown`.

**Fix.** `CloseMenu` now falls through (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11411`); `Consumed` (the
menu's own arrow/Enter navigation) still consumes. This is a three-line change inside the
context-menu arm only — it does not touch W10b's chord table.

### 2.6 The ledger recorded no `origin`, and surface-addressed rows named no window

**Root cause.** `DumpDispatchedAction` had no `origin` column, so `🐍️parity-interact-probe.mjs:308`
stamped every wgpu row `"shell"` by hand while React's rows carry a real `InputOriginV1`. Its
`windowId` was lifted from the `windowId` arg only, so a `surfaceId`-addressed row answered `null`.

**Fix.**
- `DumpDispatchedAction.origin` (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2638`), and `window_id` now
  falls back to `surfaceId` (`:2792`).
- `note_dispatched_action(action, origin)` (`:2779`) takes the provenance.
- `ShellState::dispatch_gesture_action` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7534`) is the `gesture`
  door; `SHELL_DISPATCH_ORIGIN_USER` / `_GESTURE` (`:747`) are React's own two values.
- The frame's deferred surface-action lane — every action an engine surface's bounded input authority
  produced — dispatches through it (`🧊️renderer/🦀️.rs:9906`). Every other dispatch stays `user`.

**React ref.** `InputOriginV1` and `InputLedgerRecordV1`
(`🏛️ShellHost/🎯️input-ledger/🟦️.ts:30`, `:163`).

**One line left for W9c** (the probe is W9c's, so this packet did not edit it):
`🐍️parity-interact-probe.mjs:308` should read `origin: entry.origin` instead of `origin: "shell"`.

---

## 3. Measured sequences, and where the two renderers still differ

`🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json` is the new shared oracle: per step, React's
MEASURED journal (from run-2), the sequence this lane answers, and a `why` for every difference.

| step | React (measured) | wgpu (now) | difference |
| --- | --- | --- | --- |
| `orbit-drag` | Hover, noteShellCommand, Select, setCamera, Hover | Hover, Select, setCamera, Hover | `noteShellCommand` is the shell chrome's own row for the step, not the world surface's |
| `pan-drag` | Hover, Select, Select, noteWorldNavigation, setCamera, Hover | Hover, noteWorldNavigation, setCamera, Hover | React's two `interactionSelect` are an r3f artefact: the instance mesh's `onPointerDown` fires for ANY button and `onPointerMissed` fires on the release. A middle drag is a camera gesture and selects nothing |
| `zoom-wheel` | Hover, noteWorldNavigation, setCamera, Hover | Hover, noteWorldNavigation, setCamera | identical; the trailing hover needs the moved camera to change what is under the pointer, which a one-instance law scene cannot do (a dolly keeps the pointer on its own target) |
| `pick-instance` | Hover, Select, Select, setCamera, Hover | Hover, Select, setCamera | one pick is one selection; the second `Select` is the same r3f down/missed pair |
| `context-menu` | Select, Select, setCamera, Hover | Hover, setCamera | neither renderer publishes a menu verb; the leading hover is the law scene's first — in the probe's journey the preceding step already published it, and both renderers dedupe on the published target (r3f's `onPointerOver`, this lane's `local_hover_id` gate at `🌍️world/🦀️.rs:4350`) |
| `context-menu-dismiss` | engagementAbort | engagementAbort | §2.5 |

The two select-multiplicity rows are **deliberately not replicated**: they are a DOM-raycast
duplicate on React's side, and journaling a second selection of the same target would be a defect in
this lane, not parity. They are recorded in the fixture's `why` so the probe's set-based verdict
stays green and the difference stays visible.

### Hover, precisely

Both renderers publish a hover on a CHANGE of target. The settle now re-publishes one
(`🌍️world/🦀️.rs:5843`) because a moved camera changes what the (stationary) pointer is over — this
lane's twin of r3f re-raycasting the moved scene. When nothing changed, the existing
`local_hover_id` gate drops it, which is exactly why React journals no leading hover on
`context-menu`.

---

## 4. Orthographic Top pane

Checked and covered by a law. Pan, wheel and pick are projection-agnostic all the way down:
`OrbitController::pan` scales by `1 / zoom` under the parallel frustum and `zoom` scales the frustum
rather than dollying (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs:395`, `:409`),
`world3d_camera_zoom` reports that live scale while a perspective pane reports the identity
(`🌍️world/🦀️.rs:9653`), and `classify_world_navigation_gestures` takes React's own orthographic
branch (`after.zoom / before.zoom`) rather than a dolly ratio. `an_orthographic_pane_journals_the_same_sequences_with_a_live_zoom`
drives all three on a `CameraProjection3d::Orthographic` pane and asserts the wheel actually moves the
frustum scale the report carries.

---

## 5. Tests

New: `🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` (module wired at `🌍️world/🦀️.rs:13616`), seven laws
that drive the **real** authority — `enqueue_world3d_event` → `step_world3d_interaction` → the bounded
action queue — with the probe's own gestures (a drag is one move onto the aim point, a press, eight
interpolated moves and a release; a wheel is one move and one `-360` tick; a click is one move, a
press and a release at the same point), asserting the ORDERED journal against the fixture:

- `an_orbit_drag_journals_the_reference_sequence_and_moves_no_camera`
- `a_pan_drag_journals_one_navigation_note_and_one_debounced_camera_report` (asserts exactly ONE
  `setCamera` for eight moves — the regression this packet closes)
- `a_wheel_zoom_journals_exactly_the_reference_sequence` (also compared against React's own row)
- `a_pick_journals_one_selection_and_the_unconditional_camera_report`
- `a_right_click_journals_no_menu_verb_at_all`
- `the_navigation_classifier_answers_reacts_own_thresholds_and_order`
- `an_orthographic_pane_journals_the_same_sequences_with_a_live_zoom`

Updated to the new contract:
- `🌍️world/🧪️tests/🔬️unit/🦀️.rs` — `world_wheel_plan_revalidates_before_mutation_and_publishes_flat_action`
  and `world_drag_and_paint_plans_reserve_before_exact_mutation` (a navigation step publishes nothing;
  the settle publishes `noteWorldNavigation` then `setCamera`);
  `right_click_publishes_no_menu_verb_and_still_reports_the_camera` and
  `right_drag_reports_the_camera_like_every_other_release` (replacing the two `contextMenuAt` laws);
  `a_right_gesture_publishes_nothing_while_it_runs` (replacing the cursor law).
  `world_authority_retains_front_plan_across_output_saturation_and_retries_in_order` and
  `world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer` now drive to the observed
  condition in a bounded loop instead of counting turns — both were already failing on turn
  arithmetic (a moved camera re-dirties the object registry, whose rebuild is itself several turns)
  and now pass.
- `🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs` — `a_camera_gesture_addresses_the_window_that_owns_the_surface`
  now asserts the settle's `["noteWorldNavigation", "setCamera"]` order before checking the
  `{windowId, camera{position,target,zoom,up}}` payload it already pinned.
- `🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` — the ledger law asserts `origin` for both a
  `user` press and a `gesture` row, and that a `surfaceId`-addressed row still names its window.
- `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs:583` — W9c's source-anchor law greps the
  renderer for the world claim that follows the chrome guard; its anchor string moved with §2.4's
  routing change and was updated in place (the law's subject is unchanged and it passes).
- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — `World3dState` grew by
  `WorldCameraSync`, so the committed `AdmittedSurfaceMap<World3dState>` budget moved
  22608→22752 element bytes, 48320→48608 owner bytes.

---

## 6. Gates run

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-infinite --lib -j 4` | ✅ clean (6 pre-existing warnings, none new) | `🗑️generated/w10a-infinite-check.txt` |
| `cargo test -p semio-framework-os-infinite --lib world:: -j 4 -- --test-threads=1` | **165 passed, 4 failed** — all four pre-existing and outside this lane (`live_renderer_retains_generation_wake…`, `world_component_marquee_cursor…`, `world_component_marquee_publish…`, `world_object_registry_enforces_capacity_revision_and_aba`). The packet expected 6 pre-existing; the two saturation laws above are now fixed | `🗑️generated/w10a-infinite-world-tests.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ clean | `🗑️generated/w10a-renderer-check.txt` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1` | **879 passed, 8 failed**, all pre-existing and in lanes this packet never touches (tour overlay silhouette, presenter/raster pump ×2, glass pass, board/graph wheel descriptor sync, directory bootstrap, tool-run panel ×2) | `🗑️generated/w10a-renderer-tests.txt` |
| `cargo test -p … --lib engine_canvas:: / scenes:: / interpreter::` | 58/1·143/0·50/0 — the one `engine_canvas` failure is the pre-existing board descriptor-sync law | (in the full run above) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | ✅ clean | `🗑️generated/w10a-wasm-check.txt` |

---

## 7. What live confirmation must show (W9c's next probe)

1. `pan-drag` and `zoom-wheel` journal `noteWorldNavigation` then exactly one `setCamera` — not one
   per move.
2. `pick-instance` and `context-menu` journal a `setCamera` although no camera moved.
3. `orbit-drag` journals `setCamera` and **no** `noteWorldNavigation`.
4. A right-click over the pane opens the shell's world menu (React's `hits`/`selection`) and journals
   no `worldContextMenuAt`.
5. `context-menu-dismiss` journals `engagementAbort`.
6. `dumpChrome().actions` rows carry `origin: "gesture"` for all of the above and `"user"` for the
   navbar/panel steps — after `🐍️parity-interact-probe.mjs:308` reads `entry.origin`.

## 8. Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🖱️pointer-gestures/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs`
- `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`
