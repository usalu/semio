# ⌨️🎥 W12c — why the unit-proven chord and camera routes did not fire on the live page

Packet W12c of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, families **C** (keyboard chords) and **D**
(camera/selection). The packet's premise was that wgpu's key ingress or its camera settle was wrong.
**Neither was.** Both families failed for reasons only a live page shows, and all four root causes are
recorded below with the console line that proves them.

Everything here is measured, never inferred: `🗑️generated/w12c-ladder-1`, `w12c-ladder-2`,
`w12c-bisect-world`, `w12c-parity-run-15` … `w12c-parity-run-19`.

---

## 0. The three things that were NOT the cause

| suspected | measured |
| --- | --- |
| the chord never reaches the page | every chord arrives. `🗑️generated/w11a-parity-run-14/wgpu/console.txt` carries one `os_host handle_event KeyDown` per chord — `z`+meta, `z`+meta+shift, `Escape` ×3, `k`+meta, `f`+meta+shift ×2, `1`/`2`+meta+alt — with the correct modifier state |
| the canvas is not focused / the listener is on the wrong target | `🚀️browser-boot/🟦️.ts:85` gives the canvas `tabIndex = 0` and `:349` focuses it on every `pointerdown`; the probe also focuses it at boot. `contentFocus=false` at every chord |
| the shell's chord ladder is broken | on a FRESH page every chord dispatches: `🗑️generated/w12c-ladder-1` — `fresh-escape → [engagementAbort]`, `fresh-undo → [undo]`, `fresh-redo → [redo]`, each with `[DEBUG] wgpu-shell key routing … contentFocus=false` |

The chord ladder is intact and DOM-equivalent. What killed it was **shell state the journey reaches**,
not ingress.

---

## 1. Family C — root cause: an empty dock turns every app chord into a hinted no-op

### 1.1 The measurement

A `[DEBUG] wgpu-shell chord gate` line was added at the exact point `handle_keyboard_async` computes
`idle`, naming every term the ladder branches on. Live, at every chord of the journey
(`🗑️generated/w12c-parity-run-17/wgpu/console.txt`):

```
chord gate action=Char("z") idle=true focused=None overlay=None syncCard=false dockDrag=false
           menu=false window=None session=Some("s.puzzle.puzzle3d@1/*#editor")
           editVerb=Some("undo") appBinding=Some("undo") reserved=false docked=0
```

Everything the packet suspected is green — `idle=true`, no focus, no overlay, no menu, the session is
live and BOTH the app binding and the framework edit verb resolve. The two red terms are
`window=None` and **`docked=0`**: the dock holds no window instances at all.

### 1.2 Why that is fatal, and where

`match_app_keybinding` resolves the app's own binding first (puzzle3d declares `escape → engagementAbort`
and `mod+z → undo`), and `dispatch_app_keybinding` then addresses it through
`resolve_keybinding_target_window_v1`. With no mounted window there is no target, so it raised the
`KEYBINDING_UNOWNED_CODE` banner and **returned `Ok(())`, ending the ladder** — the framework-universal
`mod+z`/`mod+shift+z` tail below it never ran. `[DEBUG] wgpu-shell edit chord` appears **0 times** in
the whole of run 15.

The dock is empty because the journey's `window-cap-focus` → `window-cap-close` steps close the wgpu
shell's two world panes, and `window-reopen` restores none. React never closed its own (§4.2).

### 1.3 Fix — an app binding that ANSWERED ends the ladder; one that could not, does not

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11712` `dispatch_app_keybinding` now returns `Result<bool, String>` —
`false` for the unowned-chord banner, `true` when it dispatched or opened the staged form — and
`🦀️.rs:11694`'s call site only returns early on `true`. This is React's own shape: `handleAppKeydown`'s
keybinding loop `continue`s past a binding it cannot resolve and its `mod+z`/`mod+shift+z`/`mod+y` tail
still fires.

**Live proof** (`🗑️generated/w12c-parity-run-18`): `chord-undo` and `chord-redo` went from 0 verbs to
1 each — `undo` and `redo` — with the dock still empty.

### 1.4 Fix — Escape closes a dismissable layer WITHOUT consuming the chord

`example-picker-dismiss` journalled nothing while React journals `engagementAbort`. With the navbar
picker open, `overlay_state = Dropdown("example")` made `idle` false, the app rung was skipped, and the
sync `handle_keyboard`'s dropdown arm closed the picker and returned.

- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11326` `ShellState::dismiss_dismissable_layers()` — closes the retained
  widget `Select`s and the navbar dropdown, and answers whether one was open.
