# Wave W-AB — interaction chrome (items 4 / 6 / 8)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Probe `$T/🔍️browser-probe.ts`. Selection publication is W-G3 — not this wave. Serve `:6014` (wasm **#35**). Never killed a process. Never rebuilt wasm.

Repo MCP was not available.

## Verdict table

| # | Item | Verdict | Artifacts | Notes |
| --- | --- | --- | --- | --- |
| 4 | Export/Import | **WORKSPACE-MENU EXPORT PASS** | `09-48-02-export.json` (`puzzle-3d.json`), `09-48-02` dump `pe: "auto"` | Dimmed WindowChrome inherited `pointer-events: none` on every leaf. `pointer-events-auto` on `contextMenuItemClassName` + `ContextMenuChrome` `active={true}` / `bodyClassName="pointer-events-auto p-single"` made Export hittable. Force + native `click()` downloaded `puzzle-3d.json`. Numbered `5 Import` is still `importFixture` (no filechooser); `Import…` / `openImportFixture` is the picker. Actions-pane download path was already live. |
| Frame | Per-window Frame | **PASS** | `10-00-36-frame-perspective.png` | Overlay Frame is hittable (`pe: auto`, 49×24 at 498,45). Handler ran against instances-group AABB (`radius ≈ 5.41`). Perspective shows the Forest table filling the view. Earlier centroid-only frame sat 1.4 m on `seed-left-001` and overshot into empty horizon (`09-57-30-frame-perspective.png`). |
| 6 | Brush | **CHROME PASS; PLACEMENT NOT PROVEN** | `10-00-36-brush-framed.png`, `10-05-52` | Brush still activates (red, Overlap 2%). After Frame, table fills Perspective. Clicks on the visible table end produced history `Resize Window` / `Set Active Example` only — **no** `addBrushObject`, **no** `[DEBUG] world dispatch addBrushObject`. Marker miss on the framed right-hand table end (vortex `seed-left-001` is off-camera), not a reserved-lane publish miss. |
| 8 | Suggestions | **GESTURE CONFIRMED; STILL WORKSPACE MENU** | `10-00-36-suggestions-open.png`, `10-05-52` | Alt+right-click without a hovered vortex still opens Workspace Menu (`1 Set Active Example…`, `suggestRows=0`). Host `hoveredVortexFullIdRef` arming is live. Same off-camera vortex as item 6. |

## Camera / Frame

Orbit wheel dollies along the look target, not toward the pointer. Blind wheel ticks at the table centroid zoomed into empty grid (`08-45-10-brush-framed.png`).

Landed:

- Overlay Frame per window: `id=world3d-frame-instances-${windowInstanceId}` inside `pointer-events-auto z-40` (parent overlay used to eat the click; `09-37-22` Frame click never reached the handler).
- `world3dFrameCameraFromInstances` — centroid fit for the no-mesh fallback (law still PASS).
- `world3dFrameCameraFromBounds` — live instances-group `Box3` via `fitCameraFromBounds`, **keeps the current look direction**. This is the path the Perspective Frame button now uses. Empty-selection `zoomToSelection` still falls through to Frame.
- Probe `frameForestTable()` dumps computed `pe` / box / `data-instances-json` length, force-clicks, and native `click()`.

`10-00-36` recapture: `instanceCount=1` (`seed-left-001`), group radius `5.405`, table fills Perspective.

## Export (item 4) — workspace-menu `onSelect`

This was **not** a missing guest `action` field and **not** the old encoding/`startsWith` crash. Binding law `keeps exportFixture and importFixture menu rows on the same leaf onSelect bind` stayed PASS.

Root cause on `:6014` (`09-37-22` dump): both Export and Import rows had computed `pointer-events: none`. WindowChrome body is hardcoded `data-dim`; ContextMenuChrome used `active={false}`, so the entire menu body inherited inert chrome — same dropped-child / control-slot class as Interpreter tree-item `.run` never mounting. Playwright real clicks fell through to the canvas. `element.click()` ignores `pointer-events` (why Import evaluate used to work); Export evaluate still raced a closing menu.

Landed:

- `export function contextMenuItemClassName` includes `pointer-events-auto`
- `data-menu-action={item.action}` on leaf and submenu buttons
- ContextMenuChrome `active={true}` and `bodyClassName="pointer-events-auto p-single"`
- probe clicks `[data-menu-action="exportFixture"]` with `force` + native `click()`

`09-48-02` recapture: `pe: "auto"`, download `puzzle-3d.json` saved as `$T/🗑️generated/probe-2026-09-10T09-48-02-export.json`.

## Brush (item 6)

Utilities unfold + `#brush` still PASS. Placement is a vortex `onClick` → `onBrushPlace` → `addBrushObject` when `brushObjectPlacementArgs` is non-null.

After Perspective Frame the table is large, but the visible end is the right-hand top. `seed-left-001` sits off that view. Clicks around canvas-local `(739, 351)` plus table-surface offsets never dispatched `addBrushObject`. That remains a **marker miss**, not reserved-lane (no guest command / host effect observed).

## Suggestions (item 8)

`world3dSuggestionsGestureArmed(altKey, hoveredVortexFullIdRef)` is live. `10-00-36-suggestions-open.png` / `10-05-52` chrome dump is still Workspace Menu. Hover never armed. Same off-camera vortex as item 6.

## Files changed

- `🌐️World3dHost` — `world3dFrameCameraFromBounds`, Frame overlay `z-40` / `pointer-events-auto`, Frame click uses instances-group AABB, empty-selection zoom falls through to frame-all
- `🖱️ContextMenu` — `contextMenuItemClassName` `pointer-events-auto`, `data-menu-action`, `id={item.id}`
- ContextMenuChrome — `active={true}`, `bodyClassName="pointer-events-auto p-single"`
- `🔬️engine-contract` — centroid frame law, AABB frame law, export-bind law, pointer-hittable law
- `$T/🔍️browser-probe.ts` — `frameForestTable()` dump + force/native click, menu `data-menu-action` click, framed-table marker spots
- this report

Temporary `[DEBUG]` taps (`frame visible instances`, `world dispatch`, `context menu leaf click`) were stripped after the `10-00-36` / `10-05-52` / `09-48-02` captures.

## Laws / suites

Run from the renderer react package: `SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run --testNamePattern="keeps exportFixture|keeps context-menu leaf rows pointer-hittable|frames world instances onto the table centroid|frames a table-sized world AABB"`

- `keeps exportFixture and importFixture menu rows on the same leaf onSelect bind` — PASS
- `keeps context-menu leaf rows pointer-hittable under dimmed window chrome` — PASS
- `frames world instances onto the table centroid without a guest selection` — PASS
- `frames a table-sized world AABB without leaving the current look direction` — PASS
- `arms the suggestions gesture from a host vortex hover without a guest InteractionView` — PASS (earlier this wave)
- `appends a download anchor and keeps the object URL alive past the click turn` — PASS (earlier this wave)
- `treats a non-string media-export encoding as a one-shot download, not a segmented marker` — PASS (earlier this wave)

Last focused contract run this turn: **4 passed | 506 skipped**.

## Rebuild / restart flags

- No wasm deploy from this wave. Export_fixture on #35 is live (workspace-menu Export proved it).
- `:6014` returned 000 while Vite compiled World3dHost, then came back 200. Did not kill.
- Peer `pluginUiIntakeStepCeiling` → `retainedUiIntakeStepCeiling` hole is closed; serve boots again.
