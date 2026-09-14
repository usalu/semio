# Window coverage audit — generation3d React battery (2026-09-14)

Scope: the **React** renderer only (`SEMIO_BATTERY_URL=http://127.0.0.1:6018/?plugin=generation3d`). The wgpu renderer has its own separate battery (`🐍️wgpu-battery.mjs`) and ticket lane (`wgpu-*`) and is out of scope here. Read-only audit; no file touched except this one.

Sources: `🐍️react-battery.mjs` (363 lines, read in full), the 14 probe scripts it invokes, the latest `🗑️generated/react-verify/scoreboard.json` (16/16 green, 16 probes, 10 total pageerrors, run finished `2026-09-14T13:18:41Z`), the generation3d editor source under `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/{✏️editor,👁️viewer,📚️examples}`, and `.🛂️descriptor.semio`'s embedded label table.

---

## 1. Window / pane / surface inventory

| # | Surface | Window id | Mode / role | Source (file:line) | User actions it offers |
|---|---|---|---|---|---|
| 1 | **Flow** (node graph) | `procedural-main` | edit mode, editor role | `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:18` (id), `:27` (`definition()`), `:268` (`render`) | Canvas pan/zoom/fit (`nodeGraphViewport`), drag a node (`nodeGraphEdit move`), draw/cut a wire (`nodeGraphEdit connect/disconnect`), right-click context menu (editor.rs:2242-2257: Reorganize always; Translate/Rotate/Scale Selection, remove-widget/remove-generation, rename/update-values, import/export, delete-selection — all conditional on selection), accessible node/wire outline list with per-row eval-status label (flow/🦀️.rs:127-169), keyboard: `arrowdown/up/left/right` → select next/prev/upstream/downstream node, `Enter`-bound `activateSelection` ("Open Node Ports"), `delete`/`backspace` → deleteSelection, `mod+alt+l` → reorganize (editor.rs:2516,2532,2548-2551), LOD-mode header measure (flow/🦀️.rs:47). Window-owned action list: editor.rs:2447-2459. |
| 2 | **Preview** (3D, edit) | `procedural-preview` | edit mode, editor role | `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:15` (id), `:21` (def) | World3d orbit/pan/zoom camera (`setCamera`), hover/click/marquee select over the `graph` interaction domain (node/edge/handle granularity, multi/single, pick/rectangle, replace/additive/subtractive/invertive/range merge — editor.rs:2489-2506), gumball transform rail Move/Rotate/Scale (`translateSelection/rotateSelection/scaleSelection`, utilities editor.rs:2420, action-refs editor.rs:2460-2470), header measures: Show-mode picker (Shaded / Shaded+edges / Wireframe / Points, preview/🦀️.rs:43-70), Sun toggle + azimuth + elevation + intensity, status pill (Evaluated/Stale/Queued/Computing/Error/Blocked, terminology.rs status_* fields). |
| 3 | **Generations** (roster) | `generation3d-generations` | **generate mode only**, editor role | `.../🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:8` (id), `:13` (def), `:34` (render) | Row list "Generation N … Remove"; Add (`addGeneration`, chord `mod+shift+g` editor.rs:2533); inline rename editor (`renameGeneration`); row click select (`selectGeneration`); per-row Remove button (`removeGeneration`). Action-refs: editor.rs:2471. |
| 4 | **Form** (generate) | `generation3d-generate-form` | generate mode only | `.../🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:11` (id), `:16` (def), `:37` (render) | One slider/input per bound flow widget of the selected generation (`updateGenerationValues`); empty-state hint "Add a generation to edit input values." (`generate_hint`) when nothing selected. Action-refs: editor.rs:2472 (only `updateGenerationValues`). |
| 5 | **Generate Preview** | `generation3d-generate-preview` | generate mode only | `.../🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:19` (id), `:25` (def) | Same World3d camera/select/gumball/show/sun surface as row 2 (action-refs editor.rs:2473-2482, utilities editor.rs:2427 — historically MISSING until fixed, see comment editor.rs:2422-2426); empty-state hint "(evaluate a generation to preview output)" (`preview_hint`). |
| 6 | **Viewer Preview** | `procedural-view-preview` | **viewer role** — separate `view` mode/module, not edit/generate | `✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:29` (id), `:44` (def), mode id `👁️viewer/🎭️modes/👁️view/🦀️.rs:10` | Read-only camera + hover/select (interaction domain `graph`, channel `pointer`, **handle-only granularity** — narrower than the editor's node/edge/handle, lines 35-39), Show-mode/LOD/Sun display toggles, example switch (`setActiveExample`), Export. No import, no node graph, no gumball/transform, no Generations/Form (viewer's `🎮️commands/` folder has no `import-document*`, `add-widget`, `add-generation`, `node-graph-edit`, or `translate/rotate/scale-selection`). |
| 7 | **Catalogue** panel | `panel:procedural-play-catalogue` | chrome side panel | `📌️panels/🛍️catalogue/🦀️.rs:31` (def), `:123` (render), `:151` (tree) | ~20+ rows grouped by operator category (brep/curve/…); click a row → `addWidget` onto the graph. |
| 8 | **Inspection** panel | `panel:procedural-play-inspector` | chrome side panel | `📌️panels/🔍️inspection/🦀️.rs:15` (def), `:27` (render), `:43` (empty state) | Shows the selected widget's Id/Value(+editable input)/Range fields; "No selection" (`no_selection`) hint when nothing is selected. |
| 9 | **Document (Artifact)** panel | `panel:procedural-play-document` | chrome side panel | `📌️panels/🗿️artifact/🦀️.rs:18` (def), `:75` (render) | Flat list of every widget in the document; row click → select. |
| 10 | **Navbar / chrome** | — | app-scoped | editor.rs:2277-2420 (`.action_with`/`.shell_action` calls) | Example picker (`setActiveExample`, 8 bundled examples + "No example" — `.action_args("setActiveExample", …)` editor.rs ~2417), mode switcher (edit/generate), Actions palette (~38 rows across categories Actions/Targets/Create/Methods/Transform/History — see gaps probe `actions-pane-de` dump), Import Document…/Export Document… rows with a format picker (`export_format_options()`). |
| 11 | **Status pill** | — | per preview window | terminology.rs (status_ok/stale/queued/computing/error/blocked) | Read-only phase readout; no user action, but 5 of its 6 states are never driven in the battery (see §5). |
| 12 | **Dialogs** | — | modal/staged forms | editor.rs 2295-2419 | Import Document (native file picker via `importDocumentRequest`), Export Document format-select form, Set Active Example select form. |
| 13 | **Gumball / transform rail** | utility overlay on Preview/Generate-Preview | edit+generate, editor role | editor.rs:2418-2427 | Move / Rotate / Scale buttons over the 3D canvas when a selection is active (confirmed live: generate-mode battery step `gumball-rail` → `["Move","Rotate","Scale"]`). |

