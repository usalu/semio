# 🎯️ W12a — the chrome press that also pressed the world (family A)

Packet W12a. Input: `📓️w11a-prepared-world-mesh-missing.md` §6 **family A** and
`🗑️generated/w11a-parity-run-14/steps.json`. Eight journey steps pressed the shell's own chrome and
ALSO reached the world pane painted under it. Everything below is verified by native `cargo
check`/`cargo test` against the real input router and the real retained world authority; **live
confirmation is W12c's next probe run** — no wasm was built and no page was driven by this packet.

---

## 1. What the run measured

| step | React journal | wgpu journal (run 14) |
| --- | --- | --- |
| `panel-tool-runs` | `noteShellCommand` | `noteShellCommand`, **`interactionHover`**, **`interactionSelect`** |
| `panel-chat` | `noteShellCommand` | `noteShellCommand`, **`interactionSelect`** |
| `panel-chat-close` | — | `noteShellCommand` |
| `pane-chip-engagement-toggle` | `noteShellCommand`, `registerBrushMesh` ×6 | **`interactionHover`**, **`interactionSelect`** |
| `split-gutter-drag` | `setActiveExample`, `noteShellCommand` | `noteShellCommand`, **`interactionHover`**, **`interactionSelect`** |
| `window-cap-focus` / `window-cap-close` | (probe artefact, see `📓️w9c` §7) | `noteShellCommand` |
| `example-switch` | `setActiveExample`, `registerBrushMesh` ×59 | `setActiveExample`, **`interactionHover`**, **`interactionSelect`**, **`setCamera`** ×2 |

The stray `interactionSelect` rows carry `"targets":"[]"` — i.e. **pressing a navbar panel tab CLEARED
the world selection**, and every chrome click published a camera report. `pane-chip-engagement-toggle`
even hovered a real instance (`seed-left-001`): the chip at `+6.4,57.6` sits over it.

## 2. Root cause — routing by geometry where React routes by LAYER

The shell paints its chrome INSIDE a pane's rect. At the probe's viewport the dock plans
`puzzle3d-main-*@…+3,54`, while the top-right panel's tabs register at `+1380.9,57.6` and the pane
chips at `+6.4,57.6` — all inside the pane. The renderer's ingress routed a pointer to a world surface
on `state.bounds.contains(x, y)` alone:

- **the press** was handed to the shell only when `pointer_press_belongs_to_shell_chrome` claimed the
  hit, and then returned — but nothing recorded that the sequence now belonged to the chrome;