- `🦀️.rs:11587` — called in `handle_keyboard_async` on Escape **before** `idle` is computed, then the
  chord carries on. Same law W10a gave the context menu's `CloseMenu`.
- `🦀️.rs:11360` — `handle_keyboard`'s two open-coded arms now delegate to the one helper.

The quick-search and find overlays are deliberately **not** dismissable layers: they are
focus-trapping dialogs with a focused query field, which is exactly React's `isEditableTarget` early
return, and `idle`'s `focused_id` term already reproduces it.

### 1.5 What family C still needs (NOT this packet's)

`chord-escape` and `context-menu-dismiss` dispatch `engagementAbort`, a **window-owned** verb. With an
empty dock React would be as silent as wgpu — the difference is that React's dock is never empty. The
remaining work is therefore the window-cap divergence in §4.2, not the keyboard.

---

## 2. Family D — root cause: the notch resolved a chrome scroll region, and the drag settles late

### 2.1 `zoom-wheel` — **fixed**

```
[DEBUG] wheel apply x=1116.0 y=411.0 delta=-360 hit=Some(ScrollRegion) control=Some("")
        propagates=false owed=false          (🗑️generated/w11a-parity-run-14/wgpu/console.txt:19476)
```

A window body registers a `HitKind::ScrollRegion` over its whole rect **before** the scene node inside
it registers its `HitKind::World3d`. On the frames where that scene row was missing, the topmost row at
the pane centre was a chrome scroll region with an EMPTY control id;
`scroll_region_is_scene_surface` only recognises ids ending `.pane`/`.map`, so
`wheel_reaches_scene_surface` answered `false` and `AppFrameTransactionPhase::WheelStart` returned
before the world ever saw the notch. React has no such question: a wheel is delivered to the DOM
element under the pointer, and that is the canvas.

**Fix.** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9632` `ShellState::scene_surface_contains(x, y)` — the wheel's
twin of the press path's own `state.bounds.contains(x, y)` claim — and `🦀️.rs:9659`
`wheel_reaches_scene_surface` now answers: *a point the shell's chrome does not own, inside a live scene
surface's rect, is that surface's*. It no longer depends on which non-chrome row is topmost or on how
its id is spelled. Chrome painted over the pane still takes the notch, which is the rule the fix must
not widen away (law in §5).

**Live proof:** `zoom-wheel` went from `[interactionHover]` to
`[interactionHover, noteWorldNavigation, setCamera]` — an exact match with React
(`🗑️generated/w12c-parity-run-17`).

### 2.2 `orbit-drag` / `pick-instance` — the settle was never missing, it was late

The settle ran correctly all along. In run 14 the world lane produced, 333 ms after the orbit release:

```
t=77119 frame input action … action=interactionSelect
t=77130 frame input action … action=setCamera         (console.txt:14716, :14724)
```

and the authority census shows the drained-queue settle plan running right after the marquee publish
(`active=Plan[0/1] → Plan[1/1] → Complete`, queue empty). The rows simply landed **after** the probe's
1600 ms post-step read and **before** the next step's cursor, so they were attributed to neither step —
visible as the seq gap 40…46 between `orbit-drag`'s last row (39) and `pan-drag`'s first (47) in
`🗑️generated/w11a-parity-run-14/steps.json`.

The cause is ingress latency, not routing: the probe's 8 synthesized `pointermove`s are unthrottled
round trips through the frame worker, so the worker is ~1 s behind Playwright by the release.

**Fix (probe, renderer-neutral).** The five scene gestures now settle 4000 ms instead of 1600 ms on
**both** renderers (`🐍️parity-interact-probe.mjs`'s journey). React's own `CAMERA_SYNC_DEBOUNCE_MS` is
**120 ms** (`📐️Canvas2dHost/🟦️.tsx:732`), so its rows land inside either window and its column is
unchanged; wgpu's fit. The reason is written at the journey entry. The latency itself is reported as a
finding, not hidden — see §4.1.

**Live proof:** `pick-instance` matches React exactly
(`interactionHover, interactionSelect, setCamera`), and `orbit-drag` now carries its `setCamera`.

### 2.3 What family D still needs (NOT this packet's)

| step | React | wgpu | remaining |
| --- | --- | --- | --- |
| `orbit-drag` | + `noteShellCommand` | — | W10b's `shell.windowActivate` note fires on a real in-session active-window CHANGE; the probe's press does not change it |
| `pan-drag`, `context-menu` | + `interactionSelect` | — | r3f's instance mesh `onPointerDown` fires for ANY button, so React selects on a middle press and on a right press. W10a recorded this as a deliberate divergence in `📜️journal-sequences.json`; replicating it means publishing a selection on a non-primary press, and for the right button it also means letting the press reach the surface as well as the shell menu |

---

## 3. P0 found and fixed on the way: the page died on the first pointer move over a pane

Both my first two probe runs went silent within 25 s:

```
world3d interaction surface=puzzle3d-main-perspective leave g=1 step=Fault … registry=true faulted=true
frame fault recorded: world3d retained interaction authority faulted
page error wgpu renderer fault: worker-frame-failed: world3d retained interaction authority faulted
                                        (🗑️generated/w12c-bisect-world/wgpu/console.txt)
