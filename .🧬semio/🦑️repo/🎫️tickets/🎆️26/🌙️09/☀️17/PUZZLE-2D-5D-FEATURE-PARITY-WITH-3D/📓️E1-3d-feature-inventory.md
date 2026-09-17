# E1 — Puzzle 3D authoritative, source-measured feature inventory (as of 2026-09-17)

Read-only, source-measured. All file:line citations are against files as they exist in the tree today
(commit `a4cda597ea` area, unstaged tree). Root path abbreviated `EDITOR/` below for:

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

Main editor file `EDITOR/🦀️.rs` is **8711 lines** (not ~7593 as the 2026-09-09 checklist said — the crate
grew ~1100 lines since). Commands folder `EDITOR/🎮️commands/` has **53** one-verb subfolders (1717 lines
total). Precompute engine `EDITOR/⏳️precompute/🦀️.rs` is 1387 lines. Full sibling-folder census:
`🎭️modes/✏️edit` (options×6, tools×1 fill, windows/main×1 + 4 utilities), `📌️panels` (artifact,
catalogue, inspection, settings), `🪟️window` (348+2 lines), `🎚️config` (372+25), `🗣️terminology` (260),
`📚️examples` (22), `👥️presence` (241), `🌉️wasm` (141, wasm-bindgen bridge, infra not a feature).

Dimension vocabulary used below per the umbrella ticket: **Node(2d)=Object(3d)=Part(5d)**,
**Handle=Vortex=Grip**, **Edge=Attraction=Fastener**. Note: `EDITOR/🗣️terminology/🦀️.rs` does **not**
itself contain this triple — its own axis is Native-vocabulary vs Reuse-vocabulary (×EN/DE), not
Node/Object/Part. The 2d=Object=Part convention is an umbrella-ticket-level mapping across the three
apps, not literally encoded in the 3D terminology file.

---

## Legend

- **Class** = `InteractiveJobClassification` (from `.action_interactive_job(id, ...)` calls,
  `EDITOR/🦀️.rs:8622-8684`, and the app-schema for `setActiveTool`/`setActiveUtility` proven separately
  at `🦀️.rs:7630-7641`). **Every single classified action in this crate is `Migrated`** — no
  `BatchOnlyPendingRewrite` or other variant appears anywhere in the file (grep-confirmed). `copy`/`cut`/
  `paste` are `ArtifactReservedJob`s and sit **outside** this classification registry entirely (no entry).
- **Lane** = publication lane, from `Puzzle3dScopeClass`/`puzzle3d_command_scope_class`
  (`🦀️.rs:2443-2503`) cross-checked against `PUBLICATION_CONTRACTS` (`🦀️.rs:7169-7233`): `Artifact`
  (document mutation), `Config` (shared cross-window), `WindowConfig` (per-window persisted),
  `WindowTransient` (per-activation scratch, dropped on utility switch), `Chrome`/`HostOnly` (view-only,
  no document/config write), `Viewport`, `Selection`.
- **Dim** = `3D` (inherently spatial-3D, no obvious 2D/5D analog as implemented) / `Neutral` (2D and 5D
  are expected to have an equivalent verb) / `Neutral*` (dimension-neutral **pattern**, but the concrete
  implementation is written in 3D-only terms — e.g. vec3/quaternion — so 2D/5D would need their own
  shape, not a literal reuse).

---

## A. Every action/verb id (retained tool ids, command ids, args, classification, lane)

62 `Puzzle3dCommand` enum variants (`🦀️.rs:2588-2655`, `TOOL_JOB_IDS` list `🦀️.rs:2658-2729`) + 2
framework-injected host verbs (`setActiveTool`, `setActiveUtility` — no `Puzzle3dCommand` variant,
proven separately `🦀️.rs:7630-7641`) + 3 reserved clipboard verbs (`copy`/`cut`/`paste`, outside the
Command enum) + framework-owned generic verbs the app never implements itself (select/marquee/select-all/
clear-selection/undo/redo/checkpoint/revert) = full user-reachable verb surface.

