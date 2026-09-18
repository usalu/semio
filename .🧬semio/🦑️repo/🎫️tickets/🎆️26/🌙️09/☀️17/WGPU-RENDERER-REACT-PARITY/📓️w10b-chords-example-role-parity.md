# ⌨️📚️👁️ W10b — chord dispatch, example picker and role switch parity

Packet W10b of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Lane: the wgpu Shell's keyboard chord
dispatch, the navbar example picker, the role switch, and the action journal for those steps.

Ground truth: `🗑️generated/parity-run-2/steps.json` (React at 6313, 2026-09-18), reproduced row for row
by runs 5, 7 and 8 — so every React expectation below is measured three times over, not inferred.
The probe is `🐍️parity-interact-probe.mjs`; a step MATCHES when both renderers dispatch the same SET of
action ids and move the same surfaces (counts, rects and timings are out of the verdict by design).

All wgpu edits are in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
(line numbers as of this write; the file is under concurrent edit by W9c and W10a).

---

## 1. What React actually does — the measured journals

| step | chord/click | React actions | React surface delta |
| --- | --- | --- | --- |
| `chord-command-palette` | `Meta+k` | **(none)** | **none** |
| `chord-escape` | `Escape` | `engagementAbort` ×1 (`s.puzzle.puzzle3d@1/*#editor`, `puzzle3d-main-perspective`, origin `user`, `applied`) | none |
| `chord-undo` | `Meta+z` | `undo` (origin `user`, `applied`) **+ `shell.windowActivate` (origin `guest`, `refused: undeclared-action`)** | none |
| `chord-redo` | `Meta+Shift+z` | `redo` + the same guest `shell.windowActivate` | none |
| `chord-fullscreen` / `-exit` | `Meta+Shift+f` | **(none)** | **none** (and no control delta either: 451 → 451) |
| `chord-panel-anchor-left` / `-right` | `Meta+Alt+1` / `Meta+Alt+2` | **(none)** | **none** |
| `example-picker-open` | click `playground.navbar.fixture` | (none) | **none** (+13 controls, all Radix internals) |
| `example-switch` | click the 2nd option | `setActiveExample` ×1 + `registerBrushMesh` ×66 (guest, origin `gesture`) | none |
| `example-picker-dismiss` | `Escape` | `engagementAbort` ×1 + `registerBrushMesh` ×13–20 | none |
| `role-viewer` | click `playground.navbar.roles.viewer` | `setActiveExample` (refused `undeclared-action` on `…#viewer`) + `registerBrushMesh` ×5–6 | +`window:framework.window.mesh`, −`window:puzzle3d-main-{top,perspective}`, −`panel:framework.panel.{catalogue,inspection}` |
| `role-editor` | click `playground.navbar.roles.editor` | `setActiveExample` + `noteShellCommand` ×1 + `registerBrushMesh` ×28 | the exact inverse |

### 🩸️ Two corrections to the packet brief, from the data

1. **`chord-command-palette` opens nothing.** React's palette chord is `mod+p`
   (`ui.search.toggle` of `SHELL_KEYBINDINGS`), not `Meta+k`. The probe's step name promises a palette;
   the journal and the control census both say React answered with nothing at all. **Parity therefore
   means staying inert** — opening a palette on `mod+k` here would have been the difference, not the fix.
2. **`chord-fullscreen` and the two `chord-panel-anchor-*` steps also do nothing in React.** The probe
   presses `Meta+Alt+1`/`Meta+Alt+2`, which name no React shell row (its anchors are the
   `ctrl|meta+b` / `+m` family); and `Meta+Shift+f` does hit `os.toggleFullscreen` but that verb is a
   pure chrome latch — no journal row, no surface change, not even a control change.

### 🔗️ Why `undo`/`redo` journal `shell.windowActivate`

