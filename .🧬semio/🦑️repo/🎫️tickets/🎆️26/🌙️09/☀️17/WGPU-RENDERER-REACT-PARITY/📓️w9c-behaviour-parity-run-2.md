# ⚖️ W9c — Behaviour parity, run 2

Packet input: `🗑️generated/parity-run-2/` (37 steps, **34 of 37 differed**). This report names the root
cause of each priority step, the fix, the React reference it was measured against, and the law that pins
it. Evidence for every claim below is the run-2 `steps.json` and the wgpu console capture beside it —
nothing here is inferred from reading React alone.

---

## 1. P0 — `window-cap-close` (step 17): the close tore the whole shell down

### 1.1 What the run recorded

`wgpu/console.txt:3744`

```
69742 wgpu-shell pointer button x=72.01 y=43.2 down=true … hit=Some((Button, Some("dock.tab.0.puzzle3d-main-top.close")))
69860 wgpu-shell dock plan canvas=1594x936 windows=1 puzzle3d-main-perspective@1594x914+3,54
69925 frame fault recorded: frame world resource admission exceeded fixed credits
69931 page error wgpu renderer fault: worker-frame-failed: frame world resource admission exceeded fixed credits
```

The close itself SUCCEEDED — the dock replanned to one window. The frame that painted the replanned dock
then faulted, and a frame fault kills the page: every surface disappeared (`controls 51 → 0`,
`ready: null`) and the renderer stopped answering `dumpChrome`, which is why steps 17–37 all read
`renderer exposes no dumpChrome`. The teardown was NOT `close_dock_window` destroying the plugin
instance (W2d's lane is intact) and NOT the probe clicking the wrong chip.

The same fault had already killed **run 1** at step 5 (`panel-inspection`, `parity-run-1/wgpu/console.txt:2066`)
with both panes still open. The common factor is not the close: it is a RELAYOUT.

### 1.2 Root cause — the world re-admitted the reference underlay every frame

`♾️infinite/🌍️world/🦀️.rs:225` · `render_world_3d`'s reference-image loop

`ensure_world_plane_texture` BORROWED the decoded pixels and `to_vec()`'d a fresh copy on every call, and
`render_world_3d` calls it once per visible reference image **per surface per frame**. Each copy is a
`PreparedRasterProducer::try_admit`, which reserves `width * height * 4` bytes against the process-wide
`PREPARED_RASTER_PRODUCER_BYTES` ledger (32 MiB, `🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:348`). A producer
holds its credit until the presenter has stepped it to completion, so a two-pane mode kept re-reserving
the same megabytes twice a frame. Under a relayout burst enough producers are outstanding at once that
`PreparedRasterLedger::reserve` returns `None`, `try_admit` answers `raster producer process credits
exhausted`, and `World3dBuildContext::append_step` hands the refusal to the frame as
`World3dBuildRejected::RasterAdmission` — reported as the generic
`frame world resource admission exceeded fixed credits`.

### 1.3 Fix — a refused admission is BACK-PRESSURE, not a frame fault

`♾️infinite/🌍️world/🦀️.rs:230` — `ensure_world_plane_texture` takes its pixels by value and, when
`PreparedRasterProducer::try_admit` refuses them, retires the refusal to terminal RIGHT THERE (its
source bytes and its ledger credit, or the exhaustion would be permanent) and returns. The frame is
handed no rejection owner, so there is nothing to fault on: this surface simply has no underlay upload
this frame, and offers it again on the next one. The process-wide ledger is shared by every surface and
every frame still in flight, so exhausting it for one frame is legitimate — killing the page over it
was not.

🩸️ **The route not taken, and why it is recorded here**: the first fix made the upload one-shot
(`mem::take` of the payload). It removed the fault — and it removed the floor plan: both panes painted
no underlay at all, because a producer rides the frame it was admitted into and a frame whose present is
ABORTED drops its staged texture (`RasterTextureTable::abort_presented_step`). The worker cannot see the
table's `live` registry, so it cannot know a texture is resident; until an upload ACK exists, the offer
has to repeat, and the table's own key-dedupe absorbs it. `reference_underlay_upload`
(`♾️infinite/🌍️world/🦀️.rs:9800`) therefore CLONES and keeps the payload — which is also what lets a
sibling pane, holding its own `World3dState`, offer the same image.

`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13973` — `world_rejection_fault` names the LANE a refused world
resource came from (upload / raster producer / raster admission — the last carries its own refusal
string — / eviction) instead of one message for all four. A frame fault kills the page, so its message
is the whole post-mortem a probe gets; the generic wording cost a full rebuild-and-rerun cycle here.

**React reference**: React uploads a reference texture once per url through the loader cache
(`🌐️World3dHost` textures) and never faults a frame over a texture.

**Law**: `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` —
`a_reference_underlay_is_offered_every_render_and_a_refused_admission_is_not_a_fault`.

**Witness**: `🗑️generated/parity-run-9/wgpu/02-dismiss-tour.png` — the floor plan paints in BOTH panes
again, matching `🗑️generated/w8b-boot-2/final.png`, and the whole 37-step journey records no renderer
fault.

---

## 2. `dismiss-tour` (step 2): the PROBE dismissed the tour during boot

The wgpu tour was armed on time — `wgpu/console.txt:700` shows `ui.introduction.veil` in the hit registry
at t≈9.4 s, well before the step ran, and the ids are already React's (`ui.introduction.{skip,next,back}`,
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `UI_INTRODUCTION_*_CONTROL_ID`). The late arming W7a measured has
since been fixed, and the W5b alias table that bridged `shell.tour.*` is dead.