| # | Feature / verb | Action id(s) | Command source (args struct) | Work/handler struct | Class | Lane | Dim | Note |
|---|---|---|---|---|---|---|---|---|
| 1 | Open Add Object dialog | `openAddObjectDialog` | shell-only, `puzzle3d_shell_only_emit` `🦀️.rs:3621` | `BoundedFirstStepCommandWork` (fallback) | Migrated | HostOnly | Neutral | `Effect::OpenDialog{dialog_id:"addObject"}`; dialog itself drives `addObjectKind` on submit |
| 2 | Transform gumball begin/end | `transformBegin`, `transformEnd` | `puzzle3d_shell_only_emit` `🦀️.rs:3636` | `NoopPuzzleCommandWork` | Migrated | HostOnly | Neutral* | Deliberately empty brackets — drag session is host/canvas-local, one absolute delta commits on release |
| 3 | Translate selection | `translateSelection` | `EDITOR/🎮️commands/🚀️translate-selection/🦀️.rs:14` — `dx,dy,dz: f64` | `Puzzle3dScaleWork` `🦀️.rs:4514` (impl `4611`) | Migrated | Artifact | Neutral* | Gumball commit + programmatic API; also drives `worldRelocate`-adjacent drag |
| 4 | Rotate selection | `rotateSelection` | `🎮️commands/🔄️rotate-selection/🦀️.rs:14` — `ax,ay,az,angle: f64` (axis-angle) | `Puzzle3dScaleWork` | Migrated | Artifact | Neutral* | Axis-angle→quaternion is a 3D-specific rotation representation |
| 5 | Scale selection | `scaleSelection` | `🎮️commands/📏️scale-selection/🦀️.rs:12` — `sx,sy,sz: f64=1.0` | `Puzzle3dScaleWork` | Migrated | Artifact | Neutral* | Applies to objects AND target volumes |
| 6 | Set active example | `setActiveExample` | `🎮️commands/🛍️set-active-example/🦀️.rs:7` — `exampleId: str` | `Puzzle3dSetActiveExampleWork` `🦀️.rs:5662` (impl `5728`) | Migrated | Artifact+Config (Chrome scope) | Neutral | Deletes all entities, replays target fixture chunked 8/turn; also resets `runtime` locally (unverified whether that round-trips to WindowConfig, per 09-09 checklist §5) |
| 7 | Set active tool | `setActiveTool` | framework-injected, no `Puzzle3dCommand` variant | n/a (`host_configuration_mutation` returns `None` deliberately) | Migrated (proven separately, `🦀️.rs:7630-7641`) | HostOnly via `Effect::SetActiveTool` | Neutral | Own handler (`🎮️commands/🧰️set-active` per 09-09 doc) sets `runtime.active_tool_id`, clears suggestion/engagement/brush-candidate state |
| 8 | Set active utility | `setActiveUtility` | framework-injected, no `Puzzle3dCommand` variant | n/a | Migrated (separate proof) | HostOnly via `Effect::SetActiveUtility` | Neutral | Same mechanism as #7 for the utility bar (Transform/Brush/Volume Brush/Relocate) |
| 9 | Add object kind (catalog→instance) | `addObjectKind` | `🎮️commands/🌱️add-object-kind/🦀️.rs:34` — `objectKind: str, origin: [f64;3]` | `Puzzle3dAddObjectKindWork` `🦀️.rs:4282` (impl `4361`) | Migrated | Artifact (bounded_catalog) | Neutral | Materializes default catalog+object on an empty document (fixed since 09-09) |
| 10 | Delete selection | `deleteSelection` | `🎮️commands/🗑️delete-selection/🦀️.rs:14` | `BoundedFirstStepCommandWork` (fallback reducer) | Migrated | Artifact | Neutral | Objects/vortices/references; also cascades attraction deletes |
| 11 | Duplicate selection | `duplicateSelection` | `🎮️commands/👯️duplicate-selection/🦀️.rs:14` | `BoundedFirstStepCommandWork` | Migrated | Artifact | Neutral | Offset `+0.5` x/y clone, ids new, re-select fixed in W-S |
| 12 | Export fixture | `exportFixture` | `🎮️commands/📤️export-fixture/🦀️.rs:116` | `Puzzle3dWindowCommandWork` `🦀️.rs:3874` (special-cased Scene stage `3969-3980`) | Migrated | HostOnly (inline effect / segmented download / refusal by size) | Neutral | Downloads current document JSON |
| 13 | Import fixture | `importFixture` | `🎮️commands/📥️import-fixture/🦀️.rs:382` — `payload/chunk/chunkCount/name/json/fixture` | `Puzzle3dWindowCommandWork` | Migrated | Artifact (bounded_catalog) | Neutral | Chunked whole-doc import |
| 14 | Open import fixture (file picker) | `openImportFixture` | `🎮️commands/🗂️open-import-fixture/🦀️.rs:7` | shell-only emit + `Puzzle3dWindowCommandWork` | Migrated | HostOnly | Neutral | `Effect::RequestFileOpen`, accepts json |
| 15 | Select same kind | `selectSameKindSelection` | `🎮️commands/🧬️select-same-kind/🦀️.rs:13` | dispatcher-inline | Migrated | Selection | Neutral | Widens selection to every object of the clicked kind (was a no-op pre-09-09, fixed W-S) |
| 16 | Set camera | `setCamera` | `🎮️commands/📷️set-camera/🦀️.rs:6` — `camera: object` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Malformed payload is a silent no-op (not user-reachable via normal orbit) |
| 17 | Set projection / projection param | `setProjection`, `setProjectionParam` | `🎮️commands/📽️set-projection/🦀️.rs:10` (delegates to framework `world3d_projection_*`) | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Orthographic/1·2·3-point family + cardinal/free orientation |
| 18 | Set vortex show | `setVortexShow` | `🎮️commands/🌀️set-vortex-show/🦀️.rs:6` — `value: Always\|Selected` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Unrecognized value silently ignored |
| 19 | Set vortex direction | `setVortexDirection` | `🎮️commands/🧭️set-vortex-direction/🦀️.rs:6` — `value: Inwards\|Outwards` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | |
| 20 | Relocate target volume | `relocateTargetVolume` | `🎮️commands/🚚️relocate-target-volume/🦀️.rs:7` — `volumeId, after:{position,quaternion,scale}` | `Puzzle3dRelocateVolumeWork` `🦀️.rs:6398` (impl `6422`) | Migrated | Artifact | 3D | Gizmo-driven volume relocate |
| 21 | World relocate (drag-to-attract) | `worldRelocate` | `🎮️commands/🌍️world-relocate/🦀️.rs:16` — `objectId, position:[f64;3]` | `Puzzle3dWorldRelocateWork` `🦀️.rs:5197` (impl `5255`) | Migrated | Artifact | 3D | Auto-creates attractions to nearby compatible vortices within `proximity_radius`; **faults on Nakagin (180 objects)** per 09-09 §11, extent bound `objects×66+attractions` vs 4096-item cap, fix scoped but not confirmed landed |
| 22 | Toggle sun | `toggleSun` | `🎮️commands/☀️apply-sun/🦀️.rs:9` (delegates to framework sun helpers) | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | |
| 23 | Set sun azimuth/elevation/intensity | `setSunAzimuth`, `setSunElevation`, `setSunIntensity` | same file | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | 0-360°/0-90°/0-4 ranges |
| 24 | Set LOD automatic | `setLodAutomatic` | `🎮️commands/🤖️set-automatic/🦀️.rs:6` — `pressed: bool?` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | |
| 25 | Set LOD depth-variable | `setLodDepthVariable` | `🎮️commands/📉️set-depth-variable/🦀️.rs:6` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | |
| 26 | Set LOD manual | `setLodManual` | `🎮️commands/✋️set-manual/🦀️.rs:7` — `value: f64` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Slider 0-1000 |
| 27 | Set grid visible | `setGridVisible` | `🎮️commands/👁️set-visible/🦀️.rs:6` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | Neutral | |
| 28 | Set grid snap enabled | `setGridSnapEnabled` | `🎮️commands/🧲️set-snap-enabled/🦀️.rs:6` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | Neutral | |
| 29 | Set grid spacing | `setGridSpacing` | `🎮️commands/↔️set-spacing/🦀️.rs:6` — value via absolute-or-delta | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | Neutral | Also drives the Settings-panel `grid-spacing` stepper (same field, two UI locations) |
| 30 | Set proximity radius | `setProximityRadius` | `🎮️commands/📡️set-proximity-radius/🦀️.rs:6` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Governs `worldRelocate` auto-attract distance |
| 31 | Set chunk size | `setChunkSize` | `🎮️commands/🧩️set-chunk-size/🦀️.rs:6` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Viewport spatial/render chunking |
| 32 | Set selectable kind | `setSelectableKind` | `🎮️commands/☑️set-selectable-kind/🦀️.rs:6` — `kind: objects\|vortices\|attractions` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | Neutral | Filter which entity kinds are pickable |
| 33 | Set selection flag (hide/lock) | `setSelectionFlag` | `🎮️commands/🔖️set-selection-flag/🦀️.rs:9` — `flag, value, entity?, ids?` | `BoundedFirstStepCommandWork` | Migrated | Artifact | Neutral | Generic across object/reference/vortex |
| 34 | Patch inspector | `patchInspector` | `🎮️commands/🩹️patch-inspector/🦀️.rs:13` — `entity, field, ids?, value?, delta?` | `Puzzle3dPatchInspectorWork` `🦀️.rs:4758` (impl `5000`) | Migrated | Artifact | Neutral | Generic field patch across object/vortex/attraction/reference/target-volume; handles attraction reconnect specially |
| 35 | Focus selection (camera zoom-to) | `focusSelection` | `🎮️commands/🎯️focus-selection/🦀️.rs:5` | `Puzzle3dFocusSelectionWork` `🦀️.rs:6229` (impl `6276`) | Migrated | Viewport (WindowConfig camera) | Neutral* | Frame/centroid math is 3D but "focus camera on selection" is universal |
| 36 | Engagement input | `engagementInput` | `🎮️commands/⌨️engagement-input/🦀️.rs:6` — `value: str` | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient | Neutral | Staged command-line text buffer |
| 37 | Engagement submit | `engagementSubmit` | `🎮️commands/📨️engagement-submit/🦀️.rs:19` — `value` parses `"fill <n>"\|"brush"\|"zoom"` only | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient/Chrome | Neutral | Placeholder text advertises `clear`/`rectangle`/`lasso` which are dead (moved to framework `vortex` domain) — advertised-but-dead defect, confirmed still present |
| 38 | Engagement repeat last | `engagementRepeatLast` | `🎮️commands/🔂️engagement-repeat-last/🦀️.rs:9` | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient | Neutral | Only meaningful while fill utility active (+1 count); no-op otherwise |
| 39 | Engagement abort | `engagementAbort` | `🎮️commands/🛑️engagement-abort/🦀️.rs:15` | `Puzzle3dWindowCommandWork` | Migrated | View | Neutral | Keybinding `escape`; also aborts a live fill run instead of disarming |
| 40 | Create attraction | `createAttraction` | `🎮️commands/💞️create-attraction/🦀️.rs:7` — `attracting, attracted: str` | `Puzzle3dCreateAttractionWork` `🦀️.rs:5443` (impl `5508`) | Migrated | Artifact | 3D | Validates kind compatibility against `meta.kind_compatibility` |
| 41 | Delete attraction | `deleteAttraction` | `🎮️commands/💔️delete-attraction/🦀️.rs:6` — `id` | `BoundedFirstStepCommandWork` | Migrated | Artifact (bounded_catalog) | 3D | |
| 42 | Set transform gumball flag | `setTransformGumballFlag` | `🎮️commands/🕹️set-transform-gumball-flag/🦀️.rs:6` — `flag: move\|rotate` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | Neutral | Scale handle deliberately absent — object scale comes from catalog, not free drag |
| 43 | Set voxel dims | `setVoxelDims` | `🎮️commands/📐️set-voxel-dims/🦀️.rs:6` — `axis: w\|d\|h, value` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Range [1,64] per axis |
| 44 | Add target volume | `addTargetVolume` | `🎮️commands/➕️add-target-volume/🦀️.rs:11` — `origin:[f64;3]` | `Puzzle3dWindowCommandWork` (special-cased) | Migrated | Artifact | 3D | Alt+click paint, grid-snapped |
| 45 | Delete target volume | `deleteTargetVolume` | `🎮️commands/🪦️delete-target-volume/🦀️.rs:6` — `id` | `BoundedFirstStepCommandWork` | Migrated | Artifact (bounded_catalog) | 3D | |
| 46 | Set target volume flag | `setTargetVolumeFlag` | `🎮️commands/🚩️set-target-volume-flag/🦀️.rs:6` — `id, flag, value` | `BoundedFirstStepCommandWork` | Migrated | Artifact (bounded_catalog) | 3D | |
| 47 | Engagement control-select | `engagementControlSelect` | `🎮️commands/🎛️engagement-control-select/🦀️.rs:7` — id like `puzzle3d.brush.candidate.<n>` | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient | Neutral | |
| 48 | Add brush object | `addBrushObject` | `🎮️commands/🖌️add-brush-object/🦀️.rs:15` — `BrushPlacePayload` (external struct) | `Puzzle3dAddBrushObjectWork` `🦀️.rs:5975` (impl `6055`) | Migrated | Artifact | 3D | Only handles `Ok(Fixture(_))` outcome — real placement errors are a silent no-op, no toast |
| 49 | Set fill count | `setFillCount` | `🎮️commands/🧮️set-fill-count/🦀️.rs:10,17` — `count`, clamped `PUZZLE3D_FILL_COUNT_MAX=1000` | `Puzzle3dPrecomputeCommandWork` `🦀️.rs:6815` (impl `6890`) | Migrated | Config (shared, not per-window) | Neutral | Chunked/resumable background planner, different lane than other fill measures |
| 50 | Set brush placement contact tolerance | `setBrushPlacementContactTolerance` | `🎮️commands/🚧️set-brush-placement-contact-tolerance/🦀️.rs:8` | `Puzzle3dWindowCommandWork` | Migrated | WindowConfig | 3D | Range [0, 0.05] step 0.001 |
| 51 | Set object-kind weight | `setObjectKindWeight` | `🎮️commands/⚖️set-kind-weight/🦀️.rs:10` — `kindId, value(0..1)` | `Puzzle3dKindWeightWork` `🦀️.rs:4043` (impl `4104`) | Migrated | Config, FillOptions scope | Neutral | Normalizes remainder across sibling kinds |
| 52 | Set vortex-kind (joint) weight | `setVortexKindWeight` | same file — `kindId, value, objectKindId` | `Puzzle3dKindWeightWork` | Migrated | Config, FillOptions scope | Neutral* | "vortex" naming 3D-specific, joint-weight concept neutral |
| 53 | Cycle brush candidate (fwd/back) | `cycleBrushCandidate`, `cycleBrushCandidateBack` | `🎮️commands/🔁️cycle-candidate/🦀️.rs:9` — `delta` | `Puzzle3dPrecomputeCommandWork` | Migrated | WindowTransient | 3D | Keybindings `tab` / `shift+tab` |
| 54 | Open vortex suggestions | `openVortexSuggestions` | `🎮️commands/🔓️open-vortex-suggestions/🦀️.rs:15` — `fullId, x, y, windowId?` | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient (bounded_catalog) | 3D | Context-menu "suggest" row; was dropped-before-render pre-09-09, fixed W-D2 |
| 55 | Close vortex suggestions | `closeVortexSuggestions` | `🎮️commands/🔒️close-vortex-suggestions/🦀️.rs:7` | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient | 3D | |
| 56 | Hover suggestion | `hoverSuggestion` | `🎮️commands/🖱️hover-suggestion/🦀️.rs:6` — `index?` | `Puzzle3dWindowCommandWork` | Migrated | WindowTransient | 3D | Still broken: precompute session never builds brush candidates on this path (render-warm-up gap, unresolved per 09-09 §13) |
| 57 | Accept suggestion | `acceptSuggestion` | `🎮️commands/✅️accept-suggestion/🦀️.rs:18` — `index?, fullId?` | `Puzzle3dAcceptSuggestionWork` `🦀️.rs:6512` (impl `6578`) | Migrated | Artifact | 3D | Always dismisses popup on any outcome; real placement error is silently invisible |
| 58 | Target brush suggestions | `targetBrushSuggestions` | `🎮️commands/🎣️target-brush-suggestions/🦀️.rs:12` — `fullId?` | `Puzzle3dWindowCommandWork` | Migrated | HostOnly | 3D | |
| 59 | Register brush mesh | `registerBrushMesh` | `🎮️commands/📋️register-brush-mesh/🦀️.rs:26` — `url, digest?, page?, pageCount?, positionsB64?, indicesB64?` | `Puzzle3dPrecomputeCommandWork` | Migrated | HostOnly | 3D | Paged base64 upload (512-char pages); silently rejects oversized payloads (no error) |
| 60 | World pointer down | `worldPointerDown` | dispatcher no-op `🦀️.rs:3698` | `NoopPuzzleCommandWork` | Migrated | View | 3D | Marker only, no behavior |
| 61 | Copy | `copy` | `Puzzle3dClipboardJob` `🦀️.rs:7646` (`emit` `7666-7693`) | `Puzzle3dClipboardJob` (`ArtifactReservedJob`, not in Command enum) | *not classified* (reserved-job, outside registry) | `Effect::ClipboardWrite` | Neutral | **Added since the 09-09 checklist**, which asserted "no app-specific clipboard handler exists" (§21) — this is now real. Fragment via `puzzle3d_copy_fragment`/`_from` (`🦀️.rs:356-379`) |
| 62 | Cut | `cut` | same job | same | *not classified* | Artifact (delete) + ClipboardWrite | Neutral | `puzzle3d_cut_operations`/`_from` (`🦀️.rs:380-397`) |
| 63 | Paste | `paste` | same job | same | *not classified* | Artifact (insert) | Neutral | `puzzle3d_paste_operations`/`_on` (`🦀️.rs:398-424`), default `PastePlacement` |
| 64 | Select (click/tree/catalogue) | `interactionSelect` (framework `SELECT_ACTION_ID`-family, generic) | n/a — puzzle3d only declares the `vortex` interaction domain | framework-owned | framework | Selection | Neutral | Puzzle3d supplies granularities only (§ below); dispatcher never mentions it |
| 65 | Select all / Clear selection | `selectAll` / background-clear | framework `SELECT_ALL_ACTION_ID`/`CLEAR_SELECTION_ACTION_ID` | framework-owned | framework | Selection | Neutral | No puzzle3d override |
| 66 | Marquee / rectangle-lasso select | host/framework `vortex` domain args | n/a | framework-owned (`SelectionMarquee`) | framework | Selection | Neutral | Method/merge-mode UI moved out of puzzle3d's own window options; marquee overlay bug fixed 09-15 (parity note: fix commit explicitly says "matching **node-graph** marquee parity" — i.e. puzzle 2D already has this working) |
| 67 | Undo / Redo / Checkpoint / Check-in / Revert | `undo`,`redo`,`commitCheckpoint`,`revertToCommand` | framework-reserved routes (`run_framework_reserved_job`) | framework-owned | framework | n/a | Neutral | Every app gets this for free; history-panel wasm hang was the top-priority unverified risk as of 09-09 |

