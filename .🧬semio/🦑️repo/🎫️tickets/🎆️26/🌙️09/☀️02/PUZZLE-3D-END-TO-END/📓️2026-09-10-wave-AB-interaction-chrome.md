# Wave W-AB — interaction chrome (items 4 / 6 / 8)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Probe `$T/🔍️browser-probe.ts`. Selection publication is W-G3 — not this wave. Serve `:6014` (wasm **#35**). Never killed a process. Never rebuilt wasm.

Repo MCP was not available.

## Verdict table

| # | Item | Verdict | Artifacts | Notes |
| --- | --- | --- | --- | --- |
| 4 | Export/Import | **EXPORT PASS; IMPORT MENU → PICKER LANDED (rides #37)** | native 2/2 picker laws | Numbered Import was bare `importFixture {}`. File-menu row is now `openImportFixture` → `RequestFileOpen` → host re-dispatch `{payload}` into `importFixture`. |
| Frame | Per-window Frame | **PASS** (reframe after census) | `10-54-49-brush-framed.png` | AABB control unchanged. Probe frames twice after `instanceCount≥1`. Recapture: table fills Perspective; `10-31-36` right-edge overshoot gone. |
| 6 | Brush | **GUEST `vorticesJson` EMPTY ON #35; ASSEMBLER FIX LANDED** | native 3/3; `10-54-49` | Forest store has 11 vortices. Selected-mode assembler omitted them at idle; brush never published. Guest now publishes on `brush`/`volumeBrush`. Wasm #35 still `vortexCount=0`. Rides #36. |
| 8 | Suggestions | **BLOCKED ON EMPTY MARKERS (#35)** | `10-54-49` menus=1 suggestRows=0 | Same empty `vorticesJson`. Select-to-reveal at default idle is reserved-lane (W-G3). Brush-arming reveal is the host-exerciseable path after #36. |

## Vortex marker hit-testing (items 6 / 8)

Investigated on host source against the last live recapture (`10-00-36` / `10-05-52`) where Frame filled Perspective and Brush was red, yet history never showed `addBrushObject` and suggestions stayed Workspace Menu.

### 1. How markers are rendered

`WorldVortexMarkers` draws, per vortex:

- a **hit-proxy** sphere at `vortex.radius ?? 0.36`
- a **visible point** sphere at `radius * 0.18` (~0.065)
- shaft + cone arrow

Pointer handlers (`onPointerOver` → `handleVortexHover`, `onClick` → `onBrushPlace` in brush mode) sit on the proxy. The proxy was `visible={false}`. Three.js `Raycaster.intersectObject` returns immediately when `object.visible === false`, so the large proxy **never participated in picking**. Only the 0.065 visible point could hit — and only if nothing closer sat on the ray.

`data-vortices-json` and a `WorldVortexHitStamp` (`data-vortex-hits` with projected `sx/sy`) now sit on the Perspective host so the probe can dump world vs screen positions and click them.

### 2. Brush clicks vs marquee

Host `onPointerDown` still starts a marquee path. Capture arms only after `world3dMarqueePointerCaptureArmed` (4 px). A click without drag does **not** steal the canvas pointer from r3f. Marquee is not the miss.

### 3. The steal: instance GLB still raycasts when pick is blocked

`worldInstancePickBlocked("brush")` is true. That sets `blockPick` → `pickEnabled=false` → instance `onClick`/`onPointerMove` no-op **without** `stopPropagation`. r3f delivers events to the **nearest** intersection and bubbles up the **parent** chain, not along the remaining hit list. The Forest table is a GLB sibling **in front of** the vortex group. The table won the raycast; the vortex sibling never saw hover or click.

`blockPick`'s comment said this was to keep vortex clicks from selecting the table — it muted the table callback and thereby made the table an opaque raycast shield.

### Fix (host-side)

- `worldVortexHitProxy(radius)` — `{ visible: true, radius: max(radius, 0.36) }`. Proxy stays on the raycaster; camera-invisible via `opacity: 0`.
- `worldInstanceMeshRaycast(pickEnabled)` — `undefined` (default) when pick is on; `() => null` when blocked.
- `applyWorldInstanceMeshRaycast(root, pickEnabled, Mesh.prototype.raycast)` — walks GLB meshes so url-instances pass through in brush/fill.
- Wired on `PaintTexturedMesh`, fallback box, and `GlbInstanceMesh` (`useLayoutEffect` on the cloned scene).

Laws (direct World3dHost import; **6 passed | 506 skipped**):

- `keeps the vortex hit proxy visible so Three's raycaster does not skip it`
- `clears instance mesh raycasts when pick is blocked so sibling vortex markers stay hittable`

Probe (`$T/🔍️browser-probe.ts`) dumps `data-vortex-hits` and clicks on-screen projections after Frame + Brush.

### Browser recapture

`:6014` bun PID still listens. After the World3dHost edit, HTTP stayed **000** (25 s timeout) for several minutes. Did not kill. Did not rebuild wasm. Re-run `--brush --suggestions` once the serve returns 200.

If a later recapture sees `[DEBUG]`-class evidence of `addBrushObject` / `openVortexSuggestions` without publication, record **dispatch-proven, blocked-on-reserved-lane** (W-G3 / wasm #36).

## Camera / Frame

Orbit wheel dollies along the look target. Frame uses `world3dFrameCameraFromBounds` on the instances-group AABB and keeps look direction. `10-00-36-frame-perspective.png` — table fills Perspective.

## Export (item 4)

Workspace-menu Export PASS (`09-48-02`). Dimmed WindowChrome inherited `pointer-events: none`; leaf `pointer-events-auto` + active ContextMenuChrome fixed it.

## Files changed

- `🌐️World3dHost` — hit proxy visible, instance/GLB raycast pass-through when pick blocked, `data-vortices-json`, `WorldVortexHitStamp`
- `🔬️engine-contract` — `worldVortexHitProxy` + `worldInstanceMeshRaycast` / `applyWorldInstanceMeshRaycast` laws
- `$T/🔍️browser-probe.ts` — vortex hit dump + click
- this report

Temporary `[DEBUG]` taps on hover/click/`addBrushObject` were stripped after the laws passed; serve had not returned 200 for a recapture.

## Laws / suites

From the renderer react package:

`SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long "../../../../🧪️tests/🔬️engine-contract/🟦️.ts" --run --testNamePattern="keeps the vortex hit proxy visible|clears instance mesh raycasts"`

Last focused run this turn: **6 passed | 506 skipped**.

## Rebuild / restart flags

- No wasm deploy. No process killed.
- `:6014` HTTP 000 after World3dHost HMR; listener still up. Stopped per wedge rule.

## Empty `vorticesJson` (guest assembler, this turn)

Coordinator DOM dump on `:6014` wasm **#35** (Concrete Forest, default and brush-mode) showed both World3dHost windows carrying `data-vortices-json="[]"`. Host hit-testing is landed; there is nothing to hit. Root cause is guest-side visibility, not a missing store record and not a paged-payload drop.

### 1. Concrete Forest store has vortices

`seed-left-001` is the placed Forest **table instance**, not a generated leftover. It lives in the example DSL:

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio`

The `objects` table lists **11** vortices (`seed-left-001:v0` … `v10`, radius `0.36`, kinds `b-l` / `b-l-m` / `b-s` / `b-s-m` / `c-b` / `c-t`). The same ids also appear on the kind-catalog object "Hexagonal Cut Concrete Forest Left". The unit suite's default `app()` fixture is this example (`initial_snapshot_is_the_concrete_forest_fixture`). Existing async law `world_vortices_reveal_in_selected_mode_only_for_the_selected_object` already asserts the fixture "must expose vortices".

This is **not** reserved-lane. Publication is not waiting on a generation step or a W-G3 job. The records are already in the store at boot.

### 2. Assembler omitted them on purpose (Selected + idle)

`world_vortices_json` in `🎭️modes/✏️edit/📡️windows/🧊️main/🦀️.rs` gated markers with:

`runtime.vortex_show == ALWAYS || interaction.touches_object(object)`

Default `vortex_show` is **`selected`** (`default_vortex_show` → `PUZZLE3D_VORTEX_SHOW_SELECTED`). Boot has no selection and no hover, so JSON is `[]` — the Selected-mode contract the existing async test encodes.

Waves K/P paged residuals did **not** strip the field. `render()` always assigned `scene.vortices_json`; the payload was an intentional empty array.

Brush arming did **not** flip visibility. Combined with host `worldInstancePickBlocked("brush")` (table mesh no longer selectable while brush is armed), the user cannot select the table to reveal vortices, then cannot hit vortices to place — a deadlock. Select-to-reveal at default idle also depends on `interaction.touches_object` and therefore on the reserved selection lane (W-G3 / wasm #36). Brush-arming reveal does **not**.

### 3. Fix (guest, rides rebuild #36)

`world_vortices_json` / `object_vortices_visible` now take `active_utility`. Markers publish when:

- `vortex_show` is Always, or
- the object (or one of its vortices) is selected/hovered, or
- `active_utility` is `brush` or `volumeBrush`

Selected + idle / transform still yields `[]`. `render()` passes `envelope.active_utility`.

Native laws next to the assembler (`vortex_payload_laws`, Forest-shaped store with **11** vortices):

- `world_vortices_json_carries_store_vortices_when_show_is_always` — Always + empty interaction + idle utility → **11**
- `world_vortices_json_carries_store_vortices_when_brush_is_armed` — Selected + empty interaction + `brush` / `volumeBrush` → **11**
- `world_vortices_json_stays_empty_in_selected_mode_without_a_touch_or_brush` — Selected + `""` / `transform` → **0**

Cargo-test (package `semio-s-artifact-puzzle-3d`, `--features component-app-assembly --lib vortex_payload_laws`): **3 passed; 0 failed; 647 filtered out**. Output: `$T/🗑️generated/vortex-payload-laws-cargo.txt`.

Wasm **#35** will keep serving `[]` until rebuild **#36**. Did not rebuild.

### 4. Frame overshoot (`10-31-36-brush-framed.png`)

The Frame **control** was not changed. `10-31-36` framed while the AABB raced scene load. `$T/🔍️browser-probe.ts` now waits until Perspective `data-instances-json` length ≥ 1, frames, waits again, and frames a second time (`frameForestTableAfterCensus`). Used on `--frame`, `--brush`, and `--suggestions`. Dump also logs `vortexCount`.

Recapture `10-54-49` on `:6014` wasm **#35** (`--interact --frame --brush --suggestions`): census settled at `instanceCount=1 vortexCount=0` on every poll (default and after Brush arm / `Set Active Utility`). Second Frame ran. `10-54-49-brush-framed.png` — table fills Perspective; the `10-31-36` right-edge crop is gone. Vortex hits stayed `{vortices:0,hits:[]}`. History has `Set Active Utility` plus later `Select`/`Hover` (table clicks), no `addBrushObject`. Suggestions opened the workspace menu (`menus=1 suggestRows=0`). Expected on #35 until rebuild **#36** ships the guest assembler.

Log: `$T/🗑️generated/probe-wave-ab-vortices-reframe.txt` and `probe-2026-09-10T10-54-49.md`.

### Files this turn

- `✏️editor/🎭️modes/✏️edit/📡️windows/🧊️main/🦀️.rs` — brush/volumeBrush publish + 3 native laws
- `$T/🔍️browser-probe.ts` — re-frame after census
- this report

## Rebuild / restart flags (this turn)

- No wasm deploy. No process killed.
- Guest assembler fix rides coordinator rebuild **#36** (after W-G3 reserved-lane conversion).
- `:6014` HTTP 200 throughout this turn (wasm #35). Probe confirmed `vortexCount=0` after census and after Brush arm.

## Workspace-menu Import file picker (item 4, this turn)

Numbered workspace-menu **5 Import** dispatched `importFixture {}` with no file chooser. Empty args abort inside `importFixture` (notice, no edit), so checklist item 4 could not complete from the menu. Export already used a host `DownloadMediaExport` effect; Import already had the mirror and the host already handled it — the menu row pointed at the wrong action.

### 1. Dual catalog

`✏️editor/🦀️.rs` registered both in `category("file")`:

- `exportFixture` — Shell, label Export (download effect)
- `importFixture` — Mutation, label Import — this was the numbered row
- `openImportFixture` — Shell, label Import… — already emitted `Effect::RequestFileOpen` (`accept` JSON, `read_as` text, `import_action` `importFixture`, `multiple` false)

`🎮️commands/🗂️open-import-fixture/🦀️.rs` was the canonical picker command. `importFixture` applies `{ payload }` (also `json` / `fixture`). Host re-dispatch after pick is the only chrome caller.

### 2. Host already had the TS half

No ShellHost / PluginRuntime edit. Coordinator undo/spawn-job traces left alone.

- `🛠️ShellHelpers.requestFileOpen` — `<input type="file">`, reads text (or data URL)
- `dispatchOpenedFiles` — re-dispatches `importAction` with `{ payload, name }`
- `🏛️ShellHost` `applyHostEffects` (`"requestFileOpen" in effect`) — runs the picker and feeds `importFixture`

Same shape as Export's download effect. No TS law added.

### 3. Guest fix (rides rebuild #37)

Clean single path, no empty-`importFixture`-opens-picker fallback:

- `importFixture` stays registered (host completion) but `.in_palette(false)` and no `category("file")`
- `openImportFixture` label is **Import** / **Importieren** (no ellipsis) and keeps `category("file")`

Menu row → `openImportFixture` → `RequestFileOpen` → picker → `{ payload, name }` → `importFixture` → document contains the imported objects.

### 4. Native laws

Next to the existing W-Y export/import tests in `✏️editor/🧪tests/🔬️unit/🦀️.rs`:

- `file_menu_import_row_opens_the_file_picker` — file-category ids are `exportFixture` + `openImportFixture`; `importFixture` is dispatchable, not a file-menu row, `in_palette` false
- `open_import_fixture_requests_file_open_then_import_applies_payload` — dispatch Import emits `RequestFileOpen` (`import_action` `importFixture`, JSON accept, text, not multiple); then `importFixture` + `{ payload }` reproduces `object_cores`

Package `semio-s-artifact-puzzle-3d`, `--features component-app-assembly --lib`, `RUST_MIN_STACK=134217728`:

- `file_menu_import`: **1 passed; 0 failed; 651 filtered out** (0.07s)
- `open_import_fixture_requests`: **1 passed; 0 failed; 651 filtered out** (3.17s)

Output: `$T/🗑️generated/import-file-picker-laws-cargo.txt`.

Existing `import_fixture_reproduces_the_exported_document` still applies the payload, then asserts one undo restores empty. That undo assertion failed this turn (`left: 1`, `right: 0`) — W-G3 history work riding the same #37 rebuild, not this picker path. The new apply law does not undo.

### Files this turn

- `✏️editor/🦀️.rs` — file-menu Import is the picker; `importFixture` is completion-only
- `✏️editor/🧪tests/🔬️unit/🦀️.rs` — two native laws
- this report

### Rebuild / restart flags (Import picker)

- No wasm deploy. No process killed. No modifying git.
- Guest catalog change rides coordinator rebuild **#37** (with W-G3 undo).
- Host picker was already live via vite; no TS change.
- No new `[DEBUG]` taps.

## Empty `setActiveTool` mid-fill bounce (W-G3 §8.8–8.9)

Fill-tab arming worked; mid-flow the guest still emitted `Effect::SetActiveTool { tool_id: "" }`. Host `applyHostEffects` mirrored that as `SET_ACTIVE_TOOL null` and disarmed the tool that had just been armed. Escape is `engagementAbort`, which rewrote `scene.active_utility` to the empty default while `config.active_tool_id` was still `fill` — `dispatch_step` treated that as leaving fill and bounced.

### True source (guest)

`Puzzle3dActionPrologue::dispatch_step` compared `active_utility_initial` / `next_active_utility` to `fill` and emitted empty `SetActiveTool` on any leave. Fill is a mode-level tool, not a window utility. Leaving it is exclusively a host `setActiveTool ""`. An empty tool effect from a utility-field rewrite is the bounce.

### Fix (guest, rides #37)

- Emit `SetActiveTool { fill }` only when entering fill. Never emit an empty tool id from this prologue.
- A real utility change still emits `SetActiveUtility` (host mutual exclusion clears the tool when the utility is non-empty).
- `engagementAbort` no longer rewrites `active_utility` while fill is armed (clears engagement scratch only).

### Law

`fill_flow_does_not_emit_empty_set_active_tool` — arm fill, then `fillBuildTick` and `engagementAbort` must not emit empty `SetActiveTool`.

Package `semio-s-artifact-puzzle-3d`, `--features component-app-assembly --lib`: **1 passed; 0 failed; 652 filtered out** (0.14s). Output: `$T/🗑️generated/fill-empty-setactivetool-laws-cargo.txt`.

## `requestContextMenu` skips the host tool overlay (W-G3 §8.8–8.9)

Windowed `requestContextMenu` used `windowViewContext` alone. That projection drops `activeToolId` (fails-before in the existing window-view-context fixture). Action dispatch already used `hostArmedViewContext`. Right-click during an armed tool therefore asked the plugin for a menu without the host-owned tool, so the overlay section the utility-bar / `injectActiveTool` path includes never appeared.

### Fix (host TS, live via vite)

ShellHost `requestContextMenu`: windowed requests now use `hostArmedViewContext(base, activeToolIdRef.current, windowInstanceId)`. Panel path still `panelViewContext(injectActiveTool(...))`. No PluginRuntime edit. Coordinator undo/spawn-job traces left alone.

### Law

engine-contract `windowed context-menu view overlays the armed tool like handleAction`: `windowViewContext` has no `activeToolId`; `hostArmedViewContext(..., "fill", "left")` has `"fill"`.

`SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` engine-contract `--run --testNamePattern="windowed context-menu view overlays"`: **1 passed | 512 skipped**. Output: `$T/🗑️generated/context-menu-armed-view-vitest.txt`.

Same assertion also landed in manifest `window-view-context` TS.

## Browser (`:6014`, wasm #36)

`--interact --fill --port=6014 --settle=2`. Serve stayed HTTP 200. Fill tab selected; `Cancel fill` appeared after End; `fillBuildTick` counted up (26 to 29). Count stayed 0 / no Isolated spawn-job this run (not this defect). Guest bounce fix is not on wasm #36 — rides #37. Context-menu host overlay is live via vite; this probe's context-menu step runs before Fill arm, so it does not recapture an armed-tool right-click. Log: `$T/🗑️generated/probe-wave-ab-fill-context.txt` and `probe-2026-09-10T11-35-05.md`.

### Files this turn

- editor prologue — no empty SetActiveTool from utility-field leave
- engagement-abort — do not clobber fill
- editor unit tests — native bounce law
- ShellHost — windowed context menu uses hostArmedViewContext
- engine-contract + window-view-context TS — armed-tool overlay law
- this report

### Rebuild / restart flags

- No wasm deploy. No process killed. No modifying git.
- Guest bounce fix rides coordinator rebuild #37.
- Host context-menu overlay is live via vite on #36.
- No new `[DEBUG]` taps.


## #38 browser proof battery (2026-09-10, :6014)

Harness-managed serve stayed **HTTP 200** before, during, and after. No serve kill, no wasm rebuild, no modifying git. Probe: `bun 🔍️browser-probe.ts --interact --brush --suggestions --fill --importexport --port=6014` (combined), then a vite-live re-verify of `--brush --suggestions` after the host hover/context-menu fix. Logs: `$T/🗑️generated/probe-wave-ab-38-battery.txt`, `$T/🗑️generated/probe-2026-09-10T13-52-33.md`, `$T/🗑️generated/probe-wave-ab-38-reverify.txt`, `$T/🗑️generated/probe-2026-09-10T14-10-34.md`.

Ignore undo/redo (W-G3). Combined run still recorded spawn-admit faults on `interactionHover` / `interactionSelect` / `noteShellCommand` — those reserved-slot misses are the shared guest/runtime stall W-G3 is on; they are cited here only where they blocked a W-AB item.

### 1. Vortex markers — publication + render **browser-proven**; click → `addBrushObject` **defect**

Brush arm on Concrete Forest / `seed-left-001` published **11** vortices (`seed-left-001:v0`…`v10`, `rawLen=2640`) after ~6 polls (Selected+idle still `[]` by design). All 11 projected on-screen. Markers visible on the table in `probe-2026-09-10T13-52-33-brush-vortices.png` / `-brush-stroke.png` (white pins, Brush checked).

Click did **not** place a brush object: `addBrushObject` history rows `[]`, no scene/census delta. Host hit-testing reached the markers (`interactionSelect` / `interactionHover` payloads named `seed-left-001:v3`, `v6`, `v1`). Those actions died in guest **spawn-admit** (`framework route '…' has no exact pending spawn slot`). `handleBrushPlace` only dispatches when guest has published `brushPreviewJson`; that preview never arrived, so the host correctly refused a partial `BrushPlacePayload` (`add_brush_object` no-ops without a full payload).

**Guest.** Root: reserved spawn-admit starves `interactionHover`/`interactionSelect`, so the assembler never publishes a brush preview. Native place path is already law-covered. **Needs #39** (and the W-G3 reserved-slot stall).

### 2. Suggestions — **defect** (workspace steal fixed host-side; menu still missing)

First battery: hover + Alt+right-click opened the **Workspace Menu** (`menus=1`, `suggestRows=0`) — screenshot `probe-2026-09-10T13-52-33-suggestions-open.png`. Two host faults stacked:

1. `onContextMenu` returned on `altKey` **without** `preventDefault`, so chrome stole the gesture.
2. `hoveredVortexFullIdRef` was written by local marker hover, then a `useEffect` overwrote it with guest `interaction.hoveredVortexFullId` (`null` after spawn-admit miss), so `world3dSuggestionsGestureArmed` was false and `openVortexSuggestions` never dispatched.

**Host fix (vite-live):**

- `world3dRetainLocalVortexHover(local, guest)` — keep the local marker hover when guest hover stays null.
- `world3dSuggestionsGestureConsumesContextMenu(altKey)` — Alt+right-click always consumes `contextmenu`.
- `useEffect` sync uses the retain helper. `onContextMenu` prevents default/stop when Alt is held.

engine-contract `arms the suggestions gesture from a host vortex hover without a guest InteractionView`: **1 passed | 834 skipped** (includes retain + consume assertions).

Re-verify (`--brush --suggestions`, HTTP 200): workspace menu **gone** (`menus=0`, `probe-2026-09-10T14-10-34-suggestions-open.png` shows markers + Brush, no chrome menu). Suggestions menu still did not open (`suggestRows=0`). `openVortexSuggestions` never appeared in the probe console. Hover/select still spawn-admit-faulted on `seed-left-001:v1`. Guest still never published `suggestionMenu`.

**Remaining guest.** Host no longer steals the gesture. Opening the menu still needs a guest-admitted `openVortexSuggestions` (or a published hover). **Needs #39**.

### 3. Fill bounce — **browser-proven**

After Fill tab arm (`panel.tab=tool.fill`, Count 0): Escape twice → `emptyBounce=0`, `stillFill=true`. No empty `SetActiveTool` in the abort window. Panel text kept Fill; tab moved to `framework.category.tool` (tool category, not a null-tool bounce). `cancelStill=false` because fill never produced a plan (`ticks=0`, `count=0`) — out of scope for this bounce check.

Native law already on this ticket: `fill_flow_does_not_emit_empty_set_active_tool` **1/0/652**. Screenshot: `probe-2026-09-10T13-52-33-fill-abort-engagement.png`.

### 4. Import round trip — **defect**

Menu row 5 is `openImportFixture` (`5 Import`, `pe=auto`). Click settled with **`effects:0`**. Playwright `filechooser` = **none**. No `importFixture` history row, no windows/census import delta. Export earlier in the same step still downloaded `puzzle-3d.json`.

Host `requestFileOpen` path is already live (`"requestFileOpen" in effect` → picker → re-dispatch `importFixture`). Guest **source** `open_import_fixture` pushes `Effect::RequestFileOpen` (`import_action` `importFixture`, JSON accept, text, not multiple). Native laws already passed this ticket (`file_menu_import_row_opens_the_file_picker`, `open_import_fixture_requests_file_open_then_import_applies_payload`).

**Guest / #38 wasm.** Browser invocation completed without the effect the native law emits. **Needs #39** recapture of that catalog path.

### Host files this battery

- `World3dHost` — retain local vortex hover; consume Alt+contextmenu
- engine-contract — retain + consume laws on the existing suggestions-gesture test
- `🔍️browser-probe.ts` — brush vortex wait, suggestions hover+Alt+right-click on a hit, fill-abort `emptyBounce`, Import `openImportFixture` + chooser log (ticket input, kept)
- this report

### Rebuild / restart flags

- No wasm deploy. No process killed. Serve stayed HTTP 200.
- Host hover/context-menu fix is live via vite on #38.
- Guest spawn-admit + Import `effects:0` need **#39**.
- No new `[DEBUG]` taps. Coordinator undo/spawn-job traces left alone.


## #38 guest defects — spawn-admit, suggestions, RequestFileOpen wire (W-AB, current source)

Undo stays W-G3 §8.18. These three #38 battery leftovers were guest-side. Root-caused and fixed in current source; proven with guest laws (no wasm rebuild). Coordinator rebuild **#39** recaptures them. W-G3 still owns `noteShellCommand` inverses / `ReplayShellCommand` decode on the same guest files — their Replay arm was left intact; `RequestFileOpen` was added on the same `decode_wire_effect` table (their §8.19).

### 1. Vortex click / hover — spawn-admit drop

**Drop point.** `dispatch_framework_reserved_action` admits the Isolated reserved job, then requires `pending_reserved.can_insert(job)`. That registry is `ArtifactFixedRegistry` with 64 slots; `can_insert(id)` is empty-slot `id % 64`. A vortex pointermove storm admits dozens of `interactionHover`s (then `interactionSelect`) before Isolated jobs finish. Exclusive hash slots fill or collide, and spawn-admit returns `framework route 'interactionHover|interactionSelect' has no exact pending spawn slot`. The host still hit-tests; brush preview / `addBrushObject` never publish because `dispatch_interaction_action` only commits hover/selection on **completed** reserved jobs, and `world_brush_preview_json` reads that hover (or `suggestion_menu.vortex_full_id`).

Earlier single `interactionSelect`s (job 28, 89/90-era) completed because they were one-at-a-time, not a hover burst. Window addressing and payload kind were not the difference.

**Fix.** `VcsArtifactApp::retire_pending_reserved_latest_wins` runs immediately after admit, before `can_insert`, and only for `INTERACTION_ACTION_IDS`: retire same-verb pending jobs, and any other interaction occupying `job % 64`. Evicted permits `finish()`. `noteShellCommand` is not in that set (W-G3).

**Law.** `vortex_hover_storm_admits_then_brush_preview_and_place` — 70 unsettled `interactionHover`s on `first_vortex_full_id`, settle the latest, settle a following `interactionSelect`, `suggestionsTick` × `PUZZLE3D_BRUSH_PICKER_TICKS`, assert `brush_preview_of` has `targetVortexFullId`, `addBrushObject` from that preview increases `object_count`.

```
test editor::puzzle3d::component::tests::vortex_hover_storm_admits_then_brush_preview_and_place ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 653 filtered out; finished in 0.63s
```

Stdout: `🗑️generated/w-ab-vortex-hover-laws.txt`.

### 2. Suggestions menu — same hover admit, then guest publication

Host Alt+right-click / retain-local-hover already landed on #38 (vite-live). The menu still needed a guest-admitted `openVortexSuggestions` after a hover that actually commits. Same latest-wins storm path as (1); then `{ fullId, x, y }` publishes `suggestionMenu.open`.

**Law.** `vortex_hover_then_open_vortex_suggestions_publishes_menu` — same 70-hover storm, settle latest, `openVortexSuggestions` with the hovered `fullId`, assert `suggestionMenu.open == true` and `vortexFullId` matches.

```
test editor::puzzle3d::component::tests::vortex_hover_then_open_vortex_suggestions_publishes_menu ... ok
```

Same stdout file as (1).

### 3. `openImportFixture` `effects:0` — wire-effect table (W-G3 §8.19)

Native `dispatch()` + `settle_into` folds `take_typed_operation_effect()` in-process — no wire — so `open_import_fixture_requests_file_open_then_import_applies_payload` already emitted `RequestFileOpen`. Browser first `performInvocation` of Migrated `openImportFixture` is typed admit (`effects:0`); the picker rides completion → `encode_wire_serialized` → `decode_wire_effect`.

**Drop point.** `decode_wire_effect` tried `from_dsl_value::<Effect>` then only W-G3's `ReplayShellCommand` fallback; anything else was `Err(())` and the host never saw the picker (`filechooser=none`). Same silent-drop family as Replay.

**Fix.** `decode_wire_request_file_open` before the Replay arm: discriminants `requestFileOpen` / `RequestFileOpen`; fields `req` (number or `{id}`), `accept`, `readAs`/`read_as`, `importAction`/`import_action`, `multiple`. Replay lines untouched.

**Laws.**

```
test component::reactor::turn::wire_effect_laws::request_file_open_survives_wire_effect_round_trip ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 640 filtered out; finished in 0.00s
```

Stdout: `🗑️generated/w-ab-request-file-open-wire-law.txt`.

Native emit still holds:

```
test editor::puzzle3d::component::tests::open_import_fixture_requests_file_open_then_import_applies_payload ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 654 filtered out; finished in 2.71s
```

Stdout: `🗑️generated/w-ab-import-file-open-law.txt`.

### Guest files touched

- `🔌️plugin/🦀️.rs` — `ArtifactFixedRegistry::each_id`; `retire_pending_reserved_latest_wins` + call site after reserved admit; `settle_framework_reserved_admission` visible to dependent guest laws (not `cfg(test)`-only).
- `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — `decode_wire_request_file_open` on the same table as W-G3 Replay; `wire_effect_laws::request_file_open_survives_wire_effect_round_trip`.
- puzzle 3d `✏️editor/🧪tests/🔬️testkit/🦀️.rs` — `dispatch_reserved_unsettled`, `hover_id_unsettled`, `select_id_unsettled`, `settle_reserved`.
- puzzle 3d `✏️editor/🧪tests/🔬️unit/🦀️.rs` — storm → preview/place law; storm → `openVortexSuggestions` menu law.

### Rebuild / restart flags

- No wasm deploy. No serve kill. No modifying git.
- Coordinator rebuild **#39** after W-AB + W-G3 both report.
- W-G3 `ReplayShellCommand` / `noteShellCommand` / `drive_framework_reserved_spawn_like_host` left in place.
- Coordinator `[DEBUG]` undo/spawn-job traces left alone. No new `[DEBUG]` taps.


## #39 browser proof (W-AB, :6014)

Harness-managed serve stayed **HTTP 200** before, during, and after. No serve kill, no wasm rebuild, no modifying git. Probe: `bun 🔍️browser-probe.ts --interact --example=Concrete --brush --suggestions --importexport --port=6014`. Combined runs: `🗑️generated/w-ab-39-probe-stdout.txt`, `w-ab-39-reverify-stdout.txt`, `w-ab-39-reverify2-stdout.txt`. Latest screenshots: `probe-2026-09-10T16-16-15-*.png`. W-G3 undo/redo/selection/clipboard/gumball ignored.

#39 wasm carried the three guest fixes (latest-wins reserved-slot retirement, suggestion-menu publication, `RequestFileOpen` wire-decode). Remaining host gaps were patched vite-live on this turn.

### Verdict table

| Item | Verdict | Evidence |
|---|---|---|
| 1. Vortex hover-storm → preview → `addBrushObject` | **Partial.** Latest-wins **browser-proven**. Preview/place **not landed**. | 70 rapid `pointermove`s over 11 onscreen `seed-left-001:v*` hits. Brush step `newFaults=none` / last run **faults=0** (vs #38 `no exact pending spawn slot`). `addBrushObject` history `[]`, instances stayed `{count:1,ids:[seed-left-001]}`. `data-brush-preview-json` `hasAttr=false` (live DOM did not show the new attr). Guest laws already prove storm → preview → place. **Needs #40** if preview still unpublished after host tick is in the served bundle. |
| 2. Suggestions menu | **Defect.** Gesture reached the canvas; menu did not open. | Last run: 11 onscreen vortices, hover + Alt+right-click, `menus=0` `suggestRows=0` `suggestionMenu.open=false`, **faults=0** (history panel closed so the earlier `history-panel.commands` fixed-capacity fault is gone). Guest law `vortex_hover_then_open_vortex_suggestions_publishes_menu` already **ok**. **Needs #40** for wasm recapture if `openVortexSuggestions` still does not publish; host already dispatches on Alt+hover `fullId`. |
| 3. Import `RequestFileOpen` → picker | **Picker browser-proven.** Fixture re-dispatch **not observed** as a census/history delta. | `import chooser=yes` on both re-verifies; `setFiles` of the just-exported Concrete Forest json. Instances stayed `seed-left-001`; history had no `importFixture` row (same-document reimport). Host `wireEffectToFriendly` was missing `request-file-open` (default drop) — vite-live case added; picker then fired. |

### 1. Vortex click / hover-storm

#38 drop was spawn-admit after a hover burst. On #39 the storm no longer faults `interactionHover`/`interactionSelect` (`framework route … no exact pending spawn slot` is gone). That is the latest-wins reserved-slot retirement.

Place still needs a published brush preview. Guest `world_brush_preview_json` wants picker ticks; the host only ran `suggestionsTick` while `suggestionMenu.pending`. Vite-live: tick also while `brushMode`. The live page never showed `data-brush-preview-json` (`hasAttr=false`), so that World3dHost edit may not have attached via HMR — we cannot restart serve. After the storm + 2.5s settle + click, `addBrushObject` still did not land.

Screenshots: `probe-2026-09-10T16-16-15-brush-hover-storm.png`, `…-brush-stroke.png`, `…-brush-vortices.png`.

### 2. Suggestions

Host Alt+right-click consume + retain-local-hover were already on #38. Closing the history panel after the brush step removed the `ui.fixed-capacity: history-panel.commands` hover faults. The menu still did not open (`menus=0`). Guest publication law already passed in source. **Needs #40** to recapture `openVortexSuggestions` → `suggestionMenu.open` on the served wasm if the host dispatch is reaching the guest.

Screenshot: `probe-2026-09-10T16-16-15-suggestions-open.png`.

### 3. Import picker

#38 settled `openImportFixture` with `effects:0` / `filechooser=none`. Guest `decode_wire_effect` now has a `RequestFileOpen` arm; the host leftover table did not — `wireEffectToFriendly` default-dropped `request-file-open`. Vite-live case added (WIT `req` + `params.accept` / `read-as` / `import-action` / `multiple` → `{ requestFileOpen }`). Re-verify: **`import chooser=yes`**, `setFiles` of `probe-2026-09-10T16-16-15-export.json`. Same-scene reimport did not add an `importFixture` history row or instance delta.

Screenshot: `probe-2026-09-10T16-16-15-export-import.png`.

### Host files this proof

- `PluginRuntime` — `wireEffectToFriendly` `request-file-open` (W-G3 `replay-shell-command` left intact)
- `World3dHost` — `suggestionsTick` while brush is armed; `data-brush-preview-json` / `data-suggestion-menu-json` observability
- `🔍️browser-probe.ts` — hover-storm, preview/instance dumps, Import `--import` alias + late chooser, suggestions hover-only + menu dump

### Rebuild / restart flags

- No wasm deploy. No process killed. Serve stayed HTTP 200.
- Host `request-file-open` map + brush ticks are vite-live in source; picker recapture confirmed.
- Preview/place + suggestions menu still need **#40** if they stay dark after the next wasm/host bundle.
- Coordinator `[DEBUG]` undo/spawn-job traces left alone. Added `[DEBUG] request-file-open mapped`.

## #39 law-vs-browser hop trace (W-AB, 2026-09-10 evening)

Guest laws for storm→preview→place and hover→`openVortexSuggestions`→menu are already green **in wasm #39**. This section names the browser hops on `:6014` (HTTP 200, harness-managed). No serve kill, no wasm rebuild.

Probes: `🗑️generated/w-ab-39-hops2-stdout.txt` + `probe-2026-09-10T16-25-18.md` (post retain-hover / place-on-pointerdown; **faults=0**, `activeUtility=brush`); `🗑️generated/w-ab-39-hops3-stdout.txt` + `probe-2026-09-10T16-54-16.md` (post Alt-hold / hop-census / canvas.click; **faults=28**).

### Verdict

| Item | Dying hop | Layer | #40? |
|---|---|---|---|
| 1. Vortex click → preview / `addBrushObject` | Guest `interactionJson` never grows `hoveredVortexFullId`; `data-vortices-json` `hovered=[]` after 55–58 `interactionHover` admits. Preview `rawLen=0`. `brush-place hop` count **0** — R3F marker `onPointerDown`/`onClick` never ran `handleBrushPlace`. | **Outside the law entry.** Law drives reserved `hover_id_unsettled` → `settle_reserved` → ticks → `addBrushObject(preview)`. Browser Interactive `interactionHover` settles `effects:0` (hops2) or **faults** `ui.fixed-capacity` at `history-panel.commands` (hops3). Host cannot invent a full `addBrushObject` payload. | **Needs #40** — hop name: **`interactionHover` Interactive settle does not commit `Puzzle3dInteractionSnapshot.hovered` (and/or UI-admit dies at `history-panel.commands`)** |
| 2. Suggestions menu | hops2: Alt+right opened the **workspace** Actions menu (`menus=1`, Export/Import/…), not vortex suggestions (`data-suggestion-menu-json` `open=false`). hops3: rightdown/contextmenu counts did **not** increment (stayed 2 from Import chrome); `openVortexSuggestions` **0**. Guest never published a menu payload. Host mount is not the miss — publication never arrives. | **Outside the law entry.** Law dispatches `openVortexSuggestions` with `fullId`. Browser never dispatches that action (Alt missing on the event, or the right-click never hit `WorldOrbitGated` / host `onContextMenu`). | Host vite-live now tracks `Alt` via keydown and treats brush+local-hover right-click as the gesture. **Re-verify once brush stays armed.** If `openVortexSuggestions` is dispatched and the menu JSON still stays empty → **needs #40** hop **`openVortexSuggestions` leftover → `suggestionMenu`**. |
| 3. Same-file reimport | Picker **browser-proven**. `request-file-open mapped` `importAction=importFixture`, `import-picker opened=1` (`7526` bytes). Instances stayed `{seed-left-001}`; no `importFixture` history row. | Host re-dispatch **did** get a non-empty `importAction`. Remaining miss is guest no-op on same fixture **or** `dispatchOpenedFiles` → `importFixture` not showing in history. | Not a wasm rebuild by itself. Next proof: `performInvocation actionId=importFixture` after `setFiles`. |

### 1. Brush place — hop-by-hop

DOM → raycast: hover-storm **hits**. hops2/hops3 both log `vortex-hover hop fullId=seed-left-001:v*` then `fullId=null keep=v*`. Local retain works.

Host dispatch: hops3 `interactionHover dispatch` **58** with `domain=vortex` `gran=vortex` `id=seed-left-001:v*`. Command ingress Interactive. hops2: 55 `interactionHover` **settled** `frames=Invocation,Ephemeral` `effects:0` — **no** `spawn-job` leftover for `framework.reserved.tool` per hover.

Guest snapshot: `data-interaction-json` keys are `activeUtility, brushCandidateIndex, fillBuild, gridFactor, meshResidency, meshReuploadUrls, revealCutoffs, suggestionMenu, voxelDims` — **no `hoveredVortexFullId`**. `world_interaction_json` only inserts that field when `hovered_vortex_full_id` resolves. `data-vortices-json` `hovered=[]`.

Preview: `world_brush_preview_json` needs `active_utility=brush` **and** `puzzle3d_brush_target_vortex` (selection or hover) **and** `session.brush_preview`. hops2 had utility `brush` and still `rawLen=0` (hover/selection never committed). hops3 had utility **`select`** (brush chip click did not stick after HMR / history-panel pressure) so `brushMode=false` and `handleBrushPlace` never ran (`brush-place hop=0` after canvas-relative click).

Place click: hops2 `page.mouse.click(box+sx)` at 1013,426 aiming v0. hops3 `canvas.click({position})` at 793,302. Neither produced `brush-place hop`. hops3 also logged `action failed interactionSelect` with **empty `targets: []`** (empty-click select) plus `ui.fixed-capacity: history-panel.commands`.

Law vs browser: the law never walks Interactive-lane leftover with a live `history-panel.commands` page. That is the same class of gap as undo (wrong door / mailbox / seq-filter): **laws green, browser dies one layer outside the law's reserved settle.**

### 2. Suggestions — hop-by-hop

Hover commits? **Host local ref yes, guest publication no** (same as §1).

Alt+right action: hops2 produced the **workspace** context menu (`Set Active Example` … `Export` …) because `event.altKey` was false — `onContextMenu` took the chrome branch. hops3 `keyboard.down(Alt)` + `modifiers:["Alt"]` produced **no** new `suggestions-rightdown` / `suggestions-contextmenu` (`census` stayed at 2 from the Import right-click) and `menus=0`. `openVortexSuggestions` never left the host.

Guest menu payload: `data-suggestion-menu-json` `rawLen=0` `open=false`. Host `ContextMenuController` had nothing to mount.

Host vite-live this round: `world3dSuggestionsAltHeld` (keydown `Alt`/`AltGraph`), brush+local-hover right-click also arms the gesture, DEBUG includes `held=` / `brush=`.

### 3. Import re-dispatch

`wireEffectToFriendly` `request-file-open` maps `params.importAction=importFixture` (keys `accept, readAs, importAction, multiple`). ShellHost `import-picker hop` `resolved=importFixture`, `opened=1`. Host now falls back to `importFixture` if the field is empty. Same-file reimport still shows no census/history delta — next hop to name is whether `dispatchOpenedFiles` actually `performInvocation`s `importFixture` in the release serve (not visible as a settled action in this console tail).

### Host files this hop trace

- `World3dHost` — retain-local-hover (no wipe on `pointerOut`); place-on-`pointerDown`; `suggestionsTick` while brush; Alt held-ref; brush+hover right-click consume; `[DEBUG] interactionHover dispatch` / `brush-place hop` / `suggestions-rightdown` / `suggestions-contextmenu`
- `ShellHost` — `import-picker` DEBUG + empty-`importAction` → `importFixture` (coordinator undo traces untouched)
- `PluginRuntime` — existing `request-file-open` map (W-G3 replay arm untouched)
- `🔍️browser-probe.ts` — hop census, vortex hovered flags, raw interaction keys, canvas-relative click, Alt keydown + modifiers

### Rebuild / restart flags

- No wasm deploy. No process killed. Serve stayed HTTP 200.
- Another rebuild **does not** fix hops2's dark preview while hover Interactive-settles with `effects:0`. That is the #39 law-vs-browser gap.
- **Needs #40** only for the named guest hop: **`interactionHover` / `interactionSelect` do not persist vortex ids into the snapshot the scene assembler reads (Interactive `effects:0`), and/or their leftover UI-admit faults at `history-panel.commands`.**
- Suggestions: host gesture wiring is vite-live; prove `openVortexSuggestions` dispatch on the next recapture before calling it #40.


---

## W-AB 2026-09-10 — import census delta + `history-panel.commands` capacity

Left the guest interaction-commit / hover-persist lane to W-G3 (§8.21). Closed the two remaining W-AB items on serve `:6014` (HTTP 200, never killed). No wasm rebuild. No modifying git.

### 1. Import census delta — root cause and working-case proof

**Ingress is not the hole.** Same-file and distinct-file both reach the guest with `payload` (string) + `name` + `windowId`.

| hop | name | payloadLen | instances after | history |
| --- | --- | ---: | --- | --- |
| same-file reimport of the live Concrete export | `probe-2026-09-10T17-11-23-export.json` | 7526 | still `{count:1, ids:[seed-left-001]}` | no import/create-object row (5 chrome rows unchanged) |
| distinct fixture (cloned object `probe-distinct-…` + origin+4) | `probe-2026-09-10T17-11-23-distinct.json` | 9177 | **`{count:2, ids:[seed-left-001, probe-distinct-2026-09-10T17-11-23]}`** | **`entry.16=create-object … id=probe-distinct-…`**; Check In 1→2; treeItems 9→12 |

`importFixture` itself does **not** dedupe. `import_fixture` always assigns `ctx.scene.fixture`. The no-delta on same-file is a **store identity fold**: identical live content emits `history_patch=none`. Law `import_fixture_of_the_live_document_records_whether_identical_content_is_an_edit` printed `[DEBUG] importFixture same-content history_patch=false` and now asserts that. Empty→payload still records one mutation (existing `import_fixture_reproduces_the_exported_document`).

Host ingress hop (vite-live, `PluginRuntime.performInvocation`): `[DEBUG] importFixture ingress {name, payloadType, payloadLen, argKeys}`. Probe hop-census now counts it. Distinct run: `import-picker hop=2`, `importFixture ingress=2`, `request-file-open mapped=2`, `faults=0`.

Working case proven in the browser: `bun 🔍️browser-probe.ts --interact --example=Concrete --importexport --port=6014` → `🗑️generated/w-ab-import-distinct-stdout.txt` + `probe-2026-09-10T17-11-23.md`. Distinct `setFiles` of a 2-object fixture produced the census/windows/history delta. Same-file silence is correct store behavior, not a dead commit.

### 2. `ui.fixed-capacity` at `history-panel.commands` — sized from the live row count

**Site:** `ui_history_panel` pushed every filtered `HistoryView.commands` row into one `BuiltChildren` (`UI_BUILT_CHILDREN_MAX = 32`). `try_push` maps to `ui.fixed-capacity: fixed UI admission failed at history-panel.commands`. hops3 left History open; 28 hovers then aborted every later publish. Bumping 32→64 is the class that failed before.

**Fix (needs #40):** page command rows with the same `UI_BUILT_CHILDREN_MAX`-ary tree `paged_text_carrier` already uses. The bound is the live filtered command-row count, chunked by the live page arity — not a new ceiling. ≤32 stays a flat Commands section (existing filter/clip laws unchanged). 39 rows → 2 pages (32+7).

Language-agnostic fixture `plugin/fixtures/history-panel-command-pages/🔣️.json` pins `{pageArity:32, overflowPastArity:7, commandRowCount:39, expectedCommandSectionChildren:2}`. Law `ui_history_panel_pages_command_rows_from_the_live_count` asserts fixture arity == `UI_BUILT_CHILDREN_MAX`, assembles 39 Interaction rows, walks every `framework.history.entry.*` key, and checks page children 32+7. **Passed** native `cargo test -p semio-framework-plugin --lib ui_history_panel_pages_command_rows_from_the_live_count` (`🗑️generated/w-ab-history-panel-pages-law.txt`).

Browser still runs wasm **#39**, so this guest page is **needs #40** (coordinator builds after W-G3 reports). Do not bump `UI_BUILT_CHILDREN_MAX`.

### Files this turn

- plugin `ui_history_panel` — `page_history_command_nodes` + Commands section consumes the paged live row list
- plugin-builder-contract unit — paging law
- `history-panel-command-pages/🔣️.json` — derived bound
- puzzle editor unit — same-content import is a store no-op
- `PluginRuntime` — `[DEBUG] importFixture ingress`
- `browser-probe.ts` — distinct-fixture import + ingress hop census

### Rebuild / restart flags

- No serve kill. `:6014` stayed HTTP 200. Vite-live host hops are in the distinct-import recapture.
- Guest paging + same-content import assert: **needs #40**. Native laws already green.
- Interaction hover/preview/place/suggestions: still W-G3 §8.21. Not reopened.

## #40 retest — leftover hover / place / suggestions (2026-09-10 18:26)

Rebuild **#40** on harness-managed `:6014` (HTTP 200, never killed). Probe `bun 🔍️browser-probe.ts --interact --example=Concrete --brush --suggestions --port=6014` → `🗑️generated/w-ab-40-retest-stdout.txt` + `probe-2026-09-10T18-26-01.md`. First #40 capture (pre-overlay) was `🗑️generated/w-ab-40-brush-sugg-stdout.txt` + `probe-2026-09-10T17-54-19.md`.

W-G3 leftover InteractionView **does** publish vortex-domain hover. Named hop from 17:54 was the **host leftover overlay**: `mergeWorldSelectionWithLeftover` returned the base scene when `selectedIds=[]`, and `interaction` was parsed only from `scene.interactionJson` — so `hoveredVortexFullId` never appeared even when leftover `hoverTarget` carried `seed-left-001:v*`. DEBUG on leftover previously omitted `hoverTarget`, which hid the publication.

### Verdicts

| lane | verdict | hop |
| --- | --- | --- |
| 1. Vortex hover → leftover vortex id → `hoveredVortexFullId` | **PARTIAL** | Leftover `hoverTarget` is `{domain:vortex, channel:pointer, id:seed-left-001:v*}` on settled Isolated leftovers (md lines after job 245+). After vite-live overlay, click-settle published `hoveredVortexFullId` on the interaction record (`hover=seed-left-001:v0`, keys include `hoveredVortexFullId`). Storm sample still saw `hover=null` because earlier leftover frames completed with `hoverTarget:null` (pointer-leave / coalesced empty) before the vortex leftover landed. |
| 1b. Preview renders | **FAIL** | `data-brush-preview-json` `rawLen=0` before storm, after storm, and after click. Dying hop is now **guest `brushPreviewJson`**, not leftover hover ids. Leftover peel overlays InteractionView; `world_brush_preview_json` still needs a world render with precompute ticks. Host must not invent `addBrushObject` args. |
| 2. Vortex click → `addBrushObject` | **FAIL** | `brush-place hop=2` with `preview=null utility=brush hover=seed-left-001:v0`. Host local hover + leftover overlay know the vortex. Place stays gated on guest preview. `addBrushObject=0`, instances still `{count:1,ids:[seed-left-001]}`. |
| 3. Suggestions Alt+right-click | **FAIL** | `suggestions-rightdown=0`, `suggestions-contextmenu=0`, `openVortexSuggestions=0`, menu `open:false`. Gesture never incremented. Host Alt-held + leftover/`hoveredVortexFullId` are live (`suggestions pre-gesture` had `hover=seed-left-001:v6`). Dying hop: **pointer right-down / contextmenu never reaches `World3dHost`**. |

Brush arm still works: 11 vortices after Brush (`seed-left-001:v0`…`v10`). `faults=0`. Serve stayed 200.

### Guest law for #41 (native green, no wasm this turn)

`interaction_hover_leftover_carries_vortex_full_id` — reserved `interactionHover` settle leftover `hoverTarget.id` is the vortex full id, domain `vortex`, and the next scene render projects `hoveredVortexFullId`. Passed `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib interaction_hover_leftover_carries_vortex_full_id`. W-G3 `selectedIds`/`locked`/`gumball` leftover left intact.

Preview/place still need a world republish of `brushPreviewJson` after leftover hover (existing `vortex_hover_storm_admits_then_brush_preview_and_place` already does settle + `suggestionsTick` ticks + render). That is the #41 guest hop if overlay `hoveredVortexFullId` is not enough for the host place gate.

### Host vite-live (no serve kill)

- `mergeWorldSelectionWithLeftoverV1` applies when leftover has **ids or hoveredId** (hover-only leftover is the `interactionHover` shape).
- `mergeWorldInteractionWithLeftoverV1` / `leftoverHoveredVortexFullIdV1` copy leftover vortex-domain `objectId:vortexId` onto `interaction.hoveredVortexFullId`.
- `data-interaction-json` serializes the leftover-merged interaction so the probe can see the overlay.
- Leftover DEBUG now includes `hoverTarget`.
- Engine-contract: `hover-only leftover InteractionView overlays vortex hover without selected ids`.

### Rebuild / restart flags

- No serve kill. No wasm rebuild. No modifying git.
- Guest leftover vortex-id law: **needs #41** only to ride the next guest binary; #40 already publishes `hoverTarget` for vortex full ids.
- Preview/place/suggestions still dark on #40 after the host overlay.

---

## 2026-09-10 W-AB last fix wave — preview lane + suggestions capture

Closed both named hops in current source. W-G3 inspection / editor / translate-selection left untouched. No serve kill. No wasm rebuild. Guest rides **#41**. Probe `🗑generated/w-ab-41-brush-sugg-stdout.txt` + `🗑generated/probe-2026-09-10T18-50-36.md`. Serve `:6014` stayed HTTP 200.

### Hop 1 — `brushPreviewJson` never publishes (`rawLen=0`)

**Root cause (guest scene publication, not an Effect encode gap).** `world_brush_preview_json` is not gated on an engagement session. It already runs in brush mode (or with a one-shot `suggestion_menu`) and follows `puzzle3d_brush_target_vortex` (selected vortex, else leftover/hover full id). `session.brush_preview` then returns `None` until the brush cache has a **free** candidate for that vortex.

That cache is filled by `suggestionsTick` → `refresh_brush_candidates` (500 µs slices). The browser leftover path stores hover on InteractionView only. `brushPreviewJson` is an **optional World3d scene lane** (`framework.scene.world3d.brushPreview`), not an Effect — §8.21 leftover completeness cannot carry it. Leftover peel overlays `hoveredVortexFullId` on the host; the last Full world body still has an absent optional lane (`rawLen=0`). The existing storm law hid this by **click-selecting before ticks**. Hover-committed leftover never did that.

**Fix (guest, #41):** `suggestions_tick` now finishes the live hover/menu target on this tick (up to 8 extra slices while `free` is empty and `unknown_pending`). The dirty world body then encodes the `brushPreview` lane. Host `brushObjectPlacementArgs` still returns null without a parsed preview — click will dispatch real `addBrushObject` with `targetVortexFullId` from that payload, never a fabricated one.

**Fix (host vite-live):** leftover `hoveredVortexFullId` in brush mode immediately `dispatch("suggestionsTick")` so the first post-hover tick sees guest hover (the hover-hop tick used to race leftover settle).

**Laws (native green):**
- `hover_committed_in_brush_publishes_preview` — brush + leftover hover settle + ticks, **no select** → preview `targetVortexFullId` is the hovered vortex.
- `hover_committed_click_places_via_published_preview` — that preview's `addBrushObject` increments object count.
- `vortex_hover_storm_admits_then_brush_preview_and_place` still passes.

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib hover_committed` — 2 passed. Storm law re-run — passed.

**:6014 #40 wasm:** `verdict brush-preview-place FAIL [expect-41]` — `rawLen=0`, `addBrushObject=0`, `brush-place hop preview=null utility=brush hover=seed-left-001:v0`. Host correctly refused to invent place args. Preview/place go green on #41.

### Hop 2 — suggestions right-down never reaches World3dHost (`suggestions-rightdown=0`)

**Root cause.** Alt-tracking was already armed. `handleWorldOrbitRightPointerDown` only ran from orbit's **canvas** capture listener. Chrome / context-menu overlay is the event target, so the canvas (and the host `onContextMenu` React handler) never see `pointerdown` button=2. Window bubble is too late; host `onPointerDown` already ignores `button !== 0`.

**Fix (host vite-live):** module-level `window` **capture** listeners for `pointerdown` (button=2) and `contextmenu`. Each mounted `World3dHost` registers a route. Hit-test uses the host rect **and** its canvases (`world3dSuggestionsEventOverHost`). Armed gesture (`Alt`+hover or brush+hover) dispatches `openVortexSuggestions` and stops the event so orbit/chrome cannot steal it. Contextmenu dedupes if capture already dispatched (80 ms). Helper `world3dSuggestionsRightDownRoutesOnWindowCapture(button, overHost)` — true only for button=2 over the host.

**Law:** engine-contract `arms the suggestions gesture from a host vortex hover without a guest InteractionView` now also asserts the capture predicate. `SEMIO_TEST_LEVEL=long` vitest — 1 passed.

**:6014 re-probe (vite-live confirmed):** after hover `seed-left-001:v1`, Alt+right-click at 1236,320:

`suggestions hop-census: {suggestions-rightdown:3, suggestions-contextmenu:1, openVortexSuggestions:4, …}`

Right-down **reaches World3dHost** and `openVortexSuggestions` dispatches. Menu dump still `{rawLen:0, open:false}` on #40 guest — menu body rides #41 with the preview tick. Host hop is closed.

### Verdicts (this wave)

| lane | source | :6014 #40 |
| --- | --- | --- |
| Hover-committed → `brushPreviewJson` non-empty | **CLOSED** (guest laws + tick) | FAIL until #41 wasm |
| Click → real `addBrushObject` with hovered vortex id | **CLOSED** (guest law; host still null-gates) | FAIL until #41 preview lane |
| Alt+right-down → World3dHost → `openVortexSuggestions` | **CLOSED** (vite-live) | `rightdown=3`, `openVortexSuggestions=4` |
| Menu `suggestionMenu.open` | guest `openVortexSuggestions` already law-green; needs #41 binary + preview tick | `open:false` on #40 |

### Rebuild / restart flags

- No serve kill. No wasm rebuild. No modifying git.
- Coordinator: build **#41** after this report (last fix wave before the final battery).

---

## 2026-09-10 W-AB — #41 battery isolation (brush / suggestions / import)

Coordinator: rebuild **#41** live on `:6014` (served `core.wasm` hash == disk, 20:56). Guest `suggestionsTick` extra slices + hover-id leftover are in the running wasm. Battery `--battery` (`probe-2026-09-10T18-57-42`) still failed the three W-AB lanes. This turn isolated each lane (standalone, then Nakagin sequence), fixed probe + vite-live host, and stopped when a forced composite refresh wedged.

Serve stayed harness-managed HTTP 200. No wasm rebuild. No modifying git. W-G3 inspection / clipboard / locked / gumball / translate-selection left untouched.

### Battery vs standalone (browser evidence)

| lane | battery (`18:57`, Nakagin 180) | isolated Concrete | Nakagin sequence / isolate |
| --- | --- | --- | --- |
| `brush-preview-place` | FAIL `instances=180 preview=null`, `utility:select`, `vortices=0` | Arm works (`utility=brush`, 11 vortices, leftover hover `seed-left-001:v*`, many `suggestionsTick` catalog-complete). **`data-brush-preview-json` still `rawLen=0`**, `laneKids=[]`, `addBrushObject=0`. Probe reads `#puzzle3d-main-perspective [data-brush-preview-json]` (`hasAttr:true`) — not a wrong-attr miss. | `#brush` click **does** reach the host (`aria-pressed=true`, `setActiveUtility hop window=puzzle3d-main-perspective next=brush body=puzzle3d.play.composite`). Guest interaction stays `select`, vortices stay `[]` for 20s+. A second hop `next=` (echo-off) follows the arm. Tick budget under 180 is **not** the first cause — Brush never publishes on Nakagin. |
| `suggestions` | FAIL `menus=0 rows=0 hops=0` | After arm: `rightdown=3`, `openVortexSuggestions=4` (window-capture **survived** the serve recycle). `menus=0 suggestRows=0` — hops closed, menu body still empty. | Unarmed (`utility:select`, `hover=null`, `vortex onscreen=0`): capture still logs `rightdown=4` but `openVortexSuggestions=0` because guest `open_vortex_suggestions` returns when `fullId` is missing. Battery `hops=0` was the console **hard cap** (`length < 4000` dropped later DEBUG) plus no local hover. |
| `import-distinct` | FAIL `before=180 after=180`, `chooser=none`, `ingress hops=[]` | PASS `1→2` (`probe-distinct-…` id appears). Same-file reimport is identity-fold (no census delta) — expected. | Menu row `openImportFixture` is enabled and clicked. Guest `performInvocation` **starts** (`seq=326/337`) and **never settles** — no `request-file-open mapped`, no `import-picker hop`. Distinct fixture is written but never imported. Not a 180-instance capacity fold; the picker never opens. |

`battery-faults` / boot / example / undo / redo were already PASS (`treeItems=90–106`). `error: suggestionsTick` in page console is split `[DEBUG] plugin_exchange actionId=` lines, **not** a panic.

### Probe-side fixes (`🔍️browser-probe.ts`)

- Console ring buffer (`shift` at 4000) so battery no longer drops late `suggestions-rightdown` / `import-picker` DEBUG.
- `armBrushUtility`: unfold perspective utilities, **one** Playwright force click on `#brush` (ToggleGroup `kind=multiple` — a second click deactivates). Poll up to 20s. Log `#brush` metrics + `aria-pressed`.
- `dumpBrushPreview` scans every `[data-brush-preview-json]` and reports `laneKids` / `hosts`. Confirmed empty carriers, not a wrong DOM attr.
- Preview poll after hover-storm (12×400 ms).
- Distinct import: execute/textarea fallback when chooser is none; filechooser wait 25s.

Concrete `--suggestions` and `--import` went green on hops / census after these fixes. Battery wiring was the first hop for suggestions `hops=0` and import `chooser=none` on Concrete; Nakagin import is a real guest hang.

### Host vite-live (kept)

- Echo-off guard on `setActiveUtility`: a follow-up deactivate within 8s of an arm is ignored (`lastUtilityArmAtRef`). Measured pair: `next=brush` then `next=` from ToggleGroup remount / picker `utilityId:""` while host `aria-pressed` is already true.
- Prior wave still live: leftover hover merge, window-capture suggestions right-down, leftover-hover `suggestionsTick`.

### Host attempt that wedged — reverted

Forcing `refreshUi` of `windowBodies: [puzzle3d.play.composite]` after `setActiveUtility` (cache-bust so a view-state-only switch cannot hash-omit the world) **wedged** the Nakagin isolate (`nakagin-brush5`, no stdout after 9 min; vite `:6014` stayed up). That composite is the 180-instance body. The force-refresh was **reverted**; echo-off guard kept. Do not retry a Full/composite refresh on Nakagin without a cheaper guest encode.

### Genuine guest gaps → needs **#42**

1. **Hover-committed preview still empty on #41 wasm (Concrete).** Armed brush + leftover `hoverTarget` + `suggestionsTick` catalog-complete + extra slices in source → `brushPreviewJson` absent, `laneKids=[]`. Native `hover_committed_in_brush_publishes_preview` / `hover_committed_click_places_via_published_preview` stay green. Browser path does not publish the optional lane (wasm 500 µs slices never produce a `free` candidate, or the encode never attaches the carrier). Host must not fabricate `addBrushObject`. Existing laws stand; #42 needs a wasm-aware law (leftover hover + ticks → non-empty `brushPreview` lane in the retained document, not only in-process `dispatch`).
2. **Nakagin `setActiveUtility` does not republish `interaction.activeUtility` / vortices.** Host session is `brush` (`aria-pressed=true`). Guest scene stays `select`, `vorticesJson="[]"`. Shared body key `puzzle3d.play.composite`. A host Full refresh of that body hung. #42: utility switch must dirty/encode the perspective world (or a cheap interaction+vortices pair) without a Full composite rebuild; law = Nakagin-scale `setActiveUtility(brush)` publishes `activeUtility=brush` and a non-empty vortices lane.
3. **`openVortexSuggestions` hops without a menu.** Concrete: `openVortexSuggestions=4`, `suggestionMenu.open=false`, `menus=0`. Guest sets `runtime.suggestion_menu` then `refresh_brush_candidates`; menu lane stays closed in the browser. Law already covers in-process open; #42 needs the browser leftover+capture path to project `suggestionMenu.open`.
4. **`openImportFixture` does not finish under Nakagin load.** Quiet + not in `puzzle3d_action_uses_precompute`, yet the Interactive job starts and never emits `RequestFileOpen` (Concrete emits `effects:1` in ~3s). #42: law that `openImportFixture` on a 180-object document returns `RequestFileOpen` inside the first job step / does not wait on world encode.

### Probe artifacts

- Battery: `🗑generated/probe-2026-09-10T18-57-42.md`
- Concrete brush: `🗑generated/w-ab-41-brush-concrete.txt` / `probe-2026-09-10T19-13-33.md`
- Concrete suggestions: `🗑generated/w-ab-41-sugg-concrete.txt` / `probe-2026-09-10T19-15-44.md`
- Concrete import: `🗑generated/w-ab-41-import-concrete.txt` / `probe-2026-09-10T19-17-06.md`
- Nakagin sequence: `🗑generated/w-ab-41-nakagin-seq.txt` / `probe-2026-09-10T19-18-15.md`
- Nakagin brush isolates: `w-ab-41-nakagin-brush.txt` … `brush4.txt` (brush5 wedged, no file)

### Rebuild / restart flags

- No serve kill. Vite `:6014` left running after the composite-refresh wedge was reverted.
- No wasm rebuild. Guest preview / Nakagin utility / Nakagin import / suggestions menu → **needs #42**.

---

## 2026-09-10 W-AB — #42 guest taps + derived publication fixes (source only)

Coordinator: #42 without new guest code changes nothing. Laws were already green on the serving wasm; the browser path diverges. This turn landed `[DEBUG]` eprintln taps on the publication path and three derived fixes so the imminent #42 binary can both *do* the work and *name* the remaining gates. No wasm rebuild. No serve kill. No modifying git. W-G3 inspection / lock / clipboard left untouched.

Law stdout: `🗑generated/w-ab-42-guest-laws.txt` — `set_active_utility*` (2), `open_import_fixture*` (1), `hover_committed*` (2), `vortex_hover_storm*` (1) all ok after the gate-tap edit.

### Guest files touched

| file | what landed |
| --- | --- |
| `✏️editor/🦀️.rs` | `setActiveUtility` / `setActiveTool` → `Puzzle3dScopeClass::Viewport` (world body, not Full chrome). `openImportFixture` → `puzzle3d_shell_only_emit` (`RequestFileOpen` + `UiDirtyScope::None`, skips 180-object `scene_from_snapshot`). Prologue `sync_step` while-loop capped at 8 turns. Taps on utility publish + import/sync. |
| `✏️editor/🎭️modes/✏️edit/🪟windows/🧊️main/🦀️.rs` | Full `world_brush_preview_json` + lane/vortices/menu publication taps with named gates. |
| `✏️editor/🎮️commands/⏱️suggestions-tick/🦀️.rs` | Enter/exit + cache before/after + slice count. Extra refresh slices (up to 7) already present; taps now name live-target priority (`menu` pin else brush hover). |
| `✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs` | Missing-`fullId` tap. **7 extra `refresh_brush_candidates` slices** after the first (same budget as tick) so the menu payload and preview share a warm cache. |
| `editor/commands/open-import-fixture` | Command-arm enter tap (dead path once shell-only wins; kept so a law that still reaches the arm is visible). |
| `✏️editor/⏳️precompute/🦀️.rs` | Public `refresh_brush_candidates` + `brush_preview` lookup taps (`hit` / `free` / `pending`). |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | New law `set_active_utility_dirties_the_world_body`. |

### Coordinator grep — exact `[DEBUG]` prefixes

```
[DEBUG] puzzle3d.brushPreview.hover
[DEBUG] puzzle3d.brushPreview.gate
[DEBUG] puzzle3d.brushPreview.cache
[DEBUG] puzzle3d.brushPreview.compute
[DEBUG] puzzle3d.brushPreview.lane
[DEBUG] puzzle3d.vortices.publish
[DEBUG] puzzle3d.utility.publish
[DEBUG] puzzle3d.suggestionsTick.enter
[DEBUG] puzzle3d.suggestionsTick.exit
[DEBUG] puzzle3d.openVortex.enter
[DEBUG] puzzle3d.openVortex.cache
[DEBUG] puzzle3d.openImport.enter
[DEBUG] puzzle3d.openImport.step
[DEBUG] puzzle3d.openImport.exit
[DEBUG] puzzle3d.prologue.sync
```

Gate `reason=` values on `brushPreview.gate`: `no-target` (`brush` / `menu` / `utility=` named), `no-free-candidate` (`vortex=` / `free=` / `pending=` / `index=`). Lane tap names `utility=` / `preview=` (byte length, 0 = gated) / `vortices=`. Vortices tap names `brush_or_volume=`. Utility publish names `action=` / `window=` / `utility=` / `map_hit=`. Tick enter names live-target priority: `menu=` then `target=`. Import `openImport.*` is gated to `action==openImportFixture` so it does not flood other verbs; `prologue.sync` still fires for any action that enters the sync loop. Shell-only success is `exit path=shell-only`. Hang of the old path is `exit hang-point=sync-budget`.

Idle select-mode `world_brush_preview_json` still returns `None` with **no** tap (would flood every frame).

### Fixes landed

1. **Nakagin utility switch trigger.** `setActiveUtility` was absent from `puzzle3d_command_scope_class` → defaulted to **Chrome / Full**. Host Full+cache-bust of `puzzle3d.play.composite` hung under 180 instances. It now dirties **Viewport** (`window_bodies: [main::BODY_KEY]`) so `interaction.activeUtility` + vortices republish without a Full chrome rebuild. `setActiveTool` same class. Law: `set_active_utility_dirties_the_world_body`. Existing `set_active_utility_emits_no_ops_and_no_history_entry` stays green (Viewport is a dirty scope, not a document mutation / SetActiveUtility effect).
2. **Suggestions menu vs the same cache the preview needs.** `open_vortex_suggestions` used to invalidate + one 500 µs refresh, then publish `suggestionMenu.open=true` with `free=[]`. It now spends up to **8 slices** (1 + 7) on that vortex — same loop as `suggestionsTick` — and taps `openVortex.cache` / `brushPreview.cache` with `free` / `pending` / `slices`. Menu JSON build taps `openVortex.cache menu vortex= pending= candidates=`.
3. **Import hang.** `openImportFixture` now short-circuits in `puzzle3d_shell_only_emit` (`RequestFileOpen` req 121, `importFixture`) and never runs `scene_from_snapshot` / the unbounded `while prologue.sync_step`. Remaining sync loops for precompute-gated actions are hard-capped at **8** turns (`hang-point=sync-budget`). Existing `open_import_fixture_requests_file_open_then_import_applies_payload` stays green via the shell-only path.

### Fixes deferred (honest — #42 battery reads the taps)

- Wasm 500 µs `refresh_brush_candidates` can still leave `free=0` after 8 slices. Browser will then show `brushPreview.gate reason=no-free-candidate` and `lane preview=0`. Do not fabricate a host `addBrushObject`. Existing hover-committed laws stay the in-process model; the tap names the browser miss.
- Menu body stays empty if that same cache is still empty after the extra slices (`candidates=0` on `openVortex.cache menu`).
- `setActiveExample` remains **Chrome**. Publication decision is tapped (`utility.publish action=setActiveExample`) but the example switch itself was not retargeted — Nakagin isolation failed on **utility**, not example.
- Command-arm `open_import_fixture` is unreachable once shell-only wins; kept as a tap + law backstop.
- No wasm rebuild this turn. Serving `:6014` still runs #41 until the coordinator builds #42.

### Rebuild / restart flags

- No serve kill. No wasm rebuild. No modifying git.
- Coordinator: build **#42** now — W-AB guest taps + Viewport utility + shell-only import + extra open-vortex slices are in current source.

---

## 2026-09-10 W-AB — #42 tap readout

Source: `🗑️generated/probe-2026-09-10T20-22-03.md` (867s, :6014 #42 wasm with W-AB taps). Suggestions **PASS**. W-G3 `interactionSelect` 4096-continuation fault ignored. No serve kill. No wasm rebuild. No modifying git.

### Brush-preview-place — tap-by-tap

| hop | evidence | died? |
| --- | --- | --- |
| Host `#brush` click | `aria-pressed=true`, `setActiveUtility hop next=brush` seq 348/357 Interactive | no |
| Guest `utility.publish action=setActiveUtility` | **absent**. Only `utility.publish action=setCamera` (4×), `map_hit=false` | **YES — guest handle never ran** |
| `brushPreview.hover` / `.gate` / `.cache` / `.compute` | **0** — those fire only when `brush \|\| menu` | skipped (utility stayed select) |
| `suggestionsTick.enter/exit` | **0** | skipped |
| `brushPreview.lane` | 21× `preview=0` `brush_or_volume=false` `vortices=318–369` | gated: utility not brush |
| `vortices.publish` | same 21×; DOM after arm `rawLen=2` (`[]`) | host/DOM empty vs small guest publishes on other renders |
| Probe | `utility=select` for 20s, `vortex onscreen=0`, `preview rawLen=0`, `addBrushObject=0` | FAIL `instances=180 preview=null` |

**Dying hop:** Interactive `performInvocation setActiveUtility` never reaches `handle_action_impl` / `utility.publish`. Actor `puzzle#1` is already in `settle … more-work` (1024→4096) encoding the 180-instance world body. Viewport dirty of `main::BODY_KEY` (composite) is the same hang if the handle ever ran. Preview gate never entered (`!brush && !menu`).

### Import-distinct — tap-by-tap

| hop | evidence | died? |
| --- | --- | --- |
| Menu click | `openImportFixture` performInvocation seq 367, 373 Interactive | no |
| Guest `openImport.enter` / `.exit path=shell-only` | **0** (taps are in this wasm; they would print if handle ran) | **YES — same hung ingress** |
| `request-file-open mapped` / `import-picker hop` | **0** | picker never armed |
| Probe chooser | `chooser=none` both same-file and distinct; `importFixture ingress=0` | `missed chooser and execute` |
| Census | before=180 after=180 | FAIL |

Shell-only `RequestFileOpen` did **not** settle — guest never ran. Distinct execute fallback looked for a dialog that does not exist.

### Fixes landed (vite-live, re-verified)

1. **Leftover `activeUtility`** — `mergeWorldInteractionWithLeftoverV1` overlays host `brush` onto `data-interaction-json` immediately on `setActiveUtility` (does not wait for guest republish). Echo-off restored to **8s** so ToggleGroup remount `next=` cannot deactivate. Law: engine-contract `leftover activeUtility overlays guest select without a hover id` (1 passed, `SEMIO_TEST_LEVEL=long`).
2. **Import picker host-arm** — `onAction openImportFixture` opens `requestFileOpen` in the shell, **returns** (does not wait for guest `openImport.*`), then re-dispatches `importFixture` with the file payload. `[DEBUG] import-picker hop host-arm openImportFixture`.

### Vite-live re-verify (Concrete, :6014, no wasm)

Probes: `🗑generated/w-ab-42-reverify-concrete.txt` + `🗑️generated/probe-2026-09-10T20-55-06.md` (`--import --brush`); `🗑generated/w-ab-42-reverify-import-only.txt` + `🗑️generated/probe-2026-09-10T20-59-37.md` (`--import` only). Serve stayed HTTP 200.

#### Brush leftover (vite-live closed; preview still #43)

| hop | measured | died? |
| --- | --- | --- |
| Host `#brush` click | `aria-pressed=true`, leftover poll 0 `utility=brush` | no — leftover overlay works |
| Guest `utility.publish action=setActiveUtility` / `brushPreview.*` / `suggestionsTick.*` | **0** (same as battery) | guest handle still never ran |
| Vortices / preview | `rawLen=2` (`[]`), `preview rawLen=0`, `hover=null`, `addBrushObject=0` | leftover cannot invent vortices or preview JSON |
| Actor | `v102_1` `poll$1` fault during `frame-perspective` (`Window is not responding` / `Agent disconnected`) | guest dead before place |

**Dying hop unchanged:** Interactive `setActiveUtility` never reaches `handle_action_impl`. Leftover made the probe read `utility=brush`; scene-lane publication (`vortices` / `brushPreview`) stayed empty. Gate variable is not a host preview-length bug — `preview=0` because `!brush && !menu` on the guest body.

#### Import host-arm (chooser closed; apply still #43)

| hop | battery 20-22-03 | vite-live Concrete |
| --- | --- | --- |
| Menu `openImportFixture` | performInvocation Interactive | same + **host-arm `return`s** |
| Guest `openImport.enter` / `exit path=shell-only` | 0 (handle never ran; hung on #41) | 0 (host-arm no longer waits for guest) |
| Chooser | `chooser=none`, `missed chooser and execute` | **`chooser=yes`** both same-file and distinct |
| `import-picker opened` | 0 | `opened=1` (7526 B export, 9177 B distinct) |
| `importFixture` ingress | 0 | **yes** — `payloadType=string` `payloadLen=7526/9177` `argKeys=[payload,name,windowId]` Interactive seq 514/523 |
| Guest apply | n/a | **`action failed importFixture`** — actor already faulted `v102_1` |
| Census | before=180 after=180 | before=1 after=1 |

`openImportFixture` **did not settle on the guest** (host-arm never forwards it). The chooser **did** open. `importFixture` **did** dispatch with the payload. Distinct-fixture import now dies at **guest apply on a faulted/busy Interactive actor**, not at probe chooser wiring.

Import-only (no frame, no brush) still showed `v102_1` notices right after Concrete example-switch (`newFaults=none` on the step, windows already `not responding`). That fault is wasm `bridge.js` `poll$1` — not leftover merge (leftover only publishes on `setActiveUtility`). Host cannot apply a puzzle fixture without a live actor.

### Needs #43 (unchanged after re-verify)

- Cheap interaction+vortices publication that does **not** re-encode 180 instances. Viewport→`puzzle3d.play.composite` is the hang. Leftover utility can make the probe read `utility=brush` but cannot invent vortices or `brushPreview` JSON.
- Command-ingress must not hold the actor busy across 4096 UI continuations, or user verbs (`setActiveUtility`, `importFixture`) starve / fail after `v102_1`.
- Guest `openImport.*` / `utility.publish action=setActiveUtility` stay the proof handle ran; they stayed **0** on #42 wasm in battery and on Concrete re-verify.
- `v102_1` `Cannot destructure property 'length'` on Concrete example-switch / frame — guest glue; not W-AB leftover. W-G3 `interactionSelect` 4096 ignored (no evidence it gated these two lanes).

### Rebuild / restart flags

- No serve kill (`:6014` HTTP 200). No wasm rebuild. No modifying git.

## 2026-09-10 W-AB — #43 readout

Source: `🗑️generated/probe-2026-09-10T21-24-53.md` (battery, actor stayed up). Targeted: `🗑generated/w-ab-43-brush-concrete.txt` + `🗑️generated/probe-2026-09-10T21-32-04.md`; vite-live re-verify `🗑generated/w-ab-43-brush-reverify.txt`; import `🗑generated/w-ab-43-import-concrete.txt` + `🗑️generated/probe-2026-09-10T21-41-18.md`. Suggestions **PASS**. setCamera / `no host context` storm is W-G3 — both W-AB lanes still produced complete guest taps after those faults, so they are not camera-only ghosts. No serve kill. No wasm rebuild. No modifying git.

### Brush-preview-place — tap-by-tap (#43 wasm, live actor)

| hop | evidence | died? |
| --- | --- | --- |
| Host `#brush` | `utility=brush`, leftover overlay | no |
| Guest `utility.publish action=setActiveUtility` | handle ran (vortices republish) | no |
| Vortices | Concrete 11 / battery 33, `vortices.publish brush_or_volume=true` `bytes=2640` | no |
| Hover commit | `interactionHover` 64–70×; leftover `hoveredVortexFullId` | no |
| `suggestionsTick.enter` | `target=Some("seed-left-001:v3")` (and v4/v5/v7/v8) | no |
| Cache | `tick-before free=0 pending=true` → `tick-after free=6 pending=false slices=3` | no — cache warms |
| `brushPreview.compute` | `vortex=v3 bytes=267` then `v7 bytes=253` | no — preview **computed** |
| `brushPreview.lane` | **`preview=267` / `253`** then later **`preview=0`** | **YES — wipe** |
| Gate | `hover vortex=None` → `gate reason=no-target` on a later body (setCamera / hover-empty render) | dying hop |
| Host place | `brush-place hop preview=null` `addBrushObject=0` | follows wipe |
| Probe | `rawLen=0` after storm even with leftover `hover=v3` | FAIL `preview=null` |

**Dying hop:** scene-lane publication **wipes** a warm preview. Tick + cache + compute all succeed. A following world encode with empty guest hover hits `brushPreview.gate reason=no-target` and publishes `preview=0`. Host leftover still shows hover; `data-brush-preview-json` reads the last guest body (empty). Vite-live retain cannot help when React never observes the 267-byte flash (same-turn wipe).

Some `suggestionsTick` / `setCamera` fail with W-G3 `no host context` or authority-slot faults. They are collateral: the successful ticks already warmed the cache and computed JSON.

### Import-distinct — tap-by-tap

| hop | battery 21-24-53 | Concrete import-only |
| --- | --- | --- |
| Chooser | **yes** (host-arm) | **yes** |
| `importFixture` ingress | payload 7526 / 9177 B | same |
| Guest `openImport.*` | 0 (host-arm does not forward `openImportFixture`) | 0 |
| Guest `utility.publish action=importFixture` | (console rotated) | **yes** `map_hit=false` |
| `performInvocation settled importFixture` | — | **yes** `historyCursor=null` `historyUpserts=0` `effects=0` |
| History | no Import row | no Import row |
| Census | before=3 after=3 (`seed-left-001`, `object-1`, `object-2`) | before=1 after=1 |
| Distinct fixture | 2 objects (`seed-left-001`, `probe-distinct-…`) | same shape |
| treeItems | 30 → 32 | 10 → 12 |

`openImportFixture` does **not** settle on the guest (host-arm `return`s). Chooser opens. `importFixture` **does** dispatch and **does** reach `handle_action_impl`. Apply **does not** mutate: settled no-op. Probe census is world `instancesJson` — a successful distinct import would show 2 ids including `probe-distinct-*`, not stay at 3. Same-file Concrete→Concrete is the documented store identity no-op; **distinct** must not be.

**Dying hop:** guest `importFixture` fold / window map (`map_hit=false`) — payload delivered, no history upsert, instances JSON unchanged. Not probe wiring.

### Fixes landed (source + vite-live)

1. **Host retain** `retainWorldBrushPreviewJsonV1` — keeps last preview JSON per vortex when leftover hover still names that target. Engine-contract law passed (`SEMIO_TEST_LEVEL=long`). Re-verify on `:6014` still `rawLen=0`: host never saw the warm JSON. Honest — not enough without #44 encode.
2. **Guest live-target** — `suggestionsTick` latches `session.brush_live_target`; `world_brush_preview_json` falls back to it when hover is empty (closes `reason=no-target` wipe). Law `brush_preview_target_falls_back_to_session_live_target` passed (`cargo test … --lib brush_preview_target_falls_back`). **Needs #44 wasm.**

### Needs #44

- Guest world encode must use the latched live target so the **last** published body keeps `brushPreviewJson` after hover-empty / setCamera renders.
- `importFixture` must fold the distinct payload into the live fixture and republish instances (`map_hit` / history upsert). Probe already counts the right lane.
- W-G3 owns `setCamera` → `surface 1:window has no host context` (×124). Do not block #44 on it; W-AB taps completed after those faults.

### Rebuild / restart flags

- No serve kill (`:6014` HTTP 200). No wasm rebuild. No modifying git.
- Coordinator: build **#44** — live-target preview encode + importFixture apply/fold.