---

## 2. Action → battery step, or UNCOVERED

Grounded in `🐍️react-battery.mjs`'s `PROBES` array (lines 44-305) and the probe scripts it runs.

| Window / action | Battery coverage |
|---|---|
| Flow: node drag/select/wire connect/disconnect | `flow-wire` (react-battery.mjs:146-160, wire diff before/cut/redraw) |
| Flow: `mod+alt+l` reorganize | `flow-reorganize` (:163-172) |
| Flow: canvas paints at all | `flow-window` (:128-143, regex on `dag draw lod=…`) |
| Flow: right-click context menu contents/groups | **UNCOVERED** by the battery — `menu-dump-probe.mjs` exists (dumps the Generations-window Actions menu) but is not in `PROBES` |
| Flow: arrow-key node traversal + `Enter`/`activateSelection` | **UNCOVERED** — `graph-keyboard-nav-probe.mjs` exists, purpose-built for exactly these 5 verbs, not wired into `PROBES` |
| Flow: `delete`/`backspace`, `mod+z`/`mod+shift+z` | `keyboard-verbs` (:193-207) — but see §3, only presence-of-some-invocation is checked |
| Preview: hover/select/orbit camera | `interact` (:87-115) |
| Preview: gumball Move/Rotate/Scale | `generate-mode` step `gumball-rail`/`gumball-lane` (:118-125) — generate mode only; **edit-mode gumball is UNCOVERED** |
| Preview: Show-mode picker (Shaded/Wireframe/Points/+edges) | **UNCOVERED** — no probe clicks this measure; only the keyboard *cycle* chord is tested |
| Preview: Sun toggle/azimuth/elevation/intensity | **UNCOVERED** — no battery step touches `toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity` |
| Preview: `mod+alt+d` cycleShowMode, `mod+alt+k` cycleLodMode | `keyboard-verbs` steps `cycle-show-mode`/`cycle-lod-mode` (:205) |
| Generations: add/select/rename/remove | `generate-mode` steps `add-1/add-2/select/rename-typed/rename/remove-click/remove` (:118-125) |
| Generations: `mod+shift+g` chord | `gaps` step `generate-chord` (react-gap-probe.mjs ~395-423) |
| Form: slider edit → mesh re-eval | `generate-mode` step `form` (:118-125); also `gaps` step `inspection-edit`/`inspection-preview-rearm` for the Inspection-panel path |
| Import/Export document (UI click path) | `io-surface` (:210-219, but see §3 — its verdict is wired wrong) and `export-encoding` (:221-229, correctly wired) |
| Import/Export via keyboard (`mod+o`, `mod+shift+e`) | **UNCOVERED** — no probe presses these chords; `keyboard-verbs`'s row set is baseline/cycle-show/cycle-lod/add-generation/undo/redo/cancel-preview-eval only |
| Catalogue: add a widget | `gaps` step `catalogue-add` (react-gap-probe.mjs 312-333) |
| Inspection panel: select + edit value | `gaps` steps `doc-panel-select`/`inspection-edit` (react-gap-probe.mjs 211-308) |
| Document panel: row select | `gaps` step `doc-panel-select` |
| Navbar example picker (8 examples + No example) | `journey` (:66-84), both edit and view rows |
| Navbar mode switch (edit/generate/viewer) | `role-switch` (:245-256) — see §3 for how shallow |
| Actions palette contents + German labels | `gaps` step `actions-pane-de` — only **5 of ~38** action labels are checked (addWidget, exportDocument, importDocumentRequest, reorganize, deleteSelection); the rest (sun controls, LOD, show-mode, undo/redo, copy/cut/paste, select-all, checkpoints/alternatives) are **UNCOVERED** in German |
| Dock resize / window focus | `gaps` steps `dock-resize`/`window-focus` |
| Undo/redo (global) | `keyboard-verbs` steps `undo`/`redo`; `gaps` step `wire-undo` |
| Cancel a running preview eval | `cancel-preview` (:175-190) |
| Locale/appearance persistence across reload | `customization-persistence` (:282-293) |
| Accessibility (named/focusable/live shells) | `i18n-a11y` (:258-279) |
| Viewer role: hover/select/camera/show/sun/LOD/export while read-only | **Only window-mount is checked** (`role-switch` step `mid-chain`/`post-viewer`); none of viewer's own actions are individually exercised by the battery |

