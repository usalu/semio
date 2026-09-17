# E3 — puzzle 🖐️5d source-measured inventory and gap vs 🧊️3d

Read-only source audit, 2026-09-17. No cargo/nx/build run — everything below is `grep -n`/`Read` on the
tree at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{🖐️5d,🧊️3d,◻️2d}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`.
5d root file: `…/🖐️5d/…/✏️editor/🦀️.rs` (8,226 lines). 3d root: `…/🧊️3d/…/✏️editor/🦀️.rs` (~7,900+ lines,
action registry 8125-8180 region). Cross-referenced against
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-user-feature-checklist.md`
(the "checklist", 25 sections) which documents 3d's now-fixed state.

## 0. Headline number

| artifact | `Migrated` | `BatchOnlyPendingRewrite` | `Unclassified` | retained-tool-registry size |
|---|---|---|---|---|
| 🧊️3d | 64 | **0** | 0 | ~62 tools (`🦀️.rs:7608-7631`) + 2 host-config tools (`setActiveTool`/`setActiveUtility`, `🦀️.rs:7633-7640`) |
| ◻️2d | 38 | **0** | 0 | (not enumerated this pass — 0 dead is the headline) |
| **🖐️5d** | **17** | **35** | 0 | **16** tools (`🦀️.rs:7626-7652`, `PUZZLE5D_RETAINED_TOOL_IDS` at `🦀️.rs:3863`) |

3d and 2d finished migrating every action off `BatchOnlyPendingRewrite` (the repo's known hard-dead
runtime-fault pattern — memory: *"BatchOnlyPendingRewrite verbs are hard-dead in the app"*). **5d never
did this pass at all**: two-thirds of its 52 registered actions (35/52) are still
`BatchOnlyPendingRewrite`, i.e. **unreachable from the live app today** even though most of the handler
bodies exist and look complete. This one fact dominates every section below — 5d's runtime surface is
roughly the 16-tool slice of what 3d had *before* its 09-09 migration waves.

`factory_type` check (memory: *"bare bounded factory means every action is dead"*): 5d's factory is
**not bare** — `Puzzle5dRetainedCommandJobFactory` (`🦀️.rs:7632`) is concretely implemented with a real
16-id tool list and a `build_tool_job` match (`🦀️.rs:7659-7669+`). The 16 `Migrated` actions are genuinely
live. The other 35 are dead not because of a bare-factory bug but because they were simply never added to
`action_interactive_job(..., Migrated)` / the tools list — an incomplete migration, not a wiring bug.

## 1. Full classification table (5d, `🦀️.rs:8131-8177`)

**Migrated (16, live):** `canvasPointerDown`, `cycleBrushCandidate`, `deleteSelection`,
`duplicateSelection`, `engagementAbort`, `engagementControlSelect`, `engagementInput`,
`engagementSubmit`, `importComposeKit`, `selectSameKindSelection`, `setFillCount`, `setFixtureJson`,
`setSelectionFlag`, `targetBrushSuggestions`, `worldPointerDown`, `zoomToSelection`.

**BatchOnlyPendingRewrite (35, dead — click does nothing, no fault surfaced to the user):**
`addBrushObject`, `addBrushPart`, `addNode`, `addPartKind`, `applyBoardEvents`, `createFastener`,
`deleteFastener`, `editFastener`, `focusSelection`, `patchFastener`, `patchGrip`, `patchPart`,
`proximityConnect`, `registerBrushMesh`, `retargetFastener`, `rotateSelection`, `scaleSelection`,
`selectSameKind`, `setActiveExample`, `setBrushPlacementContactTolerance`, `setCamera`, `setCamera2d`,
`setCamera3d`, `setGridFactor`, `setGridSnapEnabled`, `setLodMode`, `setObjectKindWeight`,
`setSuggestionOffset`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `setVortexKindWeight`,
`toggleSun`, `translateSelection`, `worldRelocate`.