What dismissed it was the probe's own boot: the wgpu driver ended `boot()` with
`canvas.click({x: 4, y: 4})`, and a press anywhere on the veil ends the tour — deliberately, because
React's veil is `pointer-events-auto` while it blocks (W5b §4.3). By the time `dismiss-tour` ran,
`ui.introduction.skip` no longer existed, which the step reported as `absent`.

**Fix** (`🐍️parity-interact-probe.mjs`): the boot FOCUSES the canvas instead of pressing it, and waits for
a populated hit registry the way the React driver waits for `controls.length > 4`. `CONTROL_ALIASES` is
now deliberately empty, with a note that an alias hides exactly the defect this probe measures.

---

## 3. Panels (steps 3–8): the probe was comparing two DOM conventions, not two shells

React's surface census was "every element carrying `data-window-id` / `data-level` / `data-surface-id`".
On React that yields `panel:framework.panelTab.framework.panel.artifact` for the tab, plus
`panel:framework.chat.send` and `panel:framework.chat.clear` for the chat panel's own BUTTONS, plus
`surface:window:puzzle3d-main-top` restating `window:puzzle3d-main-top`. The wgpu shell models a docked
panel AS a window instance, so it answered `window:framework.panel.artifact` + `surface:…` for the same
gesture. No panel step could ever match, and the control counts the packet noted (≈40 vs 6) are mostly
React's DOM scaffolding (`tree-row-layout`, `tree-gutter`, `scroll-area`…), not missing content: the wgpu
panel publishes the same tree rows (`section.chevron.<section>`, `tree.label.<item>`) and the same
`noteShellCommand` per toggle.

**Fix** — publish the shell's own surface census, with React's levels and React's element ids:

- `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2652` — `struct DumpSurface { id, level, elementId }`;
  `:2752` `note_chrome_surfaces`; `:2762` `ledger_publish_surfaces` (sorted, deduplicated, bounded);
  `surfaces` added to `DumpChrome` and to the window-scoped projection.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9359` — `chrome_surface_census`: this frame's dock panes at `window`
  level, every open anchor's ACTIVE TAB at `panel` level under `panelTabElementId(tabId)`, and the tour
  as one `dialog`. Published from `publish_retained_hit_registry`, i.e. from the same complete chrome
  walk that publishes the hit registry, so the two can never describe different frames.
- `🐍️parity-interact-probe.mjs` — both drivers now build the SAME three-level census: `window:<id>`,
  `panel:<tabId>`, `dialog`. React's `data-surface-id` rows are dropped (they restate the window rows) and
  only `[data-level="panel"]` elements whose id is a `framework.panelTab.*` are counted as panels.

**Laws**: `🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` —
`chrome_surface_census_publishes_react_levels_sorted_and_deduplicated`;
`🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` —
`the_chrome_surface_census_separates_panes_from_docked_panels`.

**Left open (documented, not fixed)**: React's step 7 CLOSES the tool-runs panel when the chat opens,
i.e. React keeps chat and the four `framework.panel.*` tabs in ONE anchor; the wgpu shell puts chat on
`top-right` (`sync_dock`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6614`) while puzzle3d's own panel group lands
on `top-left` (its own journal says `{"anchor":"top-left"}`). That is a panel-group→anchor mapping
question in W2c/W1h's lane, and changing it blind would move chrome three other packets pin.

---

## 4. Pane chips (steps 9–12): the ids were a shell-private grammar

`WindowPaneChip::control_id`'s own docstring called itself "the wgpu twin of React's `toggleId`
(`childElementId("framework.window", id, …)`)" while minting `shell.action.fold.<windowId>` and four
siblings — the window at the END, and a namespace React never spells. Nothing resolved the same chip on
the two renderers.

**Fix** — `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13268`, React's spelling verbatim, through the shared
`semio_framework::child_element_id`:

| chip | id (both renderers) | React source |
| --- | --- | --- |
| Actions | `framework.window.<segment>.engagement.toggle` | `🪟️Window/🟦️.tsx:373` |
| Search | `framework.window.<segment>.search.toggle` | `🪟️Window/🟦️.tsx:388` |
| Window Options | `framework.window.<segment>.measures.{unfold,fold}` | `🪟️Window/🟦️.tsx:330` |
| Utilities | `framework.window.<segment>.utilityBar.{unfold,fold}` | `🪟️Window/🟦️.tsx:410` |
| Projection | `framework.worldOrbit.projection.<segment>.pane.fold` | `🌐️World3dHost/🟦️.tsx:5218` + `Pane`'s default `chromeToggleId` |

`elementIdSegment` is lossy (`puzzle3d-main-top` → `puzzle3dMainTop`), so the window is RESOLVED rather
than parsed back: `window_pane_chip_target` (`:13473`) matches the segment against the instances this
shell actually carries, which also refuses a chip addressed to a window that has since closed.
`toggle_window_pane_chip` (`:13488`) replaces the five removed `starts_with` arms — one table, both
directions (`WindowPaneChip::from_pane_and_verb`, `:13282`).

The press predicate had to follow: a chip is a `HitKind::Toggle` painted INSIDE a world pane's rect, and
it only reached the shell because its id started with `shell.`. `pointer_press_belongs_to_shell_chrome`
(`:9534`) now also claims the `framework.window.` and `framework.worldOrbit.projection.` namespaces.

The shared fixture `🐚️Shell/🧫️fixtures/🪟️window-pane-chrome/🔣️.json` no longer carries a wgpu `controlId`
beside a React `react` id — there is one spelling, with `{windowSegment}`.

**Law**: `🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs` —
`every_pane_chip_id_decodes_back_to_its_own_window_and_chip`, plus the fixture pin and the dispatch pin
that already existed (both now assert React's ids).

`pane-chip-{projection,windowOptions}-toggle` (steps 13–14) resolve ABSENT on both renderers and already
matched: React spells those two `…measures.unfold` and `…projection.<segment>.pane.fold`, so the
journey's keys (`projection.toggle`, `windowOptions.toggle`) name no control on either side.

---

## 5. `split-gutter-drag` (step 15): the world surface swallowed the press

The gutter WAS registered and WAS hit — `os_host pointer hit x=534.39996 y=500 … hit=Some((DockSplit,
Some("dock.split..0")))` — but no `wgpu-shell pointer button down=true` line follows it, so the shell
never saw the press and `begin_drag` never ran. The release then found `input.drag.active == false` and
journalled nothing, where React journals `noteShellCommand("shell.windowResize")`
(`🏛️ShellHost/🟦️.tsx:10427`).

Root cause: on a PRESS the renderer hands the event to the shell only when
`pointer_press_belongs_to_shell_chrome` claims it; a `HitKind::DockSplit` was not claimed, and the split
gutter sits exactly on the seam a pane's rect starts at (`dock.split..0@20x936+524,32` against
`puzzle3d-main-perspective@…+534,54`), so `enqueue_world3d_event` took it instead.