---

## 3. Steps that pass vacuously

- **`io-surface` — all 6 steps (`boot`,`published`,`export-form-open`,`export`,`import`,`import-settled`)**: `react-battery.mjs:216` reads `x.ok !== undefined ? Boolean(x.ok) : !x.error`, but `🐍️io-surface-probe.mjs`'s `record()` never sets a top-level `.ok`/`.error` on any row (real failures live nested at `.exported.error`/`.imported.error`). `x.ok` and `x.error` are therefore always `undefined`, so `!x.error` is always `true` — **every step is structurally incapable of failing**, regardless of what the (otherwise real) probe actually observed. This is the single most consequential vacuous-step finding: io-surface is nominally the P0 import/export gate.
- **`keyboard-verbs`**: `baseline`, `add-generation`, `cancel-preview-eval` are explicitly `ok: undefined` via the `optional` map (`react-battery.mjs:204-205`) — structurally excluded from pass/fail forever. Of the remaining 5 rows (`cycle-show-mode` ×2, `cycle-lod-mode`, `undo`, `redo`), each only checks `(invoked ?? []).length > 0` — i.e. that *something* was invoked, never that the chord's *own* action id fired (`🐍️editor-verbs-keyboard-probe.mjs` `invocationsSince`, ~line 58).
- **`interact`**: `hover publishes a target` / `click publishes a selection` / `drag publishes a camera` / `no unknown merge` (`react-battery.mjs:100-109`) are plain regex tests (`/hoverTarget|interactionHover/`, `/selectedIds|interactionSelect/`, `/setCamera/`) over the whole accumulated console dump, not scoped to the specific gesture step.
- **`flow-window`**'s `node graph paints` (`react-battery.mjs:137`) passes on any `dag draw lod=… zoom=…` line appearing at least once — presence, not correctness.
- **`role-switch`**'s 6 window rows (`react-battery.mjs:251`) only check `windows.length > 0`; the `roles.editor/viewer` and `modes.edit/generate` fields are captured and printed but never used to decide `ok`, even though the probe's own polling loop used that exact predicate to know when to stop waiting.
- **`status-parity`**'s 10 rows (`react-battery.mjs:239`) only check that a `status` object is non-null on the preview host(s) — never its `phase`/`ratio` value, and despite the probe's name, never a cross-window parity comparison.
- **`gaps` → `generate-chord`** (react-gap-probe.mjs ~422): decided by a console regex for the literal action id `addGeneration`, not by the mesh-count-before/after evidence the probe itself already collected in the same step's `detail`.
- **Page errors/faults are recorded but almost never gate a probe.** Of the 16 probes, only `boot` (`react-battery.mjs:58`) and the `flow-window` mode of `flow-window-probe.mjs` (`:138`) turn `pageerrors===0` into a pass/fail step. The other 14 probes compute `pageerrors`/`faults` purely for the scoreboard's informational summary (`summarize()`, :319-325). This run recorded **10 pageerrors total** — a recurring `TypeError: Cannot read properties of null (reading 'addEventListener')` across `journey`, `interact`, `generate-mode`, `role-switch`, `i18n-a11y`, `gaps`, plus a `FlowMessageRejected: Flow message rejected` in `flow-wire` — none of which failed anything; the battery still reported `green=16/16`.
- **`customization-persistence-probe.mjs` never registers a `page.on("pageerror")` listener at all** — its `pageerrors:0` (`react-battery.mjs:291`, hardcoded) means "not observed," not "zero occurred."

