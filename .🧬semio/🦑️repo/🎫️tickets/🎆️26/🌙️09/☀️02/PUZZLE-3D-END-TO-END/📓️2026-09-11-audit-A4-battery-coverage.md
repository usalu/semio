# Audit A4 — Battery Coverage vs User-Feature Checklist

Read-only audit, 2026-09-11. Sources read in full: `🔍️browser-probe.ts` (1446 lines),
`📓️2026-09-09-user-feature-checklist.md` (449 lines), `📓️2026-09-10-cursor-coordination.md`
(sections 21:15/21:28/21:59 plus surrounding entries). Host code grepped/read for real DOM
selectors: `🌐️World3dHost/🟦️.tsx` (6112 lines), `🏛️ShellHost/🟦️.tsx` (9982 lines, targeted),
`🛠️ShellHelpers/🟦️.tsx` (targeted), `🪟️Window/🟦️.tsx`, `🆔️ElementId/🟦️.tsx`,
`🖱️ContextMenu/🟦️.tsx`, `🪜️Stepper/🟦️.tsx`, `🎀️Ribbon/🟦️.tsx`, `🌳️Tree/🟦️.tsx` (all under
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/` unless noted). Did not run the probe (another run owns
:6013). All line numbers below were verified against the file content actually read in this pass.

## 0. Battery execution order (derived from the flag-gating logic, cross-checked against the
coordination log's own description)

`--battery` sets `family = true` (line 103) and is itself a member of `extraFlags`'s OR-list
(line 104), which means the **plain click/right-click smoke steps
(`activate-perspective`/`pick-object`/`context-menu`, lines 121-134) are SKIPPED under `--battery`**
— they only run in bare `--interact` mode. `wantExample` is forced true regardless (line 107) via
`wantUndo` (family). `--fill` (lines 168, 867) and `--marquee` (line 1055) are **not** part of the
`family`/`battery` OR-list anywhere — they never run under plain `--battery`, must be requested
explicitly. The resulting battery order, confirmed against the 20:46 coordination entry ("boot →
example → undo/redo → selection → clipboard → locked → gumball → brush → suggestions → import"):

boot → example-switch → history-open → undo-unwind → undo-redo → selection-surfaces →
clipboard-copy-paste → locked-refusal → gumball-drag → frame-perspective → brush-stroke →
suggestions-open → export-import → battery-faults

One continuous `page` session for the whole run (no reload between steps) — 867s wall-clock per
the 22:40 coordination entry.

## 1. Step / verdict table

| step (line) | clicks / types | asserts | checklist section | known weaknesses |
|---|---|---|---|---|
| `boot` loop (61-78) + `verdict("boot",…)` (114) | polls `snapshot()` up to 60×3s for `windows.length>=2 && canvases>=2`; clicks a "skip" dialog button every 3rd poll (66-70) | window/canvas presence only, no content check | §1 (partial — presence, not "grid+example painted") | doesn't check WHICH two windows (top vs perspective) or that each carries a *different* projection; a boot with two identical broken canvases still passes |
| `example-switch` (136-166) | opens `[role="combobox"]` (or a `<select>`), clicks option (Nakagin regex or nth(1)), fixed/`--settle=` wait (default 20s) | `verdict("example-switch", /nakagin/i.test(afterExample), …)` — text-match on the combobox label only | §5 example switcher | never inspects `data-instances-json` count to confirm the DOCUMENT actually swapped (label could update before/without the scene swapping — checklist §5's own open nuance about `runtime` reset not round-tripping through `WindowConfig` is never probed here) |
| `history-open` (902-905) | clicks history panel tab, `#framework.history.actions`, `#framework.history.undo`, expands up to 6 collapsed tree rows | logs `historyState()` only, no verdict | §20 history panel (open only) | pure side-effect exploration; a hang here (checklist's top-priority risk, "likely still broken… no rebuild ever confirmed") would only surface as the *next* step's `step()` catch (93-100), not as its own diagnosis |
| `undo-unwind` (907-919) | `dispatchHistory("undo")` ×≤12 via DOM click on `#framework.history.undo`, falling back to `Meta+z` | `verdict("undo-unwind", /concrete forest/i && !/nakagin/i, …)` | §20 undo + §5 (example identity as the observable signal) | uses the example NAME as the only proof undo worked — an undo that reverts artifact edits but leaves the combobox stale (or vice versa) is indistinguishable from success/failure here |
| `undo-redo` (920-930) | `dispatchHistory("redo")` ×≤12 | `verdict("undo-redo", /nakagin/i.test(last.example), …)` | §20 redo | same signal weakness as above; also **order-dependent on undo-unwind having actually reached Concrete Forest first** — if unwind partially failed, redo's starting point is undefined |
| `selection-surfaces` (970-995) | `frameForestTableAfterCensus()`, opens Inspection tab, `clickForestTable()` (two hardcoded hover+click spots, 402-405), canvas click at `framed.table.x/y` | `verdict("inspection-object-fields", …)`, `verdict("inspection-locked-flag-row", …)` | §6 click-select + §16 inspection panel | `clickForestTable`'s hover/click coordinates (529,415)/(541,411) are fixed pixel guesses tuned for Concrete Forest post-frame layout, **not derived from `data-vortex-hits`/`data-instances-json` geometry** the way `brush-stroke`/`suggestions-open` do (764-783, 1162-1169) — the one step most likely to silently miss the target on Nakagin's very different layout |
| `clipboard-copy-paste` (998-1053) | frame+select, opens "Actions", clicks a `copy` button/id, `Meta+c`/`Control+c`, `Meta+v`/`Control+v`, opens history | `verdict("clipboard", censusAfter.count > censusBefore.count, …)` | §21 copy/cut/paste | checklist §21 itself says **no app-specific clipboard handler exists in the editor crate** — this step is testing a framework-generic capability the plugin never opts into; 21:28 coordination entry found the earlier apparent regression (`treeItems 4→106`) was a **measurement bug** (history-panel row paging, not the scene census) — the step's OWN `treeItems` field in `selectionState()`/`fillState()` is unreliable as a delta signal for anything except literal outliner row count |
| `locked-refusal` (1074-1106) | frame+select, opens Inspection, clicks a `[id$='puzzle3d-play-inspector.object.locked']` toggle, unfolds utility bar, clicks `#move`, drags gumball | `verdict("locked-flag-row", …)`, `verdict("locked-refusal-notice", …)` filters out `/agent disconnected\|remote: detached/i` from notices (1102) | §16 lock chrome + §8 gumball (negative path) | the notice filter is a **good, narrowly-scoped fault/collateral distinction** but exists only here — every other `chromeState().notices` consumer elsewhere in the file does NOT apply it |
| `gumball-drag` (1109-1138) | frame+select, unfolds utility bar, clicks `#move`, `dragGumballMoveX()` (hover-polls `data-gumball-hits` for an on-screen `moveX` handle, then a real `mouse.down/move/up` drag) | `verdict("gumball-handle-enter", …)`, `verdict("gumball-scene-delta", …)` | §8 gumball/transform | 21:28 entry: "leftover gumball mode forced `transform` (drag fell into rotateZ) → now `move` + diagonal drag" — a probe-side bug already found and patched once; the technique (project handle via `data-gumball-hits`, then raw mouse events) is inherently timing-sensitive (poll loop up to 16×250ms, 724-732) |
| `frame-perspective` (1141-1144) | `frameForestTableAfterCensus()` only | none (no verdict) | infra step, not itself a checklist item | exists purely to get objects on-screen for `brush-stroke`/`suggestions-open`; if this silently frames the WRONG example (state bleed from `undo-unwind`/`undo-redo` above), every downstream step aims at stale geometry |
| `brush-stroke` (1147-1220) | `armBrushUtility()` (unfolds utility bar, clicks `#brush` — single force-click, with an explicit inline comment (461-462) about a known double-activation bug when combined with a same-tick `page.evaluate` read), 70× synthetic `mouse.move` "hover storm" over on-screen vortex hits, then a real click | `verdict("brush-preview-place", …, 41)` | §9 brush utility | 21:59 entry: "preview rawLen=0 EVERYWHERE despite laws green on the serving wasm (law-vs-browser divergence)" — a genuine unresolved signal, not a probe artifact; the `expectTag: 41` is now stale (build is #44+ per the 12:25 entry) |
| `suggestions-open` (1223-1273) | `armBrushUtility()`, hovers an on-screen vortex, `Alt`+right-click (`world3dSuggestionsGestureArmed`, World3dHost/🟦️.tsx handles this at the `onContextMenu` on the host div, 5839-5851) | `verdict("suggestions", …, 41)` | §13 vortex suggestions (open only — hover-to-preview/accept/close never separately probed) | 22:40 entry: this was the one step that flipped PASS between build #41→#42; brittle on the same alt-modifier-timing/right-click-race the checklist's own §13 "DOM routing swallow" note describes |
| `export-import` (1276-1429) | `activateWorkspaceMenuOrdinal("4")`→export (waits for a `download` event), then `("5")`→import (waits for a `filechooser` event), re-imports a self-synthesized "distinct" fixture (object clone + offset) | `verdict("import-distinct", …, 41)` — **the export half and the plain same-file-reimport half are logged but never scored as their own verdict** | §24 import/export | see finding below — checklist §24 claims import/export "does not exist as a usable end-user feature," yet this step drives a real `shell-menu.action.exportFixture`/`openImportFixture` pair with real download/filechooser events; 21:59/22:40 entries note "openImportFixture hangs under 180 instances" (Nakagin) and "import flat" (no count delta) — consistent with *the menu exists and does something, but the underlying effect is dead or capacity-limited*, not with "no such feature" |
| `battery-faults` (1432) | none | `verdict("battery-faults", faults.length === 0, …)` | cross-cutting | `faults` is capped at 200 entries (line 36) and `consoleBuf` at 4000 (line 34) — see §3 below |

## 2. Checklist sections with no probe step

Verified by walking every `process.argv.includes(...)` gate in the file; confirmed absent under
plain `--battery` (not merely "not in the default step list"):

- **§1 Top-window content** — only window-count/canvas-count in the boot loop; no per-window
  content check (grid, gizmo, example footprint).
- **§2 Camera (orbit/pan/zoom)** — the only camera-touching step (`activate-perspective`, a single
  canvas click, not even a drag) is gated `!extraFlags` (line 121) and is therefore **skipped under
  `--battery` specifically**, even though it exists in the file for bare `--interact`.
- **§3 Projection pane options** — zero references to `puzzle3d-measure-projection-*` or
  `framework.worldOrbit.projection` anywhere in the probe.
- **§4 Window options (grid/LOD/vortex/sun)** — zero references to `puzzle3d-play-grid*`,
  `puzzle3d-play-lod*`, `puzzle3d-play-vortex-*`, `puzzle3d-measure-sun-*`. This is the single
  highest-value gap: checklist §4 calls this "the single largest reason a session reads as frozen,"
  fixed in source (W-D2) but **never runtime-confirmed**, and the battery still doesn't touch it.
- **§10 Volume Brush utility** — `addTargetVolume`/`setVoxelDims` never dispatched.
- **§11 Relocate utility** — `worldRelocate` never dispatched. Checklist flags this as the one
  feature specifically expected to **fault on Nakagin** (`objects×66+attractions` > 4096-item cap
  above 62 objects) — the battery cannot currently observe this at all.
- **§14 Engagement bar** — `engagementInput`/`Submit`/`RepeatLast`/`Abort`/`ControlSelect` never
  dispatched (the only `Escape` presses in the file live inside the `--fill`-gated
  `fill-abort-engagement` step, not in the plain battery).
- **§15 Context-menu rows vocabulary** — the only step that opens the raw right-click menu
  (`context-menu`, line 130) is gated `!extraFlags` like §2 and is skipped under `--battery`.
  `suggestions-open`'s `Alt`+right-click is a *different*, menu-suppressing gesture
  (`event.preventDefault()`, World3dHost/🟦️.tsx:5843-5844) — it never opens `[role="menu"]`.
  Checklist's own highest-confidence new defect (`"zoomToSelection"` is an unregistered action id)
  is therefore never exercised.
- **§17 Outliner interactions** — no step opens the document/outliner panel or clicks its inline
  hide/lock row icons; checklist's `flag_args` hardcoded-`true` defect is never exercised.
- **§18 Catalogue panel** — no step opens the catalogue tab or clicks/drags a kind row.
- **§19 Settings panel** — no step opens the settings tab or touches any of the four steppers.
- **§22 Delete/duplicate/focus selection** — `Delete`/`Backspace`, `Ctrl`/`Cmd+D`, `F` are never
  pressed anywhere in the file.
- **§23 Add Object dialog** — `openAddObjectDialog` never triggered; the checklist's flagged
  hardcoded-single-option defect is never exercised.
- **§25 Locale** — no locale/language switch anywhere in the file; EN/DE label text never compared.

**Nuance, not a clean gap — §24 Export** *is* partially exercised by the `export-import` step (see
table above); it is closer to "exercised but weakly scored and possibly testing a different
mechanism than the one checklist §24 audited" than to a true zero-coverage section. Flagged
explicitly so the coordinator doesn't duplicate work assuming it's untouched.

### Proposed steps for each gap (concrete selectors, file:line-cited)

All new `id`s below use the same `childElementId(parent, ...segments)` grammar already relied on
throughout the existing probe (dot-joined camelCase segments, `🆔️ElementId/🟦️.tsx:51-53`) — the
window id `puzzle3d-main-perspective` becomes `puzzle3dMainPerspective` in every derived id
(confirmed: probe already uses `framework.window.puzzle3dMainPerspective.utilityBar.unfold` at
lines 421, 785, matching `childElementId("framework.window", id, "utilityBar", …)` at
`🪟️Window/🟦️.tsx:410`).

**§1 Top-window content** — locator: `#framework.window.puzzle3d-main-top` and
`#framework.window.puzzle3d-main-perspective` (`🏛️ShellHost/🟦️.tsx:9264-9265`,
`childElementId("framework.window", instance.id)`); inside each, World3dHost's own root div
carries `data-surface-id`, `data-instances-json`, `data-vortices-json`, `data-meshes-json`,
`data-status-json` (`🌐️World3dHost/🟦️.tsx:5829-5838`). Sequence: after boot, for BOTH window ids
independently read `data-instances-json` length and confirm exactly one `canvas` per window.
Assertion source: parsed JSON length + canvas count, scoped per window id (not the global
`page.locator("canvas")` the rest of the file uses). Expected: Concrete Forest → instanceCount≈1 in
both windows (same document, two views); Nakagin → instanceCount≈180 in both. A cropped
before/after screenshot per window (top should look flat-orthographic, perspective should show the
3-point/50° gizmo) is the only way to check "content," not just count.

**§2 Camera** — locator: `canvas` inside `#framework.window.puzzle3d-main-perspective`. No
`data-camera-json`-equivalent attribute exists on World3dHost (confirmed absent — grepped every
`data-*` attribute it sets, see inventory below; camera state only reaches `<WorldCanvas
cameraUp=… cameraFov=…>` as React props, `🌐️World3dHost/🟦️.tsx:5887-5888`, never serialized to the
DOM). Sequence: screenshot, left-drag (orbit)/right-drag (pan)/wheel (zoom), screenshot again,
pixel-diff. Cross-check: `openHistory()` entry count must NOT change (checklist §2: `setCamera` is
declared to emit no artifact mutations) — an unchanged entry count plus the NEXT step not timing
out is itself indirect proof the notorious WindowConfig-lane hang (checklist §2/§4) is actually
fixed, since that bug hung on literally any `setCamera` call. Expected: identical mechanics on both
examples (view state, not document-dependent).

**§3 Projection** — locator: framework-generic pane `id="framework.worldOrbit.projection"`
(`🌐️World3dHost/🟦️.tsx:4224`) plus the plugin's own measures rail, reached via
`#framework.window.puzzle3dMainPerspective.measures.unfold`
(`childElementId("framework.window", id, "measures", folded?"unfold":"fold")`,
`🪟️Window/🟦️.tsx:317,332`), then the `puzzle3d-measure-projection-*` toggle-group items inside its
`bodySlot="window-measures-body"` (`🪟️Window/🟦️.tsx:347,353`). Sequence: unfold measures, click an
orthographic/1-2-3-point option, click a cardinal/free orientation option. Assertion:
`[data-slot="toggle-group-item"][id^="puzzle3d-measure-projection-"]` `aria-pressed` flips +
before/after canvas pixel-diff. Expected: identical on both examples.

**§4 Window options** — same `measures.unfold` entry point as §3. Concrete ids (already
grep-verified against `🎚️config/🦀️.rs` by the checklist itself): `puzzle3d-play-grid-visible`,
`puzzle3d-play-grid-spacing`, `puzzle3d-play-lod-auto`, `puzzle3d-play-lod-value`,
`puzzle3d-play-vortex-show`, `puzzle3d-measure-sun-enabled` (note `"puzzle3d"` prefix, not
`"puzzle3d-play"`, for the sun group specifically). Sequence: unfold once, then for each id read →
toggle/drag → wait → read again, each wrapped in its OWN try/catch (reuse the existing `step()`
helper's pattern, lines 91-100) so one hang doesn't kill the whole battery. Assertion: value flips
AND `openHistory()` gains **no** new entry (this lane is `WindowConfig`, not `Artifact` — the
absence of a history row is the correctness signal that distinguishes "worked silently" from
"genuinely no-op'd"). Expected: identical on both examples — but this is exactly the historically
100%-repeatable infinite-hang lane, so the FIRST toggle click here is the single most valuable
untested assertion in the whole checklist; if the page stops responding, that alone is the verdict.

**§10 Volume Brush** — locator: same utility-bar unfold as Brush
(`#framework.window.puzzle3dMainPerspective.utilityBar.unfold`, `🪟️Window/🟦️.tsx:402,410`), then
the Volume Brush toggle-group item — its literal DOM id was **not** located in this pass (the
existing probe locates Brush/Move by literal id, `document.getElementById("brush")` at line 444,
`#move` at line 1096; Volume Brush's equivalent needs one grep of
`🪛️utilities/🧊️volume-brush/🦀️.rs` before this step can be written). Sequence: activate, Alt+click
a grid-snapped canvas point (`addTargetVolume`), adjust the `puzzle3d-play-utility-options-volume-brush`
group's w/d/h inputs (`setVoxelDims`). Assertion: `openHistory()` new entry + outliner `treeItems`
delta (no dedicated `data-target-volumes-json` attribute was found on World3dHost — see the
attribute inventory below, this is a real instrumentation gap for whoever implements the step).
Expected: pair with a subsequent `--fill` run to confirm the volume actually constrains Fill
(checklist §10).

**§11 Relocate** — locator: utility-bar unfold → Relocate item (id likewise not located this pass).
Sequence: select an unlocked object (reuse `clickForestTable()`), drag it via raw
`mouse.down`/`mouse.move`/`mouse.up` onto a new canvas position — this is `worldRelocate`, a
distinct verb from the gumball, drag-drop not handle-drag. Assertion: `data-instances-json` pose
delta (reuse the exact before/after string-compare technique `gumball-drag` already uses, lines
1120,1126) + a `worldRelocate` history entry. **Expected — the critical case**: works on Concrete
Forest (1 object); is expected to **fault** on Nakagin (180 objects, checklist §11's
`objects×66+attractions` vs 4096-item cap above 62 objects) — so the Nakagin variant of this step
must assert `faults.length` increases, not a pose delta, and MUST run after the example-switch has
actually landed on Nakagin (order-dependent on `undo-redo`'s own reliability, see table above).

**§14 Engagement bar** — locator: `#framework.window.puzzle3dMainPerspective.engagement.toggle`
(`childElementId("framework.window", id, "engagement", "toggle")`, `🪟️Window/🟦️.tsx:358,365`)
opens the pane; its body (`bodySlot="window-engagement-body"`, `🪟️Window/🟦️.tsx:367`) contains an
`input`/`[role='textbox']`. Sequence: open, type `"brush"`+Enter, type `"fill 5"`+Enter, type
`"clear"`+Enter (checklist: advertised in the placeholder text but deliberately dropped — this step
should PROVE the no-op rather than assume it), `Escape` (`engagementAbort`). Assertion: after
`"brush"`, `dumpBrushPreview().utility === "brush"` (reuse the existing helper, lines 513-556);
after `"fill 5"`, `#tool.fill` `aria-pressed="true"` + a `puzzle3d-fill-count` value change; after
`"clear"`, assert NO state change anywhere. Expected: identical on both examples.

**§15 Context-menu rows vocabulary** — locator: right-click a selected object opens `[role="menu"]`
whose rows are `<button id={item.id} data-menu-action={item.action}>`
(`🖱️ContextMenu/🟦️.tsx:428,437`) — literal ids per checklist §15's own table:
`[id="duplicate"]`, `[id="select-same-kind"]`, `[id="zoom"]`, `[id="delete"]`, `[id="suggest"]`,
`[id="hide-show"]`, `[id="lock-unlock"]`. Sequence: select an object, right-click (re-enable the
existing `context-menu` step's gesture, currently skipped under `--battery`), click `[id="zoom"]`,
read its `data-menu-action`. Assertion: `dataMenuAction === "zoomToSelection"` (confirms the
checklist's claim about the row's wiring) AND `faults.length` before/after (does the unregistered
action id actually fault at `dispatch_action`, or silently no-op — checklist flags this as
unverified at runtime despite "confirmed by grep" at the source level). Expected: identical defect
on both examples (registry-level bug, not data-dependent) — cheapest, highest-confidence gap to
close: one right-click + one button click + one console diff.

**§17 Outliner interactions** — locator: outliner panel root `puzzle3d-play-document`; its tab's
exact id was not located this pass (the probe already captures ALL panel-tab ids generically via
`snapshot().tabs`, line 53 — a one-time dump of that array is the fastest way to find it, by analogy
with the already-known `#framework.panel.history`/`#framework.panel.inspection` ids used at lines
852,973). Sequence: open the tab, click an object row's inline hide icon, confirm it hides, click
"Show" on the SAME now-hidden row. Assertion: checklist's high-confidence defect
(`flag_args` hardcodes `("value", true)`, `📌️panels/🗿️artifact/🦀️.rs:122-124`) predicts the SECOND
click does NOT restore visibility — chain outliner-hide → outliner-"show"(expect no-op) →
context-menu `hide-show` row (§15, expect real toggle) in one sequence to directly demonstrate the
discrepancy checklist describes. Expected: identical on both examples; Concrete Forest (1 object)
is actually the easier target, no row-identity ambiguity.

**§18 Catalogue panel** — locator: `puzzle3d-play-kinds` root, section keys like
`puzzle3d-play-kinds.objects`. Sequence: open the tab, click a kind row (`addObjectKind`);
separately attempt a native drag via synthesized `DragEvent`s carrying
`PUZZLE3D_CATALOGUE_DRAG_MIME` (Playwright's raw mouse events don't reliably fire React DnD
handlers — the probe already synthesizes low-level events elsewhere for exactly this reason,
`canvas?.dispatchEvent(new PointerEvent("pointerenter",…))` at line 741). Assertion:
`dumpInstances()` delta + `openHistory()` `addObjectKind` entry + whether the new object becomes
selected (checklist explicitly flags re-select-on-add as an open question, unlike duplicate/accept-
suggestion which were fixed in W-S). Expected: works on both examples per checklist (only the
EMPTY-document case, §5's territory, is the known-fixed edge case).

**§19 Settings panel** — locator: `puzzle3d-play-settings` root; the four steppers are
`<input type="number" id="puzzle3d-play-settings.overlap-budget">` etc. — the `id` sits on the
actual `data-slot="input"` element (`🪜️Stepper/🟦️.tsx:209-211,259`), adjacent
`data-slot="stepper-minus"`/`"stepper-plus"` buttons (`🪜️Stepper/🟦️.tsx:197,264`). Sequence: open
settings, bump `puzzle3d-play-settings.grid-spacing` via `stepper-plus`; separately touch the SAME
logical value via §4's `puzzle3d-play-grid-spacing` slider from the OTHER window's measures rail.
Assertion: checklist's own flagged ambiguity — does editing from Settings bleed into BOTH windows,
or stay scoped to the window that was focused when Settings was opened (checklist: these fields are
architecturally per-`WindowConfig`, despite the panel's session-wide-sounding title). Expected:
identical mechanics on both examples.

**§22 Delete/duplicate/focus selection** — no DOM ids at all: pure keybindings sourced from
`session.app.keybindings` (plugin-declared, matched generically at `🏛️ShellHost/🟦️.tsx:7735`; no
per-app hardcoded Delete/Ctrl+D/F handler exists in ShellHost or World3dHost — grepped, none
found). Sequence: select (reuse `clickForestTable()`), `page.keyboard.press("Delete")`; re-select,
`page.keyboard.press("Meta+d")` (or `Control+d`); re-select, `page.keyboard.press("f")`. Assertion:
Delete → `dumpInstances()` count −1 + history entry; Duplicate → count +1 with a new id AND the
clone becomes the active selection (`selectionState()` — checklist: re-select-on-duplicate was
fixed in W-S, worth confirming); Focus → before/after camera screenshot pixel-diff (checklist:
`focusSelection` forces `ui_scope` to viewport-only and moves the camera). Expected: all three
"real" per checklist, on both examples.

**§23 Add Object dialog** — locator: the dialog's kind-select uses the same "select" control case
already read in `renderStagedArgControl` (`🛠️ShellHelpers/🟦️.tsx:3293-3309`,
`<SelectTrigger id={fieldId}>`); the exact trigger button that dispatches `openAddObjectDialog` was
**not located** in this pass — needs a one-time sweep (`button` filtered by text `/add object/i`,
or a menu-row search) since it wasn't found by grep in ShellHost/ShellHelpers. Sequence: trigger
the dialog, open the kind `<SelectTrigger>`, count `[role="option"]`. Assertion: `optionCount === 1`
and its label is literally `"Object"` — directly confirms/refutes checklist's high-confidence
defect (`ActionArgOption::new("Object", …)` hardcoded instead of reading
`fixture.meta.kind_catalogs`). Expected: reproduces on BOTH examples per the checklist's own
phrasing ("both Concrete Forest and Nakagin likely" have catalogs beyond `"Object"`); if one
example's catalog genuinely only has `"Object"`, fall back to the other.

**§24 Export/Import — refine, don't just add** — already partially covered by `export-import`
(see table); the gap is scoring, not coverage. Split into three independently-verdicted steps: (a)
`export-only` — verdict on `download !== null` alone (currently logged, never scored, lines
1281-1317); (b) `import-same-file` — re-import the just-exported file, verdict on
`dumpInstances()` being unchanged (idempotency, currently unscored); (c) keep `import-distinct` as
the existing scored verdict. Also add a step that separately probes the checklist's claimed dead
`setFixtureJson` path (engagement-bar text command or any exposed debug hook) — the checklist
states flatly "no import/export action id or command file found anywhere in the editor crate," yet
`shell-menu.action.exportFixture`/`openImportFixture` (`🏛️ShellHost/🟦️.tsx:9749`,
`id: \`shell-menu.action.${action.id}\``) are real, dispatchable, and trigger real
download/filechooser events — this is an unresolved contradiction between the checklist's
Rust-only reading and the probe's own observed behavior that the ticket should resolve (is
`exportFixture`/`openImportFixture` a framework-generic "save/load this fixture-shaped document"
mechanism the checklist's plugin-crate-only grep missed, or genuinely dead plumbing that opens a
dialog and goes nowhere — 21:59/22:40 entries ["openImportFixture hangs under 180 instances",
"import flat"] read as consistent with the latter, but were never scored as a hard verdict).
Expected: Nakagin's DSL is 128,755 bytes (checklist §24) against an
`8,192-byte PUZZLE_COMMAND_RAW_BYTES` wire cap the checklist warns about — if `openImportFixture`
shares that cap, Nakagin import MUST reject; add an explicit size-boundary verdict for it, not just
an informal "flat" log line.

**§25 Locale** — locator: the `setLocale`/`locale` pair exposed by ShellHost
(`🏛️ShellHost/🟦️.tsx:7515-7517`, `noteOsCommand("os.setLocale", {locale: value})`) is consumed by
some preferences/settings surface **not located in this pass** (search was scoped to
ShellHost/ShellHelpers/World3dHost/PluginRuntime; follow-up: `grep -rn 'setLocale'
🧰️framework/🔨️modules/🖱️ui/🧱️elements`). Sequence: open the shell preferences surface, switch
EN→DE. Assertion: translate checklist §25's exact failing test target
(`document_json.contains("Baukomponenten")`) into a DOM read — after switching, the
`puzzle3d-play-document` outliner's section-label text should read the DE label instead of the EN
default; also re-read `puzzle3d-play-grid`/`puzzle3d-play-lod` labels. Expected: per checklist this
is genuinely ambiguous (real locale gap vs. stale test) — the browser is the tiebreaker; should
reproduce identically regardless of which example is loaded (locale is chrome, not document data).

### World3dHost `data-*` attribute inventory (complete, grepped)

All attributes World3dHost sets on its own root div (`🌐️World3dHost/🟦️.tsx:5826-5838`) or its
overlay children, for reference by anyone writing new steps above:

`data-surface-id`, `data-orbit-view-gizmo`, `data-puzzle3d-fixture-drag-active`,
`data-meshes-json`, `data-instances-json`, `data-vortices-json`, `data-brush-preview-json`,
`data-suggestion-menu-json`, `data-interaction-json`, `data-status-json` (root div, 5829-5838) —
plus, elsewhere in the same file: `data-gumball-hits` (3801), `data-vortex-hits` (3852),
`data-level` (5902, on overlay chrome buttons), `data-world-projection-kind-switch-host` (4231, on
the fallback-anchor wrapper only), and a family of fill-specific attributes
`data-fill-{operation,base-revision,generation,sequence,stage,target-cursor,search-count,
rejected-count,sample-cursor,candidate-cursor,inside-both,has-ghost,truncated,
registry-generation}` (3245-3259). **No `data-camera-json` and no `data-target-volumes-json`
exist** — confirmed absent by full-file grep, both real instrumentation gaps for §2 and §10 above.

## 3. Robustness recommendations

**a) Per-step fresh state.** The battery is one continuous session/document across all ~13 steps
(867s). This is exactly what produced the 21:15 "battery-ordering suspicion" (steps that pass
standalone but fail in sequence) and the 21:28 finding that `undo-unwind`/`undo-redo` leave the
document in whatever state they landed on for every later step. Recommend splitting into
independent `page.goto` reloads by mutation "blast radius": **(1) read-only/idempotent** — boot,
example-switch, camera/projection/window-options (§1-4) — safe on a pristine document, run first;
**(2) mutating-but-reversible** — selection, clipboard, gumball, locked, brush, suggestions,
delete/duplicate/focus (§6,8,9,13,15,16,17,21,22) — each of THESE should also get its own reload,
since e.g. Delete permanently shrinks the object count for every subsequent step; **(3)
document-replacing** — import/export, catalogue add, add-object dialog, undo/redo, relocate
(§5,11,18,23,24) — run last, one at a time, each from a fresh boot. Keep `--fill` as its own
standalone 4th group (already effectively isolated, and internally sequential by necessity since
its own polling depends on its own prior state).

**b) Fault classification.** HARD FAULT: a `FAULT_RE` match appearing in `consoleBuf` between a
step's start and its own action (not merely co-present in the same 4s post-action window as an
unrelated later step). The 23:10 storm (`bridge.js: Cannot destructure property 'length' of
'v102_1'`) is the canonical case — everything downstream of it (setActiveUtility hangs, import-
apply death) is COLLATERAL, not independent evidence, and the coordination log already says so in
prose ("collateral is explained"). The global `faults.length < 200` cap (line 36) makes "1 fault
repeated 200×" indistinguishable from "200 distinct faults" — recommend deduplicating by a
message-prefix key (first ~60 chars) and reporting both raw and distinct counts. COLLATERAL:
transient UI banner text like `/agent disconnected|remote: detached/i` — the probe already filters
exactly this, but only inside `locked-refusal` (line 1102); hoist into a shared
`isCollateralNotice()` used everywhere `chromeState().notices` is consumed, and record (don't just
drop) a `collateralCount` alongside each verdict. SOFT/FLAKY: patterns the checklist itself already
names as load-dependent (`job-session.terminal-fault`, checklist §2/§6) — retry once inside the
step before failing it, rather than reporting a hard FAIL on first occurrence.

**c) Machine-readable summary line format.** Today `verdict()` (108-113) writes free text into the
same `lines[]` array as every timestamped log line, and the final `.md` string-joins whichever
lines merely CONTAIN `"verdict "` (1439-1442) — no structured fields, so diffing two runs means
regexing prose, and the `expect-41`/`expect-42` tags (109) are already stale against the current
build (#44+ per the 12:25 coordination entry). Recommend one JSON record per verdict appended to a
separate `probe-<stamp>.ndjson` (not mixed into the prose `.md`):
```
{"ts":"<iso>","build":"<served-wasm hash prefix — read once at boot from whatever build marker
the coordination log's 'ReplayShellCommand marker' check already uses, not a hand-typed expect
tag>","example":"concrete-forest|nakagin-capsule-tower|unknown","section":"§9",
"step":"brush-stroke","verdict":"brush-preview-place","status":"PASS|FAIL|FLAKY",
"detail":{...same facts currently interpolated into the note string, as real fields...},
"faultsBefore":N,"faultsAfter":N,"collateral":N}
```
A coordinator can then `jq` two `.ndjson` files on `[.section,.step,.verdict]` as the join key and
diff `.status`/`.detail` directly — this is precisely the workflow the 09-10 coordination doc does
by hand, entry by entry, comparing "#41 battery" vs "#42 battery" vs "#43 battery" in prose.

## 4. Exact current CLI flags and output paths

**Flags** (every `process.argv.includes`/`.find` check in the file, with gating verified):

| flag | line(s) | effect |
|---|---|---|
| `--interact` | 14, 90 | enables the interaction block (implied by any flag below or `--battery`) |
| `--battery` | 12, 90, 103, 104, 1140, 1146, 1222, 1275, 1431 | full turnkey sequence (§0 above); also sets `family=true` and is itself in `extraFlags`'s OR-list |
| `--reserved-family` | 103 | same `family` bucket as `--battery` (undo/selection/clipboard/locked/gumball) without the brush/suggestions/import/frame tail |
| `--clipboard` | 997 | `clipboard-copy-paste` step |
| `--marquee` | 1055 | `marquee-drag`+`marquee-click` steps — **not** in `family`'s OR-list, never runs under plain `--battery` |
| `--importexport` / `--import` | 1275 | `export-import` step |
| `--locked` | 1073 | `locked-refusal` step |
| `--brush` | 1140, 1146 | `frame-perspective` + `brush-stroke` steps |
| `--gumball` | 1108 | `gumball-drag` step |
| `--suggestions` | 1140, 1222 | `frame-perspective` + `suggestions-open` steps |
| `--undo` | 901, 932 | `history-open` + (`undo-once` if not `family`, else `undo-unwind`/`undo-redo`) |
| `--selection` | 969 | `selection-surfaces` step |
| `--fill` | 168, 211, 234, 254, 268, 867 | `tool-category`/`fill-tab`/`fill-abort-engagement`/`fill-wait-ready`/`fill-apply-max`/`fill-history` — independent lane, **not** implied by `--battery`, must be passed explicitly alongside it |
| `--frame` | 1140 | `frame-perspective` step alone |
| `--port=<n>` | 30 | target port, default `6013` |
| `--example=<name>` | 105, 143-144 | regex-match the example option by name instead of the default nth(1)/Nakagin heuristic |
| `--settle=<seconds>` | 150-151 | override the fixed 20s post-example-switch settle wait |

Header comment (line 3) documents only `[--interact\|--battery]` — **out of date** relative to the
actual 14-flag surface above; worth fixing so the next reader doesn't miss `--fill`/`--marquee`
being battery-exempt.

**Output paths** (all under `OUT = join(TICKET, "🗑️generated")`, line 9; `stamp` = ISO timestamp
with `:`/`.` → `-`, truncated to 19 chars, line 11, e.g. `2026-09-11T14-05-33`):

- `probe-<stamp>-boot.png` (79)
- `probe-<stamp>-<stepName>.png` — one per `step()` call regardless of pass/fail (101, inside the
  generic helper — covers every named step across all flags)
- `probe-<stamp>-brush-framed.png`, `probe-<stamp>-brush-vortices.png`,
  `probe-<stamp>-brush-hover-storm.png` — extra manual screenshots inside `brush-stroke`
  (1157-1158, 1189), additional to its own generic per-step screenshot
- `probe-<stamp>-suggestions-framed.png` — extra manual screenshot inside `suggestions-open` (1229)
- `probe-<stamp>-export.json` — `download.saveAs()` target for the exported fixture (1280, 1302)
- `probe-<stamp>-distinct.json` — self-synthesized fixture (export + one cloned/offset object)
  written via `fs.writeFileSync` for the `import-distinct` verdict (1367, 1382)
- `probe-<stamp>.md` — final report: `## verdicts` (lines containing `"verdict "` only),
  `## timeline` (full `lines[]`), `## faults (N)` (first 100 of `faults[]`), `## console tail`
  (last 1200 raw console lines) (1440-1443)

Run command per the file's own header: `bun 🔍️browser-probe.ts [--interact|--battery]`.
