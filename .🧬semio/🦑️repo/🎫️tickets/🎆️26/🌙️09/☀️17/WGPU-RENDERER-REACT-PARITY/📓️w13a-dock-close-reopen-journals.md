# 🛰️ W13a — the dock's close/reopen lane and what its caps journal

Packet W13a of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Six items, every one of them measured
against `🗑️generated/w12c-parity-run-19/steps.json` and against React's own source, never inferred.
Two of the six turned out to be one defect wearing two masks, one turned out to be a **React-side**
gap this packet could only name, and one is reported unresolved with the exact next measurement.

---

## 0. What run 19 actually shows, step by step

The packet was briefed as "`window-cap-close` closes TWO windows where React closes one and keeps the
other". The run says something more precise, and the difference matters for every later item:

| # | step | wgpu control | wgpu surface delta | React control | React surface delta |
| --- | --- | --- | --- | --- | --- |
| 16 | `window-cap-focus` | `dock.tab.0.puzzle3d-main-top.focus` | −`window:puzzle3d-main-perspective` (maximize) | `mode-dock-tab-focus` | none |
| 17 | `window-cap-close` | `dock.tab.0.puzzle3d-main-top.close` | −`window:puzzle3d-main-top`, +`window:puzzle3d-main-perspective` | `mode-dock-tab-close` | none |
| 18 | `window-reopen` | **`dock.tab..puzzle3d-main-perspective.close`** | −`window:puzzle3d-main-perspective`, **+`window:`** | `mode-dock-tabbar` | none |

So:

- **wgpu's `window-cap-close` closes exactly ONE window.** That half was already right.
- **The second window is closed by the `window-reopen` step**, because the probe's `reopenWindow`
  resolves "the dock tab" by scanning the published control rows
  (`🐍️parity-interact-probe.mjs`: `.find(/mode-dock-tab|dock\.tab|appSwitcher|window.*tab/i)`) and the
  FIRST `dock.tab.…` row this renderer published for a stack was its **destructive `close` chip**.
  React's same scan lands on `mode-dock-tabbar`, the container that owns the tabs, and does nothing.
- **React closes NOTHING at any of the three steps**, and journals nothing — not because React's caps
  are silent by design, but because they are **occluded** (§4). React's console carries zero
  `shell.windowClose` in 104 589 lines and both `[data-window-id]` windows are still alive at step 36.

---

## 1. Item 1 — the dock survives the cap sequence

### 1.1 Root cause

`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`'s `render_stack` registered each tab's action chips (`focus`,
`close`, `drag`) and only then the tab's own SELECT target. React's DOM order is the reverse:
`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1034-1100` renders the `mode-dock-tab` container, then the
`<button role="tab">` label, then `mode-dock-tab-focus`, `mode-dock-tab-close`, then the `DragHandle`.
Publication order is what an id-scanning consumer reads, and this renderer published the one
irreversible control first.

### 1.2 Fix

`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1613-1625` — the select hit is registered **before** the chip loop, in
React's order. On a normal tab the rects do not overlap (the select band ends where the chips begin), so
this is publication order only and changes no press resolution. The law and the 🩸️ trail are on
`render_stack`'s new docstring.

One case it also corrects: `select_w` is `(tab.w − chips).max(padding × 2)`, so on a tab narrow enough
to hit that floor the select band OVERLAPS its chips. With the old order the select was registered last
and therefore topmost, and swallowed the chip press; with React's order the chips are the later
siblings and win, which is what the DOM does.

**Effect on the journey:** after `window-cap-close` the survivor's first published row is
`dock.tab..puzzle3d-main-perspective` — a select target, which `shell_command_for_control` maps to no
command and which `handle_shell_hit` answers with nothing. The `window-reopen` step therefore becomes
what React's is: a no-op on an already-active tab, with an unchanged surface set. `docked` stays `1`.

### 1.3 On "reopen through the same app-switcher path React uses"