---

## 4. Placeholder / empty / TODO content

- **No genuine `todo!`/`unimplemented!`/`FIXME`/"not yet implemented" hits anywhere in the editor tree** (grepped `✏️editor/**/*.rs,*.ts`, excluding tests).
- The only `placeholder`-named API use is legitimate, intended fallback content: `flow/🦀️.rs:163-164` calls `.section_or_placeholder(...)` to show `"(no nodes)"`/`"(no wires)"` (`graph_empty`/`graph_unwired` labels) when the Flow outline is empty. Not a bug.
- **The `📌️.empty.md` marker files are scaffolding, not gaps.** Every window's `🎬️actions/` subfolder across the whole tree (edit/preview, edit/flow, generate/preview, generate/form, generate/generations, plus the viewer's preview window) contains only `.empty.md` — zero exceptions found anywhere in the tree. `editor.rs:2434-2446` explains why: window-scoped actions are declared only where a window's OWN surface dispatches something outside the shared/app-scoped catalogue (`window_kind_action_refs`); everything else — including every app-wide verb — is intentionally left off every window and copied on by the framework's `build_definition`. This is confirmed structural convention, not missing functionality.
- Two legitimate, currently-**untested** empty-state UI strings exist: Inspection panel's `"No selection"` (`no_selection`, inspection/🦀️.rs:43) and Form/Generate-Preview's `"Add a generation to edit input values."` / `"(evaluate a generation to preview output)"` (`generate_hint`/`preview_hint`). None of the three appear in any battery i18n check (see §5 item 5).
- **Stronger i18n finding (not merely untested — structurally unlocalizable):** the LOD picker's `"LOD"` label (`flow/🦀️.rs:51`), the Show-mode picker's `"Show"` label and its 4 row labels `"Shaded"/"Shaded + edges"/"Wireframe"/"Points"` (`edit/🪟️windows/👁️preview/🦀️.rs:43-49,57`), and the whole Sun measure group (`"Sun"/"Enabled"/"Azimuth"/"Elevation"/"Intensity"`, framework helper `world3d_sun_measures`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:36716-36757`) are all plain hardcoded English strings that sit entirely OUTSIDE `Generation3dLabels` — they have no `native_de` form to test even if a probe tried. The corresponding command-palette entries for the same verbs (`"Toggle Sun"/"Sonne umschalten"`, `"Set Lod Mode"/"LOD-Modus festlegen"`, etc., `editor.rs:2313-2325`) ARE bilingual, so a German-locale user sees a correctly-translated palette action next to an always-English chrome widget for the identical control. Catalogue category names (from `CatalogueSection.title`, `📌️panels/🛍️catalogue/🦀️.rs:113-118`) are English-only for the same reason.

---

## 5. Top 10 user-visible gaps to close next

1. **`io-surface` battery verdict cannot fail.** Fix `react-battery.mjs:216`'s `x.ok`/`x.error` read (or `io-surface-probe.mjs`'s `record()`) so a real import/export failure is detected. *Owner: `🐍️react-battery.mjs:216`, `🐍️io-surface-probe.mjs` `record()`.*
2. **Keyboard-only node-graph traversal (arrows + Enter/`activateSelection`) has zero battery coverage** despite a purpose-built, ready probe sitting unused. *Owner: `🐍️graph-keyboard-nav-probe.mjs` (wire into `react-battery.mjs` `PROBES`).*
3. **Page errors don't gate 14 of 16 probes.** A real recurring `TypeError…addEventListener` and a `FlowMessageRejected` fault ride along in a "16/16 green" run. *Owner: `🐍️react-battery.mjs` (each probe's `verdict`, e.g. :84,115,159,171,189,218,228,242,255,279,292,303).*
4. **The LOD picker, Show-mode picker, and entire Sun measure group render as hardcoded, unlocalized English chrome** (`"LOD"`, `"Show"`/`"Shaded"`/`"Wireframe"`/`"Points"`, `"Sun"`/`"Enabled"`/`"Azimuth"`/`"Elevation"`/`"Intensity"`) — they live outside `Generation3dLabels` entirely, so no i18n probe could catch this even if pointed at them, while the identical verbs' palette entries ARE German-localized. Also never clicked by any battery step. *Owner: `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:43-49,57` (Show), `🕸️flow/🦀️.rs:51` (LOD), `🧰️framework/.../🔌️plugin/🦀️.rs:36716-36757` (Sun, shared helper) — needs both localization and a battery step.*
5. **22 of the 39 `Generation3dLabels` fields are never asserted in German** (status_stale/queued/computing/error/blocked, no_selection, generate_hint, preview_hint, id_field, value_field, range_field, widget_group, catalog_neuron, catalog_preview, window_generate_preview, delete_selection, graph_empty, graph_unwired, graph_canvas_hint, preview_canvas_hint, schema_prefix, widgets_prefix) — and two purpose-built probes for the reachable ones already exist unwired. *Owner: `🗣️terminology/🦀️.rs` (source), `🐍️panel-i18n-probe.mjs` / `🐍️inspection-i18n-probe.mjs` (wire into battery).*
6. **Actions-palette i18n only checks 5 of ~38 labels.** *Owner: `🐍️react-gap-probe.mjs` `actions-pane-de` step (~335-376).*
7. **Status pill states beyond "Evaluated" are never driven or observed.** *Owner: `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` (status rendering) + `🐍️status-parity-probe.mjs`/`react-battery.mjs:239`.*
8. **Edit-mode gumball (Move/Rotate/Scale on the plain edit Preview, not generate) is untested** — only `generate-mode`'s gumball steps run. *Owner: `🐍️generate-mode-probe.mjs` or a new edit-mode counterpart.*
9. **Viewer role's own actions (show/LOD/sun/example-switch/export, and confirming import/gumball/node-graph are truly absent) are not individually exercised** — only that some window mounted. *Owner: `🐍️role-switch-runtime-probe.mjs` + `react-battery.mjs:251`.*
10. **The Flow window's right-click context menu and the full export-format option list are never exercised**, despite a ready dump probe. *Owner: `🐍️menu-dump-probe.mjs` (wire into battery), `editor.rs:2242-2257` (menu contents), `export_format_options()`.*