**Fix** (`:9534`): the predicate claims the dock's own structural kinds — `DockSplit`, `DockJoinCorner`,
`PanelResize`, `PanelTab` — and the `dock.` / `panel.resize.` namespaces, which `crate::dock` alone mints.
The resize's `shell.windowResize` journal already existed (`:8979`) and now runs.

React also journals `registerBrushMesh` ×4 on this step — a GUEST lane re-registering its brush meshes
for the resized viewport. Whether the wgpu guest does the same is a run-3 question; the verdict is a set,
so one such entry is enough.

---

## 6. `window-cap-focus` (step 16): a docked panel was painted over the dock's own tab bar

wgpu journalled `noteShellCommand {"commandId":"shell.panelTab","detail":{"anchor":"top-left","tabId":"framework.panel.artifact"}}`
and swapped the catalogue panel for the artifact panel — the press never reached the focus cap at all.

Root cause: `anchor_rect` laid an anchored panel out from `body.y`, which is exactly where the dock paints
its window caps. The top-left panel's tab row and the dock's tab row shared one 22 px strip and their hit
rects overlapped: the focus cap's centre (54.8, 43.2) fell inside the artifact TAB (x 3.2–72.05), and the
close cap's centre (72.01) won its own rect by four hundredths of a pixel.

**Fix** — `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:17452`, `panel_band`: the band an anchored panel may occupy is
the body MINUS the dock's own stack tab bar, read off this frame's plan (the topmost planned window body)
rather than re-derived from the theme, so a mobile stack, a maximized pane and a spawned studio surface
all state it once. `anchor_rect` applies it, so every call site — paint, hit test, drop index, the
published panel boxes and the chat header — moves together.

React lays its panels out inside the mode body for the same reason.

---

## 7. `window-cap-close` / `window-reopen`: what React actually did

React journalled NOTHING on step 17 and its surfaces did not move — yet React's own close lane journals
`noteShellCommand("shell.windowClose", …, { windowId })` (`🏛️ShellHost/🟦️.tsx:10534`). So React's window
was never closed: the journey resolves that chip by CONTAINMENT on `mode-dock-tab-close`, and what it
found is not the control that closes a window. React's "nothing happened" on steps 16–18 is the probe's
own artefact, not React behaviour, and the 25-control delta the packet read as "one window removed" is
the catalogue tree's rows leaving on step 16 and coming back on step 17.

Parity target is therefore the opposite of the recorded React row: BOTH renderers must journal a close and
lose the window surface. The wgpu side now carries React's detail too —
`note_control_command(id, Some({"windowId": …}))` at the `dock.tab.*.close` arm, matching React's
invertible entry; the id already maps to `shell.windowClose` (`:14614`).

---

## 7b. Two more page-killers the journey uncovered

Both were found only because §1 let the journey get past step 15 at all, and both kill the page the same
way a raster refusal did — a `WorldInteractionAuthorityStep::Fault` is a frame fault.

- **A stale aim.** Every world pick cursor is built through `pointer_in_pick_rect`, which refuses a point
  outside the surface's CURRENT rect, and a queued intent outlives a relayout: dragging the split gutter
  moves the pane out from under moves enqueued against its previous bounds (measured: a hover at
  `x=534.4` against `pick=…+535.6,54.4`). `♾️infinite/🌍️world/🦀️.rs` now RETIRES an intent aimed outside
  the surface instead of faulting; a refusal for a point the surface does cover still faults.