**Total distinct user-reachable actions: 64** app-declared (62 `Puzzle3dCommand` + 2 host-injected) + 3
reserved clipboard + ~6 framework-generic (select family + history family) = **~73** counted verbs.

---

## B. Tools (`ToolRunDefinition`-driven background planners)

| # | Tool | Tool id | Source | Dim | Notes |
|---|---|---|---|---|---|
| 1 | Fill | `fill` | `EDITOR/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:22` (`TOOL_ID`); `run_definition()` `:41-55`; job `Puzzle3dFillToolRunJob` `:169` (`ToolRunRetargetableJob` `:225`, `InteractiveJob` `:258`) | Neutral* | Whole-document weighted-placement generator with start/pause/resume/abort/finalize lifecycle; `mutating:true, rebase:Revalidate, reconfigure:Resume, trace:Instance3d`; concept (place N pieces toward a target count, weighted by kind, checked for collision) is dimension-neutral, collision-mesh geometry is 3D |
| 2 | Fill — count slider | `puzzle3d-fill-count` measure → `setFillCount` | `🪣️fill/🦀️.rs:62` | Neutral | Deliberately unbounded max (product decision) |
| 3 | Fill — cancel | `puzzle3d-play-fill-cancel` measure → `cancelFillBuild{job,operation,generation}` | `🪣️fill/🦀️.rs:53-69` (`abort_action`) | Neutral | Wired since some point after 09-08 design doc (was previously "no affordance at all"); no direct test/runtime evidence found it actually cancels |
| 4 | Fill — distribution weights | `puzzle3d-play-distribution` group | `🦀️.rs:2319-2369` (`puzzle3d_distribution_children`/`_group`) | Neutral | Shared with Brush; object-kind + per-joint vortex-kind sliders |
| 5 | Fill — background tick | `fillBuildTick` (automatic, 120ms host tick) | dispatcher `🦀️.rs:3644+` | Neutral | Perf-sensitive, not correctness; was over the 8ms/turn law pre-W-P3 |