Consequence, read literally: **camera control does not work at all** (`setCamera`/`setCamera2d`/
`setCamera3d` all dead — no orbit/pan/zoom in either the board or world window), **the transform gumball
does not work** (`translateSelection`/`rotateSelection`/`scaleSelection` dead), **no fastener CRUD**
(`createFastener`/`editFastener`/`deleteFastener`/`retargetFastener`/`proximityConnect` all dead — an
Edge/Attraction/Fastener cannot be created, edited, retargeted or auto-connected), **no part/node
creation via brush or catalogue-add** (`addNode`/`addPartKind`/`addBrushObject`/`addBrushPart` dead — only
pre-existing document content is usable), **the inspector cannot write back** (`patchPart`/`patchGrip`/
`patchFastener` all dead — see §16 below), **no example switching** (`setActiveExample` dead — the
document is whatever loaded at boot), and **no window options work** (`setGridFactor`/`setGridSnapEnabled`/
`toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity`/`setLodMode` all dead). The handler
bodies for nearly all of these exist and read as complete (e.g. `set-active-example/🦀️.rs:14` loads one
of `capsule_dream`/`concrete_forest`/`nakagin` examples correctly) — they are simply unreachable from
`dispatch_action` today.

## 2. Retained tool / publication-lane registries

- `🦀️.rs:7626-7652` — `bounded_first_step_tool_proofs!` block, `factory: "Puzzle5dRetainedCommandJobFactory"`,
  `contract: ToolExecutionContract::resumable(8_192, 512, 1, 262_144, 7_500, 1, 1)`, 16-tool list (matches
  Migrated set above minus one duplicate-name artifact). Compare 3d `🦀️.rs:7608-7631`: same macro, `contract:
  resumable(262_144, 16_384, 1, 262_144, 7_500, 1, 1)` — **3d's per-turn byte grant is 32× 5d's** (262,144
  vs 8,192 bytes) and its work-item ceiling 32× (16,384 vs 512). If 5d's contract is a leftover default
  rather than a deliberate product choice, list documents/fixtures the size of Nakagin will likely hit the
  same `PUZZLE_COMMAND_RAW_BYTES`/`Work::extent()` walls 3d hit before its own contract was widened
  (checklist §11, §24) — **unverified against a live document, flag for the coordinator**.
- `PUZZLE5D_WINDOW_TOOL_IDS` (`🦀️.rs:3881`): `cycleBrushCandidate`, `engagementAbort`,
  `engagementControlSelect`, `engagementInput`, `engagementSubmit`, `setFillCount`, `targetBrushSuggestions`,
  `zoomToSelection` — routed through `Puzzle5dWindowCommandWork` (`🦀️.rs:7668`). Window *option* commands
  (grid/lod/sun/vortex) are conspicuously absent from this list — they were never wired into the
  window-command path at all, consistent with their `BatchOnlyPendingRewrite` status.
- No `setActiveTool`/`setActiveUtility` action registration and no `host_configuration_mutation` override
  anywhere in 5d's `🦀️.rs` (only referenced inside `🧪️tests/🔬️unit/🦀️.rs:136`). 3d has an explicit second
  `bounded_first_step_tool_proofs!` block (`🦀️.rs:7633-7640`, factory `BoundedFirstStepCommandJobFactory`)
  dedicated to these two host-owned verbs, following the coordinator patch documented in checklist §12.
  **5d has no equivalent block — utility/tool activation for 5d rides entirely on the generic framework
  default with no puzzle5d-specific proof, unverified whether it degrades the same way 3d's did pre-patch.**
- `command_from_action` exists (`🦀️.rs:7814`), so the action-id→command dispatch itself is wired; the
  gate that's missing is purely the `Migrated` classification + tool registration for the 35 dead ids.

## 3. Tools: no `ToolDefinition`/`ToolRunDefinition` exists in 5d at all — Fill and Brush are utilities only