Not a re-activation the shell performs. The row's origin is `guest` and its `causedBy` is the `undo`'s
own `inputSeq`: React's `handleActiveWindowChange` (`🏛️ShellHost/🟦️.tsx:10393`) pushes
`noteShellCommand("shell.windowActivate", …)` on **every** active-window change, the plugin's universal
command recorder puts it on the guest's undo stack, and `undo` pops it and asks the host to **replay**
it (`🔌️plugin/🧫️fixtures/reserved-undo-browser-note.json`'s `undo.replayActionId`). The host then
refuses it as `undeclared-action` — and React's ledger journals refusals too. So the pair is only
reproducible if the wgpu shell puts the same command on the same stack first. It never did (root cause
§2.3).

---

## 2. Per-step root cause and fix

### 2.1 `chord-command-palette` / `chord-panel-anchor-left` / `chord-panel-anchor-right` — already parity, now pinned

**Root cause:** none — `mod+k`, `mod+alt+1` and `mod+alt+2` name no row of `SHELL_SHORTCUT_ROWS`,
reserve nothing and match no puzzle3d binding, so the wgpu shell is as inert as React. The risk was
*regressing into* activity, so the behaviour is pinned rather than changed.

**Test:** `the_inert_probe_chords_claim_no_shell_verb_and_reserve_nothing` (also asserts `mod+p` still
resolves `ToggleSearch`, so the real palette chord cannot be lost while pinning the fake one).

### 2.2 `chord-fullscreen` / `chord-fullscreen-exit` — already parity, now pinned

**Root cause:** none. `ShellShortcut::ToggleFullscreen` → `apply_os_command("os.toggleFullscreen")`
sets `fullscreen_toggle_requested` and returns; it dispatches no action, and `chrome_surface_census`
has no full-screen term (`fullscreen_active` only swaps the navbar chip's icon/label). Same empty
journal and same empty surface delta as React.

**Test:** `the_fullscreen_chord_flips_a_chrome_latch_and_journals_nothing` (drives `apply_os_command`
on a real `ShellState` and asserts `deferred_actions` stays empty).

### 2.3 `chord-undo` / `chord-redo` — **fixed**: the shell never noted a window activation

**Root cause:** the wgpu shell notes `shell.windowResize`, `shell.windowMove`, `shell.windowClose`,
`shell.windowMaximize`, `shell.panelToggle`, `shell.panelTab` and `shell.applyNamedLayout` — but never
`shell.windowActivate`, which React notes on *every* active-window change. So the guest's command stack
top was some other command and `mod+z` could not journal React's `undo` + guest-replay pair however
faithfully the `undo` itself was dispatched. Compounding it, this renderer has **eleven** sites that
write `active_window_id` (dock tab select, retained body press, pane chip, palette `window:` row,
keybinding owner activation, session switch, boot, …) where React has exactly one funnel — `Mode`'s
`onActiveWindowChange` callback.

**Fix:** witness the change at the one point every writer converges on, instead of asking eleven
writers to announce it (and instead of touching lines W9c/W10a are editing).

- `🦀️.rs:2612` — `ShellState::noted_active_window: Option<(u32, String)>`, the `(instance, window)` the
  guest's history was last told about.
- `🦀️.rs:13984` — `window_activation_is_user_v1(noted, instance_id, active)`: pure; true only for a
  changed, non-empty window **inside one live instance**. A first observation seeds; a change that
  crosses a session instance is a *mount*, not an activation (which is why a role switch journals none,
  matching React's `role-viewer`).
- `🦀️.rs:14798` — `ShellState::arm_window_activation_note()`: arms one
  `noteShellCommand{commandId: "shell.windowActivate", detail: {windowId}}` on `deferred_actions`.
- `🦀️.rs:10175` — called at the top of `drain_deferred_actions`, the single lane both the synchronous
  `flush_deferred_actions` and the per-frame settle pump cross, and the lane the chrome ledger taps.
- `🦀️.rs:21170` — `command.activateWindow` → `Activate Window` / `Fenster aktivieren`, matching React's
  `ui.shellCommand.windowActivate` (`🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:284`).

**React ref:** `🏛️ShellHost/🟦️.tsx:10392-10396` (`handleActiveWindowChange`) and `:6904` (`noteShellCommand`).

**Tests:** `only_a_real_in_session_activation_is_a_window_activate_note` (all five branches of the pure
guard) and `a_real_activation_arms_the_shell_window_activate_history_note` (drives a live `ShellState`:
seed → silence, change → exactly one note carrying `shell.windowActivate` + the `windowId`, re-observe →
silence). Also pinned: `the_undo_and_redo_chords_resolve_the_framework_edit_verbs`.

This fix is complementary to W10a's lane: React's `orbit-drag` step journals a `noteShellCommand` too,
and that one is this same activation.

### 2.4 `chord-escape` / `example-picker-dismiss` — **two fixes**

`engagementAbort` is not a shell verb: puzzle3d's editor declares `escape → engagementAbort`
(`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`, `manifest.apps[2].keybindings[0]`), so React resolves it in
`handleAppKeydown`'s app-keybinding loop. Both Escape steps are the same rung — by
`example-picker-dismiss` the dropdown has already been closed by the option click.

**Root cause A — multi-chord bindings were structurally dead.** `match_app_keybinding` handed the whole
`keys` string to `key_event_matches_chord`, which splits on `+` only. React splits on `,` first
(`parseKeys`). puzzle2d declares `"delete,backspace"` for `deleteSelection`: the old code resolved the
key token `"backspace"` carrying a literal `delete,` prefix and matched **neither** key. Every
comma-separated binding in the repo was unreachable on this renderer. (puzzle3d happens to declare its
chords one per row, so `escape` itself was already reachable — but the rung it travels was broken.)

**Fix:** `🦀️.rs:11541` — `match_app_keybinding` splits `keys` on `,`, trims and lowercases each chord,
and applies the reserved check **per chord** exactly where React applies it
(`if (reservedChords.has(chord)) continue`), so a binding that also claims a shell chord loses only
that chord. The caller's blanket `!is_reserved_shell_chord_in(…)` gate on the app rung is dropped
(`🦀️.rs:11494`) because the per-chord check now covers it — which is React's own shape.

**Root cause B — an open navbar dropdown had no keyboard route out.** `handle_keyboard`'s Escape arm
knows `open_selects` (retained widget selects) and the context menu; `palette_open` covers
`Search`/`Find` only. `OverlayState::Dropdown(_)` was in neither, and `idle` in
`handle_keyboard_async` requires `overlay_state == None` — so with the example picker open, Escape
(and every other chord) was swallowed. React closes it through Radix's `DismissableLayer`.

**Fix:** `🦀️.rs:11170` — Escape closes an open `OverlayState::Dropdown(_)` right after the
`open_selects` arm, following the same topmost-overlay precedence, and journals nothing.

**React ref:** `🏛️ShellHost/🟦️.tsx:8569` (`parseKeys`) and `:8610-8616` (the keybinding loop's own
`reservedChords.has(chord)` skip).

**Tests:** `escape_resolves_the_apps_own_engagement_abort_binding`,
`a_comma_separated_binding_matches_each_chord_it_declares`,
`a_binding_that_also_claims_a_shell_chord_keeps_its_other_chords`,
`escape_closes_the_open_example_dropdown`.

### 2.5 `example-picker-open` — already parity, now pinned

**Root cause:** none. The trigger is already React's own `playground.navbar.fixture`
(`shell_example_control`) and every row is `shell.example.<id>` registered as `HitKind::DropdownItem`,
which is exactly what the probe's `pickExample` fallback resolves when no DOM `[role=option]` exists.
The 13 controls React adds are Radix Select internals (`semio-select-_r_u_-content`, `select-viewport`,
`select-item-text`, …) — generated ids no canvas renderer can or should publish, and deliberately
outside the verdict. Crucially, opening the dropdown adds **no surface**: `chrome_surface_census`
(`🦀️.rs:9401`) emits a `dialog` level only for the introduction tour, so the surface delta is empty on
both renderers.

**Test:** `the_example_picker_publishes_reacts_trigger_and_row_control_ids`.

### 2.6 `example-switch` — already parity, now pinned

The `shell.example.<id>` arm of `handle_shell_hit` sets the picker's selection, closes the dropdown and
dispatches `setActiveExample` through `dispatch_action` — whose very first statement is the chrome
ledger's tap (`crate::interpreter::note_dispatched_action`). The guest's `registerBrushMesh` echoes
arrive by the same kernel path they do in React: the guest answers with
`Effect::DispatchAction`, which `🦀️.rs:5970` pushes onto `deferred_actions` and
`drain_deferred_actions` takes through the **same** `dispatch_action` funnel — so they are journaled
here too, with the same ids. (Code-verified end to end; live counts are W9c's next probe.)

**Test:** `picking_an_example_row_selects_it_and_dispatches_set_active_example`.

### 2.7 `role-viewer` / `role-editor` — **fixed**: a successor instance was never told its example

**Root cause:** React re-runs its boot-example effect for **every** session instance
(`🏛️ShellHost/🟦️.tsx:9982-9997`, guarded by `noExampleResetInstanceIdRef.current === session.instanceId`),
so a role switch re-announces the selected example to the new instance — which is the `setActiveExample`
both role steps journal, and why the successor opens the same document the picker shows. The wgpu twin
`apply_boot_example` (`🦀️.rs:5150`) covers the **boot only**, and only for `?example=`, so
`run_session_app_switch` mounted its successor on the dialect's first document and journalled nothing.

**Fix:**
- `🦀️.rs:5182` — `announce_session_example()`: dispatches `setActiveExample` with the id
  `sync_session_chrome`'s `resolve_boot_example_id` has already resolved for the successor's dialect
  (so it is exactly what the navbar picker shows), and no-ops for a dialect with no example.
- `🦀️.rs:8763` — called from `run_session_app_switch` after `push_contributions()` and before the first
  `refresh_ui`, the ordering `apply_boot_example`'s own doc pins (the guest's flow-extension registry
  must be armed before the document changes).

The dock reconfiguration React shows in the surface delta is already this target's behaviour:
`run_session_app_switch` clears `layout_override`, `sync_dock()` rebuilds `DockState::from_app` for the
successor, and `sync_session_chrome()` reloads the persisted dock for the new app id, so windows and
panel anchors the successor does not declare drop out of `chrome_surface_census`.

**Tests:** `a_freshly_mounted_instance_is_announced_its_resolved_example`,
`the_role_chips_and_their_chords_name_the_same_two_verbs`.

---

## 3. Residual risks — for W9c's next probe to settle

1. **`role-editor`'s single `noteShellCommand`.** React journals one; this shell will journal none,
   because a session switch re-seeds the activation witness rather than noting it (§2.3). React's own
   `role-viewer` journals none either, so the rung cannot be a plain "note the landing window" — the
   most likely source is React's 350 ms layout-settle debounce (`handleModeLayoutChange` →
   `classifyWindowLayoutChange` → `rearrange`), which fires for the editor's two-pane stack and not for
   the viewer's single root window. Deliberately not guessed at here.
2. **`chord-escape` vs content focus.** `handle_keyboard_async` routes keys into retained content
   whenever `content_has_focus(active_window)` — a rung React does not have (its gate is a real
   editable DOM target). If the world surface reports content focus after `pick-instance`, Escape will
   reach `dispatch_ui_event` and never reach the app-keybinding rung. Untouched here (it is a tracked
   flag owned by the retained-pointer path); the probe will show it as `chord-escape` journaling
   nothing.
3. **Escape's utility rung.** When a utility *is* active, React dispatches `SET_ACTIVE_UTILITY_ACTION_ID`
   (journaled) where this shell removes the entry locally (not journaled). Not exercised by the measured
   run — no utility was active at `chord-escape` — so it is recorded rather than changed.

---

## 4. Verification

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | clean, 62 warnings (proof the expansion ran) — `🗑️generated/w10b-native-check.txt` |
| `cargo check … --target wasm32-unknown-unknown` | clean, 70 warnings — `🗑️generated/w10b-wasm-check.txt` |
| `cargo test … --lib -- --test-threads=1 chord_example_role_parity shell_shortcuts_palette shell_input window_scope chrome_parity panel_anchor` | **120 passed, 0 failed** — `🗑️generated/w10b-tests.txt` |
| new laws alone | **13 passed, 0 failed** |

Whole-crate run: **878 passed, 9 failed**. None of the nine is in this lane or touched by these edits —
`async_boundary_tests` ×3, `engine_canvas` ×1, `scenes::admitted_surface_map_tests` ×1,
`chrome_overlays_tour_tests::window_silhouette_border_…` ×1, `tool_run_panel_tests` ×2 (retained-chrome
paint/hit registry), `command_registry_tests::directory_home_bootstrap_…` ×1 (HTTP fake transport, no
`ShellState` at all; that test file is unmodified since 2026-09-18 00:48). They are peers' in-flight
work on the same crate.

No wasm activation, no `activate-*`, no trunk, no probe run from this packet — W9c owns those, and its
next rebuild picks these changes up.

## 5. Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
  — lines 2612, 5182, 8763, 10175, 11170, 11494, 11541, 11967, 13984, 14798, 21170.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs`
  — new, 13 laws.
