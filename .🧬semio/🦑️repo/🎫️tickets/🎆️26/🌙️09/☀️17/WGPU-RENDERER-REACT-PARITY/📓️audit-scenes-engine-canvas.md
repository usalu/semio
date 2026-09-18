# wgpu ↔ React parity audit — Scene surfaces / Engine canvas hosts

Lane: SCENE SURFACES / ENGINE CANVAS (one of several Wave 0 parallel audits under this ticket; shell/window
system, interpreter/element coverage, runtime/boot/input and build/serve/theme health are separate lanes —
not duplicated here). Read-only audit, no code changed.

## 0. Method

Enumerated the 15 embeddable `SurfaceKind` variants from the canonical contract
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:83-136`): `Canvas2d`, `World3d`, `NodeGraph`,
`TextEditor`, `Table`, `Paint2d`, `VirtualFileSystem`, `TiledMap`, `Board2d`, `IconRender`, `InkCanvas`,
`GraphTimeline`, `BlockList`, `DiffView`, `EventFeed`. For each, read the React host under
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/` and diffed it against the wgpu
implementation, which lives almost entirely in three shared dispatch files rather than per-kind wgpu
targets:

- `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (4436 lines) — scene admission (`AdmittedSurfaceMap`),
  pointer/wheel routing, camera-dispatch debounce, and direct paint logic for Canvas2d/Paint2d/Table/
  VirtualFileSystem/TiledMap/Board2d/IconRender/InkCanvas/GraphTimeline/BlockList/DiffView/EventFeed.
- `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (4864 lines) — embeds three real sub-engines via
  vello offscreen compositing: `GraphHost` (`framework_surface_node_graph`) for NodeGraph, `MapHost`
  (`framework_surface_tiled_map`) for TiledMap, `EditorHost` (`framework_editor`) for TextEditor, plus
  World3d and Board2d gesture/interaction handling (line 1: doc header "Embeds GraphHost, FlowHost, and
  EditorHost via vello offscreen compositing").
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (14939
  lines) — the os-level native renderer: turn loop, job-progress presentation bridge, effect handling.

Only **IconRender** has its own dedicated wgpu target directory
(`🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs`); every other kind's wgpu behaviour lives in the
three shared files above. Per-window view state (active utility, focused window config) is owned by a
*fourth* wgpu element, `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (16346 lines) — not Scenes/EngineCanvas
— confirmed in §2.4 below.

**Structural note, not a gap:** unlike wgpu, React has no separate "Scenes" or "EngineCanvas" React
component — `🧱️elements/🎞️Scenes/` and `🧱️elements/⚙️EngineCanvas/` contain *only* `🎯️targets/🧊️wgpu` and
`🧪️tests`, no `.tsx`. React mounts each host (`World3dHost`, `Canvas2dHost`, …) directly through ordinary
component composition; the "Scenes" abstraction (a kind-keyed imperative dispatch table) is a wgpu-only
concept needed because Rust has no JSX-style declarative tree. The exact call site that maps a window's
`surfaceKind` to a host component was not conclusively located within this audit's time budget (candidates
inspected: `🏛️ShellHost/🟦️.tsx` 11059 lines, `🛠️ShellHelpers/🟦️.tsx` 5335 lines, `📌️ChromePanels/🟦️.tsx` —
none contain a `switch`/`case` or lookup table keyed on all 15 `SurfaceKind` strings). This does not affect
the gap analysis below (which compares actual per-kind behaviour), but is flagged for whoever picks up
work packet WP-0 in case it hides a 16th kind or a kind React silently never mounts.

## 1. Cross-cutting findings (apply to every / many surface kinds)

### 1.0 P0 (HEADLINE) — Production paint dispatch only reaches 4 of 15 `SurfaceKind`s; the rest is real code stranded behind `#[cfg(test)]`

This is the single biggest finding of the whole audit, discovered independently by all four per-surface
sub-passes. `render_component_scene_step`
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1223`) is the **only** production paint entry point for every
`ComponentScene` surface (its only non-test caller is `FrameworkSceneHost::paint_slot_step` in
`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1125`). Its phase-4 body:

```
if scene.component_kind == SurfaceKind::World3d { return render_world3d_surface_step(...); }
if !engine_canvas::sync_engine_scene(scene, hosts.window_id, bounds, ctx.theme) { return cursor.finish(); }
```

and `sync_engine_scene` (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2615-2621`):

```
match scene.component_kind {
    SurfaceKind::NodeGraph => sync_node_graph_scene(...),
    SurfaceKind::TiledMap  => sync_tiled_map_scene(...),
    SurfaceKind::Board2d   => sync_board2d_scene(...),
    _ => false,
}
```

So **World3d** (direct branch) and **NodeGraph / TiledMap / Board2d** (via `sync_engine_scene`) are the
only 4 of 15 kinds with a reachable production render path. For the other **11** —
`Canvas2d`, `TextEditor`, `Table`, `Paint2d`, `VirtualFileSystem`, `IconRender`, `InkCanvas`,
`GraphTimeline`, `BlockList`, `DiffView`, `EventFeed` — phase 4 falls straight to `cursor.finish()` after
phase 3 paints only the themed background panel (`ctx.draw.push_rounded(...)`, line ~1266). A window
mounted on any of these 11 kinds shows an empty rounded panel and nothing else, in every production wgpu
build.