---

## C. Utilities / modes (window-bound interactive gestures)

| # | Utility | Utility id | Source | Gesture | Settings exposed | Dim | Notes |
|---|---|---|---|---|---|---|---|
| 1 | Select | n/a (framework `vortex` domain, not a puzzle3d utility) | `EDITOR/🎭️modes/✏️edit/☑️options/🎯️select/🦀️.rs:15` (`measure`) | click/marquee | 3 toggles: objects/vortices/attractions selectable → `setSelectableKind` | Neutral | Not in the 4-utility list (`window_kind_utilities`) — it's a window-option group, not a gumball-style utility |
| 2 | Transform (gumball) | `transform` | `.../🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️.rs:11` (`UTILITY_ID`), `definition()` `:14`, `options()` `:21` | drag move/rotate handle, one commit on release | 2 toggles: move/rotate → `setTransformGumballFlag` | Neutral* | Scale handle deliberately excluded (catalog-driven scale); icon `transform-3d` |
| 3 | Brush | `brush` | `.../🪛️utilities/🖌️brush/🦀️.rs:18` (`UTILITY_ID`), `run_definition()` `:32-46` | hover a vortex → live read-only candidate search (`mutating:false, rebase:Restart`) → click to place | Slider: contact-tolerance [0,0.05] step 0.001 → `setBrushPlacementContactTolerance`; + shared distribution group | Neutral* | Candidate search machinery dimension-neutral pattern; vortex/contact-tolerance geometry is 3D |
| 4 | Volume Brush | `volumeBrush` | `.../🪛️utilities/🧊️volume-brush/🦀️.rs:11` (`UTILITY_ID`), `voxel_dim_measures()` `:19-35` | Alt+click paints a grid-snapped target volume constraining Fill | 3 sliders: width/depth/height [1,64] → `setVoxelDims{axis}` | 3D | No 2D/5D analog implied by current naming (would need a 2D "area" or 5D N-dim equivalent) |
| 5 | Relocate (world-relocate) | `worldRelocate` | `.../🪛️utilities/🚚️world-relocate/🦀️.rs:8` (`UTILITY_ID`), `definition()` `:11` | drag an object to a new world position; auto-attracts to nearby compatible vortices within `proximity_radius` | none locally (reads app-wide `runtime.proximity_radius` from Settings panel) | 3D | icon `relocate-3d` |

Utility→window binding: all four (transform/brush/volume-brush/world-relocate) bound to the single
`puzzle3d-main` window kind via `.window_kind_utilities(...)` (`🦀️.rs:8535-8540`).

---

## D. Window kinds and per-window measures

Only **one** window kind exists: `puzzle3d-main` (`WINDOW_KIND_ID`, `.../🪟️windows/🧊️main/🦀️.rs:32`),
opened as **two fixed instances** (Top orthographic + Perspective), no split/new-window/close UI by
design (09-09 checklist §1, confirmed still true — `create_puzzle3d_app` registers exactly one window
kind, `🦀️.rs:8433`).

Window-level measure groups assembled in `window_measures()` (`🧊️main/🦀️.rs:68-80`), each backed by its
own options file:

| Group | Measure ids | Source | Dim |
|---|---|---|---|
| Projection | `puzzle3d-measure-projection-*` | `☑️options/🎥️projection/🦀️.rs:16` (delegates to framework `world3d_projection_measures`) | 3D |
| Vortex show/direction | `puzzle3d-play-vortex-show`, `puzzle3d-play-vortex-direction` | `☑️options/🌀️vortex/🦀️.rs:11,25` | 3D |
| LOD | `puzzle3d-play-lod` group, `-lod-auto`, `-lod-depth-variable`, `-lod-value` (slider 0-1000) | `☑️options/🔭️lod/🦀️.rs:14` | 3D (concept could be neutral, values 3D-tuned) |
| Grid | `puzzle3d-play-grid` group, `-grid-visible`, `-grid-snap`, `-grid-spacing` (slider [0.5,50] step 0.5) | `☑️options/🌐️grid/🦀️.rs:9` | Neutral |
| Select (selectable kinds) | `puzzle3d-play-select` group, `-select-objects/-vortices/-attractions` | `☑️options/🎯️select/🦀️.rs:15` | Neutral |
| Sun | `puzzle3d-measure-sun` group (id_prefix `"puzzle3d"` not `"puzzle3d-play"`), enabled/azimuth/elevation/intensity | `☑️options/☀️sun/🦀️.rs:14` (framework `world3d_sun_measures`) | 3D |
| Transform utility options | 2 toggles (move/rotate) | `.../utilities/🔄️transform/🦀️.rs:21` | Neutral* |
| Brush utility options | contact-tolerance slider + distribution group | `.../utilities/🖌️brush/🦀️.rs:52` | Neutral*/3D |
| Volume-Brush utility options | 3 voxel sliders | `.../utilities/🧊️volume-brush/🦀️.rs:38` | 3D |

Per-window persisted state — `Puzzle3dWindowConfig` struct (`EDITOR/🪟️window/🦀️.rs:15-39`):
`lod_automatic, lod_depth_variable, grid_visible, lod_manual, grid_snap_enabled, grid_spacing,
selectable_kinds{objects,vortices,attractions}, proximity_radius, chunk_size, voxel_dims:[u32;3],
transform_move, transform_rotate, vortex_show, vortex_direction, selection_method, sun{…}, camera{…}`.
Per-activation transient scratch — `Puzzle3dWindowTransient` (`🪟️window/🦀️.rs:96-103`):
`suggestion_menu, engagement_input, brush_candidate_index, activation`.
Shared cross-window config — `Puzzle3dConfig` (`🎚️config/🦀️.rs:266-285`): `fill_count, contact_tolerance,
object_kind_weights, vortex_kind_weights, active_example_id` — **the cleanest dimension-neutral/3D split
point in the whole crate**: everything in shared `Puzzle3dConfig` is dimension-neutral by construction
(any N-d puzzle needs fill-count + tolerance + kind-weights + active-example), everything 3D-specific
(camera, voxel, sun, vortex-show/direction) lives only in the per-window struct.
Publication byte ceiling: `MAXIMUM_PUBLICATION_BYTES = 65_536` (`🪟️window/🦀️.rs:235`);
`Puzzle3dConfig` store ceiling `PUZZLE3D_CONFIG_STORE_MAXIMUM_BYTES = 32_768` (`🎚️config/🦀️.rs` area,
confirmed via `🦀️.rs:7238`).

Camera struct `Puzzle3dCamera` (`🎚️config/🦀️.rs:87-102`): `position, target, zoom(default 1.0), up?,
projection` — session-only, never a document field. `puzzle3d_camera_distance` default orbit 30 units
(`:105`).

Interaction/hover/selection definition (`puzzle3d_interaction_definition`, `🦀️.rs:8343-8366`): domain
`vortex` (label "Vortex"); 6 granularities — `object`(box), `vortex`(sparkles), `attraction`(link),
`target_volume`(box-select), `reference`(image), `kind`(layers); `HierarchyProvider::Topology`;
hover channel `PUZZLE3D_HOVER_CHANNEL`, transitive:false, broadcast:true; selection
`modes:[Multiple,Single], methods:[Pick,Rectangle,Lasso], merges:[Replace,Additive,Subtractive,
Invertive]`. This whole mechanism is dimension-neutral framework machinery; the 6-granularity set is
puzzle-domain-specific (2D/5D would declare their own analogous set, dropping 3D-only entities).

---

## E. Panels

### E1. Artifact / outliner panel (`📌️panels/🗿️artifact/🦀️.rs`, 192 lines)
Body key `puzzle.3d.play.artifact` (definition `:33-41`, tab id `FRAMEWORK_PANEL_TAB_ARTIFACT_ID`,
group `Workbench`). Root `puzzle3d-play-document`, 4 window-sectioned groups (`render()` `:177-185`):
`.objects` (default open, nested vortex rows, icon `box`), `.references` (closed, icon `globe`),
`.target-volumes` (closed, icon `cylinder`), `.attractions` (closed, icon `link`, label
`"{attracting} → {attracted}"`, **no** hide/lock row actions). Shared `.interaction_domain(...)` binding
for the whole tree (perf: one arg-arena instead of per-row). Inline row actions (`hide_lock_actions()`
`:103-118`): show/hide + lock/unlock toggles on object/reference/target-volume rows only, dispatching
`setSelectionFlag`. **Known bug (still present as of this pass, `flag_args` `:94-101`):** value is
hardcoded `true` regardless of current state — clicking "Show" on an already-hidden row re-sends
`hidden:true`; context menu and inspection panel do the correct negation. Capacity constants:
`DOCUMENT_OBJECT_SLOTS=2048`, `DOCUMENT_VORTEX_SLOTS=4096`. **Since 09-16** (ticket
`PUZZLE-3D-ARTIFACT-TREE-PAGING`): large documents (Nakagin-scale) render a bounded page
(`PANEL_RECONCILE_NODE_BUDGET≈11` interactive rows/4 sections) with a `+N` continuation row dispatching
`setPanelPage`; the continuation row's `ellipsis` icon isn't in the vendored icon catalog so it renders
as garbled text — cosmetic bug, paging itself works. **Since 09-15** (`PUZZLE3D-OBJECT-TREE-LABELS`):
rows now use `puzzle3d_object_display_label`/`puzzle3d_next_object_label` (label → catalog name → id,
with " 2", " 3"… disambiguation) instead of raw kind id — fill/brush/add-object/accept-suggestion/
duplicate all stamp labels at publish time. Dim: mechanism neutral, entity vocabulary 3D-flavored.

### E2. Catalogue panel (`📌️panels/🛍️catalogue/🦀️.rs`, 160 lines)
Body key `puzzle.3d.play.kinds` (tab `FRAMEWORK_PANEL_TAB_CATALOGUE_ID`, group `Workbench`). Drag MIME
`PUZZLE3D_CATALOGUE_DRAG_MIME` (`:25`). 4 sections (`render()` `:144-153`): `.objects` (default open,
draggable rows → `addObjectKind`, nested rim-vortex template rows), `.vortices`/`.cables`/`.attractions`
(closed, read-only, sourced from `meta.kindCatalogs`). Object rows draggable with `drag_data`
`{objectKind, meshUrl?}` when a mesh resolves. **Since 09-13** (`PUZZLE-3D-CATALOGUE-DROP-LIVE-PREVIEW`):
a live mesh-ghost now follows the cursor during drag (previously invisible — 4 separate root causes
fixed: pointermove vs dragover, `frameloop=demand` invalidate, wgpu ghost tracking, drag-payload
publish). Dim: mechanism neutral, content 3D.

### E3. Inspection panel (`📌️panels/🔍️inspection/🦀️.rs`, 252 lines)
Body key `puzzle.3d.play.inspector` (tab `FRAMEWORK_PANEL_TAB_INSPECTION_ID`, group `Details`).
Per-granularity field groups (rewritten in W-S, replacing the old static schema/domain/count summary):
`object_fields()` (`:102-115`, id/label/kind/origin/orientation/scale/mesh-url/vortices-count +
hidden/locked toggles), `vortex_fields()` (`:117-126`, full-id/object/kind/position/direction/radius,
read-only), `attraction_fields()` (`:128-140`, id/attracting/attracted/gap/shift/rise/rotation/turn/tilt,
read-only), `target_volume_fields()` (`:142-151`, id/origin/orientation/scale + hidden/locked),
`reference_fields()` (`:153-163`, id/source-url/media-kind/origin/width-world + hidden/locked). All
mutating rows dispatch `patchInspector`. Fallback `summary()` (`:178-184`) when nothing valid selected.
Dim: mechanism neutral, field shapes (vec3/quat) 3D.