- **The middle button.** No branch of the authority claims a `PointerButton` with button 1 — pan is a
  middle-DRAG, planned from the moves that follow — so the press fell through to the unclaimed-intent
  fault. The FIRST middle-click on any world surface killed the page; it now retires.

And two lanes that reach the world at all only because of §5's predicate:

- `🧊️renderer/🦀️.rs`'s `handle_pointer_move` now honours `ShellState::drag_captures_pointer`: a live
  dock/panel drag CAPTURES the pointer (React's `setPointerCapture`), so the surfaces the gutter sweeps
  across no longer receive those moves at all.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `shell_command_controller_id` — the command journal asked
  `host_controller_id()` alone, and a PLAY serve mounts no host app, so every window-cap and split-resize
  command returned before it dispatched while the panel toggles beside them journalled fine. It now reads
  the live session first, host second.
- `🧊️renderer/🦀️.rs`'s `undo_text_operation` now requires a FOCUSED control: the text buffer keeps its
  undo state after a control loses focus, so once the journey had opened the pane's Search box every later
  `mod+z` was swallowed there and the shell's own undo chord never ran.

## 8. What the runs measured

| run | build | differ / 37 | note |
| --- | --- | --- | --- |
| `parity-run-2` (the packet's input) | before | **34** | wgpu lost `dumpChrome` at step 17 and never came back |
| `parity-run-3` | after §1–§6 | 27 | the raster fault is gone; the page now dies at step 15 with a NAMED fault |
| `parity-run-6` | + pointer capture, stale-aim retirement | 26 | steps 15–19 survive; the page dies at step 20 (middle-click) |
| `parity-run-9` | + middle-button retirement, session controller, chip z-order, underlay restored | **20** | **no renderer fault in the whole journey** |
| `parity-run-10` | + the `mod+z` focus guard | 32 | ⚠️ NOT this packet's: a peer's in-flight world refactor was half-landed in that wasm (`prepared world mesh was missing`; the same tree refused to compile minutes earlier with `WorldCameraSync`/`WorldContextMenuCursor` undefined). Re-run when their lane settles. |

`🗑️generated/parity-run-9/parity.md` is the reference table: **17 of 37 steps match**, up from 3, and every
one of the 20 remaining rows is a measured behaviour difference rather than a dead renderer.

### 8.1 Steps that now match (17)

`boot` · `dismiss-tour` · all five navbar panels **and** the chat close (6) · `pane-chip-utilitybar-unfold` ·
`pane-chip-projection-toggle` · `pane-chip-windowoptions-toggle` · `chord-command-palette` ·
`chord-fullscreen` ×2 · `chord-panel-anchor-left`/`-right` · `example-picker-open`.

### 8.2 The 20 that remain, and why

| step(s) | remaining difference | where it belongs |
| --- | --- | --- |
| `pane-chip-{engagement,search}-toggle` | React ALSO dispatches `addObjectKind` when the pane opens — its Actions rail auto-arms the first verb. wgpu's chip flips the fold and arms nothing. | Actions-rail lane (W2c/W6a): the arm-on-open is a feature, not an id |
| `pane-chip-windowcontrols` | React publishes `framework.window.<segment>.windowControls` (an ActionGroup of external/maximize/close inside the window cap). wgpu publishes the DOCK TAB's caps instead (`dock.tab.<path>.<window>.{focus,close}`) — the same controls under a different parent. | the same convergence §4 did for the five pane chips |
| `split-gutter-drag` | both journal `noteShellCommand` now; React ALSO re-registers 4 brush meshes (a GUEST lane reacting to the resized viewport) | guest/viewport-resize lane |
| `window-cap-{focus,close}`, `window-reopen` | wgpu journals `noteShellCommand` (`shell.windowMaximize`/`shell.windowClose` + `{windowId}`) and MOVES the window surfaces; React journals nothing and moves nothing — see §7: the journey's containment match does not reach React's real cap control, and React's own `onWindowClose` journals `shell.windowClose` when it does close. **The wgpu side is the correct one here.** | the probe's React cap resolution |
| `orbit-drag`, `pan-drag`, `zoom-wheel`, `pick-instance` | wgpu emits `interactionHover`/`interactionSelect`/`setCamera` but not React's `noteShellCommand`/`noteWorldNavigation`, and misses `setCamera` on orbit/pick | world camera-report lane (a peer is refactoring exactly this as `WorldCameraSync`) |
| `context-menu` | wgpu emits `worldContextMenuAt` where React emits `interactionSelect` + `setCamera` | world context-menu lane |
| `context-menu-dismiss`, `chord-escape` | React dispatches `engagementAbort` to the guest; the wgpu shell closes its overlays locally and dispatches nothing — the verb has NO wgpu dispatcher at all (`grep engagementAbort` finds none) | shell chord lane, next packet |
| `chord-undo`, `chord-redo` | React dispatches `undo`/`redo`. The wgpu chord exists and dispatches through the same funnel, but `mod+z` was swallowed by the text buffer's own undo — fixed in this packet (`🧊️renderer/🦀️.rs`'s `undo_text_operation` now requires a FOCUSED control) and NOT yet re-measured, because run 10's build was contaminated | re-measure |
| `example-switch`, `example-picker-dismiss`, `role-{viewer,editor}` | wgpu reaches the controls (`exact`) and `setActiveExample` fires on the switch, but the guest's `registerBrushMesh` re-publication and the role-switch verbs do not | example/role lane |

## 9. Gates

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ clean, no new warnings (`🗑️generated/w9c-native-check-2.txt`) |
| `cargo check -p … --lib --target wasm32-unknown-unknown -j 4` | ✅ clean (`🗑️generated/w9c-wasm-check.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib` (pane chrome · panel anchor · introspection · navbar/footer · shell input · text) | ✅ **172 passed, 0 failed** (`🗑️generated/w9c-tests-6.txt`) |
| `cargo test -p semio-framework-os-infinite --lib` (world raster · interaction authority) | ✅ **5 passed, 0 failed** (`🗑️generated/w9c-tests-world.txt`) |
| `nx run @semio-tech/framework-renderer-wgpu:wasm` + `activate-puzzle3d-wgpu-dev` | ✅ (`🗑️generated/w9c-wasm-build-9.txt`) |
| parity probe, both renderers | ✅ `🗑️generated/parity-run-9/parity.md` |

🩺️ **Pre-existing hang, not this packet's**: `shell::panel_anchor_model_tests::host_panel_action_is_claimed_before_guest_and_preserves_the_session_roster_and_home_projection`
never returns — it parks forever inside `block_on(dispatch_action(controller_id: "foreign.controller"))`,
the leg its own comment calls "reaches the missing guest fixture". Sampled at 12 min: main parked on
`recv`, that test's thread parked on a dispatch semaphore, every pool worker idle. It touches nothing
this packet changed (no chrome walk, no pane chip, no world resource), and the runs above `--skip` it.

🧪️ **Laws added**

| law | file |
| --- | --- |
| `a_reference_underlay_is_offered_every_render_and_a_refused_admission_is_not_a_fault` | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` |
| `an_intent_aimed_outside_the_surface_retires_instead_of_faulting_the_frame` (+ the middle button) | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` |
| `chrome_surface_census_publishes_react_levels_sorted_and_deduplicated` | `🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` |
| `the_chrome_surface_census_separates_panes_from_docked_panels` | `🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` |
| `every_pane_chip_id_decodes_back_to_its_own_window_and_chip` | `🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs` |
| `a_pane_chips_hit_row_is_registered_after_the_panels_it_must_outrank` | `🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs` |
| `prepared_world_resources_are_send_and_deduplicate_uploads` — corrected a red-at-HEAD expectation (a raster is a PRODUCER, not an upload) | `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` |

🗄️ **Note for whoever runs next**: the shared build dir hit **397 MiB free** mid-packet
(`⚡️cache/cargo/build/debug` at 227 GiB, its `incremental` alone 53 GiB). Clearing
`debug/incremental` recovered 41 GiB and costs only recompile time; `CARGO_INCREMENTAL=0` keeps it from
regrowing under a fleet.