- **the release** (`handle_pointer_button`'s `!down` arm) called the shell and then enqueued a
  `pointer_button` intent into **every** surface containing the point, whatever the press had been.
  That release is what published `interactionSelect` (a left release under `select` runs
  `WorldRayPickPurpose::Instance`, whose miss arm publishes an empty selection) and armed
  `camera_sync.owed`, i.e. `setCamera`;
- **every move** was enqueued into every surface containing the point, so hovering a chip raycast
  through it and published `interactionHover`;
- a MODAL overlay (context menu, dropdown, dialog, tour) only owned the points its own items
  registered — the point beside an open menu fell straight through to the world.

React has none of these: its chrome is DOM elements above the `<canvas>`, so a click on chrome never
reaches the canvas, the element a `pointerdown` resolved to receives the whole sequence, and the
canvas's own hover is cleared by r3f's `onPointerOut` (`🌍️world/🎨️r3f/🟦️.tsx`'s
`dispatchInstanceHover`, `🌐️World3dHost/🟦️.tsx:6304`). Its overlays render into a layer with a
backdrop, and its resize handles call `setPointerCapture` — as three's `OrbitControls` does on the
canvas for the opposite direction.

## 3. Fix

### 3.1 One ownership model, replacing the press-only predicate

- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2615` `enum PointerHitOwner { Chrome, Surface }` — the two layers.
- `…:2636` `struct PointerCapture` — the DOM's implicit capture: `press(owner)` claims the sequence,
  `owner_of_move(under_pointer)` answers it while it runs, `release()` ends it (a release with no
  press is the surface's). Both directions: a chrome press keeps the whole gesture off the canvas, and
  a surface press keeps an orbit drag on the canvas where it sweeps over a panel.
- `…:9729` `pointer_hit_owner(hit)` — the former predicate's kind/namespace rule, now answering WHO
  rather than a bool. `pointer_press_belongs_to_shell_chrome` (`…:9719`) is one line over it, so every
  W9b/W9c/W12d law that asks it is unchanged.
- `…:9749` `pointer_input_is_modal()` — an open context menu, dialog, dropdown (`open_selects`) or
  tour owns EVERY pointer, wherever it lands.
- `…:9759` `pointer_is_over_open_panel(x, y, theme)` — an open anchored panel is an opaque LAYER, not
  a set of controls: its padding, the gaps between tree rows and the empty space under the last one
  register no hit and are all over the pane. Read from this frame's own `anchor_rect`.
- `…:9767` `pointer_owner_at(x, y, input, theme)` — modal, then panel box, then the topmost hit.
- `…:9627` `wheel_propagates_to_scene_surface` now refuses a chrome-owned hit outright, and
  `…:9668` `wheel_reaches_scene_surface(x, y, input, theme)` is the wheel's twin of the same question.

### 3.2 The renderer's ingress asks it once per phase

- `🧊️renderer/🦀️.rs:14714` the press/release head:
  `let owner = if down { self.pointer_capture.press(self.shell.pointer_owner_at(x, y, &self.input, &self.theme)) } else { self.pointer_capture.release() };`
  — a Chrome-owned press OR release goes to the shell and returns, so the world, graph, map and board
  lanes receive nothing. The old `if ShellState::pointer_press_belongs_to_shell_chrome(…)` guard below
  it is **deleted**, not left beside it.
- `🧊️renderer/🦀️.rs:11418` `AppInteractionState::pointer_capture` (plus the two other constructors,
  `🌐️browser-worker/🦀️.rs:654` and the async-boundary fixture).
- `🧊️renderer/🦀️.rs:14887` the move path: `chrome_owns_pointer` from the capture, and a surface the
  chrome owns (or that the pointer has left) is handed a **leave** instead of a move — only when it
  actually publishes a hover, so a pointer crossing chrome costs the bounded intent queue nothing once
  the hover is clear. Board surfaces get their existing `puzzle_board_pointer_leave_into`; graph and
  map get nothing.
- `🧊️renderer/🦀️.rs:12481` the wheel phase gate is `wheel_reaches_scene_surface`.

### 3.3 The world lane answers a leave with the hover clear, and nothing else

- `♾️infinite/🌍️world/🦀️.rs:2883` `WorldInteractionPhase::PointerLeave`; `…:2922`
  `WorldInteractionIntent::pointer_leave(x, y)`.
- `…:6527` `world3d_hover_is_published(state)` — what the renderer asks before it enqueues one.
- `…:6539` `plan_world3d_pointer_leave` — the clear in whichever lane this surface publishes:
  `setHover` with no args in component mode, `worldVortexHover` with a null `fullId` under the
  brush/vertex utilities, `interactionHover` with empty `targets` otherwise. `None` when nothing is
  published, so the leave is retired without a journal row.
- `…:5984` the authority's phase arm, `…:6060` a leave with nothing to clear retires (never faults,
  wherever it was aimed — a leave point is outside the pick rect by construction), and `…:5811` a
  leave does NOT move `camera_sync.at`, so a settle can never re-hover at a point over chrome.

**LAW (hit ownership):** a pointer belongs to exactly one layer per point and exactly one layer per
SEQUENCE. The engine surface under chrome receives no press, no move, no release and no wheel while
the chrome owns the pointer — only the clear of the hover it had published, which is exactly what
React's canvas receives.

## 4. Tests

| law | file |
| --- | --- |
| `every_chrome_press_of_the_journey_owns_its_whole_pointer_sequence` (all eight steps × the probe's move/press/8 moves/release) | `🐚️Shell/🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs` (new) |
| `a_press_on_the_surface_keeps_the_gesture_on_it_and_releases_it_again` | idem |
| `the_surface_body_and_the_empty_canvas_are_never_chrome` | idem |
| `an_open_overlay_owns_every_pointer_until_it_closes` (menu · dropdown · tour · wheel) | idem |
| `an_open_panels_whole_box_owns_the_pointer_over_the_pane_it_floats_on` | idem |
| `the_renderer_ingress_routes_every_pointer_phase_through_the_capture` (source anchors: press, move, leave, wheel, and that the replaced guard is gone) | idem |
| `a_chrome_owned_pointer_hands_the_world_lane_one_hover_clear_and_nothing_else` | `♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` |
| `a_leave_from_outside_the_pick_rect_clears_the_hover_without_faulting` | idem |

Oracle: `🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json` gains the `chrome-press` step, whose
`react` and `wgpu` journals are the SAME single `interactionHover`.

`🔬️wgpu-shell-chrome-parity/🦀️.rs:584`'s source anchor moved to the new single guard (its subject —
the renderer asks the shell before any surface may claim a press — is unchanged).

## 5. Gates

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ 0 errors | `🗑️generated/w12a-renderer-check.txt` |
| `cargo check -p … --lib --target wasm32-unknown-unknown -j 4` | ✅ 0 errors | `🗑️generated/w12a-wasm-check.txt` |
| `cargo test -p … --lib -- --test-threads=1 shell:: interpreter::` (shell input · pane chrome · panel anchor · chrome parity · navbar/footer · tour · introspection) | **429 passed, 1 failed** (§6) | `🗑️generated/w12a-renderer-tests.txt` |
| `cargo test -p semio-framework-os-infinite --lib -- --test-threads=1 world::` (journal sequences · pointer gestures · unit) | **177 passed, 1 failed** (§6) | `🗑️generated/w12a-infinite-world-tests.txt` |

All six new ownership laws and all nine `journal_sequence_tests` (W10a's seven plus this packet's two)
pass. `--skip host_panel_action_is_claimed_before_guest` as `📓️w9c` §9 requires — that test never
returns.

## 6. Failures NOT caused by this packet

1. `shell::window_pane_chrome_tests::each_pane_chip_dispatches_its_own_window_state` — a peer's live
   change: `toggle_window_pane_chip`'s `Search` arm no longer flips `search_open`/`overlay_state`
   (their own new docstring records why), and the test's `assert!(shell.search_open && …)` at
   `🪟️wgpu-window-pane-chrome/🦀️.rs:220` has not been updated with it. Confirmed against
   `git diff` of the shell target, not inferred. Owner: the Search-chip change (W12d's lane).
2. `world::tests::a_guest_restart_or_a_reupload_request_makes_every_surface_announce_again` — W12b's
   in-flight brush-mesh announce lane.
3. `shell::ui_prefs_themes_i18n_tests::file_prefs_store_round_trips_through_disk` — flaked once under
   the fleet with `WorkerPool: mandatory submission failed closed: Contended`; passes on its own.

Mid-packet the shared tree twice refused to compile on peers' half-landed edits (`WorldBrushMeshRun`
undefined, a `W12C_PROBE_SENTINEL2` type mismatch). Both cleared on their own.

## 7. What live confirmation must show (W12c's next probe)

1. `panel-tool-runs`, `panel-chat`, `pane-chip-engagement-toggle`, `split-gutter-drag`,
   `window-cap-focus`, `window-cap-close` and `example-switch` journal **no** `interactionHover`,
   `interactionSelect` or `setCamera` from the world controller.
2. `example-switch` journals `setActiveExample` alone (its two `setCamera` rows are the dropdown's two
   releases).
3. `pick-instance`, `orbit-drag`, `pan-drag`, `zoom-wheel` and `context-menu` are UNCHANGED — the
   press lands on the surface, so the capture is the surface's for the whole gesture.
4. A hover published over an instance clears with one `interactionHover` (`targets: []`) on the move
   that carries the pointer onto chrome, in the same step React clears its own.
5. A wheel notch over a panel, a pane chip or an open dropdown scrolls that chrome and does not move
   the pane's camera.

## 8. Known limit (measured, not fixed)

Ownership is decided by the modal state, the open panel boxes and the topmost hit's kind/namespace.
A chrome control painted over a pane that is NEITHER inside a panel box NOR in one of those
namespaces — a plugin-authored overlay button, say — would still fall through to the surface. Closing
that completely needs a LAYER tag on `HitTarget` (the ui crate's `register_hit`), which is a wider
change than this packet: `🎞️Scenes`/`⚙️EngineCanvas` register their own per-element targets as
`Generic`/`Input`, so an inversion ("anything that is not the surface's own region") would steal every
board, graph and table row press — the same trap `📓️w9b` §1 recorded rejecting.

## 9. Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs`