### E4. Settings panel (`📌️panels/⚙️settings/🦀️.rs`, 69 lines)
Body key `puzzle.3d.play.settings` (id `puzzle3d.panel.settings`, group `Settings`). **Session-wide, not
per-window** (explicit doc note `:1-6`, despite publishing through the per-window `WindowConfig` lane per
09-09 §19). 4 `NumberStepper`s, all `uniform:true`, args carry `{windowId}`: `contact-tolerance`
(step 0.001 → `setBrushPlacementContactTolerance`), `proximity-radius` (step 0.1 → `setProximityRadius`),
`chunk-size` (step 1.0 → `setChunkSize`), `grid-spacing` (step 0.5 → `setGridSpacing`, same field the
per-window Grid option also controls). The old "default selection merge mode" dropdown was **removed**
(merge is now a per-gesture modifier arg). Dim: contact-tolerance/proximity-radius/chunk-size 3D,
grid-spacing neutral.

### E5. History panel (framework-generic, every app gets one)
Not puzzle3d-owned. Tab `framework.panel.history`. `framework.history.undo`/`redo`/`checkpoint`/
`checkin` buttons; per-edit rows `framework.history.entry.${seq}` with revert (↶) →
`revertToCommand{entrySeq}`. Routed via `run_framework_reserved_job` — **as of 09-09, the single highest
flagged risk**: every framework-reserved route hung the wasm actor (`resolve_ready` poll loop never
pumped the cooperative worker pool); a fix was written and queued for rebuild #11, **never
runtime-confirmed landed**. Not re-verified in this source-only pass (no cargo/server allowed). Dim:
n/a, framework-owned.

---

## F. Engagement bar grammar

Per-window HUD, assembled by `main::engagement()` (`.../🧊️main/🦀️.rs:923-949`), status text
`WindowEngagementStatus{id:"puzzle3d-world-status", text:"<n> objects · <n> attractions"}`.
Session-active predicate `engagement_session_active` (`:950-957`): true for `brush|fill|worldRelocate`.

| Verb | Dispatch | Parses (per `engagement_submit`, `🎮️commands/📨️engagement-submit/🦀️.rs:9-32`) | Notes |
|---|---|---|---|
| Input | `engagementInput` | staged text buffer update | |
| Submit | `engagementSubmit` | `"fill <n>"` (activates fill + `setFillCount`), `"brush"` (activates brush utility), `"zoom"` (→ `focusSelection`) | Anything else silent no-op |
| Repeat last | `engagementRepeatLast` | only meaningful while fill active (+1 count) | Not a generic "repeat whatever I last submitted" |
| Abort | `engagementAbort` | clears input/candidate-index, resets active utility, aborts a live fill run | Bound to `escape` keybinding |
| Control-select | `engagementControlSelect` | typed control token e.g. `puzzle3d.brush.candidate.<n>` | |

**Confirmed-dead advertised sub-verbs:** placeholder text at `.../🧊️main/🦀️.rs:537`(-ish, per 09-09 doc)
reads `"brush, fill <n>, zoom, clear, rectangle, lasso"` but `clear`/`rectangle`/`lasso` are **not**
parsed by `engagement_submit` — selection-method ownership moved to the framework `vortex` domain.
Typing any of the three and pressing Enter does nothing. This defect was present at 09-09 and is not
addressed by any of the 09-13→09-16 follow-on tickets read for this pass — **still live**.

---

## G. Context menu (right-click)

Built by `puzzle3d_context_menu_items` (`🦀️.rs:2973-3044`), keyed off `Puzzle3dContextSelection`
(`:2905-2971`, buckets by granularity: object/vortex/attraction/target_volume/reference). Rows bypass the
registry-validated `Menu::action`/`Menu::action_args` builders — `puzzle3d_context_menu_row` constructs
`ContextMenuItemSpec` directly (`:2888-2898`), so a typo'd action id is never caught at build time.

| Selection | Rows (id → action) | Dim |
|---|---|---|
| Object(s) | `duplicate`→`duplicateSelection`, `select-same-kind`→`selectSameKindSelection`, `zoom`→**`focusSelection`**, `hide-show`(group `hand`)→`setSelectionFlag{flag:hidden}`, `lock-unlock`(group `hand`)→`setSelectionFlag{flag:locked}`, `delete`→`deleteSelection`(destructive) | Neutral |
| Vortex (single) | `suggest`(only when exactly one vortex selected)→`openVortexSuggestions{fullId}`, `zoom`→`focusSelection`, `delete`→`deleteSelection`(destructive) | 3D |
| Attraction | `delete`→`deleteAttraction{id}`(destructive) | 3D |
| Target volume | `hide-show`/`lock-unlock`(group `targets`)→`setTargetVolumeFlag`, `delete`→`deleteTargetVolume{id}`(destructive) | 3D |
| Reference | `zoom`→`focusSelection`, `delete`→`deleteSelection`(destructive) | Neutral |

**Correction vs the 09-09 checklist:** the checklist (§15) reported the zoom rows dispatched a
non-existent `"zoomToSelection"` action id (a real defect, would fault at dispatch). **Current source
(this pass, `🦀️.rs:2984/3005/3040`) dispatches the correct `focusSelection` id** — this appears to have
been fixed since 09-09 (not called out explicitly in any of the 09-13→09-16 ticket docs read, but the
source is unambiguous: no `"zoomToSelection"` string exists anywhere in the file). **This resolves 09-09
checklist priority-item #7.**

---

## H. Import/export, clipboard, add-object dialog, suggestions popup, examples, locale