There is no reopen path on React to mirror. React's `window-reopen` resolves `mode-dock-tabbar`
(rect `[3, 32, 650, 29]`, the stack's cap ROW) and clicking its centre changes nothing:
`controlCount [335, 335]`, no surfaces, no actions. A `mode-dock-tabbar` twin was considered and
**deliberately not added**: React's cap row inherits `pointer-events-none` from
`stackClassName="pointer-events-none …"` (`🎨️Canvas/🟦️.tsx:1234`), so a click on its empty band falls
THROUGH to whatever is beneath it. Registering a hit row here would make wgpu swallow presses React
lets through — a new divergence traded for a cosmetic id match. The parity verdict does not need it:
both renderers now record `resolved: "contains"`, no actions and no surface delta.

---

## 2. Item 2 — the window with an EMPTY id

### 2.1 Root cause

`collect_stack_bodies` (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1916`) pushed **every** stack's body rect and
its `active` tab id, including a stack holding no tabs, whose `active` is `String::new()`
(`empty_stack()`, React's `{ kind: "stack", children: [] }`). `ShellState::plan_dock_windows` then fed
that `("", rect)` pair straight into `dock_window_plan`, `window_content_rects` and
`window_silhouettes`, so an emptied dock minted a full-canvas window whose id was `""`:

- the engine-surface census reported `windows=["", "tool.fill", …]`
  (`🗑️generated/w12c-parity-run-19/wgpu/console.txt:5676`, 1 176 lines of it);
- the chrome census published a `window:` surface (`steps.json` step 18 `surfacesAdded: ["window:"]`);
- the body registered a full-bounds `HitKind::ScrollRegion` with an EMPTY control id — the row W12c §2.1
  measured swallowing the `zoom-wheel` notch.

`render_stack` already returned early for an empty stack, so the leak was in the PLAN only, never in
the paint. `tool.fill` in that census line is unrelated: `live_window_ids` chains
`dock_window_plan` with `panel_documents.keys()`, and `tool.fill` is an open panel tab.

### 2.2 Fix

- `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1962` — an empty stack owns no silhouette.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:17961-17962` — `dock_window_plan` and `window_content_rects` drop the
  empty window id.

`dock_drop_bodies` deliberately KEEPS the emptied stack's rect: React's `WindowChrome` keeps rendering
for an empty stack (only its `activeDescriptor`/body vanishes, `🎨️Canvas/🟦️.tsx:1176`/`:1212`), which is
how a dragged window is dropped back into an emptied dock.

---

## 3. Item 3 — the cap journals

Read against `🏛️ShellHost/🟦️.tsx`, which is the authority for what React notes:

| cap | React | wgpu before | wgpu now |
| --- | --- | --- | --- |
| focus | `activateWindow` + `toggleMaximize`; the activation is noted by `handleActiveWindowChange` (`:10392`), the maximize by **nothing** — `shell.windowMaximize` is declared nowhere in ShellHost | `noteShellCommand shell.windowMaximize` | **nothing of its own**; the activation note is armed by `arm_window_activation_note`, exactly as React |
| close | `closeWindow` → `onWindowClose` → `noteShellCommand("shell.windowClose", …, { windowId })` (`:10530-10531`) | `noteShellCommand shell.windowClose` | **unchanged — kept** |
| reopen | no control, no note | `noteShellCommand shell.windowClose` (it was pressing a close chip) | **nothing** (item 1) |

**Fixes:** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15002` drops the `.focus` arm from
`shell_command_for_control`, and `🦀️.rs:10110` drops the now-dead `note_control_command` call from the
focus arm. `🔬️wgpu-command-registry/🦀️.rs:999` is updated to assert the absence.

### 3.1 Where this packet does NOT follow its brief, and why

The packet asked for the CLOSE note to be removed too ("React records dock changes only through
`SET_SHELL_LAYOUT`, not an action"). That is not what React does:
`🏛️ShellHost/🟦️.tsx:10530-10531` notes `shell.windowClose` on every `onWindowClose`, and the wgpu arm
cites it for the invertibility W2d's close semantics depend on. Removing it would make the renderer
diverge from React's source in order to match a React RUN in which the cap never fired at all (§4).
The note stays; §4 names the real difference and its owner.

---

## 4. 🔴 Finding handed on — React's window caps are OCCLUDED by its top-left panel

Compare the two run-19 screenshots at step 15 (`react/15-split-gutter-drag.png`,
`wgpu/15-split-gutter-drag.png`):

- **React** paints the Catalogue panel from `y≈28`, directly over the LEFT stack's cap row at
  `y=32..61`. The probe's cap chips resolve at `[48, 35, 22, 22]` and `[73, 35, 22, 22]` — under the
  panel. Every click lands on the panel, which is why React's three cap steps journal nothing, close
  nothing and leave `controlCount` oscillating only with hover chips.
- **wgpu** paints the same panel from `y≈60`, BELOW the cap row, so its caps are reachable.

This is a panel-anchor geometry divergence, not a dock one, and it is the entire remaining
`window-cap-focus`/`window-cap-close` parity gap. **Owner: the panel-anchor lane** (W12d/W13b).
Whichever geometry is correct, the two renderers must agree before the cap steps can be compared at
all — today they are measuring two different applications. Evidence: zero `shell.windowClose` and zero
`shell.windowMaximize` in `react/console.txt` (104 589 lines), and both
`window:puzzle3d-main-{top,perspective}` alive until the role switch at step 36 removes them.

---

## 5. Item 4 — `shell.windowActivate` on `chord-undo`/`chord-redo`

### 5.1 The full ladder, measured

React's `chord-undo` journals TWO rows: `undo` (origin `user`) and `shell.windowActivate`
(origin `guest`, refused `undeclared-action`, `causedBy` the undo). The second is **not** a second
chord route — it is the GUEST replaying the top of its own shell-command stack:

```
139195 worker debug [DEBUG] chrome history action=undo seq=15 inverse=shell.windowActivate
139283 page warning input #69 shell.windowActivate refused: undeclared-action (guest … causedBy=#68)
                                          (🗑️generated/w12c-parity-run-19/react/console.txt:20196, :20377)
```

`🔌️plugin/🦀️.rs:26109-26130` `dispatch_chrome_history_action` finds the newest command-log entry that
has an inverse and no document edit, and answers with `Effect::ReplayShellCommand`. The entry at
`seq 15` is React's own activation note, put there at step 19 (`orbit-drag`, ledger `seq 43`
`noteShellCommand`, origin `user`) when the press on the perspective pane changed the active window.
The guest is the SAME wasm on both renderers, so the whole difference is upstream: **did the host ever
note an activation?**

### 5.2 Root cause on wgpu — no dock, no activation

`arm_window_activation_note` (W10b, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15053`) is armed at every
`drain_deferred_actions`, and the press path that feeds it is `activate_window_under_pointer`
(`🦀️.rs:9791`), which resolves the pressed window from **`dock_window_plan`**. From `window-reopen`
(step 18) on, that plan held one entry and its id was `""` — skipped by the same guard. No press could
change the active window, the guest's command log carried no activation, and `mod+z` had nothing to
replay. Nothing in the witness itself was wrong.

**Fix: items 1 + 2.** With the dock intact the plan holds both panes, the scene press at 70 %/45 %
activates the perspective pane, and the note is armed. Proven by
`a_press_in_a_docked_pane_arms_the_activation_the_undo_chord_replays`, which asserts both halves —
the armed `shell.windowActivate` note with `detail.windowId = puzzle3d-main-perspective`, and the
run-19 empty-dock case answering `false`.

---

## 6. Item 5 — `engagementAbort` on the three dismiss steps

`context-menu-dismiss`, `chord-escape` and `example-picker-dismiss` all journal exactly one
`engagementAbort` on React, at `windowId puzzle3d-main-perspective`. W12c already proved the wgpu
keyboard ladder resolves the binding at every one of them (`appBinding=Some("engagementAbort")` in the
chord-gate line) and already made Escape dismiss a layer WITHOUT consuming the chord. The verb is
**window-owned**: `resolve_keybinding_target_window_v1` needs a mounted dock window, and from step 18
on there was none — so all three raised the unowned-chord banner and dispatched nothing.

Nothing further was owed by the keyboard lane; this item is closed by items 1 + 2. Both directions are
now laws:

- `escape_with_a_docked_owner_dispatches_the_apps_engagement_abort` — the picker is dismissed by the
  same keydown, NO unowned banner is raised, and `engagementAbort` crosses the dispatch funnel.
- `escape_with_an_empty_dock_is_the_hinted_no_op_run_19_measured` — the converse, pinning the exact
  state run 19 was in.

---

## 7. Item 6 — `role-editor`'s extra `noteShellCommand` — UNRESOLVED, and what will settle it

Measured: React's `role-editor` journals `setActiveExample`, then **one** `noteShellCommand`
(`seq 274`, origin `user`, dispatch `windowId puzzle3d-main-top`), then its `registerBrushMesh` echoes.
React's `role-viewer` journals **no** note at all.

The command id is **not recoverable from run 19**: React's ledger record carries no args
(`🏛️ShellHost/🎯️input-ledger/🟦️.ts:163` — `{ inputSeq, controllerId, action, origin, windowId,
causedBy, outcome }`), and the console only shows `plugin_exchange actionId=noteShellCommand
branch=spawn-admit` at `instance=3`. Two candidates fit the data and they need different fixes:

| candidate | why it fits | why it is not proven |
| --- | --- | --- |
| `shell.windowActivate` | the editor mount lands on `puzzle3d-main-top`, and the row's `windowId` is that window | the viewer mount also lands on a dock window (`mode-dock-tab-root-framework.window.mesh` is added at step 36) and journals nothing |
| `shell.panelTab` (`🏛️ShellHost/🟦️.tsx:10042`) | `role-editor` restores TWO panels (`panel:framework.panel.catalogue`, `panel:framework.panel.inspection`); `role-viewer` restores none | the same reconciliation runs in both directions |

wgpu's witness refuses an instance-crossing landing on purpose
(`window_activation_is_user_v1`, W10b: "a successor instance's landing window is a mount, not an
activation"), generalised from React's silent `role-viewer`. Relaxing that rule would mirror
`role-editor` **and break `role-viewer`**, trading one mismatch for another, so it is deliberately left
alone.

**Next measurement (cheap, one line):** carry the noted `commandId` into `InputLedgerRecordV1` (or log
it once, diagnostics-gated, in `noteShellCommand`, `🏛️ShellHost/🟦️.tsx:6904`) and re-run the probe. The
run then names the command and the rule follows in one edit. Not done here because `🏛️ShellHost/🟦️.tsx`
is under concurrent edit by W13b/W13e and the change belongs to whoever owns the ledger contract.

---

## 8. Changes

Line numbers are as of 2026-09-18 21:25; both files are under concurrent edit, so the anchors are named
as well.

| file:line | change |
| --- | --- |
| `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1557-1570` | `render_stack` docstring: React's publication order + the 🩸️ trail |
| `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1617-1620` | the tab's SELECT target is registered before its chips |
| `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1916-1921`, `:1962` | `collect_stack_bodies` docstring; an empty stack owns no silhouette |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:18479-18493` | `plan_dock_windows` docstring: why an empty window id is no window |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:18514-18515` | the plan and the content rects drop the empty window id |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10213` | the `dock.tab.….focus` arm no longer calls `note_control_command` |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15535-15543` | `shell_command_for_control`: the `.focus` arm is gone, with React's reason |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12346` | registers the new test module |

## 9. Tests

| law | file |
| --- | --- |
| `the_close_cap_closes_exactly_the_clicked_window_and_refocuses_the_survivor` | `🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs` |
| `the_reopen_scan_after_a_close_lands_on_a_tab_select_that_closes_nothing` | idem |
| `every_dock_tab_publishes_its_select_target_before_its_chips` | idem |
| `an_emptied_dock_never_enters_the_window_plan` | idem |
| `the_focus_cap_journals_no_shell_command_and_only_its_activation` | idem |
| `a_press_in_a_docked_pane_arms_the_activation_the_undo_chord_replays` | idem |
| `escape_with_a_docked_owner_dispatches_the_apps_engagement_abort` | idem |
| `escape_with_an_empty_dock_is_the_hinted_no_op_run_19_measured` | idem |
| `the_cap_rows_of_the_journal_fixture_match_the_shell_command_table` | idem |
| `every_diverging_journal_row_states_why` | idem |
| `a_tabs_select_target_is_published_before_its_destructive_chips` | `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs` |
| `an_emptied_stack_keeps_its_drop_body_and_owns_no_window` | idem |
| `shell_command_for_control_maps_dock_and_panel_control_ids` (updated) | `🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs` |

New fixture: `🐚️Shell/🧫️fixtures/📜️journal-sequences/🔣️.json` — the dock/chord twin of
`🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json`. Nine rows, React's column measured off run 19, this
shell's column the contract, `control` where the row is reproducible without the probe, and a written
`why` on every divergence. The two laws above keep it honest: the cap rows' `commands` must equal what
`shell_command_for_control` answers, and a row whose columns differ must say why.

## 10. Gates

All run 2026-09-18 21:25-21:35, `-j 4`, foreground, one cargo at a time, `--test-threads=1`.

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **exit 0**, 62 warnings (proof of expansion) | `🗑️generated/w13a-renderer-check.txt` |
| `cargo check … --lib --target wasm32-unknown-unknown -j 4` | **exit 0**, 70 warnings | `🗑️generated/w13a-wasm-check.txt` |
| `cargo test … --lib dock_close_reopen_journal_tests` | **10 passed, 0 failed** | `🗑️generated/w13a-shell-tests.txt` |
| `cargo test … --lib dock` (dock element + shell dock lanes) | **97 passed, 0 failed** | `🗑️generated/w13a-dock-tests.txt` |
| `cargo test … --lib shell::` (whole shell suite) | 413 passed, 1 failed — W13b's own in-flight suite, green again minutes later | `🗑️generated/w13a-shell-suite.txt` |
| `cargo test … --lib` (whole renderer suite, 930 tests) | **928 passed, 2 failed — not this packet's** | `🗑️generated/w13a-renderer-suite.txt` |

The two remaining failures are `agent_bridge::tests::every_gateway_to_shell_fixture_round_trips_through_this_codec`
and `…every_modelled_shell_to_gateway_fixture_round_trips_through_this_codec`
(`🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs:51`, `AgentToolCall did not decode: UnknownTag(8)`) — an
agent-bridge codec/fixture drift in another lane, untouched by this packet. The shell-suite failure
seen minutes earlier
(`shell::window_actions_search_pane_tests::an_arg_carrying_row_opens_reacts_staged_form_and_refuses_execute_until_it_resolves`,
W13b's untracked `🎬️wgpu-window-actions-search-panes` suite) was gone by the full run.

## 11. Build note

For ~25 minutes `cargo check -p semio-framework-os-renderer-wgpu --lib` was blocked by a peer's
in-flight refactor of `semio-s-artifact-stdio-pdf`, which the renderer depends on transitively
(`pdf → semio-s-artifact-puzzle-2d → semio-s-plugin-puzzle → semio-framework-os-renderer-wgpu`):
45 errors, every one of them inside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/**` (`PdfPage::text` field →
method, `PdfSnapshot`/`PdfInfo` gaining fields, `io::text_document` removed). Polled every 2-3 minutes,
`non-pdf = 0` at every poll, and the peer landed at 21:25 — after which all five gates above ran clean.
Recorded here because it is the `📓️feedback-concurrent-cargo-workspace-churn` pattern exactly: a
repo-wide failure that belongs to a shared file, not to the packet.

## 12. For W13c's next probe

With items 1 + 2 in, the next run should show, on the wgpu column:

- `window-cap-focus`: **no actions** (was `noteShellCommand`), surface delta = the maximize only.
- `window-cap-close`: `noteShellCommand` (`shell.windowClose`), exactly one window removed.
- `window-reopen`: **no actions, no surface delta**, and the dock still holding
  `puzzle3d-main-perspective` — `docked=1` for every step below it.
- `chord-escape`, `context-menu-dismiss`, `example-picker-dismiss`: **`engagementAbort`**.
- `chord-undo`/`chord-redo`: `undo`/`redo` **plus** a guest `shell.windowActivate`. The activation
  that seeds the guest's command log now happens at `window-cap-close` itself: the close moves focus
  from `puzzle3d-main-top` to `puzzle3d-main-perspective`, which `arm_window_activation_note` witnesses.
  Step 17 therefore journals TWO `noteShellCommand` rows (`shell.windowClose`, `shell.windowActivate`)
  — one action id either way, which is what the probe compares.
- `orbit-drag` will most likely STILL differ: React's press changes its active window
  (`main-top → main-perspective`, hence its extra `noteShellCommand`) while on wgpu the close already
  left `main-perspective` active, so the press is not a change and notes nothing. That row is
  downstream of §4 — once React's caps are reachable, both renderers enter step 19 with the same
  active window and the row resolves by itself. Do not "fix" it by noting unchanged activations:
  React's `activateWindow` returns early on an unchanged window (`🎨️Canvas/🟦️.tsx:1447`).
- `window-cap-focus`/`-close` will still DIFFER from React until §4's panel occlusion is settled.