3d declares Fill as a first-class **framework Tool** with a `ToolRunDefinition` (`🎭️modes/✏️edit/🛠️tools/
🪣️fill/🦀️.rs`, `TOOL_ID = "fill"`, `run: Some(run_definition())`, `RUN_JOB_KIND`/`REVALIDATE_JOB_KIND`,
`ToolRunSettingsReads`) plus the `tool.fill` activate toggle and `#tool.fill` DOM id the checklist's §12
audits. **5d has no `🛠️tools/` directory at all** (confirmed by directory listing — 5d only has
`🎭️modes/✏️edit/☑️options/{🖌️brush,🪣️fill}`, mode-level *option* declarations, not tools). 5d's
`.build_definition()` chain (`🦀️.rs:8188-8201`) only calls `.utility(...)` seven times (select, move,
rotate, scale, brush, fill, world-relocate) — **zero `.tool(...)` calls**. `build_tool_run_job`
(`🦀️.rs:7608-7613`) branches only on `board2d::utilities::brush::UTILITY_ID`, falling through to
`fill::build_run_job` for every other tool_id — i.e. Fill in 5d is reached as the *default* branch of a
utility-run dispatch, not through a real `ToolDefinition`. **This means there is no `#tool.fill`
activate control, no `setActiveTool{toolId:"fill"}` path, and no ToolRun panel start/pause/resume/step/
abort/finalize/dismiss chrome for 5d's fill** the way 3d's checklist §12 describes — fill in 5d can only
be reached implicitly via the Fill *utility* rail, if utility activation itself works (see §2's
unverified gap above).

Precompute (`🧠️precompute/{📐️geometry,🖌️brush,🪣️fill}/🦀️.rs`) is drastically thinner than 3d's
(geometry 92 vs 2,055 lines; brush 56 vs 1,066; fill 69 vs 3,293; root 544 vs 1,387) — this is **not**
necessarily a gap: the 5d root comment (`🦀️.rs:7606-7607`) states 5d's planners deliberately wrap "the
puzzle 3d planners behind the 5d translation" rather than reimplementing. The thin files are translation
shims; correctness of that translation was not verified in this pass (would need a live probe).

## 4. Windows / options — grid, LOD, sun, vortex, select

5d option-file inventory (mode-level, per `🎭️modes/✏️edit/☑️options/` and per-window
`🪟️windows/{◻️2d,🧊️3d}/☑️options/`):
- `☑️options/🖌️brush/🦀️.rs`, `☑️options/🪣️fill/🦀️.rs` — mode-level (shared by both windows, per the file's
  own doc comment).
- `🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs` — 2d window's only option.
- `🪟️windows/🧊️3d/☑️options/☀️sun/🦀️.rs` — 3d window's only option.

**No grid option, no vortex-show/vortex-direction option, no projection option exist anywhere in 5d** —
compare 3d's six mode-level option files (`sun`, `vortex`, `grid`, `projection`, `select`, `lod`,
`🎭️modes/✏️edit/☑️options/`). Even where a 5d option *file* exists (sun, lod), its backing action is
`BatchOnlyPendingRewrite` (`toggleSun`/`setSunAzimuth`/`setSunElevation`/`setSunIntensity`/`setLodMode` —
§1), so the option renders but is inert on click. **DONE: none. PARTIAL: none (LOD/Sun UI renders but
dead). MISSING: grid, vortex show/direction, projection — no UI at all.**

## 5. Panels — inspection, artifact/outliner, catalogue, settings

| panel | 5d file | 5d lines | 3d file | 3d lines | 5d state |
|---|---|---|---|---|---|
| Inspection | `📌️panels/🔍️inspection/🦀️.rs` | 48 | `📌️panels/🔍️inspection/🦀️.rs` | 252 | **regressed to 3d's PRE-W-S state** |
| Artifact/outliner | `📌️panels/🗿️artifact/🦀️.rs` | 108 | `📌️panels/🗿️artifact/🦀️.rs` | 192 | thinner, not diffed line-by-line |
| Catalogue | `📌️panels/🛍️catalogue/🦀️.rs` | 126 | `📌️panels/🛍️catalogue/🦀️.rs` | 160 | thinner, not diffed line-by-line |
| Settings | **does not exist** | — | `📌️panels/⚙️settings/🦀️.rs` | 69 | **MISSING entirely** |