### H1. Import/export
- Actions `exportFixture`/`importFixture`/`openImportFixture` are real, declared, `Migrated` (§A #12-14).
  This **contradicts** the 09-09 checklist §24 ("no import/export action id or command file found
  anywhere… does not exist as a usable end-user feature") — by the time of this pass, dedicated command
  files exist (`🎮️commands/📤️export-fixture/🦀️.rs` 116 lines, `📥️import-fixture/🦀️.rs` 382 lines,
  `🗂️open-import-fixture/🦀️.rs`), so import/export has since been implemented as a real feature.
- **Plugin-level media bridges** (`✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:46-64`, comment): OS media-host bridges
  for `register_2d_export_handlers`/`register_dwg_import_handler`/`register_mesh_exporter`/
  `register_mesh_importer`/`register_mesh_dwg_{export,import}_handler`, registered via `.setup()`, keyed
  by legacy OS media kind strings (`"2d.puzzle"`/`"3d.puzzle"`/`"5d.puzzle"`) — a **separate** registry
  from the `.composers(...)` one, partially overlapping (composer also serves PDF/JSON/DXF/LAS/PLY/GLTF).
  Confirms mesh/DWG import-export machinery exists at the plugin level across all three dimensions.
- Wasm envelope loader (`EDITOR/🌉️wasm/🦀️.rs`, chunked page-by-page ingress/decode/seal/poll/cancel,
  `PUZZLE3D_ENVELOPE_MAXIMUM_PAGES/BYTES`) is infra for opening large documents, not a separate feature.

### H2. Clipboard (copy/cut/paste)
Real, implemented via `Puzzle3dClipboardJob` (`🦀️.rs:7646-7753`, `ArtifactReservedJob`), registered
`build_reserved_tool_job` gated on `"copy"|"cut"|"paste"` (`🦀️.rs:8000-8006`). Fragment
copy/cut/paste helpers at `🦀️.rs:356-424`. **This directly contradicts the 09-09 checklist §21**, which
asserted with high confidence that "no app-specific handler exists… no clipboard/copy/cut/paste
identifier appears anywhere in the editor crate" — **clipboard support has been added since 09-09**.
Cross-confirmed at the plugin-manifest level: `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs:99-104` requests the
`shell.clipboard` capability explicitly citing "puzzle5d's copy/cut interception" (worded for 5D but the
mechanism/capability is shared plugin-wide; puzzle3d's own `Puzzle3dClipboardJob` is proof it uses it
too). No `InteractiveJobClassification` entry exists for copy/cut/paste (outside that registry).

### H3. Add Object dialog
`openAddObjectDialog` → `Effect::OpenDialog{dialog_id:"addObject"}` (`🦀️.rs:8454`, dialog registration
`:8608-8621`), submits via the existing `addObjectKind` action. Kind options built by
`puzzle3d_object_kind_options()` (`🦀️.rs:8381-8398`, up to `PUZZLE3D_OBJECT_KIND_OPTIONS_MAX=64`), reads
object kinds from **both** `CONCRETE_FOREST_EXAMPLE_FIXTURE` and `NAKAGIN_EXAMPLE_FIXTURE`, deduped by
id — this is an **improvement over the 09-09 checklist's §23 finding** (which said the dialog hardcoded
a single static `"Object"` option, unlike the dynamic Catalogue panel) — current source enumerates from
both built-in examples, though it is still not reading the **live document's own** `meta.kind_catalogs`
the way the Catalogue panel does, so a document loaded via import with custom kinds may still only offer
the two built-in examples' kinds. **Since 09-13** (`PUZZLE-3D-WINDOW-ADD-OBJECT-BUTTON`): a duplicate
per-window "Add Object…" chip was removed from viewport chrome (each split pane was showing its own copy)
— the dialog/palette route is now the sole `openAddObjectDialog` entry point, catalogue drag is the other.

### H4. Suggestions popup (brush)
Actions `openVortexSuggestions`/`closeVortexSuggestions`/`hoverSuggestion`/`targetBrushSuggestions`/
`acceptSuggestion`/`cycleBrushCandidate(Back)` (§A #53-58). Popup state in
`Puzzle3dWindowTransient.suggestion_menu` (`🪟️window/🦀️.rs:96-103`). Page size
`PUZZLE3D_SUGGESTION_MENU_CANDIDATE_PAGE = 8` (`.../🧊️main/🦀️.rs:746`). Open/close/accept fixed since
09-09 (W-D2); hover-to-preview candidates still genuinely broken (precompute session doesn't build
candidates on the render/hover path) — **unresolved**, not addressed by any 09-13→09-16 ticket read.

### H5. Examples
Only **one** example folder exists on disk under `EDITOR/📚️examples/`: `🎬️demo-session`
(`ID="demo-session"`, icon `play`, content = a **replayable command script**
`include_str!(🖼️assets/🎮️.cmd.semio)`, not a raw fixture dump — `EDITOR/📚️examples/🎬️demo-session/🦀️.rs`).
The two examples referenced throughout the checklist and `create_puzzle3d_app`/`SetActiveExample` work
— `concrete-forest` and `nakagin-capsule-tower` — are **built-in fixtures compiled into the crate**
(`CONCRETE_FOREST_EXAMPLE_FIXTURE`, `NAKAGIN_EXAMPLE_FIXTURE`, referenced §H3), not files under this
`examples/` folder; `demo-session` is a third, separate, command-replay example whose switching mechanism
(picker/registry) was not located in the files read for this pass. `EDITOR/🗣️terminology/🦀️.rs:25` defines
an `example_concrete_forest` label but no matching directory exists under `📚️examples/` — likely refers
to the compiled-in fixture, not a folder.

### H6. Locale / terminology (EN/DE)
`app_labels!`-macro-checked `Puzzle3dLabels` struct (`EDITOR/🗣️terminology/🦀️.rs`, 260 lines), **4 cells
per term**: `native_en/native_de/reuse_en/reuse_de` — the axis is **Native vs Reuse** vocabulary, not
Node/Object/Part (that triple does not appear literally in this file; see header note). Key terms:
`objects`(Objects/Objekte ↔ Building components/Baukomponenten), `vortices`(Vortices/Vortex ↔ Connection
points/Connection point), `attractions`(Attractions/Anziehungen ↔ Connections/Connection),
`attracting`/`attracted`(↔ Host/Guest connection point). ~70 more UI/tool-vocabulary fields (fill/brush/
move/rotate/voxel dims/placement-status/import-export/show-hide-lock/selection-tools/LOD/grid/inspector
field labels/settings labels). Locale resolution admits only 4 tag combos (`en|en-US`×`de|de-DE` ×
`native|reuse`), fails closed otherwise, no default-language fallback (explicit policy). FillRun/
BrushSuggestionsRun vocabularies also localized (5 stages/5 counters/~23 reasons for fill; 4 stages/3
counters/6 reasons for brush). The 09-09 checklist's one locale test failure
(`document_and_kinds_trees_use_german_reuse_section_labels` expecting `"Baukomponenten"`) is consistent
with the "reuse" `objects` term found here — plausibly a real resolvable gap, not re-verified (no cargo
allowed this pass).

### H7. Presence (multiplayer camera + tool broadcast)
`EDITOR/👥️presence/🦀️.rs` (241 lines): `Puzzle3dPresence{camera_position:[f64;3], camera_target:[f64;3],
camera_zoom:f64, active_tool_id:Option<String>}` — shareable live subset broadcast to peers. Selection and
hover are **deliberately absent** here — already broadcast generically by the framework
(`protocol::PresencePeer.interaction`), so this file would be a second divergent authority if it
duplicated them (doc comment `:6-13`). One mutation variant (`Snapshot`, whole-presence replace,
`:89-95`); custom bounded two-step retirement (`Puzzle3dPresenceRetirement`, `:184-235`) releasing the
`active_tool_id` heap string first, then the inline root. Dim: Neutral* — camera_position/target being
`[f64;3]` is 3D-shaped, but "broadcast my viewport + active tool to collaborators" is a universal concept
2D/5D would need their own version of (2D camera would be a 2-vector + zoom, 5D whatever its viewport is).

---

## I. Keybindings (app-level, `create_puzzle3d_app`, `🦀️.rs:8441-8447`)

| Key | Action | Dim |
|---|---|---|
| `escape` | `engagementAbort` | Neutral |
| `delete` | `deleteSelection` | Neutral |
| `backspace` | `deleteSelection` | Neutral |
| `mod+d` | `duplicateSelection` | Neutral |
| `tab` | `cycleBrushCandidate` | 3D |
| `shift+tab` | `cycleBrushCandidateBack` | 3D |
| `f` | `focusSelection` | Neutral |

No other `.keybinding(...)` calls exist in the app-definition range (7753-8711 checked exhaustively).

---

## J. Reconciliation with the 2026-09-09 reference checklist

Source: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-user-feature-checklist.md`
(449 lines, 25 numbered sections + priority summary). Full section list, for cross-reference: §1 Windows
(Top/Perspective, no split), §2 Camera orbit/pan/zoom, §3 Projection options, §4 Window options
(grid/LOD/vortex/sun/select), §5 Example switcher, §6 Selection (click/tree/marquee/select-all/clear),
§7 Hover, §8 Gumball/transform, §9 Brush utility, §10 Volume Brush utility, §11 Relocate utility,
§12 Fill tool, §13 Vortex suggestions, §14 Engagement bar, §15 Context menu, §16 Inspection panel,
§17 Artifact/outliner panel, §18 Catalogue panel, §19 Settings panel, §20 History panel, §21 Copy/cut/
paste, §22 Delete/duplicate/focus, §23 Add Object dialog, §24 Import/export, §25 Locale/terminology.

**Confirmed fixed/changed since 09-09 (found by direct source reading this pass, not by re-running any
test):**
1. **Clipboard (§21)** — checklist said no handler exists; `Puzzle3dClipboardJob` now real (§H2 above).
2. **Import/export (§24)** — checklist said it doesn't exist as a usable feature; dedicated
   export/import/open-import-fixture command files now exist (§H1 above).
3. **Context menu "Zoom to Selection" (§15, priority item #7)** — checklist found it dispatched an
   unregistered `"zoomToSelection"` id; current source dispatches the correct `focusSelection` (§G above).
4. **Add Object dialog kind options (§23, priority item #9)** — checklist said hardcoded to one static
   "Object" option; now enumerates kinds from both built-in example fixtures (§H3 above), though still
   not reading a live imported document's own catalog.

**Not yet independently re-confirmed by this pass** (no cargo/browser access; these were the 09-09
checklist's own top priority items and nothing in the 09-13→09-16 tickets read directly addresses them):
History panel wasm hang risk (§20), WindowConfig-lane hang on any camera/grid/LOD/sun touch (§2-§4, said
fixed in W-D2 but never runtime-confirmed), fill-tool activation dead-reducer fix (§12), Nakagin ledger
fix (§5), `worldRelocate` extent-cap fault above 62 objects on Nakagin (§11), hover-to-preview suggestion
candidates still empty (§13), silent engine-failure swallowing in `addBrushObject`/`acceptSuggestion`
(§9/§13), outliner hide/lock toggle value-always-true bug (§17, still present per E1 above), engagement
bar advertising dead `clear/rectangle/lasso` sub-verbs (§14, still present per §F above).

**Genuinely new user-facing behavior added 09-13→09-16** (from ticket diagnosis/fix docs under
`.../🎆️26/🌙️09/☀️1[3-6]/PUZZLE*`, all found and read this pass — none are literal `ticket.json`+`status.md`
pairs, each folder has a single diagnosis/fix doc):
- **09-13 `PUZZLE-3D-CATALOGUE-DROP-LIVE-PREVIEW`**: live mesh ghost now follows cursor during catalogue
  drag (was invisible — 4 root causes fixed, wgpu + React both). See §E2.
- **09-13 `PUZZLE-3D-FILL-ORBIT-CAMERA-RESET`**: fixed camera auto-jumping during fill/orbit (viewport
  ownership + provisional-instance bounds exclusion) — a bug fix restoring intended camera-stability
  behavior, not a new feature per se.
- **09-13 `PUZZLE-3D-TRANSFORM-GUMBALL-PREVIEW`**: gumball now shows a live drag preview in sibling
  World3d panes; fixed rotate handles being silently dispatched as `translateSelection`.
- **09-13 `PUZZLE-3D-WINDOW-ADD-OBJECT-BUTTON`**: removed a duplicated per-window Add-Object chip; add-
  object is now only reachable via Catalogue drag or the dialog/palette. See §H3.
- **09-14 `PUZZLE-3D-HIDE-FIT-LANE-FRAME-OVERLAY`**: suppressed the generic host "Frame Visible" reframe
  button when puzzle3d's own document-driven auto-fit lane (`fitJson`) is active — avoids two competing
  reframe affordances.
- **09-15 `PUZZLE-3D-CAMERA-NO-IMPLICIT-JUMP`**: fixed camera implicitly jumping on catalogue placement/
  fill/edits without an explicit camera action (auto-fit key no longer includes the mesh roster).
- **09-15 `PUZZLE3D-MARQUEE-RECT-OVERLAY`**: rectangle/lasso marquee drag rubber-band is now actually
  painted (selection itself always worked; only the visual overlay was missing) — fix comment explicitly
  says this now matches "**node-graph** marquee parity", i.e. puzzle 2D already had this working, useful
  cross-reference for the parity audit.
- **09-15 `PUZZLE3D-OBJECT-TREE-LABELS`**: outliner rows now show authored/catalog display labels instead
  of raw kind ids, with numbered disambiguation for repeated instances. See §E1.
- **09-15 `PUZZLE3D-REFERENCE-3D-VISIBILITY`**: fixed reference (image plane) rows existing in the
  document tree but not rendering in the wgpu 3D viewport (lane-copy gap in `sync_world3d_state`).
- **09-16 `PUZZLE-3D-ARTIFACT-TREE-PAGING`**: outliner is now virtualized/paged for large documents
  (`setPanelPage` action, `+N` continuation row) — cosmetic bug in the continuation row's icon id
  (`ellipsis` not in the vendored icon catalog) noted but paging itself works. See §E1.

None of these 10 tickets are pure net-new user-facing verbs beyond what §A-§I already enumerate — they
are fixes/polish to existing features (camera stability, drag previews, labels, paging, visibility) except
`setPanelPage` (outliner pagination action, added to the surface) which is worth adding to a future
parity pass if 2D/5D trees will also need paging at similar document scale.

---

## K. Summary tally

- **62** app-declared `Puzzle3dCommand` action ids + **2** host-injected (`setActiveTool`/
  `setActiveUtility`) + **3** reserved clipboard verbs (`copy`/`cut`/`paste`) + **~6** framework-generic
  verbs puzzle3d relies on without overriding (select/selectAll/clearSelection/undo/redo/checkpoint) =
  **~73** total user-reachable verbs.
- **1** tool (`fill`, `ToolRunDefinition`-driven).
- **4** window-bound utilities (`transform`, `brush`, `volumeBrush`, `worldRelocate`) + 1 window-option
  group (`select`) that behaves like a 5th but isn't in the utilities list.
- **1** window kind (`puzzle3d-main`), 2 fixed instances, no split/close.
- **4** panels app-owned (artifact/outliner, catalogue, inspection, settings) + 1 framework-generic
  (history).
- **5** engagement-bar verbs, 1 of which (submit) has 3 dead advertised sub-verbs.
- **5** context-menu row types across 5 selection-kind branches.
- Rough dimension split across the ~73 verbs + tool/utility surface: roughly **40-45% inherently 3D**
  (sun, projection, voxel/volume, vortex-show/direction, world-relocate, brush-mesh, LOD) and
  **55-60% dimension-neutral in concept** (though many neutral-concept verbs are implemented in 3D-only
  shapes — vec3/quaternion — so 2D/5D parity will need re-typed equivalents, not literal reuse; marked
  `Neutral*` throughout).

Full per-verb citations are in §A-§I above; this file is the E1 deliverable for
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/PUZZLE-2D-5D-FEATURE-PARITY-WITH-3D/`.