```

`WorldInteractionRegistryBuildCursor::step` and `WorldRayPickCursor::step` each answered an EMPTY probe
slot — "this mesh key was never published" — with `WorldInteractionStep::Fault`. The guest publishes
its draw list and its mesh leases in separate deliveries, so every glb-bearing document has frames
where a draw names a lease still landing. Reproduced at t≈9.4 s on the first pointer move after the
introduction tour was dismissed, which killed the page before a single journey step could be measured.
(Run 14 survived only because its 17 chrome steps warmed the leases first.)

**Fix — a draw whose mesh has not landed is SKIPPED, never a fault:**

- `♾️infinite/🌍️world/🦀️.rs:2772` registry cursor — the `None` arm advances `self.draw`.
- `♾️infinite/🌍️world/🦀️.rs:4247` ray-pick cursor — the `None` arm and the `state.meshes.get` miss both
  call the new `WorldRayPickCursor::skip_draw` (`🦀️.rs:4244`).

This is the interaction twin of W11a's "a miss is a skipped draw, not a quarantine", and it is React's
own behaviour: a `GlbInstanceMesh` whose loader has not resolved renders nothing and raycasts to
nothing.

---

## 4. Findings handed on

### 4.1 wgpu ingress latency on a synthesized drag

The worker is ~1 s behind Playwright by the end of an 8-move drag (release logged at t=76797 ms for a
gesture Playwright finished well before). Every `pointermove` is a full round trip. This is the
`📓️project-hover-latency-anatomy-and-fixes` family and it is what forced §2.2's wider settle window.

### 4.2 `window-cap-close` closes two windows on wgpu and none on React

`window-cap-focus`/`-close` resolve `contains` on wgpu (`dock.tab.0.puzzle3d-main-top.close`) and close
real windows; React's cap chips only exist under the pointer, so the step used to resolve `absent`
there. **Probe fix applied:** `clickWindowCap` now hovers the dock tab first, so both renderers are
offered the same control (`🐍️parity-interact-probe.mjs`). It made React's cap steps resolve, but the
wgpu dock still ends at `docked=0` while React's does not — which is what still silences
`chord-escape`/`context-menu-dismiss`. Owner: whoever owns the dock/cap lane.

### 4.3 A `HitKind::ScrollRegion` with an EMPTY control id

After the cap-close steps the shell's own dock window plan carries a window with an empty id
(`wgpu-shell engine surfaces … windows=["", "tool.fill", …]`), which is the row that was swallowing the
wheel. §2.1's fix makes the wheel immune to it; the empty-id window itself is a dock bug.

---

## 5. Tests

| law | file |
| --- | --- |
| `a_wheel_over_a_live_pane_reaches_the_scene_under_any_non_chrome_row` | `🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs` |
| `a_wheel_over_chrome_painted_inside_the_pane_never_reaches_the_scene` | idem |
| `a_wheel_outside_every_scene_rect_stays_with_the_chrome_that_owns_it` | idem |
| `escape_dismisses_every_open_dismissable_layer_at_once` | idem |
| `the_palette_and_find_overlays_are_not_dismissable_layers` | idem |
| `escape_under_an_open_picker_still_reaches_the_app_keybinding_rung` | idem |
| `an_unowned_app_chord_falls_through_to_the_framework_edit_verbs` | idem |
| `a_draw_whose_mesh_has_not_landed_is_skipped_by_every_interaction_cursor` | `♾️infinite/🌍️world/🧪️tests/📜️journal-sequences/🦀️.rs` |

**Results:** shell laws **7 passed, 0 failed**; the world law **1 passed**. Gates:
`cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` clean (62 warnings — proof of expansion),
`cargo check -p semio-framework-os-infinite --lib -j 4` clean (6 warnings),
`🗑️generated/w12c-renderer-check.txt`, `🗑️generated/w12c-infinite-check.txt`.

Four wasm builds + activations, all exit 0: `🗑️generated/w12c-wasm-build-{2,3,4,5}.txt`. Build 1 failed
on a peer's in-flight `AppInteractionState::pointer_capture` missing from the wasm-only
`🌐️browser-worker` initializer — native `cargo check` cannot see that file
(`cfg(target_arch = "wasm32")`); W12a landed the field a minute later.

---

## 6. Probe additions

- `🐍️w12c-chord-ingress-probe.mjs` — the chord LIVENESS ladder: one `mod+z` after every chrome gesture
  of the journey, so the exact gesture after which chords die is measured rather than guessed. This is
  what proved the ladder intact across all chrome and narrowed the cause to the cap-close steps.
- `🐍️w12c-hit-census.mjs` — dumps the live hit registry's `ScrollRegion`/`World3d`/`Window` rows.
- `🐍️w12c-react-boot-check.mjs` — reports React's `data-semio-os-ready` second by second plus every
  console error; the 6313 serve wedged twice during this packet and had to be recycled by pid
  (`📓️project-release-serve-wedges-after-host-edit-bursts`).

---

## 7. Final per-step tally

`🗑️generated/w12c-parity-run-19/` — both renderers, 37 steps, wgpu build 5 (which carries W12a, W12b
and W12d), React at 6313 freshly recycled. **17 match / 20 differ**, against run 14's **12 / 25**.
**Zero `renderer fault`, zero `worker-*-failed`, zero `authority faulted` in the whole run.**

| # | step | React | wgpu | verdict |
| --- | --- | --- | --- | --- |
| 01 | `boot` | — | — | ✅ |
| 02 | `dismiss-tour` | — | `registerBrushMesh` | ❌ W12b |
| 03 | `panel-artifact` | `noteShellCommand` | `noteShellCommand`, `registerBrushMesh` | ❌ W12b |
| 04 | `panel-catalogue` | `noteShellCommand` | `noteShellCommand` | ✅ |
| 05 | `panel-inspection` | `noteShellCommand` | `noteShellCommand` | ✅ |
| 06 | `panel-tool-runs` | `noteShellCommand` | `noteShellCommand` | ✅ **W12a** (was family A) |
| 07 | `panel-chat` | `noteShellCommand` | `noteShellCommand` | ✅ **W12a** |
| 08 | `panel-chat-close` | `noteShellCommand` | `noteShellCommand` | ✅ **W12a** |
| 09 | `pane-chip-engagement-toggle` | `addObjectKind` | `noteShellCommand` | ❌ engagement lane |
| 10 | `pane-chip-search-toggle` | `addObjectKind` | — | ❌ engagement lane |
| 11 | `pane-chip-windowcontrols` | — | — | ✅ |
| 12 | `pane-chip-utilitybar-unfold` | — | — | ✅ |
| 13 | `pane-chip-pane-fold` | — | — | ✅ **W12d** (respelled key) |
| 14 | `pane-chip-measures-unfold` | — | — | ✅ **W12d** |
| 15 | `split-gutter-drag` | `noteShellCommand`, `registerBrushMesh` | `noteShellCommand` | ❌ W12b |
| 16 | `window-cap-focus` | — | `noteShellCommand` | ❌ §4.2 |
| 17 | `window-cap-close` | — | `noteShellCommand` | ❌ §4.2 |
| 18 | `window-reopen` | — | `noteShellCommand` | ❌ §4.2 |
| 19 | `orbit-drag` | Hover, Select, **noteShellCommand**, setCamera | Hover, Select, setCamera | ❌ **D**, `setCamera` recovered (§2.2) |
| 20 | `pan-drag` | Hover, **Select**, noteWorldNavigation, setCamera | Hover, noteWorldNavigation, setCamera | ❌ **D** (§2.3) |
| 21 | `zoom-wheel` | Hover, noteWorldNavigation, setCamera | Hover, noteWorldNavigation, setCamera | ✅ **D fixed** (§2.1) |
| 22 | `pick-instance` | Hover, Select, setCamera | Hover, Select, setCamera | ✅ **D** (§2.2) |
| 23 | `context-menu` | Hover, **Select**, setCamera | Hover, setCamera | ❌ **D** (§2.3) |
| 24 | `context-menu-dismiss` | `engagementAbort` | — | ❌ **C**, blocked on §4.2 |
| 25 | `chord-command-palette` | — | — | ✅ |
| 26 | `chord-escape` | `engagementAbort` | — | ❌ **C**, blocked on §4.2 |
| 27 | `chord-undo` | `undo`, `shell.windowActivate` | **`undo`** | ❌ **C**, verb recovered (§1.3) |
| 28 | `chord-redo` | `redo`, `shell.windowActivate` | **`redo`** | ❌ **C**, verb recovered (§1.3) |
| 29 | `chord-fullscreen` | — | — | ✅ |
| 30 | `chord-fullscreen-exit` | — | — | ✅ |
| 31 | `chord-panel-anchor-left` | — | — | ✅ |
| 32 | `chord-panel-anchor-right` | — | — | ✅ |
| 33 | `example-picker-open` | — | `interactionHover` | ❌ pointer leak, one row |
| 34 | `example-switch` | `setActiveExample`, `registerBrushMesh` | `setActiveExample` | ❌ W12b |
| 35 | `example-picker-dismiss` | `engagementAbort`, `registerBrushMesh` | — | ❌ **C**, blocked on §4.2 |
| 36 | `role-viewer` | `setActiveExample`, `registerBrushMesh` | `setActiveExample` | ❌ W12b |
| 37 | `role-editor` | `setActiveExample`, `noteShellCommand`, `registerBrushMesh` | `setActiveExample` | ❌ W12b |

### What this packet moved

| step | run 14 | run 19 |
| --- | --- | --- |
| `zoom-wheel` | `interactionHover` | **matches React exactly** |
| `pick-instance` | matched, but the settle trailed into the next step | matches, in window |
| `orbit-drag` | Hover, Hover, Select | Hover, Select, **setCamera** |
| `chord-undo` | (none) | **`undo`** |
| `chord-redo` | (none) | **`redo`** |
| every world gesture | the page could die on the first pointer move | 0 faults in 37 steps |

### The three remaining C rows, in one sentence

`chord-escape`, `context-menu-dismiss` and `example-picker-dismiss` all dispatch `engagementAbort`, a
**window-owned** verb, and the wgpu dock is at `docked=0` from step 17 on — §4.2. The keyboard ladder
itself resolves the binding at every one of them (`appBinding=Some("engagementAbort")` in the gate
line); nothing further is owed by the keyboard lane.

### Regression check on the peers' work

`🗑️generated/w12c-parity-run-19` is the first run carrying W12a, W12b and W12d. None regressed the
tally and three lanes improved inside it: the chrome-press leak is gone from `panel-tool-runs`,
`panel-chat` and `panel-chat-close` (W12a), the two respelled pane chips resolve on both renderers
(W12d), and `registerBrushMesh` now appears on the wgpu side (W12b — it is now the LEADING remaining
family, 6 steps, because the wgpu echo count and placement still differ from React's).