The render/interaction code for most of these 11 kinds **already exists, correctly, field-for-field
matched against React** — `render_canvas_2d`, `render_table`, `render_table_cell`,
`paint_icon_render_chrome`, `render_block_list`, `render_diff_view`, `render_event_feed`,
`render_graph_timeline`, `vfs_double_click_action`, `text_editor_apply_key_into`, plus the matching wire
structs (`Canvas2dScene`, `TableScene`, `IconRenderScene`, `BlockListStepJson`/`BlockListBlockJson`,
`DiffViewScene`, `EventFeedEntryJson`, `HistoryColumnJson`, `VirtualFileSystemScene`) — but every one of
these functions is defined **only** inside `🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs` (2808 lines,
entirely `#[cfg(test)]`), reached from the production file only via a raw
`include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs")` at `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:932`, which is
stripped from every non-test build. The six per-kind test modules under `🎞️Scenes/🧪️tests/` (`wgpu-table`,
`wgpu-icon-render`, `wgpu-block-list`, `wgpu-diff-view`, `wgpu-event-feed`, `wgpu-graph-timeline`) call
these functions with `use super::*;` and pass today — the implementations are real and tested, just
unreachable outside `cfg(test)`. `render_icon_render` (referenced by a doc comment at
`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4068`) and `paint_text_editor` (referenced by a test's own comment at
`🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:379-383`, which admits "a real render pass this unit
test intentionally doesn't run") don't exist anywhere in the repo at all, test or production.

Canvas2d/Paint2d/InkCanvas show the identical symptom one file over (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`
itself, not just the test-standalone include): their fill/stroke/blend/grid/label code exists as either
orphaned doc comments with no function body (grid: line ~2058; labels: none found) or `#[cfg(test)]`-only
helpers (`Canvas2dFillBlend`/`Canvas2dShapes` regions, lines 2061-2183; real `render_canvas_2d` only in
`🧊️wgpu-standalone/🦀️.rs:1674`). InkCanvas's own `//#region InkCanvasRender` block
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3778-3789`) is **literally empty** despite ~1200 lines of real,
non-test interaction logic (marquee, stroke hit-test, camera math) sitting directly above it
(`InkCanvasState`, lines 2577-3776).

**Per-ticket-coordinator note (do not duplicate in progress elsewhere in this ticket):** work packets
already claimed/in-flight by other workers on this same ticket cover most of this headline finding:
**W1a** (the list-like kinds: GraphTimeline/DiffView/BlockList/EventFeed/Table), **W1b** (Canvas2d/Paint2d/
InkCanvas/TextEditor/IconRender), and **W1f** (World3d terrain-tile fetch, `setCamera` payload shape, map
interaction verbs, map context menu — see §2 World3d/TiledMap). Section 3 below lists only what remains
**beyond** those three packets' declared scope, plus items each already-claimed packet should pick up while
it's in the file (e.g. the exact field-name drifts below).

Wheel-scroll IS wired generically for the list-like kinds regardless of the above —
`passive_scene_wheel` (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1125-1136`) updates a per-surface
`scroll_offset("history"/"diff"/"feed"/"body"/"vfs")` on every wheel event — but nothing downstream ever
reads it for painting, so today it's tracked and discarded.

Two dead byte-delivery pipelines, named specifically because they're easy to miss (no per-kind test module
calls them, so they don't even show up as "test-only, at least tested"):
- `apply_ui_image_bytes(id: &str, url: &str, bytes: &[u8])` —
  `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1495` — production `pub fn`, but its only callers
  anywhere in the repo are two unit tests in
  `🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs:156,171` (`_decodes_fetched_png_…`,
  `_rasterizes_fetched_svg_…`). No production fetch-completion path calls it.
- `apply_map_tile_bytes(kind: WorldAssetRequestKind, bytes: &[u8])` —
  `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4192` — `pub fn`, zero callers anywhere in the repo
  (production or test) as of this audit.

Both are consistent with the World3d terrain-tile-fetch dead-pipeline finding in §2 (`pending_terrain_tile_
urls`) — this looks like a repeated pattern (a producer-side "apply the fetched bytes" function written and
never wired to whatever's supposed to drive the actual HTTP/asset fetch that would call it) rather than
three unrelated bugs.

### 1.1 P1 — Wire-schema drift: `VirtualFileSystem`'s wgpu-target `SurfaceKind` never got the kebab-case rename

The contract crate's own docstring (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:74-80`) says
in so many words:

> Ported from the wgpu target's `SurfaceKind`, with its one real wire inconsistency FIXED rather than
> preserved: `VirtualFileSystem` was `"virtualFileSystem"` (camelCase) where every sibling is kebab-case.
> ... **Rename: `"virtualFileSystem"` → `"virtual-file-system"`.**

That rename landed in the contract (`🗺️surface/🦀️.rs:105-106`, `#[serde(rename = "virtual-file-system")]`)
and in the scene-pack crate (`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1730-1731`, comment confirms the migration) and
in the *newer* generated TS (`🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts:434`: `"virtual-file-system"`).

**It never landed in the wgpu UI target's own `SurfaceKind` enum**, which is the exact type
`ui_wgpu::wgpu::SurfaceKind` that Scenes/EngineCanvas import and dispatch on throughout (confirmed:
`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:27` imports `SurfaceKind` from `ui_wgpu::wgpu`; every `SurfaceKind::Table`
/ `SurfaceKind::VirtualFileSystem` / etc. match arm seen in that file, e.g. lines 1131-1135, resolves to
this type):

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2908-2909` — still
  `#[serde(rename = "virtualFileSystem")]` / `#[value(rename = "virtualFileSystem")]`.
- Same file, line 2946 — `Self::VirtualFileSystem => "virtualFileSystem"` in the hand-written `as_str()`.
- A **second, independently-stale copy of the generated TS union** exists at
  `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:897`, still `"virtualFileSystem"` — sourced from a hardcoded
  TypeScript string literal baked into `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs:1167`, a
  completely different codegen path from the contract's own (correct) `🖱️ui/🧬️contract/🧬️schema/🦀️.rs:735`.

Net effect: **two conflicting generated `SurfaceKind` TypeScript unions ship in the tree** (one kebab-case
at `📜️ui-contract/🟦️.ts`, one stale camelCase at `🪪️manifest/🟦️.ts`), and the wgpu-native renderer's own
internal `SurfaceKind` type still serializes/round-trips `VirtualFileSystem` as `"virtualFileSystem"`.

The in-process contract→wgpu conversion (`🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:309-326`, `fn
surface_kind`) is a plain variant-to-variant `match` with no string round-trip, so admission of a
`VirtualFileSystem` surface *inside* the native process is not broken by this. The risk is at any boundary
where this specific wgpu `SurfaceKind` gets serialized to JSON/text and compared against a kebab-case
string produced elsewhere (debug/telemetry output, a native↔browser bridge message, a snapshot test
fixture, or any TS code that imports the stale `🪪️manifest/🟦️.ts` union instead of `📜️ui-contract/🟦️.ts`).
This audit's budget did not extend to tracing every such consumer; flagged as WP-1 below with a concrete,
cheap acceptance check.

Status: **P1** (confirmed source-level drift against the crate's own documented intent; blast radius not
fully traced, but zero-cost to fix and verify).

### 1.2 OK — Per-window view state (active utility) IS mirrored, just outside Scenes/EngineCanvas

Ticket asked to check whether wgpu mirrors React's per-window `activeUtilityByWindowId` /
`injectActiveUtility` overlay (`🏛️ShellHost/🟦️.tsx:4427-4433`: overlays `activeUtilityByWindowIdRef` keyed
by the **active** window id onto the outgoing `ViewModel` at every plugin call). wgpu does mirror this, but
the state lives in the `🐚️Shell` wgpu element, not Scenes/EngineCanvas:

- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2684` — `pub active_utility_by_window: HashMap<String, String>`.
- Line 4382 — `view_state.active_utility_by_window_id = self.active_utility_by_window.clone();` (built onto
  every outgoing view state, same shape as React's `buildActiveUtilityByWindowId`).
- Lines 4980, 10969-10983 — `Effect::SetActiveUtility { window_id, utility_id }` handled by
  `apply_set_active_utility`/`active_utility_for_window`, keyed exactly like React's map.
- Scene surfaces themselves read it back: `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2421-2424` (Board2d reads
  `board.active_utility`, calls `host.set_active_utility(&active_utility)` on change) and
  `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3048,3245,3737` (VirtualFileSystem-adjacent document state carries its
  own `active_utility`, default `"selectDirect"`).

Status: **OK** — parity confirmed by citation on both sides. "Focused window" itself is also mirrored:
React's `focusedWindowId` (`🏛️ShellHost/🟦️.tsx:4642,4884,6721`, sourced from `activeWindowIdRef`) has a
direct wgpu counterpart at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4385`
(`view_state.focused_window_id = self.active_window_id.clone()`) and is consumed for keybinding targeting
at line 10828-10830 (`resolve_keybinding_target_window_v1`), the same role React's `focusedWindowId` plays
at `🏛️ShellHost/🟦️.tsx:8618` (`resolveKeybindingTargetWindowV1`). Both concepts are Shell-element-owned,
not Scenes/EngineCanvas-owned, on both sides — consistent placement, no gap.

### 1.3 P2 — Spawned job progress: global plumbing exists both sides, per-surface consumption unverified

React: `🏛️ShellHost/🟦️.tsx:5946-5948` subscribes `plugin.subscribeSpawnedJobProgress?.(target.instanceId,
…)` at session scope and forces a `{ kind: "full" }` host-effects refresh on any callback — i.e. a global,
coarse "redraw everything" reaction, with the docstring at line ~5936 explicitly noting "an Isolated
spawned job … publishes no typed-operation scope; its host driver reports coalesced progress instead, and
every window adopting the job's retained output re-renders through the same lane."

wgpu: a comparable turn-level primitive exists — `JobProgressPresentationBridge` /
`JobProgressPresentationLease` in the os renderer
(`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3224-3390`, consumed at 5155, 5794-6888, and attached to render/turn
structs at 11114-12346 as `job_progress: Option<kernel_runtime::JobProgressPresentationLease>`).

Both sides have real infrastructure; this audit did not trace whether a specific scene surface (e.g. a
World3d mesh/layer admitted via a page lease) is actually wired as a *consumer* of this bridge the way
React's session-level subscription forces a full refresh — see the World3d-specific gap table (§2) for the
narrower, previously-flagged gap ("scene page leases have no React consumer" — that gap is on the *React*
side, not wgpu; wgpu's presentation-bridge plumbing looks more complete here, but is unverified at the
per-surface level for lack of budget). Status: **P2 / unverified**, not P0 — do not treat as broken without
a runtime repro.

### 1.4 OK — Wire schema constants: 15/15 present in the shared native scene-pack crate

`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` defines one `SCHEMA: &'static str` const per kind,
covering all 15 (`"canvas-2d@1"` L206, `"world-3d@1"` L482, `"node-graph@1"` L1325, `"text-editor@1"`
L1466, `"table@1"` L1574, `"paint-2d@1"` L1636, `"icon-render@1"` L1688, `"virtual-file-system@1"` L1733,
`"tiled-map@1"` L1795, `"board-2d@1"` L2005, `"ink-canvas@1"` L2265, `"graph-timeline@1"` L2318,
`"diff-view@1"` L2352, `"event-feed@1"` L2398, `"block-list@1"` L2444) — all kebab-case, all `@1`, no drift
at this layer. This crate has no `⚛️react`/wasm package target (only `📦️packages/🦀️rust`), so it is
native-only; it is NOT what a plugin author encodes a document with on the React/wasm side — treat as the
wgpu-native reference shape only. Per-kind document-field-name drift (e.g. `ink.document` vs
`note.document`, `operation` vs `mutation`) is checked per-surface in §2, not here.

### 1.5 OK — Camera-dispatch debounce is real, but Canvas2d/Paint2d-scoped only

`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:839-935` (`schedule_scene_camera_dispatch`, `SceneCameraDispatchCursor`,
`scene_camera_action`) debounces a settled `setCamera` action ~350ms after the last pointer/wheel edit and
fires `ActionDescriptor{action:"setCamera", args:{surfaceId, camera:{x,y,zoom}}}` — but every docstring on
this path (lines 361, 743, 835, 917) explicitly scopes it to "Canvas2d/Paint2d". World3d's camera
publication path is separate (`render_world3d_surface_step` at line 1341 onward) — see §2 World3d gap
table for whether *its* `setCamera` publication matches React's orbit/pan/zoom/fit semantics; not
re-verified here to avoid duplicating that lane's work.

### 1.6 Mounting / admission model

wgpu admits/evicts scene surfaces through a fixed-capacity `AdmittedSurfaceMap<T>`
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:29-30,69-274`): `SCENE_SURFACE_CAPACITY = 256`,
`SCENE_SURFACE_ID_BYTE_CAPACITY = 256`, slot-based with epoch tokens, explicit `AdmittedSurfaceFault`
(`IdCapacity`/`ItemCapacity`/`ReplacementPending`/`RejectedPending`/`Closing`) and a `begin_close`/
`close_step` retirement state machine — same fixed-capacity-arena discipline as the rest of this codebase's
wgpu UI layer (see memory: mounted-session 4 KiB caps, UiFixedList page-exact capacity). React has no
capacity ceiling on how many scene hosts can mount (ordinary React component lifecycle) — this is a
structural difference worth knowing about, not itself a bug: **at 256 concurrently-admitted scene surfaces
wgpu will start rejecting admission where React would not.** Whether any product actually approaches 256
concurrent scene surfaces in one session is unknown; flagged as informational.

## 2. Per-surface gap tables

Produced by four parallel read-only sub-audits, one per cluster of surface kinds. Each sub-audit read its
React host file(s) in full and grepped/targeted-read the wgpu side. Status legend: **P0** broken/absent in
production, **P1** missing, **P2** present but diverges, **OK** parity confirmed, **N/A** feature absent on
both sides (not a gap).

### 2.1 World3d + WorldTerrainLayer

**Architecture note:** most of wgpu's real World3d behaviour does **not** live in the Scenes/EngineCanvas
files named in this ticket's file list — it lives in
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` (12,599 lines, not originally listed).
`render_world3d_surface_step` (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1341-1354`) just admits a `World3dState`
and hands the whole scene to `infinite_world::world::render_world_3d(...)`; `⚙️EngineCanvas/🎯️targets/
🧊️wgpu/🦀️.rs:2611-2612` explicitly documents the exclusion ("its host is `World3dState`, which lives in
the shell's own `world3d_states`… and paints a real 3-D pass"). Camera/orbit/fit, selection/hover/marquee,
the transform gumball, LOD/grid, terrain chunking and context menus for World3d all live in that one file.
Terrain itself is a third layer: `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs` (459 lines, DEM decode +
mesh build) is a shared core called directly by wgpu and wrapped by `wasm_bindgen` for React — genuine
shared-code parity wherever it's actually reached.

| # | Category | Status | Finding |
|---|---|---|---|
| 1 | Mounting/lease | OK | React: `useLayoutEffect` rebuilds per-lane via `useMemo` off `node.world3d.*Json`, reassembled by `PagedSurfaceView` (`🌐️World3dHost/🟦️.tsx:5433-5439`). wgpu: `World3dState` is retained (not leased) via `hosts.world3d_states.get_or_insert_with(...)`, explicitly documented as deliberate REFRESH-not-lease (`⚙️EngineCanvas/…/🦀️.rs:1878-1884`, "evicting on absence is what made the node graph unreachable — ticket 26/09/09"). Same design both sides. |
| 2 | Wire schema | OK (cosmetic doc nit) | `SCHEMA = "world-3d@1"` (`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:482`), one shared `World3dScenePack` struct, 20-lane split (`World3dSceneLane`) escaping the 32 KiB spine cap, reassembled by React's `PagedSurfaceView`. Doc comments in `🖱️ui/🧬️contract/🗺️surface/🦀️.rs:10,26,135,155` illustrate the tag as `"world3d@1"` (no hyphen) — every real usage is `"world-3d@1"`; comment-only drift, zero functional impact. |
| 3 | Camera: orbit/pan/zoom | **P2** | React `setCamera` (`🌐️World3dHost/🟦️.tsx:764-780,5809-5819`): `{windowId, camera:{position, target, zoom, up?}}` — deliberately omits `fov`. wgpu `orbit_camera_action` (`♾️infinite/🌍️world/🦀️.rs:9134-9153`): `{surfaceId, camera:{position, target, fov}}` — **no `zoom` field**, sends `fov` which React never sends. `orbit.zoom(delta)` only mutates camera position locally (dolly-zoom), never becomes a wire `zoom` scalar. Since React's `zoom` is how orthographic-projection scale round-trips, this looks like a real gap for orthographic World3d views specifically. |
| 3b | Fit-to-scene | OK | React `world3dFrameDistanceForRadius` (`🟦️.tsx:931-936`), margin `1.12` (`🟦️.tsx:919`), `fit.enabled`/`revision`/`boundsMin`/`boundsMax` + `userMoved` latch. wgpu `sync_world3d_scene_fit` (`♾️infinite/🌍️world/🦀️.rs:9675-9700`) reads the identical shape and calls the **shared** `ui_wgpu::wgpu::frame_orbit_to_bounds` with the **shared** margin constant — same math, cross-referencing the same ticket/incident in both doc comments (`🦀️.rs:9668-9672`). True parity. |
| 4 | Selection/hover/marquee | OK (one wire item to confirm) | Full marquee in `♾️infinite/🌍️world/🦀️.rs`: `WorldMarqueeGesture`/`marquee_points: Vec<[f32;2]>` (arbitrary path, not just a rect, line 1293-1294), `WorldMarqueePickCursor`/`WorldMarqueePublishJob` (2736-2821), `marquee_is_crossing_from_path` matching React's crossing-vs-contains semantics. `interactionSelect`/`interactionHover` verb names and `domainId`/`method:"pick"`/granularity `"handle"` are pinned against React by `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:288-316` ("action payloads are pinned against React's World3dHost"). Not confirmed: a dedicated ≥3-point lasso path/test for World3d specifically (the data structures support it; TiledMap demonstrably exercises lasso, World3d wasn't found to). |
| 5 | Gizmos/handles | OK | React `SceneGumball`/`UnifiedGumball` (`🟦️.tsx:2709-2761`). wgpu: full `GumballHandle` enum (Move/Rotate/Scale × X/Y/Z/planes, `♾️infinite/🌍️world/🦀️.rs:7443-7460`), hit-test (`pick_gumball_handle_at:8833`), drag math (`gumball_drag_update:11437`), and exact action-id parity: `TRANSLATE_SELECTION_ACTION_ID`/`ROTATE_SELECTION_ACTION_ID`/`SCALE_SELECTION_ACTION_ID` (`🦀️.rs:4661-4663`), built in `gumball_commit_action` (9061-9124). |
| 6 | Grid/axes | OK | React `WorldLodBridge` (`🟦️.tsx:7366-7376`: `gridFactor`/`gridSnapEnabled`/`showLodGrid`/`gridDatum`). wgpu `WorldLodRecord` (`♾️infinite/🌍️world/🦀️.rs:1617`) + `append_lod_grid_lines`/`snap_world_point_to_grid` (6816,11957) — field-for-field match. |
| 6b | Terrain color | **P2 (documented, deliberate)** | React (`🗺️WorldTerrainLayer/🟦️.tsx:61-85`) samples a smooth 256px gradient continuously per-vertex. wgpu buckets into `TERRAIN_COLOR_BANDS = 10` flat bands (`♾️infinite/🌍️world/🦀️.rs:6906-6912`) because `Instance3d`/`World3dVertex` carry no per-vertex color channel — the code's own comment says so. Honestly-labeled engine limitation, not silent drift. |
| 6c | Terrain DEM tile fetch | **P0 — likely non-functional in production** | React `TerrainTileRenderer.uploadOne` (`🟗️WorldTerrainLayer/🟦️.tsx:165-187`) does `fetch(url)` then `session.upload_elevation_tile(...)`. wgpu's `sync_terrain_state` queues a URL into `state.pending_terrain_tile_urls: HashMap<String,(u32,u32,u32)>` when `terrain_tile_mesh_json` returns null (`♾️infinite/🌍️world/🦀️.rs:7386-7388`) — but this map is written and never read anywhere else in the repo (confirmed by grep excluding the file itself and its own test). The unit test that exercises this path (`♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:2574-2596`) manually calls `upload_elevation_tile` itself to fake what a real fetch driver would do. Contrast with the generic asset pipeline, which DOES have a full bounded fetch-queue (`WorldAssetFetchOwner`, `🦀️.rs:12108-12520`) — terrain tiles aren't wired into it. **Net effect: terrain elevation data can never actually load in a real wgpu run** unless a fetch consumer exists somewhere outside `infinite_world`/`renderer.rs` that this audit didn't independently verify. Single most actionable finding in this audit. (Per the ticket coordinator, this is in W1f's declared scope — confirm it's actually been closed, not just planned.) |
| 7 | LOD strategy | OK | Same field set as #6 (`automaticLod`/`manualLod`/`depthVariableLod`/`gridFactor`); `DEFAULT_LOD_GRID_FACTOR`/`DEFAULT_MANUAL_LOD` (React `🟦️.tsx:97-98`) mirrored by `default_grid_factor`/`default_manual_lod` (wgpu `🦀️.rs:411,419`). |
| 8 | Labels/captions in 3D space | N/A | Neither renderer does 3D-space captions for World3d (React has no `<Html`/`Billboard`/`Text3d` — only a 2D DOM `IconShotFrame` overlay elsewhere). Not a gap. |
| 9 | Context menu | OK | React `resolveWorldContextMenuTarget` (`🟦️.tsx:7272-7292`). wgpu `WorldContextMenuCursor` (`♾️infinite/🌍️world/🦀️.rs:4520-4597`), dispatches `"worldContextMenuAt"` (6657), `resolve_world_context_menu_target` (10852) doc-cites React's priority order by name ("hovered vortex wins — resolveWorldContextMenuTarget in world-3d-host.tsx", line 10865). |
| 9b | Keyboard shortcuts | **P1** | React only has Alt-tracking (suggestion gesture, `🟦️.tsx:6167-6180`) and Escape-cancels-relocate (`🟦️.tsx:6840-6848`) — no broader table. No equivalent found in `♾️infinite/🌍️world/🦀️.rs` (no `keybind`/`shortcut` hits). Low severity (React itself has little here) but the Escape-cancels-relocate/suggestion-menu affordance specifically wasn't found wgpu-side. |
| 9c | Catalogue drag/drop | **P1 (narrow)** | React has full catalogue-drag-to-place (`onCatalogueDragEnter/Leave/Over/Drop`, `🟦️.tsx:7299-7302`). Catalogue drag/drop was only found wired for NodeGraph in wgpu (`spawnApp` action, `⚙️EngineCanvas/…/🦀️.rs:2902-2919`) — no `SceneDragMode` variant found for World3d catalogue-drop. |
| 10 | Spawned job progress | P1 (generic) / OK (narrow) | Neither side has a generic job-progress subscription for World3d (framework-wide gap, not specific to this comparison). React's `world3dComputeStatusV1(scene.statusJson)` (`🟦️.tsx:5610-5617`, drives `computing`/`cancellable`/`cancelAction`) is mirrored by wgpu's `World3dPreparedStatus{progress:[f64;3], total, state}` (`♾️infinite/🌍️world/🦀️.rs:9285-9303`) with a computed ratio (10587) — arguably *more* complete than React's boolean flag. |
| 11 | Per-window view state | OK (different shape) | React threads `windowInstanceId` through nearly every dispatch (`🟦️.tsx:5457`+). wgpu instead gates interaction routing off `state.active_utility: String` (`♾️infinite/🌍️world/🦀️.rs:1307,1575`, used at 5620-5764,10558-10747) — functionally equivalent since both ultimately read the same `interaction_json.activeUtility` wire field. Not a gap, different implementation shape. |
| relocate drag | (coordinator-flagged, partially re-checked) | The `worldRelocate` action commit (`♾️infinite/🌍️world/🦀️.rs:10849-10860`, `state.drag_object_id.take()` → `ActionDescriptor{action:"worldRelocate", args:{surfaceId,objectId,position}}`) and its press/drag-start bookkeeping (`drag_object_id` set at line 10943) read as ordinary production code in this pass — no `#[cfg(test)]` gate was found immediately around them. The ticket coordinator flagged "relocate drag behind cfg(test)" as a remaining item; this audit could not independently confirm that specific claim at this location in the time available — re-check before treating it as closed or as still-open. |

**Test coverage:** no dedicated World3d test directory exists (matches expectation — real coverage lives in
`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`, 497 lines, explicitly pinned against React's
World3dHost). Not covered by any test found: World3d marquee/lasso specifically, `lod_json`/`chunking_json`
fields (never read by the Scenes/EngineCanvas dispatch files at all), the terrain fetch-queue drain
(because it doesn't exist).

### 2.2 TiledMap

Shares the same `MapHost` core (`🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs`, 4129 lines) natively
from wgpu and wrapped by `wasm_bindgen` for React — pan/zoom/fit/LOD/marquee/lasso genuinely cannot drift
between the two renderers wherever this shared core is what's actually invoked.

| # | Category | Status | Finding |
|---|---|---|---|
| 1 | Mounting/lease | OK | React: `MapWasmSession` + `ResizeObserver`, retries to 240 frames for a ≥64×64px container (`🧭️TiledMapHost/🟦️.tsx:842-949`). wgpu: `sync_tiled_map_scene`/`sync_map_engine` lazily creates a `MapHost` in `ENGINE_SURFACES`, diffs via `MapSyncCache` — same REFRESH-not-lease discipline as World3d. |
| 2 | Wire schema | OK | `SCHEMA = "tiled-map@1"` (`🎬️scenes/🦀️.rs:1795`), flat 13-field struct (small enough to skip lane-splitting), one shared Rust struct for both renderers. |
| 3 | Camera pan/zoom/fit | OK (one reachability question) | Shared `MapHost` core: `fit_world_camera`/`wheel_screen`/`pointer_down/move_screen`/`set_lod_mode` — identical code both sides. React dispatch `{camera:{x,y,zoom}}` on gesture-end/wheel/pan-end (`🟦️.tsx:764-769`) confirmed matched field-for-field by wgpu's `write_map_interaction_actions` (`⚙️EngineCanvas/…/🦀️.rs:4013-4032`, same `SET_CAMERA` shape) — the one camera dispatch in this whole audit with confirmed wire parity. Flag: `tiled_map_wheel_into` (`⚙️EngineCanvas/…/🦀️.rs:4084-4089`) has no caller anywhere in Scenes/EngineCanvas — its only caller is in the os renderer file (`…engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12066`, outside this lane's file list) — worth a direct look to confirm wheel-zoom is reachable at runtime. |
| 4 | Selection/hover/marquee/lasso | **P2 (action-name divergence)** | Full marquee+lasso confirmed (`write_tiled_map_selection`/`write_tiled_map_hover`, rectangle ≥2 pts / lasso ≥3 pts + `method=="lasso"`), delegating to the shared `MapHost.features_in_rect_json`/`features_in_polygon_json` (`🗺️tiled-map/🦀️.rs:2786,2819`, wrapped identically for React at lines 4033/4038). Threshold-gated marquee-vs-click matches (`MAP_MARQUEE_THRESHOLD_PX=6` both sides). Granularity resolution is explicitly cited as reproducing React's `resolveMapInteractionSync` (a wgpu test literally says so). **But**: wgpu's dispatch layer appears to use a bespoke action verb `SET_FEATURE_SELECTION`/`SET_HOVER`, while React's `TiledMapHost` dispatches the generic framework verbs `interactionSelect`/`interactionHover` (`🧭️TiledMapHost/🟦️.tsx:191-194`, `{domainId:"features", targets, merge, method}`). This is a real action-name mismatch candidate — needs a direct guest/plugin-side check to see which name the guest actually listens for; not fully resolved by this audit. |
| 5 | Gizmos/handles | N/A | No transform-gizmo concept for TiledMap on either side. |
| 6 | Grid/axes/terrain | N/A | TiledMap has no grid/terrain concept (terrain is World3d-only). |
| 7 | LOD/chunking | OK | React `renderer.setLodMode(scene.lodMode)` (`🟦️.tsx:859,971`), z/x/y chunking with byte-LRU caches, prefetch ring, concurrent-fetch cap 12. wgpu syncs `lod_mode` straight into the shared `MapHost::set_lod_mode` — same core. |
| 8 | Labels/captions | **P2 (UX-level, not data-level)** | React shows an interactive 2D DOM hover popup (feature name/icon/source-availability) via `session.featureScreenJson` polled per-frame (`🟦️.tsx:1003-1018,1241-1257`). wgpu only forwards `labelFill`/`labelHalo` theme colors into `MapHost::set_map_theme_from_json` (`⚙️EngineCanvas/…/🦀️.rs:2244-2255`) — native in-canvas labels can match color, but there's no equivalent of React's separate interactive DOM popup. Likely acceptable for a native host, but a real UX difference. |
| 9 | Context menu | **P2/possibly P1 — needs direct look** | React: full context menu via `openSurfaceContextMenu`, hits split by domain (`🟦️.tsx:1194-1218`). wgpu: an orphaned doc comment ("Pushes GIS map context-menu items for a screen-space hit") sits directly above `write_tiled_map_selection` (which builds a *selection* action, not a menu) — no actual map-context-menu builder function was found anywhere in the Scenes wgpu file, in contrast to Board2d's real equivalent (`board_context_menu_items`, doc-cited as mirroring React). Could be a stale comment left after a refactor, or a genuinely missing feature — flagged, not resolved. |
| 9b | Keyboard shortcuts | N/A | Absent both sides (React `TiledMapHost` has none beyond default browser behaviour). |
| 9c | Drag/drop (pan) | OK | React `pointerDownScreen`/`pointerMoveScreen`/`pointerUpScreen` (middle-mouse pan, `🟦️.tsx:1100,1110,1148`) map onto the same-named shared `MapHost` methods, wired via `SceneDragMode::MapPan`. |
| 10 | Spawned job progress | N/A | No async compute-job concept for TiledMap on either side. |
| 11 | Per-window view state | **P2 (likely vestigial on React side)** | React declares `useContext(WindowInstanceIdContext)` (`🟦️.tsx:727`) but a full-file grep shows it's never read again — looks like dead/vestigial React code, not something wgpu needs to match. No equivalent read found wgpu-side either (consistent, not a gap). |

**Test coverage:** no dedicated TiledMap test directory; coverage lives in the same
`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` as World3d/Board2d (attach/selection/hover
granularity vs `resolveMapInteractionSync`). TiledMap wheel/zoom→setCamera has only a capacity-fault case
covered in `wgpu-standalone`, not a real reachability test.

### 2.3 Canvas2d, Board2d, Paint2d, InkCanvas

**Headline for this cluster (restates and confirms §1.0 from a second angle):** `sync_engine_scene`'s match
arms only cover `NodeGraph | TiledMap | Board2d` — so of these four kinds, **only Board2d** has a real,
wired production render path; Canvas2d, Paint2d and InkCanvas paint an empty themed panel and nothing else
(confirmed: `⚙️EngineCanvas/…/🦀️.rs` shows 0 grep hits for all three kind names). Canvas2d's entire drawing
implementation (fill/stroke, 16 CSS blend modes, gradient banding, LOD grid, selection ring) exists only as
orphaned doc comments plus `#[cfg(test)]`-gated helpers (`🎞️Scenes/…/🦀️.rs:1753-2183`, e.g. lines 2120-2133
literally say GPU blend compositing is "out of scope for this ticket's Canvas2d/Paint2d regions"). The
function the tests call, `render_canvas_2d`, is defined only in the test-only
`🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs` (2808 lines, all `#[cfg(test)]`) — never in a production module.
InkCanvas's `//#region InkCanvasRender` block (`🎞️Scenes/…/🦀️.rs:3778-3789`) is literally empty despite
~1200 lines of real, non-test interaction logic (marquee scanning, stroke hit-test, camera math,
`InkCanvasState` 2577-3776) sitting right above it.

#### Canvas2d

React: `📐️Canvas2dHost/🟦️.tsx` (978 lines, read in full) + `🟦️GumballOverlay.tsx` (262 lines).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | P2 | React: `node.canvas2d`, prop-driven, no explicit lease code (`🟦️.tsx:736-738`). wgpu admits the surface (phase 0-2 byte-walk, `🎞️Scenes/…/🦀️.rs:1223`) but never renders it. |
| 2 | Wire schema tag | OK (slug) / P2 (drift risk) | Shared `Canvas2dScene::SCHEMA="canvas-2d@1"` (`🎬️scenes/🦀️.rs:206`). React's context-menu uses a separate compact literal `"canvas2d"` (`🟦️.tsx:916`) — same pattern on Board2d/Paint2d/InkCanvas, an established separate namespace, not real drift. |
| 3 | Pan/zoom/fit camera | P2 | React: `wheelCameraAtScreen`, pointer pan, 120ms debounce, no `fit` (`🟦️.tsx:35-66`). wgpu: real, non-test `setCamera` settle-dispatch machinery shared with Paint2d exists (`🎞️Scenes/…/🦀️.rs:361,742,835-920`, confirmed by `wgpu-canvas2d` test:63-101) — but since Canvas2d never paints, the camera has nothing to show. No `fit` on either side (parity on that sub-item). |
| 4 | Selection/hover/marquee | N/A | React itself: "No layer pick/selection is tracked at this level… hits/selection stay empty per surface convention" (`🟦️.tsx:904-905`). Not a wgpu gap — React has no selection domain here either. |
| 5 | Drawing/stroke/blend | **P0/ABSENT** | React: `drawSceneNode`, 16 blend modes, fill/stroke/gradient/path (`🟦️.tsx:134-259`). wgpu: all `#[cfg(test)]`-only or orphaned comments (see cluster headline). Nothing is drawn in a real build. |
| 6 | Grid/axes | **P0/ABSENT** | React: `drawInfiniteCanvasGrid`, 4 LOD steps (`🟦️.tsx:355-388`). wgpu: doc comment only, no function body. |
| 7 | Labels/captions | **P0/ABSENT** | React: `fillText` + `canvasLayerDisplayLabel` (`🟦️.tsx:215-259,273-275`). wgpu: none in production. |
| 8 | Context menu/keyboard/drag-drop | P2 | React: drag-over/drop/context-menu present, no `onKeyDown` (`🟦️.tsx:869-926`). wgpu: no Canvas2d-specific handlers found — flagged for confirmation whether drag/drop and context-menu are expected to be React-UI-layer-only per repo convention, not a hard gap either way. |
| 9 | Spawned job progress (tool-run-trace) | P1 | React: `⏯️tool-run-trace/🟦️.tsx` (141 lines) has no `subscribeSpawnedJobProgress` call either. wgpu: `Canvas2dSceneLane::ToolRunTrace` exists as a wire lane (`🎬️scenes/🦀️.rs:270-280`) but no consumer render logic found. Lane plumbing exists on the wire, rendered on neither side. |
| 10 | Per-window view state | P2 | React reads `activeUtility` from parsed `layersJson` meta (`🟦️.tsx:749-756`), not from window-scoped shell state — a different mechanism than other hosts. wgpu: n/a (nothing renders). Note for cross-host consistency, not urgent. |

#### Board2d

React: `🖥️Board2dHost/🟦️.tsx` (1514 lines). **The only one of these four kinds with real wgpu rendering.**

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | OK | React: `🟦️.tsx:558-620`. wgpu: `sync_board_engine`/`sync_board2d_scene`, `register_engine_surface` w/ `EngineSurfaceKindDetail::Board2d{fixture_json}` (`⚙️EngineCanvas/…/🦀️.rs:2574-2624,1872`) — real attach/drive lifecycle. |
| 2 | Wire schema + fixture lane | OK | Shared `Board2dScene "board-2d@1"` (`🎬️scenes/🦀️.rs:2005`), fixture lane up to ~100 KiB (`Board2dSceneLane::Fixture`, 2038-2050). wgpu's `sync_board_engine` doc-cites React's `applyFixtureToSession` field-by-field (`⚙️EngineCanvas/…/🦀️.rs:2365`) — manual port, not shared codegen, so future field additions could drift silently. |
| 3 | Camera pan/zoom | OK | Bespoke per-frame host state (`scene_has_bespoke_pointer_dispatch`, `🎞️Scenes/…/🦀️.rs:1417-1418`), consistent with BoardHost owning its own input. |
| 4 | Selection/hover/marquee | **P2** | React dispatches `interactionHover` (`board2dHoverActionArgs`, `🟦️.tsx:771,778`). wgpu's Board2d region (`🎞️Scenes/…/🦀️.rs:4105-4609`) has **no** `interactionSelect`/`interactionHover` string anywhere — uses a bespoke `BoardEventKind::Select`/menu-action model instead (`selectAll`/`setSelectionFlag`/`duplicateSelection`, 4105+94-111). Wire-verb model mismatch worth reconciling. |
| 5 | Placement/brush tools | OK | Shared fields `brush_weights_json`/`placement_compatibility_json`/`area_brush_size`/`suggestion_offset`/`suggestion_menu_json` (`🎬️scenes/🦀️.rs:1960-2004`); `board2dSuggestionMenuItems` (React) mirrored by `build_puzzle2d_selection_menu_items` (wgpu, `🎞️Scenes/…/🦀️.rs:4157`). |
| 6 | Grid/LOD | OK | `lod_mode` field carried through the scene (`🎬️scenes/🦀️.rs:1965`); `engine_surface_clear` special-cases Board2d clear color (`🎞️Scenes/…/🦀️.rs:1330,1418-1420`). |
| 7 | Labels/captions | OK | Delegated to the shared board engine (`sync_board_tool_run_trace`, `⚙️EngineCanvas/…/🦀️.rs:2502`; `infinite_canvas::BoardHost` draws its own labels) — same pattern as NodeGraph. |
| 8 | Keyboard shortcuts | **P1** | React: 3 keydown listeners incl. a capture-phase Tab handler (`🟦️.tsx:716-720,1292-1313,1338-1339`). wgpu: **zero** keyboard-related matches anywhere in Board2d wgpu code (both files) — entirely missing. |
| 9 | Context menu/drag-drop | OK | Not flagged as a gap by this pass (Board2d has real context-menu code, see item 5's `build_puzzle2d_selection_menu_items`). |
| 10 | Spawned job progress | N/A | None found either side. |
| 11 | Per-window / `domain_id` | **P2** | React: `domain_id`/`interactionDomainId` scopes hover/select (`🟦️.tsx:~205-291`). wgpu: `domain_id: Option<String>` field is carried in `Board2dScenePack` (`🎬️scenes/🦀️.rs:~1998`) but **not read anywhere** in wgpu's Board2d region (no grep hit for its usage there) — field exists on the wire, consumption unverified/likely absent. |

#### Paint2d

React: `🖌️Paint2dHost/🟦️.tsx` (697 lines, read in full). **Architecturally different from the other three**:
Paint2d's actual pixel content is produced by a separate WASM `RasterSession` on the React side, not by
draw calls in the host at all. wgpu's own comment (`🎞️Scenes/…/🦀️.rs:1473-1478`) states plainly that the
sibling `RasterHost` crate "is not wired into this wgpu renderer's dependency graph, so the fit math is
reimplemented here." No dedicated wgpu test file exists for Paint2d at all.

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | P1 | React mounts a WASM `RasterSession`. wgpu: Paint2d is absent from `sync_engine_scene`'s match arms — never attached as an engine host at all. |
| 2 | Wire schema | OK (slug) / **P1 (no paging lane)** | Shared `Paint2dScene "paint-2d@1"` (`🎬️scenes/🦀️.rs:1636`); wgpu structs exist (`Paint2dDocSyncJson`/`Paint2dCameraFields`, `🎞️Scenes/…/🦀️.rs:1428-1451`). **Real gap**: `Paint2dScene` has no `SceneLane`/`split_lanes`/`merge_lane` override (unlike Canvas2d/Board2d, `🎬️scenes/🦀️.rs:1616-1637`) — cannot page content past the 32 KiB surface-doc cap if the raster document ever exceeds it. |
| 3 | Camera pan/zoom/fit/navigator | **P1 (partial)** | React has camera + a navigator viewport overlay. wgpu: pan/zoom IS real and wired (`passive_scene_wheel`/`_pointer_button`/`_move`, `🎞️Scenes/…/🦀️.rs:1136-1188`, non-test) — but the navigator "fit to content"/"you are here" overlay math is empty function bodies with doc comments only (`🎞️Scenes/…/🦀️.rs:1466-1487`). |
| 4 | Selection/hover | **P1** | React dispatches `interactionHover`/`interactionSelect` on a "layers" domain (`🟦️.tsx:367,370,381,415`). Not found anywhere in wgpu Paint2d code. |
| 5 | Brush/raster painting | **ABSENT (by design)** | Core pixel engine (`brush_size`/`brush_opacity` → WASM `RasterSession`) is architecturally not ported to wgpu at all, per the code's own comment. This is the core parity blocker for Paint2d specifically. |
| 6 | Grid/axes, 7. Labels/captions | N/A | Neither side has these for a raster canvas — parity. |
| 8 | Context menu/keyboard/drag-drop | **P1** | React: `kind:"paint2d"` context menu (`🟦️.tsx:566`). wgpu: none found. |
| 10 | Per-window (`active_utility`/`view_mode`) | **P1** | Shared fields `active_utility`/`view_mode` (`🎬️scenes/🦀️.rs:1621-1629`) drive React's tool selection and composite/navigator mode. Not consumed anywhere in wgpu Paint2d code. |

#### InkCanvas

React: `🖋️InkCanvasHost/🟦️.tsx` (1609 lines, read in full).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | P2 | React: standard prop mount (`🟦️.tsx:955`). wgpu: `checked_ink_document` (`🎞️Scenes/…/🦀️.rs:2826-2864`, in `InkCanvasState`) validates/admits the doc for real — nothing downstream renders it though. |
| 2 | Wire schema — **field-name drift (P2, real)** | **P2** | Shared `SurfaceKind`/`InkCanvasScene::SCHEMA="ink-canvas@1"` (`🎬️scenes/🦀️.rs:2265`). wgpu reads the deserialized struct field `ink.document_json` (`🎞️Scenes/…/🦀️.rs:3020,3023,3037`), which arrives via serde `#[serde(rename_all="camelCase")]` → wire key `"documentJson"`. **But** that same struct's separate `ToValue::to_value` impl (`🎬️scenes/🦀️.rs:~2278`) pushes the field under key `"snapshotJson"` instead — two different encodings of the same struct disagree on the field's wire name. Only the serde/`"documentJson"` path appears actually exercised by wgpu; the `ToValue`/`"snapshotJson"` path's consumer is unclear — audit for dead-code vs. a second live transport that could silently drop the document. **Explicitly named by the ticket coordinator as a remaining item beyond W1b's scope.** |
| 3 | Camera pan/zoom | OK (logic) / ABSENT (visual) | Real, tested camera math (`ink_screen_to_world`/`ink_world_to_screen`/`ink_wheel_into`, `🎞️Scenes/…/🦀️.rs:893-988`, verified by `wgpu-ink-canvas` test:44-51) — correct but nothing renders to show it. |
| 4 | Selection/hover/marquee | P2 | Real marquee compute: `ink_marquee_points`/`scan_marquee` (`🎞️Scenes/…/🦀️.rs:881-931`), `SceneDragMode::InkMarqueeDrag` (745,975) — hit-testing has parity, but the marquee rectangle is never drawn (InkCanvasRender region empty) so it's invisible to a user. |
| 5 | Drawing/stroke rendering | **P0/ABSENT** (rendering) / OK (n/a for smoothing) | React strokes are raw polylines with fixed width, no pressure/smoothing feature on either side (confirmed by grep — this was never a real feature to port). wgpu: stroke hit-test/erase logic is real (`🎞️Scenes/…/🦀️.rs:285-395` range) but zero stroke *rendering* — the InkCanvasRender region is empty. |
| 6 | Grid/axes | **P0/ABSENT** | React: `MathRenderer`/grid overlay (`🟦️.tsx:780-805`). wgpu: none. |
| 7 | Labels/captions | **P0/ABSENT** | React: text/block rendering (`BlockViews`, `🟦️.tsx:627-809`). wgpu: none. |
| 8 | Context menu/keyboard | **P1 (keyboard)** | React: `onKeyDown` (`🟦️.tsx:827-834,923`), context menu (1354-1388). wgpu: no keyboard handling found in the InkCanvas region (2577-3776). |
| 10 | Per-window (`active_utility`/`view_mode`/`interactive`) | **P1** | Shared fields (`🎬️scenes/🦀️.rs:2251-2258`) drive React's tool/utility selection. Not consumed anywhere in wgpu (no render exists to consume them). |

### 2.4 NodeGraph, GraphTimeline, DiffView, BlockList, EventFeed

#### NodeGraph — the healthiest of the eleven non-World3d/Board2d/TiledMap kinds

React: `🕸️NodeGraph/🟦️.tsx` (3733 lines). Picks between **three** renderers at runtime
(`isFlowGraphScene(scene.capabilitiesJson) || scene.hostSnapshotJson` → `FlowGraphCanvasHost`, wasm
`@semio-tech/flow-core`, its own WebGPU presenter; vs `WasmGraphSurface`, wasm `GraphSession` from
`@semio-tech/infinite-canvas-react-renderer`; vs `DiagramGraphFallback`, DOM/React-Flow, SSR-only). Because
`GraphSession`/`flow-core` are **shared Rust crates compiled to both wasm (browser) and native (wgpu)**,
wgpu's NodeGraph support is largely the same engine code, not a re-implementation — qualitatively different
from every other kind in this audit.

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | OK | React picks engine per `scene.capabilitiesJson` (`🟦️.tsx:1312`), syncs `scenePack` (738-743). wgpu: `sync_node_graph_scene` from `sync_engine_scene` (`⚙️EngineCanvas/…/🦀️.rs:2617`); `NodeGraphEngine::Flow`/`Dag(host)` both paint via `host.paint_scene` (2633-2646). |
| 2 | Engine-selection predicate | OK | React `isFlowGraphScene`: `caps.engine==="flow" \|\| caps.spotlight===true \|\| caps.noteEdit===true` (`🪪️WasmSessionLoader/🟦️.tsx:384-391`). wgpu: identical predicate at `⚙️EngineCanvas/…/🦀️.rs:1962`. |
| 3 | Hit-test/drag-connect/minimap/pan-zoom/marquee | OK (well-tested) | React delegates `pointerDown/Move/UpScreen`/`wheelScreen` to the wasm session (`🟦️.tsx:950-983`); marquee from `selectionPreviewPointsJson` (700). wgpu: real gesture tests exist covering press-select+drag-move, connect, disconnect, minimap press→camera move, drag-across-port (`⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs:159-329`); marquee painted via `paint_node_graph_selection_marquee` (`⚙️EngineCanvas/…/🦀️.rs:3833`); minimap math in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🗺️minimap/🦀️.rs`, wired from the shared dag crate as a "thin wrapper". |
| 4 | Catalogue drag/drop onto canvas | P2 | React: `onDragOverCanvas`/`onDrop`/`onCanvasDoubleClick` (`🟦️.tsx:3380-3469`). wgpu: `CATALOGUE_DRAG_MIME` + payload parsing present (`⚙️EngineCanvas/…/🦀️.rs:2795-2826`), dedicated `wgpu-catalogue-workflow-drop` test exists but wasn't read in depth this pass. |
| 5 | Keyboard: Escape clears selection | **P2/inconclusive** | React: `handleGraphKeyboard`, Escape→`clearSelection` (`🟦️.tsx:622-630`). Not found as NodeGraph-local code in wgpu — likely routed through the generic app-keybinding dispatcher on both sides; not independently verified end-to-end. |
| 6 | Keyboard: delete/copy-paste nodes | **P2/inconclusive** | Not implemented locally on the React side either (only Escape); wgpu's `KeyAction::Delete/Backspace` only handles **text** editing (note-edit/text-editor), never node/edge deletion. Same architecture both sides, unverified whether either actually wires node deletion via the app-keybinding layer. |
| 7 | Node/edge context menu | Not verified | React: `openSurfaceContextMenu({menu:{id:"nodeGraph"}, ...})` (`🟦️.tsx:895-928`). Not directly checked in wgpu within this pass's budget — flag for follow-up. |
| 8 | Spawned job progress | N/A | Not used by NodeGraph on either side. |
| 9 | Per-window view state | Not verified | React reads `WindowInstanceIdContext` only for introduction-resolver/context-menu targeting (`🟦️.tsx:687,2666`); no `activeUtility` usage found. wgpu side not checked. |

#### GraphTimeline — P0/ABSENT

React: `🌳️GraphTimelineHost/🟦️.tsx` (96 lines). **Correction to this ticket's framing**: this is not a
scrolling/zooming time axis — it's a plain `HistoryTable` row list from `scene.columnsJson`
(`🟦️.tsx:20,33-40,72-82`), row-select dispatches `checkoutCheckpoint` (75-81), `graphTimeline` context menu
(50-64).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | **P1/ABSENT** | `UiComponentSceneNode.graph_timeline: Option<GraphTimelineScene>` field exists, used only in test fixtures (`🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs:90`); no reachable production caller. |
| 2 | Wire schema | Schema OK, impl ABSENT | `HistoryColumnJson` (`🎞️Scenes/…/🦀️.rs:1706-1720`, `#[serde(rename_all="camelCase")]`) field names match React's `HistoryColumn` exactly, but the struct is `#[cfg(test)]`-only, unreachable from paint. |
| 3 | Rendering (rows/lanes/avatars) | **ABSENT/P0** | `history_lane_count`/`history_graph_width`/`history_lane_x`/`history_row_lane_guides`/`graph_timeline_avatar_initials`/`render_graph_timeline` are all called by the test module but **undefined anywhere in the crate** (verified via full-text search, not just grep). |
| 4 | Context menu | ABSENT | React: `menu:{id:"graphTimeline"}` (`🟦️.tsx:50-60`). Not found wgpu-side. |
| 5 | Row select/checkout | ABSENT | React: `onSelectCheckpoint`→`checkoutCheckpoint` (75-81). No wgpu dispatch found. |
| 6 | Scroll | Dead code | `scroll_offset(surface_id,"history")` tracked (`🎞️Scenes/…/🦀️.rs:1133`) but never painted against. |

#### DiffView — P0/ABSENT

React: `🔺️DiffViewHost/🟦️.tsx` (233 lines). Custom LCS line diff (`diffLines`, 26-60), unified (default) or
split (`buildSplitRows`, 63-89) rendering, no syntax highlighting; also exports `ConflictDiffPreview`
reused elsewhere outside DiffViewScene.

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | ABSENT in production | `UiComponentSceneNode.diff_view: Option<DiffViewScene>` reachable only from tests (`🎞️Scenes/🧪️tests/🔬️wgpu-diff-view/🦀️.rs:65-84`). |
| 2 | Wire schema | Schema OK, impl ABSENT | `DiffViewScene{before, after, language, mode, domain_id}` matches React's `{before, after, mode, language}` — unreachable. |
| 3 | Diff algorithm | ABSENT (P0); **P2 divergence noted for when restored** | wgpu's doc comment (`🎞️Scenes/…/🦀️.rs:1602-1638`) describes an LCS diff with an explicit `DIFF_LCS_CELL_BUDGET=200_000` fallback to positional compare for oversized input. React has **no** size cap. A real behavioral divergence once this is implemented — a huge diff could render differently shaped on the two renderers. |
| 4 | Highlighting model | ABSENT (P0) despite correct *design* | React: text-color-only tinting, no row-background wash (`🟦️.tsx:93-97`). wgpu's doc comment explicitly matches this intent ("the opposite of what React source of truth does [i.e. don't wash the row]", `🎞️Scenes/…/🦀️.rs:1629-1637`), and the test module asserts exactly this — good intent, unreachable in production. |
| 5 | Split vs unified mode | ABSENT (P0) | Split-mode test exists (`🔬️wgpu-diff-view/🦀️.rs:129-135`) but unreachable. |
| 6 | Context menu | ABSENT | React: `menu:{id:"diffView"}` (`🟦️.tsx:186`). Not found wgpu-side. |
| 7 | Scroll | Dead code | `scroll_offset(surface_id,"diff")` tracked, never painted. |

#### BlockList — P0/ABSENT, plus one documented divergence

React: `🧩️BlockListHost/🟦️.tsx` (272 lines). dnd-kit sortable steps/blocks with real pointer drag-and-drop, a
drag-and-drop block palette (native HTML5 drag + a "driver-arm-drag" mode), step/block add/remove/move
actions. No pagination/virtualisation.

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | ABSENT in production | `UiComponentSceneNode.block_list` test-only reachable (`🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs:4-27`). |
| 2 | Wire schema | Schema OK, impl ABSENT | `BlockListStepJson`/`BlockListBlockJson`/`BlockListPaletteEntryJson` (`🎞️Scenes/…/🦀️.rs:1553-1580`) match React's `StepRecord`/`PaletteEntryRecord` field-for-field, `#[cfg(test)]`-only. |
| 3 | Reordering model | **Documented P2 divergence (once implemented)** | React: continuous pointer drag via dnd-kit. wgpu's own doc comment explicitly documents an intentional divergence to click-based move-up/move-down hit targets instead of free-form drag, citing no established cross-frame drag-position-tracking primitive (`🎞️Scenes/…/🦀️.rs:1586-1592`). Currently moot since ABSENT. |
| 4 | Selection highlighting | Noted, not urgent | Neither React renders a `selected_id` concept today; wgpu's doc comment says it would render selection even though React doesn't yet — a forward-divergence to watch, not a regression. |
| 5 | Palette drag-drop (native + driver-arm) | ABSENT (P0, folded into general absence) | React: `useNativeDragArm`/`useUiDriverDragSurface` dual-mode (`🟦️.tsx:66,86,138-155`). No drag-surface concept found wgpu-side. |
| 6 | Add/remove/move step & block actions | Intent OK, impl ABSENT | Action-name comment in wgpu (`🎞️Scenes/…/🦀️.rs:1585-1586`) confirms intent to mirror React's exact verb names. |
| 7 | Context menu | ABSENT | React: `menu:{id:"blockList"}` (`🟦️.tsx:215`). Not found wgpu-side. |

#### EventFeed — P0/ABSENT, plus one field-name drift

React: `📡️EventFeedHost/🟦️.tsx` (153 lines). Scrollable log, auto-scroll-to-bottom while `scene.follow`
(72-77), row click dispatches `{surfaceId, id: entry.id}` (117-126), per-row context-menu hits (unlike
GraphTimeline/DiffView/BlockList's whole-surface-only hits).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | ABSENT in production | `UiComponentSceneNode.event_feed` test-only reachable. |
| 2 | Wire schema | Schema OK, impl ABSENT | `EventFeedEntryJson` (`🎞️Scenes/…/🦀️.rs:1652-1666`) matches React's `EventFeedEntry{id,timestampMs,iconId,title,detail?,tone?}` 1:1 incl. `tone` — `#[cfg(test)]`-only. |
| 3 | Follow / live-append auto-scroll | ABSENT (P0) despite correct design intent | React snaps scroll to bottom on new entries while `follow` (`🟦️.tsx:72-77`). wgpu's doc comment describes the identical log-tail behaviour correctly — no function exists to execute it. |
| 4 | Tone→color mapping | ABSENT (P0); **P2 tone-set risk to re-check once implemented** | React has a 5-way map (`info/success/warning/error/fatal`, `fatal` bold, `🟦️.tsx:20-28`). wgpu's doc comment suggests a *narrower* mapped-tone set ("only the tones this crate already has a token for get a distinct color") — can't confirm parity since the function body is absent. |
| 5 | Row click → activateAction — **field-name drift, already baked into the (unreachable) design** | **P2-in-waiting; overall ABSENT (P0)** | React sends `{surfaceId, id: entry.id}` (117-124). wgpu's own doc comment says it will dispatch `{"entryId": ...}` (`🎞️Scenes/…/🦀️.rs:1680-1682`) — **`id` vs `entryId`**, exactly the kind of key-name drift this ticket asked to hunt for. Unconfirmed against real code since unimplemented — fix in the same pass that restores the function, not after. |
| 6 | Per-row context menu (`hits`) | ABSENT | React reports `hits:[{domain:"entry", id: entryId}]`, unlike the other three "whole-surface-only" hosts (`🟦️.tsx:83-104`). Not found wgpu-side. |
| 7 | Scroll (non-follow) | Dead code | `scroll_offset(surface_id,"feed")` tracked, never painted. |

**Systemic note from this cluster's audit:** `Table` shows the identical "schema matches, `render_table`
undefined, absent from `sync_engine_scene`'s match" symptom — see §2.5, confirms this is one systemic gap
(the `render_component_scene_step`/`sync_engine_scene` dispatch), not five isolated ones. **Recommendation
before writing new render code for GraphTimeline/DiffView/BlockList/EventFeed/Table: first confirm whether
`render_component_scene_step`'s dispatch is even the intended call site for these kinds** — 6 test modules
currently reference undefined symbols, which is either a genuinely incomplete migration or these kinds were
meant to route through a different widget path than "engine-canvas" entirely. Cargo compile status of the
crate's test target was not checked (read-only audit).

### 2.5 TextEditor, Table, IconRender, VirtualFileSystem

Same headline pattern as §2.3/§2.4 applies to all four (see §1.0) — restated per-kind below with each
sub-audit's own citations.

#### TextEditor — P0

React: `✏️TextEditor/🟦️.tsx` (769 lines) — full wasm editor: cursor/selection via `EditorWasmSession`
(116-267), F2 multi-span rename (291-330), alt-click/Ctrl+Space completions (268-289,587-608), keyboard
editing incl. Tab/Enter/Backspace/Delete/arrows (556-696), grammar-token highlighting (`tokensJson`, SSR
fallback `HighlightedBuffer` 51-69), context menu (399-503).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | P1 | Scene wired as `UiComponentSceneNode.text_editor`; `ENGINE_SURFACES` registry entry lazily created — but never by a real paint pass. |
| 2 | Wire schema | OK | `TextEditorScene` (`🎬️scenes/🦀️.rs:1409-1443`, `SCHEMA="text-editor@1"`) — shared Rust struct, generated bindings match React field-for-field (`🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js:6889`). |
| 3 | Editing model | **P0** | `framework_editor::EditorHost` (`🧰️framework/🔨️modules/✍️editor/🦀️.rs`, 1782 lines) is a REAL engine — `build_scene()` (1217) paints colored lines/carets/diagnostics/placeholders, semantic-token spans — but nothing calls it from wgpu's paint path. Keyboard input is also unwired: `text_editor_apply_key_into` (`⚙️EngineCanvas/…/🦀️.rs:4702`) has zero production callers repo-wide (only `🧪️tests/🧊️wgpu-standalone/🦀️.rs` calls it). No IME/undo-redo in `EditorHost` — but React doesn't implement these either, so that sub-item is parity, not a gap. |
| 4 | Pointer/wheel dispatch | P2 | Real, production-wired: `process_scene_interaction`'s bespoke branch (`🗣️Interpreter/…/🦀️.rs:943-955`) calls `engine_canvas::text_editor_pointer_button_into/_move_into/_wheel_into` — but produces no visible effect since nothing paints. |
| 7 | Context menu/keyboard/drag-drop | P1 | No text-editor-specific context-menu or keyboard-shortcut wiring found in `🗣️Interpreter/…/🦀️.rs`. |
| — | Overall | **P0** | Pointer plumbing is real; render is 100% ABSENT in production; keyboard typing is ABSENT in production. |

#### Table — P0/ABSENT

React: `📊️Table/🟦️.tsx` (280 lines) — columns/rows/sort (106-158,192-206), stepper & row-action-button
cells (59-86), row click→`interactionSelect`/`selectRow` (221-233), row drag source (207-220) + drop target
(162-190), per-row context menu (42-57,234-264).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | P1 | Scene wired as `.table`, no dedicated engine host (unlike NodeGraph/TiledMap/Board2d). |
| 2 | Wire schema | OK | `TableScene "table@1"` (`🎬️scenes/🦀️.rs:1552-1591`) — matches React field names. Explicitly documented as one of "the 11 generic-fallback surface kinds" per `🧵️TaskManager/🟦️.tsx:8-16`. |
| 4 | Paging/virtualisation, sort, row selection, cells | **P0 (ABSENT in production)** | `passive_scene_pointer_button`/`_move` (`🎞️Scenes/…/🦀️.rs:1159-1200`, production) branch only on `SurfaceKind::Paint2d` — Table falls through and does nothing. Real logic (`render_table`/`render_table_cell`, `sortTable` dispatch) exists only at `🧪️tests/🧊️wgpu-standalone/🦀️.rs:619,651,683`. |
| 4b | Scroll | P1 | `passive_scene_wheel` (production) tracks a `"body"` scroll-offset for Table — dead code since nothing ever paints using it. |
| 7 | Row drag source/drop target | ABSENT/P0 | `row_drag_mime` referenced only at `🧪️tests/🧊️wgpu-standalone/🦀️.rs:728` (test-only). |
| 7b | Context menu | P1 | No production row-context-menu wiring found for `SurfaceKind::Table`. |
| — | Overall | **P0/ABSENT** | Rendering and interaction both absent in production; a complete test-only mirror exists but is unreachable. |

#### IconRender — ABSENT

React: `🖼️IconRenderHost/🟦️.tsx` (74 lines) — resolves the guest's `/mesh/…` catalog URL via
`meshAssetTransportUrl(parsed.assetUrl)` (11,25), renders an async GLB shot preview via
`iconRenderPort.render()` (35-51) inside `IconShotFrame` chrome with optional caption (69).

| # | Item | Status | Finding |
|---|---|---|---|
| — | **Naming trap** | note | The only file at `🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs` (78 lines) is completely unrelated — it's the Lucide **UI-icon atlas rasterizer** (toolbar icons), not a renderer for the `IconRender` **surface kind**. Misleading co-location worth flagging on its own. |
| 2 | Wire schema | OK | `IconRenderScene "icon-render@1"` (`🎬️scenes/🦀️.rs:1679-1696`) — no drift. |
| 5 | Mesh transport URL / camera / setCamera | **P1** | No `mesh_asset_transport_url`-equivalent exists anywhere in Rust (repo-wide grep, 0 hits) — even once rendering is wired, the `/mesh/` public-id → transport-path resolution React applies would still be missing on wgpu. |
| 3/6 | Rendering (frame border, badge, delegated GLB draw) | **ABSENT/P0** | `paint_icon_render_chrome` exists only at `🧪️tests/🧊️wgpu-standalone/🦀️.rs:2484` (exercised by `🔬️wgpu-icon-render/🦀️.rs`, 36 lines) — zero production callers. The delegated GLB-preview function `render_icon_render`, referenced by a doc comment at `🎞️Scenes/…/🦀️.rs:4068`, **does not exist anywhere in the repo**, test or production. |
| 4 | Pointer/wheel | P1 | `passive_scene_wheel`/`_pointer_button`/`_move` (production) have no `SurfaceKind::IconRender` branch — falls to the default no-op arm. |
| — | Overall | **ABSENT** | Completely unimplemented in production; the dedicated wgpu target directory contains an unrelated module. |

#### VirtualFileSystem — ABSENT (wgpu) / OK (React, once you find it)

React ground truth located (no dedicated `elements/VirtualFileSystemHost` directory exists — itself
notable): `VirtualFileSystemHost` is defined **inline** in
`🗣️Interpreter/🟦️.tsx:778-834`, dispatched via a **special-cased `if (props.kind ===
"virtual-file-system")` branch** in `renderComponentSceneHost` (`🟦️.tsx:546-552`) that **bypasses** the
normal `resolveComponentSceneHost` switch/registry (`🟦️.tsx:358-391`) — that registry has no `case
"virtual-file-system"` and falls to `default: return undefined` (388-389). `SURFACE_KIND_SCENE_FIELD`
(`🟦️.tsx:408-424`) still correctly maps `"virtual-file-system" → "virtualFileSystem"` (the scene-doc
*field* name, not the wire-kind tag — not the same drift as §1.1's `SurfaceKind` enum). Renders the real
`VirtualFileSystem` list/tree primitive from `@semio-tech/ui-react` with schema/rows/selected-ids parsing
(796-798), selection dispatch `"selectRows"` (808), row context menu (809-828), drag/drop flag (807) —
**this is a fully working consumer, not a stub**, correcting this ticket's ABSENT-hypothesis for the React
side specifically.

wgpu: `VirtualFileSystemScene "virtual-file-system@1"` (`🎬️scenes/🦀️.rs:1716-1733`, with an explicit
in-code comment documenting the intentional camelCase→kebab-case wire-tag rename — no drift at this layer,
consistent with §1.4).

| # | Item | Status | Finding |
|---|---|---|---|
| 1 | Mount/lease | P1 | Scene wired as `.virtual_file_system`, no dedicated engine host. |
| 2 | Wire schema | OK | No drift (see above). |
| 6 | Tree browsing/row rendering | **ABSENT/P0** | No render function reachable in production (same headline finding). |
| 4 | Scroll | P1 | `passive_scene_wheel` (production) tracks a `"vfs"` scroll-offset — dead, nothing paints with it. |
| 7 | Double-click to open / drag-drop | **ABSENT/P0** | `vfs_double_click_action` (`navigateUri`→`openInstance`/export actions) and the VFS row hit-test (`hit_double_click_target`) exist only at `🧪️tests/🧊️wgpu-standalone/🦀️.rs:2548-2564,512-522` — zero production callers. |
| — | Overall | **ABSENT (wgpu) / OK (React)** | React has a real, working consumer reached through a non-obvious special-case branch (not the registry); wgpu has none. |

**Cross-cutting for this cluster:** neither React nor wgpu wires spawned-job-progress subscription for any
of these four hosts (N/A both sides). Per-window view-state usage is minimal: React `Table`/
`VirtualFileSystemHost` read `WindowInstanceIdContext` only to stamp context-menu requests (`📊️Table/
🟦️.tsx:94`, `🗣️Interpreter/🟦️.tsx:786`); wgpu has no equivalent since it never reaches per-row/per-cell
interaction at all for these kinds.

## 3. Recommended work packets

Packets W1a (list-like kinds: GraphTimeline/DiffView/BlockList/EventFeed/Table), W1b (Canvas2d/Paint2d/
InkCanvas/TextEditor/IconRender) and W1f (World3d terrain fetch, `setCamera` payload, map interaction
verbs, map context menu) are already claimed/in-flight elsewhere in this ticket and cover the bulk of §1.0's
headline finding — they are **not** re-listed here. **VirtualFileSystem is not covered by either W1a or
W1b** (it's neither in W1a's five list-like kinds nor W1b's five kinds) — WP-1 below fills that hole. The
packets below are what's left: items outside W1a/W1b/W1f's declared scope, plus specific citations each of
those packets should pick up while the relevant file is already open, so they don't get re-discovered as a
separate pass later. Each is independently landable; only WP-8/WP-9 are sequenced (land after W1a/W1b's
base paint, respectively, since they touch the same functions).

1. **WP-1 (P0, standalone) — Wire `VirtualFileSystem` into the production render dispatch.**
   No declared packet covers this kind. Seam: `render_component_scene_step`
   (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1223`) / `sync_engine_scene`
   (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2615-2621`) needs a `VirtualFileSystem` arm — or, per §2.4's
   closing note, first confirm this dispatcher is even the intended call site before adding to it. Promote
   `vfs_double_click_action`/`hit_double_click_target` out of `🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs`
   into a production module. React reference: `🗣️Interpreter/🟦️.tsx:778-834`
   (`VirtualFileSystemHost`, reached via a special-case branch at 546-552, not the normal host registry —
   read that branch first, it's easy to miss). Acceptance: a `wgpu-virtual-file-system` test that calls
   `render_component_scene_step` itself (not the raw `render_*`/`vfs_*` functions directly) passes; row
   double-click and scroll produce a visible effect.

2. **WP-2 (P0, standalone) — Confirm/close the World3d terrain DEM tile fetch pipeline.**
   `pending_terrain_tile_urls` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:1381`) is
   written and never read by any non-test code found in this audit — terrain elevation likely never loads.
   Nominally in W1f's scope; this packet is to **verify** it's actually closed (not just planned) before
   moving on, using the existing bounded `WorldAssetFetchOwner` queue
   (`🦀️.rs:12108-12520`) as the wiring template if it isn't. React reference:
   `🗺️WorldTerrainLayer/🟦️.tsx:165-187` (`TerrainTileRenderer.uploadOne`). Acceptance: `grep -c
   pending_terrain_tile_urls` outside the file/its own test shows ≥1 real reader; a manual repro (World3d
   window with a terrain layer) actually shows elevation.

3. **WP-3 (P0/P1, standalone, cheap) — Wire the two other dead byte-apply pipelines.**
   Same pattern as WP-2 at smaller scale: `apply_ui_image_bytes(id, url, bytes)`
   (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1495`) and `apply_map_tile_bytes(kind, bytes)`
   (`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:4192`) are both `pub fn` with zero production
   callers anywhere in the repo (only unit tests call them). Find or build the fetch-completion path that
   should call each. Acceptance: both functions show ≥1 non-test caller.

4. **WP-4 (P2, standalone, cheap) — InkCanvas `documentJson`/`snapshotJson` field-name split.**
   `InkCanvasScene`'s serde encoding (`#[serde(rename_all="camelCase")]`, exercised path) disagrees with
   its own `ToValue::to_value` impl (`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:~2278`) on the wire key for the document
   payload — `"documentJson"` vs `"snapshotJson"`. Pick one (recommend `"documentJson"`, the one actually
   read at `🎞️Scenes/…/🦀️.rs:3020,3023,3037`) and fix or remove the other encoding path; audit whether
   `ToValue`'s path is genuinely dead or a second live transport that's silently dropping the document.
   Acceptance: repo-wide grep shows exactly one of the two key names for this field.

5. **WP-5 (P1/P2, standalone) — Board2d: `domain_id` consumption, keyboard shortcuts, selection-verb
   reconciliation.** Three related, co-located items in the Board2d wgpu region
   (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4105-4609`): (a) `domain_id: Option<String>` is carried in
   `Board2dScenePack` (`🎬️scenes/🦀️.rs:~1998`) but never read there; (b) zero keyboard handling anywhere
   in Board2d wgpu code vs. React's 3 listeners incl. a capture-phase Tab handler
   (`🖥️Board2dHost/🟦️.tsx:716-720,1292-1313,1338-1339`); (c) wgpu uses a bespoke `BoardEventKind::Select`
   model instead of React's generic `interactionHover`/`interactionSelect` verbs
   (`🖥️Board2dHost/🟦️.tsx:771,778`) — decide whether to keep the bespoke model (document why) or convert.
   Acceptance: a wgpu Board2d test exercises `domain_id`-scoped selection; a keyboard-shortcut test exists;
   the verb-model decision is recorded in a doc comment either way.

6. **WP-6 (P1/P2, standalone) — TiledMap: action-verb reconciliation, context-menu, wheel-zoom
   reachability.** Three items in the TiledMap wgpu region: (a) wgpu dispatches bespoke
   `SET_FEATURE_SELECTION`/`SET_HOVER` where React dispatches generic `interactionSelect`/`interactionHover`
   (`🧭️TiledMapHost/🟦️.tsx:191-194`) — confirm which the guest actually listens for before "fixing"
   either side; (b) an orphaned doc comment above `write_tiled_map_selection`
   (`🎞️Scenes/…/🦀️.rs:~3842`) describes a map context-menu builder that doesn't exist anywhere in the
   file — either write it (see Board2d's `board_context_menu_items` as a template) or delete the stale
   comment; (c) `tiled_map_wheel_into` (`⚙️EngineCanvas/…/🦀️.rs:4084-4089`) has no caller in
   Scenes/EngineCanvas — its only caller is in the os renderer file outside this lane's scope
   (`…engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12066`) — confirm it's actually reachable at runtime.
   Acceptance: guest-side action table cross-checked for (a); context-menu function exists and is tested,
   or comment removed with a one-line reason, for (b); a reachability test or trace confirms (c).

7. **WP-7 (P1, standalone, small) — World3d `setCamera` wire-shape mismatch.** React sends
   `{windowId, camera:{position, target, zoom, up?}}` (`🌐️World3dHost/🟦️.tsx:764-780`); wgpu's
   `orbit_camera_action` sends `{surfaceId, camera:{position, target, fov}}` — no `zoom`, and a `fov` React
   never sends (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9134-9153`). Likely breaks
   orthographic-projection zoom round-tripping specifically. Nominally W1f's scope — flagged separately
   since it's a precise, independently-landable fix. Acceptance: a wgpu test pins the exact JSON shape
   against React's `worldCameraSetCameraDispatchArgs`.

8. **WP-8 (P2, sequenced after W1a lands) — Wire-drift and divergence fixes for the list-like kinds.**
   Fold into the same diff that promotes GraphTimeline/DiffView/BlockList/EventFeed/Table out of
   `#[cfg(test)]`, so these don't need rediscovering later: EventFeed's row-click payload key is `entryId`
   in wgpu's own doc comment vs React's `id` (`🎞️Scenes/…/🦀️.rs:1680-1682` vs
   `📡️EventFeedHost/🟦️.tsx:117-124`) — fix to match React before landing; EventFeed's tone→color mapping
   may be narrower than React's 5-way `info/success/warning/error/fatal` set
   (`📡️EventFeedHost/🟦️.tsx:20-28`) — verify all 5 get distinct treatment; DiffView's `DIFF_LCS_CELL_BUDGET
   = 200_000` fallback has no React-side equivalent (`🔺️DiffViewHost/🟦️.tsx`'s `diffLines` has no size
   cap) — decide and document whether this divergence is acceptable; BlockList's native+driver-arm palette
   drag-drop (`🧩️BlockListHost/🟦️.tsx:66,86,138-155`) has no wgpu equivalent at all and isn't mentioned in
   the existing click-based-reorder divergence comment — scope it in or explicitly defer it in writing.
   Acceptance: each citation above has a resolved status (fixed or explicitly deferred with a comment) in
   the landing diff, not a follow-up ticket.

9. **WP-9 (P1/P2, sequenced after W1b lands) — Remaining items for Canvas2d/Paint2d/InkCanvas/
   TextEditor/IconRender beyond base paint.** Fold into or immediately follow W1b's diff: Paint2d has no
   `SceneLane`/`split_lanes`/`merge_lane` override (`🎬️scenes/🦀️.rs:1616-1637`, contrast Canvas2d/Board2d)
   — will silently fail to page a raster document past the 32 KiB surface-doc cap; Paint2d's navigator
   fit-overlay math is empty function bodies (`🎞️Scenes/…/🦀️.rs:1466-1487`); Paint2d/InkCanvas's shared
   `active_utility`/`view_mode`/`interactive` fields (`🎬️scenes/🦀️.rs:1621-1629,2251-2258`) aren't consumed
   anywhere in wgpu; Canvas2d's `ToolRunTrace` scene lane (`🎬️scenes/🦀️.rs:270-280`) has no render
   consumer on either side (verify it's meant to be rendered at all before building one); IconRender needs
   a `meshAssetTransportUrl`-equivalent (`/mesh/` public-id → transport-path resolution, zero Rust hits
   repo-wide) even once its chrome-paint function is written; TextEditor has no context-menu or
   keyboard-shortcut wiring in `🗣️Interpreter/…/🦀️.rs` beyond the pointer dispatch that already works.
   Acceptance: each field/function cited above has a confirmed wgpu-side reader or an explicit "intentionally
   unconsumed" comment once W1b lands.

10. **WP-10 (P2, standalone, research-only) — Resolve NodeGraph's "not verified" items.** This audit
    could not confirm (not: could not find — genuinely didn't check within budget) whether wgpu's
    app-keybinding layer actually wires NodeGraph's Escape-clears-selection
    (`🕸️NodeGraph/🟦️.tsx:622-630`) or node/edge delete — and didn't directly check the node/edge context
    menu (`🕸️NodeGraph/🟦️.tsx:895-928`) against wgpu at all. NodeGraph is otherwise the healthiest kind in
    this audit (shared native engine crates with React); closing these three unknowns is low-risk,
    low-effort verification work, not a rewrite. Acceptance: each item gets a written OK/P1/P2 verdict with
    citations, replacing "not verified".