Inspection panel (`📌️panels/🔍️inspection/🦀️.rs:1-4,26-30`) has an explicit doc comment: *"this used to
switch on live selection (grip wins over part wins over fastener) to show one editable field group per
resolved entity … now always renders the document summary"* because `ArtifactApp::render` never gained
the `InteractionView` parameter the framework-level selection threading needs — **the exact same defect
3d had before W-S** (checklist §16, "was permanently dead as a per-entity inspector"). 5d's own comment
says this is "flagged to the coordinator … not fixed here (framework file, out of this crate's remit)" —
i.e. **it is a known, acknowledged, still-open gap**, not a rediscovery. Even if the render-side selection
threading were fixed, `patchPart`/`patchGrip`/`patchFastener` are all `BatchOnlyPendingRewrite` (§1), so
the write-back path would still be dead — this is a **two-layer gap** (read: no per-entity fields; write:
no dispatch path).

Settings panel: 3d's four per-window steppers (`overlap-budget`→`setBrushPlacementOverlapBudget`,
`proximity-radius`→`setProximityRadius`, `chunk-size`→`setChunkSize`, `grid-spacing`→`setGridSpacing`,
checklist §19) have **no 5d equivalent at all** — no panel file, and none of `setProximityRadius`/
`setChunkSize`/`setGridSpacing`/`setBrushPlacementOverlapBudget` appear as action ids anywhere in 5d's
`🦀️.rs` (only `setBrushPlacementContactTolerance` exists, and it is dead). **MISSING entirely.**

## 6. Context menu

5d has its own `puzzle5d_context_menu_items` (`🦀️.rs:3436-3445`, wired at `🦀️.rs:7990-8006`) — a
context-menu builder exists (not missing wholesale), but was not read row-by-row against 3d's known
`zoomToSelection`-vs-`focusSelection` defect (checklist §15) in this pass. Given `zoomToSelection` IS in
5d's Migrated set while `focusSelection` is `BatchOnlyPendingRewrite` — the **opposite** polarity from 3d
(where `zoomToSelection` is the *unregistered* id and `focusSelection` is real) — if 5d's context-menu rows
were ported verbatim from 3d without checking which id is actually live in 5d's own registry, whichever
row uses `focusSelection` (if any) would silently no-op. **Needs a direct read of
`puzzle5d_context_menu_items`'s row action strings against §1's Migrated set — not completed this pass,
flag for a follow-up read.**

## 7. Import / export, examples, locale

- **Import/export**: no `exportFixture`/`importFixture`/`openImportFixture` command directories exist
  under `🎮️commands/` for 5d (3d has `📤️export-fixture/🦀️.rs`, `📥️import-fixture/🦀️.rs`,
  `🗂️open-import-fixture/🦀️.rs`). `setFixtureJson` is Migrated (live) in 5d — unlike 3d where it is
  `BatchOnlyPendingRewrite` per checklist §24 — so 5d's raw-JSON-load path may actually be more reachable
  than 3d's today, but there is still no dedicated Import/Export UI action, matching 3d's checklist §24
  conclusion ("does not exist as a usable end-user feature") for the menu-driven path specifically.
- **Examples**: `📚️examples/🎬️demo-session/` exists with the same four-file shape as 3d
  (`🟦️.ts`, `🦀️.rs`, `🖼️assets/🎮️.cmd.semio`, `🧪️tests/🧩️example/{🟦️.ts,🦀️.rs}`). `set-active-example/
  🦀️.rs:3-10` references three named examples: `capsule_dream_example_document`,
  `concrete_forest_example_document`, `nakagin_example_document` (`PUZZLE5D_EXAMPLE_CAPSULE_DREAM`/
  `_CONCRETE_FOREST`/`_NAKAGIN`) — parity in *content* with 3d's two (Concrete Forest, Nakagin), plus a
  third (Capsule Dream) 3d doesn't have. **But the switch action itself is dead (§1)**, so none of the
  three are reachable via the picker today; whatever loads at boot is the only visible example.
- **Locale**: `🗣️terminology/🦀️.rs` is 244 lines vs 3d's 260 — close to parity, not a material gap.

## 8. Runtime-fault pattern check (per the audit brief)

- **`BatchOnlyPendingRewrite` hard-dead**: confirmed, 35 actions (§1) — this is the dominant, systemic
  finding of this audit, not an isolated defect.
- **`Unclassified` aborting the descriptor probe**: 0 occurrences — 5d does not have this failure mode.
- **Bare bounded factory**: not present — `Puzzle5dRetainedCommandJobFactory` is concrete (§0).
- **Missing `command_from_action`**: not missing (`🦀️.rs:7814`).
- **Undeclared publication lanes**: `WindowConfig` lane exists (`🪟️window/🦀️.rs:227`,
  `MAXIMUM_PUBLICATION_BYTES: usize = 65_536`, matching 3d's post-W-D2 ceiling) — the W-D2 grant-vs-ceiling
  fix lives in shared framework code (`🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🦀️.rs:82`), so 5d
  should inherit it automatically; **not independently verified this pass** since the WindowConfig-lane
  actions that would exercise it (grid/sun/lod) are themselves dead at the dispatch gate (§1/§4) —
  i.e. 5d can't even reach the hang 3d had, because the actions that would trigger it never fire.
- **`UiFixedList`/`UiText` overflow**: only two `UiText::try_from_str` admission sites found (artifact
  panel `🦀️.rs:60`, catalogue panel `🦀️.rs:64`), both already admission-checked (`.ok_or_else(...)`) —
  no raw unchecked pushes found; not a live risk in the code read.
- **Scenes bypassing `scene_surface`**: not present — both 5d windows (`◻️2d/🦀️.rs`, `🧊️3d/🦀️.rs`) use
  `scene_surface`, matching 3d's pattern.
- **BOARD pointer capacities**: not specifically probed this pass (would need the board-host/engine
  layer, which lives in 2d's `⚙️engine/🎲️board-host`, not present in 5d's tree at all — 5d's 2d window
  is a thin wrapper, not a full board engine; out of scope for a source-only read without knowing the
  wire contract).

## 9. Section-by-section vs the 25-item 3d checklist — 5d status

| § | topic | 5d state | evidence |
|---|---|---|---|
| 1 | Windows (top/perspective↔2d/3d) | PARTIAL | 2d+3d windows both exist (`🪟️windows/◻️2d/🦀️.rs`, `🪟️windows/🧊️3d/🦀️.rs`); focus/pane semantics not separately audited |
| 2 | Camera orbit/pan/zoom | **MISSING** | `setCamera`/`setCamera2d`/`setCamera3d` all `BatchOnlyPendingRewrite`, `🦀️.rs:8161-8163` |
| 3 | Projection options | **MISSING** | no projection option file anywhere in 5d (§4) |
| 4 | Grid/LOD/vortex/sun window options | **MISSING** (UI stubs for sun+LOD only, both dead) | `🪟️windows/🧊️3d/☑️options/☀️sun/🦀️.rs`, `🪟️windows/◻️2d/☑️options/🔭️lod/🦀️.rs`; actions dead §1 |
| 5 | Example switcher | **MISSING** (3 examples defined, unreachable) | `set-active-example/🦀️.rs:3-32`; `setActiveExample` dead `🦀️.rs:8159` |
| 6 | Selection (click/marquee/tree/select-same-kind/select-all/clear) | PARTIAL | `canvasPointerDown`/`worldPointerDown`/`selectSameKindSelection` Migrated; plain `selectSameKind` dead (`🦀️.rs:8157`, likely the non-framework variant); select-all/clear are framework-generic, not separately gated |
| 7 | Hover | PARTIAL/unverified | no dedicated hover command id found distinct from pointer-down handling; not confirmed working or dead |
| 8 | Gumball/transform | **MISSING** | `translateSelection`/`rotateSelection`/`scaleSelection` all `BatchOnlyPendingRewrite`, `🦀️.rs:8155-8156,8181` (utility-flag toggles for move/rotate/scale exist via `.utility(world3d::utilities::transform::*)`, `🦀️.rs:8196-8198`, but the commit dispatch is dead) |
| 9 | Brush utility | PARTIAL | `cycleBrushCandidate` Migrated; `addBrushObject`/`addBrushPart`/`registerBrushMesh`/`setBrushPlacementContactTolerance` all dead — brush can preview/cycle but **cannot place** |
| 10 | Volume Brush | **MISSING** | no `addTargetVolume`/`setVoxelDims`-equivalent action or utility found in 5d at all — 5d has no volume-brush utility (compare 3d's `🪛️utilities/🧊️volume-brush`) |
| 11 | Relocate | **MISSING** | `worldRelocate` `BatchOnlyPendingRewrite`, `🦀️.rs:8177` (utility exists — `.utility(world3d::utilities::world_relocate::definition())`, `🦀️.rs:8201` — but its commit is dead) |
| 12 | Fill tool | PARTIAL | `setFillCount` Migrated and precompute wired (§3), but no real `ToolDefinition`/activate control exists — reached only via the Fill utility rail, no ToolRun panel chrome |
| 13 | Vortex/Grip suggestions | PARTIAL | `targetBrushSuggestions` Migrated; `setSuggestionOffset` dead (`🦀️.rs:8172`) — offset adjustment inert |
| 14 | Engagement bar | DONE | all five (`engagementInput`/`Submit`/`RepeatLast`-equiv/`Abort`/`ControlSelect`) — 4 of 5 Migrated (no explicit `engagementRepeatLast` id found in 5d's registry at all, unlike 3d — **MISSING**, not just unverified) |
| 15 | Context menu | PARTIAL/unverified | builder exists (§6) but row-by-row action-id correctness vs the Migrated set not verified this pass |
| 16 | Inspection panel | **MISSING** (regressed to pre-fix state) | `📌️panels/🔍️inspection/🦀️.rs:1-4,26-30`, explicit doc comment describing the same defect 3d fixed in W-S; write-back (`patchPart`/`patchGrip`) also dead |
| 17 | Artifact/outliner panel | PARTIAL | exists, thinner (108 vs 192 lines), row-toggle correctness (3d's §17 `flag_args` hardcoded-`true` bug) not independently checked for 5d |
| 18 | Catalogue panel | PARTIAL | exists (126 lines), but its `addObjectKind`-equivalent (`addNode`/`addPartKind`) is `BatchOnlyPendingRewrite` — clicking a kind row cannot add anything |
| 19 | Settings panel | **MISSING** | no panel file, no backing actions (§5) |
| 20 | History (undo/redo/checkpoint/revert) | DONE (framework-generic) | framework-reserved routes, not puzzle5d-specific — same caveat as 3d's checklist §20 (wasm pump risk), not re-verified |
| 21 | Copy/cut/paste | DONE (framework-generic, N/A) | no app-specific handler in either 3d or 5d, by design |
| 22 | Delete/duplicate/focus selection | PARTIAL | `deleteSelection`/`duplicateSelection` Migrated; `focusSelection` dead (`🦀️.rs:8147`) — F-key zoom-to-selection inert even though `zoomToSelection` itself is Migrated (naming collision risk, see §6) |
| 23 | Add Object/Part dialog | **MISSING** | `addPartKind`/`addBrushPart`/`addBrushObject` all dead; the action-arg schema exists (`🦀️.rs:8183-8187`, hardcoded single "Part" option, same static-option defect as 3d's checklist §23) but nothing dispatches it |
| 24 | Import/export | **MISSING** (same conclusion as 3d, different mechanism) | §7 above |
| 25 | Locale/terminology | DONE (near-parity) | 244 vs 260 lines (§7) |

## 10. Prioritized OWED work list

Ordered by unblocking the most downstream functionality per action migrated (mirrors how 3d's own W-D2/
W-S/W-B waves worked: reclassify to `Migrated`, add to the retained-tool list/contract, port the actual
fix from 3d if one is needed beyond reclassification).

1. **Camera (`setCamera`, `setCamera2d`, `setCamera3d`)** — `🎮️commands/{🎥️set-camera,🖼️set-camera-2d,
   🎦️set-camera-3d}/🦀️.rs`, classify at `🦀️.rs:8161-8163`, add to `PUZZLE5D_RETAINED_TOOL_IDS`
   (`🦀️.rs:3863`) and the `bounded_first_step_tool_proofs!` tools list (`🦀️.rs:7634-7651`). No 3d port
   needed — 3d's `setCamera` (checklist §2) is itself just a per-window `WindowConfig` publish; 5d's
   `🎚️config/🦀️.rs`/`🪟️window/🦀️.rs` already carry the matching 65,536-byte ceiling (§8). Without this,
   nothing else in the 3d pane is drivable by a human.
2. **Transform commit (`translateSelection`/`rotateSelection`/`scaleSelection`)** — `🎮️commands/{🚀️translate-
   selection,🔄️rotate-selection,📏️scale-selection}/🦀️.rs`, `🦀️.rs:8155-8156,8181`. Port pattern from 3d's
   `🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️.rs` (gumball-active gating, one-delta-on-release
   semantics, checklist §8) — 5d's own `🪟️windows/🧊️3d/🎬️actions/{↔️translate-selection,🔄️rotate-
   selection,📐️scale-selection}/{🟦️.ts,🦀️.rs}` already exist as per-window action wrappers, so the
   command-layer classification is likely the *only* missing piece, not new logic.
3. **Fastener CRUD (`createFastener`/`editFastener`/`deleteFastener`/`retargetFastener`/
   `proximityConnect`)** — `🎮️commands/{🔗️create-fastener,🔧️edit-fastener,💔️delete-fastener,🎯️retarget-
   fastener,📡️proximity-connect}/🦀️.rs`. Port from 3d's `createAttraction`/`deleteAttraction`/
   `setProximityRadius`+auto-connect (checklist §11, §15) — 5d's handler bodies already exist and read
   complete; classify + register.
4. **Inspector write-back (`patchPart`/`patchGrip`/`patchFastener`)** — `🎮️commands/{🩹️patch-part,✊️patch-
   grip,🪛️patch-fastener}/🦀️.rs` (75/72/80 lines each, look complete). Needs BOTH the classification fix
   AND the render-side `InteractionView` threading fix 3d got in W-S (`📌️panels/🔍️inspection/🦀️.rs:26-30`'s
   own doc comment says this half is a framework-level gap "out of this crate's remit" — port the pattern
   from 3d's `📌️panels/🔍️inspection/🦀️.rs` per-granularity field groups, `selected_object_inspector_renders_
   that_object_field_group`-equivalent tests).
5. **Example switching (`setActiveExample`)** — `🎮️commands/🛍️set-active-example/🦀️.rs:14-32`, classify
   at `🦀️.rs:8159`. Handler already loads all three named examples correctly; this alone unblocks §5/§9/§18
   of the checklist-mirror table above (nothing is drivable beyond the boot document today).
6. **Add Part/Node (`addNode`, `addPartKind`, `addBrushPart`, `addBrushObject`, `registerBrushMesh`)** —
   `🎮️commands/{🌱️add-node,🏷️add-part-kind,🖌️add-brush-part,🖌️add-brush-object,📋️register-brush-mesh}/
   🦀️.rs`. Port re-select-on-place and the "only handles `Ok(Fixture(_))`, silent-fail on collision" caveat
   from 3d's `addBrushObject`/`acceptSuggestion` (checklist §9/§13) so 5d doesn't reproduce the same silent
   failure mode on day one.
7. **Window options (grid/LOD/sun) — build the missing option files AND classify** — 5d has no grid or
   vortex option at all (§4); LOD/sun option files exist but their actions (`setGridFactor`,
   `setGridSnapEnabled`, `toggleSun`, `setSunAzimuth/Elevation/Intensity`, `setLodMode`) are all dead.
   Port option-file shapes from 3d's `🎭️modes/✏️edit/☑️options/{🌐️grid,🎥️projection}/🦀️.rs` (5d has no
   projection concept per se given it's a fixed dual-window layout, so grid is the real gap; vortex
   show/direction may not apply to 5d's Grip vocabulary — confirm with the product owner before porting).
8. **Fill as a real Tool** — create `🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` porting 3d's `ToolDefinition`/
   `ToolRunDefinition` shape (`TOOL_ID`, `run_definition()`, `RUN_JOB_KIND`/`REVALIDATE_JOB_KIND`) so 5d
   gets the real ToolRun panel chrome (activate toggle, pause/resume/step/abort/finalize/dismiss) instead
   of routing through the utility-rail fallback branch in `build_tool_run_job` (§3). Medium priority —
   fill *content* already works via the utility path, this is about giving it proper framework chrome.
9. **Settings panel** — create `📌️panels/⚙️settings/🦀️.rs` porting 3d's four-stepper shape (§5) once the
   backing actions (`setProximityRadius`/`setChunkSize`/`setGridSpacing`/an overlap-budget equivalent)
   are decided and added — none of these action ids exist in 5d today, so this is new work, not just
   reclassification.
10. **Volume Brush utility** — 5d has no equivalent of 3d's `🪛️utilities/🧊️volume-brush` at all; lowest
    priority since it's 3d-viewport-specific and 5d's 3d window may not need voxel target-volume painting
    the same way — confirm scope with the product owner before porting wholesale.
11. **Verify context-menu action-id polarity** (§6) — `focusSelection` vs `zoomToSelection` have the
    *opposite* live/dead status in 5d vs 3d; a direct read of `puzzle5d_context_menu_items`'s row strings
    (`🦀️.rs:3436-3445`) against §1's Migrated set is needed before assuming the menu is safe, since a
    verbatim port from 3d would put the wrong (dead) id on a "Zoom"/"Focus" row.
12. **`engagementRepeatLast`** — no equivalent id found in 5d's registry at all (checklist §14 table shows
    5 engagement sub-verbs for 3d; 5d has only 4 distinct Migrated engagement ids). Confirm whether this is
    an intentional product difference or an overlooked port.

## Files read this pass (for follow-up)

- `…/🖐️5d/…/✏️editor/🦀️.rs` (root, 8,226 lines) — action registry 8131-8201, retained-factory block
  7580-7669, context menu 3436-3445/7990-8006, tool ids 3863/3881.
- `…/🖐️5d/…/✏️editor/📌️panels/{🔍️inspection,🗿️artifact,🛍️catalogue}/🦀️.rs` (full reads of inspection).
- `…/🖐️5d/…/✏️editor/🎭️modes/✏️edit/☑️options/{🖌️brush,🪣️fill}/🦀️.rs`.
- `…/🖐️5d/…/✏️editor/🧠️precompute/{📐️geometry,🖌️brush,🪣️fill}/🦀️.rs` (line counts only).
- `…/🧊️3d/…/✏️editor/🦀️.rs` (action registry, retained-factory blocks, `🛠️tools/🪣️fill/🦀️.rs` head) for
  comparison.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-user-feature-checklist.md`
  (full read, source of the 25-section table and all "checklist §N" citations above).

Not read this pass (flagged, not claimed): 5d's `🌉️wasm/🦀️.rs`, `🧪️tests/*` bodies (only referenced for
the `setActiveUtility` test hit), `📚️examples/🎬️demo-session/🦀️.rs` content, `🟦️.ts` files, the
`🎬️actions/*` per-window TS/Rust action wrapper pairs beyond confirming their existence, and
`👥️presence/🦀️.rs`.
